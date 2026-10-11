# Codebase Research — 2026-10-10-session-links

## Scope
- **Depth:** deep · **Reads:** about 30 (file sections) · **Globs/Greps:** about 20
- **Harness rules consulted:** none — no live leg in this chunk. The smoke entries the plan lists repeat the
  previous chunk's (`viola-0.1.0/chunks/2026-10-10-budget-governor/plan.md`, its `## Test Commands` fence), read
  there with their notes.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's six notes at take-up (nothing dispatched; the founder-only
  steps on the card; the folded budget test with its control; a widening held as itself; a cut at the card; the
  CI read).
- **Read at:** HEAD `f02d54865815`, tree clean but for the wrap's bookkeeping and this chunk's take-up.

## Files inspected
- `crates/viola-core/src/lib.rs` (30–140, 470–560) — `EventKind` holds 13 variants; `ALL: [Self; 13]` (line 67),
  `from_name` over `ALL` (94), `data_keys` (100) and `as_str` (118) are exhaustive matches that each need the two
  new arms. Its test oracle `KIND_TABLE` is 13 rows (477); `kind_table_a_text_that_is_no_kind_parses_to_nothing`
  (529) lists `"link"` and `"unlink"` as no kind; `kind_table_all_is_the_thirteen_kinds_in_declaration_order`
  (546) names the count.
- `crates/viola-state/src/events.rs` (18–232; 245–266; 800–905) — `Source` is `Wrapper | Hook` (25); the one
  append helpers `append_event` (65) and `append_event_at` (83) heal a torn tail under the log's lock; the
  reader counts an unknown kind at line 261 and does not return it. Two cases use `link` as the unknown-kind
  sample: the inline one at line 809 and the every-kind table at 868–900 holds 13 rows.
- `crates/viola-state/src/replay.rs` (1–250) — `Replayed.links: Vec<Value>` (23) is never filled; `replay`
  (51) matches `Wheel`, `BudgetGate` and `SessionStart` and nothing else; it carries the span `state.replay`.
  Its test module has a helper `fn link(cause, id) -> SessionLink` (455), a name a new helper must not take.
- `crates/viola-state/src/snapshot.rs` (API) — `InstanceSnapshot.links: Vec<Value>` (54), written whole by
  `write_snapshot` (87).
- `crates/viola-state/tests/state_events.rs` (56, 160–198) — `every_kind()` returns 13 rows; the case
  `state_events_lines_outside_the_contract_are_counted_and_known_kinds_still_returned` uses a `link` line with
  `{driver, driven}` as its unknown kind (173).
- `crates/viola-state/tests/state_replay.rs` (names) — the replay's crate-level suite; `replayed()` (89) builds
  the expected state with `links` empty.
- `crates/viola-channel/src/server.rs` (345–372) — `METHODS` already holds `link` and `unlink` (362–363), so
  `channel-request` / `channel-response` lines label both today; no change there.
- `src/cmd/run.rs` (60–160, 236–310, 440–600, 780–800) — `Methods::dispatch_call` (133) routes seven methods
  and falls through to `-32601`; the unit case `methods_answer_method_not_found_for_every_other_method` (788)
  names `link` and `unlink` as not found. The start builds `WaitFeed`, rebuilds it from the log (272), restores
  the dialog counter (282), then `server.serve` (289), then `start_state` (298), whose first snapshot sets
  `links: Vec::new()` (547).
- `src/run/send.rs` (290–440) — `parse_from` (299) is the one `from` check; `send` (315) runs its refusals in
  order: the text (330–337), `human_typing` (338), the budget gate (345–352), the flight and running turn
  (354–365), the readiness wait (367–373), the second wheel and turn reads (374–383), then `ctx.issue` (384),
  which appends `send-issued` through `append_event_at` (416).
- `src/run/dialog.rs` (310–400) — `answer` (318) parses `from` (323), then refuses in order and takes the
  answer at line 367 under the slot's state lock; `hand_back` (309) is called only from the wheel's recorder
  thread (`src/run/wheel.rs:295`), never from the pump.
- `src/run/wheel.rs` (60–260) — `pause` (188) reads no gate; `release` (198) refuses a string `from` first; the
  answer-after-record form is `settled(self.apply(.., true))`.
- `src/run/snapshot.rs` (full) — `Snapshots::update` applies a change and writes the snapshot under one lock.
- `src/run/wait.rs` (96–135) — `rebuild` (112) is the start's existing pass over the log through `read_from`.
- `src/cmd/pause.rs` (full), `src/cmd/release.rs` (28–75), `src/cmd/client.rs` (1–150), `src/cmd/mod.rs`
  (12–170) — the client verb's shape: `client::start`, `live_endpoint` (the liveness-only pre-check),
  `answer_of`, `fault` (exit 20), `unreachable` (exit 21), `document` (`--json`), a span on the request fn;
  `Command` holds ten variants and neither `Link` nor `Unlink`; `client::start` files the role log under the
  target name it is given.
