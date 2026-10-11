# Scope — 2026-10-11-the-board-viola-list

## Working entry (the intent, as the route words it)

The board: viola list — BAY header over six-column board, liveness, status, wheel/budget pause, amber DIALOG, dim
stale rows, CLI verdict, read-only unwrapped rows

The entry carries eleven `CARRY:` blocks (`route.py pins`: 11 rows on its line, no abstention). Each is folded
below, under "Folded freight", as in this slice or as held for the review card.

## The cut (the operator's direction, `inputs#I1`)

The entry does not fit one window. This chunk plans the smallest coherent slice that needs no word of the
founder. Every other part goes on the review card, each priced, with a proposed owner. The founder's own
matters go on the card as themselves, unsettled, and no step of the slice rests on them: any ledger row,
`viola verify` child, re-stamp or live `claude` start; the held explicit `viola link` / `viola unlink`; the
switch from the prototype (`verification-matrix.json#v1-33`). No mutation run, witness run or dispatch is part
of this chunk. The CI read is the `ci.py conclusion` call.

## The slice — what this chunk builds

`viola list` for wrapped sessions: the read model, the machine form and the uncoloured human rows.

- The verb `viola list` with the global `--home` and with `--json`. It is not a channel method: the calling
  process reads the home's `instances/*/` itself and needs no live wrapper (architecture, the channel methods'
  `list` line).
