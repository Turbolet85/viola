@task_prose part=chunk
Turns the supply-chain CI gate green again by admitting one advisory ignore, for RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade, brought in upstream by rmcp and schemars), with its reason and its re-read ground beside it, and re-words the comment over the ignore list.
Re-measures every config and job that reads advisories (root graph, fuzz lock audit, weekly nightly advisories run, per-sync-crate runs) to learn which read this id.
Ties the in-test boots of the end-to-end crate's harness lifecycle and CLI tests, so their supervisors end with their test, killed or not.
Adds a test that kills the tied stamping viola verify and reads that nothing of it is left.
Changes no CLI output, no verb, no refusal, no hint and no rendering.
@task_file n=1 path=deny.toml mark=edits
what: the dependency-policy config the supply-chain gate reads: its advisories ignore list — edits: one ignore entry for RUSTSEC-2026-0543 and the comment above the ignore list
@task_file n=2 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the end-to-end harness lifecycle tests and their in-test boot options — edits: the options boot, built untied
@task_file n=3 path=crates/viola-e2e/tests/cli.rs mark=edits
what: the end-to-end CLI tests and their in-test boot — edits: the booted boot options, built untied
@task_file n=4 path=.github/workflows/ci.yml mark=re-reads
what: the CI supply-chain job that runs the advisories check over the root graph and the fuzz lock audit
@task_file n=5 path=.github/workflows/nightly.yml mark=re-reads
what: the weekly nightly advisories run
@task_file n=6 path=deny-sync.toml mark=re-reads
what: the per-sync-crate advisories config its runs read
@task_file n=7 path=fuzz/Cargo.lock mark=re-reads
what: the fuzz lock graph whose advisories audit is re-measured
@task_item n=1 part=needs
The supply-chain CI gate: its advisories check over the dependency graph, red on one unmaintained advisory with no safe upgrade.
@task_item n=2 part=needs
One ignore entry for that advisory id alone, its reason naming the upstream crates that bring chrono in and its re-read at each epoch boundary; no other advisory, skip or allow added.
@task_item n=3 part=needs
The comment over the ignore list stating both grounds: the disabled-feature ground and the widened unmaintained ground.
@task_item n=4 part=needs
Viola's own chrono use stays as it is; nothing in the crates that use it directly changes.
@task_item n=5 part=needs
Every config and job that reads advisories re-measured from the code and the failing run's log: which read this id and which fail on it.
@task_item n=6 part=needs
The in-test boots of the end-to-end harness lifecycle and CLI tests built tied, so their supervisors and fake agents end with their test even when it is SIGKILLed.
@task_item n=7 part=needs
A new test that kills the tied stamping viola verify process and reads that nothing of it is left running.
@task_item n=8 part=needs
The tie for test children and the process census the previous chunk built, used as they are; no test sends a kill at a process it does not own, and a site sweep covers every spelling of a constant.
@task_item n=9 part=needs
A test that waits on a channel or a thread takes a bound under the project's 20 s lint; a test socket lives under the short tmp root.
@task_item n=10 part=needs
Every acceptance predicate can fail, with the change that would turn it red named; every test filter selects at least one test; every control fires with a named timeout and an id within 16 characters.
@task_item n=11 part=needs
Acceptance is the chunk's own pre-CI run, supply-chain green on the pushed sha and the whole run green; no live claude start and no mutation run.
@task_item n=12 part=needs
The narrowed ledger note on driver process hygiene closed for what it names, claimed so the wrap can lift it; no master edited, the widened advisory rule left to the wrap as an expected amendment.
@task_item n=1 part=not_needed
The web surface absence: the chunk renders no page and adds no web surface.
@task_item n=2 part=not_needed
CLI output decisions (board, BAY line, styling, SGR, TTY detection, colour, spinners, banners, footers, progress glyphs, readback mirror, phraseology): the chunk changes no human output.
@task_item n=3 part=not_needed
CLI exit codes, refusal lines, hints and internal-error contracts: no verb gains or loses an exit, refusal or hint.
@task_item n=4 part=not_needed
The per-verb output contracts of list, send, wait, last, answer, link and unlink, run, revive, plugin install and the wheel verbs: none of those verbs change.
@task_item n=5 part=not_needed
The viola verify output and refusal contracts: the new test kills the verify process and reads the process table, not its step lines, summary or refusals.
@task_item n=6 part=not_needed
The MCP server's tools, tool results and hidden-verb decisions: the MCP crate's code is not touched, only its upstream dependency is admitted by the ignore.
@task_item n=7 part=not_needed
CLI parsing, global flags, help and usage contracts, target naming and token vocabulary: no flag, verb or name changes.
