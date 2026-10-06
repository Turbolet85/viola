# Codebase Research — 2026-10-06-local-command-and-paste-framing-rows

## Scope
- **Depth:** deep · **Reads:** 16 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, 9 additions; applied: the
  2026-09-25 "never pipe `boot`" and the 2026-09-27 `binary(<stem>)` selector. `.claude/rules/testing.md` — Session
  Additions read whole (lines 50-80), 29 additions; applied: 2026-10-05 `vhome` (never a `home` path component under
  `verify --record`), 2026-10-04 "no mutation entry in a `[[gate]]` block", 2026-09-28 "a timing red is never fixed by
  a raised bound".
- **Platform issues consulted:** none — no runner-only bullet, and the CI run read at take-up is green.
- **External inputs:** `inputs#I1` — the operator's directive: the installed `claude` is 2.1.289 (2.1.288 and 2.1.287
  stay installed), 3 of the founder's 16 live sessions are left, a larger need goes to P4 as a founder card priced per
  row, one builder window, Epoch 3 unsplit. `inputs#I2` — the P4 card's answers (snapped at P4): record 2.1.287 only
  under a cap of 8; Run B takes three compiled pastes (the founder, live); the harness-prefix and identity-floor rows
  re-home to "First live test and self-drive"; two chunks (the founder, live); and a relayed hook-layer measurement of
  14 hand-backs on 2.1.289. `inputs#I4` — the direction on step 0's STOP 3 (the overseer, founder-delegated,
  2026-10-06 ~20:05Z, snapped at the revision): fold the fix here; the unwrap consumes the CLI's framing newlines
  around a pair, bounded to what step 0 measured; a fake-agent case replays the recorded wrapped shape through
  `send`; the send-strand guard narrowed only as far as that needs; the `hook.rs:171` comment corrected; step 0 not
  re-run; the cap stays 8 with 1 used. `inputs#I6` — the founder's answer on the red record round (live,
  2026-10-06T20:49Z, relayed by the overseer, snapped at the second revision): Run B waits for the input-box literal
  after each added turn, bounded by the probe deadline, the guard kept; the cap rises to 12; one rehearsal session
  first, then one second record round of 5; a second red round returns to the founder; the product readiness gate
  and its 5 s maximum stay untouched, that reading the wrap routes.

## Measured facts

### M1 — the version the typed probes hit
- `viola verify` runs the program named after `--`, and `claude` from PATH when none is named
  (`src/cmd/verify.rs:103-106`). It stamps the version that binary's own `--version` answers
  (`src/cmd/verify.rs:147-163`, `parse_version`), so the stamp always names the binary that was probed.
- Measured on this host, 2026-10-06 ~19:20Z: `claude` on PATH is `~/.local/share/mise/installs/claude/latest/claude`,
  a link to `…/2.1.289/claude`; `claude --version` prints `2.1.289 (Claude Code)`; `mise ls` lists 2.1.287, 2.1.288
  and 2.1.289.
- So a bare `viola verify`, and `agent-run.sh run --local-live` (which uses the program default,
  `crates/viola-e2e/src/harness/run.rs:350-351`), hit **2.1.289**. The two earlier chunks reached 2.1.288 and 2.1.287
  only by naming each binary after `--` (`2026-10-05-dialog-rows-and-re-probe/plan.md:277`, `:288`).

### M2 — what the stamped rows read once the row count grows
- `ledger::verified` is true only when every row of `LedgerRow::ALL` reads `"pass"` under the version
  (`crates/viola-agent-claude/src/ledger.rs:799`). A stamp written at 14 rows lacks any new row, so it reads
  **unverified** from the first build that carries a 15th row. No code path keeps a short stamp verified, so the
  retired R2 gap does not reopen.
- `stamps_verdict` reports a short stamp as plain `(false, None)`: no `parse-rejected` line, no new detail
  (`src/run/version_gate.rs:162-175`). The obs schema does not move for this.
- What goes unverified with it: the full readiness gate (signatures are passed only when `cli_verified`), every
  non-`null` dialog decision (`src/run/dialog.rs:192`, `:339`), and local-command confirmation once it lands.
- **2.1.288 and 2.1.287**: their committed sets (`fixtures/claude/2.1.288/`, `2.1.287/`; 19 files in 2.1.288) hold the
  spine, three screens and twelve dialog variants. They hold no long-paste, tag-escaped or `clear` shape, so the fake
  agent cannot replay a new row from them. `tests/contract_ledger_probes.rs:33` lists both as `STAMPED` and asserts
  `14 pass  0 fail` (`:98`). After the growth each one either gets its new shapes recorded live (5 sessions each,
  M4) or moves to the `DRIFT_ONLY` list beside 2.1.283.
