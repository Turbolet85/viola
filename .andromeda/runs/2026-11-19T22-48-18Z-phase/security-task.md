@task_prose part=chunk
Under viola run, a prompt that a CLI-scheduled wake submits is told apart from a prompt the human typed, so a fired wake does not take the wheel, does not make the driver's next send refuse human-typing, and does not route the next question dialog to the human (capability v2-36); a prompt the human types still takes the wheel.
The means is this phase's to find: the CLI sends no source field on a fired wake's UserPromptSubmit (measured on 2.1.325 and 2.1.326), and the working hypothesis is that the wrapper, which owns the pty, can know no byte came from the human's side before a prompt was submitted while a wake was scheduled.
Where it cannot tell, the conservative side is human; an older or unverified CLI's handling, and whether a capability ledger row with a verify probe is part of the means, are left to P3.
It re-reads without a live start: readings come from the code, the installed 2.1.328 bundle and earlier evidence; the carried placeholder-hint cell attribute (a third reading, attributes only) and the intermittent rig rehearsal red (acted on only at its next red) are disposed by name, and the boot-timeout watch is read at each run, no bound moved.
@task_item n=1 part=needs
The wheel's origin decision for a submitted prompt in the wrapper: human typing takes the wheel, and the rule now has a told-apart case for a scheduled wake's prompt that leaves the wheel with its holder; when the origin cannot be told, the prompt is filed as human, never as a wake.
@task_item n=2 part=needs
The wheel event with cause human-input: it must still fire for a prompt the human typed and must not fire for a prompt told as a fired wake.
@task_item n=3 part=needs
The send refusal human-typing on the wrapper's send method and the MCP and CLI send surfaces: after a fired wake the driver's next send is confirmed, not refused; the send stays one bracketed paste with its delivery confirmation unchanged.
@task_item n=4 part=needs
The question dialog hold over the hook.dialog channel method: after a prompt told as a wake the next dialog is held for the driver; the hold time stays set only in config and dialog answers still need a verify stamp.
@task_item n=5 part=needs
The UserPromptSubmit hook payload parse in the Claude adapter: tolerant parsing, any security-bearing field (such as an optional source value schedule_wakeup or loop_wakeup) read into a closed enum with unknown values failing to unknown, the payload and its prompt text treated as content, never a command.
@task_item n=6 part=needs
A self-reported field authenticates nothing: anything the CLI's payload says about a prompt's origin is a hint the wrapper checks, never an identity, and the wrapper re-validates every hook.event frame it receives.
@task_item n=7 part=needs
The pty pump that reads the human's keys (the wrapper's byte source): any reading of whether a human byte preceded a submit counts keys by the stdin classifier's closed non-editing list, holding measured replies only; the screen model feed stays isolated from passthrough, bounded, and under a panic catch.
@task_item n=8 part=needs
The normalised event for a submitted prompt in the core crate and the turn end's scheduled-wakes count: the count is carried as a count only, no cron or wake content, and any new origin value is a closed protocol type.
@task_item n=9 part=needs
Fail-open to the human: a hook or origin-decision failure must leave the human in control; security refusals never block the human, and the hook stays silent with exit 0.
@task_item n=10 part=needs
If a capability ledger row with a verify probe is part of the means: the unverified-CLI gate applies to an older or unverified CLI, verify reads signatures only against compiled literals, never answers or types into a dialog, its probe prompts are synthetic, probe effects stay in their own directory and captures are transient.
@task_item n=11 part=needs
No read of the operator's real Claude config and nothing written under the Claude home: readings come from the installed bundle and stored evidence only; the build calls no scheduled-wake or timer tool.
@task_item n=12 part=needs
The live rig and fake agent where a wake would be rehearsed: the rig's child isolated from the user's config, its SF arm network-closed in its sandbox, keys typed only into its own pty, its kept stamp owner-only and seeded to its own home, its credentials copy transient and unrecorded.
@task_item n=13 part=needs
The carried third reading of the placeholder hint's cell attribute: it reads drawn hint cells past the cursor by attributes only, never text; live session screens stay local and screen fixtures keep signature rows only; its start and keys are the founder's word.
@task_item n=14 part=needs
The sync crates touched (core and the Claude adapter) stay free of tokio.
@task_standing n=1
convention: each new test uses its own viola home and never touches the operator's real Claude home or the e2e-home backing; test-side removal only of dirs the test created
home: secret-management · security-anti-patterns-never-do-these
@task_standing n=2
convention: each new test seam sits only under the fake-agent feature and is compiled out of the release build
home: input-validation · security-anti-patterns
@task_standing n=3
convention: each new recorded fixture is scrubbed or refused whole, reviewed before commit, and kept under the hygiene walk
home: data-protection
@task_standing n=4
convention: each new log line keeps user content and prompt text only in owner-only diagnostics, never logs CLAUDE values, and its error messages interpolate no upstream text or paths
home: bootstrap-phases-derive-for-route-setup-project · error-handling
@task_standing n=5
convention: a new seam or environment read needs its log entry
home: security-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The channel socket's bind, directory and peer-euid checks and the client's server verification and their refusal contracts: the chunk changes no endpoint, connect path or peer check.
@task_item n=2 part=not_needed
viola revive, its arguments and --json codes, and the replayed snapshot: the chunk changes no revive path.
@task_item n=3 part=not_needed
viola list, the board scan and the terminal colour facts: the chunk does not touch the board.
@task_item n=4 part=not_needed
viola link and unlink, their channel methods and the events link lines: untouched by the wheel's origin decision.
@task_item n=5 part=not_needed
The pinned binary and pinned copy hashing, placement and re-hash, plugin install and the marketplace tree: the chunk changes no install or pinning.
@task_item n=6 part=not_needed
The paste-text and dialog free-text character sets, control-character and empty-text refusals: the chunk changes whether a send is refused for the wheel, not how its text is validated.
@task_item n=7 part=not_needed
The API-error turn end classes and the Hyprland recorder and hyprctl reads: not reached by a submitted prompt's origin.
@task_item n=8 part=not_needed
Release, signing, deploy and release-artifact attestations, and the absences of hosting, network listeners, outbound calls and regulated data: the chunk adds no surface of those kinds.
@task_item n=9 part=not_needed
The CLI state root selection, CLI exit codes and instance-unreachable details: the chunk adds no CLI verb or exit path.
