DataView
========

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/dataview.svg)](https://crates.io/crates/dataview)
[![docs.rs](https://docs.rs/dataview/badge.svg)](https://docs.rs/dataview)
[![Build status](https://github.com/CasualX/dataview/workflows/CI/badge.svg)](https://github.com/CasualX/dataview/actions)

The `Pod` trait marks that it is safe to transmute between any bit pattern and an instance of the type.

The `DataView` struct provides methods to read and write pod types into the buffer.

Library
-------

This library is available on [crates.io](https://crates.io/crates/dataview).

Documentation can be found on [docs.rs](https://docs.rs/dataview/).

In your Cargo.toml, put

```
[dependencies]
dataview = "0.1"
```

Examples
--------

```rust
use dataview::Pod;

#[derive(Pod)]
#[repr(C)]
struct MyType {
	field: i32,
}

// Construct a zero initialized instance.
let mut inst = MyType::zeroed();
assert_eq!(inst.field, 0);

// Use the DataView interface to access the instance.
inst.as_data_view_mut().write(2, &255_u8);

// Returns a byte view over the instance.
assert_eq!(inst.as_bytes(), &[0, 0, 255, 0]);
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
