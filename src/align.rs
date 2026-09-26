/// Holds a value at an address aligned to at least 2 bytes.
#[doc(hidden)]
#[repr(C, align(2))]
pub struct Align2<T>(pub T);

/// Holds a value at an address aligned to at least 4 bytes.
#[doc(hidden)]
#[repr(C, align(4))]
pub struct Align4<T>(pub T);

/// Holds a value at an address aligned to at least 8 bytes.
#[doc(hidden)]
#[repr(C, align(8))]
pub struct Align8<T>(pub T);

/// Holds a value at an address aligned to at least 16 bytes.
#[doc(hidden)]
#[repr(C, align(16))]
pub struct Align16<T>(pub T);

/// Holds a value at an address aligned to at least 32 bytes.
#[doc(hidden)]
#[repr(C, align(32))]
pub struct Align32<T>(pub T);

/// Holds a value at an address aligned to at least 64 bytes.
#[doc(hidden)]
#[repr(C, align(64))]
pub struct Align64<T>(pub T);

/// Holds a value at an address aligned to at least 128 bytes.
#[doc(hidden)]
#[repr(C, align(128))]
pub struct Align128<T>(pub T);

/// Holds a value at an address aligned to at least 256 bytes.
#[doc(hidden)]
#[repr(C, align(256))]
pub struct Align256<T>(pub T);

/// Holds a value at an address aligned to at least 512 bytes.
#[doc(hidden)]
#[repr(C, align(512))]
pub struct Align512<T>(pub T);

/// Holds a value at an address aligned to at least 4096 bytes.
#[doc(hidden)]
#[repr(C, align(4096))]
pub struct Align4K<T>(pub T);
