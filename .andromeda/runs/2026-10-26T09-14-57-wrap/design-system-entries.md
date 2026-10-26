## 2026-10-26-server-verification-before-any-frame — `server-verify-failed` written by the six frame verbs
**Section:** Surface: cli › Component Patterns (surface.md: the cli-send hints group, its pending-arch-amendment bullet on the D-20 log detail codes)
**Change:**
- After the D-20 code list the bullet now says that, of those codes, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause` and `release` at their exit-21 server-verification refusal, and that the refusal's `--json` document stays `detail:null`, as the not-running cause's does (was: the code named in the list with no writer). The code list itself, the arch-shape rule and the stale-heartbeat and unwrapped-name causes having no code are unchanged.
**Why:** this build is the first that writes `server-verify-failed`, as `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` into the CLI log; the code was already a member of the diag-line `detail` enum, and no `--json` code, exit code or refusal reason was added.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-verification hint as built, and its macOS gap
**Section:** Surface: cli › Component Patterns (surface.md: two new cli-send groups)
**Change:**
- New: the server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause` and `release`, the six verbs that write a frame, as the hint cause `server-verify` beside `not-running`; it prints the hints group's server-verification line byte for byte, plain ASCII with no SGR, and holds no pid, start time, path or endpoint.
- New: on macOS no peer pid is available, so the server check passes with no comparison and the hint cannot print there, a recorded gap owned by the route entry "Unix endpoint and home hardening".
**Why:** the hint function's new `server-verify` cause prints exactly the hint row's string, and the tests assert it plain ASCII with no SGR and no pid, path or endpoint in any output; the server verdict returns a match with no comparison when no peer pid is reported, which is the case on macOS, and macOS stays on the recorded gap, owned by "Unix endpoint and home hardening".
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — exit 21's server-verification cause as built
**Section:** Surface: cli › Component Patterns (surface.md: a new cli-exit-codes group)
**Change:**
- New: exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause` and `release`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.
**Why:** a mismatch or a check that cannot be made is exit 21 with the unchanged document, measured on Linux and Windows by `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (six verbs, two modes) beside its control `security_negative_cli_verb_reaches_the_recorded_wrapper`; no exit code or `--json` code was added.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/
