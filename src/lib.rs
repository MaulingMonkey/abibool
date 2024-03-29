#![doc = include_str!("../Readme.md")]
#![no_std]

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::mem::transmute;
use core::ops::{Deref, DerefMut, Not};

// XXX: REMOVEME: Get rid of these defs in the next breaking revision of abibool.
// They're too winapi specific.  See other "XXX: REMOVEME: " comments for thoughts.
use i32 as BOOL;    // use winapi::shared::minwindef::BOOL;
use u8 as BOOLEAN;  // use winapi::shared::minwindef::BOOLEAN;


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
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct bool8(BOOLEAN);
pub use bool8 as b8;

impl bool8 {
    /// bool8(`0`)
    pub const FALSE : bool8 = bool8(0);

    /// bool8(`1`)
    pub const TRUE  : bool8 = bool8(1);

    pub fn from(value: impl Into<Self>) -> Self { value.into() }
}

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
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct bool32(BOOL);
pub use bool32 as b32;

impl bool32 {
    /// bool32(`0`)
    pub const FALSE : bool32 = bool32(0);

    /// bool32(`1`)
    pub const TRUE  : bool32 = bool32(1);

    pub fn from(value: impl Into<Self>) -> Self { value.into() }
}



impl AsRef<bool>  for bool8  { fn as_ref(&self) -> &bool { if bool::from(*self) { &true } else { &false } } }
impl AsRef<bool>  for bool32 { fn as_ref(&self) -> &bool { if bool::from(*self) { &true } else { &false } } }

impl Borrow<bool> for bool8  { fn borrow(&self) -> &bool { if bool::from(*self) { &true } else { &false } } }
impl Borrow<bool> for bool32 { fn borrow(&self) -> &bool { if bool::from(*self) { &true } else { &false } } }

// DON'T IMPLEMENT:
//  impl Borrow<BOOLEAN> for bool8  { ... }
//  impl Borrow<BOOL   > for bool32 { ... }
// "In particular Eq, Ord and Hash must be equivalent for borrowed and owned values" (https://doc.rust-lang.org/std/borrow/trait.Borrow.html)
// We've gone to pains to make bool32 behave very much like bool, with `true` acting like a single value, even when the internal BOOL might be another truthy value like `-1`.

// XXX: REMOVEME:  Too winapi specific, prone to misuse.  Main intent here is FFI interop.
// Replace with `as_[mut_]_ptr` type constrained to matching-size integer types?
impl Deref for bool8  { type Target = BOOLEAN; fn deref(&self) -> &Self::Target { &self.0 } }
impl Deref for bool32 { type Target = BOOL;    fn deref(&self) -> &Self::Target { &self.0 } }
impl DerefMut for bool8  { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
impl DerefMut for bool32 { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }

impl Default for bool8  { fn default() -> Self { Self::FALSE } }
impl Default for bool32 { fn default() -> Self { Self::FALSE } }
impl Debug   for bool8  { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug  ::fmt(&bool::from(*self), f) } }
impl Debug   for bool32 { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug  ::fmt(&bool::from(*self), f) } }
impl Display for bool8  { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&bool::from(*self), f) } }
impl Display for bool32 { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&bool::from(*self), f) } }

impl From<bool   > for bool8   { fn from(value: bool   ) -> Self { Self(value as _) } }
impl From<bool   > for bool32  { fn from(value: bool   ) -> Self { Self(value as _) } }
impl From<BOOLEAN> for bool8   { fn from(value: BOOLEAN) -> Self { Self(value) } } // XXX: REMOVEME: replace with `{u,i}8`?
impl From<BOOL   > for bool32  { fn from(value: BOOL   ) -> Self { Self(value) } } // XXX: REMOVEME: replace with `{u,i}32`?
impl From<bool8  > for BOOLEAN { fn from(value: bool8  ) -> Self { value.0 } } // XXX: REMOVEME: replace with `{u,i}8`?
impl From<bool32 > for BOOL    { fn from(value: bool32 ) -> Self { value.0 } } // XXX: REMOVEME: replace with `{u,i}32`?
impl From<bool8  > for bool    { fn from(value: bool8  ) -> Self { value.0 != 0 } }
impl From<bool32 > for bool    { fn from(value: bool32 ) -> Self { value.0 != 0 } }

