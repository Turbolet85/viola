proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 79
    section: '## Standard Contracts'
    change: 'New group `<!-- detail: values · contract-cli-json-output -->` (closed by `<!-- /detail -->`), its item: - `viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` (`since` a string, `already_linked` a boolean; exit 0) and `viola unlink <driver> <driven> --json` prints `{"v":1,"ok":{}}` (exit 0); their refused strict-modes check on the home and `instances/<driven>`, made before any role file or log opens, prints exactly the `instance-unreachable` document `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21) on stdout with stderr empty and no line in any log.'
    sidecar: 'standard-contracts: new CLI `--json` output group holding the `viola link` / `viola unlink` success documents and their strict-modes refusal document (chunk 2026-10-26-explicit-session-links, F8)'
    rationale: 'The report Changes, New verbs `viola link` / `viola unlink`: step 1 refuses a strict-modes check with `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty, no line in any log; step 6 prints `{"v":1,"ok":<ok>}` under `--json` for a `link` ok with a string `since` and a boolean `already_linked`, or an `unlink` ok object; the detail file holds no group for the body CLI `--json` output yet.'
    fact: F8
note: F11 has no home here — the `channel-response` process-log line and its `method` label belong to obs-plan (span-trace-coverage, the IPC invocation catalog); the report calls it a plan-only claim with no master site.
