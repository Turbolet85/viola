proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 11
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: Exemption 1's pre-init window also holds the instance check `viola link` / `viola unlink` make of the driven instance (`viola_state::strict::check_instance` on the home and `instances/<driven>`, the one site both verbs share in `link_call`), which runs before `client::start` opens the `cli-<driven>` role file (chunk 2026-10-26-explicit-session-links); the clause is appended to the list after `viola plugin install`'s check of its home and `marketplace/` tree dirs.
    sidecar: §10 exemption 1 (pre-init window) gains the `viola link` / `viola unlink` instance check of the driven instance, run before the `cli-<driven>` role file opens (chunk 2026-10-26-explicit-session-links).
    rationale: The report's Changes › Symbols / APIs (new verbs, `link_call` step 1) puts `check_instance` BEFORE `client::start`, so before any role file or log opens; its Expected amendments name obs-plan §10 the pre-init window for the verbs' check, and its `checkinst` sweep leaves the obs refusing-verb enumerations to this detector to extend.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 17
    section: "## 10. SLO Invariants & Telemetry Budgets"
    change: Exemption 7 also names a refused `viola link` / `viola unlink` instance check of the driven instance through `viola_state::strict::check_instance` (chunk 2026-10-26-explicit-session-links, the founder's own word `inputs#I3`), which opens no role file and writes no line in any log, the directory's mode left as found; its outcome joins the parenthesis as link and unlink exit 21 with the fixed instance-state `unable` / `hint:` pair on stderr, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty; so a panic there writes none.
    sidecar: §10 exemption 7 gains the refused `viola link` / `viola unlink` instance check (exit 21, the instance-state pair or the `instance-unreachable` document, no role file, no line), chunk 2026-10-26-explicit-session-links.
    rationale: The report's Changes › Symbols / APIs (new verbs, `link_call` step 1) states a refused check is exit 21 with the instance-state pair or the `instance-unreachable` `--json` document and no line in any log; Decisions & corrections records the exit 21 (as `list` / `plugin install`) as the founder's option text; Expected amendments name obs-plan §10 exemption 7 for the verbs' refused check.
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:44
note: no other drift in this file - the spine perf row's server check is the hook's own and the new verbs add no timed path; nothing here states the retired hold, the `-32601` answer or a frame-verb count.
