/*!
The [`Pod`] trait defines types which can be safely transmuted from any bit pattern.

The [`DataView`] type defines read and write data APIs to an underlying byte buffer.
*/

#![no_std]

use core::{mem, slice};
use core::marker::PhantomData;

mod data_view;
pub use self::data_view::DataView;

#[cfg(feature = "derive_pod")]
#[doc(inline)]
pub use ::derive_pod::Pod;

mod derive_pod;

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
/// When `Pod` is implemented for a user defined struct it must meet the following requirements:
///
/// * Must be annotated with `repr(C)` or `repr(transparent)`.
/// * Must have every field's type implement `Pod` itself.
/// * Must not have any padding between its fields, define dummy fields to cover the padding.
///
/// # Derive macro
///
/// To help with safely implementing this trait for structs, a proc-macro is provided to implement the `Pod` trait if the requirements are satisfied.
pub unsafe trait Pod: 'static {
	/// Returns a zero-initialized instance of the type.
	#[inline]
	fn zeroed() -> Self where Self: Sized {
		unsafe { mem::zeroed() }
	}

	/// Returns the object's memory as a byte slice.
	#[inline]
	fn as_bytes(&self) -> &[u8] {
		unsafe { slice::from_raw_parts(self as *const _ as *const u8, mem::size_of_val(self)) }
	}

	/// Returns the object's memory as a mutable byte slice.
	#[inline]
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		unsafe { slice::from_raw_parts_mut(self as *mut _ as *mut u8, mem::size_of_val(self)) }
	}

	/// Returns a data view into the object's memory.
	#[inline]
	fn as_data_view(&self) -> &DataView {
		unsafe { mem::transmute(self.as_bytes()) }
	}

	/// Returns a mutable data view into the object's memory.
	#[inline]
	fn as_data_view_mut(&mut self) -> &mut DataView {
		unsafe { mem::transmute(self.as_bytes_mut()) }
	}

	/// Safely transmutes to another type.
	///
	/// # Panics
	///
	/// This method panics if `sizeof(Self) != sizeof(T)`.
	///
	/// Ideally this method would assert the compatibility of the two types statically, unfortunately this is not currently possible.
	/// If Rust gains support for asserting this with where bounds the runtime panic may be changed to a compiletime error in the future.
	#[track_caller]
	#[inline]
	fn transmute<T: Pod>(self) -> T where Self: Sized {
		assert_eq!(mem::size_of::<Self>(), mem::size_of::<T>(), "Self must have equal size to target type");
		let result = unsafe { mem::transmute_copy(&self) };
		mem::forget(self);
		result
	}

	/// Safely transmutes references to another type.
	///
	/// # Panics
	///
	/// This method panics if `sizeof(Self) != sizeof(T)` or `alignof(Self) < alignof(T)`.
	///
	/// Ideally this method would assert the compatibility of the two types statically, unfortunately this is not currently possible.
	/// If Rust gains support for asserting this with where bounds the runtime panic may be changed to a compiletime error in the future.
	#[track_caller]
	#[inline]
	fn transmute_ref<T: Pod>(&self) -> &T where Self: Sized {
		assert_eq!(mem::size_of_val(self), mem::size_of::<T>(), "Self must have equal size to target type");
		assert!(mem::align_of_val(self) >= mem::align_of::<T>(), "Align of `Self` must be ge than `T`");
		unsafe { &*(self as *const Self as *const T) }
	}

	/// Safely transmutes references to another type.
	///
	/// # Panics
	///
	/// This method panics if `sizeof(Self) != sizeof(T)` or `alignof(Self) < alignof(T)`.
	///
	/// Ideally this method would assert the compatibility of the two types statically, unfortunately this is not currently possible.
	/// If Rust gains support for asserting this with where bounds the runtime panic may be changed to a compiletime error in the future.
	#[track_caller]
	#[inline]
	fn transmute_mut<T: Pod>(&mut self) -> &mut T where Self: Sized {
		assert_eq!(mem::size_of_val(self), mem::size_of::<T>(), "Self must have equal size to target type");
		assert!(mem::align_of_val(self) >= mem::align_of::<T>(), "Align of `Self` must be ge than `T`");
		unsafe { &mut *(self as *mut Self as *mut T) }
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
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}

#[cfg(test)]
mod tests;
