# abibool - C ABI compatible boolean types

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/abibool.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/abibool)
[![crates.io](https://img.shields.io/crates/v/abibool.svg)](https://crates.io/crates/abibool)
[![docs.rs](https://docs.rs/abibool/badge.svg)](https://docs.rs/abibool)
[![License](https://img.shields.io/crates/l/abibool.svg)](https://github.com/MaulingMonkey/abibool)
[![Build Status](https://github.com/MaulingMonkey/abibool/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/abibool/actions?query=workflow%3Arust)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/abibool/status.svg)](https://deps.rs/repo/github/MaulingMonkey/abibool) -->

For most sane rust APIs, you should prefer [bool] in your interfaces and simply convert between types.
However, [bool] isn't legal for all bit patterns, making it unusable for most FFI without conversions.
For simple FFI, this isn't a problem, but C APIs writing arrays of [BOOL] or [BOOLEAN], or structures containing
these types, become problematic and require allocations and copies to avoid undefined behavior.  Alternatively, you
*could* just use integer types, that can obfuscate intent and result in bugs if multiple truthy-but-different value
are directly compared when you expect boolean logic.

See [The Documentation](https://docs.rs/abibool) for details.

| abibool type      | winapi type   |
| ----------------- | ------------- |
| [b8] / [bool8]    | [BOOLEAN]     |
| [b32] / [bool32]  | [BOOL]        |



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

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

[BOOL]:             https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL
[BOOLEAN]:          https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOLEAN
[bytemuck::Pod]:    https://docs.rs/bytemuck/1.4/bytemuck/trait.Pod.html
