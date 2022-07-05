/*!
The [`Pod` trait](Pod) marks types whose values can be safely transmuted between byte arrays of the same size.

The [`DataView` type](DataView) defines read and write data APIs to an underlying byte buffer.

# Examples

```
#[derive(dataview::Pod)]
#[repr(C)]
struct MyType {
	field: i32,
}

// Construct a zero initialized instance
let mut inst: MyType = dataview::zeroed();
assert_eq!(inst.field, 0);

// Use DataView to access the instance
let view = dataview::DataView::from_mut(&mut inst);
view.write(2, &255_u8);

// Create a byte view over the instance
assert_eq!(dataview::bytes(&inst), &[0, 0, 255, 0]);
```
*/

#![no_std]

use core::{mem, slice};
use core::marker::PhantomData;

mod data_view;
pub use self::data_view::DataView;

#[cfg(feature = "derive_pod")]
#[doc(inline)]
pub use ::derive_pod::Pod;

#[cfg(feature = "derive_pod")]
#[doc(hidden)]
pub use ::derive_pod::FieldOffsets;

mod derive_pod;
mod field_offsets;
mod offset_of;

/// Types whose values can be safely transmuted between byte arrays of the same size.
///
/// # Safety
///
/// It must be safe to transmute between any byte array (with length equal to the size of the type) and `Self`.
///
/// This is true for these primitive types: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`.
/// The raw pointer types are not pod under strict provenance rules but can be through the 'int2ptr' feature.
/// Primitives such as `str` and `bool` are not pod because not every valid byte pattern is a valid instance of these types.
/// References or types with lifetimes are _never_ pod.
///
/// Arrays and slices of pod types are also pod themselves.
///
/// Note that it is legal for pod types to be a [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts).
///
/// When `Pod` is implemented for a user defined type it must meet the following requirements:
///
/// * Must be annotated with [`#[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
///   or [`#[repr(transparent)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent).
/// * Must have every field's type implement `Pod` itself.
/// * Must not have any padding between its fields, define dummy fields to cover the padding.
///
/// # Derive macro
///
/// To help with safely implementing this trait for user defined types, a [derive macro](derive@Pod) is provided to implement the `Pod` trait if the requirements are satisfied.
pub unsafe trait Pod: 'static {}

/// Returns a zero-initialized instance of the type.
///
/// ```
/// let v: i32 = dataview::zeroed();
/// assert_eq!(v, 0);
/// ```
#[inline]
pub fn zeroed<T: Pod>() -> T {
	unsafe { mem::MaybeUninit::zeroed().assume_init() }
}

/// Returns the object's memory as a byte slice.
///
/// ```
/// let v = 0xcdcdcdcd_u32;
/// assert_eq!(dataview::bytes(&v), &[0xcd, 0xcd, 0xcd, 0xcd]);
/// ```
#[inline]
pub fn bytes<T: ?Sized + Pod>(src: &T) -> &[u8] {
	unsafe { slice::from_raw_parts(src as *const _ as *const u8, mem::size_of_val(src)) }
}

/// Returns the object's memory as a mutable byte slice.
#[inline]
pub fn bytes_mut<T: ?Sized + Pod>(src: &mut T) -> &mut [u8] {
	unsafe { slice::from_raw_parts_mut(src as *mut _ as *mut u8, mem::size_of_val(src)) }
}

/// Helper trait to provide methods directly on the pod types.
///
/// Do not use this trait in any signatures, use [`Pod`] directly instead.
/// There's a blanket impl that provides these methods for all pod types.
pub trait PodMethods {
	/// Returns a zero-initialized instance of the type.
	fn zeroed() -> Self where Self: Sized;
	/// Returns the object's memory as a byte slice.
	fn as_bytes(&self) -> &[u8];
	/// Returns the object's memory as a mutable byte slice.
	fn as_bytes_mut(&mut self) -> &mut [u8];
	/// Returns a data view into the object's memory.
	fn as_data_view(&self) -> &DataView;
	/// Returns a mutable data view into the object's memory.
	fn as_data_view_mut(&mut self) -> &mut DataView;
}

impl<T: ?Sized + Pod> PodMethods for T {
	#[inline]
	fn zeroed() -> T where T: Sized {
		zeroed()
	}
	#[inline]
	fn as_bytes(&self) -> &[u8] {
		bytes(self)
	}
	#[inline]
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		bytes_mut(self)
	}
	#[inline]
	fn as_data_view(&self) -> &DataView {
		DataView::from(self)
	}
	#[inline]
	fn as_data_view_mut(&mut self) -> &mut DataView {
		DataView::from_mut(self)
	}
}

unsafe impl Pod for () {}

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

#[cfg(feature = "int2ptr")]
unsafe impl<T: 'static> Pod for *const T {}
#[cfg(feature = "int2ptr")]
unsafe impl<T: 'static> Pod for *mut T {}

unsafe impl<T: 'static> Pod for PhantomData<T> {}

unsafe impl<T: Pod> Pod for [T] {}
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}

// Strict provenance approved way of checking raw pointer alignment without exposing the pointer
const fn is_aligned<T>(ptr: *const T) -> bool {
	let addr: usize = unsafe { mem::transmute(ptr) };
	addr % mem::align_of::<T>() == 0
}

#[cfg(test)]
mod tests;

#[cfg(doc)]
#[doc = include_str!("../readme.md")]
fn readme() {}

/// Reveals the evaluated value of a constant expression.
///
/// The result is a compiletime error: `expected an array with a fixed size of 0 elements, found one with N elements` where `N` is the value of the constant expression.
///
/// ```compile_fail
/// struct Foo {
/// 	field1: i8,
/// 	field2: u16,
/// }
///
/// dataview::reveal_const!(std::mem::size_of::<Foo>());
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! reveal_const {
	($e:expr) => {
		const _: [(); 0] = [(); $e];
	};
}
