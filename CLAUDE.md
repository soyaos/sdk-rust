# AI Collaboration Guide

Follow [AGENTS.md](AGENTS.md). Work on `main`, keep the crate dependency-free,
and never commit registry credentials, `.env` files, `.crate` files, or
`target/` artifacts. Do not remove or weaken the pre-release warning in
`README.md` until a maintainer explicitly approves the first stable release.

Run every check before delivery:

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo package --locked
```

Publish only through `.github/workflows/publish.yml` with a `v<version>` tag
that exactly matches `Cargo.toml`. The first release uses a temporary bootstrap
token; later releases must use crates.io Trusted Publishing via OIDC.
