@task_prose part=session
The session recorded one live StopFailure payload on claude 2.1.328 (the start SF, 2026-11-19, error word server_error, cause server-error, green on predicates 1-7) through a new SF arm of the test-only live rig, run inside a network-closed bwrap sandbox with its own network namespace.
On that green reading the StopFailure exception is SETTLED on the founder's word (outcome B): StopFailure stays registered with no ledger row and no verify probe, no longer provisional, its rows now to cite the live reading; outcome A (a ledger row with a verify probe) is not opened. One live StopFailure on one version only; nothing is claimed for the other twelve error words or another version.
The rig gained a StopFailure capture (key names, the error word from the CLI's closed 13 else other, two byte lengths, never a text), a read of the input box's cell attributes (never text) after the turn end, a sandbox probe that refuses absent, no-runtime-dir, failed or shared, and a refusal (refused: sandbox, exit 3) before any credentials read or ledger line.
The rig's kept record dir is now keyed by CLI version first, refusing a version already kept and writing nothing; the fake agent gained two argv options, --turn-fail and --settings-hooks; four sf rehearsals were added over the fake agent with the recorded 2.1.328 set.
No product crate changed; no dependency, schema, config key, env var, harness command or spec master edited; nothing retired.
@task_file n=1 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's scratch isolation: the kept record per CLI version, the bwrap sandbox argv and probe argv, the program lookup on the rig's own PATH, the runtime-dir crossing check, and the seeding of the StopFailure capture hook
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the live rig's arms and plans: the new SF arm (labels SF and SF-2, the contain plan), the sandbox probe, the child spawn through the sandbox, the turn-end, cause and network-namespace reads and the box cell-attribute read
@task_file n=3 path=crates/viola-e2e/src/rig/capture.rs
what: edited — the rig's prompt and payload capture: the closed list of the CLI's 13 error words copied from the agent crate's API-error cause map and the StopFailure capture line (key names, error word, two text lengths)
@task_file n=4 path=crates/viola-e2e/src/rig/screen.rs
what: edited — the rig's vt100 screen model reader: cell and box attribute words (dim, italic, bold, inverse, colour), never a cell's text
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the viola-live-rig binary's argv: rehearse sf with --contain, live SF and SF-2, every live SF contained
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the rig's rehearsal integration tests over the fake agent: four sf rehearsals (turn end and cause, unknown error word with needles, contained network namespace, refusal without the sandbox)
@task_file n=7 path=src/bin/viola-fake-agent.rs
what: edited — the test-only fake agent binary: argv options --turn-fail (the turn ends with a recorded StopFailure payload) and --settings-hooks (a settings.json whose hooks run beside the plugin's, split on spaces, no shell)
@task_item n=1 part=touches
decision/capability-ledger, decision/cli-capability-ledger and decision/capability-ledger-gate: their except (StopFailure registered with no ledger row and no verify probe) is now settled on the founder's word, outcome B, no longer provisional, citing the live reading (claude 2.1.328, 2026-11-19, the start SF, the word server_error); outcome A not opened.
@task_item n=2 part=touches
decision/claude-shapes-in-one-crate, decision/claude-shapes-isolated and decision/claude-first-no-adapter-trait: their test-only except gains the rig's StopFailure capture in exactly that width — StopFailure key names, the error word of the CLI's 13 (else other), the byte lengths of two text fields, never a text; the rig holds a copy of the agent crate's 13 error words.
@task_item n=3 part=touches
decision/e2e-test-only: the test-only viola-e2e crate gains a contained arm that spawns the CLI inside a bwrap sandbox; still test-only, no product crate reaches any new symbol.
@task_item n=4 part=touches
decision/screen-rows-read-only-by-verify and decision/run-never-reads-screen-content (indirect, through the rig's screen model): the rig now reads the input box's cell attributes, never text, after the turn end; check that no row's wording forbids an attribute read by the test rig.
@task_item n=5 part=touches
decision/real-claude-local-only, decision/real-cli-probe-local-only and decision/no-live-claude-in-ci: the live SF start is local only and contained; CI runs the sf rehearsals unwrapped over the fake agent, and the contained rehearsal skips where the sandbox probe refuses.
@task_item n=6 part=touches
decision/tests-against-fake-agent and decision/ci-against-fake-agent: the fake agent can now end a turn with a recorded StopFailure payload and run a settings.json hook, so StopFailure turn ends are rehearsed in CI over it.
@task_item n=7 part=touches
decision/fake-agent-test-seams and decision/fake-agent-after-double-dash: the fake agent gains two argv options (--turn-fail, --settings-hooks), argv only, no env read; a row listing its seams may need them.
@task_item n=8 part=touches
decision/no-shell-out: the fake agent's settings hook command with no args is split on spaces and run with no shell; the rig resolves bwrap by absolute path from its own PATH.
@task_item n=9 part=touches
decision/pre-push-clean-environment (indirect): under the pre-push clean environment XDG_RUNTIME_DIR is unset, viola puts its socket dir under the temp root, which the sandbox's private temp root hides; the sandbox probe refuses no-runtime-dir there and the contained rehearsal skips.
@task_item n=10 part=touches
decision/socket-in-per-user-dir and decision/test-sockets-under-short-tmp-root (indirect): the contained arm requires XDG_RUNTIME_DIR absolute and outside the temp root so viola's socket dir crosses the sandbox boundary.
@task_item n=11 part=touches
decision/bounded-readers and decision/serde-json-recursion-limit (indirect, through the rig's capture): the StopFailure capture reads through MAX_FRAME plus one with the default JSON depth; oversize or non-object reads unread.
@task_item n=12 part=touches
decision/unparseable-external-values-unknown and decision/api-error-turn-end-clears-turn: the rehearsals read an unknown error word as cause api-error with one unknown-value line, and the live server_error as server-error; true as worded, the reading may be cited.
@task_fact n=1 slug=build-system
fact: bwrap is a test-only host dependency of the live rig alone, used by path from the rig's own PATH and never installed; a contained SF start without it is refused (refused: sandbox, exit 3) before any credentials read or ledger line.
@task_fact n=2 slug=occupied-resources
fact: the rig's kept record dir is keyed by CLI version first (kept/<version>); a version already kept is refused and nothing is written, other versions' sets stand beside it.
@task_fact n=3 slug=occupied-resources
fact: the rig's contained arm runs the wrapped CLI in a network-closed bwrap sandbox with its own network namespace, and its sandbox probe refuses (absent, no-runtime-dir, failed, shared) unless XDG_RUNTIME_DIR is absolute and outside the temp root.
@task_item n=1 part=untouched
Product contracts — channel methods and frames, CLI verbs and exit codes, MCP tools and results, JSON documents, refusal reasons: no product crate changed.
@task_item n=2 part=untouched
The hook-to-kind map, hook tiers, the turn-end cause set and turn-ended event data: true as worded; the live reading confirms the StopFailure mapping and changes no row.
@task_item n=3 part=untouched
Normalised event kinds, instance snapshot and ledger stamps contracts: no field, kind or stamp changed; verify wrote no stamp and no fixture set was added.
@task_item n=4 part=untouched
Stack and dependency policy — cargo-deny, workspace pins, toolchain, rmcp features, tokio containment: no crate dependency added or bumped.
@task_item n=5 part=untouched
CI and gates — workflows, coverage floors, harness commands, mutation and fuzz: none changed, no threshold moved.
@task_item n=6 part=untouched
Names: no crate, binary, command, environment variable or socket name taken or freed.
@task_item n=7 part=untouched
Absences — listening network port, model API calls, daemon, hosting: none touched; the live start's request never left the sandbox.
@task_item n=8 part=untouched
The product's verify capture arm (viola hook --capture) and its probe-capture contract: unchanged; the rig's capture is its own.
