@task_prose part=session
The session recorded and landed a whole new claude 2.1.325 fixture set: a clean live verify record plus a re-recorded MCP tools/list exchange, 28 files, taken by two live starts of the test-only live rig under its private config.
The live rig gained two recording arms: VR (labels R4 and spare R4-2, a verify start whose clean record is kept) and MT (label R4M, one direct claude -p start with the test-only MCP tap in front of viola mcp).
VR refuses, uncounted and before any ledger line, a scratch base whose path or canonical form holds a component named exactly home (refused: record-home, exit 3); the kept record goes to target/live-rig/<label>/record/ at 0600, never overwriting.
The stamped recorded sets the 24-row ledger probes read went from 2 to 3 (2.1.287, 2.1.302, 2.1.325); recorded sets under the fixtures tree went from 4 to 5.
Measured on 2.1.325: 24/24 relied rows, browser-tools prompt absent, verify Run E resumes to the input box, the tools/list exchange holds no usage-budget text; nothing written under the real home.
Only the test-only e2e crate changed; no product crate, format, schema, dependency or CI workflow moved. StopFailure's live payload is not part of this set: its owner moves to its own route entry.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig's core: its arms, labels, plan, start sequence, ledger and summary words, and the new MCP-tap recording arm
@task_file n=2 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's scratch isolation: private scratch homes, the child's stripped environment, the kept frames and now the kept record and the home-component check
@task_file n=3 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig test-only binary's command line: its rehearse arms and live labels, now with vr, mt and the --tap option
@task_file n=4 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal integration tests against the fake agent
@task_file n=5 path=tests/contract_ledger_probes.rs
what: the capability-ledger probe contract suite, run over every stamped recorded fixture set, its STAMPED list
@task_file n=6 path=fixtures/claude/2.1.325/
what: the recorded claude fixture set for CLI 2.1.325: scrubbed hook payload captures, screen signature fixtures and the recorded MCP tools/list exchange
@task_item n=1 part=touches
name/binary/viola-live-rig: the occupied rig binary gains live labels R4, R4-2, R4M, arm words vr and mt, the --tap option, the refusal word record-home and the kept record dir target/live-rig/<label>/record/.
@task_item n=2 part=touches
name/binary/viola-mcp-tap: the test-only tap is now started live by the standing rig's R4M arm, in front of viola mcp, under the rig's private config.
@task_item n=3 part=touches
decision/mcp-tools-list-held-to-recorded-exchange: a new recorded claude tools/list exchange (2.1.325) now stands beside 2.1.302's, re-recorded through the tap.
@task_item n=4 part=touches
decision/verify-record-only-clean: the 2.1.325 set comes from a clean verify --record (24/24) kept by the rig's VR arm.
@task_item n=5 part=touches
decision/recorded-fixtures-scrubbed: a new recorded set landed, scrubbed by the recorders and hygiene-walked green.
@task_item n=6 part=touches
decision/screen-fixtures-signature-rows-only: new screen fixtures (modal, ready, turn) for 2.1.325 under the signature-rows rule.
@task_item n=7 part=touches
contract/probe-capture/payload-file: new captures in the <PascalEvent>.<k>.json shape for 2.1.325, including SessionStart.resume-1.
@task_item n=8 part=touches
decision/verify-run-e-resumes-run-b-first-session: measured on 2.1.325, Run E resumes to the input box and its SessionStart resume payload is recorded.
@task_item n=9 part=touches
decision/browser-tools-prompt-ledger-row: on 2.1.325 under the rig's seeded-off key the browser-tools prompt reads absent.
@task_item n=10 part=touches
decision/cli-capability-ledger and decision/version-verified-all-relied-rows: 2.1.325 counted 24/24 relied rows; the stamped sets every ledger probe reads moved from 2 to 3.
@task_item n=11 part=touches
decision/real-cli-probe-local-only, decision/real-claude-local-only and decision/no-live-claude-in-ci: two live claude starts ran locally through the rig only, never in CI.
@task_item n=12 part=touches
decision/e2e-test-only and decision/test-binaries-never-released: every change sits in the test-only e2e crate and its rig binary.
@task_item n=13 part=touches
decision/no-shell-out and decision/bounded-readers (indirect, through the rig): the MT arm spawns claude and the tap directly with no shell, and reads pipes and capture bounded by MAX_FRAME.
@task_item n=14 part=touches
decision/private-mode-before-write (indirect): the kept record and the MT arm's mcp.json are written 0600 through the private replace helper.
@task_item n=15 part=touches
decision/child-env-claude-strip and decision/claude-env-stripped-from-child (indirect): the MT arm's claude child runs on the rig's stripped child environment with a private CLAUDE_CONFIG_DIR.
@task_item n=16 part=touches
contract/hook-to-kind-map/stopfailure and decision/partial-gate-on-unverified-cli: a live StopFailure payload is not in this set; its owner moves from the 2.1.325 set to its own route entry, the exception staying provisional.
@task_item n=17 part=touches
decision/test-homes-cleaned-by-owner (indirect): the rig's scratch, holding the MT arm's mcp.json, is torn down whole with a teardown census of 0.
@task_fact n=1 slug=occupied-resources
fact: viola-live-rig occupies the live labels R4 and spare R4-2 (the VR arm, word vr) and R4M (the MT arm, word mt), the --tap option, and keeps a clean record under target/live-rig/<label>/record/, gitignored with the frames.
@task_fact n=2 slug=occupied-resources
fact: the rig's VR arm refuses uncounted (exit 3, refused: record-home) a scratch base whose path or canonical form holds a component named exactly home, before any ledger line, so no record is kept under a home tree.
@task_item n=1 part=untouched
Product crates (core, channel, state, pty, agent-claude, mcp, hyprland, the root bin): no product code changed.
@task_item n=2 part=untouched
Wrapper channel, JSON-RPC frames, fault codes, event kinds, event data and snapshot contracts: no wire or on-disk format moved.
@task_item n=3 part=untouched
CLI verbs, exit codes, --json documents and refusal details of run, revive, verify, list, link and plugin install.
@task_item n=4 part=untouched
MCP tool results and errors served by viola mcp: the server is unchanged; only a recording of its exchange was taken.
@task_item n=5 part=untouched
Dependency policy, toolchain and MSRV pins, cargo-deny bans, lints and feature rules: no dependency added.
@task_item n=6 part=untouched
CI workflows, release, deployment, coverage floors and the pre-push gate: no gate or workflow changed.
@task_item n=7 part=untouched
Wheel, dialog hold, send confirmation and readiness gate decisions: no runtime behaviour of viola changed.
