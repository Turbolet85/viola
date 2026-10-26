## 2026-10-26-explicit-session-links — the link verbs' exit codes
**Section:** §Conventions (conventions.md: CLI exit codes `1`, `20`, `21` and the server-mismatch exit-21 group; architecture.md: CLI exit codes) · §Occupied Resources (occupied-resources.md: a new `res-binaries` group, the `link` / `unlink` subcommands)
**Change:**
- Exit `1`: the `cli`-role verbs gain `link` and `unlink`; for them exit 1 also covers a refusal reply and an `ok` of another shape than the verb's (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object).
- Exit `20`: a `-32602` from `link` / `unlink` is included (a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven`); the self-pair is refused by the wrapper only, never client-side.
- Exit `21`: `viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens) is the same exit and document as `list` and `plugin install` (human mode the instance-state `unable:` / `hint:` pair; no log opened, no line written, the mode left as found), not `revive`'s exit 1. architecture.md: was "a refused strict-modes check of `viola list` or `viola plugin install`"; now also `viola link` or `viola unlink`.
- Server mismatch: was six verbs, `send` to `release` (per "2026-10-26-server-verification-before-any-frame — exit 21 and the MCP unreachable result for a server that is not the recorded wrapper"); now eight with `link` and `unlink`, whose instance check (exit 21) comes first with the client's own refusals (exit 13), before any connect.
- Subcommands: `link` and `unlink` each take `<driver> <driven> [--json]` (one shared `LinkArgs`), the verb's instance the driven one; exit 0 with one result line, or `{"v":1,"ok":<ok>}` under `--json` (`link` `{since, already_linked}`, `unlink` `{}`); 21 when the instance check refuses, no live endpoint is recorded or the serving process is not the recorded wrapper; 20 for a wrapper error, a `-32602` included; 1 for a refusal reply or an `ok` of another shape.
**Why:** the chunk built both verbs; their shared call maps the reply to 0 / 20 / 21 / 1 and refuses the instance check with exit 21. A refused check exiting 21 with no log and no line is the founder's own answer to this chunk's P4 fork, live by dialog, relayed verbatim by the operator (`inputs#I3`). The wrapper-only self-pair refusal is the operator's answer at chunk 2026-10-10-session-links, a lean of the P5 card the founder approved by his own word, relayed verbatim by the operator (`inputs#I4`).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — [Links]: the explicit verbs and methods built in the instance-check form, the hold retired
**Section:** §Established Decisions [Links] (established-decisions.md: `dec-links`; architecture.md)
**Change:**
- Was "As landed (chunk 2026-10-10-session-links) only the implicit link is built" (per "2026-10-10-session-links — the implicit link as landed, the explicit verbs held"); now both are built: the driven instance's wrapper serves the `link` and `unlink` methods (`LinkSet::link` / `LinkSet::unlink`, sharing `driver_of`), sent by `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, and records the implicit link itself, its order and `-32603` unchanged.
- An explicit `viola link` of an existing pair adds no line; was "returns `{}`"; now it answers its held `since` with `already_linked: true`.
- The hold: was not built (`-32601`, no subcommand, no product `unlink` line, `LinkSet::remove` with no caller) and held on "Explicit session links" by the founder's 2026-10-13 answer (per "2026-10-13-t06-38-35-wrap — the founder's Epoch 4 batch: budget.json ratified, rate_limits ruled, the board's refusal his, the link hold in Epoch 6"); now built with no dated gap, the `unlink` method `LinkSet::remove`'s product caller, so a product path appends `unlink` lines (`source: wrapper`, `{driver, driven}`).
- New: the verbs run `viola_state::strict::check_instance` on the home and `instances/<driven>` before `client::start`, ahead of any role file, log or snapshot read, because opening a log narrows an existing home to 0700 and a check made after it passes a home at 0770; a refusal is exit 21. Then the liveness-only pre-check (`live_endpoint`), the server identity check through `cmd::client::connect`, and one request.
- architecture.md: was "only the implicit link is built, the explicit verbs held"; now the implicit link and the explicit verbs are built, the verbs checking the driven instance's files before any log and its wrapper serving `link` / `unlink`.
**Why:** the chunk landed both verbs and both served methods and retired the hold, the `-32601` answer and `LinkSet::remove`'s dead-code allow. The boundary widening they carry is ratified, with no dated gap, by the founder's own answer to this chunk's P4 fork, live by dialog, relayed verbatim by the operator (`inputs#I3`); this wrap's answer-card confirm is the overseer's, founder-delegated, PROVISIONAL (`inputs#I6`). The order is held by a test that reads the mode after the refusal.
**Kept:** the implicit link's five shapes on the operator's word stand, not ruled by the founder.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link methods' log lines and the since stamp
**Section:** §Established Decisions [Links] (established-decisions.md: two new `dec-links` bullets)
**Change:**
- The wrapper opens no span and writes no process-log line of its own for `link` / `unlink`: its `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name. The verbs run inside the `cli.link` / `cli.unlink` dispatch span.
- `LinkSet::line` stamps the `link` line's `ts`, the pair's `since`, with `Utc::now()` directly, outside the `Clock` seam.
**Why:** as built and measured at the chunk: its tests read the channel pair holding no names, and its root cases assert `since` by its shape only, no seam injecting it.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the server check before the first frame covers link and unlink
**Section:** §Occupied Resources, IPC endpoints (occupied-resources.md) · §Standard Contracts, Channel methods (standard-contracts.md: the server-check group)
**Change:**
- IPC endpoints: the CLI frame verbs that check the serving process before their first frame were `send` / `wait` / `last` / `answer` / `pause` / `release`; now also `link` / `unlink`.
- Channel methods: the clients that compare the peer pid and its OS start time with the snapshot's `pid` and `started_at` before the first frame gain `link` and `unlink` (was `send`, `wait`, `last`, `answer`, `pause`, `release`, `hook.event`, `hook.dialog` and the MCP frame tools, per "2026-10-26-server-verification-before-any-frame — the snapshot's pid and started_at as the server identity every frame-writing client checks"). macOS stays the recorded gap.
**Why:** each verb sends its one request through `cmd::client::connect`, so the CLI frame verbs are eight (six before); against an impostor server both read no frame and exit 21 in both modes on Linux and Windows. The server check is part of the form the founder named at this chunk's P4 fork, his own answer relayed verbatim by the operator (`inputs#I3`).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — link and unlink write the driven instance's cli log
**Section:** §Occupied Resources, Filesystem (occupied-resources.md: `diagnostics/`)
**Change:**
- The `cli` verbs with producers gain `link` and `unlink`: like `send` to `release` they write into `cli-<name>.ndjson`, `<name>` the driven instance, `process-start`, the client `channel-request` / `channel-response` lines and `process-exit{subject:"self"}`, with the exit-21 `instance-dead` / `server-verify-failed` and exit-20 `wrapper-fault` details.
- A refused `link` / `unlink` instance check opens no role file and writes no line.
**Why:** the verbs open the `cli-<driven>` role file only after the instance check has passed, and a refused check writes no line in any log.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — link and unlink check the instance files before any log; check_instance at three sites
**Section:** §Occupied Resources, Filesystem (occupied-resources.md: `instances/<ViolaName>/`; architecture.md: `instances/<ViolaName>/`)
**Change:**
- Was "Three readers check these files before they use them"; now four (`viola list`, `viola revive`, `hook statusline`, and `viola link` / `viola unlink` sharing one check).
- `viola link` / `viola unlink` run `check_instance` on the home and `instances/<driven>` before `client::start`, before any role file or log opens and before any snapshot value is read; a refusal is exit 21 (`--json` the `instance-unreachable` document, stderr empty; human the instance-state `unable:` / `hint:` pair), no line written, the mode left as found. Was "Both checks come first"; now "Each of these checks comes first".
- `check_instance` product call sites: was two; now three (`src/cmd/hook/statusline.rs`, `src/cmd/revive.rs` and `src/cmd/link.rs`, the one site both link verbs share; the rule `check_instance(` under `src/`).
- architecture.md: the processes that check these files ahead of any log gain `viola link` and `viola unlink`.
**Why:** measured at the chunk's tree, `check_instance(` under `src/` reads 2 → 3. The plan's "3 → 4" counted `list`'s check, which runs inside `viola_state::board::scan`, outside `src/`, against the master's own rule: a call-site count is taken by the rule its master states.
**Kept:** the count's rule stays `check_instance(` under `src/`; `list` is a checking reader with no call site under it.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the served link and unlink methods and the link set
**Section:** §Standard Contracts, Instance snapshot (standard-contracts.md: `contract-instance-snapshot`, four new bullets) · Channel methods (standard-contracts.md: a new `contract-channel-methods` group; architecture.md: the `link` / `unlink` row)
**Change:**
- Channel methods: was "`link` and `unlink` `{driver, driven}` … → `{}`. Neither is built …: the wrapper answers both `-32601`" (per "2026-10-10-session-links — the implicit link as landed, the explicit verbs held"); now `link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, each sent to the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null`.
- `link` and `unlink` read no `from`: a `from` of any shape changes nothing. Each answers only after its event line and the snapshot write.
- `links`: the set changes on an implicit link and on a served call: `LinkSet::link` adds the pair through `record`, `LinkSet::unlink` removes it through `remove`. A `link` of a held pair appends nothing and answers that pair's `since` with `already_linked: true`; a new pair answers its new `link` line's `ts` as `since`, with `already_linked: false`. `unlink` answers `{}` whether or not the set held the pair; an absent pair appends nothing and leaves the set as it was. A `-32602` writes no line; a failed record or remove is `-32603`.
**Why:** the chunk serves both methods with these params, refusals and payloads. Serving them is the boundary widening the founder ratified at this chunk's P4 fork, his own answer relayed verbatim by the operator (`inputs#I3`).
**Kept:** the set still follows the log: its restore at every start, its replay rule and its failed-append rule stand as written.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — a product path appends the unlink event
**Section:** §Standard Contracts, Event data (architecture.md: the wrapper-appended paragraph) · Channel methods (standard-contracts.md: the `wait` group's wake sentence)
**Change:**
- Was "Since chunk 2026-10-10-session-links the driven instance's wrapper also appends `link` … on an implicit link … No product path appends `unlink` yet: its one writer, `LinkSet::remove`, has no caller until the `unlink` method is served"; now the driven wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new pair, on an implicit link or a served `link` call, and `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes (`LinkSet::remove`). Both are log-only and never wake `wait`, and a replay learns the link set from them.
- `wait`: the wrapper records that never wake gain `unlink` (the `send-*`, `link`, `unlink` and start records).
**Why:** `LinkSet::remove` has its first product caller, so the `unlink` line, a kind already in the closed set, is written by a product path for the first time; it is not in `WAIT_WAKE`.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' --json documents
**Section:** §Standard Contracts (standard-contracts.md: a new `contract-cli-json-output` group; architecture.md: the subjects list gains `contract-cli-json-output`)
**Change:**
- New group: `viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` (`since` a string, `already_linked` a boolean) and `viola unlink <driver> <driven> --json` prints `{"v":1,"ok":{}}`, each with exit 0. Their refused strict-modes check on the home and `instances/<driven>`, made before any role file or log opens, prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21), stderr empty and no line in any log.
**Why:** the chunk's verbs print these documents, and the detail file held no group for CLI `--json` output.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — links.rs in the directory structure
**Section:** §Infrastructure Patterns → Project directory structure (project-directory-structure.md: `src/run/links.rs`)
**Change:**
- Was "`remove`, which has no product caller yet"; now `remove` (its product caller the served `unlink` method), and the served `link` / `unlink` channel methods, sharing `driver_of`.
**Why:** `LinkSet::link` / `LinkSet::unlink` share `driver_of`, and `LinkSet::remove` has its first product caller; the key file restated the retired no-caller claim, a site the chunk's sweep did not list.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/
