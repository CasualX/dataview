use core::{mem, ops, ptr, slice};
use super::Pod;

/// Read and write data from and to the underlying byte buffer.
///
/// Construct the data view through [`Pod::as_data_view`] or [`Pod::as_data_view_mut`].
///
/// # Operations
///
/// Each set of operations may support a try, panicking and unchecked variations, see below for more information.
///
/// * `copy(offset)`
///
///   Copies a (potentially unaligned) value out of the view.
///
/// * `copy_into(offset, dest)`
///
///    Copies a (potentially unaligned) value out of the view into the dest argument.
///
/// * `read(offset)`
///
///   Returns a reference to the data given the offset.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `read_mut(offset)`
///
///   Returns a mutable reference to the data given the offset.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `slice(offset, len)`
///
///   Returns a slice to the data given the offset and len.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `slice_mut(offset, len)`
///
///   Returns a mutable slice to the data given the offset.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `write(offset, value)`
///
///   Writes a value to the view at the given offset.
///
/// # Panics
///
/// *Panicking* methods have no prefix or suffix. They invoke the *Try* methods and panic if they return `None`.
///
/// When calling *Panicking* variation with an offset that ends up out of bounds or if the final pointer is misaligned
/// for the given type the method panics with the message `"invalid offset"`.
///
/// The relevant methods are annotated with `#[track_caller]` providing a useful location where the error happened.
///
/// # Safety
///
/// The *Unchecked* methods have the `_unchecked` suffix and simply assume the offset is correct.
/// This is *Undefined Behavior* when it results in an out of bounds read or write or if a misaligned reference is produced.
///
/// If the *Try* variation would have returned `None` then the *Unchecked* variation is *Undefined Behavior*.
pub struct DataView([u8]);

impl AsRef<[u8]> for DataView {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}
impl AsMut<[u8]> for DataView {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.0
	}
}

unsafe impl Pod for DataView {}

