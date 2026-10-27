# tests · cascade read · group 3 of 8

## Carried
- (.andromeda/test-plan-detail/unit-test-strategy.md:9) The channel frame cap is a boundary the chunk proves on both sides: the codec's unit cases must hold the cap at `MAX_FRAME` − 1, `MAX_FRAME` and `MAX_FRAME` + 1, the over-size frame refused.
  > ndjson frame codec at `MAX_FRAME` − 1, `MAX_FRAME` and `MAX_FRAME` + 1.
  (.andromeda/test-plan.md:380)
  > the ndjson frame codec at the `MAX_FRAME` boundary, the JSON-RPC error mapping
- (.andromeda/test-plan-detail/unit-test-strategy.md:10) A channel method and its params are external decisions the scope requires closed: the mapping must give -32600 for an invalid (over-size) request, -32601 for an unknown method and -32602 for bad params, each pinned by a unit case.
  > JSON-RPC error mapping: -32700 / -32600 / -32601 / -32602 with `data:{supported,wrapper}`
- (.andromeda/test-plan-detail/unit-test-strategy.md:17) The `events.ndjson` reader is one of the capped file readers and the filesystem coverage trigger names its over-long line: the reader must count an over-`MAX_FRAME` line in `torn_lines`, unknown kinds and fields in their own counts, and never panic.
  > The reader counts and never panics. `torn_lines`: an over-long line (> `MAX_FRAME`), a terminated line that is not one JSON object, and the unterminated last line of the read, which it neither returns nor rewrites. `unknown_kinds`: a line whose `kind` is absent, not a string or not a known kind; it is not returned. `unknown_fields`: once per line, a known-kind line with a top-level key outside the six or a `data` key outside its kind's list; it is returned (unit cases `three_counts`).
  (.andromeda/test-plan.md:382)
  > the reader's three skip counts (it counts and never panics)
- (.andromeda/test-plan-detail/unit-test-strategy.md:19) `snapshot.json` is a capped file read whose oversize or malformed form must be read by its reader's failure rule: the classified read must tell an unreadable or unsupported snapshot apart and send a parse failure to replay, never a panic.
  > Snapshot envelope: the classified read tells the snapshot, no file, an unreadable one and an unsupported `v` apart, reading `v` first; an unsupported `v` or a parse failure goes to replay through the read-or-replay function, which logs one `state-recovered` line for each and none for a present or an absent snapshot (unit cases `snapshot_cause`, `replay_recovers`).
- (.andromeda/test-plan-detail/unit-test-strategy.md:29) The scope names a pinned key among the names that reach a path join: the pin key's closed shape (the version and the content key) must be held by its unit case before `bin/<key>/` is joined.
  > `pin_key_is_the_version_and_the_content_key`
- (.andromeda/test-plan-detail/unit-test-strategy.md:32) A hook event's payload is an external input whose kinds and fields the scope requires closed or counted: payload parsing must count an unknown field, never fail on it.
  > Hook payload parsing per `fixtures/claude/<ver>/` via `#[files]`; unknown fields are counted, not fatal.
  (.andromeda/test-plan.md:384)
  > hook payload parsing per recorded fixture set (unknown fields counted, not fatal)
- (.andromeda/test-plan-detail/unit-test-strategy.md:36) The hook payload bounds the task names are these keys: an option's `description` / `preview` must be kept only as a string (else `null`) and the Stop's `background_tasks` carried only as its length, each pinned by these unit cases.
  > The question and Stop data keys: `question_option_details_carry_description_and_preview` and `question_option_details_are_null_when_absent_or_not_a_string` (`crates/viola-agent-claude/src/dialog.rs`: a missing or non-string description or preview is `null`, never a refusal); `turn_ended_background_tasks_counts_the_array` (the array's length only, a canary in a task's `description` never reaching the data) and `turn_ended_background_tasks_is_null_when_absent_or_not_an_array` (a non-array writes the drift entry `{"path":"background_tasks","expected":"array"}`) in `crates/viola-agent-claude/src/hook.rs`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:38) The statusline payload is a bounded external input whose parser the property suite names: a malformed `resets_at` must become the closed `"unknown"`, never a free string or a panic.
  > Statusline `rate_limits` parsing, with a malformed `resets_at` becoming `"unknown"`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:39) `claude agents --json` output is a capped reader and its row status a closed decision the scope names: an over-`MAX_FRAME` or malformed read must give the closed `AgentsFailure` (`unknown` on the board), the status map only the measured words, and a row `name` never a `ViolaName`.
  > The `claude agents --json` parser: an over-`MAX_FRAME` or malformed payload gives `unknown`, and a row `name` never becomes a `ViolaName`. As landed (chunk 2026-10-16-unwrapped-sessions-on-the-board): `agents::parse` returns `AgentsList { rows, skipped, tolerated }` or the closed `AgentsFailure` (`Oversize` · `Malformed`), and a failed, oversize or malformed read leaves the board as `board::read` gave it (STATUS `unknown`, no unwrapped row); the row's `sessionId` is its NAME, never a `ViolaName`; cases `agents_parse_*`, `agents_status_maps_each_measured_word_and_nothing_else`, `agents_row_joins_by_child_pid_or_session_id` and `agents_failure_details_are_the_closed_words`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:41) The paste-injection trigger names the `typed_text` rule and the paste rule is re-verified at the client and the wrapper: the typed text must hold no CR, change nothing but CR and LF, and be read by both of its root-bin callers.
  > The sent-text rule `typed_text`: every CR LF pair is one LF, every other CR is one LF, and every trailing CR and LF goes, every one of them in any order; the typed text holds no CR and nothing else changes (not a TAB or a space, and no LF that is not at the very end).
  (.andromeda/test-plan-detail/unit-test-strategy.md:41)
  > It has two product callers, both in the root bin: the wrapper's `send` (`src/run/send.rs`) and the client's `refusal_of` (`src/cmd/send.rs`).
