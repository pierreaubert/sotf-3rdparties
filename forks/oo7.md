# oo7

Upstream: https://github.com/linux-credentials/oo7. Base: `0.7.0`.
Revision: `9070389f33bec2e47048384e2fdbd7aab64e0df7`.
Revision status: recorded.
License: `MIT`; preserved files: `LICENSE`.
License evidence status: preserved.

## Why retained

Keep Linux keyring crypto dependencies compatible with the selected RustCrypto family.

## Retained changes

AES/CBC/cipher/digest/HKDF/HMAC/PBKDF2/SHA/MD5 API and dependency maintenance, plus standalone workspace.
Read [the existing detail record](../oo7/Cargo.toml.orig) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `retained-local-fork`.
Original path: `gpui-toolkit/crates/3rdparties/crypto-forks/oo7`. Retained source repository/commit: `gpui-toolkit@a145af14bd8d9679487f42c0dfb3f50897668a02`.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo check --manifest-path sotf-3rdparties/oo7/Cargo.toml --lib --target aarch64-unknown-linux-gnu
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-07;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
