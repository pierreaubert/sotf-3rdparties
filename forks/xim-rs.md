# xim-rs

Upstream: https://github.com/zed-industries/xim-rs.git. Base: `0.4.0-zed`.
Revision: `16f35a2c881b815a2b6cdfd6687988e84f8447d8`.
Revision status: recorded.
License: `MIT`; preserved files: `LICENSE`.
License evidence status: preserved.

## Why retained

Retain the Zed XIM fork used by the Linux GPUI backend.

## Retained changes

Exact selected Zed fork; Linux input method behavior is not qualified by the macOS build.
Read [the existing detail record](../xim-rs/Cargo.toml) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `pinned-git-snapshot`.
Archive SHA-256: `f873b8a438f2ad86a7417b2ebbed3c1dd32e0a7d4dc75e2640de170230edc2fc`. Pinned commit subject: Merge pull request #1 from mikayla-maki/main.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo check --manifest-path sotf-3rdparties/gpui/Cargo.toml -p gpui-toolkit-gpui-linux --target aarch64-unknown-linux-gnu
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-07;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
