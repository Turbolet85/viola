# Codebase Research — 2026-10-06-local-command-send-outcomes

## Scope
- **Depth:** deep · **Reads:** 22 · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, 8 additions; applied: the
  2026-09-27 `binary(<stem>)` / `test(/…/)` selector forms and the 2026-09-29 entry as extended 2026-10-06 (a common
  stall across process-spawning tests is a host stall; read the red entry's log before a re-run overwrites it).
  `.claude/rules/testing.md` — read whole, 30 additions; applied: 2026-09-25 (every new guard test carries its
  remove-the-guard run, the site re-read before each neutralising edit), 2026-09-27 (a timing window is forced open
  with a test-only hold, never sampled), 2026-09-28 (a timing red is never fixed by a raised bound), 2026-10-04 (no
  mutation entry in a `[[gate]]` block).
- **Platform issues consulted:** none — no runner-only bullet, and the CI run read at take-up is green.
- **External inputs:** `inputs#I1` — the operator's directive at the take-up: no live `claude` session, plan zero;
  the readiness-gate CARRY goes to P4 as a priced card, and nothing about the 5 s maximum or `send` under the hint is
  decided before it; a stalled-start baseline red is re-read with no other build running; one builder window;
  Epoch 3 stays one epoch. `inputs#I2` — the answer to the P4 readiness-gate card (the overseer, founder-delegated,
  unattended, 2026-10-06, snapped at P4): option B, the gate unchanged and the reading measured under the fake agent
  with the paste-hint hold; a change to what `send` does under the hint is the founder's and is not decided here; the
  `input-not-ready` hint finding is carried to "First live test and self-drive" with the measured reading.

## Measured facts

### M1 — `send` does not consume the compiled list
- `LOCAL_COMMANDS` is `[("/clear", Some(PostCondition::NewSession)), ("/remote-control", None)]`
  (`crates/viola-agent-claude/src/ledger.rs:151-154`); `PostCondition` has the one variant (`:143-147`).
- `grep -rn "LOCAL_COMMANDS\|PROBE_LOCAL_COMMAND" --include=*.rs crates src tests`: hits in `ledger.rs`,
  `src/bin/viola-fake-agent.rs`, `src/cmd/verify/typed.rs` and tests only; 0 in `src/run/` and `src/cmd/send.rs`.
- Both names are `pub` in `viola_agent_claude::ledger`, which the root bin already imports from
  (`src/cmd/verify/typed.rs:25`). No re-export is needed.

### M2 — where `cli_verified` is, and what threading it costs
- `run` reads it once from the version gate (`src/cmd/run.rs:208-209`) and builds the send slot seven lines later
  (`:216`), before the endpoint serves. The same value already goes to `DialogSlot::new` (`:221-223`) and to
  `gate::start` as the signature set (`:492-493`).
- `SendSlot::new(clock, wheel)` (`src/run/send.rs:58`) has two callers outside its own file, both in
  `src/cmd/run.rs`: `start` (`:216`) and the test helper `methods` (`:637`). Inside `src/run/send.rs` the unit tests
  call it at 15 sites (`grep -c "SendSlot::new(" src/run/send.rs`). A new parameter touches those two files only
  (graph, below).
- The value comes from `read_stamps_strict`; the slot adds no stamps read of its own.

### M3 — what the specs fix, and where they are silent
- architecture [Delivery Confirmation] (`architecture.md:49`, read whole): a listed command whose measured
  post-condition is met returns `ok`, confirmed; one whose post-condition is "none" or not yet measured on this CLI
  version returns `ok` with `{"confirmed":false,"detail":"unconfirmable"}`; "Nothing else is ever unconfirmable. Any
  other unconfirmed send is `not-delivered`. The decision depends on the ledger, never on a leading slash."
- The wire has two `ok` shapes for `send` (`architecture.md:275`): `{submitted_at, cursor}` and
  `{confirmed:false, detail:"unconfirmable", cursor}`. A post-condition-confirmed `/clear` therefore returns the
  first; design-system gives it the `[RB] read back` line (design extract, Constraints 3).
- Silent, three points (arch, tests, design and layouts extracts agree):
  1. the `events.ndjson` record of an `unconfirmable` send. architecture gives `send-confirmed` one field,
     `{cursor}`; obs-plan gives the process log `send-confirmed{confirmed:false}` (`obs-plan.md:591`, `:643`);
     design-system asks the CL-1 records for "an outcome record for `ok`/`confirmed:false`"
     (`design-system.md:576`);
  2. the detail when a listed command's measured post-condition does not arrive in the window. The closed
     `NotDelivered` set is `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`,
     `control-character` (`crates/viola-core/src/lib.rs:122-128`);
  3. the source of `submitted_at` for a post-condition-confirmed send. The landed rule for a prompt is "claim,
     append, then settle with that line's `ts`" (`architecture.md:49`; `src/run/send.rs:146-171`).
- Precedent for point 2: chunk 2026-10-04-running-turn-refusal gave `turn-running` a second cause under the one
  closed detail, with no new field, event or schema change (obs history, its item).

### M4 — the hook tap, and the missing session id
- `append_hook_event` (`src/run/send.rs:140-173`) is the one place every hook line passes before it is appended. It
  already ends the running turn on `session-start` (`:158-160`). Its one caller in product code is
  `Methods::dispatch` (`src/cmd/run.rs:117`).
- A `session-start` line's `data` is `{"cause", "agent_session_id"}` (`crates/viola-agent-claude/src/hook.rs:147-155`;
  `cause` is closed, `unknown` otherwise). The wrapper's re-validation of a `hook.event` checks the kind and that
  `data` is an object; it names no `session-start` field (`src/cmd/run.rs:89-99`). So the tap must read both fields
  as data that may be absent or `null`.