impl DataView {
	/// Returns the number of bytes in the instance.
	#[inline]
	pub const fn len(&self) -> usize {
		self.0.len()
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Copies a (potentially unaligned) value from the view.
	#[inline]
	pub fn try_copy<T: Pod>(&self, offset: usize) -> Option<T> {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.0.get(index)?;
		unsafe {
			let src = bytes.as_ptr() as *const T;
			Some(ptr::read_unaligned(src))
		}
	}
	/// Copies a (potentially unaligned) value from the view.
	#[track_caller]
	#[inline]
	pub fn copy<T: Pod>(&self, offset: usize) -> T {
		match self.try_copy(offset) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Copies a (potentially unaligned) value from the view.
	#[inline]
	pub unsafe fn copy_unchecked<T: Pod>(&self, offset: usize) -> T {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.0.get_unchecked(index);
		let src = bytes.as_ptr() as *const T;
		ptr::read_unaligned(src)
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Copies a (potentially unaligned) value from the view into the destination.
	#[inline]
	pub fn try_copy_into<T: Pod + ?Sized>(&self, offset: usize, dest: &mut T) -> Option<()> {
		let index = offset..offset + mem::size_of_val(dest);
		let bytes = self.0.get(index)?;
		unsafe {
			let src = bytes.as_ptr();
			let dst = dest.as_bytes_mut().as_mut_ptr();
			ptr::copy_nonoverlapping(src, dst, bytes.len());
			Some(())
		}
	}
	/// Copies a (potentially unaligned) value from the view into the destination.
	#[track_caller]
	#[inline]
	pub fn copy_into<T: Pod + ?Sized>(&self, offset: usize, dest: &mut T) {
		match self.try_copy_into(offset, dest) {
			Some(()) => (),
			None => invalid_offset(),
		}
	}
	/// Copies a (potentially unaligned) value from the view into the destination.
	#[inline]
	pub unsafe fn copy_into_unchecked<T: Pod + ?Sized>(&self, offset: usize, dest: &mut T) {
		let index = offset..offset + mem::size_of_val(dest);
		let bytes = self.0.get_unchecked(index);
		let src = bytes.as_ptr();
		let dst = dest.as_bytes_mut().as_mut_ptr();
		ptr::copy_nonoverlapping(src, dst, bytes.len());
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Reads an aligned value from the view.
	#[inline]
	pub fn try_read<T: Pod>(&self, offset: usize) -> Option<&T> {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.0.get(index)?;
		if bytes.as_ptr() as usize % mem::align_of::<T>() != 0 {
			return None;
		}
		unsafe {
			Some(&*(bytes.as_ptr() as *const T))
		}
	}
	/// Reads an aligned value from the view.
	#[track_caller]
	#[inline]
	pub fn read<T: Pod>(&self, offset: usize) -> &T {
		match self.try_read(offset) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Reads an aligned value from the view.
	#[inline]
	pub unsafe fn read_unchecked<T: Pod>(&self, offset: usize) -> &T {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.0.get_unchecked(index);
		&*(bytes.as_ptr() as *const T)
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Reads an aligned value from the view.
	#[inline]
	pub fn try_read_mut<T: Pod>(&mut self, offset: usize) -> Option<&mut T> {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.0.get_mut(index)?;
		if bytes.as_mut_ptr() as usize % mem::align_of::<T>() != 0 {
			return None;
		}
		unsafe {
			Some(&mut *(bytes.as_mut_ptr() as *mut T))
		}
	}
	/// Reads an aligned value from the view.
	#[track_caller]
	#[inline]
	pub fn read_mut<T: Pod>(&mut self, offset: usize) -> &mut T {
		match self.try_read_mut(offset) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Reads an aligned value from the view.
	#[inline]
	pub unsafe fn read_unchecked_mut<T: Pod>(&mut self, offset: usize) -> &mut T {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.0.get_unchecked_mut(index);
		&mut *(bytes.as_mut_ptr() as *mut T)
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Reads an aligned slice from the view.
	#[inline]
	pub fn try_slice<T: Pod>(&self, offset: usize, len: usize) -> Option<&[T]> {
		let index = offset..offset + usize::checked_mul(len, mem::size_of::<T>())?;
		let bytes = self.0.get(index)?;
		if bytes.as_ptr() as usize % mem::align_of::<T>() != 0 {
			return None;
		}
		unsafe {
			Some(slice::from_raw_parts(bytes.as_ptr() as *const T, len))
		}
	}
	/// Reads an aligned slice from the view.
	#[track_caller]
	#[inline]
	pub fn slice<T: Pod>(&self, offset: usize, len: usize) -> &[T] {
		match self.try_slice(offset, len) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Reads an aligned slice from the view.
	#[inline]
	pub unsafe fn slice_unchecked<T: Pod>(&self, offset: usize, len: usize) -> &[T] {
		let index = offset..offset + usize::wrapping_mul(len, mem::size_of::<T>());
		let bytes = self.0.get_unchecked(index);
		slice::from_raw_parts(bytes.as_ptr() as *const T, len)
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Reads an aligned slice from the view.
	#[inline]
	pub fn try_slice_mut<T: Pod>(&mut self, offset: usize, len: usize) -> Option<&mut [T]> {
		let index = offset..offset + usize::checked_mul(len, mem::size_of::<T>())?;
		let bytes = self.0.get_mut(index)?;
		if bytes.as_mut_ptr() as usize % mem::align_of::<T>() != 0 {
			return None;
		}
		unsafe {
			Some(slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut T, len))
		}
	}
	/// Reads an aligned slice from the view.
	#[track_caller]
	#[inline]
	pub fn slice_mut<T: Pod>(&mut self, offset: usize, len: usize) -> &mut [T] {
		match self.try_slice_mut(offset, len) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Reads an aligned slice from the view.
	#[inline]
	pub unsafe fn slice_unchecked_mut<T: Pod>(&mut self, offset: usize, len: usize) -> &mut [T] {
		let index = offset..offset + usize::wrapping_mul(len, mem::size_of::<T>());
		let bytes = self.0.get_unchecked_mut(index);
		slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut T, len)
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Returns the number of elements that would fit a slice starting at the given offset.
	#[inline]
	pub const fn tail_len<T>(&self, offset: usize) -> usize {
		(self.0.len() - offset) / mem::size_of::<T>()
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Writes a value to the view.
	#[inline]
	pub fn try_write<T: Pod + ?Sized>(&mut self, offset: usize, value: &T) -> Option<()> {
		let index = offset..offset + mem::size_of_val(value);
		let bytes = self.0.get_mut(index)?;
		bytes.copy_from_slice(value.as_bytes());
		Some(())
	}
	/// Writes a value to the view.
	#[track_caller]
	#[inline]
	pub fn write<T: Pod + ?Sized>(&mut self, offset: usize, value: &T) {
		match self.try_write(offset, value) {
			Some(()) => (),
			None => invalid_offset(),
		}
	}
	/// Writes a value to the view.
	#[inline]
	pub unsafe fn write_unchecked<T: Pod + ?Sized>(&mut self, offset: usize, value: &T) {
		let index = offset..offset + mem::size_of_val(value);
		let bytes = self.0.get_unchecked_mut(index);
		ptr::copy_nonoverlapping(value.as_bytes().as_ptr(), bytes.as_mut_ptr(), bytes.len());
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Index the DataView creating a subview.
	#[inline]
	pub fn index<R: ops::RangeBounds<usize>>(&self, range: R) -> Option<&DataView> {
		let start = match range.start_bound() {
			ops::Bound::Unbounded => 0,
			ops::Bound::Included(start) => *start,
			ops::Bound::Excluded(start) => *start + 1,
		};
		let end = match range.end_bound() {
			ops::Bound::Unbounded => self.len(),
			ops::Bound::Included(end) => *end + 1,
			ops::Bound::Excluded(end) => *end,
		};
		let bytes = self.0.get(start..end)?;
		Some(bytes.as_data_view())
	}
	/// Index the DataView creating a mutable subview.
	#[inline]
	pub fn index_mut<R: ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut DataView> {
		let start = match range.start_bound() {
			ops::Bound::Unbounded => 0,
			ops::Bound::Included(start) => *start,
			ops::Bound::Excluded(start) => *start + 1,
		};
		let end = match range.end_bound() {
			ops::Bound::Unbounded => self.len(),
			ops::Bound::Included(end) => *end + 1,
			ops::Bound::Excluded(end) => *end,
		};
		let bytes = self.0.get_mut(start..end)?;
		Some(bytes.as_data_view_mut())
	}
}

//----------------------------------------------------------------

impl<R: ops::RangeBounds<usize>> ops::Index<R> for DataView {
	type Output = DataView;

	#[inline]
	fn index(&self, range: R) -> &DataView {
		match self.index(range) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
}
impl<R: ops::RangeBounds<usize>> ops::IndexMut<R> for DataView {
	#[inline]
	fn index_mut(&mut self, range: R) -> &mut DataView {
		match self.index_mut(range) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
}

//----------------------------------------------------------------

#[doc(hidden)]
impl DataView {
	/// Reads the largest slice from the view.
	#[inline]
	pub fn try_slice_tail<T: Pod>(&self, offset: usize) -> Option<&[T]> {
		self.try_slice(offset, self.tail_len::<T>(offset))
	}
	/// Reads the largest slice from the view.
	#[track_caller]
	#[inline]
	pub fn slice_tail<T: Pod>(&self, offset: usize) -> &[T] {
		match self.try_slice_tail(offset) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Reads the largest slice from the view.
	#[inline]
	pub unsafe fn slice_tail_unchecked<T: Pod>(&self, offset: usize) -> &[T] {
		self.slice_unchecked(offset, self.tail_len::<T>(offset))
	}

	/// Reads the largest slice from the view.
	#[inline]
	pub fn try_slice_tail_mut<T: Pod>(&mut self, offset: usize) -> Option<&mut [T]> {
		self.try_slice_mut(offset, self.tail_len::<T>(offset))
	}
	/// Reads the largest slice from the view.
	#[track_caller]
	#[inline]
	pub fn slice_tail_mut<T: Pod>(&mut self, offset: usize) -> &mut [T] {
		match self.try_slice_tail_mut(offset) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Reads the largest slice from the view.
	#[inline]
	pub unsafe fn slice_tail_unchecked_mut<T: Pod>(&mut self, offset: usize) -> &mut [T] {
		self.slice_unchecked_mut(offset, self.tail_len::<T>(offset))
	}
}

//----------------------------------------------------------------

#[cold]
#[track_caller]
#[inline(never)]
fn invalid_offset() -> ! {
	panic!("invalid offset")
}
