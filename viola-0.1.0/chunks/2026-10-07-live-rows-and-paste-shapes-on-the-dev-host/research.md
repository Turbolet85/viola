# Codebase Research — 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, 8 additions; applied: the
  2026-09-25 "never pipe `boot`", the 2026-10-07 stalled-start rule (a stalled start is a finding about the backing,
  reported, never re-run for green), and the fake agent's `--paste-hint-ms` cap of 8 s. `.claude/rules/testing.md` —
  read whole, 31 additions; applied: 2026-10-06 "a live shape probe's record states, for every settle, how long it
  took and what it settled on", 2026-10-05 `vhome` (never a `home` path component under `verify --record`),
  2026-09-27 "force a timing window open with a hold, never sample the race", 2026-10-06 "size a new stamped-home
  case against the 10 s mutants kill", 2026-10-04 "no mutation entry in a `[[gate]]` block", and "never hand-write
  `ledger/stamps.json`".
- **Platform issues consulted:** none — no runner-only bullet, and the CI run read at take-up is green.
- **External inputs:** `inputs#I1` — the operator's directive at this take-up: 2.1.287 by path, a live cap of 10 for
  the whole chunk, R-L4's card before any gate change, a widening probe path goes to P4 as a priced founder card,
  one builder window, a split is the founder's word. `inputs#I2` — the route-adaptation relay: the founder's rulings
  R-L2, R-L3 and R-L4 (live, 2026-10-07T05:43Z) and the partition that made this entry. `inputs#I3` — the P4 card's
  answers (snapped at P4): one synthetic peer message into the scratch session and the scratch hook's names-only
  list (the founder, live, 2026-10-07T09:43Z, relayed by the overseer); measure here, rows next, no entry minted
  (the founder); the hint home a byte copy of the standing stamp home (the overseer). Not snapshotted, with the
  reason: the driver's own log of this session (M2) holds prompt text, which may not enter this repository; it is
  read by counts only and the count is re-run at implement.

## Measured facts

### M1 — the versions, and how a live round reaches 2.1.287
- `claude` on PATH is `~/.local/share/mise/installs/claude/latest/claude` and answers `2.1.289 (Claude Code)`;
  `~/.local/share/mise/installs/claude/2.1.287/claude --version` answers `2.1.287 (Claude Code)` (both read at this
  take-up). The install dir holds 2.1.287, 2.1.288 and 2.1.289 (`ls`).
- `viola verify` and `viola run` both take the program after `--` (`src/cmd/run.rs:35-42`; architecture [CLI Version
  Compatibility]); `run --local-live` passes none (`crates/viola-e2e/src/harness/run.rs:379-385`), so it reaches PATH
  `claude` and is not this chunk's channel.
- One stamped set: `tests/contract_ledger_probes.rs:36` (`STAMPED: ["2.1.287"]`), `:39`
  (`DRIFT_ONLY: ["2.1.283", "2.1.288"]`).
