# a11y-plan — amendments

## 2026-09-24-supply-chain-and-workflow-gates — Platform: ci.yml is the one push/PR workflow
**Section:** §9 Pipeline integration → Platform
**Change:** was "one workflow `ci.yml`"; now "one push/PR workflow `ci.yml`". The scheduled `nightly.yml` carries no a11y step; the E2E/a11y leg stays in `ci.yml`.
**Why:** the chunk added `nightly.yml`, the weekly `cargo deny check advisories` run, so `ci.yml` is no longer the only workflow; raised by the orchestrator, not by a detector.
**Ref:** .andromeda/runs/2026-09-24T09-41-13-wrap/

## 2026-09-24-diagnostics-plane — stale a11y-violation resolved-question bullet
**Section:** §12 A11y Decisions Log → Resolved questions
**Change:** the `a11y-violation` bullet now states what Z7 / D-A11Y-09 decided: it is not a product `event` value. It is a harness-only row validated by the tests-owned `e2e-web/schemas/a11y-row.v1.json`, and the shipped `ObsEvent` and `diag-line.v1.json` enum (19 values) carry none. Retired: "accepted as a tests + obs enum" / a "new closed-enum value".
**Why:** the chunk's contract test (green) disproved the stale claim; the D-a11y-obs-schema proposal was applied as routine.
**Kept:** the Decisions-Log history entry still reading "accepted as a tests + obs enum" stays unchanged, as history.
**Ref:** .andromeda/runs/2026-09-24T10-40-06-wrap/

## 2026-09-24-quality-gates — CLI output-discipline evidence reports under the CI `coverage` suite
**Section:** §1 (CLI output discipline bullet) · §1 cli surface (Automated tool reach) · §9 Per-pipeline-stage table (Unit / integration row) · §10 Standard+ invariants
**Change:** the CLI output-discipline tests still run on all three OS legs. Locally they report under `nextest-integration` / `nextest-e2e`; in CI they report inside the per-OS `test` job's single instrumented `coverage` suite, gated by `gate --require coverage,doctest`. The run JSON `suites[]` enum in the table row gains `coverage`.
**Why:** the chunk replaced the CI unit and integration runs with one `run --coverage`; raised by the orchestrator at Validate, since the a11y detector flagged it as outside both D-a11y invariants.
**Kept:** "failures surface only as nextest failures in `suites[].failures[]`" stays true under `coverage`, unchanged.
**Ref:** .andromeda/runs/2026-09-24T14-48-15-wrap/

## 2026-09-25-pty-wrapper-on-windows — Windows zero-viola-bytes oracle is literal absence
**Section:** §3 A11y Assertion Harness Contract → Keyboard test harness → Tooling · §6 State color tokens → CLI equivalent
**Change:** the zero-viola-bytes clause means viola's own literals absent on every leg; "no SGR or cursor control the child did not emit" (byte-for-byte against an unwrapped run) holds on Linux and macOS only. On Windows ConPTY emits its own `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H`, an OSC 0 title and `ESC[?25h` on every spawn and re-renders nested output, so the windows-2025 `viola run` check is literal absence. §6 was "zero SGR under `viola run`"; now zero SGR of viola's own.
**Why:** measured on the Windows host: ConPTY's own sequences disproved the byte-for-byte oracle on Windows.
**Kept:** §1 stays unedited — it is verbatim from a11y-scope.md per D-A11Y-15 and its lines are not deferral clauses; §3 and §6 carry the truth. "ConPTY swallows focus reports" is not amended — no measured basis; it is carried as a labelled HYPOTHESIS on the route entry that builds focus/mouse-sequence handling.
**Ref:** .andromeda/runs/2026-09-25T17-43-18-wrap/

