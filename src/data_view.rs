use core::{mem, ops, ptr, slice};
use super::*;

/// Read and write data to and from the underlying byte buffer.
///
/// # Operations
///
/// Each set of operations may support a try, panicking and unchecked variations, see below for more information.
///
/// * `read(offset)`
///
///   Reads a (potentially unaligned) value out of the view.
///
/// * `read_into(offset, dest)`
///
///   Reads a (potentially unaligned) value out of the view into the dest argument.
///
/// * `get(offset)`
///
///   Gets a reference to the data given the offset.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `get_mut(offset)`
///
///   Gets a mutable reference to the data given the offset.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `slice(offset, len)`
///
///   Gets a slice to the data given the offset and len.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `slice_mut(offset, len)`
///
///   Gets a mutable slice to the data given the offset.
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
/// If the *Try* variation returns `None` then the *Unchecked* variation invokes *Undefined Behavior*.
#[repr(transparent)]
pub struct DataView {
	bytes: [u8],
}

impl DataView {
	/// Returns a data view into the object's memory.
	#[inline]
	pub fn from<T: ?Sized + Pod>(v: &T) -> &DataView {
		unsafe { mem::transmute(bytes(v)) }
	}
	/// Returns a mutable data view into the object's memory.
	#[inline]
	pub fn from_mut<T: ?Sized + Pod>(v: &mut T) -> &mut DataView {
		unsafe { mem::transmute(bytes_mut(v)) }
	}
}

unsafe impl Pod for DataView {}

impl AsRef<[u8]> for DataView {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		&self.bytes
	}
}
impl AsMut<[u8]> for DataView {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.bytes
	}
}

impl DataView {
	/// Returns the number of bytes in the instance.
	#[inline]
	pub const fn len(&self) -> usize {
		self.bytes.len()
	}
	/// Returns the number of elements that would fit a slice starting at the given offset.
	#[inline]
	pub const fn tail_len<T>(&self, offset: usize) -> usize {
		(self.bytes.len() - offset) / mem::size_of::<T>()
	}
}

//----------------------------------------------------------------

