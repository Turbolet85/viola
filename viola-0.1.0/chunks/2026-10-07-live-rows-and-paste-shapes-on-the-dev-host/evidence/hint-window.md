# The paste-hint window on `claude` 2.1.287: timed again, with a `send` and a `wait` run inside it

The readings are the lines of `hint-window.ndjson` (two hint runs) and of `scratch-session.ndjson` (the scratch
session's four timings). The readiness gate, `GATE_MAX_WAIT`, `QUIET_PERIOD` and `send_hint` are unchanged.

## In short
- **The hint is a timer of 8.0 s from the last long paste.** Eight new timings read 8.005 s to 8.023 s from that
  paste; the first chunk read 8.000 s. How much of it falls after the turn's Stop depends only on how long the turn
  ran: 3.8 s to 7.0 s here, 6.5 s in the first chunk.
- **On a verified home a `send` issued in the window is refused** `not-delivered` / `input-not-ready`, 0.63 s after
  it was issued, with nothing typed. The fake-agent reading is confirmed on the real CLI.
- **The hint text's advice does not lead out of the window.** A `viola wait` with no cursor, issued in the window,
  timed out after its full 20 s while the input box had been back for 16.5 s of them. A `wait --after` the earlier
  cursor returns the already-logged turn end at once, and the window is still open.
- **On an unstamped home the same `send` is typed under the hint, and the real CLI takes it:** it was submitted as
  its own prompt and confirmed, twice, while the hint stood.
- **A `send` of a text ending in a newline is never claimed** (both homes): `no-prompt-submitted` after the 10 s
  window, the prompt filed `human`, the wheel moved to the human.

## How the two hint runs ran
- A second scratch driver outside the repository hosts `viola --home <home> run <name> -- <the 2.1.287 binary>
  --model haiku` in a PTY at 80×24, drains it with timestamps, and types nothing into it but the final Ctrl-C twice.
  Its cwd is a fresh 0700 `<repo root>/.viola-verify-<pid>/`, removed on its own exit paths (`dir_removed` true,
  both runs).
- The product is `target/debug/viola`, built once on the base commit `e7e5bf75db8a` with the default features,
  before any source edit. Every viola process the driver started ran with `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`
  removed and with `--home` given (research M1).
- The verbs are separate `viola` processes, each with `--json`, text from stdin. One human-mode `send` was added on
  the verified home, right after the refused one, so the refusal's lines are recorded as printed.
- The long text is the tree's 1 500-byte `PROBE_LONG_PASTE`; the short text 92 bytes; the newline text 1 500 bytes
  whose last byte is LF. All synthetic ASCII.
- The footer is read from the drained bytes through the tree's own `Screen`, by the scratch helper. The paste is
  the wrapper's `send-issued` record and the Stop its `turn-ended` record (ms); the footer instants are the driver's
  arrival times.
- **The rehearsal (no live start):** the whole sequence was first run against the fake agent at 10:11:35Z, booted
  as `send_under_the_paste_hint_on_a_verified_cli` boots it, on a home `viola verify` stamped against the fake agent
  (`17 pass  0 fail`), with `--paste-hint-ms 8000`. Its readings were the recorded ones
  (`2026-10-06-local-command-send-outcomes/evidence/paste-hint-send.md`): the in-window `send` exit 13
  `not-delivered` / `input-not-ready` in 0.305 s, nothing typed; the no-cursor `wait` `timed_out` after 20.012 s;
  the `wait --after` back at once; the later `send` confirmed.
- No modal literal was on any row at any chunk of either run, the screen model never poisoned, and neither run
  wrote a `parse-rejected` line.

## Every timing of the hint (seconds)
| # | session | the long paste | paste → Stop | paste → the literal's return | Stop → the literal's return |
|---|---|---|---|---|---|
| — | the first chunk's rehearsal (2026-10-06) | Run B's long text | 1.5 | 8.000 | 6.5 |
| 1 | scratch (start 1) | typed-then-paste | 2.512 | 8.023 | 5.511 |
| 2 | scratch | paste-then-typed | 2.259 | 8.005 | 5.746 |
| 3 | scratch | two-pastes | 1.687 | 8.306 from the first, 8.006 from the second | 6.619 |
| 4 | scratch | long-ending-newline | 1.627 | 8.005 | 6.378 |
| 5 | verified home (start 2) | step 1 | 3.830 | 8.006 | 4.176 |
| 6 | verified home | step 7, the newline text | 1.037 | 8.005 | 6.968 |
| 7 | unstamped home (start 3) | step 1 | 4.253 | 8.017 | 3.764 |
| 8 | unstamped home | step 7, the newline text | 1.539 | 8.005 | 6.466 |

- In every timing the footer lost the input-box literal 5 ms to 15 ms after the paste and showed `paste again to
  expand` in its place.
- Timing 7 had two short sends typed inside the window; the hint still ended 8.017 s after the long paste. A short
  paste does not restart the timer; a second long paste does (timing 3).
- The longest window after a Stop is 6.968 s; the shortest 3.764 s. Against the gate's constants: a quiet screen
  with no literal is refused at the first quiet instant, so `GATE_MAX_WAIT` (5 s) plays no part in the refusal. Had
  the gate waited for the literal, 5 s from the Stop would have covered timings 5 and 7 and not the other six.

## Live session 2 — a verified home (start 2)
- The home is a byte copy (`cp -a`) of the home `viola verify` wrote at the 2026-10-06 record round
  (`viola-record-20261006T211429Z/vhome`, on the operator's desk), into a new
  `target/e2e-home/viola-hint-20261007T101352Z-v/vhome`. `diff -r` of the two read clean and the two
  `ledger/stamps.json` have one SHA-256. It is a copy, the original untouched; no stamp was hand-written.
- Ledger line 10:13:58Z, start 10:14:03Z, end 10:14:44Z by Ctrl-C twice, `viola run` exit code 0.
- `process-start{subject:"claude-child"}`: `cli_version` `2.1.287`, `cli_verified` **`true`**. Its
  `env_stripped_known`, names only (the names PATH `claude` hands this builder session, the one path already ruled):
  `CLAUDECODE`, `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_EXECPATH`,
  `CLAUDE_CODE_MESSAGING_SOCKET`, `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_SESSION_ATTENDED`,
  `CLAUDE_CODE_SESSION_ID`, `CLAUDE_EFFORT`, `CLAUDE_PID`.

| step | verb | issued after the long turn's Stop | exit | document | elapsed |
|---|---|---|---|---|---|
| 1 | `send` the long text | — | 0 | `ok`, cursor 445 | 0.039 s |
| 2 | `wait --after 445 --timeout-ms 60000` | — | 0 | `turn-ended` | 3.796 s |
| 3 | `send` the short text, at once | 0.001 s | **13** | `{"v":1,"refusal":"not-delivered","detail":"input-not-ready"}` | 0.626 s |
| 3b | the same `send`, human mode | 0.628 s | **13** | (the lines below) | 0.004 s |
| 4 | `wait --timeout-ms 20000`, no `--after` | 0.632 s | 0 | `{"v":1,"ok":{"timed_out":true}}` | **20.017 s** |
| 5 | `wait --after 445 --timeout-ms 5000` | 20.649 s | 0 | the logged `turn-ended` | 0.004 s |
| 6 | `send` the short text again | 20.653 s | 0 | `ok`, confirmed | 0.021 s |
| 7 | `send` the newline text, on a settled input box | 22.520 s | **13** | `{"v":1,"refusal":"not-delivered","detail":"no-prompt-submitted"}` | 10.014 s |

- **Step 3.** The screen held no input-box literal and held the hint when the send was issued and when it
  returned. The wrapper's `send-refused` record: `refusal` `not-delivered`, `detail` `input-not-ready`, no cursor;
  its diagnostic line `side` `wrapper`, `wheel` `driver`. No `send-issued` record: nothing was typed. No
  `parse-rejected{parser:"vt100-feed"}` line stands before it, so the refusal is the hint's (research M7). The
  0.626 s is the screen going quiet after the Stop, then the 300 ms quiet period.
- **Step 3b, the refusal's lines as printed** (stderr; stdout empty):
  ```
  [/ ] unable         hint-v  not-delivered  input-not-ready
  hint: hint-v was not ready for input; viola wait hint-v, then send again
  ```
- **Step 4.** No record woke it: it returned `timed_out` at its own deadline. The footer's literal came back 4.176 s
  after the Stop, 3.5 s into the wait. With no `--timeout-ms` such a wait has no deadline (research M7, read from the
  code, not run).
- **Step 6.** The first send to succeed after the long turn was this one, issued 20.653 s after the Stop: it had to
  wait out step 4. It is an upper reading, not the window's end; the window ended 4.176 s after the Stop.
- **Step 7, the newline hypothesis end to end.** `send-issued`, then `prompt-submitted` filed **`human`** with a
  `text` of 1 499 chars (the text sent less its last LF), then `wheel` to the human (`cause` `human-input`), the
  turn's `turn-ended`, and `send-refused` `no-prompt-submitted` when the 10 s window expired. The text was
  delivered and ran a turn; the send was reported not delivered and the driver lost the wheel.

## Live session 3 — an unstamped home (start 3)
This run measures what the product already does on an unverified CLI. It is a reading and no new widening
(the overseer's review, inputs#I4).

- A fresh `target/e2e-home/viola-hint-20261007T101508Z-u/vhome` that `viola run hint-u` created itself.
- Ledger line 10:15:08Z, start 10:15:13Z, end 10:15:38Z by Ctrl-C twice, `viola run` exit code 0.
- `process-start{subject:"claude-child"}`: `cli_version` `2.1.287`, `cli_verified` **`false`**; the same ten
  stripped names.

| step | verb | issued after the long turn's Stop | exit | document | elapsed |
|---|---|---|---|---|---|
| 1 | `send` the long text | — | 0 | `ok`, cursor 445 | 0.034 s |
| 2 | `wait --after 445 --timeout-ms 60000` | — | 0 | `turn-ended` | 4.223 s |
| 3 | `send` the short text, at once | 0.001 s | **0** | `ok`, confirmed, cursor 2466 | 0.642 s |
| 4 | `wait --timeout-ms 20000`, no `--after` | 0.643 s | 0 | `turn-ended` (step 3's own turn) | 1.512 s |
| 5 | `wait --after 445 --timeout-ms 5000` | 2.156 s | 0 | the logged `turn-ended` | 0.004 s |
| 6 | `send` the short text again | 2.159 s | **0** | `ok`, confirmed, cursor 3081 | 0.649 s |
| 7 | `send` the newline text, on a settled input box | 7.031 s | **13** | `{"v":1,"refusal":"not-delivered","detail":"no-prompt-submitted"}` | 10.020 s |

- **Step 3: the short text was typed while the hint stood, and the real CLI submitted it.** The partial gate read
  the quiet screen `Ready` with no row read. Records: `send-issued`, `prompt-submitted` filed `driver` with a `text`
  equal to the text sent (92 chars), `send-confirmed`. The screen held the hint and no input-box literal when the
  send was issued and when it returned.
- **Step 6: the same again**, 2.2 s after the first Stop and still under the hint: confirmed, the text equal.
- The second paste was a new prompt each time, not an expansion of the first paste. Unmeasured: a second **long**
  paste under the hint, and a second paste of the **same** text. In the scratch session two different long pastes
  300 ms apart, before any Enter, arrived as two pairs of one prompt.
- The signature rows afterwards: `⏸ manual mode on · ← for agents`, on a quiet screen 0.318 s after the third
  turn's `wait` returned; the literal's first return was 8.017 s after the long paste.
- Step 7 read as on the verified home: `prompt-submitted` filed `human`, 1 499 chars, the wheel to the human,
  `no-prompt-submitted`.
- No session was left in a state Ctrl-C twice did not end; none was killed.

## After the two runs
- Each driver's probe dir is gone; the eleven dirs standing before are the same eleven (`probe-dir-census.md`).
- The two live homes hold real CLI content (a turn's message in `events.ndjson`). Only codes, counts, times and
  names were copied here; scratch copies of their logs stay in the session scratchpad, outside the repository.
- **The two live homes were moved, not removed** (the overseer's direction, inputs#I5, after the permission layer
  denied a plain `rm -r`; a denied removal is not done through another tool). With the rehearsal's fake-agent home
  they were moved by full name, before any gate ran, out of `target/e2e-home` into `target/e2e-home.disk/`, which
  is inside the workspace, git-ignored, and already on the founder's desk list for removal:
  - `target/e2e-home.disk/viola-hint-20261007T101352Z-v/` (the verified copy, real CLI content);
  - `target/e2e-home.disk/viola-hint-20261007T101508Z-u/` (the unstamped home, real CLI content);
  - `target/e2e-home.disk/viola-hint-rehearsal-20261007T101125Z/` (fake agent only).
  Read back at 2026-10-07T10:25:45Z: `target/e2e-home` holds none of the three, and the standing stamp home is
  where it was. Their deletion is the founder's at the desk.
- Residual: the CLI's own transcripts of the two sessions under the user's Claude projects dir, the accepted class.
- Live starts used: 3 of the cap of 10.
