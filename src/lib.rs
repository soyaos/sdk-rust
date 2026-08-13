#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// The maturity notice attached to this pre-release package.
pub const DEVELOPMENT_STATUS: &str =
    "Pre-release placeholder: APIs are unstable and the functional Rust SDK is not implemented.";

/// Returns whether this package currently contains a functional SoyaOS SDK.
///
/// Version `0.0.0-alpha.0` deliberately returns `false`. Consumers should use
/// the TypeScript or Python SDK until a functional Rust release is announced.
#[must_use]
pub const fn is_functional_sdk() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::{DEVELOPMENT_STATUS, is_functional_sdk};

    #[test]
    fn identifies_itself_as_a_non_functional_prerelease() {
        assert!(!is_functional_sdk());
        assert!(DEVELOPMENT_STATUS.contains("Pre-release"));
        assert!(DEVELOPMENT_STATUS.contains("not implemented"));
    }
}