- `src/human.rs` (170–225) — one writer per result line; `override_end` (192) formats an RFC 3339 instant.
- `src/main.rs` (60–125, 318–345) — `role_of` files every first word but `hook`, `run`, `revive` and a flag
  under `cli`, so the two verbs get the catch site's one `error: internal error` line with no product change.
- `tests/cli_budget.rs` (1–190, 385–503) — the seven `path6_*` cases and the E5 budget leg; `feed`, `usage`,
  `send`, `release`, `gate_records`, `wait_snapshot` and `assert_logs_clean` are its helpers. No case is
  `cfg(unix)`.
- `tests/cli_answer.rs` (94–104, 167–180), `tests/cli_wheel.rs`, `tests/cli_wait_last.rs` (outlines) — `raise`
  releases a gated script step and returns the dialog id; the gated scripts are `path4.json` (dialogs) and
  `path3.json` (a turn ended on cue).
- `tests/channel_paste_validation.rs` (1–75) — the raw-channel form: an unstamped booted wrapper, the endpoint
  read from the snapshot, `viola_channel::Client::connect(endpoint, "cli")` and `request(method, params)`.
- `tests/support/cli.rs` (60–102), `tests/support/home.rs` (API) — `viola(home, args, stdin, from)` sets `from`
  as `VIOLA_NAME`; `Wrapper::{boot, stop, stop_keep, release, instance_dir, home}`; `stamped_home`.
- `.config/nextest.toml` (32) — the 20 s kill filter line names the verify-driven root binaries; `cli_budget`
  is on it.
- `schemas/diag-line.v1.json` (39–125) — `method` is a free string: no schema change for the two methods.
- `.andromeda/playbook.md` (34–70) — four `verdict: escalate` rules, three of them the boundary-widening
  class.
- `viola-0.1.0/verification-matrix.json` through `matrix.py show --id v1-13` and `--id v1-36`.

## Graph impact (from the code-graph query; rust plane, `db_state` fresh; lines are editor lines)
- **`EventKind::ALL` / `from_name` / `data_keys`** — 8 references (query 0 of the run dir's trace): `from_name`
  at `crates/viola-state/src/events.rs:202` and `crates/viola-state/src/replay.rs:57`, `data_keys` at
  `crates/viola-state/src/events.rs:214`, the rest inside `crates/viola-core/src/lib.rs`. No other crate
  matches on the enum exhaustively (`grep -rn -E '(EventKind|Self)::Plan\b'` over `src`, `crates`, `tests`:
  the arms sit in `viola-core` alone), so two new variants break no match outside it.
- **`replay`** — 14 call rows (query 1): one product caller, `read_snapshot_or_replay` at
  `crates/viola-state/src/replay.rs:130`; the rest are its unit cases, `state_replay.rs:265` and
  `tests/cli_budget.rs:483` and `:490`. A change to what it returns in `links` reaches `viola revive` only
  through that one caller.
- **`parse_from`** — 6 call sites (query 2): `src/run/dialog.rs:323`, `src/run/send.rs:295`,
  `src/run/wait.rs:138` and `:174`, `src/run/wheel.rs:189` and `:208`. The implicit link reads the value two of
  them already return.
- **`live_endpoint`** — 5 client callers (query 2): `src/cmd/answer.rs:114`, `src/cmd/last.rs:34`,
  `src/cmd/pause.rs:58`, `src/cmd/send.rs:212`, `src/cmd/wait.rs:43`. Two new verbs calling it add no
  `read_snapshot(` site: the count "five product readers" (`src/cmd/client.rs:96`, `src/cmd/hook.rs:262` and
  `:477`, `src/cmd/hook/statusline.rs:142`, `src/cmd/run.rs:354`) stands.
- **`Snapshots::update`** — 4 product callers (query 5): `src/cmd/run.rs:592`, `src/run/budget.rs:333`,
  `src/run/dialog.rs:305`, `src/run/wheel.rs:293`. The link set's rewrite is a fifth.
- **Names** — no workspace symbol is named `link`, `unlink`, `Links`, `LinkSet` or `LinkSlot` but two test
  helpers `link()` (query 3: `crates/viola-state/src/replay.rs`, `src/cmd/revive.rs`).
- **Crate edges** — `viola → viola-core`, `viola → viola-state`, `viola-state → viola-core`,
  `viola-channel → viola-core` (query 4). The chunk adds none.

## Patterns detected
- **State rebuilt from the log before the endpoint serves** (`src/cmd/run.rs:272`, `src/run/wait.rs:112`): one
  pass over `read_from(instance_dir, 0)` at start. The link set takes the same place in the start.
- **A slot handed its collaborator after construction** (`src/run/send.rs:123` `budget_with`, a `OnceLock`): how
  `SendSlot` got the budget gate without a constructor change. The link set reaches `send` and `answer` the
  same way.
- **Record, then snapshot, then answer** (`src/run/budget.rs:333`, `src/run/wheel.rs:293`): a state change
  appends its line, mirrors it into the snapshot through `Snapshots::update`, and only then replies; a record
  that cannot land is `-32603`.
- **A client verb** (`src/cmd/pause.rs:29–85`): `client::start`, the liveness-only `live_endpoint`, one request
  under an `#[instrument(skip_all, name = …)]` span, `--json` through `client::document`, the human line through
  one `human::write_*` function, `run::log_self_exit(0, None)`.
- **A result line read from the reply** (`src/cmd/release.rs:57–70`): the human line is built only from the
  `ok` payload's keys.
- **The folded case's neighbours** (`tests/cli_budget.rs:111`): boot from a stamped home, `feed(&dir,
  usage(91, 10))`, then `send` reads exit 11 and `REFUSED_FIVE_HOUR`.

## Conventions to follow
- **One append path**: every product line goes through `append_event` / `append_event_at`
  (`crates/viola-state/src/events.rs:65`, `:83`), which carry the torn-line heal; no second writer.
- **Names**: `ViolaName::try_new` on every name, client and wrapper (`src/run/send.rs:299` is the form for a
  param that is a name).
- **Spans**: `#[instrument(skip_all, name = "<area>.<operation>")]`; obs-plan §2 names `cli.<verb>` for a verb
  with no dedicated client span, so `cli.link` and `cli.unlink` need no new `<area>`.