- (.andromeda/test-plan-detail/unit-test-strategy.md:48) The chunk re-verifies the vt100 panic degrade at the `catch_unwind` sites it re-reads: a forced panic in the feed must give `input-not-ready`, held by a unit case.
  > The vt100 feed runs under `catch_unwind`, and a forced panic gives `input-not-ready`.
  (.andromeda/test-plan-detail/unit-test-strategy.md:47)
  > are evaluated as the readiness-gate verdict (ready / wait / `input-not-ready`) over fed vt100 bytes
- (.andromeda/test-plan-detail/unit-test-strategy.md:55) An unknown refusal read from a wrapper is an external decision the scope requires closed: it must deserialise to `#[serde(other)] unknown` and map to exit 14, never a panic and never 20.
  > The exit-code mapping from `RefusalReason` / fault to 0/1/2/10–14/20/21. It includes the multi-version-compat case: a wrapper `{"result":{"refusal":"<literal unknown string>","detail":…}}` payload deserialises to the `#[serde(other)] unknown` variant and maps to exit 14 with a `--json` refusal document. It never panics and never maps to 20.
  (.andromeda/test-plan.md:372)
  > `RefusalReason` serde round-trip in kebab-case, including the `#[serde(other)] unknown` fallback.
  (.andromeda/test-plan.md:390)
  > the exit-code mapping (an unknown refusal maps to exit 14, never a panic and never 20)
