@task_prose part=session
The readiness read of the claude screen now counts the input box as up when a footer literal stands OR the box's own rows stand (a rule row of at least 40 box-drawing dashes directly above a row opening the prompt char), and never while a modal literal is on screen; one predicate decides for verify's guards, the input-box-signature ledger row and viola run's pre-send readiness gate.
viola run's pre-send gate therefore reads a screen ready on the box's rows alone; its quiet period, GATE_MAX_WAIT and poisoned arms are unchanged, and verify's settle and box waits still also end on a modal literal.
A recorded screen fixture keeps a box row as its literal alone (the dash run, or the prompt char alone, never the text after it); the fixture walk refuses any other form, and a recording refuses where the prompt row holds a home path, user word or email.
verify's typed run measures box_only_guards (guards passed on the box with no footer literal) into the ledger stamps' measured.typed_probe; the relied ledger rows stay 24 and the live proof ran on claude 2.1.323.
Supply chain: three reasoned cargo-deny advisory ignores for disabled rmcp features, and rustix bumped 1.1.5 to 1.1.6 in the lockfile; CI green.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the vt100 readiness gate's compiled screen signatures and its verdict — the input-box and modal literals the screen is read for, and the readiness predicate
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: verify's ledger — the input-box-signature row's check, the signature rows a recorded screen fixture keeps, the screen-fixture scrub, and the ledger stamps
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit tests — the signature-row fixtures, the screen-fixture scrub and the stamps' measured fields
@task_file n=4 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed run turn guards — the input-box readiness read before each paste, /compact and local command
@task_file n=5 path=src/cmd/verify/typed/pty.rs
what: viola verify's PTY driver — the settle and box waits on a quiet screen holding a compiled signature
@task_file n=6 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs — Run B's ready guard and the trusted run
@task_file n=7 path=src/run/gate.rs
what: viola run's pre-send readiness gate (wait_ready) that refuses a send with input-not-ready
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the screen-fixture hygiene walk — which recorded rows pass as signature rows and which are refused
@task_file n=9 path=deny.toml
what: the cargo-deny supply-chain gate — advisories, bans, licenses, sources
@task_file n=10 path=Cargo.lock
what: the locked dependency graph
@task_file n=11 path=fuzz/fuzz_targets/vt100_feed.rs
what: the vt100 feed fuzz target, which builds its own screen signatures
@task_item n=1 part=touches
decision/no-screen-content-parsing: the signatures the screen is read for grow by the input box's rule and prompt chars; reading stays limited to compiled signatures (the prompt row only for its opening char), so check its wording against the wider set.
@task_item n=2 part=touches
contract/send-refusal/input-not-ready: viola run's pre-send gate now reads ready on the input box's own rows alone, not only on a footer literal; a modal on screen still keeps it not ready.
@task_item n=3 part=touches
decision/send-refused-while-human-typing and decision/human-keystrokes-never-blocked: indirect, through the pre-send readiness gate whose ready verdict changed; the human-typing refusal itself is unchanged.
@task_item n=4 part=touches
decision/boundary-check-never-inspects-claude-content: the readiness read now looks at the input box's rule row and the prompt row's opening char, never the text after the prompt; check the row still holds.
@task_item n=5 part=touches
decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human: indirect, through the readiness gate's GATE_MAX_WAIT and quiet period, which are unchanged; a screen with the box alone now ends the wait sooner.
@task_item n=6 part=touches
contract/viola-verify/step-line: verify's typed run still prints 24 relied rows; the input-box-signature row's check now reads the box predicate, no step line's form or count moved.
@task_item n=7 part=touches
decision/tui-boundary-only and decision/wrapped-tui-reads-as-unwrapped: indirect, through the vt100 readiness signatures that the boundary reads; no byte reaches the terminal from this change.
@task_item n=8 part=touches
contract/tui-passthrough/vt100-feed-panic: the vt100 feed fuzz target now builds signatures with the box's two chars; the panic containment is unchanged.
@task_item n=9 part=touches
decision/boundary-tests-sequenced-on-records and decision/live-readings-never-gate: recorded screen fixtures now keep box rows as their literals alone and the hygiene walk accepts them; live readings still gate nothing.
@task_fact n=1 slug=11-a11y-anti-patterns-never-do-these
fact: The screen signatures read for readiness are the footer literals, the input box's own rows (a rule row of box-drawing dashes above a row opening the prompt char) and the modal literals; the prompt row is read only for its opening char, never the text typed after it.
@task_item n=1 part=untouched
Colour and SGR rows (amber, bold, dim, word beside every SGR span, no SGR off a terminal): no output styling changed.
@task_item n=2 part=untouched
CLI refusal layout, hints, code words, exit codes and --json failure documents: no refusal word, hint or exit code was added or changed.
@task_item n=3 part=untouched
viola list board, caption, unwrapped separator and NAME truncation: the board was not reached.
@task_item n=4 part=untouched
viola revive, link, unlink, plugin install, wait and hook output contracts: none of their output changed.
@task_item n=5 part=untouched
Manual Orca and keyboard passes, screen-reader targets and the a11y tier: unchanged.
@task_item n=6 part=untouched
Absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit): no capability added.
@task_item n=7 part=untouched
config.json budget gate (five_hour, seven_day, dialog_hold_s): no budget or dialog hold value moved.
