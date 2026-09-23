/*!
Auto derive the `Pod` trait.

This crate should not be used directly, instead depend on the `dataview` crate with the `derive_pod` feature enabled.
*/

use proc_macro::*;

#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
	let invoke: TokenStream = "::dataview::derive_pod!".parse().unwrap();
	invoke.into_iter().chain(Some(TokenTree::Group(Group::new(Delimiter::Brace, input)))).collect()
}

#[proc_macro_derive(FieldOffsets)]
pub fn field_offsets(input: TokenStream) -> TokenStream {
	let invoke: TokenStream = "::dataview::__field_offsets!".parse().unwrap();
	invoke.into_iter().chain(Some(TokenTree::Group(Group::new(Delimiter::Brace, input)))).collect()
}
