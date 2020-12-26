# Downloader

This is a simple project demonstrating the interopability between Rust and C/C++.
The project consists of a simple `download` function in [](./src/lib.rs), which is wrapped for C in [](./src/capi.rs).
[](./Cargo.toml) specifies the `crate-type` as `staticlib`, meaning `cargo` creates a static library in [](./target/release/libdownloader.a) which can be used from C.
The C headers are generated by `cbindgen` in [](./build.rs) and are written to [](./target/release/downloader.h).

`libdownloader.a` and `downloader.h` need to be installed (copied) into the respective `lib` and `include` directories in  [](./tests/capi/downloader/) so that the `CMake` project can use it.
[](./tests/capi/downloader/downloader-config.cmake) defines a static library that can be used in `CMake` files with `find_package(downloader REQUIRED)`.
Linking to `libdownloader.a` requires also a few system libraries, `dl, pthread, m` for `std`, and `ssl, crypto` for `reqwest`.

Then, on the C side, there is a simple `CMake` project in [](./tests/capi/CMakeLists.txt). It defines [](./tests/capi/src/main.c) as the source file and includes the static library with `find_package`.
`cmake` configure needs to be called with `-Ddownloader_DIR=<path_to_downloader-config.cmake>`.

All this is captured in code in the [](./Makefile) and can be run with
```sh
make downloader-c
```