- The wrapper keeps no session id: `grep -rn agent_session_id --include=*.rs src crates/viola-state
  crates/viola-core` returns two hits, both test literals (`crates/viola-state/src/events.rs:338`,
  `src/cmd/run.rs:871`). "A new `session_id`" needs the slot to remember the id of the last `session-start` it
  appended.
- The recorded set has what the confirmation needs: `SessionStart.default.json` carries one `session_id`
  (`c0f0cc23…`), `SessionStart.clear-1.json` another (`d5d38bc2…`) with `source` `clear`, and
  `SessionEnd.clear-1.json` a third with `reason` `clear` (`jq '{session_id,source,reason}'` over the three files of
  `fixtures/claude/2.1.287/`).
- A SessionEnd the hook appends directly, without the channel, never reaches the tap (arch history,
  2026-09-27-hooks-to-normalised-events). The confirmation keys on the `session-start` alone.

### M5 — the outcome writers and the two logs
- `Ctx::confirm` appends `send-confirmed {cursor}` and logs `send-confirmed` with `confirmed = true` and
  `duration_ms` (`src/run/send.rs:325-347`); `Ctx::refuse` is the refusal twin (`:359-389`).
- `schemas/diag-line.v1.json:59-65` admits `confirmed` (boolean), `detail` (string) and `duration_ms` on the three
  send lines; `:115-117` requires `corr` on a wrapper-side one. `send-confirmed{confirmed:false}` passes G4 with no
  schema change.
- No product code reads the `send-confirmed` record: `grep -rn "SendConfirmed\|send-confirmed" --include=*.rs src
  crates` outside `src/run/send.rs` returns the two enums in `viola-core` and one `wait` test case
  (`src/run/wait.rs:380`, a kind that does not wake). An added `confirmed:false` field meets no reader at HEAD.
- The client: `Reply::Ok(Value)` carries any `ok` body (`src/cmd/client.rs:34-43`). `deliver` hands every `ok` to
  `Out::read_back` (`src/cmd/send.rs:198-202`), which under `--json` prints the document unchanged and otherwise
  prints `[RB] read back` from `submitted_at` and `cursor` (`:113-125`). At HEAD an `unconfirmable` reply would print
  a filled box with an empty time. The `--json` half needs no change; the human half needs its own arm.
- The mirror: `MIRROR_WORD` is 13, the length of `unconfirmable` (`src/human.rs:40-41`); `mirror_line` pads every
  word to it (`:43-45`). Three writers exist (`write_send_open`, `write_read_back`, `write_send_unable`); a fourth
  joins them, and `write_send_mirror_words_line_up` (`:365-373`) is the test that pins the shared column, ASCII and
  no ESC byte.
- No trycmd case exists: `tests/cmd/` is absent. The mirror lines are pinned by literal asserts in `src/human.rs`
  and `tests/cli_send.rs`.

### M6 — the fake agent as it is
- `--local-command-mode`: a prompt starting with `/` fires nothing and is receipted `submit:"local-command"`
  (`src/bin/viola-fake-agent.rs:454-455`).
- `--framing`: the text `/clear` fires the recorded `clear-1` SessionEnd, then SessionStart, and no UserPromptSubmit
  (`:451`, `:456-458`). `--local-command-mode` wins when both are set (the `else if` order).