- **2.1.289**: no committed set, no stamp anywhere. The dev host has no `~/.viola/ledger/` at all (the earlier live
  stamps went into per-run record homes under `target/e2e-home/`). A `viola run` on PATH `claude` reads
  `cli_verified:false` today and stays so until one full verify of 2.1.289.
- The default version of the test fleet is 2.1.287 at four literal sites: `src/bin/viola-fake-agent.rs:19`,
  `crates/viola-e2e/src/harness/boot.rs:22`, `tests/support/fake.rs:16`, and `stamped_home` through it. Every root
  test that boots through `stamped_home` fails at the fixture unless that default set replays every row.

### M3 — the new rows ride Run B; no row needs a session of its own
- Run B (`src/cmd/verify/typed.rs:85-117`) pastes one compiled `PROBE_PROMPT` and waits for its UserPromptSubmit and
  Stop captures. Run C already pastes three prompts in one session, each after the previous Stop (`:122-160`), so
  several pastes in one run is a built pattern.
- Run B's plugin captures the four spine events (`CAPTURE_EVENTS`, `ledger.rs:292`): SessionStart, UserPromptSubmit,
  Stop, SessionEnd. Those are exactly what the three probe-able rows read:
  - **long-paste wrapper** — paste a compiled text past the wrap threshold; post-condition: the captured `prompt`,
    after the compiled unwrap, equals the pasted text;
  - **tag escaping** — paste a compiled text holding tag-like literals; post-condition: the captured `prompt`, after
    the compiled un-escape, equals the pasted text;
  - **local command `/clear`** — paste `/clear`; post-condition: a SessionStart capture with `source` `clear` and a
    `session_id` different from the run's first one, and no UserPromptSubmit capture for it.
- At HEAD `Probes` holds Run B's timings and screens only (`TypedRun`, `ledger.rs:451-459`); its captures are read for
  timing and dropped. The rows need Run B's captures carried in `Probes`.
