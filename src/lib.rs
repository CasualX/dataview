/*!
Provides utilities for working with plain data as its in-memory byte representation.

The central [`Pod`] trait marks types for which every possible byte pattern is a valid value.
This makes it sound to zero-initialize them, view them as bytes, fill them from raw storage, and copy typed values to and from byte buffers.

Typical uses include binary file data, memory-mapped structures, network or device buffers, and memory read from another process.

# Plain data

Deriving [`Pod`] checks that a struct has a suitable representation, contains only `Pod` fields, has no padding, and does not require dropping.

A `Pod` value can be created zero-initialized and exposed directly as bytes:

```rust
#[derive(dataview::Pod)]
#[repr(C)]
struct Header {
	magic: u32,
	version: u16,
	flags: u16,
	count: u32,
}

let mut header: Header = dataview::zeroed();

// A file, device, or process-memory API can fill the value directly.
let destination: &mut [u8] = dataview::bytes_mut(&mut header);

// And a populated value can be passed back to an API expecting bytes.
let source: &[u8] = dataview::bytes(&header);
```

No serialization or conversion takes place: these functions expose the value's native in-memory representation.

# Byte buffers

[`DataView`] is useful when the underlying storage is a byte buffer rather than a single typed value. It reads and writes `Pod` values at byte offsets:

```rust
let buffer = [0u8; 64];
let view = dataview::DataView::from(&buffer[..]);

let flags = view.read::<u16>(6);
let count = view.read::<u32>(8);
```

`read` and `write` support potentially unaligned values. Operations returning references or slices additionally require suitable alignment.

# Typed fields

[`struct@Field`] is a low-level building block for APIs that operate on structured byte buffers.

A `Field<Container, T>` identifies a field by both its byte offset and its type. [`Field!`] creates one from ordinary Rust field syntax.

This becomes useful when the rules for accessing a field depend on the underlying data. For example, an on-disk structure may begin with its own size so that newer versions can append fields while remaining compatible with older records:

```rust
#[repr(C)]
struct Record {
	size: u32,
	flags: u32,
	timestamp: u64,
}

const FLAGS: dataview::Field<Record, u32> = dataview::Field!(Record.flags);

struct RecordView {
	view: dataview::DataView,
}

impl RecordView {
	fn get<T: dataview::Pod>(&self, field: dataview::Field<Record, T>) -> Option<T> {
		let size = self.view.try_read::<u32>(0)? as usize;

		if field.span().end > size {
			return None;
		}

		self.view.try_read(field.offset())
	}
}
```

The application can describe the latest `Record` layout once, while `RecordView` decides which fields are actually present in a particular buffer.

# Representation

`dataview` works with native memory representations; it is not a serialization format. Multi-byte values therefore use the target platform's native endianness, and user-defined `Pod` types must have a stable layout suitable for byte reinterpretation.

*/

#![no_std]

use core::{fmt, mem, ops, ptr, slice};
use core::marker::PhantomData;

mod data_view;
pub use self::data_view::DataView;

mod arch;

/// Derive macro for the `Pod` trait.
///
/// The type is checked for requirements of the `Pod` trait:
///
/// * Must be annotated with [`#[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
///   or [`#[repr(transparent)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent).
/// * Must have every field's type implement `Pod` itself.
/// * Must not have any padding between its fields, define dummy fields to cover the padding.
/// * Must not require dropping, including through any of its fields.
/// * Must not contain interior mutability.
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
#[cfg(feature = "derive_pod")]
#[doc(inline)]
pub use ::derive_pod::Pod;

/// Derive macro calculates field offsets.
///
/// The type must be a struct with named fields.
///
/// For every field, the derive macro adds an associated constant with the same
/// name and visibility to the type. Each constant is a typed `Field` descriptor
/// containing the byte offset of that field in the type.
#[cfg(feature = "derive_pod")]
#[doc(inline)]
pub use ::derive_pod::FieldOffsets as Fields;

mod derive_pod;
mod offset_of;

mod fields;
pub use self::fields::*;

#[macro_use]
mod embed;

/// Types whose values can be safely transmuted between byte arrays of the same size.
///
/// # Safety
///
/// It must be safe to transmute between any byte array (with length equal to the size of the type) and `Self`.
///
/// This is true for these primitive types: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`.
/// Raw pointer types are not pod under strict provenance rules.
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
/// * Must not require dropping, including through any of its fields.
/// * Must not contain interior mutability.
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
pub const fn zeroed<T: Pod>() -> T {
	unsafe { mem::MaybeUninit::zeroed().assume_init() }
}

/// Reinterprets the bits of a `Pod` value as another `Pod` type of the same size.
///
/// # Compile errors
///
/// Both types must implement [`Pod`]:
///
/// ```compile_fail
/// let _: u8 = dataview::transmute(false);
/// ```
///
/// They must also have the same size:
///
/// ```compile_fail
/// let _: u32 = dataview::transmute(0_u64);
/// ```
///
/// # Examples
///
/// ```
/// let bytes = [0x78, 0x56, 0x34, 0x12];
/// let value: u32 = dataview::transmute(bytes);
/// assert_eq!(value, u32::from_ne_bytes(bytes));
/// ```
#[inline]
pub const fn transmute<T: Pod, U: Pod>(value: T) -> U {
	const {
		assert!(
			mem::size_of::<T>() == mem::size_of::<U>(),
			"cannot transmute between types of different sizes",
		);
	}

	// Pod guarantees that all of `value`'s bytes are initialized and that every bit pattern is valid for `U`.
	// The assertion above guarantees that `transmute_copy` reads exactly the storage occupied by `value`.
	let value = mem::ManuallyDrop::new(value);
	unsafe { mem::transmute_copy(&value) }
}

/// Returns the object's memory as a byte slice.
///
/// ```
/// let v = 0xcdcdcdcd_u32;
/// assert_eq!(dataview::bytes(&v), &[0xcd, 0xcd, 0xcd, 0xcd]);
/// ```
#[inline]
pub const fn bytes<T: ?Sized + Pod>(value: &T) -> &[u8] {
	unsafe { slice::from_raw_parts(value as *const _ as *const u8, mem::size_of_val(value)) }
}

/// Returns the object's memory as a mutable byte slice.
#[inline]
pub const fn bytes_mut<T: ?Sized + Pod>(value: &mut T) -> &mut [u8] {
	unsafe { slice::from_raw_parts_mut(value as *mut _ as *mut u8, mem::size_of_val(value)) }
}

/// Helper trait to provide methods directly on the pod types.
///
/// Do not use this trait in any signatures, use [`Pod`] directly instead.
/// There's a blanket impl that provides these methods for all pod types.
pub trait PodMethods {
	/// Returns a zero-initialized instance of the type.
	fn zeroed() -> Self where Self: Sized;
	/// Reinterprets the bits of this value as another `Pod` type of the same size.
	fn transmute<U: Pod>(self) -> U where Self: Sized;
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
	fn transmute<U: Pod>(self) -> U where Self: Sized {
		transmute(self)
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

unsafe impl<T: 'static> Pod for PhantomData<T> {}

unsafe impl<T: Pod> Pod for [T] {}
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}

#[cfg(test)]
mod tests;

#[cfg(doc)]
#[doc = include_str!("../readme.md")]
fn readme() {}
