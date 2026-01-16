/// Macro to include binary data at compile time.
///
/// ```
/// dataview::embed!(pub DATA: [u16] = "embed.rs");
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! embed {
	($vis:vis $name:ident: [$ty:ty] = $path:expr) => {
		$vis static $name: [$ty; include_bytes!($path).len() / ::core::mem::size_of::<$ty>()] = {
			fn __assert_pod<T: ::dataview::Pod>() {}
			let _ = __assert_pod::<$ty>;
			unsafe { ::core::mem::transmute(*include_bytes!($path)) }
		};
	};
}
