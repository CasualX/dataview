#![allow(dead_code)]
use super::*;

#[derive(Copy, Clone)]
#[repr(C, align(8))]
struct Foo([u32; 2]);
unsafe impl Pod for Foo {}

#[repr(C, align(4))]
struct Baz([u32; 2]);
unsafe impl Pod for Baz {}

#[test]
fn test_zeroed() {
	let a: Foo = zeroed();
	assert_eq!([0u32; 2], a.0);
	let b: [f32; 2] = zeroed();
	assert_eq!([0f32; 2], b);
}

//------------------------------------------------
// DataView tests

// Align test data to reliably produce misaligned offsets for testing
static TEST_DATA: ([u64; 0], [u8; 8]) = ([], [0, 1, 2, 3, 4, 5, 6, 7]);

#[test]
fn test_basics() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);
	assert_eq!(view.len(), bytes.len());
	assert_eq!(view.as_ref(), bytes);
}

#[test]
fn test_read() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);
	for i in 0..bytes.len() {
		let value = i as u8;
		assert_eq!(value, bytes[i]);
		assert_eq!(Some(value), view.try_read(i));
		assert_eq!(value, view.read(i));
		assert_eq!(value, unsafe { view.read_unchecked(i) });
	}
	assert!(matches!(view.try_read::<u8>(view.len()), None));
}

#[test]
fn test_read_into() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);
	let mut dest: u8;
	for i in 0..bytes.len() {
		let value = i as u8;
		assert_eq!(value, bytes[i]);
		dest = !0;
		assert_eq!(Some(()), view.try_read_into(i, &mut dest));
		assert_eq!(value, dest);
		dest = !0;
		view.read_into(i, &mut dest);
		assert_eq!(value, dest);
		dest = !0;
		unsafe { view.read_into_unchecked(i, &mut dest); }
		assert_eq!(value, dest);
	}
	dest = !0;
	assert!(matches!(view.try_read_into::<u8>(view.len(), &mut dest), None));
}

#[test]
fn test_get() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);
	for i in 0..bytes.len() {
		let value = i as u8;
		assert_eq!(value, bytes[i]);
		assert_eq!(Some(&value), view.try_get(i));
		assert_eq!(&value, view.get(i));
		assert_eq!(&value, unsafe { view.get_unchecked(i) });
		if i % 2 == 1 {
			assert!(matches!(view.try_get::<u16>(i), None));
		}
	}
	assert!(matches!(view.try_get::<u8>(view.len()), None));
}

#[test]
fn test_get_mut() {
	let mut data = TEST_DATA;
	let check = TEST_DATA.1;
	let bytes = &mut data.1;
	let view = DataView::from_mut(bytes);
	for i in 0..check.len() {
		let mut value = i as u8;
		assert_eq!(value, check[i]);
		assert_eq!(Some(&mut value), view.try_get_mut(i));
		assert_eq!(&value, view.get_mut(i));
		assert_eq!(&value, unsafe { view.get_unchecked_mut(i) });
		if i % 2 == 1 {
			assert!(matches!(view.try_get_mut::<u16>(i), None));
		}
	}
	assert!(matches!(view.try_get_mut::<u8>(view.len()), None));
}

#[test]
fn test_slice() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);
	for i in 0..=bytes.len() {
		for j in i..=bytes.len() {
			let value = &bytes[i..j];
			assert_eq!(Some(value), view.try_slice(i, j - i));
			assert_eq!(value, view.slice(i, j - i));
			assert_eq!(value, unsafe { view.slice_unchecked(i, j - i) });
			if i % 2 == 1 {
				assert!(matches!(view.try_slice::<u16>(i, (j - i) / 2), None));
			}
		}
	}
	assert_eq!(view.try_slice::<u8>(view.len(), 0), Some(&[] as &[u8]));
	assert!(matches!(view.try_slice::<u8>(view.len(), 1), None));
}

#[test]
fn test_slice_mut() {
	let mut data = TEST_DATA;
	let mut check = TEST_DATA.1;
	let bytes = &mut data.1;
	let view = DataView::from_mut(bytes);
	for i in 0..=check.len() {
		for j in i..=check.len() {
			let value = &mut check[i..j];
			assert_eq!(Some(&mut *value), view.try_slice_mut(i, j - i));
			assert_eq!(value, view.slice_mut(i, j - i));
			assert_eq!(value, unsafe { view.slice_unchecked_mut(i, j - i) });
			if i % 2 == 1 {
				assert!(matches!(view.try_slice_mut::<u16>(i, (j - i) / 2), None));
			}
		}
	}
	assert_eq!(view.try_slice_mut::<u8>(check.len(), 0), Some(&mut [] as &mut [u8]));
	assert!(matches!(view.try_slice_mut::<u8>(view.len(), 1), None));
}

