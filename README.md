# soyaos

> [!WARNING]
> **Development project — not formally released.** This crate is an early
> pre-release placeholder. Its APIs may change or disappear without notice,
> breaking changes can happen at any time, and the functionality is not stable.
> Do not use it in production.

Official Rust package namespace for [SoyaOS](https://soyaos.ai), an Agent
Operating System.

## Current status

Version `0.0.0-alpha.0` reserves the official `soyaos` package name and exposes
development-status metadata only. It is **not a functional SDK** and does not
make network requests. Use the official
[TypeScript SDK](https://github.com/soyaos/sdk-ts) or
[Python SDK](https://github.com/soyaos/sdk-python) for current experiments.

```rust
assert!(!soyaos::is_functional_sdk());
println!("{}", soyaos::DEVELOPMENT_STATUS);
```

## Install

Pre-release versions must be requested explicitly:

```bash
cargo add soyaos@=0.0.0-alpha.0
```

Installing this placeholder is normally unnecessary. A future functional Rust
SDK will be announced in the [SoyaOS repository](https://github.com/soyaos/soyaos).

## Development

Rust `1.85.0` or newer is required.

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo package --locked
```

## Operations

This repository builds a dependency-free Rust library; it does not deploy a
server, use Kubernetes, emit distributed traces, or depend on runtime external
services. GitHub Actions provides CI and registry publishing. Operational logs
are therefore the logs of the `CI` and `Publish to crates.io` workflows.

## Release safety

- Publishing is allowed only from a GitHub Release whose `vX.Y.Z` tag matches
  `Cargo.toml`.
- The initial `0.0.0-alpha.0` release used a narrowly scoped, temporary
  crates.io API Token.
- All later releases use crates.io Trusted Publishing with GitHub Actions OIDC;
  the repository does not require a long-lived publish token.
- Registry credentials and tokens must never be committed to this repository.

## License

[MIT](LICENSE) — © 2026 SoyaOS Contributors.
