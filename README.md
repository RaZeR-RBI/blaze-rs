# blaze-rs
---
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust wrapper for [blaze](https://github.com/razer-rbi/blaze) library.

__[API Documentation](https://razer-rbi.github.io/blaze-rs/api/blaze_rs/index.html)__

__[Test coverage](https://razer-rbi.github.io/blaze-rs/coverage/index.html)__

# Features
Features are the same as the original library, just Rust-flavored:
* Dynamic batched sprite drawing
* Static batched sprite drawing
* Immediate drawing
* Texture loading
* Render targets
* Custom shaders
* Screenshot saving

The API documentation contains information about the implementation.

# Installation and usage
Add to your __Cargo.toml__:
```
blaze_rs = { git = "https://github.com/RaZeR-RBI/blaze-rs.git" }
```
To use this wrapper, you must install the [blaze](https://github.com/razer-rbi/blaze) library.

Make sure you have the compiled library in the system's linker path.

# Examples
You can check the [tests directory](https://github.com/RaZeR-RBI/blaze-rs/tree/master/tests/common) to see what you can do.

# Running tests
The build script uses an environment variable called `BLAZE_PATH`, which can
point to location where libblaze.so (or libblaze.dll) is stored. If it's not set,
the system linker paths will be used by default.

To run tests with coverage collection enabled, and also generate documentation 
you can use a bundled script:
```
./build_test_and_doc.sh
```

Coverage data is provided by [tarpaulin](https://github.com/xd009642/tarpaulin), so in order to get coverage data you should have it installed.