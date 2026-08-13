//! Integration checks for the intentionally small public API.

use soyaos::{DEVELOPMENT_STATUS, is_functional_sdk};

#[test]
fn public_status_is_explicit() {
    assert!(!is_functional_sdk());
    assert!(DEVELOPMENT_STATUS.contains("APIs are unstable"));
}
