//! # Loosen
//!
//! Declares a derive-like function attr macro which creates another wrap-like
//! function with single tuple-loosened argument, retrieved from the former
//! function.
//!
//! Upon call, the loosened function propagates the call with the flattened
//! input tuple as arguments to the original function.
//!
//! ## Example
//!
//! ```
//! # extern crate loosen;
//! # use loosen::loose;
//! #
//! # pub struct A;
//! # pub struct B;
//!
//! #[loose]
//! fn fa(a: A, b: B) {}
//!
//! // normal call
//! fa(A, B);
//!
//! // loose call
//! let args = (A, B);
//! fa_loose(args);
//! // ie. instead of two arguments,
//! // there is only a single tuple argument
//!
//! // another usage exaple
//! (0..10)
//!    .map(|_| (A, B))
//!    .map(fa_loose)
//!    .collect::<Vec<_>>();
//! ```
//!
//! ## Note
//!
//! This is a draft and is my first try on proc-macros.
//! I suggested this as an rfc before realising a proc macro would suffice:
//! https://github.com/rust-lang/rfcs/issues/2667

extern crate proc_macro;
use syn;

use proc_macro::TokenStream;
use quote::quote;

/// For a function `fa(A, B)`,
/// derives `fa_loose((A, B,))` which calls `fa(A, B)`.
///
/// ie.
/// `fa_loose` has a single argument, a tuple.
/// This tuple is flattened and used as arguments to `fa`.
#[proc_macro_attribute]
pub fn loose(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    // replicates the input function
    let replica = quote! {
       #input
    };

    // extract parsed information
    let vis = &input.vis;
    let constness = &input.constness;
    let unsafety = &input.unsafety;
    let ident = &input.ident;
    let decl = &input.decl;
    let _block = &input.block;
    // -------
    // TODO: also deal with those information:
    // let attrs = &input.attrs;
    // let asyncness = &input.asyncness;
    // let abi = &input.abi;

    let generics = &decl.generics;
    let inputs = &decl.inputs;
    let output = &decl.output;
    // -------
    // TODO: also deal with this information
    // let variadic = &decl.variadic;

    let ident_loose = format!("{}_loose", ident);
    let ident_loose = syn::Ident::new(&ident_loose, ident.span());

    // iterates over FnArgs
    // and get patterns (such as idents) and types
    let (pats, types): (Vec<_>, Vec<_>) = inputs
        .iter()
        .map(|fn_arg| {
            match fn_arg {
                syn::FnArg::SelfRef(_self_ref) => {
                    panic!("TODO FnArg::SelfRef");
                }
                syn::FnArg::SelfValue(_self_value) => {
                    panic!("TODO FnArg::SelfValue");
                }
                syn::FnArg::Captured(captured) => {
                    // panic!("TODO FnArg::Captured");
                    (&captured.pat, &captured.ty)
                }
                syn::FnArg::Inferred(_pat) => {
                    panic!("TODO FnArg::Inferred");
                }
                syn::FnArg::Ignored(_ty) => {
                    panic!("TODO FnArg::Ignored");
                }
            }
        })
        .unzip();

    // given vectors of patters (such as idents) and types
    // quote a tuple definition for them
    let args_into_tuple = {
        let pats = pats.clone();
        let types = types.clone();
        quote! {
            (#(#pats,)*): (#(#types,)*)
        }
    };

    // creates a loosened wrapper for the replica
    // such that the loosened has only one parameter, a tuple,
    // and flatten that tuple as parameters for the replica call
    let loosened = quote! {
        #[inline(always)]
        #vis #constness #unsafety fn #ident_loose #generics ( #args_into_tuple ) #output {
            #ident(#(#pats),*)
        }
    };

    // println!("\n ------> ident: {}", ident.to_string());
    // println!(" ==> <  {}  >", &replica);
    // println!(" ==> <  {}  >\n", &loosened);

    let tokens = quote! {
        #replica
        #loosened
    };

    tokens.into()
}
