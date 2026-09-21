
/// Returns the offset of a field.
///
/// ```
/// #[repr(C)]
/// struct Data {
/// 	byte: u8,
/// 	float: f32,
/// }
///
/// let offset = dataview::offset_of!(Data.float);
/// assert_eq!(offset, 4);
///
/// const OFFSET: usize = dataview::offset_of!(Data.float);
/// assert_eq!(OFFSET, 4);
/// ```
///
/// The syntax is `$ty.$field`.
#[macro_export]
macro_rules! offset_of {
	($($tt:tt)*) => {
		$crate::__offset_of2!([] $($tt)*)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __offset_of2 {
	([$($ty:tt)*] . $($field:tt)*) => {
		::core::mem::offset_of!($($ty)*, $($field)*)
	};
	([$($ty:tt)*] $tt:tt $($tail:tt)*) => {
		$crate::__offset_of2!([$($ty)* $tt] $($tail)*)
	};
	([$($ty:tt)*]) => {
		compile_error!("missing field access")
	};
}

/// Returns the `start..end` offsets of a field.
///
/// ```
/// #[repr(C)]
/// struct Data {
/// 	byte: u8,
/// 	float: f32,
/// }
///
/// let span = dataview::span_of!(Data.float);
/// assert_eq!(span, 4..8);
/// assert_eq!(span.len(), 4);
///
/// const SPAN: std::ops::Range<usize> = dataview::span_of!(Data.float);
/// assert_eq!(SPAN, 4..8);
/// assert_eq!(SPAN.len(), 4);
/// ```
///
/// The syntax is `$ty.$field`.
#[macro_export]
macro_rules! span_of {
	($($tt:tt)*) => {
		$crate::__span_of!([] $($tt)*)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __span_of {
	([$($ty:tt)*] . $($field:tt)+) => {const {
		const fn field_size<T, F>(_: fn(*const T) -> *const F) -> usize {
			::core::mem::size_of::<F>()
		}

		let start = ::core::mem::offset_of!($($ty)*, $($field)+);
		let size = field_size::<$($ty)*, _>(|ptr| unsafe {
			&raw const (*ptr).$($field)+
		});

		start..start + size
	}};

	([$($ty:tt)*] $tt:tt $($tail:tt)*) => {
		$crate::__span_of!([$($ty)* $tt] $($tail)*)
	};
	([$($ty:tt)*]) => {
		compile_error!("missing field access")
	};
}

#[test]
fn tests() {
	#[repr(C)]
	struct Inner {
		byte: u8,
		value: u32,
	}

	#[repr(C)]
	struct TupleStruct(u16, u32);

	#[repr(C)]
	union Union {
		byte: u8,
		value: u32,
	}

	#[repr(C)]
	struct Everything {
		byte: u8,
		inner: Inner,
		tuple_struct: TupleStruct,
		tuple: (u16, u32),
		union: Union,
	}

	// Struct fields.
	assert_eq!(offset_of!(Everything.byte), 0);
	assert_eq!(span_of!(Everything.byte), 0..1);

	assert_eq!(offset_of!(Everything.inner), 4);
	assert_eq!(span_of!(Everything.inner), 4..12);

	assert_eq!(offset_of!(Everything.inner.byte), 4);
	assert_eq!(span_of!(Everything.inner.byte), 4..5);

	assert_eq!(offset_of!(Everything.inner.value), 8);
	assert_eq!(span_of!(Everything.inner.value), 8..12);

	// Tuple struct syntax.
	let _ = offset_of!(Everything.tuple_struct.0);
	let _ = span_of!(Everything.tuple_struct.0);
	let _ = offset_of!(Everything.tuple_struct.1);
	let _ = span_of!(Everything.tuple_struct.1);

	let _ = offset_of!(TupleStruct.0);
	let _ = span_of!(TupleStruct.0);
	let _ = offset_of!(TupleStruct.1);
	let _ = span_of!(TupleStruct.1);

	// Tuple syntax.
	let _ = offset_of!(Everything.tuple.0);
	let _ = span_of!(Everything.tuple.0);
	let _ = offset_of!(Everything.tuple.1);
	let _ = span_of!(Everything.tuple.1);

	type Tuple = (u16, u32);
	let _ = offset_of!(Tuple.0);
	let _ = span_of!(Tuple.0);
	let _ = offset_of!(Tuple.1);
	let _ = span_of!(Tuple.1);

	// Union fields.
	assert_eq!(offset_of!(Everything.union), 28);
	assert_eq!(span_of!(Everything.union), 28..32);

	assert_eq!(offset_of!(Everything.union.byte), 28);
	assert_eq!(span_of!(Everything.union.byte), 28..29);

	assert_eq!(offset_of!(Everything.union.value), 28);
	assert_eq!(span_of!(Everything.union.value), 28..32);

	assert_eq!(offset_of!(Union.byte), 0);
	assert_eq!(span_of!(Union.byte), 0..1);

	assert_eq!(offset_of!(Union.value), 0);
	assert_eq!(span_of!(Union.value), 0..4);
}

#[cfg(doc)]
/**
```compile_fail
use std::ops;
struct Target {
	target: f32,
}
struct Subject {
	field: i32,
	deref: Target,
}
impl ops::Deref for Subject {
	type Target = Target;
	fn deref(&self) -> &Target {
		&self.deref
	}
}
impl ops::DerefMut for Subject {
	fn deref_mut(&mut self) -> &mut Target {
		&mut self.deref
	}
}
let _ = dataview::offset_of!(Subject.target);
```
*/
fn deref_protection() {}
