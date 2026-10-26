proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 29
    section: '## 6. Log Coverage'
    change: '`server-verify-failed` is written by the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`; chunk 2026-10-26-explicit-session-links added `link` / `unlink`, whose role file is `cli-<driven>.ndjson`) on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, with no pid, start time, path or endpoint.'
    sidecar: 'log-coverage §6 log-details: the writers of `process-exit` `server-verify-failed` move from the six CLI frame verbs to eight, `viola link` / `viola unlink` added (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report (Changes › Symbols / APIs, step 5, and Counts › frameverbs / srvverify) states the CLI frame verbs are now eight, each writing `server-verify-failed` on a mismatch, and lists log-coverage.md:29 as stale under both patterns.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:94
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 35
    section: '## 6. Log Coverage'
    change: 'Append to the Vector 7 `strict-modes-failed` enumeration, after the `viola plugin install` clause: `viola link` / `viola unlink`, whose `check_instance` of the driven instance runs before `client::start`, exit 21 on a refused check and write no line either (no role file opened, no `process-start`, no `process-exit`, the directory mode as found), the human form the fixed instance-state `unable` / `hint:` pair, under `--json` the `instance-unreachable` document (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'log-coverage §6 log-details Vector 7: `viola link` / `viola unlink` join the verbs whose refused strict-modes check exits 21 with no line (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report (Changes › Symbols / APIs, link_call step 1) states both verbs run `check_instance` before any role file or log opens, refused → exit 21 with no line in any log and `--json` `{"v":1,"error":"instance-unreachable","detail":null}`; Counts › checkinst asks the obs detector to extend the refusing-verb enumerations, and this line closes with the claim that no arm writes the word on `process-exit` exit 21, which the two verbs keep true.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41
note: 'Outside this file but read in the body: obs-plan.md:392 (§4 E3 says the explicit link / unlink methods are not built, a Retired item) and obs-plan.md:311 (human::refuse census, link 1 missing) and obs-plan.md:418 (v_rejected -32602 causes) are stale per the report; they belong to the body / other categories detectors.'
