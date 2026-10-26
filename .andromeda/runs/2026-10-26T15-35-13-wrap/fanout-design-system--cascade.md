proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 21
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: 'In the clause naming which refusals use the fixed message form, after `viola list`''s exit-21 strict-modes refusal (its provenance kept) add `, and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance (the fixed instance-state pair; the founder''s own word, chunk 2026-10-26-explicit-session-links)`; the rest of the line unchanged.'
    sidecar: 'surface.md cli-output stderr: the fixed `unable:` form list gains the `link` / `unlink` exit-21 strict-modes refusal (chunk 2026-10-26-explicit-session-links).'
    rationale: 'report Changes, new verbs step 1 (report.md:41-46): a refused `check_instance` on the driven instance is exit 21 printing `human::instance_state_pair(driven)` through `human::refuse`, the fixed instance-state `unable` / `hint:` pair.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 261
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: 'stderr line: `- stderr: context lines, refusals (two-space `unable  <name>  <reason>  <detail>`, or the fixed `unable: <text>` form for the exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link` and `unlink`), each refusal''s `hint:` line directly after it, and errors.`'
    sidecar: 'design-system Streams: the fixed `unable:` form names the `link` / `unlink` exit-21 strict-modes refusal.'
    rationale: 'the body line the cli-output group details; same fact as surface.md:21 (report.md:41-46).'
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 37
    section: '## Surface: cli › ### Component Patterns'
    change: 'In the instance case, `(`revive`''s pair, shared)` becomes `(`revive`''s pair, shared; `viola link` / `viola unlink` print it, naming the driven instance, at their exit-21 strict-modes refusal)`; the rest unchanged.'
    sidecar: 'surface.md cli-list Refusal: the instance-state pair is also printed by `link` / `unlink`.'
    rationale: 'report Changes, new verbs step 1 (report.md:41-46): the verbs print `human::instance_state_pair(driven)`, the same fixed instance-state pair.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 73
    section: '## Surface: cli › ### Component Patterns'
    change: 'The sentence naming the writers reads `Of them, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document stays `detail:null`, as the not-running cause''s does.`; the rest of the line unchanged.'
    sidecar: 'surface.md cli-send: the writers of `server-verify-failed` are eight frame verbs, `link` and `unlink` added.'
    rationale: 'report Counts, `srvverify` stale (report.md:99-100) and The CLI frame verbs are now eight (report.md:77-79).'
    basis: .andromeda/design-system-detail/surface.md:73
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 80
    section: '## Surface: cli › ### Component Patterns'
    change: 'The server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, the eight verbs that write a frame, as the hint cause `server-verify` beside `not-running`; the rest unchanged.'
    sidecar: 'surface.md cli-send: the server-verify hint is built for eight frame verbs.'
    rationale: 'report Changes, new verbs step 5 (report.md:51-55: a server that is not the recorded wrapper gets exit 21 with the server-verify hint) and the frame verbs now eight (report.md:77-79).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 92
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add after the `release --budget` item: `- `viola link` / `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR in every mode, exit 0; the names printed are the verb''s own parsed arguments, never names read back from the reply or the log (chunk 2026-10-26-explicit-session-links).`'
    sidecar: 'surface.md cli-verbs: the built `link` / `unlink` result line, one write, plain, names from the arguments.'
    rationale: 'report Changes, Three result lines (report.md:60-66): each one `write_all`, plain in every mode; names are the verb''s own parsed `ViolaName` arguments.'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 92
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add after the `release --budget` item: `- The `since` of the `link` line is the reply''s time as UTC clock time in whole seconds with `Z` (`<HH:MM:SSZ>`); a `since` that is no RFC 3339 instant (the restore''s `unknown`) prints through the message-mode escaper of pattern 3.`'
    sidecar: 'surface.md cli-verbs: the `link` line''s `since` form and its escaped fallback.'
    rationale: 'report Changes, Three result lines (report.md:62-65): `link_since` prints UTC whole seconds with `Z`; a non-RFC 3339 `since` prints through `escape_message`.'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 92
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add after the `release --budget` item: `- `viola unlink` of a pair that is not held prints the same `unlink  <driver> -> <driven>` line and exits 0; a second `viola link` of a held pair prints the `already linked` form, the first `since` kept.`'
    sidecar: 'surface.md cli-verbs: an absent-pair unlink and a repeated link print their normal lines, exit 0.'
    rationale: 'report Changes, channel methods (report.md:24-27): a second `link` answers the same `since` with `already_linked: true`; `unlink` answers `{"ok":{}}` whether or not a pair was removed.'
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 315
    section: '## Surface: cli › ### Component Patterns'
    change: 'Lead of the message-mode item reads `- Message mode, the escaper `wait` and `last` share and `viola link` uses for a `since` that is no RFC 3339 instant:`; the rule after the colon unchanged.'
    sidecar: 'design-system Component Patterns 3: message mode''s users gain `link`''s non-instant `since`.'
    rationale: 'report Changes, Three result lines (report.md:63-65): a `since` that is no RFC 3339 instant prints through `escape_message`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 113
    section: '## Surface: cli › ### Component Patterns'
    change: 'Exit-0 row, `--json` cell: `{"v":1,"ok":{…}}`; `viola link`''s `ok` is `{"since":<ts>,"already_linked":<bool>}` and `viola unlink`''s is `{}`.'
    sidecar: 'surface.md cli-exit-codes: the exit-0 `--json` cell names the `link` / `unlink` `ok` documents.'
    rationale: 'report Changes, channel methods and step 6 (report.md:24-27, :56-57): the `ok` is printed as `{"v":1,"ok":<ok>}` under `--json`.'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 121
    section: '## Surface: cli › ### Component Patterns'
    change: 'Exit-20 row, human cell: `error: wrapper fault  <code>`, also printed by `viola link` / `viola unlink` when the wrapper refuses the pair (`-32602`: driver equal to driven, a bad or foreign name), a pair the CLI never refuses client-side; the `--json` cell unchanged.'
    sidecar: 'surface.md cli-exit-codes: exit 20 is reached by `link` / `unlink` on a pair the wrapper refuses.'
    rationale: 'report Changes, step 6 (report.md:58-59) and Decisions (report.md:276-277): a wrapper error, `-32602` included, is `client::fault`, exit 20; never refused client-side.'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 122
    section: '## Surface: cli › ### Component Patterns'
    change: 'Exit-21 row: after `viola list`''s refused strict-modes check add `; `viola link` / `viola unlink`''s refused strict-modes check of the driven instance (the verb''s instance) prints the fixed instance-state `unable:` / `hint:` pair naming the driven instance (exit 21, as `list` and `plugin install`, not `revive`''s exit 1; the founder''s own word, chunk 2026-10-26-explicit-session-links)`; the `--json` cell''s `list`''s refusal included becomes `the `list`, `link` and `unlink` refusals included`, with stderr empty.'
    sidecar: 'surface.md cli-exit-codes: exit 21 names the `link` / `unlink` strict-modes refusal of the driven instance.'
    rationale: 'report Changes, step 1 (report.md:41-46) and Decisions (report.md:271-272): refused `check_instance` on the driven instance is exit 21, the instance-state pair, `--json` `instance-unreachable` with stderr empty.'
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 340
    section: '## Surface: cli › ### Component Patterns'
    change: 'Add after the `viola plugin install` sentence: `viola link` / `viola unlink`''s refused strict-modes check of the driven instance (exit 21) prints the instance-state `unable:` / `hint:` pair, or under `--json` the `instance-unreachable` document, stderr empty.'
    sidecar: 'design-system Exit-code phraseology: the `link` / `unlink` exit-21 strict-modes refusal.'
    rationale: 'the body line the cli-exit-codes group details; same fact as surface.md exit-21 row (report.md:41-46, :271-272).'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 128
    section: '## Surface: cli › ### Component Patterns'
    change: 'Exit 21''s server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`; the rest of the line unchanged.'
    sidecar: 'surface.md cli-exit-codes: the server-verification cause covers eight frame verbs.'
    rationale: 'report Changes, step 5 and The CLI frame verbs are now eight (report.md:51-55, :77-79).'
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 136
    section: '## Surface: cli › ### Navigation Pattern'
    change: 'The As-built sentence reads `As built (chunk 2026-10-17-cli-output-discipline; the `handoff:` row chunk 2026-10-26-explicit-session-links) it lists the served verbs only: `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff:  link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.`; the pending clause naming "Explicit session links" is removed; the planned grouping and its provenance before it unchanged.'
    sidecar: 'surface.md cli-help: the `handoff:  link, unlink` row is built between `wheel:` and `setup:`; its pending owner retired.'
    rationale: 'report Changes, `viola --help` (report.md:67) and Counts, owner pattern `title` stale (report.md:133-137); Retired (report.md:86-89).'
    basis: .andromeda/design-system-detail/surface.md:136
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system.md
    near: 348
    section: '## Surface: cli › ### Navigation Pattern'
    change: '`- `viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff:  link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.` (the pending clause and its owner removed)'
    sidecar: 'design-system Navigation Pattern: the `handoff:` row built; pending owner retired.'
    rationale: 'report Changes, `viola --help` (report.md:67); owner pattern `title` stale at design-system.md:348 (report.md:133-137).'
    basis: .andromeda/design-system.md:348
    dependent-of: D-design-cli-wording
note: 'the fixed `unable:` form lists at surface.md:21 and design-system.md:261 also omit `viola plugin install`''s exit-21 home-state pair (a standing gap, not this chunk''s); the unpicked cli-verbs `link` / `unlink` sample entities should be read as built, not design-only.'
