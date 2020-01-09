/*!
The [`Pod` trait](trait.Pod.html) defines types which can be safely transmuted from any bit pattern.

The [`DataView` type](struct.DataView.html) defines read and write data APIs to an underlying buffer.
 */

#![no_std]

#![cfg_attr(feature = "nightly", feature(const_generics))]

use core::{mem, slice};
use core::marker::PhantomData;

mod data_view;
pub use self::data_view::DataView;

#[cfg(feature = "derive_pod")]
#[doc(inline)]
pub use derive_pod::Pod;

/// Defines types which can be safely transmuted from any bit pattern.
///
/// # Examples
///
/// ```
/// use dataview::Pod;
///
/// #[derive(Pod)]
/// #[repr(C)]
/// struct MyType {
/// 	field: i32,
/// }
///
/// // Construct a zero initialized instance.
/// let mut inst = MyType::zeroed();
/// assert_eq!(inst.field, 0);
///
/// // Use the DataView interface to access the instance.
/// inst.as_data_view_mut().write(2, &255_u8);
///
/// // Returns a byte view over the instance.
/// assert_eq!(inst.as_bytes(), &[0, 0, 255, 0]);
/// ```
///
/// # Safety
///
/// It must be safe to transmute between any bit pattern (with length equal to the size of the type) and Self.
///
/// This is true for these primitive types: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64` and the raw pointer types.
/// Primitives such as `str` and `bool` are not pod because not every valid bit pattern is a valid instance of these types. Reference types are _never_ pod.
///
/// Arrays and slices of pod types are also pod themselves.
///
/// Due to limitations of stable Rust only specific array sizes implement the `Pod` trait.
/// Enable the `nightly` feature on a nightly compiler to use const generics to implement the `Pod` trait for all array types.
///
/// When `Pod` is implemented for a user defined struct it must meet the following requirements:
///
/// * Be annotated with `repr(C)` or `repr(transparent)`.
///
/// * Have every field's type implement `Pod` itself.
///
/// * Not have any padding between its fields.
///
/// # Auto derive
///
/// To help with safely implementing this trait for structs, a proc-macro is provided to implement the `Pod` trait if the requirements are satisfied.
pub unsafe trait Pod: 'static {
	/// Returns a zero-initialized instance of the type.
	#[inline(always)]
	fn zeroed() -> Self where Self: Sized {
		unsafe { mem::zeroed() }
	}
	/// Returns the object's memory as a byte slice.
	#[inline(always)]
	fn as_bytes(&self) -> &[u8] {
		unsafe { slice::from_raw_parts(self as *const _ as *const u8, mem::size_of_val(self)) }
	}
	/// Returns the object's memory as a mutable byte slice.
	#[inline(always)]
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		unsafe { slice::from_raw_parts_mut(self as *mut _ as *mut u8, mem::size_of_val(self)) }
	}
	/// Returns a data view into the object's memory.
	#[inline(always)]
	fn as_data_view(&self) -> &DataView {
		unsafe { mem::transmute(self.as_bytes()) }
	}
	/// Returns a mutable data view into the object's memory.
	#[inline(always)]
	fn as_data_view_mut(&mut self) -> &mut DataView {
		unsafe { mem::transmute(self.as_bytes_mut()) }
	}
	#[doc(hidden)]
	fn _static_assert() {}
}

unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for i128 {}
unsafe impl Pod for isize {}

unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for u128 {}
unsafe impl Pod for usize {}

unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}

unsafe impl<T: 'static> Pod for *const T {}
unsafe impl<T: 'static> Pod for *mut T {}

unsafe impl<T: 'static> Pod for PhantomData<T> {}

unsafe impl<T: Pod> Pod for [T] {}

#[cfg(not(feature = "nightly"))]
macro_rules! impl_pod_array {
	($($n:tt)*) => { $(unsafe impl<T: Pod> Pod for [T; $n] {})* };
}
#[cfg(not(feature = "nightly"))]
impl_pod_array!(
	0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
	32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64
	80 100 128 160 192 256 512 768 1024 2048 4096);

#[cfg(feature = "nightly")]
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}

/// Pod derive proc-macro implementation helper.
#[doc(hidden)]
#[macro_export]
macro_rules! derive_pod {
	(
		$(#[$meta:meta])*
		$vis:vis struct $name:ident {
			$(
				$(#[$field_meta:meta])*
				$field_vis:vis $field_name:ident: $field_ty:ty,
			)+
		}
	) => {
		unsafe impl $crate::Pod for $name
			where Self: 'static $(, $field_ty: $crate::Pod)+
		{
			#[doc(hidden)]
			fn _static_assert() {
				// Assert that the struct has no padding by instantiating the transmute function
				use ::core::mem;
				let _ = mem::transmute::<$name, [u8; 0 $(+ mem::size_of::<$field_ty>())+]>;
			}
		}
	};
}
