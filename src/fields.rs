use super::*;

/// Creates a typed [`struct@Field`] descriptor for a field of a container type.
///
/// The field's byte offset and type are inferred from the field access.
///
/// # Examples
///
/// ```
/// struct Header {
/// 	flags: u16,
/// 	length: u32,
/// }
///
/// let field = dataview::Field!(Header.length);
/// // `field` has type `Field<Header, u32>`.
/// ```
///
/// Nested field accesses are also supported:
///
/// ```
/// struct Inner {
/// 	value: u32,
/// }
///
/// struct Outer {
/// 	inner: Inner,
/// }
///
/// let field = dataview::Field!(Outer.inner.value);
/// // `field` has type `Field<Outer, u32>`.
/// ```
///
/// Fields used to create references must be properly aligned. In particular,
/// fields of packed structs may be unaligned and therefore cannot necessarily
/// be accessed through references, even though their offset can still be
/// described by [`struct@Field`].
///
/// # Syntax
///
/// ```text
/// Field!(ContainerType.field)
/// Field!(ContainerType.nested.field)
/// ```
#[macro_export]
macro_rules! Field {
	($($tt:tt)*) => {
		$crate::__Field!([] $($tt)*)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __Field {
	([$($ty:tt)*] . $($field:tt)+) => {
		const {
			#[inline]
			const unsafe fn __field_from_projection<ContainerT, FieldT>(
				_projection: fn(*const ContainerT) -> *const FieldT,
				offset: usize,
			) -> $crate::Field<ContainerT, FieldT> {
				unsafe { $crate::Field::<ContainerT, FieldT>(offset) }
			}

			unsafe {
				__field_from_projection(
					|base: *const $($ty)*| &raw const (*base).$($field)*,
					::core::mem::offset_of!($($ty)*, $($field)*),
				)
			}
		}
	};

	([$($ty:tt)*] $tt:tt $($tail:tt)*) => {
		$crate::__Field!([$($ty)* $tt] $($tail)*)
	};
	([$($ty:tt)*]) => {
		compile_error!("missing field access")
	};
}

/// Describes a field at the given byte offset from the start of its container.
///
/// # Safety
///
/// `offset` must be the byte offset of an actual `FieldT` field within `ContainerT`.
///
/// If this descriptor is used to create references to the field, the field
/// must also be properly aligned for `FieldT`. In particular, fields of packed
/// structs may not satisfy this requirement.
#[inline]
#[allow(non_snake_case)]
pub const unsafe fn Field<ContainerT, FieldT>(offset: usize) -> Field<ContainerT, FieldT> {
	Field { offset, marker: PhantomData }
}

/// Describes a typed field within a containing type.
///
/// The container type prevents mixing fields belonging to different types.
/// The value type allows users to determine the field's size and access its value
/// without requiring it to be naturally aligned in an underlying byte buffer.
#[repr(transparent)]
pub struct Field<ContainerT, FieldT> {
	offset: usize,
	marker: PhantomData<fn() -> (ContainerT, FieldT)>,
}
impl<ContainerT, FieldT> Field<ContainerT, FieldT> {
	/// Returns the byte offset from the start of the containing type.
	#[inline]
	pub const fn offset(self) -> usize {
		self.offset
	}
	/// Returns the byte range occupied by the field within the containing type.
	#[inline]
	pub const fn span(self) -> ops::Range<usize> {
		self.offset..self.offset + mem::size_of::<FieldT>()
	}
	/// Composes this field with a nested field.
	///
	/// The resulting field describes the nested field relative to the original containing type.
	#[inline]
	pub const fn then<NestedT>(self, rhs: Field<FieldT, NestedT>) -> Field<ContainerT, NestedT> {
		// Safety: checked with debug_assertions
		Field {
			offset: self.offset + rhs.offset,
			marker: PhantomData,
		}
	}
}

impl<ContainerT, FieldT> Copy for Field<ContainerT, FieldT> {}
impl<ContainerT, FieldT> Clone for Field<ContainerT, FieldT> {
	#[inline]
	fn clone(&self) -> Self {
		*self
	}
}

impl<ContainerT, FieldT> Eq for Field<ContainerT, FieldT> {}
impl<ContainerT, FieldT> PartialEq for Field<ContainerT, FieldT> {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.offset == other.offset
	}
}

impl<ContainerT, FieldT, NestedT> core::ops::Add<Field<FieldT, NestedT>> for Field<ContainerT, FieldT> {
	type Output = Field<ContainerT, NestedT>;

	#[inline]
	fn add(self, rhs: Field<FieldT, NestedT>) -> Field<ContainerT, NestedT> {
		// Safety: checked with debug_assertions
		Field {
			offset: self.offset + rhs.offset,
			marker: PhantomData,
		}
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __field_offsets {
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
		#[allow(non_upper_case_globals)]
		impl $name {
			$(
				$field_vis const $field_name: $crate::Field<Self, $field_ty> =
					unsafe { $crate::Field(::core::mem::offset_of!($name, $field_name)) };
			)*
		}
	};
}
