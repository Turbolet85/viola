@task_prose part=session
The session changed only the test side (the viola-e2e test crate and the test runner's ci profile); no product crate, IPC method, endpoint, socket, env var, dependency or crate was added or changed.
It added to the standing live rig a wake arm (R3B, R3, R3D) and an imports arm (IM), a hidden capture verb registered as the rig's own UserPromptSubmit hook in the scratch child's private settings, and a no-key group-kill stop used by the imports arm alone; every wake start stopped with Ctrl-C into the rig's own pty.
It replaced the stamp-only user-config isolation witness with a mark witness: two rig-made needles searched in the user's config after a moved stamp, the bytes dropped and only a closed word kept (unchanged, absent, changed-by-others, changed-by-rig, unread); the old word changed is retired.
It measured on the real claude CLI 2.1.325 and 2.1.326: a fired scheduled wake's UserPromptSubmit carries no source key (the bundle's loop_wakeup and schedule_wakeup words disproved), the wrapper reads such a fired prompt as origin human, and a host writer rewrites the user's config about once a minute with no rig alive.
It gave the test harness boot a ready deadline and put three more viola-e2e binaries on the ci 20 s kill line; the live verify --record and R2's placeholder-hint question were re-pinned to the 2.1.325 recorded fixture set, not done here.
@task_file n=1 path=crates/viola-e2e/src/rig/capture.rs
what: new; the live rig's hidden UserPromptSubmit capture — reads the hook payload bounded at MAX_FRAME with the default JSON depth and writes one owner-only line of listed key names, a closed source word and no value
@task_file n=2 path=crates/viola-e2e/src/rig/wake.rs
what: new; the live rig's scheduled-wake arms — drive the wrapped child through viola send, read events and capture lines as closed words, keep a session id only by its shape check
@task_file n=3 path=crates/viola-e2e/src/rig/isolate.rs
what: edited; the live rig's isolation of its child from the user's config — the user-config witness (now the mark witness), the bounded read of the user's config that follows a link only inside the home, and the scratch seeds for the capture hook settings and the imports file
@task_file n=4 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the live round rig itself — its arms and labels, its keys into its own pty, its stop (Ctrl-C, or the new no-key group kill), its teardown and summary words
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited; the test-only live rig binary — its options --wake and --stand and the hidden capture --out verb
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the rig's rehearsals against the fake agent — the mark witness cases and the wake, capture and imports rehearsals
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the test harness boot of an instance — its readiness deadline, now a field of the boot options
@task_file n=8 path=crates/viola-e2e/src/harness/run/perf.rs
what: edited; the harness perf runner — names the shared boot deadline at its boot options
@task_file n=9 path=crates/viola-e2e/tests/cli.rs
what: edited; the CLI integration tests — name the shared boot deadline at their boot options
@task_file n=10 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited; the harness lifecycle tests — the never-ready boot case now bounded by the boot deadline
@task_file n=11 path=.config/nextest.toml
what: edited; the test runner's ci profile — the 20 s kill line's filter of binaries, now covering three more viola-e2e test binaries
@task_item n=1 part=touches
decision/live-rig-isolated-child is stale: its user-config witness was a whole-file stamp read changed or unchanged; it is now the mark witness — two rig-made needles, the five closed words unchanged, absent, changed-by-others, changed-by-rig, unread, nothing of the file's content kept, and the summary's needles count.
@task_item n=2 part=touches
decision/live-round-rig-keys-own-pty gains a stop: the imports arm ends by a group kill with no byte typed (stop killed-no-key); the wake arms' stop stays Ctrl-C into the rig's own pty, and keys none names the argv grant only, never the stop; whether the row's anew-for-each-chunk binds the stop is asked of the founder, the row changed only on the answer card.
@task_item n=3 part=touches
decision/hook-capture-arm and decision/capture-arms-isolated: the rig now has its own UserPromptSubmit capture beside the product's hook capture arm, a test-only hook in the scratch child's private settings, not the product hook.
@task_item n=4 part=touches
decision/max-frame-cap, decision/reads-capped-at-max-frame and decision/bounded-readers-census: two new test-side readers (the hook payload capture and the user-config read after a moved stamp) read through MAX_FRAME, over-bound reads as unread.
@task_item n=5 part=touches
decision/json-default-depth-limit and decision/no-serde-unbounded-depth: the rig's capture parses the hook payload with serde's default depth.
@task_item n=6 part=touches
decision/hook-payload-is-content, decision/transcript-path-redacted and decision/upstream-text-is-content: the capture keeps only key names matching a closed pattern and a closed source word, never a value, so prompt and transcript_path appear as key names only.
@task_item n=7 part=touches
decision/nothing-written-under-claude-home, decision/no-writes-under-claude-home and decision/viola-writes-nothing-in-claude-dir: the rig reads the user's config after a moved stamp to tell its own write from others'; the measured host writer rewrites that file about once a minute, so a whole-file stamp cannot witness one process's isolation.
@task_item n=8 part=touches
decision/live-rig-credentials-copy and decision/tests-use-own-home: the wake arms seed a private settings file (one UserPromptSubmit hook, absolute metacharacter-free paths) and an imports seed (a CLAUDE.md importing a file outside the cwd) in the rig's own scratch.
@task_item n=9 part=touches
decision/absolute-viola-path-in-commands: the rig's own hook command uses an absolute plain path, checked shell-plain before it is seeded.
@task_item n=10 part=touches
decision/verify-never-answers-import-dialog and decision/verify-never-answers-dialogs: the imports arm reads the external-imports modal on the real CLI (modal imports, box down) and answers nothing, closing with no key.
@task_item n=11 part=touches
decision/human-typing-takes-wheel and contract/wheel-event/human-input: a fired scheduled wake's prompt reads origin human at the wrapper and carries no source key on 2.1.325 and 2.1.326; v2-36 is planned and its fate is asked at route-resolve, so the rows are unchanged here.
@task_item n=12 part=touches
decision/tolerant-hook-parsing and decision/hook-security-fields-closed: the real CLI's UserPromptSubmit on a fired wake lists cwd, hook_event_name, permission_mode, prompt, prompt_id, scratchpad_dir, session_id, transcript_path and no source.
@task_item n=13 part=touches
decision/release-build-no-test-binaries and decision/release-build-without-test-features: the live rig and its capture verb are test-only binaries; release-check still reads viola only.
@task_item n=14 part=touches
decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude: the five live starts ran the real CLI on the host only; CI ran the rehearsals against the fake agent.
@task_item n=15 part=touches
decision/e2e-home-removal-own-dirs-only and decision/test-removal-own-dirs-only: every live start tore down its scratch whole with census 0; one hand control clone directory outside the repo is left for the operator.
@task_fact n=1 slug=input-validation
fact: The live rig's own UserPromptSubmit capture is test-only and closed: it reads the hook payload through MAX_FRAME with the default JSON depth and writes one owner-only line holding only key names matching [a-z_]{1,40} (others counted), a source word from schedule_wakeup, loop_wakeup, other, absent, and no value; a non-object or over-long payload writes unread.
@task_fact n=2 slug=secret-management
fact: A witness that compares a whole shared user file cannot attribute a change on a host with other writers: the rig's isolation witness reads the user's config only after a moved stamp, searches it for rig-made needles, drops the bytes and keeps one closed word.
@task_fact n=3 slug=input-validation
fact: The rig's read of the user's config is bounded at MAX_FRAME, refuses a link leading out of the home, and reads unread when the file is absent, unreadable or over the bound.
@task_fact n=4 slug=authentication-authorization
fact: A no-key stop is within the live rig's key rule: the imports arm ends by a group kill of its own child with no byte typed, and keys none in a live summary names the argv grant, never the stop.
@task_item n=1 part=untouched
The product's IPC channel, socket directory, peer euid checks and server verification: no product crate changed and no method, endpoint or socket was added.
@task_item n=2 part=untouched
Strict-modes checks and their refusal contracts, and the CLI, MCP and hook refusal and exit-code contracts: no product verb or refusal moved.
@task_item n=3 part=untouched
Dependency security — exact pins, cargo-deny, advisory lists, licence list, version floors, toolchain pins, workflow actions and permissions: no dependency added or bumped and no workflow touched.
@task_item n=4 part=untouched
Credential stripping, the identity floor, the child environment and stored-secret rows: no environment variable read or stripped changed.
@task_item n=5 part=untouched
Pinned binary and plugin placement, hashing and rewriting, the event log, diagnostics and log rotation: no product file writer changed.
@task_item n=6 part=untouched
Verify, its probes, recorded fixtures and the fixture refusal codes: the live verify --record was re-pinned out of this session and verify did not change.
@task_item n=7 part=untouched
Paste and dialog text validation, send and answer refusals, and the MCP tool inputs: no product input path changed.
@task_item n=8 part=untouched
The absences (network, hosting, accounts, secrets, database, release stage): nothing was added that reaches them.