#[test]
fn test_try_operations_overflow() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);
	let mut dest = 0_u8;

	assert!(view.try_read::<u16>(usize::MAX).is_none());
	assert!(view.try_read_into(usize::MAX, &mut dest).is_none());
	assert!(view.try_get::<u16>(usize::MAX).is_none());
	assert!(view.try_slice::<u16>(0, usize::MAX).is_none());
	assert!(view.index((core::ops::Bound::Excluded(usize::MAX), core::ops::Bound::Excluded(usize::MAX))).is_none());
	assert!(view.index(1..=usize::MAX).is_none());

	let mut data = TEST_DATA;
	let view = DataView::from_mut(&mut data.1);
	assert!(view.try_get_mut::<u16>(usize::MAX).is_none());
	assert!(view.try_slice_mut::<u16>(0, usize::MAX).is_none());
	assert!(view.try_write(usize::MAX, &0_u8).is_none());
	assert!(view.index_mut((core::ops::Bound::Excluded(usize::MAX), core::ops::Bound::Excluded(usize::MAX))).is_none());
	assert!(view.index_mut(1..=usize::MAX).is_none());
}

#[test]
fn test_index() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);

	assert_eq!(view.index(..).unwrap().as_ref(), &bytes[..]);
	assert_eq!(view.index(2..).unwrap().as_ref(), &bytes[2..]);
	assert_eq!(view.index(..4).unwrap().as_ref(), &bytes[..4]);
	assert_eq!(view.index(2..4).unwrap().as_ref(), &bytes[2..4]);
	assert_eq!(view.index(2..=4).unwrap().as_ref(), &bytes[2..=4]);

	assert!(view.index(5..4).is_none());
	assert!(view.index(9..).is_none());
}

#[test]
fn test_index_mut() {
	let mut data = TEST_DATA;
	let view = DataView::from_mut(&mut data.1);

	view.index_mut(2..5).unwrap().as_mut().fill(0xff);

	assert_eq!(data.1, [0, 1, 0xff, 0xff, 0xff, 5, 6, 7]);
}

#[test]
fn test_index_bound_overflow() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);

	// Excluded(MAX) means MAX + 1, which is not representable.
	assert!(view.index((
		core::ops::Bound::Excluded(usize::MAX),
		core::ops::Bound::Unbounded,
	)).is_none());

	// Included(MAX) means an exclusive end of MAX + 1,
	// which is not representable.
	assert!(view.index(..=usize::MAX).is_none());
}

#[test]
fn test_index_mut_bound_overflow() {
	let mut data = TEST_DATA;
	let view = DataView::from_mut(&mut data.1);

	assert!(view.index_mut((
		core::ops::Bound::Excluded(usize::MAX),
		core::ops::Bound::Unbounded,
	)).is_none());

	assert!(view.index_mut(..=usize::MAX).is_none());
}

#[test]
#[should_panic(expected = "invalid offset")]
fn test_index_operator_bound_overflow() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);

	let _ = &view[..=usize::MAX];
}

#[test]
#[should_panic(expected = "invalid offset")]
fn test_index_mut_operator_bound_overflow() {
	let mut data = TEST_DATA;
	let view = DataView::from_mut(&mut data.1);

	let _ = &mut view[..=usize::MAX];
}

#[test]
fn test_write() {
	let mut bytes = [0u8; 8];
	let view = DataView::from_mut(&mut bytes);

	let value = 0x44332211_u32;

	assert_eq!(Some(()), view.try_write(2, &value));
	assert_eq!(view.read::<u32>(2), value);

	view.write(0, &value);
	assert_eq!(view.read::<u32>(0), value);

	unsafe {
		view.write_unchecked(4, &value);
	}
	assert_eq!(view.read::<u32>(4), value);
}

#[test]
fn test_tail_len() {
	let bytes = &TEST_DATA.1;
	let view = DataView::from(bytes);

	assert_eq!(view.tail_len::<u8>(0), 8);
	assert_eq!(view.tail_len::<u16>(0), 4);
	assert_eq!(view.tail_len::<u32>(1), 1);
	assert_eq!(view.tail_len::<u32>(4), 1);
	assert_eq!(view.tail_len::<u32>(8), 0);
	assert_eq!(view.tail_len::<u32>(usize::MAX), 0);
}
