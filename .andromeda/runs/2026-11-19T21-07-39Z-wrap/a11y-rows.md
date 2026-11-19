# a11y-plan · the rows 2026-11-19-context-as-a-local-command touches — picked 6 of 235 · rule any · pin 3188

## 3-a11y-assertion-harness-contract · ## 3. A11y Assertion Harness Contract

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal. · r1
decision/tui-boundary-driver-pin · TUI boundary driver — The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins. · r1

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. A11y Assertion Harness Contract
decision/a11y-failures-are-nextest-failures · a11y failures are nextest failures — An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own. · r1
decision/a11y-runs-in-existing-test-suites · a11y runs in the existing test suites — The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own. · r1
decision/manual-screen-reader-pass · Manual screen reader pass — The screen reader check is a manual Orca pass owned by the founder, with no automated test spec. · r1
absence/rendered-page — A rendered page, and with it any browser a11y tooling · r1

## ci-integration · key of ## 3. A11y Assertion Harness Contract
decision/gate-requires-coverage-doctest · Gate requires coverage and doctest — The test job runs the harness and then gate --require coverage,doctest, with skips never allowed. · r1

## contrast-verification-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## focus-management-test-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/boundary-check-never-inspects-claude-content · Boundary check never inspects claude content — The keyboard boundary check never inspects claude content. · r1 · contradict
decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case. · r1
decision/human-key-takes-the-wheel · Human key takes the wheel — A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed. · guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins · r1 · add-to · F1
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating. · r1 · add-to · F1
decision/stdin-classifier-non-editing-list · Closed non-editing stdin list — Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel. · why: ratified, the exact grammars chosen after a narrower option was shown · r1
decision/zero-viola-bytes-on-the-terminal · Zero viola bytes on the terminal — The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run. · r1

## screen-reader-test-pattern · key of ## 3. A11y Assertion Harness Contract
decision/cli-screen-reader-test-setup · CLI screen-reader test setup — The cli surface is screen-reader tested with Orca on GNOME Terminal. · r1
decision/tui-screen-reader-test-setup · TUI screen-reader test setup — The tui surface is screen-reader tested with Orca on foot under Hyprland. · r1

## structured-violation-json-schema · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.
contract/structured-violation-json/cli-tui-a11y-rows · Structured violation JSON · a11y rows for cli and tui — The cli and tui surfaces emit no a11y rows. · r1
contract/structured-violation-json/suites-failures · Structured violation JSON · suites[].failures[] — Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[]. · r1

