// ignore-windows
// ignore-tidy-linelength

// compile-flags: -g  -C no-prepopulate-passes --remap-path-prefix=/the/aux-src=/the/double/remapped/aux-src
// aux-build:remap_path_prefix_aux.rs

extern crate remap_path_prefix_aux;

// Here we check that path can be remapped again after already having been
// remapped by compile-flags inside auxiliary/remap_path_prefix_aux.rs
// CHECK: !DIFile(filename: "/the/double/remapped/aux-src/remap_path_prefix_aux.rs", directory: ""
fn main() {
    remap_path_prefix_aux::some_aux_function();
}
