@amendment marker=2026-11-05-linux-only-code-and-ci
title: residual Windows reasons and the one-leg lint job
section: §Occupied Resources (body: `target/harness/`; occupied-resources.md: the orphans-check control); established-decisions.md ([Database / State Store]); stack-and-technologies.md (Code quality row); contracts/build-system (the orphans and rmcp-features gates); contracts/crate-dependency-direction (the root bin's Windows dev-dependency, retired)
change:
  - `target/harness/` is the harness's own cargo target dir; was that, «because a running `target/debug/viola-harness.exe` cannot be relinked on Windows».
  - The orphans-check control is a control file gated on the CI triple's own OS (`cfg(target_os = "linux")`); was a `cfg(windows)`/`cfg(unix)` file-pair control.
  - The separate lock files read as a layout chosen while Windows, where append + exclusive lock fails (rust-lang/rust#54118), was a target, and kept; was that pair stated as failing, present tense.
  - `viola-harness gate` enforces the coverage floors (was the per-OS floors); the orphans and rmcp-features gates run in the `lint` job (was the per-OS `lint` job).
  - Retired: the root bin's test-only Windows dev-dependency on windows-sys (its two features for `tests/security_negatives_channel.rs`'s pipe-type read-back); no manifest names windows-sys.
why: the Windows target, `windows-sys` and `tests/security_negatives_channel.rs` left with the chunk (the founder's own rulings of 2026-11-04, inputs#I1); `scripts/orphans-check.sh` narrows to one triple; a residual census after the cascade sweep found this wording no sweep pattern named (`cascade-dispositions.md` §Residual census).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
