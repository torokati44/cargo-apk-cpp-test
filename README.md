There are 4 crates here:

### `c-crate`

A library written in C, wrapped in a Rust crate, built with `cc`.

Exposes a single function: `add(a, b)`

Is here mostly for "baseline" purposes.

Is an unconditional dependency of the 2 app crates.

### `cpp-crate`

A library written in C++, wrapped in a Rust crate, built with `cc`.

Exposes a single function: `sub(a, b)`

Is an optional dependency of the 2 app crates.

### `desktop-crate`

Simply invokes the exported functions of the above crates, serves as a simple check that everything works in the "normal case".

If the `with_cpp` feature is enabled (and it is by default),
uses `cpp-crate`, and links to a shared C++ standard library to make it work.

### `apk-crate`

Does the same thing as `desktop-crate`, but set up to be run with `cargo-apk`.

Also has the `with_cpp` feature.