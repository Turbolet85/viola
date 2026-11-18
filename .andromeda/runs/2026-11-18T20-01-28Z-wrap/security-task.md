@task_prose part=session
The session landed the claude 2.1.325 recorded fixture set (28 files) from two live starts of the test-only live rig on 2.1.325 by path, under the rig's private CLI config with the browser-tools key seeded off.
It added two live-rig arms in the test-only e2e crate: VR (labels R4 and R4-2), the verify start whose recorded set is kept, and MT (label R4M), one direct claude -p start with the test-only MCP tap placed in front of viola mcp.
VR refuses, uncounted and before any ledger line, a scratch base whose path or canonical form names a component exactly home (refused: record-home, exit 3); it copies the record byte for byte at 0600, never overwrites, and leaves first-screen captures out.
MT writes a private 0600 mcp.json inside its scratch, removed with it; CLI stdout is drained and dropped, stderr is only counted and hashed; the tap's fixture step runs bounded with only PATH and HOME in its env.
The stamped recorded sets read by the 24-row ledger probes grew from 2 to 3 (2.1.287, 2.1.302, 2.1.325); the recorded sets under the fixture tree grew from 4 to 5. No product crate, viola format or schema changed; CI on the pushed head is green 7/7.
The founder moved the StopFailure payload out of this fixture set to its own entry, A live StopFailure payload; the provisional StopFailure exception stays as it stands, only its owner moves.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the test-only live rig's arms and start: the VR record-keeping verify arm, the MT MCP-tap arm with its private mcp config, the record-home refusal and the rig's summary words
@task_file n=2 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's scratch isolation: the private scratch home, the record keep at owner-only modes that never overwrites, and the home-component path check
@task_file n=3 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the test-only viola-live-rig binary's command line: the vr and mt rehearse arms, the tap path option, the live labels R4, R4-2 and R4M
@task_file n=4 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the live rig's rehearsal tests: the record keep beside the frames, the record-home refusal, the MCP arm's config and teardown
@task_file n=5 path=tests/contract_ledger_probes.rs
what: edited — the contract ledger probes over every stamped recorded set; its stamped list now names 2.1.325
@task_file n=6 path=fixtures/claude/2.1.325/
what: added — the recorded fixture set for claude 2.1.325: scrubbed hook payloads, screen signature fixtures, the recorded claude agents reply and the recorded MCP tools-list exchange, under the fixture hygiene walk
@task_item n=1 part=touches
decision/unverified-cli-gate: its exception names StopFailure as owned by the 2.1.325 recorded fixture set; that is now stale — the founder moved StopFailure to its own entry, A live StopFailure payload; the exception stays provisional, only its owner changes.
@task_item n=2 part=touches
decision/mcp-fixtures-via-test-only-tap: the tap's live start is now the standing live rig's R4M arm under the rig's private CLI config (user config read unchanged); the row gains that.
@task_item n=3 part=touches
decision/live-rig-isolated-child and decision/live-rig-seed-from-bundle-logic: two new rig arms run the child under the rig's private config; MT spawns claude -p directly with its own process group and the rig's child env; VR seeds the browser-tools key off.
@task_item n=4 part=touches
decision/live-rig-credentials-copy: the builder read the real credentials file's token expiry once by hand before the round, a second read outside the founder's read-once cap; nothing kept or written; the token-window check is named the rig's own (it refuses under 20 minutes).
@task_item n=5 part=touches
decision/nothing-written-under-claude-home, decision/no-writes-under-claude-home and decision/viola-writes-nothing-in-claude-dir: the live round wrote nothing under the real Claude home or viola home; the new record-home refusal guards the kept record against a scratch base under a home component.
@task_item n=6 part=touches
decision/live-session-screens-kept-local: the kept record directory under the live rig's output (per label, record) sits beside the frames, gitignored, at 0600.
@task_item n=7 part=touches
decision/fixture-recording-scrubbed, decision/fixture-scrub-and-refusal, decision/fixtures-reviewed-before-commit and decision/fixture-hygiene: a new recorded set landed as a plain copy of the recorders' scrubbed output, walked green by the hygiene walk, with an identity read finding none; its modes were set 0644 after the copy, bytes unchanged.
@task_item n=8 part=touches
decision/screen-fixture-signature-rows-only: the new set carries screen fixtures (ready, turn, modal) recorded by the rig; first-screen captures are left out of the kept record.
@task_item n=9 part=touches
decision/max-frame-cap and decision/reads-capped-at-max-frame: the rig's new readers (the record keep, the MCP arm's pipes and capture) read bounded through MAX_FRAME.
@task_item n=10 part=touches
decision/live-round-rig-keys-own-pty and decision/verify-writes-no-dialog-byte: R4 typed no byte into any CLI dialog; MT feeds only a literal prompt on the child's stdin.
@task_item n=11 part=touches
decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude: the real claude CLI ran only locally in the live round; CI replays the recorded set with no real claude.
@task_item n=12 part=touches
decision/release-build-no-test-binaries: the changed live rig and the MCP tap stay test-only binaries; no product crate reached.
@task_fact n=1 slug=data-protection
fact: The live rig's recording arm keeps the CLI's recorded set locally at owner-only modes, never overwriting an existing copy and leaving first-screen captures out, and refuses before any start a scratch base whose path or canonical form names a component exactly home.
@task_fact n=2 slug=data-protection
fact: The live rig's MCP recording arm keeps no CLI output: stdout is drained and dropped, stderr is kept only as a byte count and a SHA; its MCP config is a private 0600 file inside the scratch, removed with it.
@task_item n=1 part=untouched
The IPC channel, socket directory, peer euid and server-verification rows: no product channel, socket or client seam changed.
@task_item n=2 part=untouched
MCP tool contracts, MCP input checks and the MCP stdin line cap of viola mcp itself: viola mcp ran unchanged behind the tap.
@task_item n=3 part=untouched
Paste text, send, answer and dialog free text validation rows and their refusal contracts.
@task_item n=4 part=untouched
Strict-modes refusal contracts, CLI exit codes, revive codes and the error-handling contracts: no product error writer changed.
@task_item n=5 part=untouched
Dependency security: no dependency, toolchain pin, deny or advisory list, workflow or CI permission changed.
@task_item n=6 part=untouched
The pinned binary, plugin install, trust record writer and start-time trust rows: the pinned product was neither rebuilt nor re-pinned.
@task_item n=7 part=untouched
Verify's own probe, stamp and unclean-fixture refusal codes: verify's code did not change; only the rig ran it.
