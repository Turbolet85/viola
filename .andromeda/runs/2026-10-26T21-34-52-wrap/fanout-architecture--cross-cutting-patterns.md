proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 9
    section: "## Cross-cutting Patterns"
    change: "Add one sub-bullet after line 9 (Config management, the viola home): - As landed (chunk 2026-10-26-home-and-code-bearing-file-integrity): `viola hook`'s event and dialog paths take the `instance_of` session only when its `VIOLA_DIR` instance directory exists (`canonical`) and passes `check_instance` (`trusted`; both in `src/cmd/hook.rs`, shared with `hook statusline`), before `diagnostics_level` is read from `config.json` or obs starts; a `VIOLA_DIR` naming no existing instance directory, or an instance the check refuses, makes the hook exit 0 at once with no `config.json` read, no role file, no stdin read, no line and no frame. The two `--capture` arms, which read no `VIOLA_*`, stay ahead of it."
    sidecar: "cross-cutting-patterns Config management: the hook's home through `VIOLA_DIR` now needs an existing instance directory that passes `check_instance` (report Symbols hook.rs; Count 23, pattern viola-dir-none never fired over the masters)."
    rationale: "The report's Changes (Symbols, `src/cmd/hook.rs`; Deviation 5; Count 23 and the directive's outcome item (b), `inputs#I5`) bring a new home-resolution fact: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session and the hook writes nothing, and a refused instance reads no `config.json`. Line 9 states the `VIOLA_DIR` leg of home resolution and that the home follows `VIOLA_DIR` into the child, but not this condition; no master states it (pattern `viola-dir-none` unfired)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
note: "D-arch-resources and D-arch-decisions find no drift in this file (no env var, config key, flag, crate or dependency added; no ledger row changed), and it holds no passage on the eight retired strict-modes gaps or on `read_snapshot`; Count 23's fact may also land in Occupied Resources (`VIOLA_DIR`) or the Hook contract, so the orchestrator should dedupe across categories."
