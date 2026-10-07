# GPUI framework and support sources

This self-contained workspace owns the thirteen GPUI/Zed vendor crates formerly
in `gpui-toolkit/crates/3rdparties`, plus six support crates from the exact selected
compatibility fork. SOTF-owned toolkit UI components, themes, builders and apps
remain in the sibling `gpui-toolkit` repository.

## Why these sources are retained

The GPUI closure carries embedded application lifetime, Apple platform fixes,
renderer/custom draw behavior, line wrapping, runtime compatibility and other
recorded patches. Each crate's existing record below gives the upstream base,
retained changes and verification requirements; consult it before updating.
The previous toolkit inventory is preserved in `PREVIOUS_VENDOR_NOTES.md` as
historical evidence, not the current dependency-resolution specification.

| Source / patch record | Local package |
| --- | --- |
| [gpui](crates/gpui/VENDORED.md) | `gpui-toolkit-gpui` 0.2.3 |
| [gpui_linux](crates/gpui_linux/VENDORED.md) | `gpui-toolkit-gpui-linux` 0.1.0 |
| [gpui_macos](crates/gpui_macos/VENDORED.md) | `gpui-toolkit-gpui-macos` 0.1.2 |
| [gpui_macros](crates/gpui_macros/VENDORED.md) | `gpui-toolkit-gpui-macros` 0.1.0 |
| [gpui_web](crates/gpui_web/VENDORED.md) | `gpui-toolkit-gpui-web` 0.1.1 |
| [gpui_wgpu](crates/gpui_wgpu/VENDORING.md) | `gpui-toolkit-gpui-wgpu` 0.1.2 |
| [gpui_windows](crates/gpui_windows/VENDORING.md) | `gpui-toolkit-gpui-windows` 0.1.2 |
| [http_client](crates/http_client/VENDORED.md) | `gpui-toolkit-http-client` 0.1.0 |
| [perf](crates/perf/VENDORED.md) | `gpui-toolkit-perf` 0.1.0 |
| [scheduler](crates/scheduler/VENDORED.md) | `gpui-toolkit-scheduler` 0.1.0 |
| [sum_tree](crates/sum_tree/VENDORED.md) | `gpui-toolkit-sum-tree` 0.1.0 |
| [util](crates/util/VENDORED.md) | `gpui-toolkit-util` 0.1.1 |
| [util_macros](crates/util_macros/VENDORED.md) | `gpui-toolkit-util-macros` 0.1.0 |

## Pinned Zed support

`collections`, `gpui_util`, `gpui_shared_string`, `media`, `refineable`, and
`refineable/derive_refineable` were imported from `pierreaubert/zed` at
`3a0ea890ddf8e6247e38c795a960eb18f5182113`. Their Rust source is unchanged.
Each `UPSTREAM_CARGO.toml` preserves the original inherited manifest; the active
manifest makes those inherited values explicit and rebases local paths. No editor
or unrelated Zed application crates were imported. Original root license files
and README are preserved as `UPSTREAM_ZED_*`; review per-package declarations,
including packages that did not originally declare a separate license.

## Workspace and assets

`Cargo.toml` preserves the former toolkit's inherited dependency, feature and
lint policies for the nineteen source members. Its shared support dependencies
point at the local source above. `assets/fonts` preserves the embedded font bytes,
readmes and licenses needed by native/web rendering. The source include paths
retain their original relative layout.

## Updating and validation

From `all_of_sotf`, run the collection verifier and the SOTF GPUI shipping build:

```sh
python3 sotf-3rdparties/scripts/verify.py
cargo check --manifest-path sotf/Cargo.toml -p sotf-gpui --features shipping
```

Run the relevant per-crate regressions when changing behavior; macOS compilation
does not qualify Windows/Linux/web targets. The import script in the sibling
toolkit (`scripts/import_gpui_upstream.py`) now targets this group's `crates`
directory. Preserve local patches and source pins when using it.
Follow the [collection update procedure](../README.md#updating-a-fork).

The renderer's `cosmic-text` pin and Zed `scap` / `xim-rs` forks are maintained as
separate source groups in the parent collection. Their revisions and purposes
are listed in `../FORKS.json`; GPUI dependency paths resolve to those local copies.