- (.andromeda/test-plan-detail/unit-test-strategy.md:57) `config.json`'s thresholds and `budget.json` are capped reads with closed values the scope names: `Percent` and the threshold read must refuse an out-of-range value to its default, and `budget_read_*` must hold the bounded read of `budget.json`, an oversize or malformed file planted only by altering the writer's bytes.
  > Beside them: `percent_*`, `thresholds_*`, `gate_eval_*` and `gate_release_*` (`src/run/budget.rs`, `src/cmd/run.rs`, `src/run/wheel.rs`), `send_budget_paused_*` (`src/run/send/budget.rs`) and `budget_read_*` (`crates/viola-state/src/budget.rs`).
  (.andromeda/test-plan.md:374)
  > `Percent` accepts 0–100 and rejects 101 and negatives.
  (.andromeda/test-plan.md:908)
  > - NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version (the operator's answer at the chunk 2026-10-10-budget-governor wrap, where `budget_read_*` landed in that form).
- (.andromeda/test-plan-detail/unit-test-strategy.md:58) The paste rule's outcome must hold beside the typed-text rewrite: a refused control character before a trailing or inner CR / LF must still be `control-character`, and a CR-and-LF-only text still `empty-text`.
  > a refused control character before the trailing CR or LF is still `control-character` (this pins the outcome, not the order: CR and LF are allowed, so the refusal is the same whichever runs first)
  (.andromeda/test-plan-detail/unit-test-strategy.md:58)
  > a refused character beside an inner CR LF is still `control-character`, and a text of only CR and LF characters with a CR that is not last (CR LF CR) is still `empty-text`.
- (.andromeda/test-plan-detail/unit-test-strategy.md:67) The task names the root bin's `send` and `answer` refusal-ordering unit cases and the paste rule at the client before any frame and the wrapper before the first wheel read: `control-character` then `empty-text` must precede every other refusal on both sides, with nothing reserved, issued or typed.
  > Refusal ordering: `send` goes control-character → empty-text → human-typing → budget-paused → turn-running → input-not-ready → no-prompt-submitted, and after the gate returns ready it reads human-typing and then turn-running a second time, before anything is issued or typed (four clock-driven `send` unit cases); an empty typed text is refused directly after the typed text is taken and before the first wheel read, with nothing reserved, issued or typed and the wheel and the running-turn state not read (`send_empty_text_is_refused_first_after_control_character`, `send_empty_text_under_a_human_wheel_is_still_empty_text` and `send_empty_text_while_a_turn_runs_is_still_empty_text` in `src/run/send.rs`; the client's `deliver_empty_text_is_refused_before_any_frame` and `refusal_of_reads_a_refused_character_before_an_empty_text` in `src/cmd/send.rs`); `answer` goes human-typing → unverified-cli → unknown-dialog; `control-character` precedes all of them.
  (.andromeda/test-plan.md:377)
  > `validate_paste_text`: LF/CR/TAB and multi-byte UTF-8 accepted; ESC, other C0, DEL and U+0080–U+009F refused with `control-character`. The check runs on decoded `char`s, and a proptest property shows UTF-8 continuation bytes never trip it.
  (.andromeda/test-plan.md:386)
  > handler re-validation:
  >   - a non-`ViolaName` `target`, a non-`u64` `dialog_id`, an open-string `behavior` and control chars in `text` all give `isError:true` with code-only payloads
  (.andromeda/test-plan.md:390)
  > the `send` and `answer` refusal ordering
- (.andromeda/test-plan-detail/unit-test-strategy.md:69) `config.json`'s `dialog_hold_s` is a capped read into a closed type the scope names: `read_hold` must read it once through `take(MAX_FRAME)` into `DialogHold` and fall back to 60 with one rejection line.
  > `read_hold` takes `config.json`'s `dialog_hold_s` once at start through `take(MAX_FRAME)` and `v` = 1 (`read_hold_takes_a_whole_number_of_seconds_in_range`, `read_hold_falls_back_to_sixty_with_one_rejection`)
  (.andromeda/test-plan.md:375)
  > `DialogHold` accepts whole seconds 60–3600 and rejects 0, 59, 3601 and `u16::MAX`, its `Default` the 60 s `DIALOG_HOLD_DEFAULT` (`dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred`, `dialog_hold_bounds_are_sixty_and_thirty_six_hundred_seconds`)
- (.andromeda/test-plan-detail/unit-test-strategy.md:75) A channel `driver` / `driven` name is one the scope requires validated before a path join: the served link method must refuse a bad pair with `invalid-params`, held by its unit case.
  > `links_set_method_refuses_a_bad_pair_with_invalid_params`
  (.andromeda/test-plan.md:373)
  > `ViolaName` accepts `[a-z0-9-]` and rejects `../x`, uppercase, `/` and empty.

## Bindings
- **unit (paste rule, refusal ordering) ↔ integration and MCP handlers**: the unit ordering cases (.andromeda/test-plan-detail/unit-test-strategy.md:67) and `validate_paste_text` (.andromeda/test-plan.md:377) are the lower layer under the wrapper-channel case where `control-character` wins and the MCP handler's `invalid-params` re-validation (.andromeda/test-plan.md:386); the client and the wrapper sides are witnessed separately, neither standing for the other.
- **unit (parsers) ↔ property suite**: the hook stdin, statusline and `claude agents --json` parsers (.andromeda/test-plan-detail/unit-test-strategy.md:32), (.andromeda/test-plan-detail/unit-test-strategy.md:38), (.andromeda/test-plan-detail/unit-test-strategy.md:39), the frame codec at `MAX_FRAME` ± 1 (.andromeda/test-plan-detail/unit-test-strategy.md:9) and the vt100 feed under `catch_unwind` (.andromeda/test-plan-detail/unit-test-strategy.md:48) each pair their unit cases with the property suite's block of the same name (.andromeda/test-plan.md:618).
- **unit (vt100 degrade) ↔ chaos suite**: the unit `input-not-ready` case (.andromeda/test-plan-detail/unit-test-strategy.md:48) holds the degrade; the passthrough of the child's bytes is the cross-process half, `tests/chaos_feed_panic.rs` booting outside the scans (.andromeda/test-plan.md:431).
- **unit (hook payload) ↔ security sweep**: unknown fields counted (.andromeda/test-plan-detail/unit-test-strategy.md:32) is the parser's side; oversize stdin and malformed JSON failing open with no body is the sweep's fail-open matrix (.andromeda/test-plan.md:601).
- **unit (bounded state-file reads) ↔ test data**: an oversize or malformed `budget.json`, `snapshot.json` or `ledger/stamps.json` in a reader's unit case (.andromeda/test-plan-detail/unit-test-strategy.md:57), (.andromeda/test-plan-detail/unit-test-strategy.md:19) is planted only through the writer's own bytes (.andromeda/test-plan.md:908).
