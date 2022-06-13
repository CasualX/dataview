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

#[doc(hidden)]
#[macro_export]
macro_rules! derive_pod {
	// Regular, non generic structs
	(
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

		unsafe impl $crate::Pod for $name
			where Self: 'static $(, $field_ty: $crate::Pod)* {}

		const _: () = {
			// Assert that the struct has no padding by instantiating the transmute function
			// This is magic implemented by the Rust compiler when instatiating transmute
			const LEN: usize = 0usize $(+ ::core::mem::size_of::<$field_ty>())*;
			let _ = ::core::mem::transmute::<$name, [u8; LEN]>;
		};
	};

	// Tuple structs
	(
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

		unsafe impl $crate::Pod for $name
			where Self: 'static $($(, $field_ty: $crate::Pod)*)? {}

		const _: () = {
			// Assert that the struct has no padding by instantiating the transmute function
			// This is magic implemented by the Rust compiler when instatiating transmute
			const LEN: usize = 0usize $($(+ ::core::mem::size_of::<$field_ty>())*)?;
			let _ = ::core::mem::transmute::<$name, [u8; LEN]>;
		};
	};

	// Invalid cases
	($(#$meta:tt)* $vis:vis enum $name:ident $($tail:tt)*) => {
		compile_error!(concat!("cannot implement `Pod` for type `", stringify!($name), "`: enums are not allowed"));
	};
	($(#$meta:tt)* $vis:vis struct $name:ident < $($tail:tt)*) => {
		compile_error!(concat!("cannot implement `Pod` for type `", stringify!($name), "`: generics or lifetimes are not allowed"));
	};
	($(#$meta:tt)* $vis:vis union $name:ident $($tail:tt)*) => {
		compile_error!(concat!("cannot implement `Pod` for type `", stringify!($name), "`: unions are not allowed"));
	};
}
