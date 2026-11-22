@task_prose part=session
The session measured whether real claude on Linux loses a key typed right after a resize of its own pty, through a new KR arm of the standing live rig, and found no loss: 2.1.330 (KR) and 2.1.328 (the spare KR8-2) each kept 12 of 12 keys typed 0, 20 or 100 ms after a resize; KR8 was cut at round 8 and recorded as read, unverified.
The reading is said as no loss in 12 rounds at these delays on these versions, never as never loses; it measures the CLI unwrapped, not viola's own poll lag or the driver's paste, so no product fix was needed and no product, pty or fake-agent file changed.
The rig gained the KR arm (labels KR, KR-2, KR8, KR8-2) under the grant box,resize, its summary keys of closed words, counts, delays and settle times, and a KR-only after-key settle that counts quiet from a byte drained after the key; the shared settle of every other arm is unchanged.
The rig tells a late redraw (box clean on its re-read, the round kept, counted late-redraw) from a dirty box (still dirty on re-read, the start stops unverified).
The rehearse verb gained two rehearse-only options for the kr arm alone, a quiet window of 100 to 400 ms and a seam that withholds or delays the DEL of a round; any other arm, a malformed value, or a live start with either is refused usage, exit 2, before any ledger line.
Five KR rehearsals over the fake agent were added; nothing was retired.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig's core in the test-only E2E crate: its arms, its session pump and settle, its live-start ledger and summary keys, now the KR arm, its after-key settle and its seam and quiet options
@task_file n=2 path=crates/viola-e2e/src/rig/keys.rs
what: the live rig's key grant: which keys and actions a start may type into its own pty, now the resize grant word
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's screen model that reads box-row cell attributes, never text, now resizable to a new width
@task_file n=4 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary's command line, its rehearse, live and census verbs, now the rehearse-only quiet and seam options refused as a usage error elsewhere
@task_file n=5 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsals: the integration tests that run the live rig over the fake agent, now the five KR rehearsals
@task_item n=1 part=touches
decision/key-after-resize-waits-for-size: its except now owes the KR rehearsal beside the hold sibling, since the KR rehearsals type a key right after a resize over the fake agent; the Linux reading of the question (no loss in 12 rounds on 2.1.330 and 2.1.328) belongs beside it.
@task_item n=2 part=touches
decision/standing-live-rig: the rig gains the KR arm and its labels; the kr rehearsals take rehearse-only options (a quiet window of 100 to 400 ms and a withheld or late DEL seam) that live refuses as usage, exit 2; the KR row names its after-key settle and its late-redraw count.
@task_item n=3 part=touches
decision/no-child-screen-assertions: its except now names KR, which reads its box row's cell attributes, never text.
@task_item n=4 part=touches
The rules that the rendered child screen is never parsed (decision/no-screen-parsing, decision/never-parse-rendered-child-screen, decision/no-screen-scraping-in-e2e, decision/screen-signatures-only): KR reads box-row attributes only; any row listing the rig's attribute-reading arms as its exception now owes KR.
@task_item n=5 part=touches
decision/live-failing-reading-not-verification and decision/live-rig-hint-cell-reading: KR8 cut at round 8 is recorded as read, unverified; KR and KR8-2 kept 12 of 12 are readings, never a verification that claude never loses a key.
@task_item n=6 part=touches
decision/rig-no-key-read-from-keys-word: the KR rehearsals read each key kept from the box's attribute words and the fake agent's receipt, not from the rig's own keys word.
@task_item n=7 part=touches
decision/real-claude-local-only, decision/real-claude-never-in-ci, decision/no-real-cli-in-ci and decision/ci-never-real-claude: the KR live arm runs real claude locally under a cap and spare and is refused under CI.
@task_item n=8 part=touches
decision/fake-agent-not-proof-of-real-cli: the KR rehearsals over the fake agent prove the rig's reading only; the question itself was answered by the live KR and KR8-2 starts on real claude.
@task_item n=9 part=touches
decision/compat-tests-for-cli-versions and decision/multi-version-trigger-layers: the reading was taken on two CLI versions, 2.1.330 and 2.1.328, the spare kept on 2.1.328.
@task_item n=10 part=touches
decision/no-sleep-synchronization, decision/no-test-retries-or-sleeps and decision/no-test-owned-real-time: KR types keys at fixed delays of 0, 20 and 100 ms after a resize and settles on a quiet window; those delays are the measurement, not synchronisation, and the settle keys on bytes read.
@task_item n=11 part=touches
contract/fake-agent-receipt/size and contract/fake-agent-receipt/key: the KR rehearsals read the fake agent's size and key receipt lines to hold both sizes and each key typed.
@task_item n=12 part=touches
decision/test-only-e2e-crate and decision/single-test-only-crate: every change sits in the test-only E2E crate; no product crate depends on it.
@task_item n=13 part=touches
decision/pure-functions-tested-directly and decision/unit-tests-inline: the settle rule and the box word (clean, late-redraw, dirty) are pure functions held by inline rig cases.
@task_item n=14 part=touches
decision/test-function-naming and decision/integration-binary-surface-prefix: the five new rehearsals are named rig_rehearsal_kr_ in the existing rig_rehearsal binary.
@task_item n=15 part=touches
decision/tests-never-read-host-sessions: the KR starts keep their frames owner-only under the gitignored live-rig target dir, never read into evidence, a message or the tree.
@task_fact n=1 slug=integration-test-strategy
fact: On Linux, real claude 2.1.330 and 2.1.328 each kept 12 of 12 keys typed 0, 20 or 100 ms after a resize of their own pty: no loss in 12 rounds at these delays on these versions, never a claim that it never loses; the reading is of the CLI unwrapped, not of viola's poll lag or the driver's paste.
@task_fact n=2 slug=6-e2e-test-strategy
fact: The KR arm's after-key settle counts its quiet window only from a byte drained after the key, never from a byte before it; every other arm keeps the shared settle unchanged.
@task_fact n=3 slug=6-e2e-test-strategy
fact: The KR arm tells a late redraw from a dirty box: a box clean only on its re-read keeps the round and is counted late-redraw; a box still dirty on its re-read stops the start, recorded unverified.
@task_fact n=4 slug=6-e2e-test-strategy
fact: A rehearsal of a refusal that prints none of the shared teardown words reads the refusal's own effects (no ledger, an empty receipt, no process the rig started) instead of the shared teardown check.
@task_item n=1 part=untouched
Viola's own product resize handling and wheel rules (decision/focus-mouse-resize-keep-wheel, decision/resize-baseline-is-spawn-size): the measurement resized the CLI's own pty unwrapped; no product code changed.
@task_item n=2 part=untouched
The driver send and paste rows (decision/paste-injection-test-layers, decision/paste-validated-at-both-ends, the driver-send and viola-send contracts): the driver-paste fix was not needed and nothing in the send path moved.
@task_item n=3 part=untouched
The fake agent's command line, screens, input and hook contracts: no fake-agent file changed and no fake-agent option was added.
@task_item n=4 part=untouched
The test harness contract, agent-run status shape, boot, cleanup and pre-push rows: no harness verb, status field or exit code changed.
@task_item n=5 part=untouched
CI integration, coverage floors, mutation gates, perf budgets and supply-chain rows: no workflow, gate, threshold, dependency or tool pin changed.
@task_item n=6 part=untouched
The MCP, wrapper channel, CLI exit-code, event-log, snapshot and log-format contracts: no product surface changed.
@task_item n=7 part=untouched
Test data bootstrap, test homes, fixtures and stamps: no fixture was recorded and no home, stamp or verify rule moved.
