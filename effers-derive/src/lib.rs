use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::token::{Mut, SelfValue};
use syn::visit_mut::VisitMut;
use syn::{parse_macro_input, Expr, ExprCall, FnArg, Ident, ItemFn, Path, PathSegment, Receiver};

mod parse;
use parse::Args;

#[proc_macro_attribute]
pub fn program(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as syn::ItemFn);
    let mut args = parse_macro_input!(attr as Args);

    if args.name.is_none() {
        let i = item.sig.ident.to_string().to_case(Case::Pascal);
        args.name = Some(Ident::new(&i, Span::call_site()));
    }

    let out = if !args.effects.is_empty() {
        rewrite_item_into_struct(item, args)
    } else {
        quote!(item)
    };

    proc_macro::TokenStream::from(out)
}

#[derive(Clone, Copy)]
struct LettersIter {
    idx: u32,
}

impl LettersIter {
    fn new() -> Self {
        Self {
            idx: 'A' as u32 - 1,
        }
    }
}
impl Iterator for LettersIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        for _ in 0..100 {
            self.idx += 1;
            if let Some(c) = char::from_u32(self.idx) {
                return Some(c);
            }
        }

        None
    }
}

/// takes in the function contents, and returns the structs n stuff
fn rewrite_item_into_struct(func: ItemFn, args: Args) -> TokenStream {
    let prog_name = (&args.name).clone().unwrap();

    let intermediate_structs = intermediate_structs(&args, &prog_name);

    // structs for the builder pattern
    let inters_tokens = intermediate_structs
        .iter()
        .map(|i| i.tokens.clone())
        .collect::<TokenStream>();

    // implementations for the builder pattern
    let impls = impls(&prog_name, &intermediate_structs);

    // make the last impl, which contains the run method
    let final_impl = final_impl(intermediate_structs.last().unwrap(), func, &args);

    let out = quote! {
        struct #prog_name;

        #inters_tokens
        #impls

        #final_impl
    };

    TokenStream::from(out)
}

struct IntermediateStruct {
    tokens: TokenStream,
    id: Ident,
    traits: Vec<Path>,
    letters: Vec<Ident>,
    generics: TokenStream,
}

impl IntermediateStruct {
    fn new(
        tokens: TokenStream,
        id: Ident,
        traits: Vec<Path>,
        letters: Vec<Ident>,
        generics: TokenStream,
    ) -> Self {
        Self {
            tokens,
            id,
            traits,
            letters,
            generics,
        }
    }
}

fn intermediate_structs(args: &Args, prog_name: &Ident) -> Vec<IntermediateStruct> {
    let struct_with = format!("{}With", prog_name);
    args.effects
        .iter()
        .fold(
            (vec![], struct_with, vec![]),
            |(mut structs, name, mut traits), eff| {
                let name = format!("{}{}", &name, &eff.name);

                traits.push(eff.path.clone());

                // kinda messy
                let letters = LettersIter::new()
                    .take(traits.len())
                    .map(|c| Ident::new(&c.to_string(), Span::call_site()));
                let generics = traits
                    .iter()
                    .zip(letters.clone())
                    .map(|(t, c)| quote!(#c: #t,))
                    .collect::<TokenStream>();

                let id = Ident::new(&name, Span::call_site());
                let last = if let Some(&IntermediateStruct {
                    ref id,
                    ref letters,
                    ..
                }) = &structs.last()
                {
                    let gen = letters.iter().map(|l| quote!(#l,)).collect::<TokenStream>();
                    quote!(#id<#gen>)
                } else {
                    quote!(#prog_name)
                };
                let last_letter = letters.clone().last();
                structs.push(IntermediateStruct::new(
                    quote! {
                        struct #id<#generics>(#last, #last_letter);
                    },
                    id,
                    traits.clone(),
                    letters.collect::<Vec<_>>(),
                    generics,
                ));

                (structs, name, traits)
            },
        )
        .0
}

fn impls(prog_name: &Ident, intermediate_structs: &Vec<IntermediateStruct>) -> TokenStream {
    let mut impls = vec![];
    let mut id = quote!(#prog_name);
    let mut impl_token = quote!(impl);
    for inter in intermediate_structs {
        let next = inter.id.clone();
        let gen = inter
            .letters
            .iter()
            .map(|l| quote!(#l,))
            .collect::<TokenStream>();
        let ret = quote!(#next<#gen>);

        // lmao what a mess

        // need to get the last trait/letter, then replace P: Printer and p: P
        let t = inter.traits.last().unwrap();
        let l = inter.letters.last().unwrap();
        let func_generics = quote!(#l: #t);

        // and need to destructure self

        impls.push(quote! {
          #impl_token #id {
            fn add<#func_generics>(self, t: #l) -> #ret {
              #next(self, t)
            }
          }
        });

        id = ret;
        let generics = &inter.generics;
        impl_token = quote!(impl<#generics>);
    }
    impls.into_iter().collect::<TokenStream>()
}

fn final_impl(last: &IntermediateStruct, func: ItemFn, args: &Args) -> TokenStream {
    let id = &last.id;
    let full_gen = &last.generics;
    let gen = last
        .letters
        .iter()
        .map(|l| quote!(#l,))
        .collect::<TokenStream>();

    let func = rewrite_func(func, args);

    quote! {
      impl<#full_gen> #id<#gen> {
        #func
      }
    }
}

fn rewrite_func(mut func: ItemFn, args: &Args) -> TokenStream {
    func.sig.ident = Ident::new("run", Span::call_site());
    // add `mut self` as a parameter
    func.sig.inputs.insert(
        0,
        FnArg::Receiver(Receiver {
            attrs: vec![],
            reference: None,
            mutability: Some(Mut {
                span: Span::call_site(),
            }),
            self_token: SelfValue {
                span: Span::call_site(),
            },
        }),
    );

    FuncRewriter { args }.visit_item_fn_mut(&mut func);

    quote! {
      #func
    }
}

struct FuncRewriter<'a> {
    args: &'a Args,
}
impl<'a> syn::visit_mut::VisitMut for FuncRewriter<'a> {
    fn visit_expr_call_mut(&mut self, node: &mut ExprCall) {
        let eff_len = self.args.effects.len();

        // check if the function name is in args
        // if it is, replace it with the correct name
        if let Expr::Path(path) = &mut *node.func {
            for (i, effect) in self.args.effects.iter().enumerate() {
                for func in &effect.functions {
                    let ident = func.alias.clone().unwrap_or(func.ident.clone());
                    if path.path.is_ident(&ident) {
                        // get the effect trait path's, and append the function as a segment
                        let mut effect_path = effect.path.clone();
                        effect_path.segments.push(PathSegment {
                            ident: func.ident.clone(),
                            arguments: syn::PathArguments::None,
                        });

                        path.path = effect_path;

                        // then change the parameters so the handler is the first
                        // get the effect's index, and add the inverse num of `.0`s
                        let idx = eff_len - (i + 1);
                        let s = format!("&mut self{}.1", ".0".repeat(idx));
                        let expr: Expr = syn::parse_str(&s).unwrap();
                        node.args.insert(0, expr);
                    }
                }
            }
        }
    }
}
