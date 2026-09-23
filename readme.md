DataView
========

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/dataview.svg)](https://crates.io/crates/dataview)
[![docs.rs](https://docs.rs/dataview/badge.svg)](https://docs.rs/dataview)
[![Gate](https://github.com/CasualX/dataview/actions/workflows/gate.yml/badge.svg)](https://github.com/CasualX/dataview/actions/workflows/gate.yml)

Utilities for working with plain data as its in-memory byte representation.

The `Pod` trait marks types whose values can be safely reinterpreted as bytes and reconstructed from arbitrary byte patterns of the same size.

`dataview` also provides:

* `zeroed()` for constructing zero-initialized `Pod` values.
* `transmute()` for reinterpreting equal-sized `Pod` values.
* `bytes()` and `bytes_mut()` for exposing their in-memory representation.
* `DataView` for reading and writing typed values inside byte buffers.

Typical uses include binary file data, memory-mapped structures, device buffers, and memory read from another process.

Library
-------

This library is available on [crates.io](https://crates.io/crates/dataview).

Documentation can be found on [docs.rs](https://docs.rs/dataview/).

`dataview` aims to remain compatible across minor releases, but reserves some flexibility for breakage outside the core `Pod` API.

For most users, pinning to a minor release is recommended:

```text
[dependencies]
dataview = "~1.1"
```

Patch releases within a minor version are intended to remain compatible, with compatibility shims used where practical.

The `Pod` trait itself is considered stable across `1.x` releases. If that is the only API you depend on, a normal version requirement such as `dataview = "1"` is appropriate.

Examples
--------

```rust
#[derive(dataview::Pod)]
#[repr(C)]
struct Header {
	magic: u32,
	version: u16,
	flags: u16,
}

// Construct storage that can be filled by an external API.
let mut header: Header = dataview::zeroed();
let destination: &mut [u8] = dataview::bytes_mut(&mut header);

// For example, imagine these bytes came from a file or another process.
destination.copy_from_slice(&[
	0x44, 0x41, 0x54, 0x41,
	0x01, 0x00,
	0x02, 0x00,
]);

assert_eq!(header.version, 1);
assert_eq!(header.flags, 2);

// DataView is useful when working with a larger raw byte buffer.
let buffer: [u8; 8] = dataview::transmute(header);

let view = dataview::DataView::from(&buffer);
assert_eq!(view.read::<u16>(4), 1);
assert_eq!(view.read::<u16>(6), 2);
```

No serialization or conversion takes place: `dataview` works with the native in-memory representation of values.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
