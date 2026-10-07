# nalgebra

Upstream: https://github.com/pierreaubert/nalgebra.git. Base: `0.35.0`.
Revision: `a42573dbc9649af62fb07b469c98c852e62720fd`.
Revision status: recorded.
License: `Apache-2.0`; preserved files: `LICENSE`, `nalgebra-glm/LICENSE`, `nalgebra-lapack/LICENSE`, `nalgebra-macros/LICENSE`, `nalgebra-sparse/LICENSE`.
License evidence status: preserved.

## Why retained

Use the supported glam 0.33 conversion without retaining older conversion dependencies.

## Retained changes

Pinned fork removes unsupported older glam conversion manifest/source entries.
Read [the existing detail record](../nalgebra/Cargo.toml) before updating.
This inventory review does not claim a fresh upstream parity or security audit.

## Source provenance

Import kind: `pinned-git-snapshot`.
Archive SHA-256: `abad57157995caf98e6365d5f0c6b8890f49d7575576d82c80a3db59f0512ecf`. Pinned commit subject: Keep only supported glam 0.33 conversion.

## Validation and removal

From the aggregate `all_of_sotf` directory:

```sh
cargo check --manifest-path sotf-daw/Cargo.toml -p sotf-plugins
```

These are update gates, not a statement that all platform-specific tests ran for
this relocation. Use the owning consumer's feature set and locked resolution.
Replace this source only when a selected upstream release covers the retained behavior and the listed consumer regression checks pass.

Owner: SOTF dependency maintainers. Inventory reviewed 2026-10-07;
re-evaluate the upstream delta within 90 days and on every update.
Follow [the collection update procedure](../README.md#updating-a-fork).
