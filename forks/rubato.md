# rubato

Upstream: https://github.com/HEnquist/rubato. Base: `5.0.0`.
Revision: `6b72d0f9d8843c6623c818751730764aefcd0525`.
Revision status: recorded.
License: `MIT OR Apache-2.0`; preserved files: `LICENSE-APACHE`, `LICENSE-MIT`, `LICENSE.txt`.
License evidence status: preserved.

## Why retained

SOTF resampling needs prepared cutoff tables and exact variable-ratio output sizing.

## Retained changes

Prepared cutoff and sizing behavior documented in SOTF_FORK.md.
Read [the existing detail record](../rubato/SOTF_FORK.md) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `retained-local-fork`.
Original path: `math-audio/crates/3rdparties/rubato`. Retained source repository/commit: `sotf-3rdparties@bfd07543a9f61040d70f8a1f5aa644fa80c7fb60`.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo test --manifest-path sotf-3rdparties/rubato/Cargo.toml --lib
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-07;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
