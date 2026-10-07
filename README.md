# Shared third-party forks

This repository owns the shared vendored crates used by the sibling workspaces
in `all_of_sotf`. Each crate keeps its upstream licenses and fork provenance.
There is no root Cargo workspace; consumers use local paths to individual crates.

| Directory | Purpose |
|-----------|---------|
| `rubato` | Shared SOTF Rubato 5.0.0 resampling fork |
| `derive_more` | Derive macro facade |
| `derive_more-impl` | Derive macro implementation using the maintained case conversion dependency |
| `predicates` | Shared test predicate fork |

These crates were moved from `math-audio/crates/3rdparties` without changing
their source contents. Sibling Cargo manifests point here directly.

Read
[`rubato/SOTF_FORK.md`](rubato/SOTF_FORK.md) before updating its upstream base
or changing its realtime behavior.
