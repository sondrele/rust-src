# rust-src
==========

A build-dependency to download a tarball of the Rust source code
associated with the current version of `rustc`. 
Makes it easier to compile many of Rust's different crates and add them 
as dependencies to a project where the target triple is not natively supported by `rustc`.

Add as a build-dependency in `Cargo.toml`
```toml
build = "build.rs"

[build-dependencies.rust_src]
git = "https://github.com/sondrele/rust-src.git"
```

And download the rust source code in the build script, here `build.rs`
```rust
extern crate rust_src;

fn main() {
    rust_src::fetch();
}
```
