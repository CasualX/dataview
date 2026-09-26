
/// Macro to include binary data at compile time.
///
/// The embedded bytes are interpreted as the native representation of `$ty`.
/// In particular, multi-byte integer and floating-point values use the target platform's endianness.
///
/// The file size must be an exact multiple of `size_of::<$ty>()`.
///
/// ```
/// dataview::embed!(pub DATA: [u16] = "embed.rs");
/// ```
///
/// To give the static a specific alignment, wrap the array in an `AlignN` type:
///
/// ```
/// dataview::embed!(pub DATA: Align8<[u16]> = "embed.rs");
/// let values: &[u16] = &DATA.0;
/// assert_eq!(values.as_ptr().addr() % 8, 0);
/// ```
#[macro_export]
macro_rules! embed {
	($vis:vis $name:ident: $align:ident <[$ty:ty]> = $path:expr) => {
		$vis static $name: $crate::$align<[$ty; {
			const SIZE: usize = ::core::mem::size_of::<$ty>();
			const LEN: usize = ::core::include_bytes!($path).len();

			assert!(SIZE != 0, "cannot embed a zero-sized type");
			assert!(LEN % SIZE == 0, "embedded file size is not a multiple of the element size");

			LEN / SIZE
		}]> = $crate::$align({
			fn __assert_pod<T: $crate::Pod>() {}
			let _ = __assert_pod::<$ty>;
			unsafe { ::core::mem::transmute(*::core::include_bytes!($path)) }
		});
	};
	($vis:vis $name:ident: [$ty:ty] = $path:expr) => {
		$vis static $name: [$ty; {
			const SIZE: usize = ::core::mem::size_of::<$ty>();
			const LEN: usize = ::core::include_bytes!($path).len();

			assert!(SIZE != 0, "cannot embed a zero-sized type");
			assert!(LEN % SIZE == 0, "embedded file size is not a multiple of the element size");

			LEN / SIZE
		}] = {
			fn __assert_pod<T: $crate::Pod>() {}
			let _ = __assert_pod::<$ty>;
			unsafe { ::core::mem::transmute(*::core::include_bytes!($path)) }
		};
	};
}