- `Wrapper::boot` always passes `--cli-version 2.1.287`, `--screens` and the trusted root
  (`tests/support/home.rs:357-390`). A `stamped_home` is a verified child (the full gate); the `boot` helper of
  `tests/cli_send.rs` uses `StampedHome::unstamped` (`:33`), an unverified one.
- So every W1 outcome is reachable with the options that exist:
  - unverified, `--local-command-mode`, `/clear` or `/remote-control`: `unconfirmable`;
  - verified, `--local-command-mode`, `/remote-control`: `unconfirmable`;
  - verified, `--framing`, `/clear`: confirmed by the replayed SessionStart;
  - verified, `--local-command-mode`, `/clear`: the post-condition never arrives;
  - any home, `--local-command-mode`, a slash text off the list: `not-delivered` after the window.

### M7 — what the gate does with a quiet screen that holds no literal
- `Screen::verdict` (`crates/viola-agent-claude/src/screen.rs:122-152`): while the screen is not quiet it waits,
  up to `GATE_MAX_WAIT` from the start of the wait (`:131-135`). Once the screen has been quiet for `QUIET_PERIOD`
  (300 ms) it decides at once: no signature set, `Ready` (`:137-139`); a modal literal, `InputNotReady` (`:142-144`);
  an input-box literal, `Ready`; neither, `InputNotReady` (`:147-151`).
- The equality the W2 hypothesis needs — a verified gate, a quiet screen, no `for agents` row, no modal row, gives
  `InputNotReady` — is pinned at unit level already: `wait_ready_verified_over_the_input_box_is_ready`
  (`src/run/gate.rs:557-568`) asserts it for a screen holding only `thinking`, 300 ms after its last byte.
- `send` types nothing on that verdict and records `send-refused` with no cursor (`src/run/send.rs:264-267`).
- So on a verified CLI a `send` issued while the paste hint stands is refused about 300 ms after the screen goes
  quiet. The 5 s maximum is not reached and plays no part. Raising it alone changes nothing for this case.
- verify's own settle differs by design: `settled` waits out `GATE_MAX_WAIT` on a quiet literal-less screen
  (`src/cmd/verify/typed.rs:477-487`), and `box_wait` keeps waiting past it (`:502-505`).
- The driver-facing side: the hint printed for `input-not-ready` is "`<name>` was not ready for input; viola wait
  `<name>`, then send again" (`src/human.rs:208-210`). In the hint window the turn has already ended, so a `wait`
  has no later event to wake on.

### M8 — the paste-hint hold reaches a `send` test as it stands
- `--paste-hint-ms` acts inside the agent's `submit`: for the compiled long text under `--framing`, with
  `--turn-stop`, the agent fires the recorded UserPromptSubmit and the Stop, clears the screen, sleeps the hold, then
  draws the `turn` screen (`src/bin/viola-fake-agent.rs:460-475`). It does not matter who pasted the text.
- `tests/cli_send.rs` already sends that text under `--framing`: `send_long_text_wrapped_by_the_cli_is_confirmed`
  takes it from the recorded `paste-1` variant (`:493-510`).
- On a `stamped_home` wrapper with `--framing --turn-stop --paste-hint-ms <n>`: the first send is confirmed, the
  Stop ends the turn, and a second send issued inside the hold meets a quiet cleared screen. No new option is needed.
- The agent sleeps on its stdin thread, so it reads no key during the hold and `Wrapper::stop` returns only after
  it. A test's length is the stamp, the boot and the hold.
- Bounds a new case lives under: `binary(cli_send)` is killed at 10 s × 2 in `[profile.ci]`
  (`.config/nextest.toml:29-31`); the `send_window_` prefix buys 15 s × 2 under `[profile.mutants]` only (`:48-50`).

### M9 — the guard before the local-command paste has no control
- `framing_turns` (`src/cmd/verify/typed.rs:257-293`) holds the check three times: inside the loop before the long
  and the tag-like paste (`:261-263`) and once more before `PROBE_LOCAL_COMMAND` (`:278-280`).
- The agent draws the set's `turn` screen after every turn (`src/bin/viola-fake-agent.rs:474`).
  `verify_pastes_nothing_more_once_the_turn_screen_shows_a_modal` (`tests/cli_verify.rs:966-994`) writes a modal row
  beside the input-box row into its own set's `turn` screen, so the first check stops Run B and the third is never
  reached. The prior chunk's control neutralised the first two (`evidence/paste-guard-control.md`).
- A control for the third needs the modal drawn after the tag-like turn alone: one new fake-agent argv option that
  names the screen to draw after that turn, read from the same set. The test writes that screen into its own set, as
  the existing test does (`write_screen`), from the compiled literals; no committed fixture and no new committed
  screen phase.
