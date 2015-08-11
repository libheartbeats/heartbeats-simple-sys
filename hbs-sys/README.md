# Simple Heartbeats Rust Bindings

The `hbs-sys` crate provides declarations and linkage for the `hbs-static` C
library.
Following the *-sys package conventions, this crate does not define
higher-level abstractions over the native library functions.

The latest `heartbeats-simple` C libraries can be found at
[https://github.com/connorimes/heartbeats-simple](https://github.com/connorimes/heartbeats-simple).

## Dependencies

In order to use this crate, you should have the `heartbeats-simple` libraries
installed to the system where they can be found by `pkg-config`.

If the libraries are not found, the build process will try to fetch and
compile them.

## Usage
Add `hbs-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.hbs-sys]
git = "https://github.com/connorimes/heartbeats-simple-sys.git"
```