/// Reads a (potentially unaligned) value from the view.
impl DataView {
	/// Reads a (potentially unaligned) value from the view.
	#[inline]
	pub fn try_read<T: Pod>(&self, offset: usize) -> Option<T> {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.bytes.get(index)?;
		unsafe {
			let src = bytes.as_ptr() as *const T;
			Some(ptr::read_unaligned(src))
		}
	}
	/// Reads a (potentially unaligned) value from the view.
	#[track_caller]
	#[inline]
	pub fn read<T: Pod>(&self, offset: usize) -> T {
		match self.try_read(offset) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Reads a (potentially unaligned) value from the view.
	#[inline]
	pub unsafe fn read_unchecked<T: Pod>(&self, offset: usize) -> T {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.bytes.get_unchecked(index);
		let src = bytes.as_ptr() as *const T;
		ptr::read_unaligned(src)
	}
}

//----------------------------------------------------------------

/// Reads a (potentially unaligned) value from the view into the destination.
impl DataView {
	/// Reads a (potentially unaligned) value from the view into the destination.
	#[inline]
	pub fn try_read_into<T: ?Sized + Pod>(&self, offset: usize, dest: &mut T) -> Option<()> {
		let index = offset..offset + mem::size_of_val(dest);
		let bytes = self.bytes.get(index)?;
		unsafe {
			let src = bytes.as_ptr();
			let dst = bytes_mut(dest).as_mut_ptr();
			ptr::copy_nonoverlapping(src, dst, bytes.len());
			Some(())
		}
	}
	/// Reads a (potentially unaligned) value from the view into the destination.
	#[track_caller]
	#[inline]
	pub fn read_into<T: ?Sized + Pod>(&self, offset: usize, dest: &mut T) {
		match self.try_read_into(offset, dest) {
			Some(()) => (),
			None => invalid_offset(),
		}
	}
	/// Reads a (potentially unaligned) value from the view into the destination.
	#[inline]
	pub unsafe fn read_into_unchecked<T: ?Sized + Pod>(&self, offset: usize, dest: &mut T) {
		let index = offset..offset + mem::size_of_val(dest);
		let bytes = self.bytes.get_unchecked(index);
		let src = bytes.as_ptr();
		let dst = bytes_mut(dest).as_mut_ptr();
		ptr::copy_nonoverlapping(src, dst, bytes.len());
	}
}

//----------------------------------------------------------------

/// Gets an aligned reference into the view.
impl DataView {
	/// Gets an aligned reference into the view.
	#[inline]
	pub fn try_get<T: Pod>(&self, offset: usize) -> Option<&T> {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.bytes.get(index)?;
		let unaligned_ptr = bytes.as_ptr() as *const T;
		if !is_aligned(unaligned_ptr) {
			return None;
		}
		unsafe {
			Some(&*unaligned_ptr)
		}
	}
	/// Gets an aligned reference into the view.
	#[track_caller]
	#[inline]
	pub fn get<T: Pod>(&self, offset: usize) -> &T {
		match self.try_get(offset) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Gets an aligned reference into the view.
	#[inline]
	pub unsafe fn get_unchecked<T: Pod>(&self, offset: usize) -> &T {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.bytes.get_unchecked(index);
		&*(bytes.as_ptr() as *const T)
	}
}

//----------------------------------------------------------------

/// Gets an aligned mutable reference into the view.
impl DataView {
	/// Gets an aligned mutable reference into the view.
	#[inline]
	pub fn try_get_mut<T: Pod>(&mut self, offset: usize) -> Option<&mut T> {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.bytes.get_mut(index)?;
		let unaligned_ptr = bytes.as_mut_ptr() as *mut T;
		if !is_aligned(unaligned_ptr) {
			return None;
		}
		unsafe {
			Some(&mut *unaligned_ptr)
		}
	}
	/// Gets an aligned mutable reference into the view.
	#[track_caller]
	#[inline]
	pub fn get_mut<T: Pod>(&mut self, offset: usize) -> &mut T {
		match self.try_get_mut(offset) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Gets an aligned mutable reference into the view.
	#[inline]
	pub unsafe fn get_unchecked_mut<T: Pod>(&mut self, offset: usize) -> &mut T {
		let index = offset..offset + mem::size_of::<T>();
		let bytes = self.bytes.get_unchecked_mut(index);
		&mut *(bytes.as_mut_ptr() as *mut T)
	}
}

//----------------------------------------------------------------

/// Gets an aligned slice into the view.
impl DataView {
	/// Gets an aligned slice into the view.
	#[inline]
	pub fn try_slice<T: Pod>(&self, offset: usize, len: usize) -> Option<&[T]> {
		let index = offset..offset + usize::checked_mul(len, mem::size_of::<T>())?;
		let bytes = self.bytes.get(index)?;
		let unaligned_ptr = bytes.as_ptr() as *const T;
		if !is_aligned(unaligned_ptr) {
			return None;
		}
		unsafe {
			Some(slice::from_raw_parts(unaligned_ptr, len))
		}
	}
	/// Gets an aligned slice into the view.
	#[track_caller]
	#[inline]
	pub fn slice<T: Pod>(&self, offset: usize, len: usize) -> &[T] {
		match self.try_slice(offset, len) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Gets an aligned slice into the view.
	#[inline]
	pub unsafe fn slice_unchecked<T: Pod>(&self, offset: usize, len: usize) -> &[T] {
		let index = offset..offset + len * mem::size_of::<T>();
		let bytes = self.bytes.get_unchecked(index);
		slice::from_raw_parts(bytes.as_ptr() as *const T, len)
	}
}

//----------------------------------------------------------------

/// Gets an aligned mutable slice into the view.
impl DataView {
	/// Gets an aligned mutable slice into the view.
	#[inline]
	pub fn try_slice_mut<T: Pod>(&mut self, offset: usize, len: usize) -> Option<&mut [T]> {
		let index = offset..offset + usize::checked_mul(len, mem::size_of::<T>())?;
		let bytes = self.bytes.get_mut(index)?;
		let unaligned_ptr = bytes.as_mut_ptr() as *mut T;
		if !is_aligned(unaligned_ptr) {
			return None;
		}
		unsafe {
			Some(slice::from_raw_parts_mut(unaligned_ptr, len))
		}
	}
	/// Gets an aligned mutable slice into the view.
	#[track_caller]
	#[inline]
	pub fn slice_mut<T: Pod>(&mut self, offset: usize, len: usize) -> &mut [T] {
		match self.try_slice_mut(offset, len) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
	/// Gets an aligned mutable slice into the view.
	#[inline]
	pub unsafe fn slice_unchecked_mut<T: Pod>(&mut self, offset: usize, len: usize) -> &mut [T] {
		let index = offset..offset + len * mem::size_of::<T>();
		let bytes = self.bytes.get_unchecked_mut(index);
		slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut T, len)
	}
}

//----------------------------------------------------------------

/// Writes a value into the view.
impl DataView {
	/// Writes a value into the view.
	#[inline]
	pub fn try_write<T: ?Sized + Pod>(&mut self, offset: usize, value: &T) -> Option<()> {
		let index = offset..offset + mem::size_of_val(value);
		let bytes = self.bytes.get_mut(index)?;
		bytes.copy_from_slice(crate::bytes(value));
		Some(())
	}
	/// Writes a value into the view.
	#[track_caller]
	#[inline]
	pub fn write<T: ?Sized + Pod>(&mut self, offset: usize, value: &T) {
		match self.try_write(offset, value) {
			Some(()) => (),
			None => invalid_offset(),
		}
	}
	/// Writes a value into the view.
	#[inline]
	pub unsafe fn write_unchecked<T: ?Sized + Pod>(&mut self, offset: usize, value: &T) {
		let index = offset..offset + mem::size_of_val(value);
		let bytes = self.bytes.get_unchecked_mut(index);
		ptr::copy_nonoverlapping(crate::bytes(value).as_ptr(), bytes.as_mut_ptr(), bytes.len());
	}
}

//----------------------------------------------------------------

impl DataView {
	/// Index the DataView creating a subview.
	#[inline]
	pub fn index<R: ops::RangeBounds<usize>>(&self, range: R) -> Option<&DataView> {
		let start = match range.start_bound() {
			ops::Bound::Unbounded => 0,
			ops::Bound::Included(&start) => start,
			ops::Bound::Excluded(&start) => start + 1,
		};
		let end = match range.end_bound() {
			ops::Bound::Unbounded => self.len(),
			ops::Bound::Included(&end) => end + 1,
			ops::Bound::Excluded(&end) => end,
		};
		let bytes = self.bytes.get(start..end)?;
		Some(DataView::from(bytes))
	}
	/// Index the DataView creating a mutable subview.
	#[inline]
	pub fn index_mut<R: ops::RangeBounds<usize>>(&mut self, range: R) -> Option<&mut DataView> {
		let start = match range.start_bound() {
			ops::Bound::Unbounded => 0,
			ops::Bound::Included(&start) => start,
			ops::Bound::Excluded(&start) => start + 1,
		};
		let end = match range.end_bound() {
			ops::Bound::Unbounded => self.len(),
			ops::Bound::Included(&end) => end + 1,
			ops::Bound::Excluded(&end) => end,
		};
		let bytes = self.bytes.get_mut(start..end)?;
		Some(DataView::from_mut(bytes))
	}
}

//----------------------------------------------------------------

impl<R: ops::RangeBounds<usize>> ops::Index<R> for DataView {
	type Output = DataView;
	#[track_caller]
	#[inline]
	fn index(&self, range: R) -> &DataView {
		match self.index(range) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
}
impl<R: ops::RangeBounds<usize>> ops::IndexMut<R> for DataView {
	#[track_caller]
	#[inline]
	fn index_mut(&mut self, range: R) -> &mut DataView {
		match self.index_mut(range) {
			Some(value) => value,
			None => invalid_offset(),
		}
	}
}

//----------------------------------------------------------------

#[cold]
#[track_caller]
#[inline(never)]
fn invalid_offset() -> ! {
	panic!("invalid offset")
}
