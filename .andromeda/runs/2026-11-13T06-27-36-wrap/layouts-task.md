@task_prose part=session
The input box now reads up on a footer literal OR on the box's own rows (a rule row of at least 40 box-drawing chars directly above a row opening the prompt char), never beside a modal; one predicate decides for viola verify's guards, its input-box-signature row and viola run's pre-send readiness gate.
So viola run's gate (and viola send through it) now reads a screen ready on the input box alone; the quiet period, the gate's maximum wait and the poisoned arm are unchanged.
A recorded screen (viola verify --record) keeps a box row as its literal alone; a box whose prompt row holds a home path, the user word or an email makes the recording refuse as an unclean fixture.
The ledger stamps' measured typed probe gains box_only_guards (an integer or null); the relied verify rows stay 24 and no verify output line changed.
Supply chain: three reasoned rmcp advisory ignores in the cargo-deny config, a lockfile bump of one crate; CI green. The newest claude CLI, 2.1.323, verified and stamped.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures (footer, modal and now input-box literals) and the readiness verdict viola run's pre-send gate and viola verify read
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the verify ledger: the relied rows' checks (the input-box-signature row), the recorded screen fixture's kept signature rows and its clean-fixture refusal, the stamps' measured block
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the tests of the verify ledger's rows, recorded screen rows, fixture refusal and stamps
@task_file n=4 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed run turns: the ready guards each verify step waits on
@task_file n=5 path=src/cmd/verify/typed/pty.rs
what: viola verify's terminal driver: the settle and input-box waits of the typed run
@task_file n=6 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs (the trusted run and its ready guard) that feed the verify steps
@task_file n=7 path=src/run/gate.rs
what: viola run's pre-send readiness gate, the wait behind viola send's input-not-ready outcome
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene check: which rows a recorded screen may keep, refusing any other row
@task_file n=9 path=deny.toml
what: the supply-chain gate's dependency policy: bans, licenses and the advisory ignore list
@task_file n=10 path=Cargo.lock
what: the locked dependency graph
@task_file n=11 path=fuzz/fuzz_targets/vt100_feed.rs
what: the fuzz target that feeds terminal bytes to the screen reader with its own signatures
@task_item n=1 part=touches
viola send's input-not-ready outcome and its hint: the pre-send gate behind it now reads a screen ready on the input box's own rows alone, so the outcome fires on fewer screens (a modal still refuses); wording and exit unchanged; a row saying when input-not-ready is reported, or that readiness is read off a footer literal, may be stale.
@task_item n=2 part=touches
viola run's verb row and its passed start output: the wrapped claude's readiness, as the run gate reads it on a verified CLI, now holds on the footer literal or the box's rows; a row naming the footer literal as the readiness sign is touched.
@task_item n=3 part=touches
viola mcp's send tool: it goes through the same pre-send gate, so its input-not-ready refusal moves with it (indirect, wording unchanged).
@task_item n=4 part=touches
viola verify refusals, recorded fixture not clean: --record now also refuses where the input box's prompt row holds a home path, the user word or an email; the refusal wording is unchanged, its reach widened.
@task_item n=5 part=touches
viola verify output, --record: what a recorded screen keeps changed — a box rule row as its run of box-drawing chars alone, a prompt row as the prompt char alone, footer and modal rows whole as before, every other row empty.
@task_item n=6 part=touches
viola verify output, step line, failing row, summary and summary line: the relied row count stays 24 ([NN/24]) and the input-box-signature row keeps its id; it now passes on box-only screens. A row holding the count 24 or the row's meaning (an input-box literal) is to be checked, not moved.
@task_item n=7 part=touches
viola verify output, --help paragraph: if it says verify reads readiness off the footer literal, it is now the footer literal or the input box's own rows.
@task_item n=8 part=touches
readback vocabulary state words: the input-not-ready word is now reached less often, its meaning (no input box up, or a modal) unchanged (indirect, through the gate).
@task_item n=9 part=touches
the stamps viola verify writes gain a measured box_only_guards count; no verify output line prints it — a touch only where a row lists what verify records or prints.
@task_item n=1 part=untouched
The viola list board, the BAY line, its columns, colours, truncation and --json document: no board or list rendering changed.
@task_item n=2 part=untouched
No persistent footer and the other no-chrome decisions (banner, spinner, progress glyphs, pager): they rule over viola's own output; the footer literal here is the wrapped claude's screen text, not a viola footer.
@task_item n=3 part=untouched
CLI exit codes, the refusal line form, the hint rules and the --json error shapes: no code, word or field was added or renamed.
@task_item n=4 part=untouched
viola revive, link and unlink, wait, last, answer, plugin install and the wheel verbs (pause, release, budget) output: unchanged.
@task_item n=5 part=untouched
CLI token rendering (colour, spacing, radius, motion, roles): no styling changed.
@task_item n=6 part=untouched
The web surface absence and the deep-link absence: no surface added.
@task_item n=7 part=untouched
The MCP tool list, its hidden verbs and tool result shapes: no tool added, removed or reshaped.
