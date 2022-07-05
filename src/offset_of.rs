
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
/// ```
///
/// The syntax is `$ty.$field`.
///
/// No support for tuples, tuple structs or unions.
///
/// No support for projecting through multiple fields.
#[macro_export]
macro_rules! offset_of {
	($($tt:tt)*) => {
		$crate::__offset_of!([] $($tt)*)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __offset_of {
	([$($ty:tt)*] . $($field:ident)?) => {{
		type Ty = $($ty)*;
		// Assert that field exists on the type
		// This prevents auto-Deref from causing UB
		let Ty { $($field)?: _, .. };
		// Use MaybeUninit as the subject of the field offset
		let mut uninit = ::core::mem::MaybeUninit::<Ty>::uninit();
		let uninit_ptr = uninit.as_mut_ptr();
		// We've asserted that the field exists on the type
		// No Deref coercion or dereferencing a reference
		// Hope that's enough to keep the code safe
		#[allow(unused_unsafe)]
		unsafe {
			let field_ptr = ::core::ptr::addr_of_mut!((*uninit_ptr).$($field)?);
			(field_ptr as *mut u8).offset_from(uninit_ptr as *mut u8) as usize
		}
	}};
	([$($ty:tt)*] . $($field:tt)?) => {
		compile_error!("offset of tuple field not supported")
	};
	([$($ty:tt)*] $tt:tt $($tail:tt)*) => {
		$crate::__offset_of!([$($ty)* $tt] $($tail)*)
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
/// ```
///
/// The syntax is `$ty.$field`.
///
/// No support for tuples, tuple structs or unions.
///
/// No support for projecting through multiple fields.
#[macro_export]
macro_rules! span_of {
	($($tt:tt)*) => {
		$crate::__span_of!([] $($tt)*)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __span_of {
	([$($ty:tt)*] . $($field:ident)?) => {{
		type Ty = $($ty)*;
		// Assert that field exists on the type
		// This prevents auto-Deref from causing UB
		let Ty { $($field)?: _, .. };
		// Use MaybeUninit as the subject of the field offset
		let mut uninit = ::core::mem::MaybeUninit::<Ty>::uninit();
		let uninit_ptr = uninit.as_mut_ptr();
		// We've asserted that the field exists on the type
		// No Deref coercion or dereferencing a reference
		// Hope that's enough to keep the code safe
		#[allow(unused_unsafe)]
		unsafe {
			let field_ptr = ::core::ptr::addr_of_mut!((*uninit_ptr).$($field)?);
			let start = (field_ptr as *mut u8).offset_from(uninit_ptr as *mut u8) as usize;
			let end = (field_ptr.offset(1) as *mut u8).offset_from(uninit_ptr as *mut u8) as usize;
			start..end
		}
	}};
	([$($ty:tt)*] . $($field:tt)?) => {
		compile_error!("offset of tuple field not supported")
	};
	([$($ty:tt)*] $tt:tt $($tail:tt)*) => {
		$crate::__span_of!([$($ty)* $tt] $($tail)*)
	};
	([$($ty:tt)*]) => {
		compile_error!("missing field access")
	};
}

#[test]
fn nested_fields() {
	#[repr(C)]
	struct Foo<T> { byte: u8, value: T }

	assert_eq!(offset_of!(Foo<i32>.value), 4);
	assert_eq!(span_of!(Foo<i32>.value), 4..8);
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
