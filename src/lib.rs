use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Error, Expr, Ident, ItemFn, Lit};

fn transform(method: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let method = Ident::new(method, Span::call_site());

    let args = parse_macro_input!(args as Expr);
    let Expr::Lit(path) = args else {
        return Error::new(args.span(), "expected path to be a string literal")
            .to_compile_error()
            .into();
    };
    let Lit::Str(path) = path.lit else {
        return Error::new(path.span(), "expected path to be a string literal")
            .to_compile_error()
            .into();
    };
    let path = path.value();

    let input = parse_macro_input!(input as ItemFn);
    let fn_name = input.sig.ident.clone();

    TokenStream::from(quote! {
        fn #fn_name(router: axum::Router<crate::State>) -> axum::Router<crate::State> {
            #input
            router.route(#path, axum::routing::#method(#fn_name))
        }
    })
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    transform("delete", args, input)
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    transform("get", args, input)
}

#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    transform("patch", args, input)
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    transform("post", args, input)
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    transform("put", args, input)
}