## 2026-09-27-browser-verdict-reachability — the browser pipe on three OSes, the a11y verdict ubuntu-judged, axe pin deferred
**Section:** §3 (CI integration Runner; the a11y harness Command; Bootstrap phases `a11y-tooling-install`) · §9 (the layer table's Unit/integration and E2E rows; Pipeline integration; the ubuntu-only sentence) · §11 (CI anti-pattern) · §12 (D-A11Y-12)
**Change:**
- `run --browser` runs the locked Playwright CLI (`node node_modules/@playwright/test/cli.js test` after `npm ci`; was `npx --prefix e2e-web playwright test`) on all three legs of the `test` job (was the ubuntu leg only, `browser-linux-only` elsewhere); the a11y verdict stays judged on the ubuntu leg only. `browser-missing` covers a failed `npm ci` as well as a missing Chromium.
- The test job's gate is `coverage,doctest,playwright` (was `coverage,doctest` / `playwright`).
- `a11y-tooling-install` adds `@axe-core/playwright@4.13.0` itself (was "already declared by tests"): tests declared only `@playwright/test@1.63.0`.
- §11 bans judging the a11y verdict on Windows or macOS (was: running `--browser` there). D-A11Y-12's reason is the ubuntu-judged verdict, not an ubuntu-only `--browser`.
**Why:** founder ruling W125 put the browser pipe on all three CI OSes before the a11y harness lands on it (chunk 2026-09-27-browser-verdict-reachability); the axe pin was deferred to Epoch 8.
**Kept:** §1 (the verbatim a11y-scope copy) and the §12 key-decision history ("already declared") stand as written.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/

## 2026-09-28-mutation-testing-to-the-epoch-boundary — the CLI output-discipline invariants meet mutation at the epoch-boundary audit
**Section:** §10 SLO Invariants & A11y Budgets (the CLI output-discipline invariants P4, P6)
**Change:** where those invariants live in crate tests, the epoch-boundary mutation audit (`run --mutants`, test-plan §10) covers them (was "under the tests' mutation gate").
**Why:** the founder's 2026-09-28 17:59 ruling removed the per-chunk and CI mutation gates.
**Ref:** .andromeda/runs/2026-09-28T21-04-49-wrap/

## 2026-09-29-sideloaded-conpty — the Windows zero-viola-bytes check names both ConPTY hosts
**Section:** §3 A11y Assertion Harness Contract → Keyboard test harness (Tooling); §6 Visual Design Verification → CLI equivalent
**Change:** was "ConPTY itself emits `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H`, an OSC 0 title and `ESC[?25h` on every spawn" (the inbox host only); now the inbox host's bytes (fact 4) and the sideloaded `OpenConsole.exe`'s, `ESC[1t ESC[c ESC[?1004h ESC[?9001h` at spawn and `ESC[?1004l ESC[?9001l` at exit, as measured at the chunk's `evidence/da1-stall.md`. Its DA1 query is answered by the terminal (in the tests, the piped driver), never by viola. The Windows check — viola's own literals absent — runs on both backends: the default case on the sideload and `conpty_sideload`'s tampered case on the inbox fallback. §6 credits both hosts' SGR, cursor and query bytes.
**Why:** the plan asked for the measured preamble if the sideloaded one differed; it does, and the zero-viola-literals oracle holds on both. §1 stays verbatim.
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-t15-07-57-wrap — registry migration (U35): the a11y-plan Decisions Log leaves the body
**Section:** §12 A11y Decisions Log · §3 → A11y testing tool pick · §3 → WCAG criteria mapping · §3 → Screen reader test pattern (each in its key file under `registries/contracts/a11y-plan/`)
**Change:** the log moved verbatim to a11y-plan-amendments-archive.md (6 entries: the 2026-09-24 initial entry with D-A11Y-01 … D-A11Y-20, resolved questions and deferrals, plus five 2026-09-24 subsequent entries — the Phase 0 design-excerpt deviation, the overseer directions, P3.5 review 1, overseer fix pass 3 and its Z7 leftover). Every other in-force item already stood in the body. Lifts, each hand-landed in its §3 key file after the migration (a key's span leaves the body verbatim):
- §3 A11y testing tool pick (under the tui driver bullet): the portable-pty `=0.8.1` pin is an inherited risk (0.9.0 is the maintained line) that a11y never re-pins (D-A11Y-13).
- §3 WCAG criteria mapping (after Tier coverage): below 760 CSS px, data tables may use the SC 1.4.10 two-dimensional exception, and the ATIS, tape and strips get a layout when the phone view lands (resolved question "Reflow below 760 CSS px").
- §3 Screen reader test pattern (after Supplemental to automated): Guidepup real-AT automation enters CI only through a tests-harness change, since the a11y verdict is ubuntu-judged and Guidepup has no Orca support (D-A11Y-12, as amended 2026-09-27).
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/

## 2026-10-01-t12-19-55-wrap — a11y lint and component semantics: React + TypeScript
**Section:** §2 Strategy → the Robust row; §4 ARIA Patterns → the landmark row and the Session strip row; §4 → the web-spa tooling row; §9 CI Integration → Lint; §10 → the lint-stage budget line; §11 Anti-Patterns (the two role NEVERs); §3 → a11y testing tool pick; §3 → Bootstrap phases; §3 → Focus management test harness; §3 → Structured violation JSON schema
**Change:**
- The lint is eslint-plugin-jsx-a11y (was eslint-plugin-lit-a11y 5.1.1), over the JSX/TSX sources in `crates/viola-ui/`; its version is OPEN, owned by the route's frontend-toolchain entry. The `check_source` value is `eslint-jsx-a11y` (was `eslint-lit-a11y`).
- `<header>` is rendered by the `viola-atis` component (was "in light DOM by `<viola-atis>`").
- Session strip renders native `<tr>`/`<td>`; any element left between `<tbody>` and the `<tr>` has no role and `display: contents`; `data-*` state stays on the component's outermost rendered element.
- NEVER a row/table/cell role on a wrapper element of a `viola-*` component; NEVER a role set imperatively (`setAttribute` from an effect or ref) — roles belong in the JSX (was the Lit host / `connectedCallback` / `html` template form).
- The html-validate config declares any `viola-*` elements the rendered page carries; APG patterns are rendered by the React components; native focus is in the DOM they render.
**Why:** founder ruling of 2026-09-30, relayed by the overseer.
**Kept:** §1's Lit mentions (:130, :153), the verbatim upstream copy; every rendered-DOM assertion.
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — the dialog deadline bounds the driver, never the human
**Section:** §8 Cognitive Accessibility → Timeout extensions (CLI)
**Change:** was "viola imposes no limit on a human"; now a raised dialog takes a `viola answer` only within `DIALOG_DEADLINE` (60 s PROVISIONAL, under the `hooks.json` `timeout` of 75 s); on expiry the hook exits 0 with no body, the dialog renders for the human in the `claude` TUI, and a later `answer` is refused `not-delivered` / `unknown-dialog` (exit 13). The terminal still carries no conformance claim.
**Why:** the chunk served `answer` with a deadline; the limit falls on the driver's answer path, and architecture's fail-open contract leaves the dialog to the human.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/

## 2026-10-04-the-wheel — the tui wheel boundary: terminal replies non-editing, the Windows platform fact
**Section:** §3 → Keyboard test harness (Tooling) · §1 → `viola run` TUI passthrough · §1 → Critical path 4 (tui) · §4 → P4 (tui) case (3) · §11 Anti-Patterns → Keyboard
**Change:**
- The non-editing set: was focus, mouse and resize; now the closed list of focus reports, mouse reports (X10, SGR, urxvt) and terminal replies (DA1, DA2, CPR, DECRPM, kitty flags, OSC, DCS), every other byte editing; the replies are proven by the classifier's unit table.
- Case (3) as landed: step-wise (the resize behind its size receipt, then the focus reports with a mouse report as their read barrier), the wheel probed by an `answer` to no pending dialog (exit 13 driver, 10 human). Linux and macOS: the full assertion (no `wheel` record, bytes unchanged). `windows-2025`: was "on all three OS legs … do not move the wheel"; now the platform fact — the inbox ConPTY swallows focus reports, and under win32-input-mode the injected mouse report arrives as win32 key-down records and takes the wheel.
- The Keyboard ban: a mouse report the Windows ConPTY already turned into key-down records is typing.
**Why:** the founder's live rulings F-W2 and F-W3 ("pin the platform fact", the error only ever favouring the human), 2026-10-04, relayed by the overseer; a real Windows terminal's mouse report is measured live at route `:82`.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/

## 2026-10-07-t05-47-07-wrap — the real Windows terminal's mouse report re-pointed to its new route entry
**Section:** §3 → keyboard-test-harness
**Change:** a mouse report from a real Windows terminal is measured live at route `:140` (was `:90`). That coordinate is the entry "Windows-only live measurements", blocked on an interactive Windows host; `:90` was "First live test and self-drive" when it was written.
**Why:** the founder split "First live test and self-drive" three ways and put the first live test on the Linux dev host (R-L1 and R-L2, live, 2026-10-07, relayed by the overseer). The overseer's answers at this wrap moved the Windows-only live items to a new Epoch 7 entry, since no interactive Windows host exists.
**Ref:** .andromeda/runs/2026-10-07T05-47-07-wrap/

## 2026-10-07-send-waits-out-the-paste-hint — the gate's bound falls on the driver; a key during the wait wins
**Section:** §3 → Keyboard test harness (the key file, the tui sentence) · §4 P4 (the tui bullet) · §8 Timeout extensions (the CLI clause)
**Change:**
- §8: a `viola send` on a verified CLI waits in the readiness gate for the input box for at most `GATE_MAX_WAIT` (8.5 s) and is then refused `not-delivered` / `input-not-ready` with nothing typed. The bound falls on the driver only: a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is refused `human-typing`.
- §3 key file and §4 P4: a further outer-PTY case, `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, under boundary clause (2). It is sequenced on the wrapper's `channel-request` line and the `wheel` record, never a timer. Its stated limit: it cannot tell `send`'s first wheel read from its second, whose own proof is the unit tier. The three boundary cases stand as they were.
**Why:** The gate's wait and its 8.5 s bound are the founder's live ruling of 2026-10-07T10:29Z, relayed by the overseer. The refusal of a send whose wait a human key fell into is the overseer's technical answer of 2026-10-07T12:10Z, not the founder's: "the human always wins" has to hold through a wait of seconds. The case's limit is stated on the overseer's disposition at the wrap.
**Ref:** .andromeda/runs/2026-10-07T12-57-41-wrap/
## 2026-10-08-first-live-test-and-self-drive — the keyboard harness names the list as extended and the live takeover reading
**Section:** §3 → Keyboard test harness (Tooling, the tui sentence) · §1 A11y Scope Summary (the NVDA pairing line)
**Change:** the closed non-editing list's terminal replies are DA1, DA2, CPR, DECRPM, kitty flags, OSC and DCS replies and seven more after `CSI`, by exact grammar (`0 n`, `? 997;1 n` and `? 997;2 n`, `4;n;n t`, `6;n;n t`, `8;n;n t`, `48;n;n;n;n t`, `> 4;n m`); the nearest human keys and one-field or one-prefix neighbours stay typing; proof is the unit table plus the reply probe on foot 1.28.0 (was: the first seven only, proven by the unit table alone). It records one live reading of the takeover clause on Linux, supplemental and never gating: a `wtype` key into a live session's window on a compositor the test started itself wrote `wheel{human, human-input}`, the focus wrote none, and the next driver `send` exited 10. The NVDA line calls Windows "the first target" (was: "the live-supported target", citing architecture's old wording). §1's clauses on the list name it by reference and took no edit.
**Why:** the same widening and the same founder ratification as architecture's entry of this chunk; the takeover key's method is the founder's ruling for this chunk.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-09-t14-44-30-wrap — the real Windows terminal's mouse report cited by its route entry's title
**Section:** §3 → Keyboard test harness (Tooling, the tui outer-PTY case)
**Change:** the sentence that says where a real Windows terminal's mouse report is measured live names the working-route entry by its title, "Windows-only live measurements"; was the bare route number `:140`, which no longer named that entry's line (the entry stood at `:144` when this wrap began and stands at `:148` after its two head insertions). Nothing else in the contract moved: the injected report's reading (ci#37227518624) and the three boundary assertions are as they were.
**Why:** a bare route number is not a citation the sweep follows, so every insertion above the entry stales it, and a title moves with nothing. The operator chose it in this wrap's route-adaptation dialogue (item 8 of the overseer's relay), from three options shown: by title, by the new number, or left. A precedent for the next bare route number met in master text, not a standing rule: the other bare numbers read at this wrap were left as they stand.
**Ref:** .andromeda/runs/2026-10-09T14-44-30-wrap/

## 2026-10-10-viola-revive — `revive` among the CLI verbs; a passed revive enters the TUI passthrough boundary
**Section:** §1 A11y Scope Summary (CLI and terminal entities: CLI verbs, TUI passthrough; Boundary-only vendor zones; Surface cli, Notes; Surface tui) · §3 → Keyboard test harness · §6 Visual Design Verification (CLI equivalent) · §8 Cognitive Accessibility (Error recovery, CLI) · §11 A11y Anti-Patterns (Strategy; Keyboard; Visual)
**Change:**
- §1 CLI verbs: the list names `revive`. It takes no `--json`; `--list` prints static ASCII rows `<ts>  <cause>  <id>` with no header, colour or ESC byte; each exit-1 refusal is one `unable:` line and one `hint:` line last; a clap usage error exits 2.
- §1 TUI passthrough, vendor zones and Surface tui: the passthrough is entered by `viola run` and by a `viola revive` whose preflight passed. The boundary binds both; the one clause read by a revive case of its own is that a passed revive writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`).
- §1 cli Notes, §6 CLI equivalent, §11 Visual: the zero-SGR list names a passed `viola revive`; revive's refusal pairs and `--list` rows carry no ESC byte.
- §8 Error recovery: revive's exit-1 refusals take the fixed-message form of design-system cli pattern 2.
- §11 Strategy and Keyboard: the no-conformance-claim ban and the keystroke ban cover a start a passed revive made.
- Key file, Keyboard test harness: the zero-own-lines reading of a revived start is recorded; the keystroke clause and the focus / mouse / resize clause have no revive-specific case.
**Why:** the chunk landed a second verb that hosts the wrapped `claude` TUI through the same start as `run`.
**Kept:** the surface keeps its name, the `viola run` TUI. §8's general CLI form, `unable  <reason>  <detail>`, stands; the fixed-message exceptions live in design-system.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-statusline-pass-through — the statusline-bearing start's zero-viola-bytes reading, and whom the command's 5 s bound falls on
**Section:** §3 → Keyboard test harness (Tooling) · §8 Cognitive Accessibility (Timeout extensions, CLI)
**Change:**
- Keyboard test harness: a statusline-bearing start has its own reading of the zero-viola-bytes clause. `passthrough_with_a_statusline_source_adds_no_viola_bytes` in `tests/tui_passthrough.rs` asserts that a start from a home holding a statusline source adds none of viola's own literals to the outer-PTY stream, green on the three CI runners. On Unix that start writes the settings override and passes `--settings`; on Windows it writes none. `viola hook statusline` hands the user's status line output to the child byte for byte and writes nothing to the terminal itself.
- Timeout extensions: `viola hook statusline` runs the user's status line command under `STATUSLINE_DEADLINE` (5 s, PROVISIONAL) and exits 0 with empty stderr in every case. The bound falls on the user's own command, never on a keystroke and never on the human; no key is blocked, refused or delayed, and the wheel is untouched.
**Why:** the chunk added a start variant and a time bound, and the plan lists every bound viola holds with whom it falls on.
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-budget-governor — the budget pause as a driver-only bound
**Section:** §8 Cognitive Accessibility (Timeout extensions, CLI) · §3 → Keyboard test harness (Tooling)
**Change:** §8's CLI bullet lists the budget pause beside the other bounds that fall on the driver: an automated `viola send` is refused `budget-paused` with detail `five-hour` or `seven-day` (exit 11), nothing typed, until the window's reset or a `viola release <name> --budget` for that instance; no key is blocked, refused or delayed by it. `answer`, `wait`, `last` and `pause` do not read the gate, written as read in the diff with no case yet. The Tooling label gains the closed gate's own reading: `path6_a_reading_over_the_threshold_refuses_the_send` asserts that a human key reaches the child and that the outer PTY holds none of `unable`, `hint:` and `budget-paused`; the focus, mouse and resize clause has no budget-specific case.
**Why:** §8 enumerates every bound that falls on the driver and not the human, and the chunk added one. The chunk adds no web element, so no criterion changes.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-session-links — the four verbs under a closed gate have their case
**Section:** §8 Cognitive Accessibility (Timeout extensions, CLI) · §3 → Keyboard test harness (Tooling)
**Change:** §8: was "no case drives one of them under a closed gate yet"; now the clause that `answer`, `wait`, `last` and `pause` do not read the budget gate is read in the diff and held by `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`): it closes the gate, reads the automated `send` refused exit 11 `budget-paused` / `five-hour` as its control, and reads from the four verbs the exit code and document each gives under an open gate, with one `send-refused` record. The Tooling label names that case as the driver-side reading beside the closed gate's keyboard reading: it types no human key and reads no terminal stream, so the keystroke clause still has its one closed-gate reading and the focus, mouse and resize clause still has no budget-specific case.
**Why:** The previous wrap pinned the missing test on this chunk and it landed. The chunk adds no interactive element, so no criterion changes.
**Ref:** .andromeda/runs/2026-10-11T00-33-08-wrap/