## wcag-criteria-mapping · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose · r1
decision/fixed-copy-from-design · Fixed CLI copy comes from design — The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design. · r1
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text. · r1
decision/no-reading-level-gate · No reading-level gate — No reading-level target is gated and no readability scorer is introduced. · why: SC 3.1.5 Reading Level (AAA) is not claimed, and no readability scorer is in a11y-research · r1
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`. · r1 · contradict
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest · r1
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it. · r4
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code. · r1
contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint. · r1
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths, which print a fixed stderr line and no document. · r1
contract/cli-refusal/unknown-refusal · CLI refusal output · the `unknown` refusal — The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing. · r1
contract/viola-list-caption/order · viola list caption · order — The caption order is `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI`, pinned by the `CAPTION` literal. · r1
absence/user-time-limit — A time limit on the user, and any timeout-extension component · r1

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/claude-agents-read-fails-open · claude agents read fails open — A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's preflight. · r1
decision/client-euid-refusal-test-strategy · Client euid refusal test strategy — No foreign peer is driven: the client euid refusal's decision is tested over injected euids and its real read on Linux, and each client seam's wiring is read only by forced refusals in remove-the-guard runs. · r1
decision/dialog-hold-bounds-the-driver · Dialog hold bounds the driver — A raised dialog takes viola answer only within the dialog hold; at its end the dialog renders for the human in the claude TUI, and any human key hands a held dialog back at once whatever the hold. · r2
decision/error-bodies-have-writer-and-witness · Error bodies have a writer and a witness — Each error body names a writer and a witness that stand, and each product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand · r1
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists. · r1
decision/faults-carry-no-hint · Faults carry no hint — Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint. · r1
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose. · r1
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text · r1
decision/json-one-document-no-hint · One --json document with no hint — Under --json a failure gives one document with no hint. · except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document · r1
decision/link-pair-validated-by-wrapper · Link pairs validated by the wrapper — viola link and viola unlink never refuse a pair client-side; a pair the wrapper refuses as invalid params reaches the verb as a fault, exit 20, no hint. · r1
decision/link-result-lines-one-write-no-sgr · Link result lines in one write, no SGR — The link and unlink result lines are each one write with no SGR in any mode. · guard: link_verb_lines_are_the_design_lines_in_one_write · r1
decision/no-hint-names-release · No hint names release — No hint names the release verb. · guard: no_hint_names_release · r1
decision/one-hint-per-code · One hint per code — No two codes share a hint. · guard: no_two_codes_share_a_hint · r1
decision/peer-replies-screened-to-code-words · Peer replies screened to code words — A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP). · r1
decision/printed-names-are-parsed-arguments · Printed names are the parsed arguments — Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log. · r1
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start. · r1
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify. · r1
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — No thiserror message interpolates a value. · guard: thiserror_messages_interpolate_nothing · r1
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel. · r4
decision/unknown-refusal-has-no-hint · unknown refusal carries no hint — The unknown refusal prints no hint. · why: its detail is a code word or nothing, opaque to a reader, so no hint is printed rather than a guess · r1
decision/unlisted-code-words-stay-visible · Unlisted code words stay visible — A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14. · why: visible by design · r1
decision/unstattable-home-is-strict-modes-refusal · Unstattable home is a strict-modes refusal — A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21. · r1
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was budget-paused) and never reused; an older wrapper's budget-paused refusal reads as unknown, exit 14. · r2
contract/cli-exit-codes/13 · CLI exit codes · 13 — An answer for an expired or absent dialog is refused not-delivered / unknown-dialog. · r1
contract/cli-exit-codes/14 · CLI exit codes · 14 — unknown: a refusal whose word is outside the closed refusal list. · r1
contract/cli-exit-codes/2 · CLI exit codes · 2 — A usage error, or an unusable answer response or send text written as one fixed stderr line with no document. · r1
contract/cli-exit-codes/20 · CLI exit codes · 20 — A fault, with no hint. · r1
contract/cli-exit-codes/21 · CLI exit codes · 21 — instance-unreachable: the frame verb was refused before reaching the instance, the cause in detail. · r1
contract/cli-fixed-message/form · CLI fixed-message refusal · form — viola revive's exit-1 refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last. · r1
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code. · r1
contract/code-word/format · code word · format — 1 to 64 bytes of [a-z0-9-], a lower-case letter first. · r1
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — Whole seconds 60 to 3600, default 60, read once at the wrapper's start; any other value gives 60 and one parse-rejected. · r1
contract/event-line/dialog-expired · the event line · dialog-expired — Appended once at the dialog hold's end with source wrapper and dialog_id, dialog_kind; wakes a parked wait. · r1
contract/fault-body/message · fault body under --json and on MCP · message — A code outside the five known reads the fixed message wrapper fault with null data. · r1
contract/human-fault-line/wrapper-fault · human fault line · wrapper fault — error: wrapper fault  <code>. · r1
contract/instance-unreachable-detail/endpoint-lost · instance-unreachable detail · endpoint-lost — The wrapper was lost with the call open. · r1
contract/instance-unreachable-detail/instance-dead · instance-unreachable detail · instance-dead — The name has no live endpoint but its instance directory stands, or the instance is stale. · r1
contract/instance-unreachable-detail/server-verify-failed · instance-unreachable detail · server-verify-failed — The process serving the endpoint is not the recorded wrapper and was not contacted. · r1
contract/instance-unreachable-detail/socket-dir-refused · instance-unreachable detail · socket-dir-refused — The socket directory was refused; the verb sent no frame. · r1
contract/instance-unreachable-detail/strict-modes-failed · instance-unreachable detail · strict-modes-failed — The instance failed the strict-modes check; refused before any frame or log, no log line written. · r1
contract/instance-unreachable-detail/unwrapped-name · instance-unreachable detail · unwrapped-name — The name has no live endpoint and no instance directory (absent or a regular file). · r2
contract/instance-unreachable-json/document · exit 21 under --json · document — One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC. · r1
contract/json-fault/internal-error · CLI fault under --json · internal-error — Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's start, mcp and hook stay silent there. · r1
contract/mcp-wait-tool/timeout-ms · MCP wait tool · timeout_ms — Clamped to 50 000 ms, which is also the default when none is passed, so a wait with no timeout is answered timed-out after 50 s. · r1
contract/send-refusal/input-not-ready · viola send refusal · input-not-ready — A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed. · r1
contract/send-refusal/no-prompt-submitted · viola send refusal · no-prompt-submitted — A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted. · r1 · stale
contract/start-holder-refusal/already-live · start against a holder, exit 1 · already-live — The holder is live, or a fresh replayed beat was read. · r1
contract/start-holder-refusal/stale-heartbeat · start against a holder, exit 1 · stale-heartbeat — The holder is stale; stale heartbeat is exit 1's cause alone. · r1
contract/start-json-refusal/document · run / revive start under --json · refusal document — Every refusal of the start revive shares with run gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr. · r1
contract/viola-link-line/time · viola link line · time — The reply's since as UTC clock time in whole seconds with Z. · r1
contract/viola-verify/interrupted-pair · viola verify · interrupted pair — unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI. · r1
contract/viola-wait/timed-out · viola wait · timed_out — viola wait returns timed_out true only for a timeout the caller passes in. · r1

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning. · r1
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed · r1
decision/list-captions-fixed · Board captions fixed by their literal — The caption set and order of `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) never drift from its `CAPTION` literal (SC 3.2.4). · r1
decision/machine-output-a11y-review · No human-only a11y review — No a11y review is human-only without machine output. · why: the canonical agent-runnable principle · r1
decision/manual-checks-supplemental-only · Manual checks are supplemental only — Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK. · r1
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured. · r1
decision/no-information-by-colour-alone · No information by colour alone — Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1). · r1
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · except: the test-only live rig reads the trust selector's place, the input box as empty, holds-text or absent, modal dialogs and, after a turn end, the box row's cell attributes, as closed words, never text · r4 · stale
decision/no-soft-a11y-slo · No soft a11y SLO — An a11y SLO is never soft: it triggers a build or deploy failure on its threshold. · r1
decision/no-spinners · No spinners — CLI output is static and linear, with no spinner or redraw. · why: design ban · r1
decision/no-stack-trace-only-error · No stack trace as the only error text — The CLI never renders a stack trace as the only error text. · r1
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1). · r1
decision/no-vendor-locked-a11y-tool · No vendor-locked a11y tool as only method — A vendor-locked a11y tool without machine output or API (Stark Visual Studio plugin, Tenable.io UI-only) is never the ONLY method. · r1
decision/non-editing-sequences · Terminal sequences are not human editing — Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4). · r1
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing · r1
