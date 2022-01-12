# abibool - C ABI compatible boolean types

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/abibool.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/abibool)
[![crates.io](https://img.shields.io/crates/v/abibool.svg)](https://crates.io/crates/abibool)
[![docs.rs](https://docs.rs/abibool/badge.svg)](https://docs.rs/abibool)
[![License](https://img.shields.io/crates/l/abibool.svg)](https://github.com/MaulingMonkey/abibool)
[![Build Status](https://github.com/MaulingMonkey/abibool/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/abibool/actions?query=workflow%3Arust)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/abibool/status.svg)](https://deps.rs/repo/github/MaulingMonkey/abibool) -->

For most sane rust APIs, you should prefer [bool] in your interfaces and simply convert between types.
However, [bool] isn't legal for all bit patterns, making it unusable for most FFI without conversions.
For simple FFI, this isn't a problem, but C APIs writing arrays of
[BOOL](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL) or
[BOOLEAN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOLEAN),
or structures containing
these types, become problematic and require allocations and copies to avoid undefined behavior.  Alternatively, you
*could* just use integer types, that can obfuscate intent and result in bugs if multiple truthy-but-different value
are directly compared when you expect boolean logic.

## Type Map

Note that this is *incredibly* system specific.
E.g. BOOL for windows is 4 bytes, but for OS X it's 1 byte... probably.<br>
When using abibool to write FFI crates, you may wish to [`cc`](https://docs.rs/cc/)
a bunch of [`static_assert`](https://en.cppreference.com/w/cpp/language/static_assert)s
in a build script to validate the size of your underlying C types.

| C / C++ type                                                                                              | abibool type      | notes |
| --------------------------------------------------------------------------------------------------------- | ----------------- | ----- |
| `bool` (C++)                                                                                              | *varies*          | Often 1 byte, [but sometimes 4](https://github.com/OpenTTD/OpenTTD/commit/82f7140357b8b13e5f3c2eea715af936e5debb28) or worse
| `_Bool` (C99 / stdbool.h)                                                                                 | *varies*          | Often 1 byte, [but sometimes 4](https://stackoverflow.com/a/10630231) or worse
| [`BOOLEAN`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOLEAN) (Win32)    | [b8] / [bool8]    |
| [`BOOL`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL) (Win32)          | [b32] / [bool32]  |
| [`BOOL`](https://opensource.apple.com/source/objc4/objc4-706/runtime/objc.h.auto.html) (OS X / objc.h)    | [b8] / [bool8] ?  | Typically `signed char`, but sometimes [bool](https://stackoverflow.com/a/544250) or [unsigned char](https://code.woboq.org/gcc/libobjc/objc/objc.h.html)
| [`jboolean`](https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/types.html) (Java / JNI)     | [b8] / [bool8]    |

## References

*   [BOOL / bool / Boolean / NSCFBoolean](https://nshipster.com/bool/)          - Objective C truthy types



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



<!-- references -->

[b8]:               https://docs.rs/abibool/*/abibool/struct.bool8.html
[bool8]:            https://docs.rs/abibool/*/abibool/struct.bool8.html
[b32]:              https://docs.rs/abibool/*/abibool/struct.bool32.html
[bool32]:           https://docs.rs/abibool/*/abibool/struct.bool32.html
[winapi]:           https://docs.rs/winapi/
