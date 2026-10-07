# nih-plug

Upstream: https://github.com/robbert-vdh/nih-plug. Base: `de421011f41a6d10fc8c7a6084e4f4dee0143683`.
Revision: `de421011f41a6d10fc8c7a6084e4f4dee0143683`.
Revision status: recorded.
License: `ISC`; preserved files: `LICENSE`.
License evidence status: preserved.

## Why retained

Preserve native plugin automation, transport, state return, tail bounds and auxiliary bus contracts.

## Retained changes

Detailed timing, state, tail and routing fixes are recorded in the fork README; no GUI re-vendor.
Read [the existing detail record](../nih-plug/README.md) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `retained-local-fork`.
Original path: `sotf-daw/crates/3rdparties/nih-plug`. Retained source repository/commit: `sotf-daw@bbb0a8b924bbb368631f0d6a7426af25e66878f6`.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo test --manifest-path sotf-daw/Cargo.toml -p plugins-nih --no-default-features --features gain --lib wrapper::transport
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-07;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