- After the tag-like turn Run B's `box_wait` ends on a quiet screen that holds any compiled literal, a modal
  included (`typed.rs:505`, its unit case at `:788-800`), so the new case does not wait out a deadline.
- The fake agent's argv options stand at seven on architecture's registry line (arch history,
  2026-10-06-local-command-and-paste-framing-rows); this makes eight. `--local-command-mode` is a mode built earlier
  and is on test-plan §7's list, not on that line.

### M10 — the settle count is ten
- The key file: "Since chunk 2026-10-06-local-command-and-paste-framing-rows Run B's three added turns make it ten
  300 ms waits" (`.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md`).
- Re-derived from the call sites of `src/cmd/verify/typed.rs`: Run A one (`:76`); Run B five (`:105`, `:246`,
  `:276` twice, `:291`); Run C two (`:136`, `:155`); Run D two (`:179`, `:185`).
- The two stale comments: `.config/nextest.toml:19` and `tests/support/verify.rs:390`, each "seven 300 ms settles".

### M11 — the host tonight
- Sibling build dirs on the same volume: `andromeda-pulse`, `conductor`, `escher` (`ls -d ~/dev/projects/*/target`).
  Load average at 22:46Z: 0.46, 3.34, 14.68 — a heavy build ended minutes before.
- The stalled-start shape is on record with its cause (`evidence/block-reds-host-contention.md` of the prior chunk):
  18 s to 19 s at the start of every test that spawns a process, during another project's link.

### M12 — the rewritten-path warning has no case that reads it
- `warn_if_rewritten` writes `warning: argument looks like a Git Bash rewritten path` to stderr from the two
  clap value parsers (`src/cmd/send.rs:40-48`, `:53-59`).
- `grep -rn "Git Bash rewritten\|warn_if_rewritten\|looks_rewritten" --include=*.rs src tests crates`: the
  product lines in `src/cmd/send.rs` and one unit case of the predicate (`:242`); 0 hits under `tests/`.
  `send_leading_slash_argument_is_a_usage_error` asserts exit 2 and an empty stdout, on arguments this host does
  not rewrite.
- `verification-matrix.json#v1-29`'s acceptance ends "a rewritten path draws a warning". The predicate is
  tested; the drawn line is not. Found at P5's premise check of the cap's existing witnesses.

## Files inspected
- `src/run/send.rs` (full) — the slot, the claim / settle pair, the window, the tap, the outcome writers, the unit
  table `send_refusal_order`
- `src/cmd/send.rs` (full) — the client: `Out`, `deliver`, the exit table
- `src/human.rs` (full) — the mirror writers and their tests
- `src/run/gate.rs` (full) — the feed, `Gate::wait_ready`, the verified-gate unit cases
- `crates/viola-agent-claude/src/screen.rs` (1-180) — the constants, `SIGNATURES`, `Screen::verdict`
- `crates/viola-agent-claude/src/ledger.rs` (125-175) — `PostCondition`, `LOCAL_COMMANDS`, `FRAMING_TURNS`
- `crates/viola-agent-claude/src/hook.rs` (140-160) — the `session-start` data
- `src/cmd/run.rs` (56-130, 196-231) — `Methods`, the `hook.event` re-validation, the start order
- `src/cmd/client.rs` (34-112) — `Reply`, `answer_of`
- `src/cmd/verify/typed.rs` (125-310, 590-632) — Runs C and D, `framing_turns`, `settle`, `box_wait`
- `src/bin/viola-fake-agent.rs` (26-118, 120-177, 400-520, 755-775) — the options, the screens, `submit`
- `tests/cli_send.rs` (1-130, 300-545) — the helpers and the cases this chunk changes or copies
- `tests/cli_verify.rs` (960-1010) — the modal case and the paste-hint case
- `tests/support/home.rs` (245-275, 333-390) — `StampedHome`, `Wrapper::boot`
- `schemas/diag-line.v1.json` (22-36, 56-65, 113-133) — the send lines
- `.config/nextest.toml` (1-56) — the kills
- `.andromeda/architecture.md` (48, 49) — [Screen Model] and [Delivery Confirmation], each read whole
- the prior chunk's `evidence/rehearsal-shapes.md`, `evidence/paste-guard-control.md`,
  `evidence/block-reds-host-contention.md`

