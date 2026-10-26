proposals:
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 80
    section: '## Surface: cli › ### Component Patterns'
    change: 'The server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, the eight verbs that write a frame, as the hint cause `server-verify` beside `not-running`: it prints the server-verification line above byte for byte, plain ASCII with no SGR, and it holds no pid, start time, path or endpoint.'
    sidecar: 'surface.md cli-send: the server-verification hint verb list grows from six to eight (`link`, `unlink`), chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report states the CLI frame verbs are now eight, each taking the server check through `cmd::client::connect` and refusing exit 21 with the server-verify hint (Changes, Symbols / APIs step 5 and "The CLI frame verbs are now eight"); the expected amendments name design-system §Surface: cli the server-verify hint verb list.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:77
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 73
    section: '## Surface: cli › ### Component Patterns'
    change: 'In the closing sentences of the `--json` hint bullet, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document stays `detail:null`, as the not-running cause''s does (rest of the bullet unchanged).'
    sidecar: 'surface.md cli-send: the writers of `server-verify-failed` gain `link` and `unlink`, chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report counts this line stale under `srvverify` ("`design-system-detail/surface.md:73` lists the six writers of `server-verify-failed`"); the two verbs write `process-exit{… detail:"server-verify-failed"}` on a mismatch (Changes step 5).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:99
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 128
    section: '## Surface: cli › ### Component Patterns'
    change: 'Exit 21''s server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.'
    sidecar: 'surface.md cli-exit-codes: the server-verification cause verb list grows from six to eight, chunk 2026-10-26-explicit-session-links.'
    rationale: 'Same retired six-verb claim: the report states a server that is not the recorded wrapper gets no frame from `link` / `unlink`, exit 21 `instance-unreachable` with the server-verify hint (Changes step 5) and `--json` exactly the `instance-unreachable` document.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:53
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 21
    section: '## Surface: cli › ### Tokens (platform-specific)'
    change: 'The stderr bullet''s fixed-message-form list reads: only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, `viola list`''s exit-21 strict-modes refusal (its existing provenance kept), and the exit-21 strict-modes refusal of `viola link` / `viola unlink` (the fixed instance-state pair, chunk 2026-10-26-explicit-session-links, the founder''s own word `inputs#I3`) use the fixed message form, `unable: <text>` (rest of the bullet unchanged).'
    sidecar: 'surface.md cli-output: `viola link` / `viola unlink`''s exit-21 strict-modes refusal joins the fixed `unable:` form list, chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report states a refused `check_instance` in `link_call` exits 21 printing `human::instance_state_pair(driven)` through `human::refuse` (the fixed instance-state `unable` / `hint:` pair) and that this refusal is exit 21 as `list` / `plugin install`, not `revive`''s exit 1 (Changes step 1; Decisions & corrections).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:44
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 122
    section: '## Surface: cli › ### Component Patterns'
    change: 'The exit-21 row''s human cell reads: `unable  instance-unreachable` (…) then the cause''s hint; `viola list`''s refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (its provenance kept), and `viola link` / `viola unlink`''s refused instance check prints the fixed instance-state pair of the driven instance; the `--json` cell reads `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), `list`''s and `link` / `unlink`''s strict-modes refusals included, with stderr empty.'
    sidecar: 'surface.md cli-exit-codes: the exit-21 row names `link` / `unlink`''s strict-modes refusal (instance-state pair, `instance-unreachable` document), chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report states the refusal prints the fixed instance-state pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty (Changes step 1); the expected amendments name the exit-21 row.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:45
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 37
    section: '## Surface: cli › ### Component Patterns'
    change: 'The Refusal bullet''s instance pair reads: an instance → `unable: <name>''s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` (`revive`''s pair, shared, and `viola link` / `viola unlink`''s for the driven instance, chunk 2026-10-26-explicit-session-links).'
    sidecar: 'surface.md cli-list: the instance-state pair''s sharers gain `link` / `unlink`, chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report states the verbs refuse through `human::instance_state_pair(driven)`, the fixed instance-state pair (Changes step 1); the line names `revive` alone as its sharer.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:44
    dependent-of: D-design-cli-wording
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 136
    section: '## Surface: cli › ### Navigation Pattern'
    change: '`viola --help` groups verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install` (`revive` beside `run`: the operator''s answer at the chunk 2026-10-10-viola-revive wrap). As built (chunk 2026-10-17-cli-output-discipline; the `handoff:` row, chunk 2026-10-26-explicit-session-links) it lists the served verbs only, those five rows in that order, `handoff:  link, unlink` between `wheel:` and `setup:`, then `Options:`.'
    sidecar: 'surface.md cli-help: the pending `handoff:` row note retired; the row is built between `wheel:` and `setup:`, chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report retires the hold on the explicit verbs (Retired) and states `viola --help` gains the row `handoff:  link, unlink` between `wheel:` and `setup:` (Changes, Symbols / APIs); its owner-pattern `title` count names `design-system-detail/surface.md:136` stale.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:67
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 92
    section: '## Surface: cli › ### Component Patterns'
    change: 'New cli-verbs item after the `release --budget` item: `viola link <driver> <driven>` → `link    <driver> -> <driven>  since <HH:MM:SSZ>`, or `link    <driver> -> <driven>  already linked` for a held pair; `viola unlink <driver> <driven>` → `unlink  <driver> -> <driven>` (an absent pair prints the same). Each is one unstyled stdout write, plain in every mode, ASCII `->`; the time is the reply''s `since` as UTC clock time in whole seconds with `Z`, and a `since` that is no RFC 3339 instant (the restore''s `unknown`) prints escaped; the names printed are the verb''s own arguments, never read back from the reply or the log. Under `--json` the result is `{"v":1,"ok":<ok>}`, `<ok>` `{"since":…,"already_linked":<bool>}` for `link` and `{}` for `unlink` (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'surface.md cli-verbs: the `link` / `unlink` result lines and `--json` documents as built, chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report adds three result lines (`write_linked`, `write_already_linked`, `write_unlinked`, one `write_all` each, plain in every mode), the `since` rendering through `link_since` / `escape_message`, the names rule, and the `--json` `{"v":1,"ok":<ok>}` reply shapes (Changes, Symbols / APIs); the file holds no record of these output shapes.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:60
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 128
    section: '## Surface: cli › ### Component Patterns'
    change: 'New cli-exit-codes item after the server-verification cause: `viola link` / `viola unlink` refuse in a fixed order, the instance being the driven one: a driven instance failing strict-modes → exit 21 with the fixed instance-state `unable:` / `hint:` pair before any log opens (`--json` the `instance-unreachable` document, stderr empty); no live wrapper → exit 21 with the not-running hint last; a server that is not the recorded wrapper → exit 21 with the server-verification hint; a wrapper `-32602` (a bad name, a foreign `driven`, `driver` equal to `driven`) → exit 20 `error: wrapper fault`, the self-pair refused by the wrapper only, never client-side (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'surface.md cli-exit-codes: the `link` / `unlink` refusal order and exit codes as built, chunk 2026-10-26-explicit-session-links.'
    rationale: 'The report states `link_call`''s order (check_instance → client::start → live_endpoint → connect with server check → answer_of), exit 21 for the strict-modes, not-running and server-verify refusals, exit 20 through `client::fault` for a wrapper error, and the self-pair refused by the wrapper only (Changes steps 1-6; Decisions & corrections).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:40
note: The body carries the same drift outside this file (design-system.md:348 the pending `handoff:` row, :261 the fixed `unable:` form list, :303 / :338 / :340 no `link` / `unlink` strict-modes refusal); token detector D-design-tokens finds no drift (report Coverage `tokens n/a`, link output plain as surface.md:14 states).
