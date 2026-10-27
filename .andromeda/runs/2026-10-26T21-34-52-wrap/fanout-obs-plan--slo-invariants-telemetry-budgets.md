proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 11
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "Exemption 1 (the pre-init window) gains, after «… which runs before their `cli` role file opens»: «, and the strict-modes checks chunk 2026-10-26-home-and-code-bearing-file-integrity put ahead of every role file: the instance check (`client::refuse_untrusted`, `check_instance` on the home and `instances/<name>`) of `viola send` · `wait` · `last` · `answer` · `pause` · `release`, the instance check of `viola hook`'s event and dialog paths (`canonical` / `trusted`, before `read_diagnostics_level` and obs init; the two `--capture` arms stay ahead of it), and the home-tree check (`check_home_tree`) of `viola run` and `viola revive`'s start arm (through `checked_exe`, before `open_wrapper_log`) and of `viola verify` (before its `cli` log)»."
    sidecar: "slo-gates exemption 1: the pre-init window names the six frame verbs' instance check, the hook event/dialog instance check and the run / revive-start / verify home-tree check (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "The report's Symbols put each new check before its role file opens (client.rs refuse_untrusted; hook.rs canonical/trusted before viola_obs_init; run.rs checked_exe/refuse_tree before open_wrapper_log; verify.rs check_home_tree before its cli log); Count 20 names this file's exemption list (slo-invariants-telemetry-budgets.md:17) among the no-line rows to re-read for the new sites."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 13
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "Exemption 3 reads: «`hook` without `VIOLA_NAME` (D-16), or with a `VIOLA_DIR` naming an instance directory that does not exist, which canonicalises to no session, so the hook opens no file and writes nothing (chunk 2026-10-26-home-and-code-bearing-file-integrity)»."
    sidecar: "slo-gates exemption 3: a hook whose VIOLA_DIR names no existing instance directory writes nothing, as D-16 (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols (src/cmd/hook.rs): «A `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so the hook writes nothing»; Count 23 (pattern viola-dir-none, a fact no master states); Deviation 5."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 14
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "Exemption 4 gains, after «a failed step-4 role-file open (§3 init step 4).»: «A refused `viola_state::strict::check_diagnostics` (run by `obs::check_diagnostics` in `open_role_file` and `write_detail` on the existing `diagnostics/` folder and the role or detail file before anything is narrowed: a symlink, a path another user may write, or on Unix may read) is exactly such a failed open (chunk 2026-10-26-home-and-code-bearing-file-integrity).»"
    sidecar: "slo-gates exemption 4: a refused check_diagnostics on the role file or its folder is a failed step-4 open (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols (src/obs.rs): check_diagnostics runs before ensure_private_dir in open_role_file and write_detail; «a refusal is an `io::Error`, exactly a failed open (the writer ends at `std::io::sink`, a detail line dropped)»."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 17
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "A new item follows exemption 7: «8. a refused strict-modes check at an entry point chunk 2026-10-26-home-and-code-bearing-file-integrity added: the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` (exit 21, the fixed instance-state pair or under `--json` the `instance-unreachable` document), `viola hook`'s event and dialog paths (exit 0, nothing printed, no stdin read, no frame, no SessionEnd append), `viola run` and `viola revive`'s start arm on a refused home tree (exit 1, the home pair, or the instance pair for an instance-level refusal; revive `--json` keeps `strict-modes-failed`) and `viola verify` (exit 1, the home pair, no step line, no summary): it opens no role file and writes no line, so a panic there writes none, by §7's rule; no forced-panic case reads it. That a refused check there writes no line, as on `link` / `unlink`, is the founder's own answer (P4 Q2, `inputs#I3` of that chunk).»"
    sidecar: "slo-gates exemption 8 added: the refused checks of the six frame verbs, the hook event/dialog paths, run / revive start and verify write no line (chunk 2026-10-26-home-and-code-bearing-file-integrity, the founder's P4 Q2)."
    rationale: "Report Symbols lines 15-19 and Decisions (the founder's P4 answer: «a refused check at every new entry point writes no line, as `link` / `unlink`»); Count 20 names slo-invariants-telemetry-budgets.md:17 for re-reading; exemption 7 (line 17) is near the 2 000-char cap, so the new sites take their own item."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:111
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 28
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "The spine-hooks row's budget cell gains, after «… bounded by the same gate»: «; since chunk 2026-10-26-home-and-code-bearing-file-integrity the timed path also holds the hook's instance strict-modes check (`trusted`, before obs init), and its one `snapshot.json` read is `read_snapshot_or_replay` (a replayed or absent snapshot fails open as `channel-unreachable`), inside the same gate (that chunk's `run --perf`: max 2.4–3.3 ms over the five rows)»."
    sidecar: "slo-budgets spine row: the hook's instance check and the replay-aware snapshot read sit in the timed path, gated by the same max (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Outcome: «`run --perf` keeps the five hook rows under `SPINE_DEADLINE` with the check in the timed path — MET (max 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms)»; Symbols (hook.rs): the check before viola_obs_init and connect on read_snapshot_or_replay."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:137
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 30
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: "The obs-overhead row gains, after «… the gate passes vacuously»: «; it passes vacuously too when `VIOLA_DIR` names no existing instance directory or the instance fails the strict-modes check (the hook then opens no file and makes no channel call), so the perf instance is a viola-made one (chunk 2026-10-26-home-and-code-bearing-file-integrity); logger init's one `open` now follows the instance check's and `check_diagnostics`' metadata reads»."
    sidecar: "slo-budgets obs-overhead row: a missing or refused instance directory is a second vacuous pass; the perf instance is viola-made (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "Report Symbols (hook.rs: a refused or absent instance returns exit 0 at once, no config read, no line, no frame; a non-existent VIOLA_DIR writes nothing) and Deviation 5 (cases expecting lines run in a viola-made instance dir, `Env::Made`); the row's vacuous-pass clause names only the D-16 no-op."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:98
note: "D-obs-pii and D-obs-stack find no drift here (no logging or dependency added); no passage of this file states a retired dated gap as standing (line 28's «macOS the recorded gap» is the server-check gap, not retired); viola mcp's refused frame-tool check is post-init (its mcp role file open), so it is no panic exemption and has no row here."
