#![allow(dead_code)]

use dataview::Pod;

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
