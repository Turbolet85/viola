@task_prose part=chunk
A test-only corrective chunk that makes a green mutation baseline and a green coverage suite reachable for the next boundary's audit, and runs neither.
It gives the hook fail-open test's channel endpoint a socket name that does not grow with the pid or the event, and proves its length bound under the mutation scratch with a 7-digit pid.
It widens the nextest mutants profile's slow-case kill and moves the shared receipt wait with it, re-measuring the ~5 s slow cases' wall times at HEAD with the host's load beside them.
It kills the ten surviving mutants in the viola-hypr-tap recorder with tests that fail on each mutated site (or the test plan states why the recorder's main is outside the tier).
It enforces the record-home naming rule: a test home handed to viola verify --record never holds a path component named home, through the test-home helper.
@task_file n=1 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open integration tests and their test channel endpoint, named viola-test-chan-<pid>-<label> under a temp dir — edits: the test channel endpoint's socket name and its bind, plus a test proving the path length bound
@task_file n=2 path=.config/nextest.toml mark=edits
what: the nextest configuration, the mutants profile that mutation runs use and its slow-case kill — edits: the mutants profile's slow-timeout (period and terminate-after)
@task_file n=3 path=tests/support/watch.rs mark=edits
what: the root tests' shared receipt wait bound WITHIN, kept strictly below the mutants profile's kill, used by every fake wait_for caller — edits: the WITHIN constant and its doc comment
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the viola-hypr-tap test-only recorder binary in the viola-e2e crate, recording Hyprland exchanges — edits: its tests for main's argument guard and record / record-dispatch arms, version_of's Version arm and texts' array arm; main only if a test proves a recorder line wrong
@task_file n=5 path=tests/support/home.rs mark=edits
what: the test-home helper giving each test its own viola home (TestHome::new, the home() fixture, scratch()), from which every verify --record caller takes its home — edits: the home-naming rule so no path component is named home
@task_item n=1 part=needs
The test channel endpoint name viola-test-chan-<pid>-<label> and the rule that test channel endpoints stay outside the product endpoint namespace: the new name must not grow with the pid or the event label and must stay a filesystem socket path (no abstract-namespace socket), short like the product's hashed endpoint names.
@task_item n=2 part=needs
The socket path length bound: the test's endpoint path under the mutation scratch directory the mutation-scratch row fixes, with a 7-digit pid and the longest label, must fit sun_path's 107 bytes, proved by a computed or asserted bound in a test; that scratch row itself is read, never amended.
@task_item n=3 part=needs
Mutation testing at the epoch boundary and the mutants profile: mutation runs use their own target dir and the nextest mutants profile; the profile's slow-case kill is resized to clear the ~5 s class in a cold parallel baseline while staying below cargo-mutants' 20 s auto-timeout floor.
@task_item n=4 part=needs
Root test waits share one bound: the receipt wait WITHIN must stay strictly below the mutants profile's kill, so the two move together; whether a per-call or a scaled bound is the narrower change is closed at P3 from the wait_for callers.
@task_item n=5 part=needs
The slow cases' wall times (the instance-state, chaos revive and chaos snapshot-replay cases named in the scope, and the verify resume-run case red in the coverage suite) re-measured at HEAD against the new margins, with the host's load recorded beside each.
@task_item n=6 part=needs
The viola-hypr-tap binary is a test-only binary of the test-only viola-e2e crate, kept out of release builds; its ten survivors are each killed by a test that fails on the mutated site, or the test plan's tier text states why the recorder's main is outside the tier.
@task_item n=7 part=needs
Proof that a new test kills its mutant: a one-off remove-the-guard control run through the gate's control step under a kill deadline, its case the test's name alone, holding only when that case ran and failed.
@task_item n=8 part=needs
The record-home naming rule: a test home given to viola verify --record never holds a path component named home, enforced in the test-home helper so a plan cannot name it wrong; it serves the rule that recorded fixtures are recorded only from a clean verify and are scrubbed of home paths (the unclean-fixture home-path refusal).
@task_item n=9 part=needs
Each test in its own viola home, created through the shared helper and removed only by its owner; temp dirs placed under TMPDIR (a tempfile behaviour measured at P3, not assumed).
@task_item n=10 part=needs
Test-only bound: no product crate line changes unless a test proves one wrong; a red or defect found now folds into this chunk as an inferred item with its own acceptance, one found at P3 outside scope is a carry-forward to its owner entry.
@task_item n=11 part=needs
No mutation run and no coverage run as acceptance: the coverage floors gate and the mutation baseline are read by the next boundary's audit on the green suite this chunk gives.
@task_item n=12 part=needs
No live claude start and no live desktop step: every new or changed test runs against the fake agent, and the real claude CLI stays local-only and outside this chunk.
@task_item n=13 part=needs
How a tool or crate behaves (nextest's override and profile inheritance, the slow-timeout form, tempfile's use of TMPDIR) is measured at P3 and cited, never assumed.
@task_item n=14 part=needs
Test code stays under the lint gate: formatting and clippy with warnings denied, the print and dbg macro bans, and no orphan modules for any new test module.
@task_item n=15 part=needs
No capability claimed and no requirement added: the capability ledger and its stamps are untouched by this chunk.
@task_standing n=1
convention: every new test runs against the fake agent in CI on ubuntu, never a live claude
home: ci-cd-approach
@task_standing n=2
convention: every new test takes its own isolated viola home, removed by its owner, with evidence surviving a killed test
home: occupied-resources
@task_standing n=3
convention: every new test file passes fmt and clippy with warnings denied and the print and dbg bans
home: stack-and-technologies
@task_standing n=4
convention: an OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_item n=1 part=not_needed
MCP tool, MCP result and MCP JSON-RPC error contracts: no MCP surface is touched.
@task_item n=2 part=not_needed
Wrapper channel methods, frames, fault codes and refusal contracts: no wire behaviour changes.
@task_item n=3 part=not_needed
CLI exit codes, CLI verbs and --json error documents of the frame verbs, list, link and revive: no product CLI line changes.
@task_item n=4 part=not_needed
Normalised event kinds, event data per kind and the event line shape: no event is added or changed.
@task_item n=5 part=not_needed
Instance snapshot, snapshot envelope and session liveness contracts: no state format changes.
@task_item n=6 part=not_needed
Wheel, dialog hold, send confirmation and readiness gate decisions in established decisions: no driver behaviour changes.
@task_item n=7 part=not_needed
config.json keys, configuration precedence and the human output colour environment: no configuration is read or added.
@task_item n=8 part=not_needed
Fuzz workspace, fuzz toolchain and weekly fuzzing decisions: no fuzz target is touched.
@task_item n=9 part=not_needed
Release, deployment model and install decisions (dist, cargo install, crates.io publishing): nothing ships.
@task_item n=10 part=not_needed
Design-philosophy absences such as the usage-budget gate, and the graphical, mobile and network-port absences: nothing of the kind is approached.
@task_item n=11 part=not_needed
Plugin install, plugin folder and marketplace decisions: the plugin is not touched.
