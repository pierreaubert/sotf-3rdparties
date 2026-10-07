# dst-decoder

Upstream: https://github.com/bleggett/dst-decoder. Base: `0.1.2`.
Revision: `not recorded`.
Revision status: exact upstream commit not recorded; retained source commit identifies the imported fork.
License: `Apache-2.0`; preserved files: `LICENSE`.
License evidence status: preserved.

## Why retained

Supply DST decoding for SOTF SACD integration with retained fixture and lint fixes.

## Retained changes

Retained decoder tests/fixture gating and lint maintenance; inherited edition/anyhow made explicit during relocation.
Read [the existing detail record](../dst-decoder/README.md) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `retained-local-fork`.
Original path: `symphonia-add-ons/dst-decoder`. Retained source repository/commit: `symphonia-add-ons@53c255d29ceaf61bbaa14f573a0b44341c5ae452`.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo test --manifest-path sotf-3rdparties/dst-decoder/Cargo.toml --lib
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-07;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
