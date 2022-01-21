use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{parenthesized, Ident, Path, Token};

#[derive(Debug)]
pub struct Args {
    pub name: Option<Ident>,
    pub effects: Vec<Effect>,
}

#[derive(Debug)]
pub struct Effect {
    pub name: Ident,
    pub path: Path,
    pub paren: Paren,
    pub functions: Vec<EffectFunction>,
}

#[derive(Debug)]
pub struct EffectFunction {
    pub ident: Ident,
    pub alias: Option<Ident>,
    pub mut_token: Option<Token![mut]>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = if input.peek2(Token!(=>)) {
            let name: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;
            Some(name)
        } else {
            None
        };

        let effects: Vec<Effect> = Punctuated::<Effect, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(Args { name, effects })
    }
}

impl Parse for Effect {
    fn parse(input: ParseStream) -> Result<Self> {
        let path: Path = input.parse()?;
        let content;
        let paren = parenthesized!(content in input);
        let functions = Punctuated::<EffectFunction, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        let name = (&path)
            .segments
            .last()
            .expect("There must be at least one PathSegment")
            .ident
            .clone();

        Ok(Effect {
            name,
            path,
            functions,
            paren,
        })
    }
}
impl Parse for EffectFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut_token: Option<Token![mut]> = if input.peek(Token![mut]) {
            input.parse()?
        } else {
            None
        };

        let ident = input.parse()?;

        let alias: Option<Ident> = if input.peek(Token![as]) {
            input.parse::<Token![as]>()?;
            input.parse()?
        } else {
            None
        };

        Ok(EffectFunction {
            ident,
            alias,
            mut_token,
        })
    }
}