## 2026-10-11-the-board-viola-list — the board's caption pinned by literal; separator, help groups and colour not built
**Section:** §1 (Surface cli ARIA roles inventory and notes; Critical path 6); §4 (per-surface landmark roles, cli row; P1 cli mirror; P6 setup and the outer-PTY SGR bullet); §6 (CLI equivalent); §8 (Consistent identification, Plain language verification, Error recovery cli)
**Change:**
- The caption order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI is pinned by the `CAPTION` literal in `list_human_board_is_the_literal_rows` and the unit `board_row_caption_and_default_row_are_the_literals` (was: a trycmd case); `list`'s fixed strings and its exit-21 refusal pairs are pinned by literals (`tests/cli_list.rs`, the `src/human.rs` units, `instance_and_home_state_pairs_are_the_fixed_texts`).
- The board as built is BAY line → caption row → wrapped rows; the `-- UNWRAPPED - READ-ONLY --` separator is owed to "Unwrapped sessions on the board", the `--help` group labels to "CLI output discipline".
- P6: `list` reads no `claude agents --json`; its row escape is held by unit cases on a recorded CLI version (ESC, C1, DEL, `\n`, `\t` → `\xHH`) and two root cases asserting no ESC or non-ASCII byte on either stream.
- No SGR on the board: each state is carried by its word alone; the amber / dim / bold spans and their outer-PTY check wait on "CLI output discipline".
**Why:** the board as built; owners per the review card the founder approved.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/

## 2026-10-13-epoch-4-cleanup — the board units re-cited after the human split
**Section:** §4 ARIA Patterns & Roles (P1 cli mirror); §8 Cognitive Accessibility (the pinned strings)
**Change:** `board_row_caption_and_default_row_are_the_literals`, `bay_line_*`, `board_row_*` and `instance_and_home_state_pairs_are_the_fixed_texts` sit in `src/human/board.rs`; was `src/human.rs`.
**Why:** the pure-move split of `src/human.rs` by phase; every test kept its name.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/

## 2026-10-13-interrupted-verify-cleanup — verify's interrupted pair
**Section:** §1 Surface: cli → the CLI verbs entity · §8 Error recovery → CLI
**Change:** A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary. It prints only one `unable: verify was stopped before it finished` line and one `hint: run viola verify again to stamp this CLI` line last on stderr (plain ASCII, design-system pattern 2, pinned by the stop table in `tests/cli_verify.rs`), and exits 1. The stop path adds no time bound: its waits end at the stop, and its kill and pump join end at once.
**Why:** The operator's P4 answer gave the interrupted run exit 1 and a fixed pair; the wording is a planning form with the founder's wording owed.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/

## 2026-10-16T14-58-10-wrap — the web entities, browser harness, ARIA and contrast leave 0.1.0; the terminal surfaces stand
**Section:** §1 A11y Scope Summary (preamble, web-spa entities, GUI HTTP API, phone view, Surface: web-spa, triggers); §2; §3 (tool pick, WCAG mapping, violation schema, focus, keyboard, screen-reader and contrast harnesses, and the matching keys with their index labels); §4 (Paths 1–6, landmarks, headings, component catalog, P1–P6); §5–§7 (web bodies to the stub); §8; §9; §10; §11
**Change:**
- Was a web-spa a11y contract: WCAG 2.1 AA + 2.4.11 + 2.5.8 on the page, axe in Playwright, ARIA patterns, focus order, contrast tokens, live regions and violation rows. Now the frontend is deferred to 0.2.0 and its technology is not chosen.
- §5–§7 and the wholly-web keys (WCAG criteria mapping, Focus management, Contrast verification, Structured violation schema) hold that stub line. Their emptied index labels are removed.
- Paths 1 and 5 and P1 and P5 are cut. Path 6 / P6 carry the `viola list` caption-order line that the deferred page must share.
- The terminal surfaces stand: CLI output discipline, the TUI boundary, the wheel case (focus, mouse and resize never move the wheel) and the manual terminal screen-reader passes on P4 and P6.
- The a11y bootstrap no longer names the 2026-09-30 React + TypeScript components.
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI and moves the frontend to 0.2.0, its technology not chosen. The page's screen-reader pass left for `.andromeda/residuals.md`.
**Kept:** By the operator's answer at this wrap (overseer, technical, founder-delegated): the masters stay true to the tree.
- The browser leg (lint, E2E and aggregation stages, `run --browser`, axe tooling, `e2e-web/`) stands, with one marker line in §9.
- The tier line `Standard (1)` is not re-decided. Its justification now rests on the terminal surfaces alone, so the tier is the founder's to re-decide.
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/

