# Cascade compare — architecture · wrap · 2026-10-26-home-and-code-bearing-file-integrity
E items 42 · held 21 · lost at classify 3 · lost at strong 0 · beside a placed fact 2 · cascade only 7 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### cross-cutting-patterns#1.1 · lost at classify · .andromeda/architecture-detail/cross-cutting-patterns.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 9
    section: "## Cross-cutting Patterns"
    change: "Add one sub-bullet after line 9 (Config management, the viola home): - As landed (chunk 2026-10-26-home-and-code-bearing-file-integrity): `viola hook`'s event and dialog paths take the `instance_of` session only when its `VIOLA_DIR` instance directory exists (`canonical`) and passes `check_instance` (`trusted`; both in `src/cmd/hook.rs`, shared with `hook statusline`), before `diagnostics_level` is read from `config.json` or obs starts; a `VIOLA_DIR` naming no existing instance directory, or an instance the check refuses, makes the hook exit 0 at once with no `config.json` read, no role file, no stdin read, no line and no frame. The two `--capture` arms, which read no `VIOLA_*`, stay ahead of it."
    sidecar: "cross-cutting-patterns Config management: the hook's home through `VIOLA_DIR` now needs an existing instance directory that passes `check_instance` (report Symbols hook.rs; Count 23, pattern viola-dir-none never fired over the masters)."
    rationale: "The report's Changes (Symbols, `src/cmd/hook.rs`; Deviation 5; Count 23 and the directive's outcome item (b), `inputs#I5`) bring a new home-resolution fact: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session and the hook writes nothing, and a refused instance reads no `config.json`. Line 9 states the `VIOLA_DIR` leg of home resolution and that the home follows `VIOLA_DIR` into the child, but not this condition; no master states it (pattern `viola-dir-none` unfired)."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16

### established-decisions#15.3 · lost at classify · beside a placed fact F8 (near 116, established-decisions#14.1) · .andromeda/architecture-detail/established-decisions.md:125
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 125
    section: "## Established Decisions"
    change: "New bullet after it ([Hook Contract]): The event and dialog paths take the instance from `instance_of(…)`, canonicalise it and run `check_instance` on the home and that instance (`canonical` · `trusted`, shared with `hook statusline`) before the diagnostics level is read or obs is initialised; a refused instance, or a `VIOLA_DIR` naming an instance directory that does not exist, exits 0 at once with no body: no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append (no line on a refused check: the founder's own answer, `inputs#I3` Q2 of chunk 2026-10-26-home-and-code-bearing-file-integrity). The two `--capture` arms stay ahead of it, unchanged."
    sidecar: "established-decisions: the hook's event and dialog paths run the instance check first and fail open silently on a refusal or a missing instance directory."
    rationale: "Report Symbols, src/cmd/hook.rs: canonical / trusted before read_diagnostics_level / viola_obs_init, a refused or absent instance exit 0 at once with nothing read or written; Deviation 5 (VIOLA_DIR naming no directory writes nothing, Count 23); Retired (1), (6)."

### established-decisions#11.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:92
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 92
    section: "## Established Decisions"
    change: "New sentence at the end of the [MCP] paragraph: Each frame tool (`send`, `wait`, `last`, `answer`) runs `check_instance` on the home and the target's instance directory before `live_endpoint`; a refusal is the `instance-unreachable` result with `detail: null`, no frame and no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "established-decisions: viola mcp's frame tools run the instance strict-modes check before the liveness pre-check."
    rationale: "Report Symbols: viola-mcp tools.rs frame runs check_instance(&ctx.home, &instance_dir) before live_endpoint, a refusal result::unreachable(), no frame, no line; Retired (8)."

### established-decisions#20.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:197
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 197
    section: "## Established Decisions"
    change: "New sentence at the end of the [CLI Version Compatibility] entry: Before its `cli` log `viola verify` runs `strict::check_home_tree(&home, None, None)` (the home under the read rule, `ledger/`, `bin/`, `plugin/`, each that exists); a refusal prints the home pair, exit 1, no step line, no summary and no line. The one stamps writer `update_stamps` runs `strict::check_stamps` before it creates `ledger/` and again under its lock before the capped read, returning `StampsError` (`StrictModes` · `State`); a refusal reads and writes nothing, and verify maps it to the same home pair."
    sidecar: "established-decisions: verify's home-tree check before its log and update_stamps' strict-modes check before any read or create."
    rationale: "Report Symbols: verify.rs check_home_tree before its cli log, refuse_home; stamps.rs StampsError, check_stamps before create_private_dir(ledger/) and under the lock; Retired (2); Deviation 1."

### occupied-resources#4.1 · lost at classify · beside a placed fact F19 (near 171, occupied-resources#25.1) · .andromeda/architecture-detail/occupied-resources.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 29
    section: "## Occupied Resources"
    change: "Add to the subcommand exit codes: `send`, `wait`, `last`, `answer`, `pause` and `release` exit 21 when their instance check refuses (`client::refuse_untrusted`: human the instance-state `unable:` / `hint:` pair, under `--json` the `instance-unreachable` document; no role file, no line), as `link` / `unlink` do; `verify` exits 1 with the home-state pair, before its `cli` log and any step line, when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/` or `update_stamps` refuses `ledger/stamps.json`; `run` and `revive`'s start exit 1 before any log when the home tree check refuses (`revive --json` keeps `strict-modes-failed`); on Unix an unstattable home (a path under a regular file) is such a refusal (`verify`'s home pair, the frame verbs' instance pair), while on Windows that path stats `NotFound` and `verify` keeps the `cli` catch-site exit 1 (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "res-binaries: the six frame verbs' exit 21, verify / run / revive start's exit 1 on a refused strict-modes check, the unstattable home per OS."
    rationale: "Report Symbols (client.rs refuse_untrusted, verify.rs 64-70 / 288-296, run.rs checked_exe / refuse_tree) and Counts 8, 9, 13."
    basis: "viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15"

## Held

- conventions#2.1
- conventions#3.1
- conventions#5.1
- established-decisions#10.1
- established-decisions#13.1
- established-decisions#2.1
- established-decisions#21.1
- established-decisions#21.2
- established-decisions#22.1
- established-decisions#24.1
- occupied-resources#1.1
- occupied-resources#10.1
- occupied-resources#15.1
- occupied-resources#16.1
- occupied-resources#18.1
- occupied-resources#2.1
- standard-contracts#12.1
- standard-contracts#14.1
- standard-contracts#2.0
- standard-contracts#3.0
- standard-contracts#5.1

## Cascade only

- established-decisions#12.1
- established-decisions#14.1
- established-decisions#18.2
- occupied-resources#25.1
- occupied-resources#27.1
- standard-contracts#13.1
- standard-contracts#9.1
