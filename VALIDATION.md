# Relocation validation

Validated locally on Apple Silicon with Rust 1.99.0 and
`MACOSX_DEPLOYMENT_TARGET=15.0` on 2026-10-07. This is source relocation acceptance,
not a claim that the complete macOS release or every upstream platform is qualified.

## Integrity

- 23 relocated/imported source directories, 5,755 retained files, plus the four previously migrated shared forks.
- 2,645 Rust files and 106 license files are byte-identical.
- Twelve pinned Git snapshots were imported with `git archive` at the exact revisions in `FORKS.json`.
- Two inherited manifests became standalone: dst-decoder edition/anyhow and nnnoiseless criterion. NIH dependency paths and moved maintenance notes were updated.
- All nine top-level consumer manifests and the AutoEQ GPUI example manifest pass locked, offline metadata parsing. The standalone moved audio/Librespot manifests parse independently.

## Checks

| Check | Result |
| --- | --- |
| Collection inventory / licenses / consumer patch paths | 34 records and 126 local patch declarations pass |
| SOTF GPUI with shipping features | cargo check passes after final relocation |
| Capture library | cargo check passes |
| AU/VST3 worker and macOS sandbox helper | cargo check passes |
| Symphonia DSD / DST decoder tests | 5 pass |
| Relocated NIH state-return helper | 3 existing tests pass in an isolated harness with the allocation guard enabled |
| Required scaffold path regression | 1 passes |
| Release-tooling regressions | 62 pass |

The scoped scaffold regression validates the required dependency-path edit; standalone
GPUI Toolkit and AutoEQ product QA remain outside the current release scope.

## Resolution and limitations

The SOTF, DAW, Capture, AutoEQ, Toolkit, Sofa Reader and example macOS dependency
graphs resolve offline. SOTF, Capture, AutoEQ, Toolkit, Sofa Reader, and example
lock refreshes retain package versions. DAW drops unused ordered-float 5.5.0 after
nnnoiseless stops being a workspace member. Systemwide catches up to the existing
DAW 0.8.1 / toml 1.1 source edits; those version changes predate this relocation.

Full standalone math-audio resolution is blocked by a pre-existing offline index
gap for its locked jiff-tzdb 0.1.9. Its nalgebra/rusty-fork lock entries were moved
to local source identity without changing versions; the affected math sources
compile in the SOTF consumer build.

The broader plugins-nih library test build fails on seven existing private-type
references in params_hiss_profile_tests.rs (ParameterId / ParameterValue imported
through sotf_host::plugin). The isolated check compiles the actual relocated
gui_state_return_tests.rs and its referenced vendor helper, preserving allocation
assertions. The unrelated broader failure is not represented as a passing gate.

Some imported path dependencies emit existing compiler warnings. No blanket
vendor formatting or lint suppression was applied. Block declares MIT in its
upstream manifest but its original import has no separate license file; its fork
record explicitly documents that evidence gap.

No GitHub push, publication, native device routing, signing or notarization was
performed for this migration. The expanded consolidation remains local and
uncommitted; aggregate source pins must advance with the eventual collection commit.

Reproducible commands and detailed logs/receipts are retained locally under
`/Volumes/home_tmp/cache/mbx/patched-crate-consolidation-20261007`.

## GPUI scope expansion

The complete toolkit vendor closure is now in `gpui/crates`: thirteen previously
vendored crates and six exact-revision Zed compatibility support crates. The new
GPUI workspace owns their inherited settings; support manifests preserve their
originals as UPSTREAM_CARGO.toml. Text rendering, screen capture and XIM source
snapshots also live here at their existing pins (`cosmic-text`, `scap`, `xim-rs`).

All 224 GPUI Rust files, 11 support Rust files and 11 embedded-asset files retained
their bytes. Consumer Cargo paths, the required scaffold source include, vendor
inventory and importer destination were updated. SOTF-owned toolkit crates remain
in gpui-toolkit. No third-party crate source remains in the former donor
crates/3rdparties directories.

The scoped dependency-graph audit reports existing version/source alignment
findings and three unsupported inherited dependency references in upstream Tract
benchmark/debug manifests. It is not a passing aggregate release gate. This
consolidation does not perform cross-workspace version upgrades to suppress those
findings. The detailed static report is retained with the migration receipts.

Upstream cosmic-text LFS test fonts and images are fully fetched at the pinned
revision and stored in this repository's LFS storage (`git lfs fsck` passes),
so the vendored copy is self-contained. SOTF's actual embedded fonts remain
available under gpui/assets. Standalone cosmic-text font fixture QA is not
claimed by the SOTF compile check.

Final expanded-scope acceptance: the SOTF GPUI shipping-feature check passes
with all relocated GPUI dependencies. All nine consumers, the AutoEQ GPUI example,
and the shared GPUI workspace pass locked/offline manifest parsing. The scoped
scaffold path regression and AU/VST3 helper checks also pass after moving GPUI.
Toolkit's lock drops thirteen development-only packages after its vendor crates
leave that workspace; no replacement versions were introduced. Maintenance
Python scripts compile successfully. Aggregate source pins remain pending the
future local commits and synchronization.