## 2026-10-16-remove-the-browser-leg — the browser a11y leg leaves; cli / tui cases gated in the coverage suite
**Section:** §1 → CI integration (Runner, Where results land, Gate); §3 → CI integration (Pipeline integration, Command, Artifact); §3 → Bootstrap phases (all seven, labels kept); §9 (the stand-until paragraph, Per-pipeline-stage artifacts, Pipeline integration, Artifact retention); §10 (Performance budget, Cross-input parallel, Build / deploy failure conditions); §11 → Strategy and CI
**Change:**
- Was a browser a11y leg in the tests' Playwright suite: `run --browser`, `npm ci` and Chromium on every CI leg with the verdict on ubuntu, `gate --require playwright`, axe / html-validate / jsx-a11y lint, `a11y-violations` NDJSON rows with `process:"ui"`, a `jq` SC-coverage aggregation and the `e2e-web/test-results/` upload. Now 0.1.0 runs no browser a11y leg: it renders no page, and the frontend's a11y tooling is a 0.2.0 input, its technology not chosen.
- The cli output-discipline and TUI boundary cases run as nextest cases in the per-OS `test` job's `coverage` suite on all three legs, each leg's `gate --require coverage,doctest` its own verdict; results land in `suites[].failures[]`; no a11y artifact is uploaded; no stage, budget or second driver of its own.
- §9's table keeps the Unit / integration row alone. §10's failure condition is a `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`); the zero-flakiness parallel bans `#[ignore]`.
- Every Bootstrap phase reads none in 0.1.0; violation-json-emission-wire: cli / tui emit no a11y violation rows, and obs's `process` enum is `run · hook · mcp · cli`.
- §11 drops the Playwright second-stack ban, the runtime-axe ban, the `webServer` / `npm test:a11y` ban and the `browser-missing` ban; it bans a `suite` value or a separate a11y driver outside `scripts/agent-run.* run`.
**Why:** This chunk removed the browser test leg and the obs `ui` process (founder ruling 2026-10-16, item 3). No 0.2.0 ban is reserved (the plan's Constraints).
**Kept:** The tier `Standard (1)`, the founder's to re-decide; the manual NVDA / VoiceOver / Orca pass, founder-owned.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — the board's separator and unwrapped rows built; the 5 s agents read
**Section:** §1 (cli ARIA roles inventory; critical path 6, Required ARIA roles); §4 (per-surface landmark roles, the cli row; P6 C0/C1 escaped, Setup); §6 (CLI equivalent); §8 (Timeout extensions)
**Change:**
- The board is the BAY line, the caption row and the wrapped rows, then, only when the one `claude agents --json` read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped read-only rows (NAME the row's `sessionId` through `escape_row`, `n/a` in WHEEL, DIALOG and CLI); no separator without an unwrapped row; a failed read leaves the board as before. The caption line and the separator stand; only the `--help` group labels stay owed to "CLI output discipline". Was: the separator and its rows owed.
- P6: `viola list` reads `claude agents --json` and prints every unwrapped NAME through `escape_row` on one line, held by the `board_row_*` units and `tests/cross_list_rows.rs` in three modes (piped, `NO_COLOR` / `TERM=dumb`, outer PTY; the no-ESC assertion Unix-only, on windows-2025 each escaped name read as text under a 200-column terminal because ConPTY re-renders escapes). Was: no agents read, the fixture-row setup waiting on this chunk.
- §6: the STATUS words `idle` / `busy` and the rack's `n/a` cells are printed words, with no SGR and no truncation.
- §8: `list` and `revive` each make one `claude agents --json` read (direct spawn, `take(MAX_FRAME + 1)`, a 5 s kill); a failed, oversize, malformed or timed-out read leaves the board and passes `revive` (fail open); the bound falls on viola's own child, never on a keystroke or the human.
**Why:** The chunk built the rack (`matrix#v1-37`). `revive`'s read is the founder's own answer to P4 fork 3 (`inputs#I1`, relayed verbatim by the operator); the card confirm provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary.
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-17-cli-machine-contract — a11y: revive takes --json on both arms
**Section:** §1 A11y Scope Summary → CLI verbs entity; §8 Cognitive Accessibility → Error recovery (CLI bullet)
**Change:**
- `revive` takes `--json` on both arms (was: "`revive` takes no `--json`"): each strict-modes or preflight exit-1 refusal is one `{"v":1,"error":"<code>","detail":null}` document on stdout with empty stderr (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`), `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; without `--json` the static ASCII rows and the `unable:` / `hint:` pair are byte-identical. A post-preflight `run` start refusal prints no document (owed to "Exit-cause code catalogue").
- Error recovery: revive's fixed-message pair is the human form; under `--json` the document carries no hint (`revive_json_*`, `tests/cli_revive.rs`).
**Why:** the chunk landed revive's machine form; the output-discipline clauses already name `--json` mode, so no other a11y clause moves.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/

## 2026-10-17-cli-output-discipline — the colour spans built and checked; --help group labels built
**Section:** §1 A11y Scope Summary (cli Notes; Critical path 6, Required ARIA roles); §4 ARIA Patterns & Roles (P6; landmark inventory, cli row); §6 Visual Design Verification (CLI equivalent); §8 Cognitive Accessibility (Plain language, Verification)
**Change:**
- §1 Notes and §6: was "`viola list` prints words only / emits no SGR and reads no `NO_COLOR` / `TERM` / `COLORTERM` … owed to 'CLI output discipline'"; now on a terminal bold every NAME value, amber only on the `DIALOG` word of a wrapped row with a pending dialog (kind plain), a wrapped `stale` row dim end to end (NAME dim and bold, `DIALOG` amber and not dim), the BAY line, caption, empty line and separator plain, each state still carried by its word; the decision reads `NO_COLOR` (empty included), `TERM`, `COLORTERM` and stdout's terminal state, none under `--json` or a passthrough role, depth truecolor / 256 / 16 (Windows after a VT enable restored afterwards); on a narrow terminal an unwrapped NAME cut with `...`, never inside an escape, the width from `viola_pty::host_size()`.
- §4 P6: was "no check until the colour layer lands"; now checked under the outer PTY (`cross_list_rows_style_each_span_beside_its_word_on_a_terminal`) and by the `board_row_style_*` units; dim only on stale rows at the unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity"; `viola wait` and `viola --help` carry no ESC and no lone CR on Unix.
- §4 landmark row and §1 critical path 6: was "the `--help` group labels … not built"; now built over the served verbs (board / traffic / wheel / setup, then `Options:`), the `handoff:` row owned by "Explicit session links", `plugin install` by "Optional plugin install".
- §8: `viola --help` joins `viola list` as pinned by literal (`help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`).
**Why:** the chunk built the layer and the table. The three reads are the founder's own answer «Читать их» to P4 fork Q1 (relayed verbatim by the operator; card confirm a delegate's, the Viola overseer, PROVISIONAL). The width source and served verbs only are the overseer's technical answers to Q2 and Q3, PROVISIONAL. The stale-dim owner is the halt's answer (the overseer, a delegate).
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/

## 2026-10-17-mcp-server-for-drivers — the MCP wait cap, a driver-only bound
**Section:** §8 Cognitive Accessibility → Timeout extensions
**Change:** beside CLI `viola wait`'s caller-passed timeout, the MCP `wait` tool clamps `timeout_ms` to `WAIT_CAP` 50 000 ms, also its default (`viola_mcp::wait_timeout`), so an MCP `wait` with no timeout is answered `timed-out` after 50 s; the bound falls on the driver only (the hidden, plugin-run `viola mcp` serves a driving session, never a person) — no key blocked, refused or delayed, the wheel untouched.
**Why:** the chunk closed the MCP `wait` bound below the client's tool-call timeout; a person's timing is not touched.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-optional-plugin-install — plugin install built: setup: row, verb entity, pinned lines
**Section:** §1 A11y Scope Summary (the CLI verbs entity); §4 (the cli landmarks row); §8 Cognitive Accessibility (Plain language and Error recovery verification)
**Change:**
- §4: was "`plugin install` in `setup:` [owned] by "Optional plugin install""; now built, held by `help_lists_only_served_verbs`, a bare `plugin` unserved; the `handoff:` row still owned by "Explicit session links".
- §1: the CLI verbs entity gains `plugin install` and its output discipline (four plain-ASCII lines in one write, no SGR, no ESC or non-ASCII byte piped, under `NO_COLOR` or `TERM=dumb`; the `ok` and refusal documents; `viola plugin` exit 2).
- §8: its planning-form lines are literal-pinned (`plugin_install_prints_the_two_commands`, `write_plugin_installed_is_the_planning_form_in_one_write`), and its exit-21 home pair and `--json` refusal by `plugin_install_refuses_a_widened_home`.
**Why:** the chunk served the verb; §1 is a verbatim upstream copy, kept current by the founder's ruling.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/

## 2026-10-18-compact-post-condition-ledger-row — the compact send's 40 s confirm window, a bound on the driver only
**Section:** §8 Cognitive Accessibility → Timeout extensions (CLI)
**Change:** once typed, a confirmed `viola send` waits for its confirmation within `CONFIRM_WINDOW_FALLBACK` (10 s), and a `/compact` send on a verified CLI within its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL); unconfirmed inside it, `not-delivered` / `no-prompt-submitted`. The bound falls on the driver only: the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched. Was: the section named no confirm window.
**Why:** the chunk added the compact window (report Symbols / APIs); the plan's expected amendment 8, raised by the wrap.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/

## 2026-10-19-dialog-hold-for-the-driver — the driver's dialog hold is configurable, the human's key unchanged
**Section:** §8 Cognitive Accessibility → Timeout extensions (the CLI bullet).
**Change:** A dialog takes a `viola answer` within the dialog hold: `dialog_hold_s`, whole seconds 60–3600, default 60, a `DialogHold` with PROVISIONAL bounds, under the `hooks.json` dialog `timeout` of 3615 s and the hook's bound of 3605 s. At the hold's end the wrapper replies `null` and appends one `dialog-expired` line that wakes `wait`. The hold bounds the driver, never the human: any human key still hands the held dialog back at once, whatever the hold. Was `DIALOG_DEADLINE`, 60 s, under a 75 s `timeout`.
**Why:** A longer, configurable hold lets a driver relay a dialog to a person and back. The key is a boundary widening the founder answered himself to P4 fork Q1 (`inputs#I2`); the wrap's answer-card confirm was the Viola overseer's (delegate viola-overseer), PROVISIONAL. No human-facing time limit is added.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/

## 2026-10-19-overseer-switch-to-viola — the takeover clause's second live reading, the founder's own key on the product
**Section:** §3 → Keyboard test harness (the takeover clause).
**Change:** The takeover clause has two live readings on Linux, supplemental and never gating. The second, on the product binary over a live `claude` 2.1.296 session: a key the founder typed himself in that session's own terminal wrote `wheel{holder:"human",cause:"human-input"}`, the next driver `send` exited 10 `human-typing` with no `send-issued`, and his own `viola release` wrote `wheel{holder:"driver",cause:"release"}`. Was "one live reading" (a `wtype` key on 2.1.287).
**Why:** The switch's gated step 14 read the takeover against the real terminal and the real CLI, no agent typing the key (the founder's own answer to P4 fork Q3, `inputs#I14`).
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/