- **Tests**: root cases use only `tests/support/` helpers; oracles are literals; a wrapper is stopped cleanly;
  a case that sends twice ends the first turn with a scripted Stop (`fixtures/fake-scripts/path3.json`).
- **Inline test modules** stay `#[cfg(test)] mod tests { … }` in the file.

## Mechanisms the plan rests on (each read at HEAD)
- **The control reads `budget-paused` whatever the turn state.** `send` evaluates the gate at
  `src/run/send.rs:345–352`, after `human_typing` (338) and before the running-turn check (356). With the
  driver holding the wheel and a five-hour reading of 91 with a reset ahead, the reply is `budget-paused` /
  `five-hour`, exit 11 (`tests/cli_budget.rs:125–127` is the same call).
- **`pause` goes last in the folded case.** After a `pause` the wheel is the human's and `send` returns at
  line 338, never reaching the gate.
- **The four verbs read no gate.** `answer` is `src/run/dialog.rs:318–370`, `wait` and `last` are
  `src/run/wait.rs:135` on, `pause` is `src/run/wheel.rs:188–192`; none holds a gate call (the five call sites
  are listed in scope.md's folded freight).
- **`send-issued` keeps its own offset.** `append_event_at` computes the line's start under the lock
  (`crates/viola-state/src/events.rs:90`), so a `link` line appended before `ctx.issue` moves no cursor rule.
- **The first snapshot is where a start's set must land.** `start_state` writes `links: Vec::new()` at
  `src/cmd/run.rs:547`; a restored set that is not put there is lost until the next change.

## New files to create
- `src/run/links.rs` — the wrapper's link set: restored from the log, changed by record and remove, mirrored to the snapshot
- `src/cmd/link.rs` — the `viola link` client verb
- `src/cmd/unlink.rs` — the `viola unlink` client verb
- `tests/cli_links.rs` — root cases for the implicit link, the restart, E5's link step and the two verbs
- `tests/channel_links.rs` — the raw-channel name-validation negative for the two methods

## Files to modify
- `crates/viola-core/src/lib.rs` — the two kinds and their test oracle
- `crates/viola-state/src/replay.rs` — the link rule and the replay's link half
- `crates/viola-state/src/events.rs` — two inline cases that use link as an unknown kind or list every kind
- `crates/viola-state/tests/state_events.rs` — the every-kind round trip and the unknown-kind sample
- `crates/viola-state/tests/state_replay.rs` — the crate-level link cases
- `src/run/mod.rs` — the new module's declaration
- `src/run/send.rs` — the implicit link on an issued send
- `src/run/dialog.rs` — the implicit link on a taken answer
- `src/cmd/run.rs` — the set at start, the first snapshot, the two dispatch arms and their unit cases
- `src/cmd/mod.rs` — the two subcommands and their dispatch
- `src/human.rs` — the three result lines
- `src/main.rs` — two role cases for the new verbs
- `tests/cli_budget.rs` — the folded closed-gate case
- `.config/nextest.toml` — the 20 s kill filter line gains the new stamped-home binary

## Open questions
- Does a `send` or an `answer` the wrapper refuses record the link? → blocks: plan-decision
- What keys does the `link` result carry, and does `unlink` carry any? → blocks: plan-decision
- What does an `unlink` of a pair that is not in the set do (a line, no line, a refusal)? → blocks: plan-decision
