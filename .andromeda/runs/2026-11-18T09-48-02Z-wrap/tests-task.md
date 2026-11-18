@task_prose part=session
The relied trust-record ledger row was read live on claude 2.1.325 by the standing live rig: start S0B passed 23/23 with trust-record pass, so the row is verified on 2.1.325 under the rig's private config only; nothing is claimed for 2.1.326 or 2.1.327.
The earlier rig starts S0R, S0R-2 and S0R-3 read RED 7/23 and keep their recorded RED; S0R-3 named the cause, 2.1.325's browser-tools prompt raised after an earlier start, and S0B proved it.
viola verify now classifies each interactive run's first screen into closed words (trust, imports, box, none, exited) and writes them, their settle walls and closed row facts into the stamp's measured block; a hidden rig-only flag records a failed verify's first screens as whole scrubbed rows, kept local and gitignored.
The live rig's V arm stands its scratch as a git root, seeds five used-config keys including the browser-tools key, prints new summary lines, gains a VC arm that reads the first frame at 80x24 and stops with no key, and takes the S0B and S0B-2 labels.
The verify_window_ nextest kill moved from 60 s to 80 s in the ci and mutants profiles; the rig seed's marker set moved from six to five keys and one unit case name was retired.
A pre-push red from stray coverage profiles, written by a version probe spawning the instrumented fake agent under a cleared environment in the test's cwd, was fixed by running that probe in the rig's scratch root.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: edited: the compiled screen signatures and the screen classifier; gains the first-screen classifier (modal before box), its closed words and the closed row facts over the box literals
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: edited: the capability ledger and the verify stamp writer; the stamp's measured block gains each run's first-screen word, settle wall and row facts
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited: the ledger's inline unit cases
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited: the ledger's stamp unit cases; gains the case that the stamp writes the six first screens, their settles and facts
@task_file n=5 path=src/cmd/verify.rs
what: edited: the viola verify command and its fixture recording; gains the hidden rig-only failed-verify first-screen record, scrubbed and refused whole when unclean
@task_file n=6 path=src/cmd/verify/typed.rs
what: edited: verify's typed interactive runs A to F
@task_file n=7 path=src/cmd/verify/typed/run.rs
what: edited: one typed verify run and its ends by kill, Ctrl-C or finish; each now keeps its first screen reading and rows
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: edited: the dialog, plan and resume typed verify runs; each returns its first screen
@task_file n=9 path=tests/cli_verify.rs
what: edited: the viola verify integration binary over the fake agent; gains the first-screen record cases and the help-names-no-option case
@task_file n=10 path=tests/cli_verify/runs.rs
what: edited: the verify typed-run integration cases; gains the first screens read trust then box on the fake agent
@task_file n=11 path=tests/support/verify.rs
what: edited: the shared verify test helper that starts a tied verify under a test-owned home; its start now takes a place record
@task_file n=12 path=crates/viola-e2e/src/rig/isolate.rs
what: edited: the live rig's isolated child scratch and its private config seed; writes git's layout as files, seeds five used-config keys, keeps failed first screens
@task_file n=13 path=crates/viola-e2e/src/rig/mod.rs
what: edited: the standing live rig's arms, labels and summary lines; gains the VC arm, the S0B and S0B-2 labels, the V arm's record flags and the version probe's cwd
@task_file n=14 path=crates/viola-e2e/src/rig/screen.rs
what: edited: the live rig's screen model; gains a sized model
@task_file n=15 path=crates/viola-e2e/src/rig/ledger.rs
what: edited: the live rig's start ledger and its caps; a -3 label is a spare
@task_file n=16 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited: the standing live rig binary and its live verb
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited: the live rig's rehearsals over the fake agent; gains the VC first-frame rehearsal and the V kept-first-screen rehearsal
@task_file n=18 path=.config/nextest.toml
what: edited: the nextest profiles and their per-class kill lines; the verify_window_ class kill moves 60 s to 80 s in ci and mutants
@task_file n=19 path=.gitignore
what: edited: the repository ignore list; gains the recorded first-screen files under the claude fixtures
@task_item n=1 part=touches
decision/verify-window-60s-kill names a 60 s kill for the verify_window_ class; the kill is now 80 s in the ci and mutants profiles, with measured walls, so the row's value and its title are stale.
@task_item n=2 part=touches
Kill-line ordering rows (kill lines below cargo-mutants' timeout, root waits bounded below the kills, the per-test kill for verify-driven tests) rule over the moved 80 s verify_window_ kill and must still hold against it.
@task_item n=3 part=touches
decision/standing-live-rig gains: the V arm stands its scratch as a git root spawning nothing; it prints screen words, settle walls, facts, the relied row count and the failing ids; its seed carries the used-config markers and prints seed-markers; the VC arm; the record copy and kept line; the S0B and S0B-2 labels and the browser-tools key.
@task_item n=4 part=touches
decision/live-failing-reading-not-verification: the RED rig starts are kept as read; S0B is the live verification of the trust-record row on 2.1.325 under the rig's private config, and nothing is claimed for 2.1.326 or 2.1.327.
@task_item n=5 part=touches
Fixture recording rows (fixtures recorded only when nothing fails, recorder scrubs home paths and usernames, dirty payload refuses the recording, fixtures refreshed by live verify) gain an exception: a hidden rig-only flag makes a FAILED recording verify write first-screen files of whole scrubbed rows, refused whole when any row or seam is unclean, kept local and gitignored; a plain record verify and a passing verify write none.
@task_item n=6 part=touches
decision/screen-fixture-box-rows-as-literals and decision/screen-fixture-scrub-as-detector: the rig-only first-screen record keeps whole rows, not box literals alone, scrubbed and checked by the screen fault check.
@task_item n=7 part=touches
Screen-reading rows (screen read only for compiled signatures, no assertion beyond compiled literals, rendered child screen never parsed, no assertions on the child's screen) rule over verify's new first-screen classifier, which reads only compiled signatures and records closed words, whole numbers and literal ids, never screen text.
@task_item n=8 part=touches
decision/closed-enums-need-decisions-log rules over the new closed word sets: the first-screen words trust, imports, box, none, exited and the row families rule, banner, text.
@task_item n=9 part=touches
Stamp rows (stamps written only by verify, verify is the only stamp writer, single writers of snapshot and stamps) rule over the stamp's measured block, which verify now extends with first_screen, first_settle_ms and first_facts.
@task_item n=10 part=touches
decision/verify-takes-no-json and the viola verify output contracts: verify gains a hidden option absent from its help, passed only by the rig's V arm; no step line, summary line or exit detail changed.
@task_item n=11 part=touches
Profile rows (env_clear tests re-add LLVM_PROFILE_FILE, cleared environment keeps LLVM_PROFILE_FILE, profraw census is a dev-host tool, child viola processes counted in coverage) are reached by the pre-push red: a version probe spawning the instrumented fake agent under a cleared environment wrote coverage profiles into the test's cwd; the fix runs the probe in the scratch root.
@task_item n=12 part=touches
decision/fake-agent-not-proof-of-real-cli: the step 15 fake-agent oracle premise that runs B to F read a rule above the prompt was measured false (the fake draws none); the real-CLI claim rests on the live S0B start.
@task_item n=13 part=touches
Version-gate rows (version gate needs every relied row, unverified CLI version gate, compatibility tests across CLI versions, every ledger row has a verify probe): the relied row count stays 23 and the trust-record row's probe now passes live on 2.1.325; the 2.1.325 recorded fixture set still needs a live verify in a live home.
@task_item n=14 part=touches
Real-CLI locality rows (real claude CLI tested local-only, live verify never in CI, CI never touches the real agent CLI, tests never read the host's claude sessions) rule over the live rig starts and its private config seed, whose five keys come from the bundle's own logic and none from the founder's config.
@task_item n=15 part=touches
Tied verify start rows (every test verify start tied to a test-owned home, test verifies run under a test-owned user home, one shared copy of each test helper) rule over the shared verify start helper, which now takes a place record instead of three parameters.
@task_item n=16 part=touches
Fixture hygiene walk rows (fixture hygiene run-time walk, fixture hygiene walk) may meet the new gitignored first-screen files under the claude fixtures on a dev host after a failed rig verify.
@task_fact n=1 slug=7-test-data-fixtures
fact: A failed recording verify run by the live rig alone writes one first-screen file of whole scrubbed rows per run not reaching the input box (Run A's trust screen excepted), refused whole when unclean, gitignored and never committed; no other verify writes it.
@task_fact n=2 slug=quality-gates-coverage-targets
fact: A test probe that spawns an instrumented binary under a cleared environment runs with a scratch directory as its cwd, so no coverage profile lands in the tree.
@task_fact n=3 slug=6-e2e-test-strategy
fact: The trust-record relied ledger row is verified live on claude 2.1.325 only, by rig start S0B under the rig's private config; no later CLI version is claimed.
@task_item n=1 part=untouched
MCP server, MCP tools and their invalid-params, refusal and reply-screen contracts: no MCP surface changed.
@task_item n=2 part=untouched
Wrapper channel, JSON-RPC errors, link and unlink methods and the event log contracts: no channel or event shape changed.
@task_item n=3 part=untouched
viola send, answer, wait, last, release, list and revive contracts and refusal orders: viola run and send are unchanged for the browser-tools prompt.
@task_item n=4 part=untouched
Hook exec, hook decision bodies, hook deadlines and performance budgets: no hook path or perf budget moved.
@task_item n=5 part=untouched
Harness agent-run commands, status shape, boot, cleanup, pre-push and gate contracts and their exit codes: no agent-run command, status shape or CI step changed.
@task_item n=6 part=untouched
CI workflows, artifacts, supply-chain gates, toolchain pins, fuzzing and coverage floors: none moved; no dependency was added.
@task_item n=7 part=untouched
Fake agent command line, receipt, input and screen contracts: the fake agent itself did not change.
@task_item n=8 part=untouched
Log format, log line fields and corr contracts: no log line changed.
