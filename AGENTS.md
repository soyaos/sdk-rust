# Repository Guidelines

## Development workflow

- Use trunk-based development on `main`; do not keep long-lived branches.
- Keep this crate dependency-free until a functional Rust SDK is approved.
- Preserve the prominent pre-release warning in `README.md` while the API is
  unstable and breaking changes remain possible.
- Never commit crates.io tokens, registry credentials, `.env` files, or build
  artifacts.

## Required checks

Run all checks before delivery:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo package --locked
```

## Releases

- The Git tag must be `v<version>` and match `Cargo.toml` exactly.
- Publish only through `.github/workflows/publish.yml`.
- The first release uses a temporary bootstrap token; all later releases use
  crates.io Trusted Publishing via GitHub Actions OIDC.
