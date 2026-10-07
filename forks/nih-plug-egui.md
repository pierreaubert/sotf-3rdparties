# nih-plug-egui

Upstream: https://github.com/robbert-vdh/nih-plug. Base: `28b149ec4d62757d0b448809148a0c3ca6e09a95`.
Revision: `28b149ec4d62757d0b448809148a0c3ca6e09a95`.
Revision status: recorded.
License: `ISC`; preserved files: `LICENSE`.
License evidence status: preserved.

## Why retained

Keep the optional existing NIH GUI adapter paired with the maintained NIH source.

## Retained changes

Pinned adapter, sibling nih_plug path and explicit f32 stroke-width literal; this GUI is outside current release priority.
Read [the existing detail record](../nih-plug-egui/UPSTREAM.md) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `retained-local-fork`.
Original path: `sotf-daw/crates/3rdparties/nih-plug-egui`. Retained source repository/commit: `sotf-daw@bbb0a8b924bbb368631f0d6a7426af25e66878f6`.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo check --manifest-path sotf-3rdparties/nih-plug-egui/Cargo.toml --lib
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-07;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
