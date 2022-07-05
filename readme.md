DataView
========

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/dataview.svg)](https://crates.io/crates/dataview)
[![docs.rs](https://docs.rs/dataview/badge.svg)](https://docs.rs/dataview)
[![Build status](https://github.com/CasualX/dataview/workflows/CI/badge.svg)](https://github.com/CasualX/dataview/actions)

The `Pod` trait marks types whose values can be safely transmuted between byte arrays of the same size.

The `DataView` type defines read and write data APIs to an underlying byte buffer.

Library
-------

This library is available on [crates.io](https://crates.io/crates/dataview).

Documentation can be found on [docs.rs](https://docs.rs/dataview/).

In your Cargo.toml, put

```text
[dependencies]
dataview = "~1.0"
```

Examples
--------

```rust
#[derive(dataview::Pod)]
#[repr(C)]
struct MyType {
	field: i32,
}

// Construct a zero initialized instance
let mut inst: MyType = dataview::zeroed();
assert_eq!(inst.field, 0);

// Use DataView to access the instance
let view = dataview::DataView::from_mut(&mut inst);
view.write(2, &255_u8);

// Create a byte view over the instance
assert_eq!(dataview::bytes(&inst), &[0, 0, 255, 0]);
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