- The machine form `viola list --json`: one document `{"v":1,"ok":{"items":[…],"budget":…,"skipped":{…}}}` on
  stdout (layout-templates, Output structure — `viola list --json`).
  - each item is `{name, wrapped, liveness, status, wheel?, budget_paused?, dialog_pending, cli_version?,
    cli_verified?}` (architecture, GUI list envelope). Every item of this slice is a wrapped one.
  - `budget` is `{five_hour, seven_day, read_at, paused}` or `"unknown"`.
    [premise-corrected: `viola_state::budget::read_budget` returns the two windows only and drops `read_at`
    (`crates/viola-state/src/budget.rs`, `Stored`), and nothing on the `list` side reads the thresholds
    `paused` is measured against] The slice gives the one checked read a second form that also returns
    `read_at`, and `paused` is the gate's own crossed rule over the thresholds `config.json` gives the wrapper
    (`src/run/budget.rs`). No second reader of `budget.json` is written.
  - `skipped` is `{unknown_kinds, unknown_fields, torn_lines}`: the reader's three counts, shown for the first
    time. The total is the sum of one whole-log pass per instance directory that passes its check, gone
    instances included; a replayed instance's pass is the replay's own (research, "Findings the plan rests
    on").
- The human form `viola list`: the BAY context line, the caption row `NAME LIVE STATUS WHEEL DIALOG CLI`, one
  strip row per wrapped session ordered by name, and the empty-board line (layout-templates, Output structure —
  `viola list`, both forms, and the two cli Components for the BAY line and the strip rows).
  - It prints the uncoloured form on every stream: the same characters the masters give for a piped stream,
    with no SGR. Verified against layout-templates and design-system: that form is lossless in words. The
    masters also require bold NAME, an amber `DIALOG` word and a dim stale row on a colour-capable terminal;
    that requirement stands unmet after this chunk and goes on the review card with the colour layer.
  - Nothing is truncated: the width is treated as unknown, and only unwrapped names are ever clipped
    (design-system, cli Tokens, Width).
  - The row-mode escaper lands with its first consumer here, for the recorded CLI version text: every control
    character as `\xHH` text, LF and TAB included, DEL and C1 included.
    [premise-corrected: the masters name the row mode for fields of `claude agents --json` origin only, and
    word its set without DEL; the landed message mode already escapes DEL (`src/human.rs`, `escape_message`)]
    Applying the row mode to a snapshot field is this slice's own rule and an expected amendment.
- The read model, per instance directory:
  - the instance check (`viola_state::strict::check_instance`) before any snapshot or log value is used and
    before any file under the home is opened, the process log included (opening it narrows the home). What
    `list` does when the home, `instances/` or one instance fails the check is worded by no master. The
    operator's answer to the P4 fork (`inputs#I2`, not a ruling of the founder): the whole board is refused
    with exit 21, nothing read, no log opened, the home's mode left as found; `--json` prints the one worded
    exit-21 document, and the human form the worded pair for what failed.
  - the snapshot through `viola_state::replay::read_snapshot_or_replay`, so a session whose snapshot is
    unreadable or newer still shows `wheel`, `budget_paused` and its liveness.
  - liveness `live` / `stale` from the heartbeat age and the snapshot's pid and start time; a gone instance is
    omitted. For an unreadable or newer snapshot the liveness rests on the heartbeat alone and is never
    `stale`: a fresh beat is `live`, anything else leaves the instance off the board. An absent snapshot gives
    no row.
  - `status`. [premise-corrected: no file on disk says a session is idle or busy; the snapshot has no such
    field and a running turn records nothing (architecture sidecar, 2026-10-04-running-turn-refusal)] The
    masters source it from `claude agents --json` and give `unknown` when that read is missing, failed or not
    joined. Every row of this slice reads `unknown`.
  - `dialog_pending` only from the snapshot's `pending_dialog`; a reader that fell back to replay reports
    `dialog_pending: false` (architecture).
  - the CLI cell from the snapshot's recorded version and `cli_verified`; a missing version prints `unknown`.
    No ledger file is read for it.
  - an instance directory name passes `ViolaName::try_new` before any path join.
  - nothing of the snapshot's `endpoint`, `pinned_bin`, `cwd` or `statusline_command`, and nothing of a
    pending dialog but its kind word, reaches a row, the document, an error or a log line.
- Tests owed with these surfaces (test-plan §6): Scenario E5's `list --json` signal, driven on a live
  wrapper's snapshot cut mid-object and on a sibling set to `v` 99; the Chaos suite's `skipped.torn_lines`
  reading and the forward-compat case's reading, both on `list --json`; Path 6's `list --json` readings
  (`budget` and each item's `budget_paused`) and the board's budget cells, as Unix root cases.

## Boundaries — not built by this chunk

- No unwrapped row, no `-- UNWRAPPED - READ-ONLY --` separator, no read of `claude agents --json`, no parser for
  it, no fake-agent `agents --json` stub.
- No colour: no SGR token, no read of `NO_COLOR` / `TERM` / `COLORTERM`, no `IsTerminal` branch, no Windows
  console mode change.
- No link column and no link shown anywhere (layout-templates, "No link column"; the session-links plan's
  Boundaries). The operator's rule of `inputs#I1`, item 3, binds the first surface that shows a logged link
  name: it parses the name through `ViolaName::try_new` again. No surface of this slice shows one.
- No `viola link` / `viola unlink`, no channel method, no MCP tool, no `/api/sessions` route.
- No `session-live` preflight for `viola revive`.
- No `viola-e2e` `path_` binary, no `run --e2e` selector, no `agent-run boot --statusline-echo`, and the
  harness `status` document's `list` member stays `null`.
- No grouped `--help` table: the built help is clap's ordered list.
- No ledger row, no `viola verify` child, no re-stamp, no live `claude` start, no fixture recording.
- No mutation run, no witness run, no workflow dispatch.

## Folded freight — the eleven carries

In this slice:

- **self-healing-state** (the operator's direction at that wrap, that chunk's own input I4): in full. The three counts as
  `list --json`'s `skipped`; the Chaos suite's `skipped.torn_lines` reading and the forward-compat reading on
  `list --json`; `wheel`, `budget_paused` and liveness for an unreadable or newer snapshot through
  `read_snapshot_or_replay`; Scenario E5's `list --json` step. `verification-matrix.json#v1-41` is advanced and
  not proven until these show.
  - "`events::read_from` returns `Skipped{unknown_kinds, unknown_fields, torn_lines}` per read, from zero":
    verified in `crates/viola-state/src/events.rs`.
  - [premise-corrected: `LoggedLines::skipped` has two library callers, `replay` and `session_chain`, which
    carry the counts out in their results; no product code reads either result's `skipped` (`viola revive`
    drops it)] The board is the first product reader of the counts.
- **budget-governor**, the board half only: `list --json`'s `budget` and each item's `budget_paused`, and the
  board's cell (`driver - budget-paused`, the BAY line's `gate open` / `budget-paused`).
  - "The gate never polls, so a snapshot can read `budget_paused: true` after the reset until a `send`
    arrives or a turn ends (architecture [Budget Governor]); the board shows what the snapshot holds."
    Verified against the architecture sidecar (2026-10-10-budget-governor): the snapshot's two budget fields
    are written only on a change. The item's `budget_paused` is the snapshot's word; the envelope's `paused`
    is recomputed from the reading. They are two facts.
