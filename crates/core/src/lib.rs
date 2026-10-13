//! `church-core` holds everything that does not need a UI: it compiles and tests on any host,
//! so CI can run `cargo test -p church-core` quickly without a browser or wasm toolchain.

pub mod manual;
pub mod scripture;
pub mod version;
