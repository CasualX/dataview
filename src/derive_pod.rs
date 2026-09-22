// Derive macro implemented in a macro by example, because why not

#[doc(hidden)]
#[macro_export]
macro_rules! derive_pod_check_attrs {
	// Terminal case: Repr attribute not found
	() => {
		compile_error!("missing repr: `Pod` structs must be annotated with `#[repr(C)]` or `#[repr(transparent)]`");
	};
	// Check for expected repr attributes
	(#[repr(transparent $($reprs:tt)*)] $($tail:tt)*) => {};
	(#[repr(C $($reprs:tt)*)] $($tail:tt)*) => {};
	(#[repr($token:tt $($reprs:tt)*)] $($tail:tt)*) => {
		$crate::derive_pod_check_attrs!(#[repr($($reprs)*)] $($tail)*);
	};
	(#[repr()] $($tail:tt)*) => {
		$crate::derive_pod_check_attrs!($($tail)*);
	};
	// Keep looking through the other attributes
	(#[$meta:meta] $($tail:tt)*) => {
		$crate::derive_pod_check_attrs!($($tail)*);
	};
}

/// Derive macro for the [`Pod`](crate::Pod) trait.
///
/// The type is checked for the requirements of the `Pod` trait:
///
/// * Must be annotated with [`#[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
///   or [`#[repr(transparent)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent).
/// * Must have every field's type implement `Pod` itself.
/// * Must not have any padding between its fields; define dummy fields to cover padding.
/// * Must not require dropping, including through any of its fields.
/// * Must not contain interior mutability.
///
/// Note that it is legal for pod types to be a
/// [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts).
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
///   The struct has padding: its size differs from a byte array whose length is
///   the sum of the sizes of its fields.
///
/// * `error: cannot implement Pod for type $TYPE`
///
///   Deriving `Pod` is not supported for this type. This includes enums, unions,
///   and structs with generics or lifetimes.
#[macro_export]
macro_rules! Pod {
	// Regular, non generic structs
	derive() (
		$(#$meta:tt)*
		$vis:vis struct $name:ident {
			$(
				$(#[$field_meta:meta])*
				$field_vis:vis $field_name:ident: $field_ty:ty
			),*
			$(,)?
		}
	) => {
		$crate::derive_pod_check_attrs!($(#$meta)*);

		#[automatically_derived]
		unsafe impl $crate::Pod for $name
			where Self: 'static $(, $field_ty: $crate::Pod)* {}

		const _: () = {
			// Assert that the struct has no padding by instantiating the transmute function
			// This is magic implemented by the Rust compiler when instatiating transmute
			const LEN: usize = 0usize $(+ ::core::mem::size_of::<$field_ty>())*;
			let _ = ::core::mem::transmute::<$name, [u8; LEN]>;
			// Assert that the type does not implement Drop
			assert!(!::core::mem::needs_drop::<$name>());
		};
	};

	// Tuple structs
	derive() (
		$(#$meta:tt)*
		$vis:vis struct $name:ident$((
			$(
				$(#[$field_meta:meta])*
				$field_vis:vis $field_ty:ty
			),*
			$(,)?
		))?;
	) => {
		$crate::derive_pod_check_attrs!($(#$meta)*);

		#[automatically_derived]
		unsafe impl $crate::Pod for $name
			where Self: 'static $($(, $field_ty: $crate::Pod)*)? {}

		const _: () = {
			// Assert that the struct has no padding by instantiating the transmute function
			// This is magic implemented by the Rust compiler when instatiating transmute
			const LEN: usize = 0usize $($(+ ::core::mem::size_of::<$field_ty>())*)?;
			let _ = ::core::mem::transmute::<$name, [u8; LEN]>;
			// Assert that the type does not implement Drop
			assert!(!::core::mem::needs_drop::<$name>());
		};
	};

	// Invalid cases
	(@derive $(#$meta:tt)* $vis:vis enum $name:ident $($tail:tt)*) => {
		compile_error!(concat!("cannot implement `Pod` for type `", stringify!($name), "`: enums are not allowed"));
	};
	(@derive $(#$meta:tt)* $vis:vis struct $name:ident < $($tail:tt)*) => {
		compile_error!(concat!("cannot implement `Pod` for type `", stringify!($name), "`: generics or lifetimes are not allowed"));
	};
	(@derive $(#$meta:tt)* $vis:vis union $name:ident $($tail:tt)*) => {
		compile_error!(concat!("cannot implement `Pod` for type `", stringify!($name), "`: unions are not allowed"));
	};
}

/// The derive rejects types that require dropping.
///
/// ```compile_fail
/// #![feature(macro_derive)]
///
/// #[derive(dataview::Pod)]
/// #[repr(C)]
/// struct Resource(u8);
///
/// impl Drop for Resource {
/// 	fn drop(&mut self) {}
/// }
/// ```
#[cfg(doc)]
fn reject_drop() {}
