/// A one-byte boolean-like value whose entire `u8` range is valid.
///
/// Unlike Rust's `bool`, `Bool8` implements [`crate::Pod`] and can be viewed
/// directly in byte-oriented data. Zero conventionally means false and one
/// means true; other byte values remain valid and are preserved unchanged.
/// Equality compares the stored bytes, so `Bool8(2)` differs from [`Bool8::TRUE`].
///
/// ```
/// let value: dataview::Bool8 = dataview::transmute(2_u8);
/// assert_eq!(value.0, 2);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Bool8(pub u8);

unsafe impl crate::Pod for Bool8 {}

impl Bool8 {
	/// The conventional true value, stored as `1`.
	pub const TRUE: Bool8 = Bool8(1);
	/// The conventional false value, stored as `0`.
	pub const FALSE: Bool8 = Bool8(0);
}
