@task_prose part=session
The session read the relied trust-record ledger row live on claude 2.1.325 in the live rig's private home: start S0B passed 23/23, so the row is verified on 2.1.325 under the rig only, and nothing is claimed for 2.1.326 or 2.1.327.
It named the screen that stopped verify runs B to F short of the input box: 2.1.325's browser-tools prompt, raised after an earlier start under the same config. Only the rig was mended, with a browser-tools key in its seed. viola verify and viola run are unchanged for the prompt, and the prompt becomes its own route entry.
verify's stamp gains measured first-screen words, settle walls and closed fact rows: closed words, whole numbers and literal ids only, with no screen text.
verify gains a hidden rig-only option. On a failed recording verify it writes each failed run's first screen as whole scrubbed rows, refuses an unclean set whole, keeps the files local and git-ignored, and leaves it out of help. The founder answered this widening in his own words.
The rig's seed moved from six used-config markers to five bundle keys, one of them the browser-tools key set to false. Two markers were retired. The rig writes a minimal git layout into its scratch and spawns no process to do it, and its version probe now runs inside the scratch.
The verify window class kill in the test runner moved from 60 s to 80 s. The relied ledger row count stays 23.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the Claude adapter's screen model, which classifies PTY rows only against compiled signature literals (modal before input box) and now also names a run's first screen in closed words and closed row facts
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger and the verify stamp writer, whose measured part now holds first-screen words, settle walls and closed fact rows; also the shared fixture scrub and the screen-fault check
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the capability ledger
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the verify stamp's measured fields
@task_file n=5 path=src/cmd/verify.rs
what: the viola verify command, covering its fixture recording through the scrub and the whole-set refusal and now the hidden rig-only failed-verify first-screen record
@task_file n=6 path=src/cmd/verify/typed.rs
what: verify's typed runs A to F over the CLI child in a PTY
@task_file n=7 path=src/cmd/verify/typed/run.rs
what: one verify run's bounded PTY read, settle and end by kill or ctrl-c, now keeping the first screen it reads
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: verify's dialog, plan and resume runs, each returning its first-screen reading
@task_file n=9 path=tests/cli_verify.rs
what: integration tests of viola verify's recording, its refusals and its help over the fake agent
@task_file n=10 path=tests/cli_verify/runs.rs
what: integration tests of verify's six runs over the fake agent
@task_file n=11 path=tests/support/verify.rs
what: test support that starts verify in a test-owned home
@task_file n=12 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation of its child, covering the private scratch home, the seed of the child's config, the transient credentials copy, a reader bounded at MAX_FRAME and the local copy of kept first screens
@task_file n=13 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's arms, start labels, version probe and summary lines of closed words
@task_file n=14 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's own screen model of its pty
@task_file n=15 path=crates/viola-e2e/src/rig/ledger.rs
what: the live rig's start ledger and its founder-set caps and spares
@task_file n=16 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary that runs the real claude CLI locally
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: rig rehearsals over the fake agent, including the check that no record holds a credential or host path
@task_file n=18 path=.config/nextest.toml
what: the test runner config, covering the verify window class kill in ci and mutants
@task_file n=19 path=.gitignore
what: the git ignore list that keeps test homes and local recordings out of git, now also first-screen files under the fixtures
@task_item n=1 part=touches
decision/live-rig-isolated-child: the rig child's seed now holds five keys taken from the bundle's own logic, none from the founder's config. Four are used-config markers and one is the browser-tools key set to false. The rig also writes git's minimal layout into its private scratch without spawning a process.
@task_item n=2 part=touches
decision/verify-signature-only and decision/verify-reads-only-signatures: these gain an exception. A hidden rig-only option on a failed recording verify keeps whole scrubbed rows of each failed run's first screen.
@task_item n=3 part=touches
decision/verify-screen-read-against-literals and decision/input-box-readiness-from-literals: the first-screen classifier reads only compiled literals (trust, imports, box, none, exited), modal before box, and the whole-row record is the exception.
@task_item n=4 part=touches
decision/fixture-scrub-and-refusal, decision/fixture-recording-scrubbed and decision/fixture-hygiene: the first-screen files go through the same scrub, every row and seam is checked by the screen-fault check, and an unclean set is refused whole.
@task_item n=5 part=touches
decision/screen-fixture-signature-rows-only: the hidden first-screen record keeps whole rows, not only signature rows. It is rig-only, local and never committed.
@task_item n=6 part=touches
decision/live-session-screens-kept-local and decision/fixtures-reviewed-before-commit: kept first screens stay local, and their pattern is git-ignored under the fixtures.
@task_item n=7 part=touches
contract/verify-refusal-codes/unclean-fixture, contract/fixture-refusal-line/form and the contract/fixture-refusal-codes rows (absolute-path, email, home-path, username): the first-screen record's whole-set refusal goes through the same refusal path.
@task_item n=8 part=touches
decision/stamps-written-only-by-verify and decision/trust-record-closed-words: verify's stamp gains measured first-screen words, settle walls and fact rows, holding closed words, whole numbers and literal ids only and no screen text.
@task_item n=9 part=touches
decision/test-seams-compiled-out, decision/test-seams-fake-agent-only and decision/new-env-read-needs-log-entry: verify gains a hidden option that is absent from help and passed only by the rig's V arm. It is not compiled out of release.
@task_item n=10 part=touches
decision/controls-not-switchable and decision/no-control-switch-off: the hidden option widens what verify records. It switches no scrub or refusal off, and a plain recording or a passing verify writes no screen.
@task_item n=11 part=touches
decision/verify-never-answers-dialogs and decision/verify-writes-no-dialog-byte: the browser-tools prompt is a dialog that verify does not yet read as a modal. verify still answers nothing, and the rig avoids the prompt through its seed key.
@task_item n=12 part=touches
decision/live-round-rig-keys-own-pty and decision/live-rig-prompt-capture-closed: a new VC arm reads its first frame at 80x24 and stops with no key. Every start held keys none, and the new summary lines print closed words, numbers and literal ids.
@task_item n=13 part=touches
decision/live-rig-credentials-copy: the rig's isolation module changed. The teardown still removed the credentials and left the scratch absent.
@task_item n=14 part=touches
decision/nothing-written-under-claude-home, decision/no-writes-under-claude-home and decision/viola-writes-nothing-in-claude-dir: the rig, not viola, writes the child's config seed and git layout, and only inside its private scratch.
@task_item n=15 part=touches
decision/bounded-readers-census, decision/reads-capped-at-max-frame and decision/max-frame-cap: the rig adds a JSON file reader bounded at MAX_FRAME.
@task_item n=16 part=touches
decision/trust-record-writer-guarded, decision/verify-never-answers-import and decision/verify-trusted-runs-risk: the trust-record row is now verified live on 2.1.325 under the rig's private config, so any wording that says the row is unverified on the real CLI is stale. A live home still needs its own live verify.
@task_item n=17 part=touches
decision/real-cli-local-only and decision/verify-children-direct-spawn: the rig's version probe now runs with the scratch as its cwd, so an instrumented child leaves no profile files in the tree.
@task_item n=18 part=touches
decision/test-homes-gitignored and decision/no-secrets-in-git: the git ignore list gains the first-screen file pattern under the fixtures.
@task_fact n=1 slug=data-protection
fact: viola verify has one exception to signature-only reading. A hidden rig-only option on a failed recording verify writes one first-screen file per run that did not reach the input box (Run A's trust excepted). The file holds whole rows, scrubbed, and an unclean set is refused whole. The files are kept local and git-ignored, and a plain recording or a passing verify writes none.
@task_fact n=2 slug=secret-management
fact: The live rig seeds its child's private config with five keys whose values come from the CLI bundle's own logic, none read from the founder's config. Four are used-config markers and one turns the browser-tools prompt off. The rig writes git's minimal layout into its scratch and spawns no process to do it.
@task_fact n=3 slug=input-validation
fact: verify's stamp records each run's first screen only as closed words (trust, imports, box, none, exited), whole-number settle walls and closed fact rows of literal ids and row families. It records no screen text.
@task_item n=1 part=untouched
The IPC channel, the socket directory, peer euid checks and the server verification before a frame: no channel, socket or client seam changed.
@task_item n=2 part=untouched
The MCP tools and the wrapper channel methods and their contracts: no tool, method, frame or error code changed.
@task_item n=3 part=untouched
Dependency security, covering pins, cargo-deny, licences, advisory floors, toolchains and workflow actions: no dependency or CI step changed.
@task_item n=4 part=untouched
Paste and send text validation, typed-text character sets, and the hook payload and its fail-open rules: none was reached.
@task_item n=5 part=untouched
revive, list, link and unlink, the snapshot and the event log, with their refusal contracts: no change.
@task_item n=6 part=untouched
viola run's pre-send gate and its version gate: unchanged, since the browser-tools prompt is a route entry and is not built here.
@task_item n=7 part=untouched
The pinned binary and plugin install, the marketplace tree and the release build rules: no change.
@task_item n=8 part=untouched
The absences (network, hosting, accounts, secrets, database, release stage): no capability of that kind was added.
