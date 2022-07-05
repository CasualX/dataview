/*!
Auto derive the `Pod` trait.

This crate should not be used directly, instead depend on the `dataview` crate with the `derive_pod` feature enabled.
*/

use proc_macro::*;

/// Derive macro for the `Pod` trait.
///
/// The type is checked for requirements of the `Pod` trait:
///
/// * Must be annotated with [`#[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
///   or [`#[repr(transparent)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent).
/// * Must have every field's type implement `Pod` itself.
/// * Must not have any padding between its fields, define dummy fields to cover the padding.
///
/// Note that it is legal for pod types to be a [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts).
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
/// * `error: cannot implement Pod for type $TYPE`
///
///   Deriving `Pod` is not supported for this type.
///
///   This includes enums, unions and structs with generics or lifetimes.
#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
	let invoke: TokenStream = "::dataview::derive_pod!".parse().unwrap();
	invoke.into_iter().chain(Some(TokenTree::Group(Group::new(Delimiter::Brace, input)))).collect()
}

/// Derive macro calculates field offsets.
///
/// The type must be a struct and must implement `Pod` or an error is raised.
///
/// The derive macro adds an associated constant `FIELD_OFFSETS` to the type.
/// `FIELD_OFFSETS` is an instance of a struct with `usize` fields for every field in the type.
/// The value of each field is the offset of that field in the type.
#[proc_macro_derive(FieldOffsets)]
pub fn field_offsets(input: TokenStream) -> TokenStream {
	let invoke: TokenStream = "::dataview::__field_offsets!".parse().unwrap();
	invoke.into_iter().chain(Some(TokenTree::Group(Group::new(Delimiter::Brace, input)))).collect()
}
