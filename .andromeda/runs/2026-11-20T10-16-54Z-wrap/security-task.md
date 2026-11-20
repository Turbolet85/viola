@task_prose part=session
The input-box readiness predicate no longer refuses on any row that merely holds a compiled modal literal; it refuses only on a standing modal.
A standing modal is a row holding a modal literal that is option-shaped (any indent, optional selector glyph, optional numbering, the literal, only spaces after) or that has no input-box row below it; footer literals never count as the box.
So a modal label quoted mid-sentence in a transcript row above the input box no longer refuses a send on an idle, verified session, while a real dialog still refuses with input-not-ready.
The one predicate still serves viola run's pre-send gate, verify's paste guard, the ledger post-condition and the test rig's box read; no signature, refusal word, detail, exit code, hint, log line, config key, dependency or stamp moved.
The fix is proved over the fake agent, recorded trust screens and named SYNTHETIC screens only; no live claude work; CI green 7/7 on the pushed head.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the Claude adapter's compiled screen signatures and the input-box readiness predicate read only against compiled literals, with its unit tests
@task_file n=2 path=src/run/gate.rs
what: viola run's pre-send readiness gate over the screen model (the verified and unverified CLI gate and its at-once modal refusal), with its unit tests
@task_file n=3 path=tests/support/fake.rs
what: test-only support for the fake agent, holding the synthetic screens the integration tests feed
@task_file n=4 path=tests/cli_send.rs
what: the integration tests of viola send against a verified CLI over the fake agent
@task_item n=1 part=touches
input-validation decision input-box-readiness-from-literals: its rule that any row holding a modal literal means not ready is now stale; the modal refusal applies only to an option-shaped label row or a label with no input-box row below it; one predicate still decides for verify and for run's pre-send gate.
@task_item n=2 part=touches
input-validation decision verify-reads-only-signatures: verify's paste guard reads the moved predicate; the reading is still against compiled characters and literals only, now with a positional rule (row shape, rows below).
@task_item n=3 part=touches
input-validation decision verify-pty-output-caught: the verify path reading PTY output through the screen predicate now reads the standing-modal rule; bounds and catch unchanged.
@task_item n=4 part=touches
threat-model-summary decision verify-screen-read-against-literals: the screen read still compares only against compiled literals but now also weighs a label row's shape and whether the box stands below it.
@task_item n=5 part=touches
threat-model-summary decision verify-signature-only: signatures-only reading holds; the new rule keeps and returns no row text, only a boolean.
@task_item n=6 part=touches
authentication-authorization decision unverified-cli-gate: run's pre-send gate refusal at once now fires on a standing modal, not on any label; quiet period, maximum wait and the unverified arm unchanged.
@task_item n=7 part=touches
indirect, through the readiness predicate: threat-model-summary decisions upstream-text-is-content and security-anti-patterns-never-do-these decision upstream-text-is-never-a-command, since a modal label quoted in upstream transcript text no longer steers the gate into refusing.
@task_item n=8 part=touches
indirect, through the readiness predicate: security-anti-patterns-never-do-these decision compiled-classification-sources, since the new option-shape and box-below rule is built only from the compiled selector glyph and compiled modal and box literals.
@task_item n=9 part=touches
indirect, through run's gate: threat-model-summary decisions verify-never-answers-dialogs and verify-writes-no-dialog-byte, since a standing dialog still refuses and no byte is typed into it; an option-shaped label with the box below still refuses.
@task_item n=10 part=touches
indirect: the send results input-not-ready refusal keeps its word and hint; only the condition that yields it narrowed, so any row naming when input-not-ready is returned reads the standing-modal rule.
@task_fact n=1 slug=input-validation
fact: A compiled modal literal refuses input readiness only when it stands: its row is option-shaped (indent, optional compiled selector glyph, optional numbering, the literal, then only spaces) or no input-box row stands below it; a footer literal never counts as the box; a label quoted mid-row above the box reads ready.
@task_fact n=2 slug=input-validation
fact: The standing-modal rule decides on compiled characters and literals alone and keeps or returns no row text; the imports dialog's real screen, the trust dialog's selector row and cell attributes stay unmeasured and decide nothing under it.
@task_item n=1 part=untouched
Socket, peer euid, strict-modes and server-verification rows: no channel, socket, home check or refusal order changed.
@task_item n=2 part=untouched
Dependency security and supply chain: no crate, dependency, pin, toolchain or workflow added or bumped.
@task_item n=3 part=untouched
Secret management and credential strip rows: no environment read, credential or live rig arm touched; no live claude work.
@task_item n=4 part=untouched
Error-handling and channel or MCP contract items: no refusal word, detail, exit code, hint, log line or JSON-RPC code added or changed.
@task_item n=5 part=untouched
Data-protection fixture rows and screen-fixture hygiene: no recorded fixture or schema changed; stamps and the trust record writer unmoved.
@task_item n=6 part=untouched
Paste text and dialog free-text character sets, empty-text and control-character refusals: text validation is unchanged.
@task_item n=7 part=untouched
The screen feed rows (bounded queue, isolation, panic catch): the feed itself is unchanged; only the predicate over its rows moved.
