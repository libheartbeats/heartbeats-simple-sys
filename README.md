# Simple Heartbeats Rust Bindings

The `heartbeats-simple-sys` crate provides declarations and linkage for the
`heartbeats-simple` C libraries.
Following the *-sys package conventions, the `heartbeats-simple-sys` crate
does not define higher-level abstractions over the native `heartbeats-simple`
library functions.

The latest `heartbeats-simple` C libraries can be found at
[https://github.com/connorimes/heartbeats-simple](https://github.com/connorimes/heartbeats-simple).

## Dependencies

In order to use the `heartbeats-simple-sys` crate, you must have the
`heartbeats-simple` libraries installed to the system.

## Usage
Add `heartbeats-simple-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies.heartbeats-simple-sys]
git = "https://github.com/connorimes/heartbeats-simple-sys.git"
```
