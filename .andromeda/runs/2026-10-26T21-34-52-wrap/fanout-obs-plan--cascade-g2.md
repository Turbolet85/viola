proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 292
    section: "## 1. Obs Scope Summary"
    change: "Append to the paragraph: the product also holds these files at open — `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symlink is refused, a folder is held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows; a refusal is a failed open, the writer ends at `std::io::sink` and the line is dropped (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#26.1: the 0600-in-0700 diagnostics rule is now also enforced at open by check_diagnostics (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: src/obs.rs new private check_diagnostics (69-76) runs strict::check_diagnostics in open_role_file and write_detail before ensure_private_dir, a refusal exactly a failed open ending at std::io::sink; strict.rs check_diagnostics (127-145) refuses a symlink, holds a folder to the read rule, a file to the read rule on Unix and the write rule on Windows. The entity names only the tests secret scan as the enforcement."
    basis: "src/obs.rs:69"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 299
    section: "## 1. Obs Scope Summary"
    change: "In the must-trace chain insert `state.snapshot_recover` under `run.start` before `run.collision_check` (the one snapshot read the collision check takes): `run.start` › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › … the rest of the chain and the log events unchanged."
    sidecar: "obs-scope-summary#27.2: state.snapshot_recover joins the run start's must-trace chain under run.start before run.collision_check (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Deviations 4: collision_check takes the caller's one read (Option<&Recovered>), so state.snapshot_recover now sits under run.start before run.collision_check and two span-order oracles list it; Symbols run.rs collision_check (425-432)."
    basis: "src/cmd/run.rs:425"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 300
    section: "## 1. Obs Scope Summary"
    change: "After «A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor).» add: A `run` or a revived start whose home-tree check (`check_home_tree`, through `checked_exe` and `refuse_tree`) refuses exits 1 with the home pair (home level) or the instance pair (instance level) and writes no line either: the check precedes `open_wrapper_log`, so that exit-1 cause has no `detail` code (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#27.3: run's and a revived start's refused home tree joins the no-line exit-1 causes (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: run.rs checked_exe (189-200) and refuse_tree (202-215); run (166-172) and revive's start (revive.rs:125-135) call them before open_wrapper_log: exit 1, no log opened, no line, no child. The entity's claim that exit-1 causes use a distinct detail code no longer covers this cause."
    basis: "src/cmd/run.rs:166"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 327
    section: "## 1. Obs Scope Summary"
    change: "Append: a refused instance check writes no line and so carries none of these fields — CLI `answer` (`refuse_untrusted`, before any frame) exits 21 with the instance pair, or the `instance-unreachable` document under `--json`; the dialog hook exits 0 at once with empty streams, no frame and no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#30.3: the dialog scenario's refused instance checks (CLI answer, the dialog hook) write no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: client.rs refuse_untrusted (20-41) called first by answer (94-96), exit 21, no line; hook.rs canonical / trusted before obs init on the event and dialog paths, a refused instance exit 0 with no line, no frame; Outcome (security) rows."
    basis: "src/cmd/client.rs:20"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 370
    section: "## 1. Obs Scope Summary"
    change: "Replace «`viola hook statusline` is the one arm that runs an instance check, and a refused check leaves no line» with: every hook arm but the two `--capture` arms runs an instance check before obs init — `viola hook statusline` (chunk 2026-10-10-budget-governor) and the event and dialog paths (chunk 2026-10-26-home-and-code-bearing-file-integrity) — and a refused check leaves no line."
    sidecar: "obs-scope-summary#35.1: the hook event and dialog paths now run the instance check too; statusline is no longer the one arm (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols src/cmd/hook.rs: canonical (75-81) and trusted (83-86) moved from statusline.rs and shared by both arms; the event and dialog paths take instance_of(…).and_then(canonical).filter(trusted) before read_diagnostics_level / viola_obs_init, a refused instance exit 0 with no line. The entity's «one arm» claim is contradicted."
    basis: "src/cmd/hook.rs:83"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 370
    section: "## 1. Obs Scope Summary"
    change: "Add to the fail-open bullet: the hook event and dialog paths take `canonical`, then `trusted`, before `read_diagnostics_level` and obs init; a refused or absent instance exits 0 at once — no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append; the two `--capture` arms stay ahead of the check, and `FAKE_AGENT_HOOK_PANIC` stays after obs init (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#35.1: the hook event and dialog paths' instance check, its order and its silent refusal (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols src/cmd/hook.rs (75-86): the check before read_diagnostics_level / viola_obs_init, a refused or absent instance exit 0 with nothing read or written; the capture arms unchanged ahead of it; FAKE_AGENT_HOOK_PANIC after viola_obs_init."
    basis: "src/cmd/hook.rs:75"
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 370
    section: "## 1. Obs Scope Summary"
    change: "Add beside missing `VIOLA_NAME`: a hook whose `VIOLA_DIR` names an instance directory that does not exist canonicalises to no session and writes nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#35.1: a VIOLA_DIR naming no existing directory leaves the hook silent (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols src/cmd/hook.rs: a VIOLA_DIR naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing; Deviations 5 (hook_fail_open's no_instance_dir row pins the silence)."
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 379
    section: "## 1. Obs Scope Summary"
    change: "Append: a frame tool (`send`, `wait`, `last`, `answer`) runs `check_instance` before `live_endpoint`, and a refused check returns `instance-unreachable` with `detail: null`, no frame and no log line — a rejected call that writes no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#36.1: an mcp frame tool's refused check_instance is a rejected call with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: viola-mcp tools.rs frame (337-339) runs check_instance(&ctx.home, &instance_dir) before live_endpoint, a refusal result::unreachable() with detail null, no frame, no line. The entity says rejected tool calls are logged."
    basis: "crates/viola-mcp/src/tools.rs:337"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 397
    section: "## 1. Obs Scope Summary"
    change: "Replace the sentences from «As landed a strict-modes failure has a line only on the `ledger-stamps` read» to the bullet's end with: As landed a strict-modes failure has a line in two places: the `ledger-stamps` read in `run`'s version gate, and on Windows x64 a refused `conpty/` companion (`PinError::StrictModes`), seen as `process-start.sideload_fallback` `strict-modes-failed` with the exit unchanged. Every other strict-modes refusal leaves no line: `viola revive`'s instance check; `run`'s and a revived start's home-tree check (`check_home_tree`, exit 1, before `open_wrapper_log`); `viola verify`'s home-tree check and `update_stamps`'s `StampsError::StrictModes` (exit 1); `viola hook statusline`'s and the hook event and dialog paths' instance check (exit 0); the instance check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`refuse_untrusted`, exit 21) and of `viola link` / `viola unlink` on the driven instance (made before any role file opens; exit 21); the `viola mcp` frame tools' check (`instance-unreachable`); the `diagnostics/` opens (`check_diagnostics`: a refused symlink, folder or file is a failed open whose writer ends at `std::io::sink`); the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor); and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list)."
    sidecar: "obs-scope-summary#38.1: a refused conpty/ companion now has a line; the no-line strict-modes refusals gain the chunk's new sites (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: pin_companions' check_present refusal is PinError::StrictModes, mapped by sideload_outcome to strict-modes-failed (run.rs:510), schemas/diag-line.v1.json line 70 gains the word; refuse_untrusted (client.rs 20-41), hook.rs canonical / trusted, tools.rs 337-339, verify.rs 64-70 and 288-296, run.rs 166-172 / revive.rs 125-135, obs.rs check_diagnostics (69-76) each refuse with no line. The entity's «a line only on the ledger-stamps read» and its five-site list are contradicted."
    basis: "src/cmd/run.rs:510"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 396
    section: "## 1. Obs Scope Summary"
    change: "Append to the bullet: and refused at open by the product (`check_diagnostics` in `open_role_file` and `write_detail`: a symlink refused, a folder held to the read rule, a file to the read rule on Unix and the write rule on Windows; a refusal is a failed open, the writer ends at `std::io::sink`) (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#38.1: the diagnostics 0600-in-0700 rule is also enforced at open (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols src/obs.rs check_diagnostics (69-76) and strict.rs check_diagnostics (127-145); the same fact as the 26.1 occurrence."
    basis: "src/obs.rs:69"
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 397
    section: "## 1. Obs Scope Summary"
    change: "Add: by the founder's own answer (`inputs#I3` Q2 of chunk 2026-10-26-home-and-code-bearing-file-integrity) every refused check that chunk added writes no line, as `viola link` / `viola unlink`'s does: logging `strict-modes-failed` after a refused check was rejected."
    sidecar: "obs-scope-summary#38.1: the founder's no-line ruling for every new refused strict-modes check (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Decisions & corrections: the founder's own answers at P4 (inputs#I3), a refused check at every new entry point writes no line, as link / unlink; Reverted / negative API facts: logging strict-modes-failed after a refused check was rejected by the founder (inputs#I3 Q2)."
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 440
    section: "## 1. Obs Scope Summary"
    change: "Add under the replay-recovery class: a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so `live_endpoint` over it is `None` (unreachable), hook `connect` is `NotSent::Unreachable` (fail open, `channel-unreachable`) and `viola hook statusline`'s `serve` runs and prints nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#43.1: what the four new replay readers do over a replayed snapshot (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: liveness.rs live_endpoint (92-94) proceeds only on Recovered::Snapshot; hook.rs connect (224-226) Replayed / Absent → NotSent::Unreachable; statusline.rs serve (128-132) runs nothing over a replay with no statusline_command."
    basis: "crates/viola-state/src/liveness.rs:92"
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 464
    section: "## 1. Obs Scope Summary"
    change: "Append: outside Section 10's bounded exemptions where no file exists, a process whose role file `check_diagnostics` refuses at open (a symlinked or widened `diagnostics/` folder or role file) has its writer at `std::io::sink`, so a panic in it writes no record (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#45.1: a refused diagnostics open leaves no panic record (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols src/obs.rs: check_diagnostics (69-76) runs before the role file opens in open_role_file, a refusal an io::Error exactly a failed open, the writer ending at std::io::sink; the entity's «each panic produces exactly one single-line record» for all four roles is stale for such a process."
    basis: "src/obs.rs:69"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 487
    section: "## 1. Obs Scope Summary"
    change: "Append: `run`'s and a revived start's refused home tree (`check_home_tree`: exit 1, the home pair at home level or the instance pair at instance level; revive `--json` keeps `strict-modes-failed`) carries no logged `detail` (no line): the check precedes `open_wrapper_log` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#48.1: the run / revive start's refused home tree is an exit-1 cause with no logged detail (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: run.rs checked_exe (189-200) / refuse_tree (202-215) called by run (166-172) and revive's start (revive.rs:125-135) before open_wrapper_log: exit 1, no log, no line, no child, revive --json keeps strict-modes-failed; the same fact as the 27.3 occurrence."
    basis: "src/cmd/run.rs:202"
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 487
    section: "## 1. Obs Scope Summary"
    change: "Add: `send`, `wait`, `last`, `answer`, `pause` and `release` each first run `refuse_untrusted` (`check_instance` on the home and `instances/<name>`); a strict-mode fail exits 21 with the instance pair, or under `--json` exactly the `instance-unreachable` document with `detail` null, opens nothing under the home and carries no logged `detail` (no line), as `viola link` / `viola unlink` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#48.1: the six frame verbs' strict-mode fail, exit 21 with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: client.rs refuse_untrusted (20-41) called first by send (190-192), wait (43-45), last (30-32), answer (94-96), pause (32-34), release (35-37); refused → refused_document(instance-unreachable) under --json, else the instance pair; Some(21); opens nothing, writes no line."
    basis: "src/cmd/client.rs:20"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 487
    section: "## 1. Obs Scope Summary"
    change: "Add: `viola verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log; a refusal exits 1 with the home pair, no step line, no summary and no line, and `update_stamps`'s `StampsError::StrictModes` maps to the same pair — an exit-1 cause with no logged `detail` (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    sidecar: "obs-scope-summary#48.1: verify's refused home tree and StampsError::StrictModes, exit 1 with no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Changes, Symbols: verify.rs check_home_tree(&home, None, None) (64-70) before its cli log, refuse_home (95-99) the home pair, exit 1, no step line, no summary, no line; measure's StampsError::StrictModes maps to the same pair (288-296)."
    basis: "src/cmd/verify.rs:64"
    fact: F9
note: "Not this group's: F1, F2, F3, F8 (span-trace-coverage), F7, F14 (error-capture-reporting), F10, F11 (log-coverage); the same stale claims stand in other sections' body lines — obs-plan.md:361 (§4, «strict-modes-failed cannot occur» on wait / last), obs-plan.md:637 (§10 zero-unlogged-panics, no exemption for a refused diagnostics open) and obs-plan.md:119 / :388 (hook silent only without VIOLA_NAME)."
