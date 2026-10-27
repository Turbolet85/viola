proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 34
    section: '## Conventions'
    change: 'Exit `21` item: after the `viola link` / `viola unlink` clause add that `viola send`, `wait`, `last`, `answer`, `pause` and `release` refuse the same way (chunk 2026-10-26-home-and-code-bearing-file-integrity): `client::refuse_untrusted`, which `link` / `unlink` now share, runs `check_instance` on the absolute home and `instances/<name>` (the home, the instance directory and its `diagnostics/` under the read rule, `instances/` and `snapshot.json` under the write rule, `events.ndjson` the write rule on Unix and the read rule on Windows, each that exists) before any role file, log or connect; a refusal is exit 21, under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, else the instance-state `unable:` / `hint:` pair; nothing under the home opened, no frame, no line, the mode or ACL left as found (the founder''s own answers, P4 Q2 and Q3, `inputs#I3`).'
    sidecar: 'conv-exit-codes: exit 21 gains the six frame verbs'' refused instance check (`refuse_untrusted`, the instance pair, no line) beside link / unlink, with check_instance''s widened set.'
    rationale: 'Report Changes, Symbols (`refuse_untrusted` called first by send / wait / last / answer / pause / release; `check_instance` now reads the home, the instance dir and `diagnostics/` to the read rule) and Count 8 (the six frame verbs join the instance-pair list); Outcome criterion 1 MET.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 40
    section: '## Conventions'
    change: 'Replace «and for `link` / `unlink` the instance check of the driven instance (exit 21), come first, before any connect» with «and the instance check (`client::refuse_untrusted`, exit 21: the target instance for the six frame verbs, the driven instance for `link` / `unlink`), come first, before any connect».'
    sidecar: 'conv-exit-codes: the server-verify block''s order sentence names the instance check for all eight frame verbs, not only link / unlink.'
    rationale: 'Report Changes, Symbols: `refuse_untrusted` is called first by the six verbs and `link_call` moved onto it, so the line-40 statement that only link / unlink run the instance check before the connect no longer holds.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 23
    section: '## Conventions'
    change: 'Exit `1` item, a new continuation line after line 23 (under 2 000 chars): `viola run` and `revive`''s start arm refusing a home tree that fails `strict::check_home_tree` (the home under the read rule; with the name, `check_instance`''s set; `ledger/`, `bin/`, `plugin/`; with the key, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists), checked by `run::checked_exe` before `open_wrapper_log`: exit 1, the home-state pair for a home-level refusal and the instance-state pair for an instance-level one (`run::refuse_tree`), no log opened, no line, no child; `revive --json` keeps `strict-modes-failed` (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'conv-exit-codes: exit 1 gains run''s and revive''s start-arm home-tree refusal (`check_home_tree`, home or instance pair, before any log).'
    rationale: 'Report Changes, Symbols (`checked_exe` / `refuse_tree`, run.rs 166-172, revive.rs 125-135) and Count 9 (`run` prints the home pair, or the instance pair for an instance-level refusal, exit 1); Outcome criterion 4 MET.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 25
    section: '## Conventions'
    change: 'Exit `1` item, a new continuation line after line 25 (under 2 000 chars): `viola verify` refusing its home tree (`check_home_tree(home, None, None)`: the home, `ledger/`, `bin/`, `plugin/`) before its `cli` log, and `update_stamps`'' `StampsError::StrictModes` (checked before `ledger/` is created and again under the lock, nothing read or written): the home-state `unable:` / `hint:` pair, exit 1, no step line, no summary, no line, `stamps.json` unchanged; an unstattable home (a path under a regular file) is on Unix this refusal (and, for the frame verbs, exit 21 with the instance pair), no longer the `cli` catch site''s `error: internal error`, while on Windows such a path stats `NotFound` and the catch-site line stands (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'conv-exit-codes: exit 1 gains verify''s home-tree and stamps strict-modes refusal (home pair, no line) and the unstattable home''s narrowed catch-site reach.'
    rationale: 'Report Changes, Symbols (verify.rs 64-70, 95-99, 288-296; `StampsError`) and Counts 9, 13 (an unstattable home now a strict-modes refusal on Unix, the catch-site line on Windows) and 17; Outcome criterion 4 MET.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 53
    section: '## Conventions'
    change: 'Add: before `live_endpoint`, each frame tool runs `check_instance` on the home and the target''s instance directory (chunk 2026-10-26-home-and-code-bearing-file-integrity); a refusal returns the same `instance-unreachable` `isError` result, `detail: null`, text `refused: instance-unreachable`, with no frame written and no log line.'
    sidecar: 'conv-mcp-results: the MCP frame tools'' instance check before `live_endpoint`, refused as `instance-unreachable` with no frame and no line.'
    rationale: 'Report Changes, Symbols (`viola-mcp` tools.rs `frame` 337-339: `check_instance` before `live_endpoint`, `result::unreachable()`, no frame, no line); Outcome criterion 3 MET.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
note: 'Not in this file: the body''s Conventions line architecture.md:100 (exit `21` «a refused strict-modes check of `viola list`, `viola plugin install`, `viola link` or `viola unlink`») omits the six frame verbs — the body owner should extend it; also the instance-state pair''s wording («state files can be written by another user», line 23) now fronts read-rule refusals (`Refused::Readable`) too, a wording question the report does not settle; and Count 14 (`viola list`''s board-scan home-check inconsistency) is left to its P5 route owner, line 34 being true as written.'
