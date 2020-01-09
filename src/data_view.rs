use core::{mem, ptr, slice};
use super::Pod;

/// Read and write data APIs to the underlying buffer.
///
/// Construct the data view through the [`Pod` trait](trait.Pod.html#method.as_data_view).
pub struct DataView([u8]);

impl DataView {
	/// Copies a (potentially unaligned) value from the buffer.
	#[inline(always)]
	pub fn try_copy<T: Pod>(&self, offset: usize) -> Option<T> {
		unsafe {
			let slice = self.0.get(offset..offset + mem::size_of::<T>())?;
			Some(ptr::read_unaligned(slice.as_ptr() as *const T))
		}
	}
	/// Copies a (potentially unaligned) value from the buffer into the destination.
	#[inline(always)]
	pub fn try_copy_into<T: Pod + ?Sized>(&self, offset: usize, dest: &mut T) -> Option<()> {
		unsafe {
			let byte_size = mem::size_of_val(dest);
			let slice = self.0.get(offset..offset + byte_size)?;
			ptr::copy_nonoverlapping(slice.as_ptr(), dest.as_bytes_mut().as_mut_ptr(), byte_size);
			Some(())
		}
	}
	/// Reads an aligned value from the buffer.
	#[inline(always)]
	pub fn try_read<T: Pod>(&self, offset: usize) -> Option<&T> {
		unsafe {
			let slice = self.0.get(offset..offset + mem::size_of::<T>())?;
			if slice.as_ptr() as usize % mem::align_of::<T>() != 0 {
				return None;
			}
			Some(&*(slice.as_ptr() as *const T))
		}
	}
	/// Reads an aligned value from the buffer.
	#[inline(always)]
	pub fn try_read_mut<T: Pod>(&mut self, offset: usize) -> Option<&mut T> {
		unsafe {
			let slice = self.0.get_mut(offset..offset + mem::size_of::<T>())?;
			if slice.as_mut_ptr() as usize % mem::align_of::<T>() != 0 {
				return None;
			}
			Some(&mut *(slice.as_mut_ptr() as *mut T))
		}
	}
	/// Reads an aligned slice from the buffer.
	#[inline(always)]
	pub fn try_read_slice<T: Pod>(&self, offset: usize, len: usize) -> Option<&[T]> {
		unsafe {
			let byte_size = usize::checked_mul(len, mem::size_of::<T>())?;
			let slice = self.0.get(offset..offset + byte_size)?;
			if slice.as_ptr() as usize % mem::align_of::<T>() != 0 {
				return None;
			}
			Some(slice::from_raw_parts(slice.as_ptr() as *const T, len))
		}
	}
	/// Reads an aligned slice from the buffer.
	#[inline(always)]
	pub fn try_read_slice_mut<T: Pod>(&mut self, offset: usize, len: usize) -> Option<&mut [T]> {
		unsafe {
			let byte_size = usize::checked_mul(len, mem::size_of::<T>())?;
			let slice = self.0.get_mut(offset..offset + byte_size)?;
			if slice.as_mut_ptr() as usize % mem::align_of::<T>() != 0 {
				return None;
			}
			Some(slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T, len))
		}
	}
	/// Reads the largest slice from the buffer.
	#[inline(always)]
	pub fn try_read_many<T: Pod>(&self, offset: usize) -> Option<&[T]> {
		unsafe {
			let slice = self.0.get(offset..)?;
			if slice.as_ptr() as usize % mem::align_of::<T>() != 0 {
				return None;
			}
			let len = slice.len() / mem::size_of::<T>();
			Some(slice::from_raw_parts(slice.as_ptr() as *const T, len))
		}
	}
	/// Reads the largest slice from the buffer.
	#[inline(always)]
	pub fn try_read_many_mut<T: Pod>(&mut self, offset: usize) -> Option<&mut [T]> {
		unsafe {
			let slice = self.0.get_mut(offset..)?;
			if slice.as_mut_ptr() as usize % mem::align_of::<T>() != 0 {
				return None;
			}
			let len = slice.len() / mem::size_of::<T>();
			Some(slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T, len))
		}
	}
	/// Writes a value to the buffer.
	#[inline(always)]
	pub fn try_write<T: Pod + ?Sized>(&mut self, offset: usize, value: &T) -> Option<()> {
		unsafe {
			let byte_size = mem::size_of_val(value);
			let slice = self.0.get_mut(offset..offset + byte_size)?;
			ptr::copy_nonoverlapping(value.as_bytes().as_ptr(), slice.as_mut_ptr(), byte_size);
			Some(())
		}
	}

	/// Copies a (potentially unaligned) value from the buffer.
	#[inline(always)]
	pub fn copy<T: Pod>(&self, offset: usize) -> T {
		self.try_copy(offset).expect("invalid offset")
	}
	/// Copies a (potentially unaligned) value from the buffer into the destination.
	#[inline(always)]
	pub fn copy_into<T: Pod + ?Sized>(&self, offset: usize, dest: &mut T) {
		self.try_copy_into(offset, dest).expect("invalid offset")
	}
	/// Reads an aligned value from the buffer.
	#[inline(always)]
	pub fn read<T: Pod>(&self, offset: usize) -> &T {
		self.try_read(offset).expect("invalid offset")
	}
	/// Reads an aligned value from the buffer.
	#[inline(always)]
	pub fn read_mut<T: Pod>(&mut self, offset: usize) -> &mut T {
		self.try_read_mut(offset).expect("invalid offset")
	}
	/// Reads an aligned slice from the buffer.
	#[inline(always)]
	pub fn read_slice<T: Pod>(&self, offset: usize, len: usize) -> &[T] {
		self.try_read_slice(offset, len).expect("invalid offset")
	}
	/// Reads an aligned slice from the buffer.
	#[inline(always)]
	pub fn read_slice_mut<T: Pod>(&mut self, offset: usize, len: usize) -> &mut [T] {
		self.try_read_slice_mut(offset, len).expect("invalid offset")
	}
	/// Reads the largest slice from the buffer.
	#[inline(always)]
	pub fn read_many<T: Pod>(&self, offset: usize) -> &[T] {
		self.try_read_many(offset).expect("invalid offset")
	}
	/// Reads the largest slice from the buffer.
	#[inline(always)]
	pub fn read_many_mut<T: Pod>(&mut self, offset: usize) -> &mut [T] {
		self.try_read_many_mut(offset).expect("invalid offset")
	}
	/// Writes a value to the buffer.
	#[inline(always)]
	pub fn write<T: Pod + ?Sized>(&mut self, offset: usize, value: &T) {
		self.try_write(offset, value).expect("invalid offset")
	}

	/// Copies a (potentially unaligned) value from the buffer.
	#[inline(always)]
	pub unsafe fn copy_unchecked<T: Pod>(&self, offset: usize) -> T {
		let slice = self.0.get_unchecked(offset..offset + mem::size_of::<T>());
		ptr::read_unaligned(slice.as_ptr() as *const T)
	}
	/// Copies a (potentially unaligned) value from the buffer into the destination.
	#[inline(always)]
	pub unsafe fn copy_into_unchecked<T: Pod + ?Sized>(&self, offset: usize, dest: &mut T) {
		let byte_size = mem::size_of_val(dest);
		let slice = self.0.get_unchecked(offset..offset + byte_size);
		ptr::copy_nonoverlapping(slice.as_ptr(), dest.as_bytes_mut().as_mut_ptr(), byte_size);
	}
	/// Reads an aligned value from the buffer.
	#[inline(always)]
	pub unsafe fn read_unchecked<T: Pod>(&self, offset: usize) -> &T {
		let slice = self.0.get_unchecked(offset..offset + mem::size_of::<T>());
		&*(slice.as_ptr() as *const T)
	}
	/// Reads an aligned value from the buffer.
	#[inline(always)]
	pub unsafe fn read_unchecked_mut<T: Pod>(&mut self, offset: usize) -> &mut T {
		let slice = self.0.get_unchecked_mut(offset..offset + mem::size_of::<T>());
		&mut *(slice.as_mut_ptr() as *mut T)
	}
	/// Reads an aligned slice from the buffer.
	#[inline(always)]
	pub unsafe fn read_slice_unchecked<T: Pod>(&self, offset: usize, len: usize) -> &[T] {
		let byte_size = usize::wrapping_mul(len, mem::size_of::<T>());
		let slice = self.0.get_unchecked(offset..offset + byte_size);
		slice::from_raw_parts(slice.as_ptr() as *const T, len)
	}
	/// Reads an aligned slice from the buffer.
	#[inline(always)]
	pub unsafe fn read_slice_unchecked_mut<T: Pod>(&mut self, offset: usize, len: usize) -> &mut [T] {
		let byte_size = usize::wrapping_mul(len, mem::size_of::<T>());
		let slice = self.0.get_unchecked_mut(offset..offset + byte_size);
		slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T, len)
	}
	/// Reads the largest slice from the buffer.
	#[inline(always)]
	pub unsafe fn read_many_unchecked<T: Pod>(&self, offset: usize) -> &[T] {
		let slice = self.0.get_unchecked(offset..);
		let len = slice.len() / mem::size_of::<T>();
		slice::from_raw_parts(slice.as_ptr() as *const T, len)
	}
	/// Reads the largest slice from the buffer.
	#[inline(always)]
	pub unsafe fn read_many_unchecked_mut<T: Pod>(&mut self, offset: usize) -> &mut [T] {
		let slice = self.0.get_unchecked_mut(offset..);
		let len = slice.len() / mem::size_of::<T>();
		slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T, len)
	}
	/// Writes a value to the buffer.
	#[inline(always)]
	pub unsafe fn write_unchecked<T: Pod + ?Sized>(&mut self, offset: usize, value: &T) {
		let byte_size = mem::size_of_val(value);
		let slice = self.0.get_unchecked_mut(offset..offset + byte_size);
		ptr::copy_nonoverlapping(value.as_bytes().as_ptr(), slice.as_mut_ptr(), byte_size);
	}
}
