@answer n=1 entity=threat-model-summary#1.1 v=no
why: the events.ndjson writer, its append-only retention and its MCP and CLI replay are not among the session's changes
@answer n=2 entity=threat-model-summary#1.2 v=no
why: tool-call argument, plan and dialog logging are unchanged, and the session touched no permission or dialog path
@answer n=3 entity=threat-model-summary#1.3 v=yes mode=stale
why: the pinned executable path form bin/<version>-<hash>/viola(.exe) is retired, the Windows viola.exe form being gone
@answer n=4 entity=threat-model-summary#1.4 v=yes mode=stale
why: the CASE_INSENSITIVE fold is removed, so the R8 strip of the CLAUDE* names now matches names exactly
@answer n=5 entity=threat-model-summary#1.5 v=no
why: the snapshot and budget.json fields listed here are unchanged by the session
@answer n=6 entity=threat-model-summary#2.1 v=yes mode=contradict
why: the Windows named-pipe endpoint and the macOS carve-out no longer exist, the endpoint being a Unix socket in the per-user 0700 directory only
@answer n=7 entity=threat-model-summary#3.1 v=no
why: the bridge send framing and the delivery-confirmation check are unchanged, the session removing only Windows ConPTY code
@answer n=8 entity=threat-model-summary#4.1 v=no
why: the stdin classifier's closed non-editing list is unchanged, and the session removed only the Windows console-control arm, which this entity does not name
@answer n=9 entity=threat-model-summary#5.1 v=no
why: hook fail-open, the strict-modes gate on hook paths and the statusline shell-out stand unchanged on Linux
@answer n=10 entity=threat-model-summary#6.1 v=no
why: the MCP strict-modes, socket-directory and peer euid checks are named without any OS, and the Linux path stands
@answer n=11 entity=threat-model-summary#7.1 v=no
why: the MCP stdin CappedLines reader at MAX_FRAME is untouched, the session removing only a SERVER_PID_REPORTED argument
@answer n=12 entity=threat-model-summary#8.1 v=yes mode=stale
why: the viola list board scan holds the home to the read rule on every OS, an on-every-OS clause reworded to the Linux statement
@answer n=13 entity=threat-model-summary#9.1 v=yes mode=contradict
why: the Windows owner and DACL arm of the strict-modes check and the Windows pin_companions conpty re-hash are removed
@answer n=14 entity=threat-model-summary#10.1 v=no
why: the one-shot pinned-copy placement through place_private_new is not among the removed retry items, so the claim holds on the changed fs.rs
@answer n=15 entity=threat-model-summary#11.1 v=yes mode=contradict
why: the npm-shim to claude.exe resolution and the Windows Passes none arm are retired with the .cmd/.bat refusal and its SHIM_TARGET
@answer n=16 entity=threat-model-summary#12.1 v=no
why: the cargo dependency posture, the SHA-pinned action list and the root-lockfile deny coverage are unchanged, and no crate was added or bumped
@answer n=17 entity=threat-model-summary#13.1 v=yes mode=contradict
why: the native matrix of windows-2025 and macos-latest is now ubuntu-latest alone, and windows-mutants.yml is deleted
