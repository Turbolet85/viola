@task_prose part=chunk
A viola run (and the start of viola revive where it shares the version gate) that meets a claude CLI version with no verify stamp gets that version stamped without a human running viola verify by hand, then goes on verified, its sends and answers no longer refused unverified-cli.
The form of the automatic verify (before the session starts, beside it, a separate verb or flag, a config key turning it on or off, or another) is the founder's at P3; P4 plans only the form he answers.
A red automatic verify never loops on every start of a version that failed: it says which rows failed, leaves the session to the human as today, records the failed attempt so it is not repeated, and names what clears it.
The spend of the user's model turns and the stamp written into the real viola home with no human in the loop come with what the product asks or tells before the first one, and what a user who never wants it does.
It re-reads at each run the CI-only red of the cli_wheel pause-then-release test, its job log and its own events, before any mend.
@task_file n=1 path=src/run/version_gate.rs mark=edits
what: the version gate of viola run's start, reading the claude version and its stamp — edits: the gate's unverified-version path
@task_file n=2 path=src/run/mod.rs mark=edits
what: viola run's start path and its start order
@task_file n=3 path=src/cmd/revive.rs mark=edits
what: viola revive's start where it shares run's version gate
@task_file n=4 path=src/cmd/verify.rs mark=edits
what: viola verify, the process the automatic form starts, the one stamps writer
@task_file n=5 path=crates/viola-state/src/stamps.rs mark=edits
what: the ledger stamps envelope and its writer under the stamps lock — edits: update_stamps
@task_file n=6 path=tests/cli_wheel.rs mark=re-reads
what: the CI-only red watch of path5_a_turn_left_running_is_cleared_by_pause_then_release, read at each run with no bound, retry or assertion moved
@task_item n=1 part=needs
The version gate at run's start: what it does on a claude version with no stamp changes from a partial readiness gate and a transport-only degrade to an automatic verify, after which the session reads verified (cli_verified and cli_version in the instance snapshot).
@task_item n=2 part=needs
The unverified-cli refusal reason on sends, answers and the hook.dialog path: no longer returned for a version the automatic verify stamped green; still returned for a version whose automatic verify failed.
@task_item n=3 part=needs
Revive's start resumes through the same start path and resolves its program from its own PATH; where it shares the gate it gets the same automatic stamp.
@task_item n=4 part=needs
Stamps are written only by verify, through one writer: the automatic form must reach the stamp through viola verify, never by run writing a stamp itself.
@task_item n=5 part=needs
The stamps writer merges only its own version, checks strict modes before and under its lock, and the gate reads stamps strictly; a malformed or untrusted stamp reads unverified.
@task_item n=6 part=needs
A version is verified only when every relied row counts (27 relied ledger rows at the base); a red verify names the rows that failed through verify's step and summary lines and exit 1.
@task_item n=7 part=needs
The new record of a failed automatic verify attempt, so a failed version is not retried on every start, and what clears it: an own on-disk format, versioned and tolerant, written through the one atomic replace helper with its mode set before any byte, under the on-disk file naming.
@task_item n=8 part=needs
What viola run may spawn and in what order: the child spawned last, no shelling out, the claude program resolved by viola itself, verify's bounded child set; the automatic verify must check the same claude program and version the session then starts.
@task_item n=9 part=needs
Verify's existing refusals and process-exit details (program-not-found, version-unreadable, strict-modes-failed, pinned-hash-mismatch, unclean-fixture, interrupted) as the automatic form meets them at run's start, and how run surfaces them.
@task_item n=10 part=needs
Exit 12 and the CLI exit-code table: what run's start exits with when the automatic verify fails or is declined, and that retired codes are never reused.
@task_item n=11 part=needs
Run writes only the child's output to the terminal once the child holds it, and no JSON document after that: any word the automatic verify says to the human (consent, failed rows, what clears it) falls before the child holds the terminal or goes elsewhere.
@task_item n=12 part=needs
Consent and opt-out if the founder's form is a key: config.json's closed key set equal to the config schema, unknown keys, bad values falling back to defaults, and environment variables never a configuration channel.
@task_item n=13 part=needs
If the form is a separate verb or flag: new capabilities as subcommands, the CLI verb and argument table for verify and run, flag and subcommand style, human text by default and --json for agents.
@task_item n=14 part=needs
Verify's own runs, probes, typed bytes and dialog answers stay unchanged unless the founder's form needs it (it never types into a CLI dialog, writes and removes its own trust record, removes only its own probe dirs); any widening is said whole at P3.
@task_item n=15 part=needs
Verify's accepted residuals: its CLI transcripts and the model turns it spends, now spent with no human in the loop on the user's subscription.
@task_item n=16 part=needs
Real claude probes run only locally and never in CI: the automatic form is proved over the fake agent; any live start runs in a scratch viola home under the live rig, never the live home.
@task_item n=17 part=needs
Viola home resolution and the strict-modes check before any log or stamps read: the automatic verify writes into the same viola home run resolved, and refuses an untrusted home tree or stamps as verify does.
@task_item n=18 part=needs
Codes-only process logs per role and diagnostics_level as the sole log level: any line the automatic verify writes to run's diagnostics holds codes, never upstream text.
@task_item n=19 part=needs
The capability ledger stays the single gate for CLI behaviour, version-stamped; the automatic form changes only who starts verify, not what a stamp means.
@task_item n=20 part=needs
Human wheel and link controls stay CLI-only and driver release stays deterred: the automatic verify gives a driver no new way to stamp, start verify or lift unverified-cli through the channel or MCP.
@task_standing n=1
convention: each new test runs in its own isolated viola home, cleaned by its owner
home: established-decisions
@task_standing n=2
convention: each new test drives the fake agent through its named test seam variables, never a live claude, and no test seam is a configuration channel
home: conventions
@task_standing n=3
convention: no live claude in CI; test homes and test sockets kept outside the product namespace and paths
home: occupied-resources
@task_standing n=4
convention: each new process log line codes-only, sanitised, with upstream text kept out of it
home: cross-cutting-patterns
@task_standing n=5
convention: new code held to the coverage floors and the gate steps failing closed
home: ci-cd-approach
@task_standing n=6
convention: zero panic lines in end-to-end runs; insta snapshots in check mode only
home: project-directory-structure
@task_item n=1 part=not_needed
The MCP server, its tools, results and error documents: the automatic verify starts at run's start, not through MCP.
@task_item n=2 part=not_needed
Hook tiers, the hook-to-kind map and hook exit bodies: no hook is added or changed by a stamp written at start.
@task_item n=3 part=not_needed
Wheel claims, human interrupt and the byte-source wheel rows: who holds the wheel does not change.
@task_item n=4 part=not_needed
Bracketed-paste send, typed-text normalisation, send time bounds and send confirmation: how a send is typed and confirmed does not change, only whether the version gate refuses it.
@task_item n=5 part=not_needed
The board and list payload, agents listing and session liveness rows: list reads state and is not gated on a stamp.
@task_item n=6 part=not_needed
Hyprland seam errors, window features and the terminal fleet console: no compositor or view is touched.
@task_item n=7 part=not_needed
Release via dist and the release build's contents: no release change.
@task_item n=8 part=not_needed
Wrapper channel framing, protocol version and channel fault codes: no channel method or frame changes.
@task_item n=9 part=not_needed
Link recording, link and unlink events and their exit codes: links are untouched.
@task_item n=10 part=not_needed
The Claude Code plugin's MCP json, plugin install and its marketplace tree: the plugin is unchanged.
