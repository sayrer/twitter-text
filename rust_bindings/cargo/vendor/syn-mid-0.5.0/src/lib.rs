//! Providing the features between "full" and "derive" of syn.
//!
//! This crate provides the following two unique data structures.
//!
//! * [`syn_mid::ItemFn`] -- A function whose body is not parsed.
//!
//!   ```text
//!   fn process(n: usize) -> Result<()> { ... }
//!   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^     ^
//!   ```
//!
//! * [`syn_mid::Block`] -- A block whose body is not parsed.
//!
//!   ```text
//!   { ... }
//!   ^     ^
//!   ```
//!
//! Other data structures are the same as data structures of [syn]. These are defined in this crate
//! because they cannot be used in [syn] without "full" feature.
//!
//! ## Optional features
//!
//! syn-mid in the default features aims to provide the features between "full"
//! and "derive" of [syn].
//!
//! * **`clone-impls`** — Clone impls for all syntax tree types.
//!
//! [`syn_mid::ItemFn`]: struct.ItemFn.html
//! [`syn_mid::Block`]: struct.Block.html
//! [syn]: https://github.com/dtolnay/syn

#![doc(html_root_url = "https://docs.rs/syn-mid/0.5.0")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all)]
#![allow(clippy::eval_order_dependence, clippy::large_enum_variant)]

// Many of the code contained in this crate are copies from https://github.com/dtolnay/syn.

#[macro_use]
mod macros;

mod arg;
mod pat;
mod path;

pub use self::{arg::*, pat::*};

use proc_macro2::TokenStream;
use syn::{
    punctuated::Punctuated, token, Abi, Attribute, Generics, Ident, ReturnType, Token, Variadic,
    Visibility,
};

ast_struct! {
    /// A braced block containing Rust statements.
    pub struct Block {
        pub brace_token: token::Brace,
        /// Statements in a block
        pub stmts: TokenStream,
    }
}

ast_struct! {
    /// A free-standing function: `fn process(n: usize) -> Result<()> { ...
    /// }`.
    pub struct ItemFn {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub sig: Signature,
        pub block: Box<Block>,
    }
}

ast_struct! {
    /// A function signature in a trait or implementation: `unsafe fn
    /// initialize(&self)`.
    pub struct Signature {
        pub constness: Option<Token![const]>,
        pub asyncness: Option<Token![async]>,
        pub unsafety: Option<Token![unsafe]>,
        pub abi: Option<Abi>,
        pub fn_token: Token![fn],
        pub ident: Ident,
        pub generics: Generics,
        pub paren_token: token::Paren,
        pub inputs: Punctuated<FnArg, Token![,]>,
        pub variadic: Option<Variadic>,
        pub output: ReturnType,
    }
}

mod parsing {
    use syn::{
        braced, parenthesized,
        parse::{Parse, ParseStream, Result},
        parse2, Abi, Attribute, Generics, Ident, ReturnType, Token, Type, Variadic, Visibility,
        WhereClause,
    };

    use super::{Block, FnArg, ItemFn, PatType, Signature};

    impl Parse for Block {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let content;
            Ok(Self { brace_token: braced!(content in input), stmts: content.parse()? })
        }
    }

    impl Parse for ItemFn {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let constness: Option<Token![const]> = input.parse()?;
            let asyncness: Option<Token![async]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let abi: Option<Abi> = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_terminated(FnArg::parse)?;
            let variadic = inputs.last().as_ref().and_then(get_variadic);

            #[allow(clippy::trivially_copy_pass_by_ref)]
            fn get_variadic(input: &&FnArg) -> Option<Variadic> {
                if let FnArg::Typed(PatType { ty, .. }) = input {
                    if let Type::Verbatim(tokens) = &**ty {
                        if let Ok(dots) = parse2(tokens.clone()) {
                            return Some(Variadic { attrs: Vec::new(), dots });
                        }
                    }
                }
                None
            }

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let block = input.parse()?;

            Ok(Self {
                attrs,
                vis,
                sig: Signature {
                    constness,
                    asyncness,
                    unsafety,
                    abi,
                    fn_token,
                    ident,
                    paren_token,
                    inputs,
                    output,
                    variadic,
                    generics: Generics { where_clause, ..generics },
                },
                block: Box::new(block),
            })
        }
    }
}

mod printing {
    use proc_macro2::TokenStream;
    use quote::{ToTokens, TokenStreamExt};
    use syn::{punctuated::Punctuated, Token, Type};

    use super::{Block, FnArg, ItemFn, Signature};

    impl ToTokens for Block {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.stmts.clone());
            });
        }
    }

    fn has_variadic(inputs: &Punctuated<FnArg, Token![,]>) -> bool {
        let last = match inputs.last() {
            Some(last) => last,
            None => return false,
        };

        let pat = match last {
            FnArg::Typed(pat) => pat,
            FnArg::Receiver(_) => return false,
        };

        let tokens = match pat.ty.as_ref() {
            Type::Verbatim(tokens) => tokens,
            _ => return false,
        };

        tokens.to_string() == "..."
    }

    impl ToTokens for Signature {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.constness.to_tokens(tokens);
            self.asyncness.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.abi.to_tokens(tokens);
            self.fn_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.paren_token.surround(tokens, |tokens| {
                self.inputs.to_tokens(tokens);
                if self.variadic.is_some() && !has_variadic(&self.inputs) {
                    if !self.inputs.empty_or_trailing() {
                        <Token![,]>::default().to_tokens(tokens);
                    }
                    self.variadic.to_tokens(tokens);
                }
            });
            self.output.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemFn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(&self.attrs);
            self.vis.to_tokens(tokens);
            self.sig.to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.block.stmts.clone());
            });
        }
    }
}
