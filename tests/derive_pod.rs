#![allow(dead_code)]
#![feature(macro_derive)]

use dataview::{Field, Fields, Pod};

#[derive(Pod)]
#[repr(C)]
struct Struct0 {}

#[derive(Pod)]
#[repr(C)]
/// doc comment
struct Struct1 {
	field1: i32,
}

#[derive(Pod)]
/// doc comment
#[repr(C)]
struct Struct2 {
	field1: i32
}

#[derive(Pod)]
#[allow(dead_code)]
/// doc comment
#[doc(hidden)]
#[repr(C)]
#[repr(align(8))]
struct Struct3 {
	field1: i32,
	field2: f32,
}

#[derive(Pod)]
#[repr(align(8))]
#[repr(C)]
struct Struct4 {
	field1: i32,
	field2: f32
}

#[derive(Pod)]
#[repr(align(8), C)]
struct Struct5 {
	field1: i32,
	field2: f32
}

#[derive(Pod, Fields)]
#[repr(C, align(8))]
struct Struct6 {
	field1: i32,
	field2: f32
}

const _: Field<Struct6, i32> = Struct6::field1;
const _: Field<Struct6, f32> = Struct6::field2;
const _: [(); 0] = [(); Struct6::field1.offset()];
const _: [(); 4] = [(); Struct6::field2.offset()];

#[repr(transparent)]
struct Private(i32);

#[derive(Fields)]
#[repr(C)]
pub struct Struct7 {
	private: Private,
}

const _: Field<Struct7, Private> = Struct7::private;

mod visibility {
	use super::*;

	#[derive(Fields)]
	#[repr(C)]
	pub struct Struct8 {
		pub public: i32,
		pub(crate) crate_visible: i32,
		private: i32,
	}
}

const _: Field<visibility::Struct8, i32> = visibility::Struct8::public;
const _: Field<visibility::Struct8, i32> = visibility::Struct8::crate_visible;

#[derive(Fields)]
#[repr(C)]
struct Struct9 {
	byte: u8,
	word: u32,
}

const _: [(); 0] = [(); Struct9::byte.offset()];
const _: [(); 4] = [(); Struct9::word.offset()];

const _: [(); 0] = [(); dataview::Field!(Struct9.byte).offset()];
const _: [(); 4] = [(); dataview::Field!(Struct9.word).offset()];

#[derive(Fields)]
#[repr(C)]
struct Struct10 {
	prefix: u8,
	inner: Struct11,
}

#[derive(Fields)]
#[repr(C)]
struct Struct11 {
	prefix: u16,
	value: u32,
	other: u32,
}

const STRUCT10_VALUE: Field<Struct10, u32> = Struct10::inner.then(Struct11::value);
const _: [(); 8] = [(); STRUCT10_VALUE.offset()];
const _: [(); 8] = [(); STRUCT10_VALUE.span().start];
const _: [(); 12] = [(); STRUCT10_VALUE.span().end];

#[test]
fn field_descriptor_spans_and_composition() {
	let value = Struct10::inner.then(Struct11::value);
	let other = Struct10::inner + Struct11::other;

	assert_eq!(Struct10::inner.span(), 4..16);
	assert!(value == STRUCT10_VALUE);
	assert!(value == dataview::Field!(Struct10.inner.value));
	assert_eq!(value.offset(), 8);
	assert_eq!(value.span(), 8..12);
	assert!(value != other);
	assert!(other == dataview::Field!(Struct10.inner.other));
}

#[derive(Pod)]
#[repr(C)]
struct Tuple0();

#[derive(Pod)]
#[repr(C)]
struct Tuple1(i32);

#[derive(Pod)]
#[repr(C)]
struct Tuple2(i32,);

#[derive(Pod)]
#[repr(C)]
struct Tuple3(i32, f32);

#[derive(Pod)]
#[repr(C)]
struct Tuple4(i32, f32,);

#[derive(Pod)]
#[repr(C)]
struct Unit;
