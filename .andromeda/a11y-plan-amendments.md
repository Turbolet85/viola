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
