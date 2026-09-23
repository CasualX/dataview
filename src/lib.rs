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

#[macro_use]
mod embed;

pub use dataview_1_1::Pod;

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

// Strict provenance approved way of checking raw pointer alignment without exposing the pointer
fn is_aligned<T>(ptr: *const T) -> bool {
	let addr: usize = ptr.addr();
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
