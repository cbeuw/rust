// ignore-windows
// ignore-tidy-linelength

// compile-flags: -g  -C no-prepopulate-passes --remap-path-prefix=/home/USERNAME=REDACTED
// aux-build:pseudo_std.rs

extern crate pseudo_std;
// Here we check that imported code from std has their path remapped.
// If we were to use an actual std function to test, we'd have to remap {{cwd}}/library/...,
// but that will break if rustc was built with remap-debuginfo = true in config.toml.
// Hence we simulate the path rust-src would take in a real linux installation
// by remapping the prefix inside pseudo_std.rs
// CHECK: !DIFile(filename: "REDACTED/TOOLCHAIN/lib/rustlib/src/rust/library/std/src/pseudo_std.rs", directory: ""
fn main() {
    pseudo_std::some_std_function();
}
