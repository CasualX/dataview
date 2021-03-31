// Derive macro implemented in a declarative macro, because why not

#[doc(hidden)]
#[macro_export]
macro_rules! derive_pod_check_repr {
	// Terminal case: Repr attribute not found
	() => { compile_error!("Pod structs must be annotated with `#[repr(C)]` or `#[repr(transparent)]`.\nHint: If you have a `#[repr(align(N))` put it in a separate repr attribute."); };
	// Check for expected repr attributes
	(#[repr(C $($extra:tt)*)] $(#$tail:tt)*) => {};
	(#[repr(transparent)] $(#$tail:tt)*) => {};
	// Recursive step: keep looking through the other attributes
	(#[$meta:meta] $(#$tail:tt)*) => { $crate::derive_pod_check_repr!($(#$tail)*); }
}

/// Pod derive proc-macro implementation helper.
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
			),+
			$(,)?
		}
	) => {
		$crate::derive_pod_check_repr!($(#$meta)*);

		unsafe impl $crate::Pod for $name
			where Self: 'static $(, $field_ty: $crate::Pod)+
		{
			#[doc(hidden)]
			fn _static_assert() {
				// Assert that the struct has no padding by instantiating the transmute function
				// This is magic implemented by the Rust compiler when instatiating transmute
				const EXPECTED_SIZEOF: usize = 0usize $(+ ::core::mem::size_of::<$field_ty>())+;
				let _ = ::core::mem::transmute::<$name, [u8; EXPECTED_SIZEOF]>;
			}
		}
	};
	// Tuple structs
	(
		$(#$meta:tt)*
		$vis:vis struct $name:ident(
			$(
				$(#[$field_meta:meta])*
				$field_vis:vis $field_ty:ty
			),+
			$(,)?
		);
	) => {
		$crate::derive_pod_check_repr!($(#$meta)*);

		unsafe impl $crate::Pod for $name
			where Self: 'static $(, $field_ty: $crate::Pod)+
		{
			#[doc(hidden)]
			fn _static_assert() {
				// Assert that the struct has no padding by instantiating the transmute function
				// This is magic implemented by the Rust compiler when instatiating transmute
				const EXPECTED_SIZEOF: usize = 0usize $(+ ::core::mem::size_of::<$field_ty>())+;
				let _ = ::core::mem::transmute::<$name, [u8; EXPECTED_SIZEOF]>;
			}
		}
	};
}