## 2026-10-20-t06-58-31-wrap — the founder's Epoch 5 batch: served verbs only and the width source his
**Section:** 1. A11y Scope Summary (Required ARIA roles) · 4. ARIA Patterns & Roles (cli) · 6. Visual Design Verification (CLI equivalent)
**Change:**
- `--help`'s rows over the served verbs only and the board's cut width from `viola_pty::host_size()` stand as built; was "`inputs#I1` Q3 / Q2 of that chunk, PROVISIONAL" (three sites); now the marks are gone.
- In this file's history the provisional marks of these entries are superseded 2026-10-20, their other claims standing, none pruned: 2026-10-13-interrupted-verify-cleanup — verify's interrupted pair (the wording, his); 2026-10-16T14-58-10-wrap — the web entities, browser harness, ARIA and contrast leave 0.1.0; the terminal surfaces stand; 2026-10-16-unwrapped-sessions-on-the-board — the board's separator and unwrapped rows built; the 5 s agents read; 2026-10-17-cli-output-discipline — the colour spans built and checked; --help group labels built; 2026-10-19-dialog-hold-for-the-driver — the driver's dialog hold is configurable, the human's key unchanged.
**Why:** the founder's own answers to his Epoch 5 batch, 2026-10-20, relayed by the Viola overseer through the operator's directive, and his own word widening it to every Epoch 5 delegate mark and the interrupted pair's wording. The bodies take writing rule 7's form, by the overseer's technical answer at this wrap.
**Ref:** .andromeda/runs/2026-10-20T06-58-31-wrap/

## 2026-10-20-epoch-5-cleanup-ii — the board units re-cited at their module
**Section:** §4 ARIA Patterns & Roles (P6, the cli mirror) · §8 Cognitive Accessibility (Plain language → Verification)
**Change:** the `viola list` literal-pinning units are cited under `src/human/board/`. Was: `src/human/board.rs`.
**Why:** `board.rs` is now a module root over `board/`; every unit kept its name.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-25-u55-door — a11y-plan: split into its body and 4 category files
**Section:** (the whole master — the door)
**Change:** split into its body and 4 category files; the map .andromeda/a11y-plan.map.tsv
**Why:** the operator's word: the founder's standing word of 2026-10-25 for the door on viola's masters (F2), the review closed on spec 3b — the reviewer's wording verbatim and lines 307–308 moved — at 1.83× its cost guide, the third pass skipped as it would cross the stop; relayed by overseer1.
**Ref:** $TMPDIR/claude-1000/-home-turbolet-dev-projects-additional-andromeda-worker/a2a855dc-9f94-4016-87d5-2f52c0246ab5/scratchpad/a11y-plan/