- The normalisation the two paste rows check is already compiled: harness prefix on the raw start, then
  `unwrap_pastes`, then `unescape_tags` (`crates/viola-agent-claude/src/hook.rs:176-189`, `:195-235`). The rows add
  probes and checks, not normalisation code. [premise-corrected: step 0 measured the wrapped prompt with two
  newlines before the pair and one after, which `unwrap_pastes` keeps, so the long-paste check cannot pass on the
  compiled normalisation as it stands — M10; the fix is folded here by inputs#I4]
- `--record` writes the print probe's spine as `<Event>.default.json`, the dialog variants and the three screens
  (`src/cmd/verify.rs:386-433`). The new shapes are three more recorded payloads through the same scrub.
- The fake agent fires `UserPromptSubmit` from `UserPromptSubmit.default.json` with only `prompt` replaced by the
  typed text (`src/bin/viola-fake-agent.rs:425-445`), and fires SessionStart once at launch (`:737`). It emits no
  paste wrapper and no second SessionStart (grep `pasted_content` over the file: 0 hits). The tests extract's
  "paste-wrap form past the fixture's threshold" is test-plan target text, not built.

### M4 — the live-session price
- One `viola verify` is five `claude` starts: the print probe (`verify.rs:171`), then Runs A, B, C, D
  (`typed.rs:61-82`). `VerifyArgs` has two fields, `record` and `program` (`verify.rs:40-47`): there is no way to run a
  subset, and a subset that stamped would be the R2 gap again.
- Measured twice on 2026-10-05: 5 sessions per version, 10 for two (`2026-10-05-dialog-rows-and-re-probe/evidence/
  live-sessions.md`, rows 4-13).
- Rows riding Run B add pastes to an existing session, so the per-version price stays 5. `/clear` opens a second CLI
  session inside Run B's one process: one more transcript under `~/.claude/projects/`, the accepted residual class.
- A scratch shape probe before any code ("step 0") cost 1 session per shape in both earlier chunks, and each needed a
  re-run after a STOP (4 and 3 spent on step 0).
- Price per row, as the directive asks:

  | row | own sessions | rides |
  |---|---|---|
  | local command `/clear` | 0 | Run B, one more paste |
  | long-paste wrapper | 0 | Run B, one more paste |
  | tag escaping | 0 | Run B, one more paste |
  | harness prefix, cross-session half | no probe path (M6) | needs a second live session sending in, plus the messaging names let through R8 |
  | R8 identity floor | 0 | no probe path inside the capture arm's ruled shape (M7) |
  | shared: step 0 shape probe on 2.1.289 | 1 | one Run-B-shaped scratch session, all three pastes |
  | shared: one full verify per stamped version | 5 | any row count ≥ 1 costs the same |

- Totals: 2.1.289 alone = 1 + 5 = **6**; 2.1.289 and 2.1.288 = **11**; all three = **16**. Each STOP re-run adds 1
  (step 0) or 5 (a record round). The cap has 3 left, so the smallest honest total is 3 over it before any spare.
- What the 3 left can buy alone: step 0 and two spares. No stamp, so no recorded shape, so no green CI for a new row.

### M5 — `send` today, and what `unconfirmable` touches
- The wrapper's `send` settles on exactly two things: the in-flight text matching a `prompt-submitted` line, or the
  window expiring (`src/run/send.rs:108-131`, `:272-275`). A local command ends `not-delivered` /
  `no-prompt-submitted` after the full 10 s window.
- `append_hook_event` already sees every hook line, `session-start` included, and already ends the running turn on it
  (`send.rs:140-173`). The `/clear` post-condition has its hook point there; the line's `data` carries `cause` and
  `agent_session_id` (`hook.rs:148-155`).
- `SendSlot` holds no `cli_verified` (`send.rs:49-55`); `run` has it at `src/cmd/run.rs:209`. "Not measured on this
  CLI version" needs that bit threaded in.
- The outcome writers: `confirm` records `send-confirmed {cursor}` and logs `send-confirmed{confirmed:true}`
  (`send.rs:325-347`). `schemas/diag-line.v1.json:59-61` already admits `confirmed` as a boolean on the three
  `send-*` events, so `send-confirmed{confirmed:false}` passes G4 with no schema change.
- The human mirror: `MIRROR_WORD = 13` is already sized by `unconfirmable` (`src/human.rs:40-41`), with
  `write_send_open`, `write_read_back`, `write_send_unable`. No `unconfirmable` writer exists, and the client
  (`src/cmd/send.rs:113-127`) renders only `read_back` and `unable`.
- The pinned interim case is `tests/cli_send.rs:373-386`: `/clear` under `--local-command-mode` exits 13.
- The only product mention of the word at HEAD is the comment at `src/human.rs:40` (grep `unconfirmable` over `src`,
  `crates`, `tests`: 1 hit).
- The local-command list per architecture [Delivery Confirmation] is two commands: `/clear` (post-condition measured
  by the probe) and `/remote-control` (no post-condition, so `unconfirmable`). `/remote-control` is never typed by a
  probe: it would open a remote-control link on the founder's account.

### M6 — the harness-prefix row has no probe path here
- The four prefixes are compiled at `hook.rs:105-110` and matched on the raw start at `:176-182`.
- The literal `cross-session-message` lives in `crates/viola-agent-claude/src/hook.rs` alone among `.rs` files under
  `src`, `crates` and `tests` (`grep -rn cross-session-message --include=*.rs src crates tests`: 11 lines, one file). No standing script or test gates that census (grep over
  `scripts`, `tests`, `.github`: 0 hits); the security history's "census gate" was a one-off chunk gate.
- Both messaging names are on the identity floor (`crates/viola-agent-claude/src/lib.rs:19-20`), and a 2.1.289
  session exports both (M7). A `verify` child is spawned under that strip, so it cannot be addressed by another
  session. CARRY 4's claim holds at HEAD, read from the code rather than from the cited research M10, whose text is
  not at that coordinate.
- Model-layer observation on 2.1.289, this phase's own P2: all 14 subagent hand-backs reached the orchestrator framed
  `Another Claude session sent a message:` ahead of `<agent-message from="…">`, and every background-task
  notification arrived inside a `<system-reminder>` block opening `[SYSTEM NOTIFICATION - NOT USER INPUT]`, ahead of
  `<task-notification>`. The risk that raised: if the UserPromptSubmit `prompt` carried either preface, `starts_with`
  would miss both prefixes, the turn would be filed `human`, and the next driver `send` would be refused
  `human-typing`.
- Hook-layer measurement on 2.1.289, relayed with the P4 answers (inputs#I2) and re-derived here by counts only, no
  content read out (a scratchpad script over the driver's own log of this session, outside this repository): 16
  UserPromptSubmit records; 14 hold a prompt that STARTS with `<agent-message from=`; 0 start with the
  `Another Claude session sent a message` preface; 0 start with `<task-notification>`. So the preface is a
  model-layer frame only: on 2.1.289 a hand-back reaches the hook starting at the tag, and the compiled prefix
  matches it. CARRY 4's hypothesis is falsified for the `agent-message` form at 2.1.289. The `<task-notification>`
  form stays unmeasured at the hook layer: the 14 task notifications of this phase fired no UserPromptSubmit of their
  own.

### M7 — the identity floor, measured by name on the Linux host at 2.1.289
- `env | cut -d= -f1 | grep -E '^CLAUDE'` in this 2.1.289 session's tool environment, names only: 10 names —
  `CLAUDECODE`, `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_EXECPATH`,
  `CLAUDE_CODE_MESSAGING_SOCKET`, `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_SESSION_ATTENDED`,
  `CLAUDE_CODE_SESSION_ID`, `CLAUDE_EFFORT`, `CLAUDE_PID`.
- All 10 are on the 11-name `IDENTITY_FLOOR` (`lib.rs:14-26`); the 11th, `CLAUDE_CODE_BRIDGE_SESSION_ID`, is absent
  here. No name outside the floor. The Windows-measured floor holds on Linux at 2.1.289: no spec drift to raise.
- A ledger row needs a probe with a post-condition. The capture arm reads no environment and writes only the hook's
  stdin (its founder-ruled shape), so no built probe can see a child's `CLAUDE*` names. A row would need the arm to
  record environment names, a widening of a ruled boundary.

### M8 — the sites a row-count move touches
- `/14]`, `14 pass` or `; 14]` literals by file (`grep -c` per file): `tests/cli_verify.rs` 31,
  `crates/viola-e2e/src/harness/run.rs` 18, `src/cmd/verify.rs` 4, `tests/contract_ledger_probes.rs` 2,
  `crates/viola-agent-claude/src/ledger.rs` 2.
- `LedgerRow::ALL` is read at `src/cmd/verify.rs:354-355` and `src/run/version_gate.rs:183` outside its own file.
- `.config/nextest.toml` holds the `verify_window_` class (45 s) and a literal list of verify-driven binaries at 20 s
  (`:24`, `:30`). Three more turns in Run B lengthen every fake-agent verify; under the fake agent a turn is a hook
  spawn pair, not a model call.
- 2.1.289's screens are unmeasured. The compiled `SIGNATURES` literals were chosen on 2.1.288
  (`2026-10-05-real-cli-verify-probes/evidence/screen-probe-2.1.288.md`); if 2.1.289 draws its input box differently,
  `input-box-signature` fails there. Step 0 on 2.1.289 reads this before any code.

### M9 — sizing against one builder window
- The only measured anchor is `:78`, a lighter entry, at 86.7 % of the window (the split record of
  2026-10-05-real-cli-verify-probes). The nearest precedent in shape, the dialog rows (four rows, two runs, the fake
  agent's `--dialogs`), was a whole chunk with three CI rounds and two live STOPs.
- This entry is two strands that share no code path:
  - the rows: `ledger.rs`, `typed.rs`, `verify.rs`, the fake agent's replay, three recorded shapes per stamped set,
    the count literals at 57 sites, the harness literals, a live step 0 and a record round;
  - `send`'s outcomes: `send.rs`, `cmd/run.rs`, `cmd/send.rs`, `human.rs`, the events and log lines, the pinned test,
    the trycmd pins, v1-29.
- Together they exceed the dialog chunk's footprint by the whole `send` strand. The second strand needs no live
  session and consumes what the first compiles, so the seam between them is clean.

### M10 — step 0's measurement, and what the folded fix touches (the revision, inputs#I4)
- Step 0 ran once on 2.1.287 (2026-10-06T19:59:04Z, `evidence/step0-shapes.md`; the raw prompts whole in
  `evidence/step0-prompts.json`). STOP 3 fired; the other four conditions read clear.
- **The wrapped shape.** The 1 500-byte paste's `prompt` is `"\n\n<pasted_content id=\"7ccf\">\n"` + the text +
  `"\n</pasted_content id=\"7ccf\">\n"` (1 558 chars, asserted byte for byte by the recording script). The id is 4
  lowercase hex characters. The 200-byte paste was not wrapped.
- **What HEAD does with it.** `unwrap_pastes` replaces only the pair with its inner text
  (`crates/viola-agent-claude/src/hook.rs:195-214`, `paste_pair` at `:217-223`), so `prompt_text` returns two
  newlines + the text + one newline: 1 503 chars against 1 500 (read through `hook::normalise` by a scratch helper
  built against the crate by path).
- **Three sites pin that HEAD shape as expected**, each with the same lab-measured raw prompt (id `2f85`):
  - `hook.rs:490-493`, the `cli_pair` case: raw `"\n\n<pasted_content id=\"2f85\">\nA paste\n</pasted_content
    id=\"2f85\">\n"` → `"\n\nA paste\n"`;
  - `hook.rs:652-662`, the property `prompt_text_prop_round_trips_a_wrapped_paste`: `lead` + pair + `"\n"` →
    `lead` + text + `"\n"`, with `lead` drawn from `""`, `"\n\n"`, `"note: "`;
  - `tests/hook_events.rs:276-282`, the `paste` case of `hook_prompts_arrive_normalised_with_their_origin`: the
    same raw prompt → `"\n\nA paste {CANARY}\n"`.
  A fourth holder, the fuzz seed `fuzz/corpus/hook_stdin/paste-pair`, is input only: the target asserts no text
  (`fuzz/fuzz_targets/hook_stdin.rs:12`, one `is_object` assert). The census: `grep -rn -E
  'unwrap_pastes|unescape_tags|prompt_text|pasted_content' crates src tests fuzz scripts schemas
  fixtures/fake-scripts .config`, hits outside `hook.rs` in those two files only.
- **Two spec sentences disagree once the framing is measured.** architecture [CLI Version Compatibility] says the
  unwrap "keeps the ends byte for byte" (`.andromeda/architecture.md:81`). architecture [Delivery Confirmation]
  says matching compares the sent text with `prompt-submitted`'s `text` exactly, after the wrapper is removed
  (`:49`), and the `prompt-submitted` contract says `text` "is the submitted prompt as it was sent" (`:292`). The
  direction resolves it for the second (inputs#I4); line 81 becomes an expected amendment.
- **`send` and the wrapped shape (read from the code; no `send` was run).** `SendSlot::claim` takes a prompt for
  the in-flight send only on `f.text == text` (`src/run/send.rs:81-90`), and `append_hook_event` moves the wheel to
  the human for an unclaimed prompt the hook filed `human` (`:146-167`). The only reader of the normalised text
  outside `hook.rs` is that claim (`grep -rn 'data\["text"\]' src/run src/cmd crates/viola-mcp/src`: `send.rs:147`,
  and `src/cmd/run.rs:95`, a type check).
- **The fake agent can carry the case.** `Wrapper::boot` hands its extra arguments to the fake agent
  (`tests/cli_send.rs:28-42`, the `boot` helper; `--local-command-mode` at `:374`), and `submit`
  (`src/bin/viola-fake-agent.rs:425-445`) is where `--framing` answers a text whose stem is `paste-1` with the
  recorded variant. So a `send` of the compiled long text against a `--framing` agent reaches the wrapper as the
  recorded wrapped prompt. This is test-plan §6 Path 2 step 3's first half (`.andromeda/test-plan.md:738`), which
  no test builds today (M3).
- **The escape, as measured mid-text.** The typed `<pasted_content id="1">` and its close arrived as
  `<\pasted_content …>` and `<\/pasted_content …>`; the typed `<task-notification>` arrived as typed. The doc
  comment on `prompt_origin` (`hook.rs:171-175`) and architecture's Tag escaping bullet (`architecture.md:89`) say a
  typed tag arrives escaped. The start-of-prompt position was not measured.
- **`/clear`.** SessionEnd (`reason` `clear`, the old session, with `prompt_id`) then, 30 ms later, SessionStart
  (`source` `clear`, a new `session_id`, no `model` key); no UserPromptSubmit; the input box settled 1.3 s after.
  The Ctrl-C exit fires one more SessionEnd (`reason` `prompt_input_exit`), so Run B's capture set holds two
  SessionEnd captures and the `/clear` one is told apart by `reason` or claim order.
- **Static readings of the 2.1.287 binary's bundled script** (no session; not live measurements): a paste becomes a
  reference, and so is wrapped, when it is longer than 800 characters or holds more than 2 lines; the wrapping sits
  behind a feature flag whose compiled default is off; when the pasted text already ends in a newline the CLI adds
  none before the close tag, so `paste_pair` would take that text's own last newline for the pair's.
- **Unmeasured, and left so:** a wrapped paste beside typed text in one prompt; two wrapped pastes in one prompt; a
  pasted text that ends in a newline.

### M11 — the red record round, and what the wait change touches (the second revision, inputs#I6)
- **The round** (`evidence/record-round-red.md`, `evidence/round-203444Z.txt`, rows 2-6 of
  `evidence/live-sessions.md`): `stamped 2.1.287  15 pass  2 fail`, exit 1, nothing recorded. The CLI's own
  transcript of Run B holds two prompts, the probe prompt and the wrapped long text (id `31a3`); the tag-like text
  and the local command were never pasted. Run B lasted about 12 s.
- **The footer after a long paste** (measured from step 0's raw PTY bytes through `Screen`, and from step 0's drive
  log, both read after the round and recorded in `evidence/record-round-red.md`; the scratch files themselves are
  outside the repository and under the temp dir, which `inputs.py snap` refuses as a source): from the paste on, row
  23 reads `paste again to expand` and no row holds `for agents`; the footer line is drawn again late in that
  window. Step 0 read the input box settled 5.8 s after the long-paste turn's Stop, 1.3 s after the tag-like turn's
  and 1.3 s after the local command, each with 1 s of quiet. Step 0's long turn took 3.4 s; the round's took 1.5 s.
  The hint's length as a timer from the paste is an inference from that one run (about 8 s), not a measurement.
- **What Run B does with it** (read from the tree as the first implement run left it, uncommitted):
  - `framing_turns @ src/cmd/verify/typed.rs:257` pastes each added text only when `input_box_up @ :225` holds over
    the rows the previous settle returned (`:261`, `:278`), and takes those rows from `Run::settle` (`:276`);
  - `settled @ :478` answers a quiet screen that holds a compiled literal at once, and a quiet screen without one
    only at `from + GATE_MAX_WAIT`, with its rows as they are; `Run::settle @ :570` polls it up to `PROBE_DEADLINE`;
  - so a footer that stays replaced for more than 5 s after the Stop yields rows without the literal, and the guard
    pastes nothing more. That is the round's reading.
- **Who else calls the settle** (`grep -n -E '\.settle\(' src/cmd/verify/typed.rs`: 8 call sites): Run A's modal
  (`:76`), Run B's ready (`:105`) and first turn (`:246`), Run C (`:155`), Run D (`:185`), `input_box_settles`
  (`:586`), and the three in `framing_turns` (`:276` in the loop for both paste turns, `:291` after the local
  command). Only `:276` feeds a paste guard after an added turn. The first turn's settle (`:246`) also sets
  `turn_settle_ms` and the `turn` screen, which the `quiet-period` and `input-box-signature` rows read against
  `GATE_MAX_WAIT`; it must keep its meaning.
- **The product gate is a different reader of the same constant**: `Screen::verdict @
  crates/viola-agent-claude/src/screen.rs:122-133` returns `InputNotReady` once `GATE_MAX_WAIT` (`:14`, 5 s) has
  passed without `QUIET_PERIOD` (`:10`, 300 ms) of quiet. Nothing in `src/cmd/verify/` calls it, and the wait change
  does not touch it (inputs#I6).
- **The bound**: `PROBE_DEADLINE @ src/cmd/verify.rs:37` is 120 s, already the bound of every capture wait in the
  typed runs.
- **Forcing the window in a test**: the fake agent draws a recorded screen through `write_screen @
  src/bin/viola-fake-agent.rs:156` (`render_screen @ :149`: a clear, then the rows), and after a turn's Stop it
  draws `turn` (`submit @ :432`). A clear with no rows is a quiet screen without a literal. The file's existing
  test-only hold, `--stop-receipt-hold-ms`, is an argv option capped by a constant (`STOP_RECEIPT_HOLD_CAP_MS @
  :29`, parsed at `:94-97`): the form a second hold takes.
- **The test class for a designed wait**: a test named `verify_window_…` runs under the nextest override at
  `.config/nextest.toml:24` (15 s period, 3 periods, a 45 s kill; `:55` under the `mutants` profile). The one such
  test today, `verify_window_without_screens_fails_every_interactive_row @ tests/cli_verify.rs:829`, read 21.1 s
  at the first implement run. A fake-agent verify read 3.9 s to 4.6 s there, so one held window of 6 s gives a
  test of about 10 s to 11 s (predicted; to be measured).
- **Sessions**: 6 of the cap are used (step 0 = 1, the first round = 5). The cap is 12 (inputs#I6): the rehearsal
  takes 1 and the second round 5, with none spare.

## Files inspected
- `src/cmd/verify.rs` (40-250, 380-456) — the program default, the version read, the five spawns, the stamp merge,
  `--record`.
- `src/cmd/verify/typed.rs` (1-290) — the four runs, Run B's one paste, Run C's three, the capture waits.
- `crates/viola-agent-claude/src/ledger.rs` (20-60, 400-490, 596-800) — the row set, `Probes`, `check`,
  `merge_stamp`, `verified`.
- `crates/viola-agent-claude/src/hook.rs` (96-235) — the prefixes, `prompt_origin`, the unwrap and un-escape.
- `crates/viola-agent-claude/src/lib.rs` (10-30) — `IDENTITY_FLOOR`.
- `src/run/send.rs` (1-440) — the slot, the window, `append_hook_event`, the outcome writers.
- `src/run/version_gate.rs` (160-192) — `stamps_verdict`.
- `src/cmd/send.rs` and `src/human.rs` (function index) — the client's rendering and the mirror writers.
- `src/bin/viola-fake-agent.rs` (40-110, 395-470) — the argv options and `submit`.
- `tests/cli_send.rs` (360-400), `tests/contract_ledger_probes.rs` (whole) — the interim pin and the two lists.
- `crates/viola-e2e/src/harness/run.rs` (325-352) — `LEDGER_ROWS` and the `--local-live` suite.
- `schemas/diag-line.v1.json` (grep) — `confirmed` on the `send-*` events.
- `.andromeda/architecture.md` (lines 49, 77, 80, 81, 88, 89, 457) — the five row families and delivery confirmation.
- `2026-10-05-dialog-rows-and-re-probe/plan.md` (262-300) and its `evidence/live-sessions.md` — the live-leg firing
  form and the session ledger.
- At the second revision: `src/cmd/verify/typed.rs` (whole, as the first implement run left it), `src/cmd/verify.rs`
  (whole), `src/bin/viola-fake-agent.rs` (whole), `crates/viola-agent-claude/src/screen.rs` (27-43, 113-133),
  `tests/cli_verify.rs` (whole), `tests/support/verify.rs` (whole), `.config/nextest.toml` (whole),
  `evidence/record-round-red.md`, `evidence/paste-guard-control.md`, `evidence/live-sessions.md`.
- At the revision: `crates/viola-agent-claude/src/hook.rs` (90-240, 438-664) — the unwrap, `paste_pair`, the case
  table and the properties; `tests/hook_events.rs` (240-330) — the `paste` case; `tests/cli_send.rs` (1-256,
  370-387) — the `boot` helper, Path 2's case, the local-command pin; `src/run/send.rs` (70-173) — `claim` and
  `append_hook_event`; `src/bin/viola-fake-agent.rs` (60-110, 395-470) — the argv options and `submit`;
  `src/cmd/verify/typed.rs` (whole) — the four runs; `fuzz/fuzz_targets/hook_stdin.rs` (grep);
  `.andromeda/architecture.md` (lines 49, 81, 89, 292) and `.andromeda/test-plan.md` (730-742);
  `evidence/step0-shapes.md` and `evidence/step0-prompts.json`.

## Graph impact (from the code-graph query; rust plane, trace `tree-query-2026-10-06-local-command-and-paste-framing-rows.json`)
- **`LedgerRow::ALL`** — 12 references: `crates/viola-agent-claude/src/ledger.rs:187`, `:799`, `:1068`, `:1088`,
  `:1352`, `:1486`, `:1781`, `:2439`, `:2450`; `src/cmd/verify.rs:354`, `:355`; `src/run/version_gate.rs:183`. Growing
  the array reaches verify's step loop and the gate's tests with no signature change.
- **`verified`** — one production caller, `stamps_verdict @ src/run/version_gate.rs:170`; nine test calls in
  `ledger.rs`. The verdict change of M2 needs no edit there.
- **`check`** (`ledger.rs`) — one production caller, `check_step @ src/cmd/verify.rs:364`; `Probes` gaining a field is
  a struct change every test builder of `Probes` in `ledger.rs` follows.
- **`PROBE_PROMPT`** — `src/cmd/verify.rs:23`, `:168`; `src/cmd/verify/typed.rs:24`, `:233`; `ledger.rs:624`: the
  pattern a new compiled probe text follows.
- **`CONFIRM_WINDOW_FALLBACK`** — `src/run/send.rs:14`, `:109`, `:182`; `ledger.rs:16`, `:637`: the window a
  post-condition wait stays inside.
- **`IDENTITY_FLOOR`** — `crates/viola-agent-claude/src/lib.rs:102` and its own tests: no caller outside the crate.
- **`unwrap_pastes`, `prompt_text`, `claim`** (the revision's query, 6 rows): `unwrap_pastes` is called by
  `prompt_text @ hook.rs:188` alone, and `paste_pair` by `unwrap_pastes @ hook.rs:200`; `prompt_text` by `data_of @
  hook.rs:158` and the two tests at `:523` and `:661`; `claim` by `append_hook_event @ src/run/send.rs:149`. The
  unwrap change reaches production through one path: `normalise` → `prompt-submitted`'s `text` → `claim`.

## Patterns detected
- **A row is a variant, an id, words, and one arm of `check`** (`ledger.rs:20-53`, `:614-643`): the row reads
  `Probes` only, never a file.
- **Several pastes in one run, each gated on the previous Stop** (`typed.rs:135-154`): the shape Run B's added pastes
  take.
- **A recorded shape is replayed only when argv asks** (`viola-fake-agent.rs:432-434`, `--dialogs`): a fixture set
  without the shape fails only the rows that need it.
- **A live record entry names its binary and copies by glob** (`2026-10-05-dialog-rows-and-re-probe/plan.md:277`): the
  home is `vhome`, the stop kills only `claude` processes whose cwd is a probe dir, and the expect atom is the
  `stamped <version>  <n> pass  0 fail` line printed at `src/cmd/verify.rs:225-228`.
- **An outcome is one event record plus one codes-only line** (`send.rs:325-347`, `:359-388`).

## Conventions to follow
- **Claude shapes stay in `viola-agent-claude`**: the local-command list and its post-conditions are data there;
  `src/run/` takes a normalised verdict (`hook.rs` is the only file naming a tag; `send.rs` names none).
- **Test oracles are literals**: row ids, the row count and the floor's names are typed in the test, never imported
  (`tests/contract_ledger_probes.rs:15-30`; `.claude/rules/testing.md:40`).
- **One human writer module**: every `send` line goes through `src/human.rs` (`:43-61`), one `write_all` per line.
- **A fake-agent replay is an argv option, never an environment variable** (`viola-fake-agent.rs:66-97`).
- **Editor lines**: the graph's `line` is 0-indexed; every coordinate above is `line + 1`.
- **A change to the unwrap or the un-escape lands as labelled `#[case]` rows beside a property**, with the
  crate's committed `proptest-regressions/` kept current (the tests extract, per test-plan §4 What unit tests cover →
  viola-agent-claude and §6 Property suite; the table at `hook.rs:489-526`, the property at `:649-662`).
- **A root send case boots through the file's own `boot` helper and reads `events.ndjson` records after an
  offset** (`tests/cli_send.rs:28-42`, `:170-197`): the verdict is the exit code, the JSON document and the CL-1
  records, never the screen.

## New files to create
- derived `fixtures/claude/2.1.287/UserPromptSubmit.paste-*.json` by `target/debug/viola --home <vhome> verify --record <out> -- <the 2.1.287 binary>` — the recorded long-paste and tag-like prompts
- derived `fixtures/claude/2.1.287/*.clear-*.json` by `target/debug/viola --home <vhome> verify --record <out> -- <the 2.1.287 binary>` — the hooks the recorded `/clear` fired
- `viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/` — the live-session ledger, the step-0 shape record, the record round

## Files to modify
- `crates/viola-agent-claude/src/ledger.rs` — the three rows, their probe texts and checks, the local-command list, Run B's captures in `Probes`
- `crates/viola-agent-claude/src/hook.rs` — the unwrap's framing newlines, the doc comment on `prompt_origin`, labelled cases over the measured shapes, the reworked wrapped-paste property
- `crates/viola-agent-claude/proptest-regressions/hook.txt` — only if the reworked property records a failing seed
- `tests/hook_events.rs` — the `paste` case's expected text
- `tests/cli_send.rs` — one added case, the wrapped send; no existing line changes
- `src/cmd/verify.rs` — Run B's captures read, the new shapes recorded
- `src/cmd/verify/typed.rs` — Run B's three added pastes
- `src/bin/viola-fake-agent.rs` — the replay of the three shapes behind an argv option
- `tests/cli_verify.rs` — the step-line and summary literals
- `tests/contract_ledger_probes.rs` — the row ids, the count, the two lists
- `tests/contract_fake_agent_drift.rs` — the new recorded payloads in the byte-for-byte replay
- `tests/contract_fixture_hygiene.rs` — the new fixture names in the walk
- `tests/cli_fake_agent.rs` — the new replay option's cases
- `tests/support/verify.rs` — the verify helper's replay arguments
- `tests/support/home.rs` — the stamped home's verify arguments
- `crates/viola-e2e/src/harness/run.rs` — `LEDGER_ROWS` and its unit tests
- `crates/viola-e2e/src/harness/boot.rs` — boot step 4's verify arguments
- `schemas/claude-fixture.v1.json` — only if the recorded `clear` shapes need a field the schema lacks
- `.config/nextest.toml` — only if a measured verify-driven floor moves

## Open questions
- none — the three plan decisions research raised were answered at P4 (inputs#I2): 2.1.287 alone is recorded, under
  a cap of 8 live sessions; Run B takes the three compiled pastes; the cross-session half of the harness-prefix row
  and the identity-floor row re-home to "First live test and self-drive"; `send`'s outcomes move to a new entry. The
  `send` strand's files (`src/run/send.rs`, `src/cmd/run.rs`, `src/cmd/send.rs`, `src/human.rs`, `tests/cli_send.rs`)
  left this list with it.
- none after the second revision — the red record round was answered by inputs#I6: the wait change, the rehearsal,
  the second round and the cap of 12. The readiness-gate reading is the wrap's to route.
- none after the revision either — step 0's STOP 3 was answered by inputs#I4. `tests/cli_send.rs` is back on the
  list for one added case only; `src/run/send.rs`, `src/cmd/send.rs` and `src/human.rs` stay off it.
