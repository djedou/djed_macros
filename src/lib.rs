//! This crate provides Djed's procedural macro `html!` which allows using JSX-like syntax
//! for generating html and the `Properties` derive macro for deriving the `Properties` trait
//! for components.
//!
//! The `html!` macro uses [proc_macro_hack](https://github.com/dtolnay/proc-macro-hack) in order
//! to be used in the expression position.
//!
//! ```
//! # #[macro_use] extern crate djed;
//! use djed::prelude::*;
//!
//! struct Component {
//!   link: ComponentLink<Self>,
//! }
//!
//! #[derive(Clone, Properties)]
//! struct Props {
//!     prop: String,
//! };
//!
//! # enum Msg { Submit }
//! #
//! # impl djed::Component for Component {
//! #     type Message = Msg;
//! #     type Properties = Props;
//! #     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn change(&mut self, props: Self::Properties) -> ShouldRender {
//! #         unimplemented!()
//! #     }
//! #
//! #     fn view(&self) -> Html {
//! #
//! // ...
//!
//! html! {
//!   <div>
//!     <button onclick=self.link.callback(|_| Msg::Submit)>
//!       { "Submit" }
//!     </button>
//!     <>
//!       <Component prop="first" />
//!       <Component prop="second" />
//!     </>
//!   </div>
//! }
//! #
//! #     }
//! # }
//! #
//! # fn main() {}
//! ```
//!


#![recursion_limit = "128"]
extern crate proc_macro;

mod derive_props;
mod html_tree;

use derive_props::DerivePropsInput;
use html_tree::{HtmlRoot, HtmlRootVNode};
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse_macro_input;

trait Peek<'a, T> {
    fn peek(cursor: Cursor<'a>) -> Option<(T, Cursor<'a>)>;
}

trait PeekValue<T> {
    fn peek(cursor: Cursor) -> Option<T>;
}

fn non_capitalized_ascii(string: &str) -> bool {
    if !string.is_ascii() {
        false
    } else if let Some(c) = string.bytes().next() {
        c.is_ascii_lowercase()
    } else {
        false
    }
}

#[proc_macro_derive(Properties, attributes(prop_or, prop_or_else, prop_or_default))]
pub fn derive_props(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DerivePropsInput);
    TokenStream::from(input.into_token_stream())
}

#[proc_macro_hack]
pub fn html_nested(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRoot);
    TokenStream::from(quote! {#root})
}

#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlRootVNode);
    TokenStream::from(quote! {#root})
}