- This session's tool environment carries `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (names read with `env | cut
  -d= -f1`), set by the bridge that wraps this builder, which is another tree's build. The product reads
  `VIOLA_NAME` as `from` (`src/cmd/client.rs:106`) and as the instance for its process log (`src/cmd/mod.rs:117`).
  So every product command a live round starts from this session runs with those three names removed and `--home`
  given, or it would file itself under the bridge's instance.

### M2 — harness prompts at the hook layer, read from the driver's own log (counts only)
- Method: a scratchpad script over `~/.viola/sessions/viola-builder/events.ndjson` (the bridge's log of this builder
  session, outside this repository), run at 2026-10-07T09:03:10Z and 09:03:22Z. It classifies each record's `prompt`
  by `str.startswith` against seven literals and prints counts and key names only; no prompt text is printed or
  kept.
- Reading: 136 `UserPromptSubmit` records. 75 start with `<agent-message from=`. **28 start with
  `<task-notification>`.** 33 start with none of the seven. 0 start with `<cross-session-message`, 0 with
  `<\cross-session-message`, 0 with the model-layer preface `Another Claude session sent a message`, 0 with
  `[SYSTEM NOTIFICATION`, 0 with `<system-reminder`. 0 hold `<task-notification>` anywhere but the start.
- What it settles: the `<task-notification>` form reaches UserPromptSubmit starting at the tag, unescaped, with no
  preface, as the compiled prefix expects (`crates/viola-agent-claude/src/hook.rs:105-110`, `:178`). The CARRY's
  "unmeasured at the hook layer: the `<task-notification>` form" is closed by this reading for the CLI this session
  runs.
- What it does not settle: the version. The log's records carry no CLI version; this session was started through
  PATH `claude`, whose `latest` link has pointed at 2.1.289 since 2026-10-06 (the link's date). It is not a 2.1.287
  reading. And it is the CLI's own injection, not a typed tag: whether a human who types `<task-notification>` at the
  very start of a prompt gets it escaped is still unmeasured.
- No cross-session message has reached this session through the hook in the log's life: the cross-session form
  stays unmeasured here.

### M3 — what the CLI does with a cross-session message, read statically from the 2.1.287 binary
- `/usr/bin/grep -a -c 'cross-session-message' <the 2.1.287 binary>` reads 10 lines. The surrounding text states
  that incoming peer messages "arrive as user-role messages wrapped in `<cross-session-message from="...">`", and
  the binary's own readers match `^<cross-session-message\b[^>]*>\n?` and strip a closing
  `\n</cross-session-message>`. Attributes read by those matchers: `from`, `from-name`.
- So at the model and transcript layer the tag is unescaped. What the UserPromptSubmit `prompt` holds (the relayed
  measurement says escaped, `<\cross-session-message`) cannot be read statically: the hook payload is built
  elsewhere. Both compiled forms stay as they are until a live reading.
- The same binary says worker results arrive "containing `<task-notification>` XML ... normally inside a
  `<system-reminder>`": the model-layer frame. M2 shows the hook layer has no such frame.

### M4 — a wrapped child and the peer listing (the premise behind widening 1)
- `Run::spawn` passes `env_remove: inputs.strip.remove` (`src/cmd/verify/typed.rs:544-551`), and `viola run` plans
  the same strip (`plan_strip @ src/cmd/run.rs:207`). Both messaging names are floor names
  (`crates/viola-agent-claude/src/lib.rs:14-26`). So a child never sees its parent session's socket or token.
  Verified.
- The CARRY's inference from that, "so it cannot be addressed by another session", is not established by the code:
  the strip removes the parent's identity from the child's environment, and says nothing about whether the child
  registers itself with the CLI's peer listing.
- Observed during this phase (the session's own peer listing, PATH `claude`): nine peer sessions are listed, three of
  them builders that are each a child of the other tree's bridge. That bridge's strip is not this product's and was
  not read. So a wrapped interactive session can be listed and addressed on this host; whether a 2.1.287 child
  started under viola's strip is listed is unmeasured, and the first scratch session can read it at no extra start.
- Consequence for the probe-path card: the cross-session measurement needs no change to the strip, to `viola verify`
  or to any product code. What crosses is one message, sent by a second Claude session, into a probed session.

### M5 — the identity floor: what can see a child's names today
- The floor is 11 names (`lib.rs:14-26`); its only readers are `plan_strip_with @ lib.rs:102` and the crate's tests.
- The capture arm takes the event, the dir and stdin and nothing else (`src/cmd/hook.rs:94-95`, `:359-372`): it
  reads no environment. Verified.
- One names-only path is already ruled: `log_child_start` writes `env_stripped_known` and `env_kept` on
  `process-start{subject:"claude-child"}` (`src/run/mod.rs:39-60`; the only site, `grep -rn 'env_stripped_known'
  src crates`: 1 file). It lists the `CLAUDE*` names `viola run` removed from **its own** inherited environment. A
  `viola run` this builder starts therefore records the names this session exports (PATH `claude`), names only, at
  no widening. It records a 2.1.287 session's names only when `viola run` is started from inside a 2.1.287 session,
  which no planned step does.
- Read here by count: this session exports 10 `CLAUDE*` names (`env | cut -d= -f1 | grep -c '^CLAUDE'`), the count
  the prior chunk's research M7 read by name on 2.1.289.
- A 2.1.287 reading needs something running inside a 2.1.287 session to list names: a hook's own environment (what
  that CLI hands its hooks) or a tool's (what it hands a command the model runs). The two may differ; the floor's
  Windows origin measured a wrapped host's inherited set.

### M6 — the three paste shapes against the compiled unwrap
- `unwrap_pastes` (`hook.rs:199-219`) and `paste_pair` (`:222-228`): a pair is `<pasted_content id="X">`, a
  newline, the inner text, a newline, `</pasted_content id="X">`. The two newlines directly before the open tag and
  the one directly after the close go with it.
- The compiled expectation for shape 1 (typed text beside a paste) and shape 2 (two pastes) already stands as unit
  cases written from the one-paste measurement, not from a measurement of those shapes:
  `text_then_framed_pair` (`hook.rs:556-559`: `note:` + the framed pair gives `note:A long paste`) and
  `two_framed_pairs` (`:560-563`). If the CLI frames a second pair or a pair after typed text differently, those
  two cases encode a guess.
- Shape 3, re-derived at HEAD: with a pasted text `T\n`, if the CLI adds no newline before the close tag, the raw
  prompt is `…">\nT\n</pasted_content id="X">…`, `paste_pair` returns `T`, and delivery matching compares `T` with
  the sent `T\n` exactly (architecture [Delivery Confirmation]): no claim, and the send ends `not-delivered` /
  `no-prompt-submitted` when the 10 s window expires (`src/run/send.rs:349-351`). The code half is verified; the
  CLI half ("adds no newline") is the freight's static reading and stays the hypothesis the live paste tests.
- No unit case holds a pasted text that itself ends in a newline (`/usr/bin/grep -c 'case::'
  crates/viola-agent-claude/src/hook.rs`: 44 labelled cases; the five whose label names a newline, `hook.rs:511`,
  `:544`, `:548`, `:552`, `:564`, are about the frame's newlines, read one by one).
- Static read of the 2.1.287 binary for the wrapper: the open form is `<pasted_content id="${t}">` + newline, the
  close `</pasted_content id="${t}">`, the id 4 characters (`wKt=4`). What sits between the text and the close is
  not readable from the matched fragments.
- Callers (graph): `unwrap_pastes` ← `prompt_text @ hook.rs:189` alone; `prompt_text` ← `data_of @ hook.rs:158`,
  `pasted @ crates/viola-agent-claude/src/ledger.rs:678`, and three tests (`hook.rs:528`, `:569`, `:709`). A change
  to the unwrap reaches production through `normalise` → `prompt-submitted`'s `text` → the claim, and through the
  two paste rows' check.

### M7 — the hint window: the gate, the hint, and `wait`
- The gate (`crates/viola-agent-claude/src/screen.rs:122-152`): not quiet for 300 ms keeps waiting up to 5 s; a
  quiet screen with signatures is `Ready` only when a row holds `for agents` and none holds a modal literal; **with
  no signatures** (`sigs` `None`, an unverified CLI) a quiet screen is `Ready` with no row read. `send` refuses
  `input-not-ready` on anything but `Ready` (`src/run/send.rs:338-341`).
- So the same long-paste sequence has two readings on the real CLI: on a verified home the fake-agent result
  predicts `input-not-ready`; on an unstamped home the partial gate reads `Ready` and the second text **is typed
  while the hint stands**. The footer text during the window is `paste again to expand` (the prior chunk's
  `evidence/rehearsal-shapes.md:99`), so what a second paste does there (a new prompt, or an expansion of the first
  paste) is exactly the fact the card's "type under the hint" option needs, and it is unmeasured.
- A verified home needs a stamp whose seventeen rows pass for 2.1.287. One stands on disk, written by `viola
  verify` at the prior chunk's green record round: `target/e2e-home.disk/viola-record-20261006T211429Z/vhome/
  ledger/stamps.json` reads `2.1.287` 17 of 17 `pass` (read with a one-line python count). That directory is on the
  handoff's operator desk as the operator's to delete. Without it a verified home costs one full by-path verify,
  five `claude` starts.
- The first timing: the hint is a timer from the paste, 8.000 s between two footer draws, with the turn's Stop 1.5 s
  in (`rehearsal-shapes.md:102`, `:112`). A `send` placed before `turn-ended` reads `turn-running`, not the gate
  (`send.rs:324-327`), so the live `send` goes after the `turn-ended` record.
- `input-not-ready` has a second cause, a poisoned screen model (`screen.rs:128-130`), which writes
  `parse-rejected{parser:"vt100-feed"}` first. A live refusal is attributed to the hint only with no such line
  before it.
- `wait`, re-derived at HEAD (`src/run/wait.rs:135-170`; `src/cmd/wait.rs:1-4`): with no `--after` the scan starts
  at a held dialog's line, else **at the log's end at the call**, and only `turn-ended`, `question`, `permission`,
  `plan` and `session-end` wake it. In the hint window the turn's `turn-ended` is already logged, and the refused
  send's `send-refused` is a wrapper line that wakes nothing. So a `viola wait <name>` with no `--after` and no
  `--timeout-ms` parks with no deadline. With `--after` at a cursor before the `turn-ended` it returns that record
  at once, and a driver that then sends again is refused again while the hint stands. The freight's hypothesis is
  re-established by reading; the live run is its test.
- The hint string is `{name} was not ready for input; viola wait {name}, then send again` (`src/human.rs:215-217`).
- `GATE_MAX_WAIT` is referenced at `ledger.rs:16`, `:760`, `:761`, `screen.rs:132`, `:184` and
  `src/cmd/verify/typed.rs:28`, `:487` (graph `refs`): the sites a "longer bound" option would reach, with the
  `quiet-period` row's validation among them.
- The fake agent's hold is capped at 8 000 ms (`src/bin/viola-fake-agent.rs:32`, `:105`), equal to the first
  reading: a longer second timing cannot be replayed through the hold unchanged.

### M8 — what a kill leaves, and what stands at the root now
- `TrustedDir` is a drop guard (`src/cmd/verify/typed.rs:334-351`); `create` first removes a dir of the same pid
  name. No signal handling exists in `src/cmd/verify.rs`, `src/cmd/verify/typed.rs` or `src/main.rs`
  (`grep -n 'signal\|ctrlc\|SIGTERM'`: 0 hits). A kill runs no destructor, so the freight's claim holds: a killed
  verify leaves the dir of the run it was in.
- A leftover's name carries the pid alone, with no start time, so the repository's owner-record sweep (pid + start
  time, `tests/support/home.rs`) cannot be applied to it as it is.
- Standing at 2026-10-07T08:52:43Z: ten dirs dated 2026-10-06T22:12:45Z to 22:12:50Z, four `-dialogs` (empty) and
  six `-plan` (one entry each), beside the operator desk's `.viola-verify-2095228/`. None is a bare
  `.viola-verify-<pid>/`, which fits kills that landed during Run C or Run D. What killed them is not established.

### M9 — the sites a landed row moves
- Row-count literals at seventeen (`/usr/bin/grep -c -E '/17\]|17 pass|; 17\]|seventeen'` per file):
  `tests/cli_verify.rs` 40, `crates/viola-e2e/src/harness/run.rs` 22, `src/cmd/verify.rs` 7,
  `crates/viola-agent-claude/src/ledger.rs` 3, `tests/contract_ledger_probes.rs` 2, `src/run/version_gate.rs` 1:
  75 lines in six files.
- The step-line denominator is derived, not a literal: `LedgerRow::ALL.len()` (`src/cmd/verify.rs:355`).
- A row is a variant, an id, a phrase and one arm of `check(row, &Probes)` (`ledger.rs:20-110`, `:740-770`);
  `Probes` holds `print`, `typed`, `trusted`, `dialogs` (`:542-547`). A row whose evidence is neither a Run B
  capture nor a dialog capture needs a new `Probes` field, a recorded variant, a fake-agent replay and the drift
  contract's replay of it.
- The stale `--local-live` list sits in the test-plan key file,
  `.andromeda/registries/contracts/test-plan/5-command-implementation.md:53` (six spine rows and eight more named).

### M10 — the live-session price of each piece
One `viola verify` is five `claude` starts; a scratch session is one; a `viola run` is one.

| piece | starts | what it rides |
|---|---|---|
| three paste shapes, and the typed `<task-notification>` at a prompt's start | 1 | one scratch session in step 0's form, four pastes |
| the cross-session prompt | 0 | the same scratch session, if a second session may send one message into it |
| a 2.1.287 hook's `CLAUDE*` names | 0 | the same scratch session, if its scratch hook script may list names |
| the hint: timing, a `send` and a `wait` inside it, on a verified home | 1 | one `viola run` on the standing stamp; 5 more if that stamp is gone |
| the hint: what a paste does under it, on an unstamped home | 1 | one `viola run` |
| a landed ledger row (either one, or both) | 5 | one by-path `verify --record`; 5 more on a red round |

- Measure only: 3 starts, 7 spare. With rows landing in the same chunk: 8, and one red record round passes the cap.
- The scratch driver of the prior chunk is described in its `evidence/step0-shapes.md` ("How it ran") and
  `evidence/rehearsal-shapes.md` (the wait rule). Copies stand today in three other sessions' scratchpads under the
  host temp dir (`find`: `step0` twice, `rehearsal` once); the temp dir is aged and does not survive a reboot, so the
  plan cannot depend on them.

### M11 — sizing against one builder window
- Anchors (inputs#I2 R-L2): the last two chunks each ended implement near half the window (51.7 % and 48.9 %); the
  three-row chunk rode an existing run and still took two record rounds and three revisions.
- Measure-only work: one scratch driver (rebuilt or reused), three live sessions, the evidence, unit cases over the
  measured shapes, an unwrap fix only if a shape falsifies it, the hint card. No row-count move, no fixture, no
  fake-agent mode.
- A landed row adds, per M9: a probe with a post-condition, a `Probes` field, a recorded variant, a fake-agent
  replay, 75 literal lines, the harness literals, a record round. The cross-session row adds a second session to
  the probe's shape, which no run has today.

## Files inspected
- `src/cmd/verify/typed.rs` (1-130, 196-400, 537-577) — the four runs, Run B's pastes, the probe dirs' guard, the
  spawn's strip.
- `crates/viola-agent-claude/src/hook.rs` (105-110, 160-244, 492-571) — the prefixes, `prompt_origin`, the unwrap,
  the case tables.
- `crates/viola-agent-claude/src/lib.rs` (1-112) — the floor and `plan_strip`.
- `crates/viola-agent-claude/src/screen.rs` (1-155) — the gate's verdict and constants.
- `crates/viola-agent-claude/src/ledger.rs` (20-65, grep index of `Probes`, `check`, the probe texts) — the row
  shape.
- `src/run/send.rs` (296-351) — `send`'s rungs through the gate.
- `src/run/wait.rs` (96-185), `src/cmd/wait.rs` (1-30) — the start cursor and the wake loop.
- `src/run/mod.rs` (28-63) — the names-only child-start line.
- `src/cmd/hook.rs` (function index) — the capture arm's inputs.
- `src/cmd/run.rs` (35-57) — the run verb's arguments.
- `src/human.rs` (209-225) — `send_hint`.
- `crates/viola-e2e/src/harness/run.rs` (300-424) — `LEDGER_ROWS` and `local_live`.
- `tests/contract_ledger_probes.rs` (grep) — the two lists.
- The prior chunks' records: `2026-10-06-local-command-and-paste-framing-rows/research.md` (26-195),
  `evidence/step0-shapes.md` (1-60), `evidence/rehearsal-shapes.md` (1-70 and grep), `evidence/
  block-reds-host-contention.md` (grep), `inputs/I2-relay-2.md.txt` (whole);
  `2026-10-06-local-command-send-outcomes/evidence/paste-hint-send.md` (whole).
- `.andromeda/architecture.md` (lines 69-91), `.andromeda/registries/contracts/test-plan/
  5-command-implementation.md` (line 53), `.andromeda/playbook.md` (30-58).
- The 2.1.287 binary, statically (`/usr/bin/grep -a` for the cross-session, task-notification and paste-wrapper
  fragments).

## Graph impact (from the code-graph query; rust plane, trace `tree-query-2026-10-07-live-rows-and-paste-shapes-on-the-dev-host.json`)
- **`prompt_text`** — 6 callers: `data_of @ crates/viola-agent-claude/src/hook.rs:158`, `pasted @
  crates/viola-agent-claude/src/ledger.rs:678`, three tests at `hook.rs:528`, `:569`, `:709`, and the `ledger`
  module's import at `ledger.rs:15`. An unwrap change moves the paste rows' check with it.
- **`unwrap_pastes`** — one caller, `prompt_text @ hook.rs:189`. **`prompt_origin`** — one caller, `data_of @
  hook.rs:158`.
- **`plan_strip`** — two production callers, `start @ src/cmd/run.rs:207` and `measure @ src/cmd/verify.rs:145`,
  and two tests at `lib.rs:344`, `:351`. Nothing here changes it.
- **`framing_variants`** — `record @ src/cmd/verify.rs:405` and one test: the path a recorded paste variant takes.
- **`local_live`** ← `run_with @ crates/viola-e2e/src/harness/run.rs:151`; **`live_rows_pass`** ← `local_live @
  :387`; **`LEDGER_ROWS`** — one reference, `:405`.
- **`TrustedDir`** — created at `src/cmd/verify/typed.rs:93`, `:130`, `:169`; defined `:337`, dropped `:347`; two
  test sites `:862`, `:869`. A leftover-removal change stays in this one file.
- **`IDENTITY_FLOOR`** — `lib.rs:102` and its tests `:361-363`. **`HARNESS_PREFIXES`** — `hook.rs:178` alone.

## Patterns detected
- **A live shape probe is a scratch driver outside the repository** (`evidence/step0-shapes.md:8-26` of the prior
  chunk): a Python PTY driver at 80×24, a scratch plugin whose spine hooks run a scratch script in exec form, the
  2.1.287 binary by path with `--model haiku`, the inherited `CLAUDE*` names removed, cwd a 0700
  `<repo root>/.viola-verify-<pid>/` removed by the driver, each paste one bracketed write into a settled input box
  after the previous Stop, the end Ctrl-C twice.
- **Screens and normalisation are read through the tree's own code** by a scratch helper built against
  `crates/viola-agent-claude` by path (the same record, lines 21-25), so a reading and the product never use two
  parsers.
- **A measured shape lands as a labelled `#[case]` with a literal oracle** (`hook.rs:535-571`,
  `prompt_text_drops_the_cli_framing_around_a_pair`).
- **A timing window is forced open on the fake agent, never sampled** (`tests/cli_send.rs`,
  `send_under_the_paste_hint_on_a_verified_cli`): the fake-agent pair is what a corrected live reading changes.
- **A removal takes only what its maker created, by an exact identity** (`tests/support/home.rs` `remove_owned`,
  the owner record).

## Conventions to follow
- **Claude shapes stay in `viola-agent-claude`** (`hook.rs` is the only file naming a tag; `lib.rs` the floor).
- **Test oracles are literals**, never the product's own constants (`.claude/rules/testing.md:40`).
- **Names only**: no `CLAUDE*` value in any log, fixture or evidence file (`src/run/mod.rs:37-38`).
- **A fixture comes only from the recorder** (`viola verify --record`), never from a scratch capture; a scratch
  reading is recorded in the chunk's `evidence/`.
- **No fake-agent shape before a recorded fixture**: a live reading that has no fixture changes no fake-agent mode.
- **Editor lines**: the graph's `line` is 0-indexed; every coordinate above is `line + 1`.

## New files to create
- `viola-0.1.0/chunks/2026-10-07-live-rows-and-paste-shapes-on-the-dev-host/evidence/` — the live-session ledger, the shape record, the hint record and its card, the probe-dir census

## Files to modify
- `crates/viola-agent-claude/src/hook.rs` — labelled cases over the measured shapes, the doc comment on `prompt_origin`, and the unwrap only where a measured shape falsifies it
- `crates/viola-agent-claude/proptest-regressions/hook.txt` — only if the wrapped-paste property records a failing seed

## Open questions
- none — the three plan decisions research raised were answered at P4 (inputs#I3): the cross-session prompt is
  measured by one synthetic peer message from the builder session into the scratch 2.1.287 session, and the names
  by the scratch hook's names-only list (both the founder's, live, each after the widening was shown); the chunk
  measures and no ledger row lands in it (the founder's), so the files a landed row writes stay off the two lists;
  the hint run's verified home is a byte copy of the standing stamp home (the overseer's).
