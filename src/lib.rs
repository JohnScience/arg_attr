#![doc = include_str!("../README.md")]

use proc_macro as pm;
use proc_macro2 as pm2;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    FnArg, ItemFn,
};

struct Args(Punctuated<FnArg, Comma>);

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Punctuated::<FnArg, Comma>::parse_terminated(input).map(Self)
    }
}

#[proc_macro_attribute]
pub fn args(meta: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    fn inner(meta: pm2::TokenStream, input: pm2::TokenStream) -> syn::Result<pm2::TokenStream> {
        let mut input = syn::parse2::<syn::ItemFn>(input)?;
        let Args(meta) = syn::parse2::<Args>(meta)?;
        let ItemFn {
            sig: syn::Signature { inputs, .. },
            ..
        } = &mut input;
        *inputs = meta;
        Ok(input.into_token_stream())
    }
    match inner(meta.into(), input.into()) {
        Ok(output) => output.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
