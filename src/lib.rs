#![doc = include_str!("../Readme.md")]
#![no_std]

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::mem::transmute;
use core::ops::{Deref, DerefMut, Not};

pub trait Impl : Copy + PartialEq {
    const FALSE : Self;
    const TRUE  : Self;
}

//impl Impl for bool { const FALSE : Self = false; const TRUE : Self = true  ; }
impl Impl for char { const FALSE : Self = '\0' ; const TRUE : Self = '\x01'; }

impl Impl for u8   { const FALSE : Self = 0; const TRUE : Self = 1; }
impl Impl for u16  { const FALSE : Self = 0; const TRUE : Self = 1; }
impl Impl for u32  { const FALSE : Self = 0; const TRUE : Self = 1; }
impl Impl for u64  { const FALSE : Self = 0; const TRUE : Self = 1; }
impl Impl for u128 { const FALSE : Self = 0; const TRUE : Self = 1; }

impl Impl for i8   { const FALSE : Self = 0; const TRUE : Self = 1; }
impl Impl for i16  { const FALSE : Self = 0; const TRUE : Self = 1; }
impl Impl for i32  { const FALSE : Self = 0; const TRUE : Self = 1; }
impl Impl for i64  { const FALSE : Self = 0; const TRUE : Self = 1; }
impl Impl for i128 { const FALSE : Self = 0; const TRUE : Self = 1; }



/// Boolean type that's ABI-compatible with `T` (`u8`, `u32`, etc.)
///
/// 99% of the time, you should prefer [bool] in your interfaces and simply convert between types.
/// However, some windows APIs take [BOOLEAN] arrays, or contain structures with [BOOLEAN]s.
/// <code>[Boolean]\<[u8]\></code> can be used in these cases to avoid the need for internal allocations or conversions for mere ABI conversions.
///
/// `0` is `false`y, all other bit patterns are `true`thy.
///
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Boolean<T: Impl>(T);

impl<T: Impl> Boolean<T> {
    /// Boolean(`0`)
    pub const FALSE : Self = Self(T::FALSE);

    /// Boolean(`1`)
    pub const TRUE  : Self = Self(T::TRUE );

    pub fn from(value: impl Into<Self>) -> Self { value.into() }
}



/// 8-bit boolean type that's ABI-compatible with Win32's [BOOLEAN].
///
/// 99% of the time, you should prefer [bool] in your interfaces and simply convert between types.
/// However, some windows APIs take [BOOLEAN] arrays, or contain structures with [BOOLEAN]s.
/// [bool8] can be used in these cases to avoid the need for internal allocations or conversions for mere ABI conversions.
///
/// `0` is `false`y, all other bit patterns are `true`thy.
///
/// [BOOLEAN]:      https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOLEAN
#[allow(non_camel_case_types)] // Okay, `bool8` is kind of a weird type name I agree... warranted in this case though IMO
pub type bool8 = Boolean<u8>;
pub use bool8 as b8;

/// 32-bit boolean type that's ABI-compatible with Win32's [BOOL].
///
/// 99% of the time, you should prefer [bool] in your interfaces and simply convert between types.
/// However, some windows APIs take [BOOL] arrays, or contain structures with [BOOL]s.
/// [bool32] can be used in these cases to avoid the need for internal allocations or conversions for mere ABI conversions.
///
/// `0` is `false`y, all other bit patterns are `true`thy.
///
/// [BOOL]:         https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL
#[allow(non_camel_case_types)] // Okay, `bool32` is kind of a weird type name I agree... warranted in this case though IMO
pub type bool32 = Boolean<u32>;
pub use bool32 as b32;


impl<T: Impl> AsRef <bool> for Boolean<T> { fn as_ref(&self) -> &bool { if bool::from(*self) { &true } else { &false } } }
impl<T: Impl> Borrow<bool> for Boolean<T> { fn borrow(&self) -> &bool { if bool::from(*self) { &true } else { &false } } }

// DON'T IMPLEMENT:
//  impl<T: Impl> Borrow<T> for Boolean<T> { ... }
// "In particular Eq, Ord and Hash must be equivalent for borrowed and owned values" (https://doc.rust-lang.org/std/borrow/trait.Borrow.html)
// We've gone to pains to make bool32 behave very much like bool, with `true` acting like a single value, even when the internal BOOL might be another truthy value like `-1`.

// XXX: REMOVEME:  Too winapi specific, prone to misuse.  Main intent here is FFI interop.
// Replace with `as_[mut_]_ptr` type constrained to matching-size integer types?
impl<T: Impl> Deref    for Boolean<T>   { type Target = T; fn deref(&self) -> &Self::Target { &self.0 } }
impl<T: Impl> DerefMut for Boolean<T>   { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }

impl<T: Impl> Default for Boolean<T>    { fn default() -> Self { Self::FALSE } }
impl<T: Impl> Debug   for Boolean<T>    { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug  ::fmt(&bool::from(*self), f) } }
impl<T: Impl> Display for Boolean<T>    { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&bool::from(*self), f) } }

