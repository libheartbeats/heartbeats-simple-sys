# Simple Heartbeats Rust Bindings

The `heartbeats-simple-sys` crate provides declarations and linkage for the
`heartbeats-simple` C libraries.
Following the *-sys package conventions, the `heartbeats-simple-sys` crate
does not define higher-level abstractions over the native `heartbeats-simple`
library functions.

The latest `heartbeats-simple` C libraries can be found at
[https://github.com/libheartbeats/heartbeats-simple](https://github.com/libheartbeats/heartbeats-simple).

## Dependencies

In order to use the `heartbeats-simple-sys` crate, you should have the
`heartbeats-simple` libraries installed to the system where they can be found
by `pkg-config`.

If the libraries are not found, the build process will try to fetch and
compile them.

## Usage
Add `heartbeats-simple-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.heartbeats-simple-sys]
git = "https://github.com/libheartbeats/heartbeats-simple-sys.git"
```
