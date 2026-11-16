# Proof

Run these checks in order from the repository root. Every command must exit zero.

| Check | Command |
| --- | --- |
| test | `cargo test --locked -j 2` |
| clippy | `cargo clippy --locked --all-targets -j 2 -- -D warnings` |
| fmt | `cargo fmt --check` |
| build | `cargo build --locked -j 2` |

## Requirements

- Engine: Bevy 0.20.1, a Cargo dependency locked in `Cargo.lock`; nothing to install separately.
- Rust 1.97.1 or newer with Cargo and the `clippy` and `rustfmt` components.
- Ubuntu packages from the README: `build-essential`, `pkg-config`, `libx11-dev`, `libxkbcommon-dev`, `libvulkan1`, and `mesa-vulkan-drivers`.

## Coverage

- `build`: `Cargo.lock`
- `build`: `Cargo.toml`
- `build`: `src/**`
- `clippy`: `Cargo.lock`
- `clippy`: `Cargo.toml`
- `clippy`: `src/**`
- `fmt`: `Cargo.toml`
- `fmt`: `src/**`
- `test`: `Cargo.lock`
- `test`: `Cargo.toml`
- `test`: `src/**`
- Review-only: `.github/workflows/proof.yml`
- Review-only: `.gitignore`
- Review-only: `AGENTS.md`
- Review-only: `CLAUDE.md`
- Review-only: `LICENSE`
- Review-only: `README.md`
- Review-only: `docs/agents/**`
