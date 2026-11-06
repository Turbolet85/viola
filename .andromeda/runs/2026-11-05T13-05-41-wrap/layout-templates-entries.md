@amendment marker=2026-11-05-linux-only-code-and-ci
title: the Windows VT enable leaves the cli colour surface
section: Surface: cli (Tooling context); Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) (As built)
change:
  - The Tooling context no longer names a VT enable: was «Windows VT output is enabled with windows-sys 0.61.2 `SetConsoleMode`, and colour is off if that fails»; now clap 4.6.7, std `IsTerminal`, the hand-written SGR module and its fixed-width padding, nothing else.
  - `viola list`'s uncoloured conditions: was `--json`, a piped stdout, `NO_COLOR` (an empty value included), `TERM=dumb` or a failed Windows VT enable; now `--json`, a piped stdout, `NO_COLOR` (an empty value included) or `TERM=dumb`. The styled form, the agents read and the width cut are unchanged.
why: Viola is Linux-only by the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); the chunk removed `src/human/style.rs`'s VT enable, its `vt` fact and the `windows-sys` dependency, with rungs 1–6 and the depth rule unchanged and Linux human output byte-identical.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: verify's relied count and stop on one OS
section: Output structure — `viola verify` (the wireframe; Usage)
change:
  - The wireframe's annotation: was «(a Unix host: 24 relied rows)»; now «(24 relied rows)».
  - `MM`, `LedgerRow::relied()`'s length: was 24 on Unix and 21 on Windows, where the relied set skipped the three statusline rows; now all 24 rows on every build, ending at `local-command-compact`, growing as owning chunks land their rows.
  - The stop: was Ctrl-C or TERM as Unix `SIGINT` / `SIGTERM` or Windows Ctrl-C / Ctrl-Break; now Ctrl-C or TERM (`SIGINT` / `SIGTERM`), the same plain ASCII pair and no stamp.
why: the founder's own rulings of 2026-11-04 make Viola Linux-only (inputs#I1, inputs#I5 item 6); the chunk folded `WRAPPED_HERE`, so the Windows relied set of twenty-one is gone, and removed verify's console-control stop arm, `SIGINT` / `SIGTERM` unconditional; `[NN/24]` measured on Linux.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the per-OS wording of help and plugin install
section: Output structure — `viola --help` (As built; per-verb usage); Output structure — `viola plugin install` (Usage)
change:
  - The usage line: was the override literal «on every OS»; now the override literal.
  - `viola <verb> --help`: was a usage line naming the program by its executable, `viola.exe` on Windows; now by its executable.
  - plugin install's `<dir>`: was printed bare unless the host's shell would split it, Unix single-quoted and Windows double-quoted when it holds a space; now bare unless the shell would split it, single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :`.
why: the founder's own rulings of 2026-11-04 make the OS target Linux only, the Windows code removed and recoverable at the tag `multi-os-final` (inputs#I1, inputs#I5 item 6); an «on every OS» clause is stale wording on a Linux-only product, re-worded with its claim kept.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the .cmd/.bat refusal leaves revive and verify
section: Output structure — `viola revive` (`--json` start refusals); Component — Primary content block 2: refusal lines and the `unable` column (Line form)
change:
  - revive's `--json` start-refusal `<code>`: was `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused` or `internal-error`; now `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused` or `internal-error` (revive's closed refusal codes 11 -> 10).
  - verify's exit-1 pairs: was a pair `the claude CLI is a .cmd or .bat script` / `pass the real executable, not a .cmd or .bat shim` between the not-found and the version pairs; now the not-found, version, fixture, stop and home pairs, with no `.cmd`/`.bat` pair.
why: the chunk removed `AgentError::BatchScriptChild` and its start and verify arms on the Linux-only target (the founder's own rulings of 2026-11-04, inputs#I1, inputs#I5 item 6); a closed word no build can emit leaves with its emitter, by the Viola overseer's technical answer of 2026-11-05 (inputs#I2).
kept: the «on Unix» qualifier of the socket-directory refusal of the `run` and `revive` starts stands, true on Linux.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the server-check hint without its OS scope
section: Component — Primary content block 2: refusal lines and the `unable` column (Hint line)
change:
  - The server-check sentence: was scoped «On Linux, Windows and macOS», with the macOS carve-out of an euid mismatch alone (no pid and start-time comparison, owned by no route entry); now unscoped: a frame verb whose server check finds the process on the endpoint is not the recorded wrapper, or cannot make the check, writes no frame and exits 21 with its hint, or the `server-verify-failed` document under `--json`.
why: the founder's own rulings of 2026-11-04 make Viola Linux-only (inputs#I1, inputs#I5 item 6); the chunk retired the macOS no-peer-pid carve-out with `SERVER_PID_REPORTED`, `PEER_EUID_REPORTED` and the macOS `Matches` arm of `liveness::server_verdict`.
kept: the «On Unix» sentence of a frame verb meeting a refused socket directory stands, true on Linux.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