impl From<&BOOLEAN> for &bool8   { fn from(value: &BOOLEAN) -> Self { unsafe { transmute(value) } } } // XXX: REMOVEME: replace with `{u,i}8`?
impl From<&BOOL   > for &bool32  { fn from(value: &BOOL   ) -> Self { unsafe { transmute(value) } } } // XXX: REMOVEME: replace with `{u,i}32`?
impl From<&bool8  > for &BOOLEAN { fn from(value: &bool8  ) -> Self { unsafe { transmute(value) } } } // XXX: REMOVEME: replace with `{u,i}8`?
impl From<&bool32 > for &BOOL    { fn from(value: &bool32 ) -> Self { unsafe { transmute(value) } } } // XXX: REMOVEME: replace with `{u,i}32`?

// slices are always foreign, so we can't implement these - transmute yourself I guess
// impl From<&[BOOLEAN]> for &[bool8  ] { fn from(value: &[BOOLEAN]) -> Self { unsafe { transmute(value) } } }
// impl From<&[BOOL   ]> for &[bool32 ] { fn from(value: &[BOOL   ]) -> Self { unsafe { transmute(value) } } }
// impl From<&[bool8  ]> for &[BOOLEAN] { fn from(value: &[bool8  ]) -> Self { unsafe { transmute(value) } } }
// impl From<&[bool32 ]> for &[BOOL   ] { fn from(value: &[bool32 ]) -> Self { unsafe { transmute(value) } } }

// All comparisons, hashes, etc. are based on truthiness, not the underlying bit patterns!

impl Not               for bool8  { type Output = bool; fn not(self) -> Self::Output { self.0 == 0 } }
impl Not               for bool32 { type Output = bool; fn not(self) -> Self::Output { self.0 == 0 } }

impl Eq                for bool8  {}
impl Eq                for bool32 {}
impl PartialEq<bool8 > for bool8  { fn eq(&self, other: &bool8 ) -> bool { bool::from(*self) == bool::from(*other) } }
impl PartialEq<bool32> for bool32 { fn eq(&self, other: &bool32) -> bool { bool::from(*self) == bool::from(*other) } }
impl PartialEq<bool8 > for bool32 { fn eq(&self, other: &bool8 ) -> bool { bool::from(*self) == bool::from(*other) } }
impl PartialEq<bool32> for bool8  { fn eq(&self, other: &bool32) -> bool { bool::from(*self) == bool::from(*other) } }

impl PartialEq<bool  > for bool8  { fn eq(&self, other: &bool  ) -> bool { bool::from(*self) == *other } }
impl PartialEq<bool  > for bool32 { fn eq(&self, other: &bool  ) -> bool { bool::from(*self) == *other } }
impl PartialEq<bool8 > for bool   { fn eq(&self, other: &bool8 ) -> bool { bool::from(*other) == *self } }
impl PartialEq<bool32> for bool   { fn eq(&self, other: &bool32) -> bool { bool::from(*other) == *self } }

impl PartialOrd<bool8 > for bool8  { fn partial_cmp(&self, other: &bool8 ) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), &bool::from(*other)) } }
impl PartialOrd<bool32> for bool32 { fn partial_cmp(&self, other: &bool32) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), &bool::from(*other)) } }
impl PartialOrd<bool8 > for bool32 { fn partial_cmp(&self, other: &bool8 ) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), &bool::from(*other)) } }
impl PartialOrd<bool32> for bool8  { fn partial_cmp(&self, other: &bool32) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), &bool::from(*other)) } }

impl PartialOrd<bool  > for bool8  { fn partial_cmp(&self, other: &bool  ) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), other) } }
impl PartialOrd<bool  > for bool32 { fn partial_cmp(&self, other: &bool  ) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), other) } }
impl PartialOrd<bool8 > for bool   { fn partial_cmp(&self, other: &bool8 ) -> Option<Ordering> { PartialOrd::partial_cmp(self, &bool::from(*other)) } }
impl PartialOrd<bool32> for bool   { fn partial_cmp(&self, other: &bool32) -> Option<Ordering> { PartialOrd::partial_cmp(self, &bool::from(*other)) } }

impl Ord for bool8  { fn cmp(&self, other: &bool8 ) -> Ordering { Ord::cmp(&bool::from(*self), &bool::from(*other)) } }
impl Ord for bool32 { fn cmp(&self, other: &bool32) -> Ordering { Ord::cmp(&bool::from(*self), &bool::from(*other)) } }

impl Hash for bool8  { fn hash<H: Hasher>(&self, state: &mut H) { bool::from(*self).hash(state) } }
impl Hash for bool32 { fn hash<H: Hasher>(&self, state: &mut H) { bool::from(*self).hash(state) } }

#[cfg(feature = "bytemuck")] mod _bytemuck {
    use super::*;

    unsafe impl bytemuck::Pod for bool8  {}
    unsafe impl bytemuck::Pod for bool32 {}
    unsafe impl bytemuck::Zeroable for bool8  {}
    unsafe impl bytemuck::Zeroable for bool32 {}
}
