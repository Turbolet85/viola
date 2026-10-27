proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 10
    section: '## 6. Log Coverage'
    change: 'In the `process-start` row, `sideload_fallback` reads: (claude-child, Windows, present only on a degrade: `hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`; D-36; `strict-modes-failed` when the `conpty/` companions'' owner/DACL check (`strict::check_present` on `bin/<key>/conpty/` and on each companion that exists, inside `pin_companions`) refuses, `PinError::StrictModes`, the child left on the inbox ConPTY with its exit unchanged; chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    sidecar: 'log-coverage.md:10 — `process-start.sideload_fallback` gains `strict-modes-failed` (4 → 5 words): the `conpty/` companions'' owner/DACL leg built (chunk 2026-10-26-home-and-code-bearing-file-integrity, Count 7).'
    rationale: 'Report Schema / config: `schemas/diag-line.v1.json` line 70, the `process-start.sideload_fallback` enum gains `strict-modes-failed`; Symbols (`viola_state::pin`: `pin_companions` runs `check_present`, `PinError::StrictModes`; `sideload_outcome` maps it to `strict-modes-failed`); Coverage of new surfaces (`conpty/` leg → instrumentation `sideload_fallback` `strict-modes-failed`); Count 7 names log-coverage.md:10 among the 6 stale word-list rows.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:24
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 35
    section: '## 6. Log Coverage'
    change: 'In the Vector 7 item, replace «and the nine hook events run no instance check» with «and the nine hook events'' event and dialog paths now run the instance check too (`src/cmd/hook.rs` `canonical` / `trusted`, before `read_diagnostics_level` and `viola_obs_init`), a refused or absent instance leaving no line either — exit 0, no `hook-decision`, no frame, no SessionEnd append (chunk 2026-10-26-home-and-code-bearing-file-integrity)».'
    sidecar: 'log-coverage.md:35 — the hook events'' «no instance check» retired: the event and dialog paths check the instance before obs init and a refusal writes no line (chunk 2026-10-26-home-and-code-bearing-file-integrity, Retired gaps 1 and 6).'
    rationale: 'Report Retired: gaps (1) `viola hook`''s `hook.event` read without strict-modes and (6) `hook.dialog` — each now runs the strict-modes check before its first read or open; Symbols `src/cmd/hook.rs`: `instance_of(…).and_then(canonical).filter(trusted)` before obs init, a refused instance returns exit 0 with no line; Count 20 names log-coverage.md:35 among the no-line exemption rows to re-read.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 36
    section: '## 6. Log Coverage'
    change: 'Add after line 36 a continuation line of the Vector 7 item: « From chunk 2026-10-26-home-and-code-bearing-file-integrity the strict-modes check runs before the first read or open at every entry point, and each refusal writes no line (the founder''s word, `inputs#I3` Q2, as `link` / `unlink`): `send`, `wait`, `last`, `answer`, `pause` and `release` through `client::refuse_untrusted` (`check_instance` on the home and `instances/<name>`) exit 21 with the instance pair, or under `--json` the `instance-unreachable` document, with no `process-start` and no `process-exit`; `viola mcp`''s `send` / `wait` / `last` / `answer` tools return `instance-unreachable` with `detail: null` and no line; `viola verify`''s `check_home_tree` (the home, `ledger/`, `bin/`, `plugin/`) and `update_stamps`'' `StampsError::StrictModes` print the home pair and exit 1 before its `cli` log, no step line; `viola run`''s and `viola revive`''s start (`check_home_tree` with the name and the pin key) exit 1 before the wrapper log opens; `viola hook` with a `VIOLA_DIR` naming no existing instance directory writes nothing; on Unix a home under a regular file is such a refusal where the catch-site `internal-error` line stood (on Windows it stats `NotFound` and the catch-site line stands); and a `diagnostics/` folder or role or detail file that `strict::check_diagnostics` refuses (a symlink, or a mode another user may read, or on Windows write) is a failed open: the role writer ends at `std::io::sink` and a detail line is dropped. `strict-modes-failed` reaches a line only on `parse-rejected{parser:"ledger-stamps"}` and on `process-start{subject:"claude-child"}`''s `sideload_fallback`. »'
    sidecar: 'log-coverage.md:36+ — the new no-line strict-modes refusal sites (six frame verbs, MCP frame tools, `verify`, `run` / `revive` start, the hook paths, the diagnostics opens) and where `strict-modes-failed` still reaches a line (chunk 2026-10-26-home-and-code-bearing-file-integrity, Counts 13, 20, 23).'
    rationale: 'Report Symbols (`refuse_untrusted`, `tools.rs` `frame`, `verify.rs` 64-70 / 288-296, `checked_exe` / `refuse_tree`, `src/obs.rs` `check_diagnostics`), Coverage of new surfaces (each instrumentation n/a: no line by `inputs#I3` Q2; diagnostics opens end at `sink`), Count 13 (unstattable home), Count 23 (`VIOLA_DIR` naming no directory) and Count 20 (log-coverage.md:35 an exemption-list row to extend); line 35 is near the 2 000-char entity bound, so the detail takes its own continuation line as line 36 does.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:47
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 26
    section: '## 6. Log Coverage'
    change: 'In the exit-1 item, replace «the start arm''s instance check (`check_instance`) runs before the wrapper log opens and outside the preflight» with «the start arm''s instance check (`check_instance`, and since chunk 2026-10-26-home-and-code-bearing-file-integrity `strict::check_home_tree` through `run::checked_exe` over the home, the instance, `ledger/`, `bin/`, `plugin/` and the pinned copy, which `viola run`''s start runs too) runs before the wrapper log opens and outside the preflight».'
    sidecar: 'log-coverage.md:26 — the start arm''s pre-log check is `check_home_tree` (via `checked_exe`), shared by `viola run`''s start: a refused tree is exit 1 with no log (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report Symbols `src/cmd/run.rs`: `checked_exe` (189-200) runs `check_home_tree(home, Some(name), Some(&key))`; `run` (166-172) and `revive`''s start (revive.rs:125-135) call it and `refuse_tree` before `open_wrapper_log`: exit 1, no log opened, no line, no child; revive `--json` keeps `strict-modes-failed`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
note: 'The `sideload_fallback` word is an enum extension owed its D-36-class Decisions Log entry (report Expected amendments, obs-plan); body lines outside this file owe companions — obs-plan.md:215 (a strict-modes failure now also has the `sideload_fallback` line) and :361 (`cannot-occur`, Count 11); D-obs-stack and D-obs-pii found no drift (no dependency change; every new refusal is codes-only or line-free).'
