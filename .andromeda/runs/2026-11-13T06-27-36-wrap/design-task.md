@task_prose part=session
The session changed how a verified agent CLI's screen is read as ready: the input box reads up when a footer literal stands OR the box's own rows stand (a rule row of at least 40 box-drawing chars directly above a row opening the prompt char), never while a modal literal is on screen.
One predicate now decides readiness for verify's typed run, for the ledger's input-box-signature row and for viola run's pre-send readiness gate; that gate's quiet period, maximum wait and refusals are unchanged.
A recorded screen keeps a box row as its literal alone (the rule run, the bare prompt char); a recording whose live prompt row holds a home path, the user word or an email is refused as an unclean fixture.
The verify stamp's measured typed-probe block gains a count of guards that passed on the box alone; verify ran live on claude 2.1.323 (24 step lines, 23 pass, 1 fail on statusline-rate-limits; stamped 2.1.323).
Supply chain: three reasoned rmcp advisory ignores (each in a disabled rmcp feature) and a lockfile bump of rustix; CI supply-chain green again.
Nothing retired; no verb, exit code, output line, hint or colour was added or changed.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures of the agent CLI and the readiness verdict over its rendered rows: footer, modal and now input-box rule and prompt literals, the input-box-up predicate
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the verify ledger: the relied rows and their checks (input-box-signature), the recorded screen fixture form (signature rows only) with its unclean-fixture fault, and the stamps.json measured block
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit cases: box-only screens, recorded box rows, the unclean-fixture refusal on a box prompt row, the box-only guard count in the stamp
@task_file n=4 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed-run turn guards: the input-box-ready guard over each framing turn and its box-only count
@task_file n=5 path=src/cmd/verify/typed/pty.rs
what: viola verify's typed-run terminal waits: the settle and box waits that end on a quiet screen holding the input box or a modal literal
@task_file n=6 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs (the trusted run and Run B) and their ready guard
@task_file n=7 path=src/run/gate.rs
what: viola run's pre-send readiness gate (wait for the input box on a verified CLI, input-not-ready refusal on timeout or modal)
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene check: a recorded screen may hold only signature rows, any other row refused
@task_file n=9 path=deny.toml
what: the supply-chain gate's cargo-deny policy: advisories, bans, licenses, sources
@task_file n=10 path=Cargo.lock
what: the locked dependency graph
@task_file n=11 path=fuzz/fuzz_targets/vt100_feed.rs
what: the fuzz target feeding terminal bytes to the screen reader with its own signature set
@task_item n=1 part=touches
contract/viola-verify-refusals/unclean-fixture: the unclean-fixture refusal is now also reached where the agent's input-box prompt row holds a home path, the user word or an email (the kept box rule row's seam is the live prompt row); the refusal's wording and exit are unchanged.
@task_item n=2 part=touches
contract/refusal-hints/not-delivered-input-not-ready: the readiness gate behind the input-not-ready refusal now reads a screen ready on the input box's own rows alone (no footer literal needed); a modal on screen still reads not ready; the hint and refusal text are unchanged.
@task_item n=3 part=touches
contract/cli-verification/verified and contract/cli-verification/unverified-cli: the verified state now carries a stricter-or-wider readiness reading on that CLI (the box signature beside the footer literals); the newest stamped CLI is 2.1.323; the states and their display are unchanged.
@task_item n=4 part=touches
contract/viola-verify-output/step-line, contract/viola-verify-output/summary and contract/viola-verify-output/summary-line: verify's run whose output they rule changed its guards; the relied row count stays 24 ([NN/24]) and the summary form is unchanged (stamped 2.1.323 23 pass 1 fail); a row holding the 24 stays true.
@task_item n=5 part=touches
contract/viola-verify-output/fail-word and contract/viola-verify-output/stopped-or-refused: the one fail word printed on the live run is statusline-rate-limits, as before; the input-box-signature row now passes on box-only screens and fails on a box beside a modal or no box.
@task_item n=6 part=touches
contract/viola-verify-refusals/stamps-update and contract/viola-verify-output/stamps-refused: stamps.json, the file verify updates, gains a box-only guard count under the measured typed-probe block (integer or null, no version bump); the refusal on a failed update is unchanged.
@task_item n=7 part=touches
contract/send-readback/prompt, contract/viola-send-mirror/open and contract/viola-send-mirror/unable: indirect, through viola run's pre-send readiness gate: a send may now pass the gate on a screen showing only the agent's input box; the readback lines and their wording are unchanged.
@task_item n=8 part=touches
decision/no-success-before-confirmation and decision/no-optimistic-send-confirmation: indirect, through the pre-send gate reading readiness on the box alone; nothing is announced earlier, the send is still confirmed by readback.
@task_item n=9 part=touches
name/command/verify and name/command/run: the verbs whose internals changed (verify's typed run, run's readiness gate); no name taken or freed, no flag added.
@task_item n=1 part=untouched
The readback box (decision/readback-box, decision/readback-box-signature-check, contract/readback-box/*): viola's own printed readback element, not the agent CLI's input box this session reads; nothing about it moved.
@task_item n=2 part=untouched
ASCII-only output (decision/cli-ascii-only, decision/ascii-only-human-output, the CLI glyph and separator contracts): the box-drawing rule char and the prompt char are read from the child agent's screen and kept in recorded fixtures, never printed by viola.
@task_item n=3 part=untouched
Colour, typography, spacing, iconography and the strip-board (viola list board, flight-progress strip, colour tokens and depth): no rendering, column, width or colour changed.
@task_item n=4 part=untouched
Exit codes, exit-21 causes, revive and run start refusals, link and unlink, wait, last, pause and release output, the hook and mcp verbs: no code, cause, line or hint moved.
@task_item n=5 part=untouched
The advisory ignores and the lockfile bump are supply-chain policy with no row in this master; no CLI output, verb or dependency surface of the design changed through them.
@task_item n=6 part=untouched
Brand identity, anti-patterns and self-validation decisions (expression level, no spinner, no motion, no stack traces, no internals in errors): the session added no output and no progress pattern.