## 2026-10-26-server-verification-before-any-frame — the six CLI frame verbs' server-verify refusal
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the CLI verbs entity); §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the cognitive-errors pattern, its CLI item)
**Change:**
- The CLI verbs entity now records that `send`, `wait`, `last`, `answer`, `pause` and `release` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: exit 21, stdout empty and the server-verify `hint:` line last on stderr (plain ASCII, no SGR), or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; on macOS no peer pid is read and the refusal cannot occur (the recorded gap); their `--json` codes and exit codes are unchanged.
- The cognitive-errors CLI item now carries the refusal's text: `unable  <name>  instance-unreachable` on stderr, then `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` the same document with stderr empty and no hint.
**Why:** the chunk built the check: each of the six verbs goes through one shared connect that compares the serving process's pid and start time against the snapshot before its first frame, and `send_hint`'s new cause `server-verify` prints that hint, the design-system cli-send string byte for byte; a mismatch or an unmakeable check is exit 21 with the unchanged document. macOS stays on the recorded gap, its euid and socket-directory checks owned by "Unix endpoint and home hardening".
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the plugin against an impostor wrapper writes no human-facing byte
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new scope-entities group after the TUI passthrough group)
**Change:**
- New entity detail, the first for the not-assertable plugin entity: the Claude Code plugin against an endpoint whose serving process is not the recorded wrapper — the `viola hook` event and dialog calls and the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`), off macOS (no peer pid is read there and the check reads as passed: the recorded gap).
- Source: `src/cmd/hook.rs` (`connect`, `NotSent`) and `crates/viola-mcp/src/tools.rs` (`server_verified`); `tests/hook_fail_open.rs` (the impostor rows) and `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`).
- Assertability: not-assertable. A hook event or dialog call that meets such a server sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s; an MCP frame tool returns `isError` with the code `instance-unreachable` only (text `refused: instance-unreachable`) and sends no frame. Neither writes a human-facing byte, so no human keystroke is touched.
**Why:** the chunk gave the hook one shared connect, used by the event and the dialog paths, whose failed server check is the `NotSent` detail `server-verify-failed` with no frame, and the MCP `frame` a `server_verified` step that returns the unreachable result with no frame; the server-identity half of the hook and MCP dated gaps is retired on Linux and Windows, measured by the impostor rows of `hook_fail_open` and `binary(handlers)`.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-verify refusal's literal pin and its two-leg reach
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the cognitive-errors verification, its cli item and a new cli, macOS item; a11y-plan.md: the Error recovery Verification line)
**Change:**
- The cli verification item now names the six frame verbs' exit-21 server-verify refusal as pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs`, whose `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reads, over the six verbs in both modes, the hint as the last stderr line with no ESC byte and stdout empty, or under `--json` the exact document with stderr empty, and no output holding the recorded pid, the home's path or the endpoint, beside its control `security_negative_cli_verb_reaches_the_recorded_wrapper`.
- Its OS reach: was «These run on all three OS legs»; now all three legs except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only. The body's Verification line now adds the same exception (macOS reports no peer pid).
- New cli, macOS item: the refusal cannot occur there (no peer pid; the check reads as passed, the recorded gap), so its cases compile out; its macOS witness is owed to "Unix endpoint and home hardening".
**Why:** the refusal landed in a new security-negatives binary rather than a trycmd case, gated off macOS, green on CI ubuntu-latest and windows-2025. The macOS server-verification cases joined "Unix endpoint and home hardening" as one CARRY on P5 review card 1's word — delegate the Viola overseer, founder-delegated.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-verify hint among the literal-pinned strings
**Section:** §8 Cognitive Accessibility › Plain language (cognitive-accessibility.md: the cognitive-plain-language verification; a11y-plan.md: the Plain language Verification line)
**Change:**
- The literal-pinned exceptions to trycmd now add the server-verify hint (`send_hint`'s cause `server-verify`, `src/human/hints.rs`), pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs`.
- The body: was «by literal» for `viola list`, `viola --help` and `viola plugin install`; now for those three and the server-verify hint.
**Why:** the chunk added a fixed hint string and pinned it by literal, adding no trycmd case, so the claim that trycmd pins every fixed CLI string outside the named exceptions needed the new exception named.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-explicit-session-links — the eight CLI frame verbs' server-verify refusal
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the CLI verbs entity); §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the cognitive-errors CLI item and its cli verification item)
**Change:**
- The CLI verbs entity's server-verify sentence: was `send`, `wait`, `last`, `answer`, `pause` and `release` (per 2026-10-26-server-verification-before-any-frame — the six CLI frame verbs' server-verify refusal); now `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper, the rest of the sentence unchanged.
- The cognitive-errors CLI item: the same eight verbs now refuse it with exit 21, the `unable  <name>  instance-unreachable` line and the server-verify `hint:` last, or the `--json` document with stderr empty and no hint.
- The cli verification item: was "the six frame verbs' exit-21 server-verify refusal", read over the six verbs (per 2026-10-26-server-verification-before-any-frame — the server-verify refusal's literal pin and its two-leg reach); now "the eight frame verbs'", `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reading `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` in both modes; its macOS gating unchanged.
**Why:** the chunk made the CLI frame verbs eight: `link` and `unlink` take the liveness pre-check and the server check through `cmd::client::connect` before their frame, and a server that is not the recorded wrapper gets no frame, exit 21 `instance-unreachable` with the server-verify hint; the identity table in `tests/security_negatives_server_identity.rs` grew to eight verbs in both modes plus the controls.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' entity and the link line's escaped since
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new scope-entities group after the CLI verbs entity; a new scope-surfaces group before the tui surface)
**Change:**
- New entity: `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, the verb's instance the driven one, in the human TTY, plain and `--json` modes: each prints one plain-ASCII result line in one write with no SGR in any mode, or under `--json` one `{"v":1,"ok":<ok>}` document; each refuses with exit 21 a driven instance that fails the strict-modes check (before any log opens), a stopped driven instance and a server that is not the recorded wrapper; a pair the wrapper refuses as invalid params reaches it as a fault, exit 20, no hint. Source: `src/cmd/link.rs` (`link_call`), `src/cmd/unlink.rs`, `src/human/verbs.rs` (`write_linked`, `write_already_linked`, `write_unlinked`); `tests/cli_links.rs`. Assertability: assertable, output-discipline assertions on text and streams only.
- New cli surface detail: a `viola link` line whose `since` is no RFC 3339 instant (the restore's `unknown`, or a value from a hand-altered log) prints it through `escape_message`, which keeps `\n`, `\t` and non-ASCII; the wrapper's own `since` is a link line's `ts` or `unknown`, so such a byte reaches the line only from a hand-altered log.
**Why:** the chunk built the two verbs and the wrapper's `link` / `unlink` methods, in the form the founder named himself in his own answer to this chunk's P4 fork, live by dialog, relayed verbatim by the operator (inputs#I3): the instance check first, then the liveness pre-check and the server check like `send`; `link_since` prints a `since` that is no RFC 3339 instant through `escape_message`.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the `handoff:` help row built
**Section:** §1 A11y Scope Summary (a11y-plan.md: critical path 6, Required ARIA roles; a11y-scope-summary.md: path-p6-board); §4 ARIA Patterns & Roles (aria-patterns-roles.md: the cli landmark row)
**Change:**
- a11y-plan.md path 6: was "the `--help` group labels, whose `handoff:` row is owed to "Explicit session links""; now "the `--help` group labels", the owed clause gone.
- path-p6-board: was "board / traffic / wheel / setup over the served verbs; the `handoff:` row joins with "Explicit session links""; now board / traffic / wheel / handoff / setup over the served verbs, the `handoff:` row `handoff:  link, unlink` between `wheel:` and `setup:`.
- The cli landmark row: was the rows `board:` · `traffic:` · `wheel:` · `setup:`, the `handoff:` row owned by "Explicit session links" (per 2026-10-17-cli-output-discipline — the colour spans built and checked; --help group labels built, and 2026-10-17-optional-plugin-install — plugin install built: setup: row, verb entity, pinned lines); now `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:`, the `handoff:` row `handoff:  link, unlink` between `wheel:` and `setup:`, held by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`.
**Why:** `viola --help` gained the row `handoff:  link, unlink` between `wheel:` and `setup:`, met by the two help cases; the chunk retired the hold on the explicit verbs and every master line naming "Explicit session links" as the row's pending owner.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' refusals in Error recovery
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: three new cognitive-errors values groups and a new cognitive-errors procedure group)
**Change:**
- Strict-modes: a driven instance that fails the strict-modes check is refused with exit 21 before any log opens, no log line written: the fixed instance-state `unable` / `hint:` pair on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint.
- Stopped instance: a driven instance with no live endpoint is refused with exit 21, the not-running `hint:` as the last stderr line.
- Wrapper refusal: a pair the wrapper refuses as invalid params (`-32602`, e.g. `driver` equal to `driven`) reaches the verb as a fault, exit 20, no hint; the verbs never refuse it client-side.
- Verification, in `tests/cli_links.rs`: `link_verbs_against_a_stopped_instance_exit_21` reads the stopped-instance refusal, and `link_verbs_refuse_a_widened_instance_before_any_log` the strict-modes refusal (a group-writable driven instance refused before any log, its mode read back as found; `cfg(unix)`, so ubuntu-latest and macos-latest only).
**Why:** `link_call` runs `check_instance` before `client::start`, then the liveness pre-check through `client::unreachable`, and sends a wrapper error, `-32602` included, through `client::fault`; the check-first order is the founder's own answer to this chunk's P4 fork, live by dialog, relayed verbatim by the operator (inputs#I3).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' result lines among the literal-pinned strings
**Section:** §8 Cognitive Accessibility › Plain language (cognitive-accessibility.md: two new cognitive-plain-language procedure groups; a11y-plan.md: the Plain language Verification line)
**Change:**
- New verification: the three result lines (`write_linked`, `write_already_linked`, `write_unlinked`; their text in design-system §Surface: cli), each one write with no SGR in any mode, pinned by literal in the unit `link_verb_lines_are_the_design_lines_in_one_write` (`src/human/verbs.rs`) and the root case `link_verbs_print_the_design_lines` (`tests/cli_links.rs`).
- New: the `link` line's time is the reply's `since` as UTC clock time in whole seconds with `Z`, pinned by the unit `link_since_is_the_utc_time_alone`; the names each line prints are the verb's own parsed arguments, never names read back from the reply or the log.
- a11y-plan.md: was "by literal" for `viola list`, `viola --help`, `viola plugin install` and the server-verify hint (per 2026-10-26-server-verification-before-any-frame — the server-verify hint among the literal-pinned strings); now for those and the `viola link` / `viola unlink` result lines.
**Why:** the chunk added three fixed result lines, each one `write_all` and plain in every mode, and pinned them by literal with no trycmd case, so the named exceptions to trycmd needed them added.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the CLI verbs' strict-modes refusals as scope entities
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: two new `values · scope-entities` blocks after the `link` / `unlink` entity)
**Change:**
- New assertable entity: `viola send`, `wait`, `last`, `answer`, `pause` and `release` against an instance that fails the strict-modes check, in the human TTY, plain and `--json` modes, refuse before any frame or log (`send` after its text read) with exit 21, stdout empty and `link`'s instance-state pair on stderr, `hint:` last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; no line written, the mode or ACL left as found, on all three OSes. Source: `src/cmd/client.rs` (`refuse_untrusted`), the six verbs' files, `frame_verb_refuses_a_widened_instance_before_any_log`.
- New assertable entity: `viola run`, a `viola revive` start and `viola verify` over a home tree that fails the check refuse before any log opens with exit 1, stdout empty, the home-state pair on stderr, `hint:` last, and start no child; `run` and revive's start print the instance-state pair for an instance-level refusal, revive under `--json` gives its `strict-modes-failed` document; `verify` prints no step line and no summary, and its `update_stamps` refusal gives the same home-state pair. Source: `src/cmd/run.rs` (`checked_exe`, `refuse_tree`), `src/cmd/revive.rs`, `src/cmd/verify.rs` (`refuse_home`), and `run_refuses_a_widened_home_tree_before_its_log`, `verify_refuses_a_widened_ledger_before_any_step`, `verify_under_a_regular_file_refuses_the_home`, `a_home_another_user_may_read_refuses_run`.
- Each is assertable by output-discipline assertions on text and streams only; no automated a11y tool reaches a terminal; each points its refusal text to Cognitive Accessibility › Error recovery.
**Why:** the chunk put the strict-modes check first in the six frame verbs, `run`, revive's start and `verify`, and the file had recorded the strict-modes refusal for `link` / `unlink` only; the six verbs print `link`'s instance pair and a refused check writes no line, as `link`, by the founder's own answers at P4 (`inputs#I3`, live by dialog, relayed by the overseer); green on all three CI legs.
**Kept:** the CLI-verbs entity's «the refusal cannot occur» on macOS stands: it is the server-verify refusal's (no peer pid read), a different fact from the strict-modes refusal, which runs on all three OSes.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the plugin against a strict-modes-refused instance, not assertable
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new `values · scope-entities` block after the impostor-server plugin entity)
**Change:**
- New not-assertable entity: the Claude Code plugin against an instance that fails the strict-modes check — the `viola hook` event and dialog calls and the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`). Source: `src/cmd/hook.rs` (`canonical`, `trusted`), `crates/viola-mcp/src/tools.rs` (`frame`), `hook_refused_by_the_instance_check_fails_open_and_writes_nothing`, `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`.
- Its reason: a refused hook call returns exit 0 at once with empty stdout and stderr, under 1.0 s — no `config.json` read, no stdin read, no frame, no line, no SessionEnd append — and a `VIOLA_DIR` naming no existing instance directory writes nothing; an MCP frame tool returns `isError` with the code `instance-unreachable` only (`detail: null`), no frame, no line. Neither writes a human-facing byte, and no human keystroke is touched.
**Why:** the chunk runs the instance check on the hook's event and dialog paths before obs init and in the MCP `frame` before `live_endpoint`, with no line on a refused check by the founder's own answer at P4 (`inputs#I3`, relayed by the overseer); a `VIOLA_DIR` naming no existing directory canonicalises to no session. The file held only the impostor-server form of this entity, which stands beside it.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — an unstattable home, per OS
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the `run` / revive start / `verify` entity's last clause); §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: a new `values · cognitive-errors` entity, and the strict-modes verification's per-OS oracle)
**Change:**
- A home path under a regular file is a strict-modes refusal with its hint on Unix (`verify`: the home-state pair, exit 1; the six frame verbs: the instance-state pair, exit 21).
- On Windows such a path stats `NotFound`, the check skips it as absent, and the `cli` catch site's fault line stands, with no hint.
- `verify_under_a_regular_file_refuses_the_home` (`tests/cli_verify.rs`) reads it through an oracle per OS: the home-state pair on Unix, the catch-site line on Windows.
**Why:** measured on Windows (a CI red of this chunk's own test): a path under a regular file stats `NotFound`, so a strict-modes check skips it as absent; the test's oracle was made per OS after it, and a per-OS oracle is owed wherever a test plants a regular file above a checked path.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the strict-modes refusals' text
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: three new `values · cognitive-errors` entities ahead of `link` / `unlink`'s stopped-instance entity)
**Change:**
- CLI `send` / `wait` / `last` / `answer` / `pause` / `release`: an instance that fails the strict-modes check is refused with exit 21 before any frame or log (`send` after its text read), no line written: the same fixed instance-state `unable` / `hint:` pair as `viola link` / `viola unlink` on stderr, `hint:` last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty, no hint.
- CLI `viola run` / `viola revive` start: a home tree that fails the check (the home, the instance's files, `ledger/`, `bin/`, `plugin/`, the pinned copy and its plugin folders) is refused with exit 1 before any log opens, no line, no child: the home-state pair for a home-level refusal, the instance-state pair for an instance-level one, `hint:` last, stdout empty; under `--json` `revive` gives its `strict-modes-failed` document.
- CLI `viola verify`: a home whose tree (the home, `ledger/`, `bin/`, `plugin/`) fails the check is refused with exit 1 before its log opens, no step line, no summary, no line: the home-state pair, `hint:` last; a `ledger/` that `update_stamps`'s own check refuses gives the same pair.
**Why:** the chunk's `refuse_untrusted` (the six verbs), `checked_exe` / `refuse_tree` (`run`, revive's start) and `refuse_home` (`verify`, and `StampsError::StrictModes` from `update_stamps`) print these fixed pairs through `human::refuse`; the six verbs take `link`'s instance pair and a refused check writes no line by the founder's own answers at P4 (`inputs#I3`, live by dialog, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the strict-modes refusals' witnesses and pinned pairs
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: a new `procedure · cognitive-errors` block after `link` / `unlink`'s verification) and › Plain language (its Verification pinned-string list)
**Change:**
- New verification: `frame_verb_refuses_a_widened_instance_before_any_log` (`tests/security_negatives_strict_modes.rs`) reads, over the six frame verbs in both modes against a live wrapper whose instance directory another user may write, exit 21 with the `INSTANCE_PAIR` literal on stderr and stdout empty, or under `--json` the exact `instance-unreachable` document with stderr empty, no line added, no frame sent, the widening left as found; `run_refuses_a_widened_home_tree_before_its_log` (`tests/run_cli.rs`) reads `run`'s exit 1 with the `HOME_PAIR` or `INSTANCE_PAIR` literal, stdout empty, no line, no child; `verify_refuses_a_widened_ledger_before_any_step` and `verify_under_a_regular_file_refuses_the_home` (`tests/cli_verify.rs`) read `verify`'s home-state pair with no step line or summary. Unlike the server-verify cases, these run on all three OS legs.
- The pinned-string list: was ending at the server-verify hint's `HINT` literal; now also the instance-state and home-state pairs as the six frame verbs and `run` print them, pinned by the `INSTANCE_PAIR` / `HOME_PAIR` literals in `tests/security_negatives_strict_modes.rs` and `tests/run_cli.rs` (`verify`'s home-state pair by literal in `tests/cli_verify.rs`).
**Why:** the chunk's new strict-modes witnesses and their pair literals, green on all three CI legs.
**Kept:** the master `a11y-plan.md` is unchanged: its §8 Error recovery Verification still names the frame verbs' server-verify refusal as windows-2025 and ubuntu-latest only, and its Plain language Verification line names no state pair; the detail file's macOS «the server-verify refusal cannot occur» stands (a different fact).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the tui passthrough boundary after a contained vt100 feed panic
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new scope Entity after the `viola run` TUI passthrough Entity; a11y-plan.md: the `viola run` TUI passthrough Entity line)
**Change:**
- a11y-scope-summary.md gains the Entity `viola run` TUI passthrough after a contained vt100 feed panic (the boundary of the entity above), boundary-only, its Source `tests/chaos_feed_panic.rs` (`chaos_feed_panic_refuses_send_and_keeps_passthrough`). Its Reason: the driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, on all three OSes; on Unix the child's own bytes (`e4 b8 ad`) reach it.
- The body's tui passthrough Entity line ended at "do not count as human editing"; it now goes on: the boundary still holds after a contained vt100 feed panic (no viola literal on the outer terminal on every OS, the child's own bytes on Unix).
**Why:** the report records the case as new coverage: the outer-terminal half of the feed-panic case is witnessed, the bytes on Unix and the literal absence on every OS. Its (a11y) outcome is met, green on ubuntu, macOS and windows-2025. The report's Expected amendments name a11y-plan §1's two tui Entity sites for the witness.
**Kept:** §4 ARIA Patterns & Roles (aria-patterns-roles.md, the P4 tui bullet's clause (2)) is unchanged. The case's `send` refusal and its typed key reaching the child landed at chunk 2026-10-04, and this chunk adds only the outer-terminal half. §3's keyboard test harness key is unchanged too, as the report names it no-change.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the Windows byte reading after a feed panic, owed
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new scope-entities Owed line after the feed-panic Entity; a11y-plan.md: the `viola run` TUI passthrough Entity line)
**Change:**
- a11y-scope-summary.md gains an Owed line: the Windows outer-terminal reading of the child's own bytes (`e4 b8 ad`) after a contained vt100 feed panic. ConPTY re-renders a wide character on a 1-column screen, so windows-2025 reads only viola's literal absence. The reading is owed to "Windows-only live measurements".
- The body's tui passthrough Entity line now ends with the same debt: its Windows byte reading is owed to "Windows-only live measurements".
**Why:** the report finds no Windows byte oracle for the panicking wide character assertable here: ConPTY re-renders the child's output, and a wide character on a 1-column screen has no guaranteed rendering. The case's byte reading is `unrunnable-here` on Windows. The reading is the CARRY on «Windows-only live measurements» in the P5 card's Beyond-the-plan row. That card was approved by the word of delegate overseer1 (founder-delegated), relayed by the operator, and it stays PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-31-sanitised-error-surfaces — the `--json` failure documents' witness
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: scope-entities, CLI verbs; scope-surfaces, cli) · §8 Error recovery (cognitive-accessibility.md: cognitive-errors, verification)
**Change:**
- CLI verbs gains the sub-item Failures under `--json`: each failure `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives is one document, a literal exit, empty stderr, no ESC byte and a clean body.
- The cli no-ESC rule: was the `viola revive --list` rows and the revive refusal pairs; now those and each `--json` failure document that case drives.
- The cognitive-errors verification gains the item `--json` failure documents: that case holds each failure it drives to one document, a literal exit, empty stderr, no ESC byte and a clean body.
**Why:** The chunk added this case, and its report reads each failure the case drives as one document, a literal exit, empty stderr, no ESC and clean. The no-ESC rule stands and gains a witness.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `--json`'s one document and its exit-2 exception
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: scope-entities, CLI verbs) · §8 Error recovery (a11y-plan.md; cognitive-accessibility.md: cognitive-errors)
**Change:**
- The `--json` rule: was one document with no hint, without exception; now one document with no hint, except the `viola answer` and `viola send` exit-2 paths (a11y-plan.md: a fixed stderr line, no document; cognitive-errors: "below").
- cognitive-errors gains the item CLI, the exception to one document: those paths write the fixed `UNUSABLE_RESPONSE` or `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document; `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) names them.
- The CLI verbs sub-item Failures under `--json` carries the same: those paths write a fixed stderr line and no document, the text at Cognitive Accessibility › Error recovery.
**Why:** Measured: the exit-2 paths write that fixed line to stderr whatever `--json` says, and no document, so a one-document rule without exception did not hold. The new item read as a contradiction of the standing one-document sentence and was escalated; the overseer, as the founder's delegate, answered apply: PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `unknown`'s detail, a code word or nothing
**Section:** §8 Error recovery (a11y-plan.md; cognitive-accessibility.md: cognitive-errors) · §11 Cognitive (a11y-plan.md)
**Change:**
- §8 Pattern: was `unknown` (exit 14) "whose detail is opaque"; now "whose detail is a code word or nothing, opaque to a reader".
- cognitive-errors: was "its detail is opaque, so design prints no hint rather than a guess"; now "its detail is a code word or nothing, opaque to a reader, so design prints no hint rather than a guess".
- §11's hint ban: was `unknown` (exit 14, opaque detail, design cli); now `unknown` (exit 14, its detail a code word or nothing, design cli).
**Why:** A string `detail` that is no code word now reads nothing, so `unknown`'s detail can no longer be free text.
**Kept:** `unknown` still prints no hint, and its detail stays opaque to a reader: design's reason for the carve-out stands.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — a peer's refusal screened to a code word
**Section:** §8 Error recovery (cognitive-accessibility.md: cognitive-errors)
**Change:**
- Gains the item CLI, a peer's refusal: `reply_of` and `of_reply` screen a peer's reply with `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first); a `refusal` that is no code word reads `unknown`, and a string `detail` that is no code word reads nothing (null under `--json` and on MCP).
- Gains the item CLI, a word outside the list: a code word outside the closed refusal list is visible by design, in the document and in `refused: <word>`, and takes `unknown`'s exit 14.
**Why:** The chunk's contract is that a peer's `message`, `data`, `refusal` or `detail` reaches no document or text; a code word outside the closed list stays visible by design, its exit `unknown`'s 14.
**Kept:** §8 Plain language and §11 Cognitive still tie a kebab-case code to the plain word `unable`: that is the human form, `refused: <word>` is MCP's text, so neither moved.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — a fault's body and the `wrapper fault` literal
**Section:** §8 Error recovery (cognitive-accessibility.md: cognitive-errors) · §8 Plain language (cognitive-accessibility.md: cognitive-plain-language, verification)
**Change:**
- cognitive-errors gains the item CLI, a fault's body: under `--json` and on MCP a fault's body is read through `viola_channel::sanitised_fault`; a code outside the five known reads the fixed `message` `wrapper fault` with null `data`. The human line is `error: wrapper fault  <code>`.
- The plain-language verification gains the pin: the `message` literal `wrapper fault` under `--json` and on MCP, for a code outside the five known, is pinned by the unit case `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` (`crates/viola-channel/src/lib.rs`).
**Why:** `sanitised_fault` replaces the two private `fault_detail` copies, both gone, and its `wrapper fault` is a new fixed output word on `--json` and MCP, so it takes a literal pin like the entity's other fixed strings.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the census of error writers and the body check
**Section:** §8 Error recovery (cognitive-accessibility.md: cognitive-errors, verification)
**Change:**
- Gains the item the census of error writers: in `tests/contract_error_bodies.rs`, `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds each error body to a writer and a witness that stand, `product_error_writers_are_each_named` holds each product error writer to a name, and `thiserror_messages_interpolate_nothing` holds each `thiserror` message to no interpolation.
- Gains the item the body check: `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes.
**Why:** The planned after-every-test error-body check was measured absent and is not built; the three surface witnesses, the enumeration and the census of writers replace it, reading this check.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-exit-cause-code-catalogue — `viola revive --json` documents every refusal of its start
**Section:** §1 A11y Scope Summary (a11y-plan.md: the Owed line; a11y-scope-summary.md: the CLI verbs entity, the `viola run` TUI passthrough entity) · §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the CLI pattern)
**Change:**
- The CLI verbs entity: each exit-1 refusal was of `revive`'s strict-modes check or its preflight; now of its strict-modes check, its preflight or the start it shares with `run`.
- Its codes were five, `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; now ten, adding `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`.
- The clause that a refusal `run`'s start makes after a passed preflight prints no document, owed to "Exit-cause code catalogue", is removed; the master's **Owed:** line is replaced by the standing **Under `viola revive --json`:** rule.
- New under the entity: `batch-script-child`, `pinned-hash-mismatch` and `squatted-name` each print one exit-1 document in place of the human pair, before the child spawns, and without `--json` each prints its pair; a `claude` not found at a start reads as the `internal-error` document and has no human pair.
- New in the TUI passthrough boundary and the master's rule: a `pump_child` error prints no document, the child having held the terminal, and viola writes nothing past it.
- The CLI pattern: its strict-modes and preflight refusals gave one document; now every refusal of the start it shares with `run` does, its witnesses `revive_json_*` and now the start rows in `tests/cross_exit_causes.rs`.
**Why:** The post-preflight gap, a carry of chunk 2026-10-17-cli-machine-contract, is closed: `revive` now prints one exit-1 document for each refusal of that start, and the sentence that it prints none stopped being true. `run`'s start reaching a driver through `revive --json` is inside the founder's own word, live, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q4). The overseer as delegate read these rows as inside his answers and answered their check-7 rows apply, Q7, the companion edits removing the standing clause (`inputs#I6`, PROVISIONAL); the ratification stays the founder's.
**Kept:** The exit-1 document's `detail` stays null, its code in `error`.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — exit 21's `--json` document names its cause
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the CLI verbs entity, the frame verbs' strict-modes entity) · §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the CLI pattern, the exit-21 entity, the two strict-modes entities, the two Verification items)
**Change:**
- New entity, **CLI, exit 21 under `--json`:** one document per cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` on stdout with stderr empty, no hint and no ESC, `<cause>` one of the closed words `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`.
- `detail` was null; now `strict-modes-failed` for `viola plugin install`, for `send` / `wait` / `last` / `answer` / `pause` / `release` and for `viola link` / `viola unlink`.
- `detail` was null; now `server-verify-failed` for the eight frame verbs' server-verify refusal, in both files.
- The CLI verbs entity's closing clause was "their `--json` codes and exit codes are unchanged"; now "their `error` word and exit codes are unchanged".
- Verification: the literal pinning `viola list`'s exit-21 document was `detail` null, now `strict-modes-failed`; the frame verbs' strict-modes oracle was "the exact `instance-unreachable` document", now the one whose `detail` is `strict-modes-failed`.
**Why:** Every cause used to print one document, so a same-user driver could not tell them apart. The cause in `detail`, with `error` staying `instance-unreachable` and `v` staying 1, is the founder's own word, live, through the overseer's dialog, given after this widening was shown to him (`inputs#I2` Q1). The overseer as delegate read these rows as inside that answer, and answered the new entity's check-7 row apply, Q7, the companion edits rewriting the null lines (`inputs#I6`, PROVISIONAL); the ratification stays the founder's.
**Kept:** No exit moved and the human pairs stand. The macOS server check stays the recorded gap, its witness owed to "Unix endpoint and home hardening", a carve-out of `inputs#I2` Q4.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — MCP's `isError` body names its cause
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the two Claude Code plugin entities, server-verify and strict-modes)
**Change:**
- A frame tool against a server that is not the recorded wrapper was `isError` with the code `instance-unreachable` only; now with that code and `detail` `server-verify-failed`, the text still `refused: instance-unreachable`.
- A frame tool against an instance failing the strict-modes check was the code only, `detail: null`; now the code and `detail` `strict-modes-failed`.
**Why:** MCP's body is the CLI's document less `v`, so its cause crosses with the CLI's: the founder's own word, live, through the overseer's dialog, given after this widening was shown to him (`inputs#I2` Q1). The overseer as delegate read these rows as inside that answer (`inputs#I6`, PROVISIONAL); the ratification stays the founder's.
**Kept:** Each refusal sends no frame and the strict-modes one writes no line, `isError` their record (`inputs#I2` Q4). Neither writes a human-facing byte, so both entities stay not-assertable; the hook calls still fail open.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a lost wrapper and a stale holder each read their own code
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: two new entities)
**Change:**
- New, **CLI, a lost or stale wrapper:** a wrapper lost with the call open is `endpoint-lost`; a frame verb against a stale instance reads `instance-dead`, stale heartbeat being exit 1's cause alone.
- New, **CLI, a start against a holder:** exit 1 tells a stale holder from a live one, `stale-heartbeat` for a `stale` holder, `already-live` for a live holder and a fresh replayed beat.
**Why:** `already-live` as a stale holder's refusal is retired: the start now prints and logs `stale-heartbeat` for one. A frame verb finds no live endpoint for a stale wrapper, so it still reads `instance-dead`; a wrapper lost with the call open no longer reads `instance-dead` but `endpoint-lost`.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `cli` catch site's `internal-error` document
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the unstattable-home entity, a new entity)
**Change:**
- New, **CLI, a fault under `--json`:** where a CLI verb prints `error: internal error`, under `--json` it prints the `internal-error` document with a null `detail` on stdout in that line's place, stderr empty; without `--json` the line stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent there.
- The unstattable home on Windows was the catch site's fault line with no hint; now also, under `--json`, a frame verb there prints the `internal-error` document on stdout in that line's place, stderr empty (`verify` takes no `--json`).
**Why:** The catch site's document under `--json`, in place of its stderr line with exit 1 and the human line staying, is the founder's own word, live, through the overseer's dialog, given after this widening was shown to him (`inputs#I2` Q1). The overseer as delegate read the unstattable-home row as inside that answer (`inputs#I6`, PROVISIONAL); the ratification stays the founder's.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a name with no endpoint reads the instance directory
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the `viola link` / `viola unlink` stopped-instance entity, a new entity)
**Change:**
- New, **CLI, a name with no endpoint:** the refusal reads the instance directory: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) is `unwrapped-name`, and the strict-modes check stays first.
- `viola link` / `viola unlink` was: a driven instance with no live endpoint is refused with exit 21, the not-running `hint:` last. Now that holds where its instance directory stands; a driven name with no live endpoint and no such directory reads `unwrapped-name`, its own `hint:` last.
**Why:** The no-endpoint arms now take their cause from one reading of the directory, which opens nothing and writes no line. The reading is `inputs#I2` Q2, the word of the delegate, the overseer, PROVISIONAL. This body carries no mark: the one gap marker stands at its owner in architecture, and this entry carries the word.
**Kept:** `not-running` stays the hint's key, not renamed to the log's `instance-dead`.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `unwrapped-name` and `endpoint-lost` hints and their pins
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: a new entity) · §8 Cognitive Accessibility › Plain language (cognitive-accessibility.md: its Verification; a11y-plan.md: the Verification line)
**Change:**
- New, **CLI, the `unwrapped-name` and `endpoint-lost` human forms:** each ends with its own `hint:` as the last stderr line, in plain ASCII, the name in a hint being the parsed `ViolaName`.
- New under Plain language's Verification: both hints (`src/human/hints.rs`) are pinned by the unit `send_hint_is_the_design_string` and by the exit-cause table (`tests/cross_exit_causes.rs`).
- The master's literal-pinned list was "the server-verify hint"; now "the server-verify, `unwrapped-name` and `endpoint-lost` hints".
**Why:** The chunk built the `unwrapped-name` hint and added the `endpoint-lost` hint as new text, so a copy change to either now fails CI until the pinned string is updated on purpose. The `endpoint-lost` hint is `inputs#I2` Q3, the word of the delegate, the overseer, PROVISIONAL. This body carries no mark: the one gap marker stands at its owner in design-system, and this entry carries the word.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the exit-cause table and one hint per code
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: two new Verification items)
**Change:**
- New, **Verification, one hint per code:** in `tests/cross_exit_causes.rs`, `no_two_codes_share_a_hint`, with its control `shared_hints_reads_a_planted_duplicate`, holds each code to its own hint, and `no_hint_names_release` holds every hint from naming `release`.
- New, **Verification, the exit-cause table:** `tests/cross_exit_causes.rs`, a `fake-agent` E2E binary, is the exit-cause table: its rows check each cause's exit, document and hint in human and `--json` modes, and the comment at its top names the causes held outside the table.
**Why:** The chunk built the table, a new root `cross_` binary requiring the `fake-agent` feature, and retired the "not built" lines of the exit-cause matrix. Its `no_two_codes_share_a_hint`, the control and the no-`release` rule read green.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
