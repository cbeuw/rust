// ignore-windows

// compile-flags: -g  -C no-prepopulate-passes --remap-path-prefix={{cwd}}/library/std=/the/std

// Here we check that imported code from std has their path remapped
// Ideally we want to do
// COM: CHECK-NOT: !DIFile(filename: "{{cwd}}/library/std/src/thread/mod.rs"
// But {{cwd}} will not be expanded here, so we have to check on the bit afterwards
// Note that we can't use a positive check on `/the/std` either, because the rempap flag will
// have no effect when `remap-debuginfo` is set to true in `config.toml`, causing the test to fail

// CHECK-NOT: !DIFile(filename: "{{.*}}/library/std"
fn main() {
    std::thread::spawn(|| {
        println!("hello");
    });
}
