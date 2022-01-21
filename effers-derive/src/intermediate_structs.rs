use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Path};

use crate::{lette::LettersIter, Args};

pub struct IntermediateStruct {
    pub tokens: TokenStream,
    pub id: Ident,
    pub traits: Vec<Path>,
    pub letters: Vec<Ident>,
    pub generics: TokenStream,
}

impl IntermediateStruct {
    pub fn new(
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

pub fn intermediate_structs(args: &Args, prog_name: &Ident) -> Vec<IntermediateStruct> {
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
                        #[derive(Clone, Copy)]
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
