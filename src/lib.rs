#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! # Introduction
//! This is the tokio version of <https://wtfm-rs.github.io/doc/wtfm_rs/>.
//! ## Why do we need a separate repo?
//! We want to use [tokio] runtime for async functions.
//! Our minimum `Cargo.toml` only pulls in [tokio] crate
//! so that we can write tests and deep dive into the dependencies.
//! ```toml
//! [package]
//! name = "wtfm-rs-tokio"
//! version = "0.1.0"
//! edition = "2024"
//!
//! [dependencies]
//! tokio = { version = "1.49.0", features = ["full"] }
//! ```
//! ```text
//! % cargo tree
//! └── tokio v1.49.0
//!    ├── bytes v1.11.0
//!    ├── libc v0.2.180
//!    ├── mio v1.1.1
//!    │   └── libc v0.2.180
//!    ├── parking_lot v0.12.5
//!    │   ├── lock_api v0.4.14
//!    │   │   └── scopeguard v1.2.0
//!    │   └── parking_lot_core v0.9.12
//!    │       ├── cfg-if v1.0.4
//!    │       ├── libc v0.2.180
//!    │       └── smallvec v1.15.1
//!    ├── pin-project-lite v0.2.16
//!    ├── signal-hook-registry v1.4.8
//!    │   ├── errno v0.3.14
//!    │   │   └── libc v0.2.180
//!    │   └── libc v0.2.180
//!    ├── socket2 v0.6.2
//!    │   └── libc v0.2.180
//!    └── tokio-macros v2.6.0 (proc-macro)
//!        ├── proc-macro2 v1.0.106
//!        │   └── unicode-ident v1.0.22
//!        ├── quote v1.0.44
//!        │   └── proc-macro2 v1.0.106 (*)
//!        └── syn v2.0.114
//!            ├── proc-macro2 v1.0.106 (*)
//!            ├── quote v1.0.44 (*)
//!            └── unicode-ident v1.0.22
//! ```
//! ## current thread
//! ```
//! tokio::runtime::Builder::new_current_thread()
//!        .enable_all()
//!        .build()
//!        .unwrap()
//!        .block_on(async {
//!            assert!(true);
//!        })
//!
//! ```
//! ## multi-thread
//! ```
//! tokio::runtime::Builder::new_multi_thread()
//!        .enable_all()
//!        .build()
//!        .unwrap()
//!        .block_on(async {
//!            assert!(true);
//!        })
//!
//! ```

//! ## [tokio::main]
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
#[cfg(test)]
#[tokio::test]
async fn current_thread_test() {
    assert!(true);
}

#[test]
fn current_thread_expanded_test() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            assert!(true);
        })
}

#[tokio::test(flavor = "multi_thread")]
async fn multi_thread_test() {
    assert!(true);
}

#[test]
fn multi_thread_expanded_test() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            assert!(true);
        })
}
