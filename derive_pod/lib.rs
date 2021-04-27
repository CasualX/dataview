/*!
Auto derive the `Pod` trait.

This crate should not be used directly, instead depend on the `dataview` crate with the `derive_pod` feature enabled.
*/

use proc_macro::*;

/// Derive macro for the `Pod` trait.
///
/// The type is checked for requirements of the `Pod` trait:
///
/// * Must be annotated with `repr(C)` or `repr(transparent)`.
/// * Must have every field's type implement `Pod` itself.
/// * Must not have any padding between its fields, define dummy fields to cover the padding.
///
/// # Compile errors
///
/// Error reporting is not very ergonomic due to how errors are detected:
///
/// * `error[E0277]: the trait bound $TYPE: Pod is not satisfied`
///
///   The struct contains a field whose type does not implement `Pod`.
///
/// * `error[E0512]: cannot transmute between types of different sizes, or dependently-sized types`
///
///   This error means your struct has padding as its size is not equal to a byte array of length equal to the sum of the size of its fields.
///
/// * `error: no rules expected the token <`
///
///   The struct contains generic parameters which are not supported.
///   It may still be possible to manually implement `Pod` but extra care should be taken to ensure its invariants are upheld.
///
/// * `error: no rules expected the token enum`  
///   `error: no rules expected the token ;`
///
///   Deriving `Pod` implementations for enums, unit structs are not supported.
///
#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
	let invoke: TokenStream = "::dataview::derive_pod!".parse().unwrap();
	invoke.into_iter().chain(Some(TokenTree::Group(Group::new(Delimiter::Brace, input)))).collect()
}
