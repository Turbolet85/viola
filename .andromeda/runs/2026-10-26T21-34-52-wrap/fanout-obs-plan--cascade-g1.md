proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 123
    section: '## 1. Obs Scope Summary'
    change: 'Append to the Reason line: «The home''s strict-modes check at each new entry point (`check_instance`, `check_home_tree`: viola''s stat and, on Windows, owner/DACL reads) is a call site whose refusal writes no line: the founder ruled no line for every new refused check, as for `link` / `unlink`, and rejected logging `strict-modes-failed` after one (`inputs#I3` Q2, chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary OS-APIs reason: the strict-modes check''s reads join the call sites, their refusal unlogged by the founder''s ruling (F13).'
    rationale: 'Report Decisions & corrections (the founder''s P4 answers, inputs#I3: a refused check at every new entry point writes no line, as link / unlink) and Reverted / negative API facts (logging strict-modes-failed after a refused check was rejected).'
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 133
    section: '## 1. Obs Scope Summary'
    change: 'Add a sub-bullet after the snapshot.json scrub rule: «  - A `snapshot.json` replayed from `events.ndjson` (`read_snapshot_or_replay`) carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` over it is `None` (unreachable), hook `connect` is `NotSent::Unreachable` (fail open, `channel-unreachable`), and statusline `serve` runs and prints nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary persistent-stores scrub rules: a replayed snapshot.json carries none of the four runtime fields, and its three new readers'' outcomes (F12).'
    rationale: 'Report Symbols: liveness.rs 92-94 (only Recovered::Snapshot proceeds; a replayed snapshot carries no endpoint, pid, started_at), hook.rs 224-226 (Replayed / Absent -> NotSent::Unreachable), statusline.rs 128-132 (no statusline_command, nothing runs or prints).'
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 158
    section: '## 1. Obs Scope Summary'
    change: 'Append to the cli Exporter line: «`send`, `wait`, `last`, `answer`, `pause` and `release` first run `refuse_untrusted` (`check_instance` on the home and `instances/<name>`): a refusal exits 21 with the instance pair, or under `--json` exactly the `instance-unreachable` document with `detail` null, opens nothing under the home and writes no line (chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary cli exporter: the six frame verbs log as cli only after refuse_untrusted passes; a refusal opens no file (F4).'
    rationale: 'Report Symbols: src/cmd/client.rs refuse_untrusted (20-41), called first by send, wait, last, answer, pause, release; opens nothing under the home and writes no line.'
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 158
    section: '## 1. Obs Scope Summary'
    change: 'Append to the cli Exporter line: «`viola verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log: a refusal exits 1 with the home pair, with no step line, no summary and no line; `update_stamps`'' `StampsError::StrictModes` maps to the same pair (chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary cli exporter: verify logs as cli only after its home-tree check passes; StampsError::StrictModes takes the same home pair (F9).'
    rationale: 'Report Symbols: src/cmd/verify.rs 64-70 (check_home_tree before its cli log; refuse_home 95-99, exit 1) and 288-296 (StampsError::StrictModes to the same pair); stamps.rs StampsError.'
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 164
    section: '## 1. Obs Scope Summary'
    change: 'Founder Direction 4 note now reads: «Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code, except a refused strict-modes check, which writes no line by the founder''s own answer (`inputs#I3` Q2 of chunk 2026-10-26-home-and-code-bearing-file-integrity, as `link` / `unlink`): the six frame verbs'' exit 21 and `verify`''s and `run` / `revive` start''s exit 1 carry their cause only as the fixed stderr pair (or the `--json` document), in no line.»'
    sidecar: 'obs-scope-summary cli notes: Founder Direction 4''s per-cause detail code carved out for the no-line strict-modes refusals, by the founder''s inputs#I3 Q2.'
    rationale: 'Report Decisions & corrections (inputs#I3: every new refused check writes no line) and Symbols (refuse_untrusted exit 21; run / revive refuse_tree and verify refuse_home exit 1 before any log): those exit-21 / exit-1 causes have no detail code in any line, contradicting the note as written.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 186
    section: '## 1. Obs Scope Summary'
    change: 'Append to the hook Service identity line: «The event and dialog paths take `canonical`, then `trusted` (`check_instance`), before `read_diagnostics_level` and obs init: a refused or absent instance exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append; the two `--capture` arms stay ahead of it, and `FAKE_AGENT_HOOK_PANIC` stays after obs init (chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary hook identity: a refused or absent instance is a silent exit 0 before obs init, beside the missing-VIOLA_NAME no-op (F5).'
    rationale: 'Report Symbols: src/cmd/hook.rs canonical (75-81) and trusted (83-86) taken before read_diagnostics_level / viola_obs_init; a refused or absent instance returns exit 0 at once with nothing read, opened or written.'
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 189
    section: '## 1. Obs Scope Summary'
    change: 'Append to the latency note: «The instance check (`canonical` + `trusted`) runs before logger init, inside the timed path; `run --perf` keeps the five hook rows under `SPINE_DEADLINE` with it (max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms, chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary hook notes: the pre-init instance check sits in the hook''s timed path, re-measured by run --perf.'
    rationale: 'Report Outcome (obs): run --perf keeps the five hook rows under SPINE_DEADLINE with the check in the timed path, max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms; the note bounds logger-init I/O and does not yet name the I/O the check adds ahead of it.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 190
    section: '## 1. Obs Scope Summary'
    change: 'Append to the no-instance note: «A `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary hook notes: a VIOLA_DIR naming no existing directory resolves no instance and writes nothing (F6).'
    rationale: 'Report Symbols (src/cmd/hook.rs: a VIOLA_DIR naming an instance directory that does not exist canonicalises to no session) and Deviation 5.'
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 232
    section: '## 1. Obs Scope Summary'
    change: 'Insert after init step 3: «Where the role checks the home before any file (the hook event and dialog paths: `canonical` + `trusted`; `send` · `wait` · `last` · `answer` · `pause` · `release` · `link` · `unlink`: `refuse_untrusted`; `verify`: `check_home_tree`; `run` and `revive`''s start: `checked_exe` + `check_home_tree`), run that strict-modes check: a refusal ends the process with no file opened and no line (the founder''s `inputs#I3` Q2, chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary init order: the strict-modes check between resolving the home and opening the process file, its refusal unlogged.'
    rationale: 'Report Symbols (hook.rs canonical/trusted before viola_obs_init; client.rs refuse_untrusted first in the six verbs and link_call; verify.rs 64-70 before its cli log; run.rs 166-172 and revive.rs 125-135 before open_wrapper_log) and Remaining-caller facts (viola_obs_init its 6, each now behind a check).'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 233
    section: '## 1. Obs Scope Summary'
    change: 'Append to init step 4: «— the open first runs `strict::check_diagnostics` on the existing `diagnostics/` folder and the file before `ensure_private_dir` narrows anything (a symlink refused; a folder the read rule; a file the read rule on Unix, the write rule on Windows), a refusal being exactly a failed open (`src/obs.rs`, chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'obs-scope-summary init order: step 4''s open is preceded by check_diagnostics on the folder and the file.'
    rationale: 'Report Symbols: src/obs.rs check_diagnostics (69-76) runs strict::check_diagnostics on the existing diagnostics/ folder and the role or detail file before ensure_private_dir, in open_role_file and write_detail; a refusal is an io::Error, exactly a failed open.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 237
    section: '## 1. Obs Scope Summary'
    change: 'The run sentence now reads: «For `run`, this completes before the start sequence, so the collision check and version gate are logged; only `run`''s home-tree check (`checked_exe` / `refuse_tree`) precedes it, and its refusal exits 1 before `open_wrapper_log` with no log, no line and no child.»'
    sidecar: 'obs-scope-summary init order: run''s home-tree check precedes the wrapper log and is the one unlogged start refusal.'
    rationale: 'Report Symbols: src/cmd/run.rs checked_exe (189-200) and refuse_tree (202-215), called by run (166-172) and revive''s start (revive.rs 125-135) before open_wrapper_log: exit 1, no log opened, no line, no child.'
note: 'Outside the picked set but the same carve-out: obs-plan.md:161 and :253 (Founder Direction 4) and :215 (Vector 7, strict-modes line only on ledger-stamps) are body lines of unpicked entities; F7 / F14 are error-capture-reporting''s, F8 / F1-F3 span-trace-coverage''s, F10 / F11 log-coverage''s.'
