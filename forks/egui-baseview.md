# egui-baseview

Upstream: https://github.com/BillyDM/egui-baseview.git. Base: `0.5.0`.
Revision: `ec70c3fe6b2f070dcacbc22924431edbe24bd1c0`.
Revision status: recorded.
License: `MIT`; preserved files: `LICENSE`.
License evidence status: preserved.

## Why retained

nih-plug-egui pins egui-baseview from git, and crates.io forbids git
dependencies, so the pinned source is vendored here for `sotf-egui-baseview`
publication.

## Retained changes

Exact pinned snapshot; no SOTF Rust patch. SOTF manifest maintenance only:
`baseview` git pin replaced by the vendored `sotf-baseview` path,
`screenshot.png` excluded from the published package, package renamed
`sotf-egui-baseview` for publication.
Read [the existing detail record](../egui-baseview/Cargo.toml) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `pinned-git-snapshot`.
Archive SHA-256: `4f050062675a8f31600d4778815317e377ac182568007deafc7fc5084b0efa39`. Pinned commit subject: update to egui 0.31.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo check --manifest-path sotf-3rdparties/egui-baseview/Cargo.toml --lib
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-08;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
