//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Enable the feature "pass" for the tests
    println!("cargo:rustc-cfg=feature=\"pass\"");

    // Set the environment variable TEST_FOO to the current timestamp
    let start = SystemTime::now();
    let timestamp = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}