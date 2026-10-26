proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 149
    section: '## Established Decisions'
    change: 'Replace "As landed (chunk 2026-10-10-session-links) only the implicit link is built." with "As landed, chunk 2026-10-10-session-links built the implicit link and chunk 2026-10-26-explicit-session-links the explicit one: the driven instance''s wrapper serves the `link` and `unlink` methods (`LinkSet::link` / `LinkSet::unlink`, sharing `driver_of`), sent by the verbs `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`. The implicit link:" and keep the rest of the line as written.'
    sidecar: 'established-decisions [Links]: the explicit link is built beside the implicit one (chunk 2026-10-26-explicit-session-links); the "only the implicit link is built" qualifier retired.'
    rationale: 'Report Symbols / APIs: channel methods `link` and `unlink` are served (before: `-32601`) and the two verbs exist; Retired names every master line stating the hold.'
    basis: .andromeda/architecture-detail/established-decisions.md:149
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 149
    section: '## Established Decisions'
    change: 'Replace "an explicit `viola link` of an existing pair adds none and returns `{}`" with "an explicit `viola link` of an existing pair adds none and answers that pair''s `since` with `already_linked: true` (a new pair answers its new `link` line''s `ts` with `already_linked: false`)".'
    sidecar: 'established-decisions [Links]: an explicit link of a held pair answers `{since, already_linked: true}`, not `{}`.'
    rationale: 'Report Symbols / APIs: `link` answers `{"ok":{"since":<the link line''s ts>,"already_linked":<bool>}}`, a second `link` of a held pair appending nothing and answering the same `since` with `already_linked: true`.'
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 150
    section: '## Established Decisions'
    change: 'Replace the line whole with: "The explicit `viola link` / `viola unlink` and their two channel methods are built (chunk 2026-10-26-explicit-session-links) in the form the founder named at P4, his own word relayed verbatim (`inputs#I3`), which ratifies the boundary widening they carry with no dated gap: the wrapper serves `link` and `unlink` (no longer `-32601`), and `LinkSet::remove` has its first product caller, the `unlink` method, so a product path appends an `unlink` event line (`source: wrapper`, `{driver, driven}`); its dead-code allow is gone. The hold, and the working-route entry «Explicit session links» as its pending owner, are retired."'
    sidecar: 'established-decisions [Links]: the explicit-verb hold retired: verbs and methods built in the founder''s form (inputs#I3), `LinkSet::remove` called by the `unlink` method.'
    rationale: 'Report Retired: the hold itself, the `-32601` answer for `link` / `unlink` and `LinkSet::remove`''s `dead_code` allow; Counts: `remove` and the `title` owner pattern both read established-decisions.md:150 stale; Decisions: the founder''s word at P4 ratified the widening, no dated gap taken.'
    basis: .andromeda/architecture-detail/established-decisions.md:150
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 150
    section: '## Established Decisions'
    change: 'Add after the [Links] lines: "- The explicit verbs run `viola_state::strict::check_instance` on the home and `instances/<driven>` before `client::start`, so ahead of any role file, log or snapshot read, because opening a log narrows an existing home to 0700 and a check made after it passes a home at 0770; a refusal is exit 21. Then come the liveness-only pre-check (`live_endpoint`) and the server identity check through `cmd::client::connect`, then one request: the founder''s form (`inputs#I3`), a boundary widening with no dated gap."'
    sidecar: 'established-decisions [Links]: the explicit verbs'' order: instance check before any log, then liveness, then the server check (F6).'
    rationale: 'Report Symbols / APIs steps 1 to 5 of `link_call`; Decisions & corrections: the order trap held by `link_verbs_refuse_a_widened_instance_before_any_log`.'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 150
    section: '## Established Decisions'
    change: 'Add after the [Links] lines: "- The wrapper opens no span and writes no process-log line of its own for `link` / `unlink`: its existing `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name. The CLI side runs inside the `cli.link` / `cli.unlink` dispatch span."'
    sidecar: 'established-decisions [Links]: the link methods'' observability: the channel pair only on the wrapper, the `cli.link` / `cli.unlink` span on the CLI (F9).'
    rationale: 'Report Symbols / APIs: neither method opens a span nor writes a process-log line; the verbs run under `#[instrument(skip_all, name = "cli.link")]` / `name = "cli.unlink"`; Outcome (obs): the channel pair holds no names.'
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 150
    section: '## Established Decisions'
    change: 'Add after the [Links] lines: "- `LinkSet::line` stamps the `link` line''s `ts`, the pair''s `since`, with `Utc::now()` directly, outside the `Clock` seam (`src/run/links.rs:92`); root cases assert `since` by its shape only."'
    sidecar: 'established-decisions [Links]: the link line''s `since` is stamped outside the `Clock` seam (F10).'
    rationale: 'Report Decisions & corrections: `LinkSet::line` stamps `since` with `Utc::now()` directly (`src/run/links.rs:92`), outside the clock seam; root cases assert `since` by shape only (`is_clock_time`).'
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 51
    section: '## Established Decisions'
    change: 'Replace "only the implicit link is built, the explicit verbs held." with "the implicit link and, since chunk 2026-10-26-explicit-session-links, the explicit verbs and the served `link` / `unlink` methods are built, the verbs checking the driven instance''s files before any log (the founder''s form)."'
    sidecar: 'architecture body [Links]: the broad line no longer holds the explicit verbs.'
    rationale: 'Report Counts / Expected amendments: `linkverbs` reads `architecture.md:51` stale; Retired: every master line stating the hold.'
    basis: .andromeda/architecture.md:51
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 26
    section: '## Conventions'
    change: 'In the `cli`-role enumeration read "so `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `link`, `unlink`, `verify`, `revive --list` and `plugin install`", and add after the `wait` / `last` exit-1 clause: "for `viola link` / `viola unlink` exit 1 also covers a `Refused` reply and an `ok` of another shape than the verb''s (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object)".'
    sidecar: 'conventions CLI exit codes: `link` / `unlink` join the cli-role verbs; their unexpected reply shape is exit 1.'
    rationale: 'Report Symbols / APIs step 6: a `Refused` or an `ok` of another shape is an internal error, exit 1; `role_of` already reads both words as `Role::Cli`, and the verbs are now built.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 33
    section: '## Conventions'
    change: 'Extend the `20` item: "`20` protocol fault from the wrapper, a `-32602` from `link` / `unlink` included (a missing, non-string or invalid name, a `driven` that is not the serving wrapper''s own name, a `driver` equal to `driven`); the self-pair is refused only by the wrapper, never client-side".'
    sidecar: 'conventions CLI exit codes: exit 20 carries the link methods'' `-32602`; the self-pair is wrapper-refused only (F5).'
    rationale: 'Report Symbols / APIs step 6: a wrapper error (`-32602` included, e.g. `driver` equal to `driven`) is `client::fault`, exit 20; Decisions & corrections: the self-pair is refused by the wrapper only, never client-side.'
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 34
    section: '## Conventions'
    change: 'Append to the `21` item: "; `viola link` / `viola unlink`''s refused strict-modes check of the home or `instances/<driven>` is the same exit and document (human mode the fixed instance-state `unable:` / `hint:` pair; no log opened, the mode left as found): exit 21 as for `list` and `plugin install`, not `revive`''s exit 1 (a reply of an unexpected shape is exit 1, the `1` item)".'
    sidecar: 'conventions CLI exit codes: exit 21 covers the link verbs'' strict-modes refusal, not revive''s exit 1 (F7).'
    rationale: 'Report Symbols / APIs step 1: refused, exit 21, human `human::instance_state_pair(driven)`, `--json` the `instance-unreachable` document; Decisions & corrections: exit 21 as `list` / `plugin install`, not `revive`''s exit 1 (card lean 5).'
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 40
    section: '## Conventions'
    change: 'Read "on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`", and after "The client''s own refusals (exit 13) come first" add "; for `link` / `unlink` the strict-modes check of the driven instance (exit 21) comes before any connect".'
    sidecar: 'conventions CLI exit codes: the server-mismatch exit 21 covers eight verbs.'
    rationale: 'Report Symbols / APIs: the CLI frame verbs are now eight, each taking the liveness pre-check and the server check through `cmd::client::connect` before its frame; only `link` / `unlink` also run `check_instance` first.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 100
    section: '## Conventions'
    change: 'Read the `21` entry "`21` instance unreachable, and a refused strict-modes check of `viola list`, `viola plugin install`, `viola link` or `viola unlink`."'
    sidecar: 'architecture body CLI exit codes: exit 21 names the link verbs'' strict-modes refusal.'
    rationale: 'Report Symbols / APIs step 1: a refused `check_instance` is exit 21; Expected amendments: §Conventions CLI exit codes, exit 21 for the two verbs'' strict-modes refusal.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 41
    section: '## Standard Contracts'
    change: 'Append: "Since chunk 2026-10-26-explicit-session-links the set also changes on an explicit `link` / `unlink` method call (`LinkSet::link` through `record`, `LinkSet::unlink` through `remove`), and each method answers only after its line and the snapshot write; the set still follows the log."'
    sidecar: 'standard-contracts instance snapshot `links`: the explicit methods also change the set.'
    rationale: 'Report Symbols / APIs: `link` calls `record(driver)`, `unlink` calls `remove(driver)`, each answering after its line and the snapshot write.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 41
    section: '## Standard Contracts'
    change: 'Add after the `links` entry: "- A `link` of a pair the set already holds appends nothing and answers the held `since` with `already_linked: true`; a new pair answers `since`, the new `link` line''s `ts`, with `already_linked: false`."'
    sidecar: 'standard-contracts `links`: the `link` answer for a held and a new pair (F1).'
    rationale: 'Report Symbols / APIs: a second `link` of a held pair appends nothing and answers the same `since` with `already_linked: true`.'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 41
    section: '## Standard Contracts'
    change: 'Add after the `links` entry: "- `unlink` answers `{}` whether or not a pair was removed; an absent pair appends nothing and leaves the set as it was."'
    sidecar: 'standard-contracts `links`: `unlink` answers `{}` either way, an absent pair appending nothing (F2).'
    rationale: 'Report Symbols / APIs: `unlink` answers `{"ok":{}}` whether or not a pair was removed (an absent pair appends nothing).'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 41
    section: '## Standard Contracts'
    change: 'Add after the `links` entry: "- A `-32602` from `link` / `unlink` (a missing, non-string or invalid name, a `driven` that is not the serving wrapper''s own name, a `driver` equal to `driven`) writes no line; a `record` / `remove` failure is `-32603` and leaves the set unchanged."'
    sidecar: 'standard-contracts `links`: the link methods'' refusals write no line and a failed record leaves the set (F4).'
    rationale: 'Report Symbols / APIs: each bad pair is `ProtocolError::InvalidParams` (`-32602`, `data: null`), no line written; a `record` / `remove` error is `ProtocolError::Internal` (`-32603`); unit case `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`.'
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 47
    section: '## Standard Contracts'
    change: 'Read "the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.event` and `hook.dialog`, and of the MCP `send` / `wait` / `last` / `answer` tools".'
    sidecar: 'standard-contracts channel methods: the server-check client list gains `link` / `unlink`.'
    rationale: 'Report Symbols / APIs step 5: one request through `cmd::client::connect` (peer pid and its start time against the snapshot''s `pid` / `started_at`, Linux and Windows; macOS the recorded gap).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 47
    section: '## Standard Contracts'
    change: 'Add to the channel-methods group: "- `link` and `unlink` read neither `from` nor `conn` as identity: a `from` of any shape changes nothing. Each answers only after its event line and the snapshot write have landed."'
    sidecar: 'standard-contracts channel methods: `link` / `unlink` take no identity from `from` or `conn` (F3).'
    rationale: 'Report Symbols / APIs: neither method reads `from` (a `from` of any shape changes nothing); each answers after its line and the snapshot write; unit case `links_set_method_a_from_changes_nothing`.'
    fact: F3
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 193
    section: '## Standard Contracts'
    change: 'Replace the line with: "- `link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, each sent to the driven instance''s wrapper, which serves both since chunk 2026-10-26-explicit-session-links (before: `-32601`); a missing or invalid name, a `driven` that is not the wrapper''s own name or a `driver` equal to `driven` is `-32602` with `data: null`, and neither reads `from` ([Links])."'
    sidecar: 'architecture body channel methods: `link` / `unlink` served, their answers and `-32602` causes; the hold retired.'
    rationale: 'Report Symbols / APIs: both methods served with `{since, already_linked}` / `{}`; Counts: `notserved` reads `architecture.md:193` stale; Retired: the `-32601` answer.'
    basis: .andromeda/architecture.md:193
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 14
    section: '## Occupied Resources'
    change: 'Read "the CLI frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink`".'
    sidecar: 'occupied-resources IPC endpoints: the server-checking CLI frame verbs are eight.'
    rationale: 'Report Counts: `frameverbs` reads `architecture-detail/occupied-resources.md:14` stale (six → eight).'
    basis: .andromeda/architecture-detail/occupied-resources.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 97
    section: '## Occupied Resources'
    change: 'Add `link` and `unlink` to "the `cli` verbs `verify`, `send`, … `release`, `revive --list` and `plugin install` have producers today", and read the producer clause "`send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink` — `<name>` the target instance, the driven one for `link` / `unlink` — write …", adding "(a refused `link` / `unlink` strict-modes check opens no role file and writes no line)".'
    sidecar: 'occupied-resources diagnostics: `link` / `unlink` write `cli-<driven>.ndjson`.'
    rationale: 'Report Symbols / APIs: step 1 refuses before `client::start` with no line in any log; step 2 `client::start(home, &driven)` opens the `cli-<driven>` role file (`process-start`); step 5 writes `process-exit{… exit_code:21, detail:"server-verify-failed", during:"connect"}`.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 105
    section: '## Occupied Resources'
    change: 'Replace "Three readers check these files before they use them." with "Four readers check these files before they use them (`viola link` and `viola unlink` sharing one check)."'
    sidecar: 'occupied-resources instance files: the link verbs are a fourth checking reader.'
    rationale: 'Report Symbols / APIs step 1: `check_instance` on the home and `instances/<driven>` before `client::start`.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 106
    section: '## Occupied Resources'
    change: 'Before "Both checks come first" add "`viola link` / `viola unlink` (chunk 2026-10-26-explicit-session-links) run `check_instance` on the home and `instances/<driven>` before `client::start`, so before any role file, log or snapshot read; a refusal is exit 21 (`--json` the `instance-unreachable` document, stderr empty; human the instance-state pair), no line written, the mode left as found."; read "Each of these checks comes first"; and replace "`check_instance` has these two product call sites" with "`check_instance` has these three product call sites (`src/cmd/hook/statusline.rs`, `src/cmd/revive.rs` and `src/cmd/link.rs`, the one site both link verbs share)".'
    sidecar: 'occupied-resources instance files: `check_instance(` product call sites 2 → 3 (rule: `check_instance(` under `src/`).'
    rationale: 'Report Counts: `checkinst` reads `architecture-detail/occupied-resources.md:106` "these two product call sites" stale, 2 → 3 by the master''s own rule (+ `src/cmd/link.rs:98`); Spec claims disproved: the master''s rule reads 2 → 3, not 3 → 4.'
    basis: .andromeda/architecture-detail/occupied-resources.md:106
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 325
    section: '## Occupied Resources'
    change: 'Read "`viola list`, `viola revive`, `hook statusline`, `viola link` and `viola unlink` check these files before they use them, ahead of any log, because opening a log narrows an existing home."'
    sidecar: 'architecture body instance files: the link verbs check the driven instance''s files ahead of any log.'
    rationale: 'Report Symbols / APIs step 1: `check_instance` before `client::start`, before any role file or log opens.'
note: 'Not picked, for the new-detail pass or the orchestrator: architecture.md:220 (contract-event-data: "No product path appends `unlink` yet", stale by the report''s `remove` pattern, and the implicit-only `link` writer); F8 (CLI `--json` output: the link / unlink `ok` documents and the strict-modes `instance-unreachable` document) and F11 (an unserved method''s channel response line carries no `method` key) have no picked entity to carry them.'