- **cli-output-tokens**, the escaper only: the row-mode escaper in `src/human.rs`, with its first consumer.

Held for the review card, a founder's matter (each shown as itself, unsettled; no step rests on it):

- **capability-ledger-and-viola-verify**: the `claude agents --json` join-field ledger row and its
  `viola verify` probe.
- **fake-agent-and-test-data-fixtures**: the fake agent's `agents --json` stub, pinned against a recorded
  `claude agents --json` output (a recording is a live start).
- **viola-revive**: the `session-live` preflight refusal, with its join-field row, its probe and the study's two
  measurements (one more live start).
- **first-live-test-and-self-drive**: the switch from the prototype and `verification-matrix.json#v1-33`; which
  groups of that chunk's gap list the switch needs, and which chunk claims `v1-33`.
- **session-links**: the explicit `viola link` / `viola unlink`, held for the founder's own word on a boundary
  widening, his three options as that plan's step 11 words them. Nothing of its steps 12 to 15 is planned here.
- **cli-output-tokens**, the colour layer: reading `NO_COLOR` / `TERM` / `COLORTERM` widens architecture's env
  sentence and needs a Decisions Log entry and a live founder ratification (playbook "Boundary widening").

Held for the review card, no founder's word needed (each priced at P4, with a proposed owner):

- **quality-gates**: the proptest property and the fuzz target for the `claude agents --json` parser. It waits
  on the parser, which waits on the founder's matters above.
- **instance-state-and-start-order**: the `viola-e2e` `path_` E2E binary for test-plan §6 Path 1.
- **h2-conpty-resize-probe**: the harness's `run --e2e` selector and the integration filterset's `!binary(…)`
  exclusion, owed with the first `path_` binary. Until then no plan lists `run --e2e` as a gate, this one
  included.
- **budget-governor**, the harness half: `agent-run boot --statusline-echo`, the harness-session form of Path 6,
  and step 7's two-instance leg on a booted pair.
- **cli-output-tokens**, the pure parts of the colour layer (the SGR tokens, the decision order as a pure
  function): no code without a live consumer (that chunk's P4 ruling), so they travel with the colour layer.

## CI verdict read at Setup

- `995dc55a3d6b` (the last wrap's flip, HEAD): green, 15/15 checks, `ci#38100502990`, wall 540 s. No other
  commit stands between the flip and HEAD. Nothing to disposition.

## Premises closed by research

- No `list` verb, module or test exists in the tree: verified (`src/cmd/` holds no `list.rs`; the only `list`
  words in `src/` are `revive --list` and hint texts).
- [premise-corrected: `role_of` already files `list` under the `cli` role, since every first word but `hook`,
  `run`, `revive` and a leading flag is one (`src/main.rs`)] No product edit there; one table row is added.
- The replay's result holds `wheel`, `budget_paused` and the pass's `skipped`: verified
  (`crates/viola-state/src/replay.rs`, `Replayed`).
- A home that does not exist passes the check ("each that exists") and is an empty board. An entry of
  `instances/` that is not a directory, is a link, or whose name is no `ViolaName` gives no row; obs-plan §6
  already catalogues the one line such a scan writes (`parse-rejected`, parser `state-entry`).
- Every test reading of this slice is a root case (`tests/cli_*.rs`, `tests/chaos_*.rs`). The harness-session
  forms are outside it.
