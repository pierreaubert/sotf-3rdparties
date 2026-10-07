# coreaudio-rs

Upstream: https://github.com/pierreaubert/coreaudio-rs.git. Base: `0.14.2`.
Revision: `29b0407363c479be0b7de681ba6ba050725a5174`.
Revision status: recorded.
License: `MIT/Apache-2.0`; preserved files: `LICENSE-APACHE`, `LICENSE-MIT`.
License evidence status: preserved.

## Why retained

Preserve CoreAudio input buffer allocation capacity and initialize WatchOS callback buffers correctly.

## Retained changes

Two fork commits fix input buffer capacity accounting and first-frame WatchOS initialization.
Read [the existing detail record](../coreaudio-rs/Cargo.toml) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `pinned-git-snapshot`.
Archive SHA-256: `ea13120bd5be7714e6c6af97714ba5de80cf1031a837770dd0108446064e1de7`. Pinned commit subject: Initialize WatchOS callback buffer on first frame.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo check --manifest-path sotf-capture/Cargo.toml --lib
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-07;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
