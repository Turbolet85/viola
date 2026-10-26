proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 14
    section: '## Occupied Resources'
    change: 'In the parenthetical list of processes that check the server before their first frame, the CLI frame verbs read `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink` (eight; `link` / `unlink` since chunk 2026-10-26-explicit-session-links, each through `cmd::client::connect`, and only those two also run `viola_state::strict::check_instance` on the driven instance first).'
    sidecar: 'occupied-resources res-ipc-endpoints: the server-check list of CLI frame verbs gains `link` / `unlink` (six to eight).'
    rationale: 'Report Counts / qualifiers moved, pattern `frameverbs`: CLI frame verbs six to eight, `architecture-detail/occupied-resources.md:14` named stale; Symbols / APIs: the CLI frame verbs are now eight, each taking the liveness pre-check and the server check through `cmd::client::connect` before its frame, only `link` / `unlink` running `check_instance` first.'
    basis: .andromeda/architecture-detail/occupied-resources.md:14
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 106
    section: '## Occupied Resources'
    change: 'The closing count reads: `check_instance` has these three product call sites (the rule: `check_instance(` under `src/`): `src/cmd/hook/statusline.rs`, `src/cmd/revive.rs` and `src/cmd/link.rs`, the one site `viola link` and `viola unlink` share (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'occupied-resources res-filesystem: `check_instance` product call sites two to three (`src/cmd/link.rs`, shared by `link` / `unlink`).'
    rationale: 'Report Counts / qualifiers moved, pattern `checkinst`: `check_instance(` product call sites 2 to 3, `architecture-detail/occupied-resources.md:106` "these two product call sites" named stale; Spec claims disproved: the master rule reads 2 at 8fc3d1df and moves to three with `src/cmd/link.rs`.'
    basis: .andromeda/architecture-detail/occupied-resources.md:106
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: '## Occupied Resources'
    change: '"Three readers check these files before they use them" reads "Four readers check these files before they use them" (`viola list`, `viola revive`, `viola hook statusline`, and `viola link` / `viola unlink` through their one shared check).'
    sidecar: 'occupied-resources res-filesystem: the instance-file readers that check before use gain `viola link` / `viola unlink` (three to four).'
    rationale: 'Report Symbols / APIs: `link_call` runs `viola_state::strict::check_instance(<absolute home>, <home>/instances/<driven>)` before `client::start`; the same `checkinst` fact as :106, a further occurrence of the moved count in the same record.'
    basis: .andromeda/architecture-detail/occupied-resources.md:105
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 97
    section: '## Occupied Resources'
    change: 'The `cli` verbs with producers today add `link` and `unlink` (chunk 2026-10-26-explicit-session-links): into `cli-<name>.ndjson`, `<name>` the driven instance (`cli_sink(&home, &args.driven)`), they write the same lines as `send` / `wait` / `last` / `answer` / `pause` / `release` (`process-start`, the client `channel-request` / `channel-response` pair holding no link name, `process-exit{subject:"self"}`, and exit 21 `detail:"server-verify-failed"` + `during:"connect"` with no `channel-request` line when the serving process is not the recorded wrapper); a refused instance check, made before `client::start`, opens no role file and writes no line.'
    sidecar: 'occupied-resources res-filesystem: the `cli-<name>` producers gain `link` / `unlink` (`<name>` the driven instance), their refused instance check writing no line.'
    rationale: 'Report Symbols / APIs: `link_call` steps 1-5 (check_instance before `client::start` with no line in any log; `client::start(home, &driven)` the `cli-<driven>` role file and `process-start`; the server-verify `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}`); Outcome: `link_verbs_log_the_channel_pair_and_no_names`; Expected amendments names occupied-resources for the `cli-<name>` producers.'
    basis: .andromeda/architecture-detail/occupied-resources.md:97
    dependent-of: D-arch-resources
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 106
    section: '## Occupied Resources'
    change: 'After the `viola hook statusline` sentence, add: `viola link` and `viola unlink` (chunk 2026-10-26-explicit-session-links) run `check_instance(<absolute home>, <home>/instances/<driven>)` before `client::start`, so before any role file or log opens and before any snapshot value is read, and only then the liveness pre-check and the server check; a refusal is exit 21 (as `list` and `plugin install`, not the exit 1 of `revive`), the fixed instance-state `unable:` / `hint:` pair on stderr through `human::refuse`, under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty, no line in any log and the directory mode left as found; the order is held by `link_verbs_refuse_a_widened_instance_before_any_log` (the founder''s own word at P4, that chunk''s `inputs#I3`, the boundary widening ratified with no dated gap).'
    sidecar: 'occupied-resources res-filesystem: the `link` / `unlink` instance check before any log, its exit-21 refusal and its founder ratification recorded.'
    rationale: 'Report Symbols / APIs step 1 of `link_call`; Decisions & corrections: the founder at P4 (`inputs#I3`) and the exit 21 (card lean 5); the order trap held by `link_verbs_refuse_a_widened_instance_before_any_log`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 29
    section: '## Occupied Resources'
    change: 'Add to the Subcommands record: `link` and `unlink` (chunk 2026-10-26-explicit-session-links) each take `<driver> <driven> [--json]` (one shared `LinkArgs`), the instance being the driven one; each exits 0 with its one result line, or `{"v":1,"ok":<ok>}` under `--json` (`link`: `{since, already_linked}`; `unlink`: `{}`), 21 when the instance check refuses, no live endpoint is recorded or the serving process is not the recorded wrapper, 20 for a wrapper error (`-32602` included, such as `driver` equal to `driven`), and 1 for a refused or wrongly shaped reply.'
    sidecar: 'occupied-resources res-binaries: the `link` / `unlink` arguments and exit codes recorded beside the other verbs.'
    rationale: 'Report Symbols / APIs: new verbs `viola link <driver> <driven> [--json]` / `viola unlink`, `LinkArgs` shared, the verb instance the driven one; `link_call` step 6 maps the reply to exits 0 / 20 / 21 / 1, steps 1, 3 and 5 to exit 21.'
note: 'Outside this file: body architecture.md:325 names only `viola list`, `viola revive` and `hook statusline` as checking instance files ahead of any log, and body architecture.md:100 (exit 21) names only `list` / `plugin install` strict-modes refusals; both omit `link` / `unlink` and belong to the body / conventions passes.'
