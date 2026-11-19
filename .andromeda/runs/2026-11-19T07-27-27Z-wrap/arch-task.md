@task_prose part=session
viola verify now prints one line before its summary when a run met the browser-tools prompt: the prompt stood at start, viola never answers it, answer it once in claude and run verify again; the summary stays the last line and the prompt-absent stdout is unchanged; no log line, span or event is added.
The agent crate gains a public pure reader, browser_tools_stood, true exactly when a run met the browser-tools prompt; the private reading keeps its callers.
The test-only live rig gains two arms: vh, a live start under the founder's real home and real Claude config that reads key names and one needle only and keeps no value; and cx, a send of /context through viola send whose post-condition is read from the session transcript, bounded by MAX_FRAME.
Measured live on claude 2.1.328: L1 under the real home read verify 24/24, every relied row passing, trust-record pass and browser-tools-prompt absent; the live home's 2.1.328 stamp entry is now a 24-row stamp replacing a 21-row one, other versions' entries kept; /mcp disable left its needle absent in the real Claude config; L2's /context send read not-delivered no-prompt-submitted with the transcript absent, so its post-condition stays unverified.
CI on the pushed head was green 7/7; the release build of that head was placed beside the pinned product with a rollback copy, the product binary itself not switched.
No IPC method, endpoint, event kind, socket, port, env var, config key, stamp shape or dependency was added; nothing was retired.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the per-behaviour CLI capability ledger in the agent crate: relied rows, their probe readings including the browser-tools-prompt row, and the stamps check; edited to add a public reader of whether a run met the browser-tools prompt
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the capability ledger's stamps and row readings; edited to add a case for the new browser-tools reader
@task_file n=3 path=src/cmd/verify.rs
what: the viola verify command in the root bin: runs the probes, writes the ledger stamps as the one stamps writer, and prints the step lines and the summary line on stdout through the one human writer; edited to print the browser-tools line before the summary
@task_file n=4 path=tests/cli_verify.rs
what: integration tests of viola verify's stdout, exit codes and refusals against the fake agent; edited to add the browser-tools line case
@task_file n=5 path=tests/support/fake.rs
what: shared test support that scripts the fake agent's screens; edited to hold the browser-tools prompt screen used by two test files
@task_file n=6 path=tests/cli_send.rs
what: integration tests of viola send and run against the fake agent; edited to use the shared browser-tools screen
@task_file n=7 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig in the viola-e2e crate: its arms, labels, plans and summary words for live starts of the real claude CLI; edited to add the real-home arm vh and the /context arm cx and their readings
@task_file n=8 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation of each start: the child environment, scratch homes and the bounded name-only readers of a Claude config and the stamps file; edited to add the real-home child environment and the key-name, needle and other-stamps readers
@task_file n=9 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig test-only binary: its live and rehearse subcommands and their flags; edited to add the vh and cx arms and the rehearse flags target-home and claude-dir, vh refused as usage exit 2 without an explicit home, claude dir and user config
@task_file n=10 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsals that run the live rig against the fake agent; edited to add rehearsals of the vh and cx arms and a kept-frame reader
@task_item n=1 part=touches
viola verify stdout summary line and step line contract: a new fixed line now precedes the summary when the browser-tools prompt was met; the summary stays the last line; the prompt-absent stdout is unchanged.
@task_item n=2 part=touches
The browser-tools-prompt ledger row and its readings: its reading now surfaces on verify's stdout through a new public reader in the agent crate; on claude 2.1.328 the row read absent live for the first time.
@task_item n=3 part=touches
Verify never answers or types into a CLI dialog: the new line restates it to the human (viola never answers the browser-tools prompt); the rule stands, now said on stdout.
@task_item n=4 part=touches
viola-agent-claude does no file I/O and claude shapes stay in one crate: the new reader is pure over a typed run and lives in the agent crate; the rule holds.
@task_item n=5 part=touches
One human writer in the root bin and the no-print-macros ban: verify's new line goes through the one human writer, not a print macro.
@task_item n=6 part=touches
The stamps rules (verify is the one stamps writer, stamps merge replaces only its own version, the stamps writer checks strict modes): the live home's 2.1.328 entry was replaced by a 24-row stamp and the other versions' entries were read kept.
@task_item n=7 part=touches
A CLI version is verified only when every relied row counts, and the capability ledger's relied-row count of 24: 2.1.328 now holds all 24 passing; the 2.1.325 entry stands at 21 rows and reads unverified to a binary built from this tree.
@task_item n=8 part=touches
The trust-record ledger row and its probe: trust-record read pass live on claude 2.1.328 for the first time.
@task_item n=9 part=touches
The occupied name viola-live-rig: the binary now carries the arms vh and cx, the labels L1, L1-2 and L2, and the rehearse flags target-home and claude-dir.
@task_item n=10 part=touches
viola-e2e is test-only and test-only binaries are kept out of release builds: all rig changes stay in the test-only crate; the release check read viola only.
@task_item n=11 part=touches
Each test in its own viola home, Claude env names stripped from the child, and the wrapped child's environment: the vh arm is an exception in a test-only live rig, its child env being the rig's usual env without CLAUDE_CONFIG_DIR, so the start runs under the real home and real Claude config.
@task_item n=12 part=touches
Real claude CLI probes run only locally and no live claude in CI: the L1 and L2 starts were local live starts by path on claude 2.1.328; CI stayed on the fake agent.
@task_item n=13 part=touches
Bounded readers at every boundary and the MAX_FRAME bound: the rig's real-config, stamps and transcript readers read through MAX_FRAME and keep no value.
@task_item n=14 part=touches
Environment strip reads names only: the rig's real-config readers follow the same names-only discipline (key names and one needle word, never a value).
@task_item n=15 part=touches
The local-command echo probe and local commands listed with delivery post-conditions: /mcp disable on 2.1.328 wrote nothing naming its server into the real Claude config; the /context post-condition was read only on its prompt-submitted half, not verified.
@task_item n=16 part=touches
The not-delivered no-prompt-submitted refusal detail: L2's /context send came back with it within the submit bound.
@task_item n=17 part=touches
Every send confirmed after the fact and unconfirmed delivery reported as not-delivered: the cx arm's send read not-delivered rather than claiming delivery.
@task_item n=18 part=touches
Release build yields viola only: the re-pin's release build of the pushed head printed release-check viola only.
@task_fact n=1 slug=occupied-resources
fact: The test-only viola-live-rig occupies the arms vh (labels L1 and L1-2) and cx (label L2) and the rehearse flags target-home and claude-dir; a vh rehearsal without all three of target-home, claude-dir and user-config is refused as usage, exit 2.
@task_fact n=2 slug=occupied-resources
fact: The live rig's vh arm is the one real-home exception: a live start under the user's real home and real Claude config, its child env without CLAUDE_CONFIG_DIR, reading no credentials, only key names and one closed needle word, every read bounded by MAX_FRAME and no value kept or printed.
@task_fact n=3 slug=established-decisions
fact: viola verify prints, when a probe run met the browser-tools prompt, one fixed line before the summary saying the prompt stood, viola never answers it, and to answer it once in claude and run verify again; the summary stays the last line, the prompt-absent stdout is unchanged, and the line adds no log line, span or event.
@task_fact n=4 slug=occupied-resources
fact: On claude 2.1.328 /mcp disable left its server's needle absent before and after in the real Claude config files; why no new projects entry appeared for verify's probe folder is unmeasured, and the reading is one version's, never a general measurement.
@task_fact n=5 slug=occupied-resources
fact: The /context post-condition is unverified: on claude 2.1.328 the cx arm's send read not-delivered no-prompt-submitted with the session transcript absent, so context-post none rests on the prompt-submitted half alone.
@task_item n=1 part=untouched
The wrapper channel protocol: JSON-RPC frames, channel methods, fault codes, refusal details and protocol versioning; no method or field moved.
@task_item n=2 part=untouched
The MCP server and its tools, results and exit codes; the hook contracts, hook tiers and hook-to-kind map.
@task_item n=3 part=untouched
The event log and its normalised event kinds, event data, snapshots and liveness; no event kind was added.
@task_item n=4 part=untouched
The wheel, dialog hold, link and unlink, pause and release rules; list, the board and revive.
@task_item n=5 part=untouched
config.json keys, the VIOLA_ env namespace, endpoint naming, sockets and the absence of a listening port; nothing was added.
@task_item n=6 part=untouched
The build system and dependency policy (cargo-deny, toolchain, MSRV, lint bans), the CI workflow and its jobs; no dependency or CI step changed.
@task_item n=7 part=untouched
The deployment model and the pinned-copy rules for run: the placement of a release build beside the product binary is an operator host step, not a change to how viola installs or pins its own copy.
@task_item n=8 part=untouched
The Hyprland seam, the terminal fleet console and the pty seam.
