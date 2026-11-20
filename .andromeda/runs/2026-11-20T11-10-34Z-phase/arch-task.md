@task_prose part=chunk
A question dialog (the CLI's question tool) first raised through the PermissionRequest hook is answered on the driver's answer with an allow decision body whose updatedInput carries the driver's answers, where today it stays null to the human.
A new relied capability-ledger row in the Claude agent crate covers that body, with its own viola verify probe; the dialog decision stays gated on a verify stamp on which every relied row counts.
The CLI versions are re-stamped against the grown relied set (which versions and which home are settled at P3 on the founder's word).
The fixtures and the fake agent's PermissionRequest question script are extended to cover the new body.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs mark=edits
what: the Claude agent crate's dialog answer mapping, the hook decision body and the capability ledger's relied rows and their count — edits: the dialog answer mapping for a PermissionRequest question, the hook decision body, the ledger's relied row list and its count
@task_file n=2 path=src/cmd/verify mark=edits
what: viola verify and its probes, the one writer of the ledger stamps — edits: a new probe for the PermissionRequest question body, the verify stamp's row set
@task_file n=3 path=src/cmd/hook mark=edits
what: the viola hook command and the run wheel's dialog relay — edits: the PermissionRequest path for a question
@task_file n=4 path=fixtures/claude mark=edits
what: recorded per-version Claude CLI fixtures and the fake agent's PermissionRequest question script — edits: the PermissionRequest question fixture and script
@task_item n=1 part=needs
A question first raised by PermissionRequest is no longer left null: on the driver's answer carrying the answers, the hook replies with an allow decision whose updatedInput carries those answers. This replaces the rule that such a question stays null.
@task_item n=2 part=needs
The PermissionRequest hook contract and the sync-with-decision hook tier: the decision body's shape, the exit 0 with a JSON body, and the hook-to-kind map for PermissionRequest raising a question.
@task_item n=3 part=needs
The dialog relay over the channel: hook.dialog and its continuation, the pending dialog, the channel answer method and the answer response for a question, the unknown-dialog refusal.
@task_item n=4 part=needs
The question event and its data (options, option details), logged exactly once whether raised through PreToolUse or PermissionRequest; one pending dialog per instance.
@task_item n=5 part=needs
The capability ledger: a new relied row for the PermissionRequest question body, the relied-row list and its count, and the rule that a CLI version is verified only when every relied row counts.
@task_item n=6 part=needs
The dialog decision stays gated on the verify stamp: on an unverified CLI the hook.dialog path keeps its unverified-CLI behaviour and the partial or transport-only degrade.
@task_item n=7 part=needs
viola verify gains a probe for the new row: its step line and summary line, its exit 1 on a failed relied row, and its rules — verify never types into or answers a CLI dialog, keeps its own bounded child set and removes only its own probe dirs, and no probe exists for a shape no send relies on.
@task_item n=8 part=needs
The ledger stamps envelope and its writer: rows and verified version, verify as the sole stamps writer, a merge replacing only its own version, the strict-modes checks before and under the lock, and malformed or untrusted stamps reading unverified.
@task_item n=9 part=needs
The re-stamp of the CLI versions runs the real claude CLI only locally, never in CI, and no live start or read of the operator's real home happens in this phase.
@task_item n=10 part=needs
The human always wins: the human at the keyboard can answer the dialog himself at any moment, a human-held wheel renders dialogs at once, and a refusal reaches automation only.
@task_item n=11 part=needs
The dialog hold and its bound: viola, never Claude Code, ends the dialog wait; an expired hold keeps today's conservative fall-back with its null reply and dialog-expired event.
@task_item n=12 part=needs
Mechanism, not policy: viola carries the driver's answers and never chooses them; option and answer text is untrusted upstream content, never a command, and stays out of human lines.
@task_item n=13 part=needs
Hooks fail open toward the human: a failed server check, a lost endpoint or an untrusted instance leaves the dialog to the human, and the hook never writes stderr.
@task_item n=14 part=needs
The PreToolUse answer paths stay as they are: ExitPlanMode answered only through PreToolUse and the PreToolUse question path; the new allow-with-updatedInput body is the PermissionRequest analogue, and permission answers still carry no suggestions.
@task_item n=15 part=needs
Claude payload shapes, the decision body included, live only in the Claude agent crate, which does no file I/O and parses external payloads tolerantly; hook output is produced by one crate.
@task_item n=16 part=needs
The MCP answer tool and the CLI answer verb change only if the answer's shape must change; any change keeps their refusal and result contracts.
@task_item n=17 part=needs
Fixtures for the new body are recorded only from a clean verify, scrubbed and hygiene-walked, and the fake agent drives the PermissionRequest question through its test seams.
@task_standing n=1
convention: every new test runs against the fake agent, never a live claude, in its own isolated viola home
home: ci-cd-approach · established-decisions
@task_standing n=2
convention: every recorded fixture is scrubbed and hygiene-walked, keeping only signature rows
home: occupied-resources
@task_standing n=3
convention: new code meets the coverage floors, the zero-panic end-to-end gate and insta snapshots in check mode
home: ci-cd-approach · project-directory-structure
@task_standing n=4
convention: new code passes formatting and clippy with warnings denied, with no print macros and no tracing level macros
home: build-system
@task_standing n=5
convention: new log lines go through codes-only process logs, content detail only in the instance detail file
home: cross-cutting-patterns
@task_item n=1 part=not_needed
Revive, its refusals, exit codes and --json documents: the chunk starts and revives no instance.
@task_item n=2 part=not_needed
The send path: atomic bracketed paste, typed text normalisation, send confirmation, send refusals and the send flight lock.
@task_item n=3 part=not_needed
The board and list: list payload, list items, agents listing and their exit codes.
@task_item n=4 part=not_needed
Links: link and unlink verbs, link recording, link events and their exit codes.
@task_item n=5 part=not_needed
The Hyprland compositor seam and the window features.
@task_item n=6 part=not_needed
The MCP stdin line cap, the oversize refusal and the viola mcp exit codes.
@task_item n=7 part=not_needed
The trust writer code words and the trust-record ledger row.
@task_item n=8 part=not_needed
Plugin install, the marketplace tree and the plugin's .mcp.json.
@task_item n=9 part=not_needed
Release and deployment: the dist release workflow and the single binary on PATH.
@task_item n=10 part=not_needed
Local commands (/clear, /compact) and their confirmation windows and echo probes.
