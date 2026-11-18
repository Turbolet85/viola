@task_prose part=session
The session landed the recorded fixture set for claude 2.1.325: a clean live verify recording and the MCP tap re-record, copied byte for byte into the tree as 28 new fixture files.
It extended the test-only live rig with two recording arms: VR (word vr, labels R4 and R4-2) keeps the verify start's record, and MT (word mt, label R4M) spawns claude -p against the test-only MCP tap in front of viola mcp under the rig's private config.
The VR arm refuses, uncounted and with exit 3, a scratch base that names a home path component (refused: record-home); the rig gains summary words record, mcp-child, stdout-bytes, stderr-bytes, stderr-sha, capture-lines, tap-fixture.
The stamped recorded sets every 24-row ledger probe reads moved from 2 to 3 (2.1.287, 2.1.302, 2.1.325); the recorded sets under the fixtures tree moved from 4 to 5.
Two live starts were spent (R4 24/24 green with browser-tools-prompt absent and Run E box; R4M green); no byte went into any CLI dialog and nothing was written under the real home.
No product crate, no viola output, refusal, exit code, schema or CI workflow changed; only the test-only e2e crate, one e2e probe list and the fixtures moved.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only standing live rig's arms and plan: the live labels, the VR record keep and record-home refusal, the MT arm's MCP tap start and its summary words
@task_file n=2 path=crates/viola-e2e/src/rig/isolate.rs
what: the test-only live rig's scratch isolation: keeping a recorded set out of the scratch at 0600, and the check that a path names a home component
@task_file n=3 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig test-only binary's command line: the vr and mt rehearse arms and the tap option
@task_file n=4 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal integration tests against the fake agent: the vr record keep, the record-home refusal, the mt config and teardown
@task_file n=5 path=tests/contract_ledger_probes.rs
what: the 24-row contract ledger probe suite and its list of stamped recorded fixture sets it reads
@task_file n=6 path=fixtures/claude/2.1.325/
what: the claude 2.1.325 recorded fixture set: hook payloads, the screen signatures (ready, turn, modal) and the recorded MCP tools-list exchange, replayed by the drift and ledger suites
@task_item n=1 part=touches
decision/live-readings-never-gate: the session spent two live readings (R4, R4M) to record fixtures; they gate no CI run, only the replayed recorded set does.
@task_item n=2 part=touches
decision/boundary-tests-sequenced-on-records: a new recorded set (2.1.325) now feeds the boundary and drift tests that run on records; the recorded sets grew from 4 to 5 and the stamped sets the ledger probes read from 2 to 3.
@task_item n=3 part=touches
decision/no-screen-content-parsing: the new set carries screen signature fixtures (ready, turn, modal) recorded from 2.1.325; signatures only, no wider screen parsing added.
@task_item n=4 part=touches
decision/boundary-check-never-inspects-claude-content: the MT arm drops claude's stdout and only counts and hashes stderr; the recorded content is the recorders' scrubbed output.
@task_item n=5 part=touches
decision/single-vendor-content-boundary: vendor content from 2.1.325 enters only as recorded fixtures through the recorders and the MCP tap; no new vendor-content path in the product.
@task_item n=6 part=touches
decision/cli-not-keyboard-interactive and decision/cli-no-interactive-input: the live starts put no byte into any CLI dialog; the MT start feeds claude -p a literal stdin line.
@task_item n=7 part=touches
contract/viola-verify/step-line: R4 ran viola verify live on 2.1.325 to 24/24 with its own screen; verify's step line is read, not changed.
@task_item n=8 part=touches
decision/a11y-runs-in-existing-test-suites and decision/a11y-inside-existing-suites: the new cases sit in existing unit, integration and e2e suites (rig rehearsal, ledger probes, drift); no new suite.
@task_item n=9 part=touches
decision/ci-test-gate-no-skips and decision/zero-flakiness: the ledger probe over every stamped set now runs three sets in CI; CI read green 7/7 with no skip or retry.
@task_item n=10 part=touches
decision/manual-a11y-pass-gates-nothing and decision/manual-checks-supplemental-only: the live rig round is a recording step outside CI and gates nothing.
@task_item n=11 part=touches
decision/time-bounds-fall-on-the-driver: the MT arm's bounds (the deadline, a 5 s leave wait, a 20 s tap bound) fall on the test-only rig, never on a human.
@task_item n=1 part=untouched
CLI refusal, fault, hint, exit-code and --json document contracts: the record-home refusal is the test-only rig's own word and exit, not a viola CLI refusal; no viola refusal or exit code moved.
@task_item n=2 part=untouched
Colour and SGR rules, the viola list board, captions, NAME truncation and stale-row dim: no output styling changed.
@task_item n=3 part=untouched
TUI passthrough contracts (entry, human keystrokes, terminal bytes, focus/mouse/resize sequences, feed panic, run diagnostics): the wrapped run path did not change.
@task_item n=4 part=untouched
Screen-reader targets and setups (Orca with GNOME Terminal, Orca with foot) and the cognitive-accessibility rows on fixed copy and reading level: nothing reached them.
@task_item n=5 part=untouched
The a11y CI integration rows (GitHub Actions workflow, coverage suite, no a11y artifact, gate requires coverage and doctest): the CI workflow and gate are unchanged.
@task_item n=6 part=untouched
The absences (web or graphical frontend, touch surface, internationalization, rendered page, user time limit): none gained or lost.
@task_item n=7 part=untouched
MCP server frame tool and hook call refusal contracts, link/unlink, revive and wait output contracts: no product crate reached.