impl<T: Impl> From<bool> for Boolean<T> { fn from(value: bool      ) -> Self { if value { Self::TRUE } else { Self::FALSE } } }
impl<T: Impl> From<Boolean<T>> for bool { fn from(value: Boolean<T>) -> Self { value.0 != T::FALSE } }

impl<T: Impl> From<  T > for Boolean<  T > { fn from(value: T) -> Self { Self(value) } }
impl          From<Boolean<  u8>> for   u8 { fn from(value: Boolean<  u8>) -> Self { value.0 } }
impl          From<Boolean< u16>> for  u16 { fn from(value: Boolean< u16>) -> Self { value.0 } }
impl          From<Boolean< u32>> for  u32 { fn from(value: Boolean< u32>) -> Self { value.0 } }
impl          From<Boolean< u64>> for  u64 { fn from(value: Boolean< u64>) -> Self { value.0 } }
impl          From<Boolean<u128>> for u128 { fn from(value: Boolean<u128>) -> Self { value.0 } }
impl          From<Boolean<  i8>> for   i8 { fn from(value: Boolean<  i8>) -> Self { value.0 } }
impl          From<Boolean< i16>> for  i16 { fn from(value: Boolean< i16>) -> Self { value.0 } }
impl          From<Boolean< i32>> for  i32 { fn from(value: Boolean< i32>) -> Self { value.0 } }
impl          From<Boolean< i64>> for  i64 { fn from(value: Boolean< i64>) -> Self { value.0 } }
impl          From<Boolean<i128>> for i128 { fn from(value: Boolean<i128>) -> Self { value.0 } }

impl<T: Impl> From<&          T  > for &Boolean<T> { fn from(value: &          T  ) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean<  u8>> for &        u8 { fn from(value: &Boolean<  u8>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean< u16>> for &       u16 { fn from(value: &Boolean< u16>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean< u32>> for &       u32 { fn from(value: &Boolean< u32>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean< u64>> for &       u64 { fn from(value: &Boolean< u64>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean<u128>> for &      u128 { fn from(value: &Boolean<u128>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean<  i8>> for &        i8 { fn from(value: &Boolean<  i8>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean< i16>> for &       i16 { fn from(value: &Boolean< i16>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean< i32>> for &       i32 { fn from(value: &Boolean< i32>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean< i64>> for &       i64 { fn from(value: &Boolean< i64>) -> Self { unsafe { transmute(value) } } }
impl          From<&Boolean<i128>> for &      i128 { fn from(value: &Boolean<i128>) -> Self { unsafe { transmute(value) } } }

// slices are always foreign, so we can't implement these - transmute yourself I guess
// impl From<&[BOOLEAN]> for &[bool8  ] { fn from(value: &[BOOLEAN]) -> Self { unsafe { transmute(value) } } }
// impl From<&[bool8  ]> for &[BOOLEAN] { fn from(value: &[bool8  ]) -> Self { unsafe { transmute(value) } } }

// All comparisons, hashes, etc. are based on truthiness, not the underlying bit patterns!

impl<T: Impl            > Not                       for Boolean<T>  { type Output = bool; fn not(self) -> Self::Output { self.0 == T::FALSE } }
impl<T: Impl            > Eq                        for Boolean<T>  {}
impl<L: Impl, R: Impl   > PartialEq<Boolean<R>>     for Boolean<L>  { fn eq(&self, other: &Boolean<R>) -> bool { bool::from(*self) == bool::from(*other) } }
impl<T: Impl            > PartialEq<bool>           for Boolean<T>  { fn eq(&self, other: &bool      ) -> bool { bool::from(*self) == *other } }
impl<T: Impl            > PartialEq<Boolean<T>>     for bool        { fn eq(&self, other: &Boolean<T>) -> bool { bool::from(*other) == *self } }
impl<L: Impl, R: Impl   > PartialOrd<Boolean<R>>    for Boolean<L>  { fn partial_cmp(&self, other: &Boolean<R>) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), &bool::from(*other)) } }
impl<T: Impl            > PartialOrd<bool>          for Boolean<T>  { fn partial_cmp(&self, other: &bool      ) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), other) } }
impl<T: Impl            > PartialOrd<Boolean<T>>    for bool        { fn partial_cmp(&self, other: &Boolean<T>) -> Option<Ordering> { PartialOrd::partial_cmp(self, &bool::from(*other)) } }
impl<T: Impl            > Ord                       for Boolean<T>  { fn cmp(&self, other: &Boolean<T>) -> Ordering { Ord::cmp(&bool::from(*self), &bool::from(*other)) } }
impl<T: Impl            > Hash                      for Boolean<T>  { fn hash<H: Hasher>(&self, state: &mut H) { bool::from(*self).hash(state) } }

#[cfg(feature = "bytemuck")] mod _bytemuck {
    use super::*;

    unsafe impl<T: Impl + bytemuck::Pod     > bytemuck::Pod         for Boolean<T> {}
    unsafe impl<T: Impl + bytemuck::Zeroable> bytemuck::Zeroable    for Boolean<T> {}
}
