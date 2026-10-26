proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 40
    section: '## Conventions'
    change: 'Exit `21` also covers, on the eight CLI frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (`link` / `unlink` since chunk 2026-10-26-explicit-session-links), a server that is not the recorded wrapper (the connected peer''s pid or OS start time is not the snapshot''s, or the check cannot be made): no frame is written, and the verb prints the same `{"v":1,"error":"instance-unreachable","detail":null}` under `--json` with stderr empty, or the `unable  <name>  instance-unreachable` line with the server-verify hint last on stderr (its wording: design-system, surface cli-send). The client''s own refusals (exit 13) come first, and on `link` / `unlink` the driven instance''s strict-modes check and the liveness pre-check come first; the cause has no exit code, `--json` code or `RefusalReason` of its own.'
    sidecar: 'conv-exit-codes: the exit-21 server-mismatch verb list gains `link` and `unlink` (six → eight CLI frame verbs), and the order clause names their instance check and liveness pre-check ahead of the server check (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s Symbols / APIs make `link` / `unlink` CLI frame verbs that take the liveness pre-check and the server check through `cmd::client::connect`, a mismatch exit 21 `instance-unreachable` with the server-verify hint (report.md:51-55, :77-79); its Expected amendments name §Conventions CLI exit codes for the two verbs'' server mismatch and note no pattern fired for this row (report.md:186-187, :194-195). Line 40 still lists six verbs.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:77
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 34
    section: '## Conventions'
    change: 'Append to the `21` item: `viola link` / `viola unlink`''s refused strict-modes check of the driven instance (`viola_state::strict::check_instance` on the absolute home and `instances/<driven>`, made before `client::start`, so before any role file or log opens and before any snapshot value is read) is the same exit and document: human mode the fixed instance-state `unable:` / `hint:` pair (`human::instance_state_pair`), `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, no line in any log, the directory''s mode as found — exit 21 as `list` / `plugin install`, not `revive`''s exit 1 (the founder''s own word at P4, `inputs#I3`, card lean 5; chunk 2026-10-26-explicit-session-links).'
    sidecar: 'conv-exit-codes: exit 21 gains `viola link` / `viola unlink`''s refused instance strict-modes check (before any log; instance-state pair or the instance-unreachable document), the founder''s word inputs#I3 (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report''s step 1 of `link_call` runs `check_instance` before `client::start`, refused → exit 21 with the instance-state pair or the `instance-unreachable` document, no log line, mode as found (report.md:41-46); Decisions & corrections rule it exit 21 as `list` / `plugin install`, not revive''s exit 1 (report.md:265-267, :271-272); Expected amendments name §Conventions CLI exit codes for the strict-modes refusal (report.md:186-187). Line 34 names only `list` and `plugin install`.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 26
    section: '## Conventions'
    change: 'The `cli` role''s verb list reads `so `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `link`, `unlink`, `verify`, `revive --list` and `plugin install``, and the item adds: for `viola link` / `viola unlink` exit 1 also covers a `Refused` reply and an `ok` of another shape than the verb''s own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object), each an internal error (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'conv-exit-codes: the `cli` role''s verb enumeration gains `link` and `unlink`, and exit 1 names their Refused / wrong-shape reply as internal error (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report adds the verbs `viola link` / `viola unlink`, read by `role_of` as `Role::Cli`, whose `Refused` reply or `ok` of another shape is an internal error, exit 1 (report.md:38, :56-59, :67-70); line 26 enumerates the `cli` verbs without them.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:59
    dependent-of: D-arch-resources
note: 'Outside this category file: the body''s Conventions broad line (.andromeda/architecture.md:100, `21` … a refused strict-modes check of `viola list` or `viola plugin install`) needs `viola link` / `viola unlink` added to match the line-34 proposal; no D-retired hit here (the file states neither the link-verbs hold nor their `-32601`), and D-arch-decisions found no drift (no dependency, the instance-check form the founder''s word).'
