//! # Introduction
//! This is the tokio version of <https://wtfm-rs.github.io/doc/wtfm_rs/>.
//! ## Why do we need a separate repo?
//! We want to use [tokio] runtime for async functions.
//! Our minimum `Cargo.toml` only pulls in [tokio] crate
//! so that we can write tests/examples and review the minimum amount of
//! dependencies.
//! ```toml
//! [package]
//! name = "wtfm-rs-tokio"
//! version = "0.1.0"
//! edition = "2024"
//!
//! [dependencies]
//! tokio = { version = "1.49.0", features = ["macros", "process", "rt-multi-thread"] }
//! ```
//! ```
//! use tokio::process::Command;
//!
//! async fn hello_world() -> String {
//!    let output = Command::new("echo").arg("Hello,").arg("world!").output();
//!    let output = output.await.expect("No such file or directory");
//!    String::from_utf8(output.stdout).expect("Format error")
//! }

//! #[tokio::main]
//! async fn main() {
//!    assert_eq!(hello_world().await, "Hello, world!\n");
//! }
//! ```
//!
