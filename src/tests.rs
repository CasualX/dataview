use super::*;

#[derive(Copy, Clone)]
#[repr(C, align(8))]
struct Foo([u32; 2]);
unsafe impl Pod for Foo {}

#[repr(C, align(4))]
struct Baz([u32; 2]);
unsafe impl Pod for Baz {}

#[test]
fn test_transmute() {
	let data = [42, 13];
	let mut a = Foo(data);
	let b: Baz = a.transmute();
	assert_eq!(data, b.0);
	let c: &Baz = a.transmute_ref();
	assert_eq!(data, c.0);
	let d: &mut Baz = a.transmute_mut();
	assert_eq!(data, d.0);
}

#[test]
fn test_zeroed() {
	let a = Foo::zeroed();
	assert_eq!([0u32; 2], a.0);
	let b = <[f32; 2]>::zeroed();
	assert_eq!([0f32; 2], b);
}

//------------------------------------------------
// DataView tests

// Align test data to reliably produce misaligned offsets for testing
static TEST_DATA: ([u64; 0], [u8; 8]) = ([], [0, 1, 2, 3, 4, 5, 6, 7]);

#[test]
fn test_basics() {
	let bytes = &TEST_DATA.1;
	let view = bytes.as_data_view();
	assert_eq!(view.len(), bytes.len());
	assert_eq!(view.as_ref(), bytes);
}

#[test]
fn test_copy() {
	let bytes = &TEST_DATA.1;
	let view = bytes.as_data_view();
	for i in 0..bytes.len() {
		let value = i as u8;
		assert_eq!(value, bytes[i]);
		assert_eq!(Some(value), view.try_copy(i));
		assert_eq!(value, view.copy(i));
		assert_eq!(value, unsafe { view.copy_unchecked(i) });
	}
	assert!(matches!(view.try_copy::<u8>(view.len()), None));
}

#[test]
fn test_copy_into() {
	let bytes = &TEST_DATA.1;
	let view = bytes.as_data_view();
	let mut dest: u8;
	for i in 0..bytes.len() {
		let value = i as u8;
		assert_eq!(value, bytes[i]);
		dest = !0;
		assert_eq!(Some(()), view.try_copy_into(i, &mut dest));
		assert_eq!(value, dest);
		dest = !0;
		view.copy_into(i, &mut dest);
		assert_eq!(value, dest);
		dest = !0;
		unsafe { view.copy_into_unchecked(i, &mut dest); }
		assert_eq!(value, dest);
	}
	dest = !0;
	assert!(matches!(view.try_copy_into::<u8>(view.len(), &mut dest), None));
}

#[test]
fn test_read() {
	let bytes = &TEST_DATA.1;
	let view = bytes.as_data_view();
	for i in 0..bytes.len() {
		let value = i as u8;
		assert_eq!(value, bytes[i]);
		assert_eq!(Some(&value), view.try_read(i));
		assert_eq!(&value, view.read(i));
		assert_eq!(&value, unsafe { view.read_unchecked(i) });
		if i % 2 == 1 {
			assert!(matches!(view.try_read::<u16>(i), None));
		}
	}
	assert!(matches!(view.try_read::<u8>(view.len()), None));
}

#[test]
fn test_read_mut() {
	let mut data = TEST_DATA;
	let check = TEST_DATA.1;
	let bytes = &mut data.1;
	let view = bytes.as_data_view_mut();
	for i in 0..check.len() {
		let mut value = i as u8;
		assert_eq!(value, check[i]);
		assert_eq!(Some(&mut value), view.try_read_mut(i));
		assert_eq!(&value, view.read_mut(i));
		assert_eq!(&value, unsafe { view.read_unchecked_mut(i) });
		if i % 2 == 1 {
			assert!(matches!(view.try_read_mut::<u16>(i), None));
		}
	}
	assert!(matches!(view.try_read_mut::<u8>(view.len()), None));
}

#[test]
fn test_slice() {
	let bytes = &TEST_DATA.1;
	let view = bytes.as_data_view();
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
	let view = bytes.as_data_view_mut();
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
