@task_prose part=chunk
Turns the supply-chain CI gate green again by one advisory ignore for the unmaintained chrono advisory, its reason naming the upstream crates that bring chrono in and its re-read at each epoch boundary, with the ignore-list comment re-worded to state both grounds.
Re-measures every config and job that reads advisories (root graph, fuzz lock audit, weekly nightly advisories run, per-sync-crate runs) for which of them read this id and fail on it.
Ties the in-test wrapper boots of the end-to-end crate's harness-lifecycle and CLI tests so their supervisors end with their test, killed or not.
Adds a test that kills the tied stamping viola verify and reads that nothing of it is left.
No CLI output, verb, refusal or rendering changes; Viola's own chrono use stays.
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny supply-chain config of the root graph, its advisories ignore list — edits: the advisories ignore list and the comment above it
@task_file n=2 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the end-to-end harness-lifecycle tests that boot a wrapper in-test — edits: the untied boot options of its in-test boot
@task_file n=3 path=crates/viola-e2e/tests/cli.rs mark=edits
what: the end-to-end CLI tests that boot a wrapper in-test — edits: the untied boot options of its booted helper
@task_file n=4 path=.github/workflows/ci.yml mark=re-reads
what: the supply-chain CI job's cargo deny advisories check, re-measured against the failing run's log
@task_file n=5 path=.github/workflows/nightly.yml mark=re-reads
what: the weekly advisories run, re-measured for whether it reads this advisory id
@task_file n=6 path=deny-sync.toml mark=re-reads
what: the per-sync-crate cargo-deny advisories runs, re-measured for whether they read this advisory id
@task_file n=7 path=fuzz/Cargo.lock mark=re-reads
what: the fuzz graph's advisories and sources audit, re-measured for whether it reads this advisory id
@task_item n=1 part=needs
The viola verify verb as a process under test: the tied stamping verify run is killed by a test, and its process and children must leave nothing behind; the verify verb's printed output, step lines, summary and refusals are not changed.
@task_item n=2 part=needs
An interrupted viola verify run: the new kill test ends a stamping run mid-way, so any rule on what an interrupted or stopped verify run leaves or refuses binds what the test may observe and must not regress.
@task_item n=3 part=needs
The stamps.json update that a stamping verify writes: a kill mid-run must leave no stray process; any rule that a stamps update is refused or left whole on an interrupted run binds the kill test's reading.
@task_item n=4 part=needs
The wrapper boot in tests (viola run as the wrapped child supervisor): in-test boots are built tied so the supervisor and its fake agent end with their test when the test is SIGKILLed, using the existing harness and root tie constants and the census.
@task_item n=5 part=needs
No test sends a kill at a process it does not own, and the proof leaves nothing running; a site sweep covers every spelling of a tie constant.
@task_item n=6 part=needs
A test that waits on a channel or thread takes a bound under the project's 20 s lint, and a test socket lives under the short /tmp root (wrapper socket and socket-directory rules reach the tests through the boot they make).
@task_item n=7 part=needs
The supply-chain advisory ignore: exactly one advisory id admitted, no skip or allow, its reason written beside it naming the upstream crates and its epoch-boundary re-read; what would turn the gate red again is a new advisory id or the same id once a safe upgrade exists.
@task_item n=8 part=needs
Acceptance predicates owe a change that would turn them red, every test filter is checked to select at least one test, every control names its timeout, and the run has no live claude start and no mutation run.
@task_item n=1 part=not_needed
Brand-identity rows on strips, readback box, cocked dialog-pending strip, handoff and phraseology: the chunk changes no displayed session state.
@task_item n=2 part=not_needed
Color-palette, typography, spacing and iconography rows (amber accent, dim, bold NAME column, glyph and separator contracts, list columns): no rendered output is added or changed.
@task_item n=3 part=not_needed
viola list board, table, BAY line, NAME cut and skip-count contracts: the chunk does not touch the board.
@task_item n=4 part=not_needed
Send readback and send mirror contracts, refusal hints for not-delivered sends, peer reply screening and wrapper fault --json detail: no send or peer reply path is touched.
@task_item n=5 part=not_needed
viola revive output, refusals and --json exit-1 codes, and viola run start refusals: neither verb's behaviour changes.
@task_item n=6 part=not_needed
viola link, unlink, wait, last, answer, pause, release and help output contracts and the command names register: no verb is added, renamed or re-worded.
@task_item n=7 part=not_needed
CLI colour decision order, colour depth and colour tokens, hand-written SGR module and TTY detection: no styling code is touched.
@task_item n=8 part=not_needed
The web-spa surface and rendered-page absences, and the self-validation protocol's squint, swap and token tests: no page or visual design is produced.
