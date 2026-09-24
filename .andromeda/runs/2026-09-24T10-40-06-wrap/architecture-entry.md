
## 2026-09-24-diagnostics-plane — diagnostics roots, config key, diag schemas, v exception
**Section:** §Stack and Technologies (ORM / migrations row) · §Established Decisions [Hook Contract] · §Occupied Resources → Filesystem (`config.json`, `diagnostics/`, `instances/<ViolaName>/…` rows) · §Occupied Resources → Repository (`schemas/` rows) · §Infrastructure Patterns → Project directory structure (`schemas/` comment) · §Cross-cutting Patterns → Config management (Settings) · §Cross-cutting Patterns → Diagnostic output channels
**Change:**
- **`v` rule:** "every record carries `v`" now names its exception. Process-log lines carry no `v`, and their version lives in the schema filename.
- **Logging destinations:** `hook` logs to the home-level `diagnostics/hook-<name>.ndjson`, with content-bearing detail only in the instance's `detail-hook.ndjson`. Every role's codes-only log goes to its home-level role file (`run-`/`hook-`/`mcp`/`ui-`/`cli-`). `mcp` falls back to stderr JSON only when its file cannot be opened.
- **Registry and settings:**
  - The `config.json` row registers `diagnostics_level` (info | debug), the `MAX_FRAME`-capped read that requires `v` = 1, and the `parse-rejected` report.
  - The home `diagnostics/` row lists all five role files; only `run` has a producer today.
  - The instance `diagnostics/` row names the owner-only `detail-<role>.ndjson` files.
  - `schemas/diag-line.v1.json` and `diag-detail.v1.json` are registered in the Repository rows and the tree.
  - The Settings bullet names `diagnostics_level` as the only source of the level; `RUST_LOG` has no effect.
**Why:** chunk 2026-09-24-diagnostics-plane shipped these facts: report Changes → Schema/config and Counts, and Spec claims disproved #2. All 9 D-arch fan-out proposals were applied, re-derived. The `v` exception is also the operator's P5 direction.

This pass's sweep covered all seven masters, CLAUDE.md, `.claude/rules/*`, `.claude/docs/**`, playbook and drift-base. Patterns: `Every record carries .v.|every (viola )?format carries`, `reserved for .hook.|diagnostics go only to the instance|diagnostics go to stderr|run-<name>.ndjson. today|Its diagnostics go`, `test-side JSON schemas`, `budget thresholds, GUI port\)`, plus the mechanism reads `` `mcp`.{0,80}stderr ``, `when .VIOLA_DIR. is set`, `diagnostics go (only )?to`, `in-session .mcp. diagnostics`.
- **Amended:**
  - arch :21, :67, :365, :369, :370, :378 (+1 row), :435, :483, :492;
  - obs :554 cited the retired arch wording as a verbatim quote and was rewritten to cite §Diagnostic output channels.
- **No change:**
  - obs :202, :45, :286 sit in obs §1, a verbatim copy of obs-scope whose pending wording is kept by rule (obs :485).
  - obs :485, :635, :1298, :1749 already state D-09.
- **Routed:** CLAUDE.md :119 (`USER:session-learnings`) goes to P3 curation, per the operator's directive.
- **Leaf re-derived:** `.claude/docs/stack.md:17`.
- **Control:** `unevaluatedProperties` fired 2 hits in obs.
