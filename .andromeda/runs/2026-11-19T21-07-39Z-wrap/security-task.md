@task_prose part=session
The Claude adapter's compiled local-command list gains /context, confirmed by the stdout tag it writes to the transcript, and the relied ledger set gains its 25th row, local-command-context: /context writes its stdout tag and submits no prompt.
verify's Run B now pastes eight compiled literals, the eighth /context, and stamps its latency; a stamp holding only the 24 earlier rows now reads unverified, so the live home's 24-row stamp of claude 2.1.328 reads unverified until the product re-pin step re-stamps it.
A wrapped send of /context is confirmed by a new stdout tag; on an unverified CLI, or before its transcript exists, it answers ok with confirmed false (unconfirmable). The send path's product code is unchanged; no channel method, endpoint, socket, port or env var was added.
The test-only live rig gains two arms: CS runs one verify into its scratch home and keeps the stamp, reading the top level of its private claude config as plain names and file counts (config-top); CC seeds that kept stamp, sends twice and reads the attribute words of every drawn hint cell past the cursor's cell.
Measured live on claude 2.1.328 under the rig's private config: /context passed as a verify probe (25/25) and as a wrapped send (one tag, no prompt submitted); the live home's stamps were unmoved; the hint reading was unread (no cell drawn).
The /context paste's own residual could not be separated: config-top was read after the whole verify (9 claude processes) only.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: edited; the Claude adapter's compatibility ledger: the compiled local-command list, the relied ledger rows and their checks, the verify probe literals, the stamps file entry writer (merge_stamp)
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited; unit cases of the compatibility ledger
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited; unit cases of the ledger rows' checks
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: edited; unit cases pinning the verify framing probe literals and the local-command list
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: edited; unit cases of the ledger checks over verify's typed probe runs
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited; unit cases of the stamps file: which stamped rows read a version verified or unverified
@task_file n=7 path=src/cmd/verify.rs
what: edited; viola verify, the only writer of the stamps file: its step lines and pass/fail summary over the relied rows
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: edited; verify's typed probe runs (Run B among them) against the real claude child
@task_file n=9 path=src/cmd/verify/typed/turns.rs
what: edited; verify Run B's framing turns: the compiled pastes typed into the verify child and the transcript reads after each
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: edited; the test-only fake agent binary (fake-agent feature): its --framing echo of the verify probe literals
@task_file n=11 path=src/run/send/classify.rs
what: edited; the wrapper's send classifier: listed local commands and how their delivery is confirmed from the transcript
@task_file n=12 path=src/run/version_gate.rs
what: edited; viola run's version gate: reads the stamps file and refuses an unverified CLI
@task_file n=13 path=tests/cli_send.rs
what: edited; integration tests of viola send
@task_file n=14 path=tests/cli_verify.rs
what: edited; integration tests of viola verify
@task_file n=15 path=tests/cli_verify/runs.rs
what: edited; integration tests of verify's probe runs
@task_file n=16 path=tests/cli_verify/stamps.rs
what: edited; integration tests of the stamps verify writes
@task_file n=17 path=tests/contract_ledger_probes.rs
what: edited; contract tests replaying the stamped recorded fixture sets through the ledger probes
@task_file n=18 path=crates/viola-e2e/src/harness/run.rs
what: edited; the e2e harness run: its pinned list of ledger rows and the live verify output it expects
@task_file n=19 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the test-only live-round rig: its arms and labels, its private claude config and scratch home, its kept stamp, its closed summary words
@task_file n=20 path=crates/viola-e2e/src/rig/screen.rs
what: edited; the live rig's screen model: reads of the input box and its hint cells as attribute words
@task_file n=21 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited; the live rig binary: its rehearse and live commands
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the live rig's rehearsals against the fake agent
@task_item n=1 part=touches
The row accepting verify's echo probe residuals names the pastes up to the seventh (/mcp disable viola-probe-absent); Run B now pastes an eighth compiled literal, /context, whose residual is owed beside it.
@task_item n=2 part=touches
Rows holding verify's screen and transcript reads to compiled literals and signatures only: the new /context probe is one more compiled literal read the same way; check none of them lists the literals.
@task_item n=3 part=touches
Rows accepting verify's real-CLI transcripts and holding probe effects to their own directory: Run B's /context paste writes one more stdout tag into the probe transcript.
@task_item n=4 part=touches
The unverified CLI gate and the version gate's stamps read: the relied set moved 24 to 25 rows; a stamp of only the 24 earlier rows now reads unverified, the live home's 2.1.328 among them until re-stamped.
@task_item n=5 part=touches
The convention that only viola verify writes the stamps file: the test-only rig now copies verify's stamp out of its CS scratch home (kept no-clobber, owner-only) and seeds it into CC's scratch home's ledger; a rig exception may be owed.
@task_item n=6 part=touches
Rows reading the stamps file through the strict-modes check first: the stamps entry gains a row and a measured latency field, and a seeded scratch stamp is now read by the gate.
@task_item n=7 part=touches
The live rig's context arm keeps no transcript byte: CC shares that arm's start, transcript and reading paths, and counts the /context tag only.
@task_item n=8 part=touches
The live rig's prompt capture keeps closed words only: new closed summary words kept-stamp, config-top, stamp-seeded, send-1-turn, send-2-reply, hint-cells, hint-cells-attr, typed-attr, hint-told-apart, local-command-context.
@task_item n=9 part=touches
Rows isolating the live rig's child from the user's config, seeding it from bundle logic and keeping its credentials copy transient: CS and CC ran under that private config; teardown removed credentials and scratch, user config unchanged.
@task_item n=10 part=touches
Live session screens kept local: the rig's new hint-cell reading reads attribute words only, never text; the row stands as the home of kept local raw frames (the a11y except's clause is ruled against it).
@task_item n=11 part=touches
Rows that viola writes nothing under the Claude home: CS's config-top reads the rig's private config only; nothing written under the real claude home or the live viola home.
@task_item n=12 part=touches
A wrapped send of /context is one bracketed paste through the paste text validation; its confirmation counts the transcript tag only, no byte kept, under the bounded count-only transcript read.
@task_item n=13 part=touches
Verify children spawned directly and probe prompts synthetic: /context is a synthetic compiled literal; verify's claude children stay nine.
@task_item n=14 part=touches
Test seams only under the fake-agent feature, and a new seam or env read needs a log entry: the fake's --framing echo now takes the /context literal; no option and no env read were added.
@task_item n=15 part=touches
The live-round rig keys only into its own pty: CC types x and DEL into its own child after the second send.
@task_item n=16 part=touches
The scratch footer reading is no standing permission: the new hint-cell and config-top readings are likewise readings of one start, not permissions.
@task_item n=17 part=touches
The live rig's cache-key name-only read: config-top likewise reads names and counts only, names outside [A-Za-z0-9._-]{1,40} read as other.
@task_fact n=1 slug=data-protection
fact: verify's Run B pastes eight compiled literals, the eighth /context; its residual is accepted as measured only: the rig's private claude config after a whole verify, the share of the /context paste alone not separated.
@task_fact n=2 slug=data-protection
fact: the live rig's config-top reading keeps the top-level names of its private claude config and their regular-file counts only, a name outside [A-Za-z0-9._-]{1,40} read as other, no file content read.
@task_fact n=3 slug=data-protection
fact: the live rig keeps a stamp from its CS scratch home owner-only and no-clobber, and seeds it only into its own CC scratch home; CC with no kept stamp refuses before its ledger line and before any scratch.
@task_fact n=4 slug=input-validation
fact: the live rig's CC reading takes the attribute words of every drawn hint cell past the cursor's cell, never their text, as closed words.
@task_fact n=5 slug=threat-model-summary
fact: a wrapped send of a listed local command before its transcript exists, or on an unverified CLI, answers ok with confirmed false (unconfirmable), never a confirmation.
@task_item n=1 part=untouched
The wrapper channel methods, the MCP tools and their contracts, the socket, peer-euid and server-verification rows: no method, endpoint, socket or port was added or changed.
@task_item n=2 part=untouched
Dependency security, CI workflows, actions pinning, advisory and licence lists, toolchain pins: no dependency, CI step or tool version moved.
@task_item n=3 part=untouched
Error handling contracts, CLI exit codes, refusal words and the strict-modes refusal contracts: none gained or lost a code.
@task_item n=4 part=untouched
Recorded fixture hygiene, fixture refusal codes and the stamped recorded sets: no new recording was made.
@task_item n=5 part=untouched
Secret management's credential strip, identity floor and canary scan rows: no env var, credential handling or upload changed.
@task_item n=6 part=untouched
revive, link and unlink, list, hook and config.json rows: none of those surfaces moved.
