@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=31 class=omission
leaf-text: the socket-directory check and the peer-euid check on the hook `connect` sit in that timed path too
source: fct-4od5l5b2 «the peer euid decision, then the server check (the connected endpoint's peer pid and its OS start time against the one `snapshot.json` read)»
replace-with: the socket-directory check, the peer-euid decision and the server check (the connected endpoint's peer pid and its OS start time against the one `snapshot.json` read) on the hook `connect` sit in that timed path too
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=32 class=stale
leaf-text: 5.0 s live / 5.1 s stale
source: fct-gqc4mh6b «The flip happens at >5 s (4.9 s live / 5.1 s stale or gone).»
replace-with: 4.9 s live / 5.1 s stale or gone
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=38 class=omission
leaf-text: `os` + `pid` on `process-start`
source: fct-rrjwwqcp «`os` (`linux`, from `std::env::consts::OS`; the schema's one word), `pid`»
replace-with: `os` (`linux`, from `std::env::consts::OS`, the schema's one word) + `pid` on `process-start`
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=51 class=retired
leaf-text: on a Windows host it sits in the harness's host mutation scratch outside the repository, named by no document (only `scratch_bytes`); on the Linux dev host the same-named dir is only the operator's NOCOW `TMPDIR`, printed by no document either
source: fct-ydc7ueix «On the Linux dev host a directory named `viola-mutants-scratch/` is only the operator's NOCOW `TMPDIR` for cargo-mutants' copy trees»
replace-with: it stays at the repository root (gitignored); on the Linux dev host a directory named `viola-mutants-scratch/` is only the operator's NOCOW `TMPDIR`, printed by no document either
@leaf_edit n=5 leaf=.claude/docs/obs-summary.md line=67 class=retired
leaf-text: `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child|internal-error|socket-dir-refused`
source: fct-dsupowm6 «`squatted-name`, `pinned-hash-mismatch`, `internal-error`, `socket-dir-refused`»
replace-with: `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|internal-error|socket-dir-refused`
@leaf_edit n=6 leaf=.claude/docs/obs-summary.md line=67 class=retired
leaf-text: `batch-script-child|program-not-found|pinned-hash-mismatch|version-unreadable|strict-modes-failed|unclean-fixture`
source: fct-dsupowm6 «under a parsed `VIOLA_NAME`: `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`»
replace-with: `program-not-found|pinned-hash-mismatch|version-unreadable|strict-modes-failed|unclean-fixture`
@leaf_edit n=7 leaf=.claude/docs/obs-summary.md line=67 class=retired
leaf-text: macOS compares no peer pid, the named carve-out from the distinct-code rule, owned by no route entry, and there `server-verify-failed` means an euid mismatch alone
source: fct-2tnvi4o3 «The peer pid and start-time comparison runs on Linux, the one target.»
replace-with: the peer pid and start-time comparison runs on Linux, the one target, and `server-verify-failed` means a server of another euid, a server that is not the recorded wrapper or a check that cannot be made
@leaf_edit n=8 leaf=.claude/docs/obs-summary.md line=69 class=retired
leaf-text: only when a command runs (never on Windows)
source: fct-gqw4imf3 «`process-start` / `process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}`, only when the snapshot records a `statusline_command`»
replace-with: only when the snapshot records a `statusline_command`
@leaf_edit n=9 leaf=.claude/docs/obs-summary.md line=69 class=retired
leaf-text: (`viola verify`'s Run B probe capture, Unix only)
source: fct-f3gi2vhc «The second hidden capture arm, `hook statusline --capture <DIR> [--shell <WORD>]` (sole caller `viola verify`'s Run B probe settings)»
replace-with: (`viola verify`'s Run B probe capture)