## Graph impact (from the code-graph query; rust plane, trace `tree-query-2026-10-06-local-command-send-outcomes.json`)
- **`SendSlot::new`** (`src/run/send.rs`) — 17 call sites: `start` @ `src/cmd/run.rs:216`, `methods` @
  `src/cmd/run.rs:637`, and 15 in the unit tests of `src/run/send.rs` (the grep of M2). The graph's 23 rows for a
  `new` defined in that file also hold the test helper `Pastes::new`; every row is in those two files. A new
  parameter stays inside them.
- **`append_hook_event`** — 16 rows: `Methods::dispatch` @ `src/cmd/run.rs:117`, the other 15 unit tests in
  `src/run/send.rs`. Its signature need not change: the slot it already takes carries the new state.
- **`send`** (`src/run/send.rs`) — 15 rows: `Methods::dispatch_call` @ `src/cmd/run.rs:125`, the other 14 unit
  tests in the same file. Its signature need not change.
- **`read_back`** — 1 caller, `deliver` @ `src/cmd/send.rs:200`. **`write_read_back`** — `Out::read_back` @
  `src/cmd/send.rs:119` and three unit tests in `src/human.rs`. **`mirror_line`** — the three writers in
  `src/human.rs`. A fourth writer and a fourth `Out` arm add callers and change none.
- **`exit_of` / `reason_of`** — `src/cmd/send.rs` and `src/cmd/answer.rs`; untouched (an `ok` exits 0).

## Patterns detected
- **Claim under the lock, append, then settle** (`src/run/send.rs:81-102`, `:146-171`): the tap marks the in-flight
  send claimed before the line is appended and settles it with the appended line's `ts`, so the confirming line
  always precedes the outcome record. A post-condition confirmation is the same three steps on another kind.
- **One outcome writer per outcome** (`src/run/send.rs:325-389`): each appends one record and logs one line, both
  keyed on the send's cursor. Every send ends in exactly one of them.
- **The window is clock-driven** (`src/run/send.rs:107-131`): `confirm` reads the injected `Clock`, and the unit
  tests expire it with `JumpClock` (`:416-424`) with no real wait.
- **A fake-agent hold is a capped argv option** (`src/bin/viola-fake-agent.rs:30-33`, `:99-108`), parsed in
  `Opts::parse` and unit-tested beside it (`:834-891`).
- **A guard's control is two readings of one command** (the prior chunk's `evidence/paste-guard-control.md`): the
  guard's line re-read before each edit, red with it off, green with it back.

## Conventions to follow
- **Human lines only from `src/human.rs`**, one `write_all` per line, no print macro, no `#[allow]`
  (`src/human.rs:1-3`, `:43-62`); the caller picks the stream (`src/cmd/send.rs:106-143`).
- **Oracles are literals in the test**: the command texts, the `detail` strings and the exit codes are written out
  in `tests/cli_send.rs`, never imported from the product (`tests/cli_send.rs:373-386`).
- **Waits key on `events.ndjson`**, through `wait_events` and `records_after` (`tests/cli_send.rs:48-72`), bounded
  by `WITHIN`.
- **Send lines go through `obs_event!`** with `corr`, `rpc_id`, `conn`, `srv_conn` passed as typed fields
  (`src/run/send.rs:334-345`); no field outside obs-plan §6's catalog.
- **A test that waits out the product's window carries `send_window_`** (`.config/nextest.toml:48-50`), and the
  same verdict has a clock-driven unit case.

## New files to create
- `viola-0.1.0/chunks/2026-10-06-local-command-send-outcomes/evidence/` — the two controls' readings and the forced-window case's readings

## Files to modify
- `src/run/send.rs` — the slot's verified bit and last session id, the local-command decision, the post-condition claim, the unconfirmable outcome, their unit cases
- `src/cmd/run.rs` — the verified bit passed to the slot at both construction sites
- `src/cmd/send.rs` — the client's arm for an unconfirmable reply; the warning's calls neutralised and restored for its control
- `src/human.rs` — the fourth mirror writer and its cases
- `tests/cli_send.rs` — the rewritten local-command case, the new outcome cases, the forced-window case, the warning case
- `src/bin/viola-fake-agent.rs` — the argv option that names the screen drawn after the tag-like turn
- `tests/cli_fake_agent.rs` — that option's case
- `tests/cli_verify.rs` — the control's case for the guard before the local-command paste
- `src/cmd/verify/typed.rs` — the guard neutralised for the control's red reading and restored; no net change
- `.config/nextest.toml` — the comment's settle count
- `tests/support/verify.rs` — the comment's settle count

## Open questions
- none — the one question P3 carried (what this chunk does about a `send` issued while the paste hint stands) was
  answered at the P4 card: option B (inputs#I2). The gate's two files leave the list.
