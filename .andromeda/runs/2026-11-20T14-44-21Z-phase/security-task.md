@task_prose part=chunk
A viola run (and the start of viola revive where it shares the version gate) that meets a claude CLI version with no verify stamp gets that version stamped by an automatic viola verify, with no human running verify by hand, then goes on verified.
The form of the automatic verify (before the session, beside it, a separate verb or flag, a config key turning it on or off) is the founder's at P3; P4 plans only the form he answers.
A red automatic verify does not loop: it says which rows failed, leaves the session to the human unverified as today, records the failed attempt so it is not repeated, and names what clears it.
The spend and the consent: an automatic verify spends the user's model turns and writes a stamp in the user's real viola home; what the user is told or asked first, and how a user who never wants it opts out, is the founder's at P3.
Proved over the fake agent; any live start runs in a scratch viola home under the rig. The CI watches on the cli_wheel pause-release test and the rig rehearsal's box-empty test are read again only at their next red.
@task_file n=1 path=src/run/version_gate.rs mark=edits
what: viola run's version gate, the unverified-cli gate that reads the stamps through the strict-modes check — edits: the gate's handling of a version with no stamp
@task_file n=2 path=src/cmd/run.rs mark=edits
what: viola run's start, the CLI verb that resolves and spawns its child program — edits: the start where the automatic verify is set off
@task_file n=3 path=src/cmd/revive.rs
what: viola revive's start, where it shares viola run's version gate
@task_file n=4 path=src/cmd/verify.rs
what: viola verify, the process the automatic form starts and the only writer of the stamps file; its runs, probes, typed bytes and dialog answers unchanged unless the founder's form needs it
@task_file n=5 path=crates/viola-state/src/stamps.rs
what: the stamps file in viola-state, its stamp writer update_stamps and the stamps lock, stamps checked before and under lock
@task_item n=1 part=needs
The unverified-cli gate and viola run's version gate: a version with no stamp now sets off an automatic verify, and the session turns verified only when a stamp for that version lands; until then the driver's sends and dialog answers stay refused unverified-cli, and the gate's stamps read stays behind the strict-modes check.
@task_item n=2 part=needs
Only viola verify writes the stamps file: the automatic form must produce its stamp through verify's own stamp writer under the stamps lock, never by viola run or revive writing a stamp themselves, and the stamps are touched only after the strict-modes check.
@task_item n=3 part=needs
What viola run may spawn: an automatic verify is a new child of run (or of a launcher verb), so it must be resolved to an absolute program, spawned directly without a shell, with inherited CLAUDE credentials and parent identity variables stripped as for every claude child, and never launched as root.
@task_item n=4 part=needs
Verify's own floors carried into the automatic run: verify never answers a dialog and never types into one, reads the screen only against compiled literals and signatures, needs a pre-trusted folder, keeps its probe effects in its own directories removed by drop guards; any widening of these is said whole at P3.
@task_item n=5 part=needs
Exit 12 and unverified-cli stay the outcome when the automatic verify is red or not yet landed: the human keeps the session as today, and the refusal never blocks the human's own typing.
@task_item n=6 part=needs
The red-verify record: a new piece of state in the viola home that a failed attempt for a version leaves, so no start repeats it, with a named way to clear it; it is written owner-only, through the one shared replace helper, its reader tolerant and strict-modes checked before use.
@task_item n=7 part=needs
Saying which rows failed: the message a red automatic verify gives names failed rows in closed code words, interpolates no upstream text, path, pid or identity value, and full detail goes only to diagnostics.
@task_item n=8 part=needs
The config file's closed key set, if a consent or on-off key is the founder's form: a new key is a config.json contract entry with its bad-value handling and tolerant parse, and no config key, env var or flag may switch off the unverified-cli gate or any other control, only the automatic verify itself.
@task_item n=9 part=needs
The driver-permission rows: dialog answers and sends from the driver need a verify stamp; an automatically written stamp grants exactly what a hand-run verify's stamp grants, no more.
@task_item n=10 part=needs
The residual rows the automatic verify now spends with no human in the loop: verify's trusted runs, its /context and echo probe residuals and its real-CLI transcripts, accepted risks that a run now takes on its own once per new version.
@task_item n=11 part=needs
Nothing of the operator's real Claude home is read and viola writes nothing under the Claude home or Claude's config; the automatic verify writes only in the viola home it runs for.
@task_item n=12 part=needs
Live starts confined: proved over the fake agent; any live run is a scratch viola home under the rig, its kept stamp owner-only and seeded to its own home, its credentials copy transient; the real claude CLI only locally, never in CI.
@task_item n=13 part=needs
The automatic verify's process and log lines: a new seam, env read or child start it adds needs its log entry, the process-start claude-child line keeps names only, and the never-log floor holds on stderr and in diagnostics.
@task_item n=14 part=needs
viola verify's own refusals stay its own when started automatically: its home-tree strict-modes check first, its refusal codes (strict-modes-failed, program-not-found, version-unreadable, pinned-hash-mismatch, unclean-fixture) are what run reports when the automatic verify cannot start.
@task_item n=15 part=needs
The two CI watches re-read at their next red only: the cli_wheel pause-then-release test and the rig rehearsal's box-empty-after-x-and-del test; no bound, retry rule, wait or assertion moves to get past them.
@task_standing n=1
convention: each new test runs in its own viola home, never the operator's, and removes only the dirs it created
home: secret-management
@task_standing n=2
convention: each new test seam (a fake verify outcome, a forced red) lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_standing n=3
convention: each new fixture recorded from a verify run is scrubbed or refused whole and reviewed before commit
home: data-protection
@task_standing n=4
convention: each new seam, environment read or child start needs its log entry
home: security-anti-patterns-never-do-these
@task_standing n=5
convention: CI holds no secrets and runs no real claude CLI; new CI coverage runs over the fake agent
home: threat-model-summary
@task_item n=1 part=not_needed
The MCP server's rows (stdin line cap, tool inputs, isError documents, tool list): the chunk adds no MCP tool and changes none.
@task_item n=2 part=not_needed
The channel socket's rows (socket directory check, peer euid checks, bind and chmod, server verification before a frame): the chunk changes no listener, no channel method and no client seam.
@task_item n=3 part=not_needed
The hook's rows (stdin cap, payload parsing, silent exit 0, hook.dialog reply bound): the chunk does not touch the hook.
@task_item n=4 part=not_needed
The send and answer text rows (paste character set, control-character refusal, bracketed paste, line-ending normalisation): what the driver types is unchanged; only whether the gate lets it through moves.
@task_item n=5 part=not_needed
viola link and unlink, viola list's board scan and terminal colour facts, and the Hyprland recorder: none is reached by the version gate or verify.
@task_item n=6 part=not_needed
The revive --json code list and revive's recorded cwd and argv rows: revive's start is reached only where it shares the gate; its other codes and spawn rules are unchanged.
@task_item n=7 part=not_needed
The CI workflow rows (actions pinned by SHA, least permissions, zizmor, fuzz workflows) and the release, signing and attestation rows: the chunk changes no workflow and cuts no release.
@task_item n=8 part=not_needed
The pinned binary and plugin folder rows (placed once, re-hashed, no-clobber, plugin files rewritten at start): the chunk does not move the pin or the plugin install.
