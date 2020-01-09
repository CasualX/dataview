/*!
Auto derive the `Pod` trait.

This crate should not be used directly, instead depend on the `dataview` crate with the `derive_pod` feature enabled.
 */

extern crate proc_macro;

use proc_macro::*;

/// Auto derive the `Pod` trait for structs.
///
/// The type is checked for requirements of the `Pod` trait:
///
/// * Be annotated with `repr(C)` or `repr(transparent)`.
///
/// * Have every field's type implement `Pod` itself.
///
/// * Not have any padding between its fields.
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
///   The struct contains generic parameters which are not supported. It may still be possible to manually implement `Pod` but extra care should be taken to ensure its invariants are upheld.
///
#[proc_macro_derive(Pod)]
pub fn pod_derive(input: TokenStream) -> TokenStream {
	return format!("::dataview::derive_pod!{{ {} }}", input).parse().unwrap()
}
