@task_prose part=chunk
A viola run (and the start of viola revive where it shares the version gate) that meets a claude CLI version with no verify stamp gets that version stamped by an automatic viola verify, with no human running verify by hand, then goes on verified.
The driver's sends and answers are then no longer refused unverified-cli for that version; the form of the automatic verify (before the session, beside it, a separate verb or flag, a config key) is the founder's at P3.
A red automatic verify does not loop: it says which ledger rows failed, leaves the session to the human as today, records the failed attempt so it is not repeated, and names what clears it.
The consent and spend of an automatic verify (model turns spent, a stamp written in the user's real viola home) and the opt-out are the founder's at P3; all of it is proved over the fake agent.
Re-reads at their next red the CI-only cli_wheel turn-left-running release red and the intermittent rig rehearsal box-empty red, reading logs and events before any mend.
@task_file n=1 path=src/run/ mark=edits
what: viola run's start and its unverified CLI version gate, and revive's start where it shares the gate — edits: the version gate at start
@task_file n=2 path=src/cmd/verify
what: viola verify, the sole stamp writer, as the process the automatic form starts; its runs, probes, typed bytes and dialog answers kept unless the founder's form needs them
@task_file n=3 path=viola-state
what: on-disk product state — the stamps writer update_stamps, the stamps lock, and config.json's closed key set if a consent key is the founder's form
@task_item n=1 part=needs
The unverified CLI version gate at viola run's start: a version with no stamp now triggers one automatic verify, after which the instance reads verified; the gate still needs every relied ledger row (27 at this tree) before it reads verified.
@task_item n=2 part=needs
The start of viola revive where it shares the version gate, and the order of run and revive start checks before the child spawn (strict modes, socket directory, collision) staying ahead of any automatic verify.
@task_item n=3 part=needs
Stamps written only by verify: the automatic form writes its stamp only through verify's own stamp writer under the stamps lock, the single writer of stamps; a stamp refuses a widened ledger.
@task_item n=4 part=needs
The unverified-cli refusal and its exit 12 on viola answer and the MCP answer tool, unverified CLI degrading to send only, the null hook dialog and no decision body without a driver on a stamped CLI: these keep holding for a version whose automatic verify is red or not yet landed.
@task_item n=5 part=needs
A red automatic verify: a failing ledger row is no refusal of the run, the failing rows are named to the user, no verify repeats on every start of that version, the failed attempt is recorded and what clears it is named.
@task_item n=6 part=needs
viola verify's outputs the automatic form surfaces or reads: the stamped line, the step and summary lines, the last line, and the process-exit details (interrupted, version-unreadable, program-not-found, strict-modes-failed, pinned-hash-mismatch, unclean-fixture).
@task_item n=7 part=needs
What viola run may spawn: an automatic verify started by run is a child process — the claude child environment strip and what the child holds of viola, tied children spawned on a thread that outlives them, process-start and process-exit records for it.
@task_item n=8 part=needs
The human is never blocked: an automatic verify running before or beside the session must not hold the human's terminal or the passthrough, and the session stays the human's when verify is red.
@task_item n=9 part=needs
The consent and opt-out: if the founder's form is a config key, it joins config.json's closed key set, a bad value falls back to its default, and no switch disables a security control; what the product tells the user before the first automatic verify is a human-output line pinned by literal.
@task_item n=10 part=needs
Strict-modes and home read rules over the home the automatic verify writes into: verify, run and revive keep their strict-modes refusals, a home another user may read is refused, private filesystem permissions on the stamp it writes.
@task_item n=11 part=needs
Proof over the fake agent: stamps in tests come only from verify against the fake agent, a new CLI version is presented by the fake agent's --cli-version, the real claude CLI is never touched in CI, and any live start is a scratch viola home under the rig, never the live home.
@task_item n=12 part=needs
Every test verify start tied to a test-owned home and a test-owned user home; verify-driven tests under their per-test kill bound and the verify-window kill; the kill witness for a tied stamping verify.
@task_item n=13 part=needs
Compatibility across CLI versions: recorded fixtures per CLI version, every CLI-version fixture set replayed, fixtures recorded by verify only when no row fails and free of real data.
@task_item n=14 part=needs
The harness's view of the stamp: harness boot making the home and stamping it, the boot refusal reason verify-failed, one verify stamp per build, the polled cli_verified field of agent-run status.
@task_item n=15 part=needs
The CI watch carries, re-read at their next red: the cli_wheel release of a turn left running (send exit 0 where 13 is expected) and the rig rehearsal box-empty after x and del; no bound, retry rule, assertion or wait loosened to get past either.
@task_standing n=1
convention: each new test creates and owns its own private home and user home, seeds its own data, and leaves product state to viola rather than hand-writing it
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=2
convention: each new test is named by the test function naming rule, sits in a root integration binary prefixed by its surface, and uses the shared helpers rather than copies
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: no new test sleeps to synchronise, retries, parks with ignore, or mutates the process environment; waits key on logged events
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each new CLI output, human line or refusal is pinned by a literal assert, without escape bytes when not on a terminal
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=5
convention: each new log line is one structured JSON line with its fixed fields and corr where defined, codes only, carrying no secret
home: log-format
@task_standing n=6
convention: each new verify-driven test stays under its per-test kill bound and below the mutants timeouts
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=7
convention: each new stamping test runs verify against the fake agent under a test-owned user home, one stamp per build
home: test-data-bootstrap
@task_standing n=8
convention: the change keeps coverage at the tier floors and its mutants caught at the epoch boundary gate
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_item n=1 part=not_needed
MCP tools/list tool set, MCP invalid-params fields and MCP stdin line caps: no MCP tool is added or changed.
@task_item n=2 part=not_needed
Wrapper channel JSON-RPC error codes and the served link and unlink methods: the chunk adds no channel method and changes no frame.
@task_item n=3 part=not_needed
viola send confirmation, paste framing, slash-command post-conditions and send refusal order: the chunk changes what a send meets only through the version gate, not how a send is typed or confirmed.
@task_item n=4 part=not_needed
viola list board columns, row escaping and the human hint causes: the board is not changed.
@task_item n=5 part=not_needed
viola wait and viola last wake kinds, cursors and timeouts: untouched.
@task_item n=6 part=not_needed
Hook deadlines, the fail-open spine and the performance budgets of the hooks: the chunk adds nothing on the hook path.
@task_item n=7 part=not_needed
Heartbeat staleness classification and instance liveness: untouched by stamping.
@task_item n=8 part=not_needed
Fuzz targets, corpora and nightly fuzzing: the chunk adds no parser surface.
@task_item n=9 part=not_needed
The pre-push gate's stages, details and reasons and the secret-scan hit classes: the chunk changes neither tool.
@task_item n=10 part=not_needed
Windows and macOS support and a browser test suite: absent and not reached.
