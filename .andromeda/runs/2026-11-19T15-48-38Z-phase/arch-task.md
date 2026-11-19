@task_prose part=chunk
Mends the standing live rig, test-only, so its kept record dir is keyed by the CLI version and checked per version, not per label, so a later version's recorded set never meets an earlier one's; landed before the chunk's first recording start.
Records one contained live start under the rig's private Claude config whose turn ends in a CLI StopFailure (the connection-lost class if reachable), its payload kept through the rig's scrub, never under the operator's real Claude config or viola home.
Reads that payload's error word against the nine-word turn-end cause map: the turn end's cause is the word it maps to, and no parse-rejected unknown-value line is written for any of the CLI's 13 words.
Settles the provisional StopFailure exception on that reading, the founder's call: a capability-ledger row with its viola verify probe, or the exception kept on his word; the plan carries both shapes.
Disposes each of the entry's five carries by name; the placeholder-hint cell reading and the intermittent rehearsal red stay open, the red acted on only at its next occurrence.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs mark=edits
what: the standing live rig (the viola-live-rig test-only driver of real claude starts) — edits: where it keeps a run's record dir and its record-exists check, now keyed by the CLI version
@task_file n=2 path=crates/viola-e2e/src/rig/isolate.rs mark=edits
what: the live rig's isolation of a start, its private Claude config and scratch, and the keeping of the recorded product — edits: the kept record dir path
@task_file n=3 path=crates/viola-e2e/tests/rig_rehearsal.rs mark=edits
what: the rig rehearsal tests, run under the fake agent, that prove the rig's record keeping — edits: the kept record dir case, per version
@task_item n=1 part=needs
The StopFailure hook's mapping to a turn-ended kind: the hook registered with no ledger row and no verify probe, a provisional exception this chunk settles.
@task_item n=2 part=needs
The turn-end cause set and the turn-ended event's cause field: the live payload's error word must map to one of the nine causes.
@task_item n=3 part=needs
Unparseable external values become unknown and the parse-rejected unknown-value line: none may be written for a word among the CLI's 13.
@task_item n=4 part=needs
External payloads parsed tolerantly: the live StopFailure payload is read with tolerant serde, its unknown fields ignored.
@task_item n=5 part=needs
An API-error turn end clears the running turn: the live StopFailure reading checks the same turn end the static reading built.
@task_item n=6 part=needs
The per-behaviour CLI capability ledger as the single gate for CLI behaviour: a new ledger row for StopFailure, if the founder chooses it, with its viola verify probe.
@task_item n=7 part=needs
A CLI version is verified only when every relied row counts, and the ledger stamps envelope rows: a new relied row changes what a version's stamp must hold.
@task_item n=8 part=needs
No probe for a shape no send relies on, and real CLI probes run only locally: what decides whether the exception stays on the founder's word or gains a probe.
@task_item n=9 part=needs
viola verify's bounded child set, its own probe dirs and trust record, and that it never types into a CLI dialog: the rules a new StopFailure probe must keep.
@task_item n=10 part=needs
Fixtures recorded only from a clean verify, scrubbed and hygiene-walked, and the unclean fixture refusal words: the recorded payload passes the rig's scrub.
@task_item n=11 part=needs
The live rig binary and its AGENT_RUN_RIG knob: the rig is test-only, kept out of release builds, and its record dir now carries the CLI version under the label.
@task_item n=12 part=needs
The wrapped child's environment and the CLAUDE names stripped from it: a containment that sets the child's API base URL to a closed port must pass through the env strip and keep-list as they stand.
@task_item n=13 part=needs
No listening network port of any kind: a closed-port containment opens no listener of viola's own.
@task_item n=14 part=needs
No model API calls and wrapping the unmodified claude CLI: containment redirects or cuts the child's network, never calls or alters the model API or the CLI.
@task_item n=15 part=needs
Harness variables never read by viola, test seams are not a configuration channel, and the VIOLA_ namespace is product plumbing: any containment knob lives in the rig or harness, never in the product.
@task_item n=16 part=needs
No shelling out and the no-shell dependency: a namespace containment the rig wraps runs a closed argv, never a shell.
@task_item n=17 part=needs
Test homes cleaned by owner, test evidence survives a killed test, and removals outside the working directory: the rig's private config, the 0600 credential copy removed at teardown, and every process started ended.
@task_item n=18 part=needs
No live claude in CI and the real claude CLI only locally: the live start and the rig mend's live leg run on the dev host only, CI holds only the fake-agent rehearsal.
@task_item n=19 part=needs
The Claude CLI version output's first line: the version the rig keys its record dir by.
@task_item n=20 part=needs
Documentation states no figure the code holds (counts, member lists, versions) unless a master row holds it.
@task_standing n=1
convention: each new test runs in its own test-owned viola and user home, never the operator's real home
home: established-decisions · occupied-resources
@task_standing n=2
convention: each test wait takes a bound shared with the root test waits, under the 20 s lint
home: occupied-resources
@task_standing n=3
convention: each new test runs against the fake agent in CI; a live leg is local-only and never in a CI job
home: ci-cd-approach
@task_standing n=4
convention: each recorded fixture is scrubbed and hygiene-walked before it is kept, keeping only signature rows of a screen
home: occupied-resources
@task_standing n=5
convention: each OS-gated test names its reason, and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=6
convention: each end-to-end run holds zero panic lines and no print macros; a new log line goes through obs_event! as codes only
home: project-directory-structure · build-system
@task_item n=1 part=not_needed
The MCP tools and MCP tool results (send, wait, last, answer, list): the chunk adds no tool and changes no result.
@task_item n=2 part=not_needed
The wheel, byte-source claim, human take and release rules: the chunk types nothing into a driven session and moves no wheel.
@task_item n=3 part=not_needed
Send confirmation, paste, refusal order and send-refusal details: no send path changes.
@task_item n=4 part=not_needed
Revive and its --json error codes and refusals: no revive behaviour is touched.
@task_item n=5 part=not_needed
The Hyprland seam, window features and the terminal fleet console: no compositor or view work.
@task_item n=6 part=not_needed
The board and list payloads, agents listing and session liveness: the chunk changes no listing.
@task_item n=7 part=not_needed
Channel link and unlink methods, link recording and the channel fault codes: no channel method or link changes.
@task_item n=8 part=not_needed
Plugin install, the marketplace tree and the plugin's .mcp.json: no plugin change.
@task_item n=9 part=not_needed
Release tooling, dist and crates.io publishing: nothing is released.
@task_item n=10 part=not_needed
Fuzz workspace, weekly fuzzing and mutation testing: no mutation run, no fuzz target.
