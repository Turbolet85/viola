@task_prose part=session
viola verify now prints one fixed line on stdout before the summary when a run met the claude browser-tools prompt; the summary stays the last line, and the prompt-absent stdout is unchanged.
The new stdout line is a fixed ASCII literal: verify adds no log line, span or event for it, and the browser-tools prompt reading stays uninstrumented.
The capability ledger gains one public reader, true exactly when a typed run met the browser-tools prompt; its stamps shape and the parse of the stamps file are unchanged.
The test-only live rig gains a real-home arm (vh) that runs verify under the founder's real home and Claude config, reads names and one needle only and keeps no value, and a context arm (cx) that sends /context and reads its post-condition.
Live on claude 2.1.328, the real home's stamp for that version is now a 24-row stamp, every row passing; the /context reading found no readable transcript and the send came back not-delivered:no-prompt-submitted, recorded as a reading and not a verification.
The re-pin is placed, not switched: a new release build stands beside the product binary with a rollback copy, the product binary is unchanged; CI on the pushed head read green 7 of 7.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the capability ledger of the claude agent crate: the relied rows, the stamps and the readers of a typed run, now with a public reader of whether the browser-tools prompt stood
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — the unit cases of the capability ledger's stamps, gaining the case for the browser-tools reader
@task_file n=3 path=src/cmd/verify.rs
what: edited — the viola verify command: the probes, the stamps write and verify's human stdout (the step lines and the summary line), now with the browser-tools line before the summary
@task_file n=4 path=tests/cli_verify.rs
what: edited — the integration tests of viola verify's output and exit codes against the fake agent
@task_file n=5 path=tests/support/fake.rs
what: edited — the shared fake-agent test support, now holding the browser-tools prompt screen
@task_file n=6 path=tests/cli_send.rs
what: edited — the integration tests of viola send and viola run against the fake agent
@task_file n=7 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the test-only live rig: its arms, labels, plan and summary words, now with the real-home arm vh and the context arm cx
@task_file n=8 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the test-only rig's isolation of the child environment and the test home, now with the real-home child env (no CLAUDE_CONFIG_DIR) and the names-only readers of the real Claude config
@task_file n=9 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the viola-live-rig test-only binary: its live and rehearse commands and flags, now with the target-home and claude-dir flags
@task_file n=10 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the rig rehearsal tests that drive the live rig against the fake agent
@task_item n=1 part=touches
The verify stdout contract (the step line and the summary line): a met browser-tools prompt adds one fixed line before the summary, in the founder's words; the summary stays the last line and the prompt-absent output is unchanged.
@task_item n=2 part=touches
The decision that the browser-tools prompt reading is uninstrumented: it stands, now with a stdout line that adds no log line, span or event.
@task_item n=3 part=touches
The decisions that verify lines carry no content and that no obs gate depends on verify lines: the new stdout line is a fixed literal with no input, and no gate reads it.
@task_item n=4 part=touches
The decisions that the capability ledger path and stamps are never logged and that an unreadable ledger counts as unverified: the ledger gains a public browser-tools reader and logs nothing new.
@task_item n=5 part=touches
The decision that verify is the only stamps writer: the live home's 2.1.328 stamp was rewritten as a 24-row stamp by verify, driven by the rig; the other versions' entries were kept.
@task_item n=6 part=touches
The decisions that real-CLI verify never runs in CI and CI verify runs against the fake agent only: the real-CLI verify runs happened by hand through the test-only rig, and CI stays on the fake agent.
@task_item n=7 part=touches
The decisions that test homes live under one root and the scanned root: the rig's vh arm is a test-only exception that runs under the founder's real home and Claude config, outside CI, reading names and one needle only.
@task_item n=8 part=touches
The decisions on CLAUDE environment stripping and stripped CLAUDE values never logged: the vh child env is the rig's child env without CLAUDE_CONFIG_DIR, and the rig prints no value.
@task_item n=9 part=touches
The decision that the pinned copy is placed, never replaced: the instance bin directory gained a new content key beside the old ones; the product binary re-pin is placed beside the product and not switched.
@task_item n=10 part=touches
The decisions that Claude Code transcripts and wrapped claude internals are not a contract: on 2.1.328 no session transcript stood under the config's projects when the /context send began, so the rig's tag read stayed unread.
@task_item n=11 part=touches
The wrapper send-refused detail no-prompt-submitted: the cx arm's /context send came back not-delivered:no-prompt-submitted live on 2.1.328.
@task_item n=12 part=touches
The decisions forbidding retry policies and retry-once: the open intermittent rig rehearsal red is carried with no retry rule and no loosened entry; its cause is read at the next red.
@task_item n=13 part=touches
The decision against sleep-based log waits: the cx arm polls for the transcript tag within the submit bound rather than reading once.
@task_item n=14 part=touches
The verify exit code 0 contract: the live verify on the real home read exit 0 and 24 of 24 relied rows passing.
@task_fact n=1 slug=6-log-coverage
fact: verify's browser-tools line on stdout writes no log line, span or event; the line itself is the only record that the prompt stood at start.
@task_fact n=2 slug=4-span-trace-coverage
fact: verify stdout on a met browser-tools prompt carries one fixed line before the summary, saying the prompt stood at start, viola never answers it, answer it once in claude and run viola verify again; the summary stays the last line.
@task_fact n=3 slug=9-ci-integration
fact: the test-only live rig's vh arm is the one exception to test homes under one root: it runs verify by hand under the real home and Claude config, never in CI, reads names and one needle only, and keeps and prints no value.
@task_item n=1 part=untouched
Panic capture, the panic line and panic outcome by role: no panic path changed.
@task_item n=2 part=untouched
The MCP stdio server, its tools and its refusals: no MCP surface changed.
@task_item n=3 part=untouched
The channel JSON-RPC lines, corr and conn correlation and the dialog and hook paths: no IPC method, endpoint, event or socket was added.
@task_item n=4 part=untouched
The diagnostics line schema, its fields, detail words and the event enum: no field, detail code or event kind was added.
@task_item n=5 part=untouched
Metrics, SLO budgets, the hook perf gates and the CI gates G1 to G4: no threshold, budget or gate moved, and no CI step changed.
@task_item n=6 part=untouched
The secret scan, artifact uploads and redaction rules: no upload or scan rule changed.
@task_item n=7 part=untouched
OTel, exporters, error reporters and network egress absences: nothing was added; no dependency changed.
@task_item n=8 part=untouched
config.json keys, home resolution, log file locations and modes, and the state replay and recovery lines: no config key or file path changed.
