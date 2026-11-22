# P3 block — «Paste write bounded» (2026-11-22-paste-write-bounded)

**This block holds five forks of the operator's (F1–F5). The phase stops here, before P4.**

- **Window:** the phase run `.andromeda/runs/2026-11-22T10-36-57Z-phase`. The chunk was promoted at tx 3902 and the directive snapped as `inputs#I1`. P2's row read: 7 masters, 1081 of 4793 rows picked, 7 history files. Research is `doc:2026-11-22-paste-write-bounded/research` rev 1, and scope rev 3 is closed. No live start and no rig start were made. No claude ran. Nothing of the operator's `~/.claude*` was read.

## 1. What the code does today: measured with a child that stops reading

**The measuring child.** It is `stopchild.py`, a raw-mode child that never reads its stdin until it is released. The measurement drove the debug build at HEAD (`viola run`, transport-only). Everything ran under a test-owned user HOME, a test-owned viola home and a test-owned runtime dir. Runs 4 and 5 read alike on every point. The evidence is `.andromeda/cache/chunks/2026-11-22-paste-write-bounded/evidence/` (`p3-stopchild-run5.json` and the scripts).

| What | Measured |
|---|---|
| Bytes a raw pty takes, nobody reading | **20 480**, 3 of 3 runs, and the same with a sleeping child holding the slave (`pty_capacity.py`). A typed text past 20 467 bytes blocks. A canonical slave never blocked up to 8 MiB, but claude runs raw. |
| Largest paste viola sends | A typed text up to `MAX_FRAME` (16 MiB): `src/cmd/send.rs:57`. |
| Who blocks | The wrapper's `send`, on the channel connection thread. It runs ONE `write_all` (`crates/viola-pty/src/pump.rs:84-86`) while holding both the send's **flight guard** and the shared **input-writer lock**. |
| Driver, CLI `send` of 200 000 bytes | Blocked **47.0 s** (2/2), returning only after the child read again. Then its 10 s window ran. The CLI has no deadline of its own. Over MCP, a longer block reads `call-deadline` while the send keeps going. That is inferred from code; MCP was not run. |
| **Second finding:** a second send during the block | Waited **35.1 s** on the flight guard (2/2), then read `turn-running`. It is refused only after the blocked paste ends, not at once (`decision/send-never-queued`). |
| **Third finding:** the human's key during the block | The wheel moved to `human` / `human-input` **at once** (2/2). The key BYTE was held about 33 s and reached the child **after the paste's end marker and CR** (2/2). |
| **Fourth finding:** hook events during the block | The hook process returned in 4–7 ms (`hook.event` is a notify). Its `activity` line was **absent from `events.ndjson` until the child read again** (2/2): `append_hook_event` takes the same flight guard. A turn end or a prompt claim arriving then waits the same way. |
| The child's output, and the outer terminal | The child's output kept flowing: 39 ticks in 39 s. No viola byte reached the outer terminal (2/2). |
| Recovery | Only when the child reads again. The paste then lands whole, start marker + text + end marker + CR, and the held key after it. Viola never ends the block itself. |
| What a CUT would leave | A reading, not measured on claude. The bytes already in the kernel queue stay there, up to 20 480. A cut mid-frame leaves the start marker and part of the text, **with no end marker and no CR**: an unclosed bracketed paste, so every later byte, the human's keys included, reads as paste content. A cut must close the bracket. That closing write can block too, against a child that still does not read. With no CR, nothing is submitted. The next pasted text would then join the leftover in ONE prompt: its confirmation fails (`no-prompt-submitted`) while a mixed prompt WAS submitted. |

**Unmeasured, said:**
- how long a real claude takes to read a large paste. A slow but legitimate paste would be cut at the bound.
- how claude treats an unclosed or a closed-but-unsubmitted bracketed paste.
- the MCP path end to end.

All three need a live start, and this chunk makes none.

## 2. The answer paths, each priced

| Path | What it changes in viola-pty's contract (callers: `pump_child` for `run` and `revive` at `src/cmd/run.rs:641`; verify's `Run::paste_text` at `src/cmd/verify/typed/run.rs:182`; four `pump.rs` tests) | How it fails |
|---|---|---|
| **(B) A writer of its own with a bounded hand-off. RECOMMENDED** | One worker thread owns the PTY writer, under `catch_unwind` like the two pump threads. The human's copy thread and the paste hand it bytes over a bounded channel. `paste(text, bound)` waits for the writer's ack up to the bound. At the bound it returns a fixed-message `PtyError` variant (hand-written, no thiserror, no Tokio). The writer writes in chunks. On a cut it stops after its in-flight chunk, writes the end marker and **no CR**, then passes queued human bytes. | At most one chunk and the end marker land after the send has returned. That is harmless, because no CR is written. While the child never reads, the end marker and the human's keys stay queued, as they would against an unwrapped claude. If the channel fills, the human thread stops reading stdin and keys wait in the terminal's own buffer. obs rows naming "the two run PTY pump threads" become three. |
| (A) A non-blocking write with a deadline | Needs the master's raw fd through a new `Pty` seam method: portable-pty's writer exposes none, only the master's `as_raw_fd`. | portable-pty's reader and writer are `dup`s of ONE open file description, so `O_NONBLOCK` on the writer makes the output-copy thread's read return EAGAIN. `copy` ends on that error, and **the child's output stops**. The fallback is `poll(POLLOUT)` plus blocking writes, but a write larger than the room left still blocks. The room is unmeasured. Set aside. |
| (C) A timed blocking write | The same `write_all` on a helper thread with a timed join, or interrupted by a signal. | With a join: the write keeps the lock and **lands whole with its CR after the driver read "not delivered"**. A re-send then doubles the prompt. With a signal: process-wide `SIGALRM`/`pthread_kill` handling in a multithreaded binary, which can interrupt other threads' syscalls. Set aside. |

**The bound (F2): the arithmetic.**
- The longest delivered send is the `/compact` send: `GATE_MAX_WAIT` 8.5 s + P + `COMPACT_CONFIRM_WINDOW` 40 s < `MCP_CALL_DEADLINE` 55 s, so **P < 6.5 s**.
- At P = 5 s that send totals 53.5 s, leaving 1.5 s for the record writes, the hand-off and the frame I/O.
- A cut send totals 8.5 + 5 = 13.5 s. An ordinary send totals 8.5 + 5 + 10 = 23.5 s.
- The bound becomes a compiled constant beside `GATE_MAX_WAIT` in `crates/viola-agent-claude/src/screen.rs`. viola-pty takes it as a parameter, since it knows no agent.
- `mcp_call_deadline_sits_between_the_wrapper_bounds_and_the_client_read` gains the term.
- It is a NEW time bound, a named member of `a11y-plan · decision/time-bounds-never-on-human` and `· decision/time-bounds-fall-on-the-driver`. Both cells hold it under their 300-character bound: 278 → about 291, and 284 → about 297.

**"The wheel unmoved", in code.** The send ends through `Ctx::refuse` after `send-issued`, with its cursor. That is the path the paste-failure refusal takes today (`src/run/send.rs:168-171`). It writes no `wheel` record and moves no holder. The measured wheel move on a human key stays as it is.

## 3. Every master row the change reaches

All seven masters were searched (the `rec.py find` over each, and the picked rows by term). Every hit was already in the picked set.

**Reached:**
- **architecture:**
  - `decision/send-time-bound`: "plus the paste" becomes a compiled bound.
  - `decision/atomic-bracketed-send`: a cut paste is no longer "one write plus CR".
  - `contract/send/paste-failure-after-issue`: a cut is that failure.
  - `contract/refusal-order/send` and `contract/send-refusal/order`: the cut's place after `send-issued`; under F3's L1, the refusal of a send after a cut.
  - `decision/mcp-call-deadline`: its arithmetic.
  - With F1's W1 also `contract/refusal-reason/not-delivered` and `contract/cli-exit-codes/13`. With W2 instead `contract/send-results/input-not-ready` and `contract/human-hint-lines/input-not-ready`.
- **security-plan:**
  - `decision/bridge-send-one-bracketed-paste` and `decision/send-text-is-one-bracketed-paste`: a cut paste is part of the text, bracketed and closed, with no Enter.
- **test-plan:**
  - `contract/driver-send/typed-text`, `contract/viola-send/paste-framing` and `contract/send-event-log/send-issued`: "one bracketed paste plus Enter is written".
  - `decision/send-refusal-order`.
  - A NEW `contract/fake-agent-cli/*` row for the stop-reading argv option.
- **obs-plan:**
  - `decision/run-threads-inside-catch-unwind` and `decision/run-worker-panic-ends-run`: "the two run PTY pump threads" become three.
  - `contract/pty-seam-log/paste`: whether `pty.paste_write` carries a closed outcome.
  - With W1 also `contract/send-refused-detail/refusal-reason-words`.
- **a11y-plan:**
  - `decision/time-bounds-never-on-human` and `decision/time-bounds-fall-on-the-driver`: the new member.
  - `decision/human-keystrokes-never-blocked` and `contract/tui-passthrough/human-keystrokes`: the "current atomic paste" a key may wait behind becomes the bounded or cut one. The wording holds; what a key meets changes (F4).
- **design-system:**
  - With W1, `contract/cli-exit-codes/13` and a new `contract/refusal-hints/not-delivered-<word>`.
  - With W2, `contract/refusal-hints/not-delivered-input-not-ready`: its "send again" lands the next paste after the leftover text.
- **layout-templates:**
  - With W1, `contract/cli-exit-codes/13`.

**Read and not reached; the wording holds:**
- architecture:
  - `decision/send-flight-lock`: the paste stays under it, now for at most P. Its "a human key never takes the lock" holds: keys wait on the input-writer lock, not the flight lock.
  - `decision/send-never-queued`: a send behind a cut waits at most P.
  - `decision/human-always-wins-wheel`, `decision/every-send-confirmed`, `decision/unconfirmed-delivery-not-delivered`, `decision/gate-timings-compiled`, `decision/pty-error-hand-written`, `decision/one-error-enum-per-crate`, `decision/portable-pty-seam`, `decision/fake-agent-test-seams` (an argv option, no env seam), `contract/channel-methods/send`.
- security-plan: `decision/config-cannot-disable-controls` (the bound is compiled), `decision/vt100-feed-bounded-queue`, `contract/mcp-iserror/*`.
- test-plan: `decision/time-faked-through-seams`, `decision/cli-mcp-identical-payloads`, `contract/cli-exit-codes/14`.
- obs-plan: `decision/claude-boundary-only-instrumentation`, `decision/no-hot-path-instrumentation`, `contract/confirmed-send-spans/span-chain`, `contract/send-refused-detail/turn-running`.
- a11y-plan: `decision/human-key-takes-the-wheel`, held as measured.
- design-system: `decision/run-prints-nothing-while-wrapped`, held as measured.

**The widening, said here whole:**
1. a new closed word crossing to the driver, if W1;
2. a new time bound in the two a11y rows;
3. a change to what a human key meets;
4. under L1, a new refusal of a send after a cut.

No cell lacks room under its bound.

## 4. The operator's forks

**F1: the closed word at the bound.**
- **W1 (recommended):** a new `not-delivered` detail. Its name is his word; the plan's working name is `paste-cut`. Exit 13, with its own hint and no exit code added. CLI `send --json` prints `{"v":1,"refusal":"not-delivered","detail":"paste-cut"}`. The MCP `send` tool returns the same document less `v`, `isError: true`, with the text `refused: not-delivered`.
  - Cost: the viola-core word, the hint row, the exit-13 lists in three masters, obs's refusal words, the error census.
  - Until the re-pin, the pinned driver reads the new word as `unknown`, exit 14. That is a residual.
- **W2:** reuse `input-not-ready` with the cursor, as the paste-failure contract already says. No new word and no widening. The driver cannot tell a cut from an unready screen, and the hint's "send again" lands the next paste after the leftover text.

**F2: the bound's value.**
- **5 s (recommended):** the compact path reaches 53.5 s, a 1.5 s margin.
- **3 s:** 51.5 s, but a large legitimate paste on a slow claude is more likely to be cut. Claude's read speed is unmeasured.
- **6 s:** 54.5 s, a 0.5 s margin; too thin.

**F3: the cut text left in the box.**
- **L1 (recommended):** the wrapper refuses every send after a cut with the same closed word, until a human key or a prompt or session edge clears it. The hint says a human must look at the session. It reads no screen and types no byte.
  - Cost: wrapper state, plus one rung after the gate. A driver whose human is away stays refused.
- **L2:** nothing, kept as a residual. The next send pastes after the leftover, so a mixed prompt reaches the model and its send reads `no-prompt-submitted`.
- **L3:** a clearing key, which is a new typed byte class. An undocumented CLI behaviour needs a ledger row and a verify probe measured live, so it is a live start. That makes it a fork of the block: not in this chunk.
- **L4:** refuse until the gate reads the box empty. That is a new product screen reading (a `decision/no-screen-content-parsing` widening), and it works on a verified CLI only.

**F4: what a human key meets.**
- Today: held without a bound behind a blocked paste, and it arrives after the driver's CR.
- Under (B):
  - during a blocked paste, a key waits behind at most the in-flight chunk and the end marker. It still waits on the child to read, as against an unwrapped claude.
  - after a cut, keys land in the box after the leftover text and the closing marker.
  - the wheel moves at once, as today.
- Recommended: as described.

**F5: what "verified" rests on. This entry claims NO capability.**
- The pool's 12 unclaimed caps are fleet and console caps.
- The gap is v2-24's own `observed_gap`: "48.5 s plus the paste, which no bound caps". v2-24 is verified and claimed by «MCP calls bounded», and the matrix refuses a claim on a verified cap.
- Recommended: the wrap adds a dated note to v2-24 naming the compiled bound and its tests. The alternative is a new capability, which is Phase A's at the next version's intake.

**Technical, not his:**
- the fake agent's stop-reading switch is an argv option with its own test-plan row, never an env seam;
- the cross-process case waits about the gate plus 5 s, under the 12 s `WITHIN` and the 20 s lint;
- the unit cases pass a short bound as a parameter.

## 5. One implement window?

Yes, with W1 + 5 s + L1 + (B):
- the viola-pty writer worker and its unit cases on injected writers;
- the send path, the core word, the hint and mirror lines;
- the fake agent's option and one cross-process `tests/cli_send.rs` case;
- the arithmetic test and the error census.

**Left for the founder's desk.** These are test-owned runtime dirs, not deleted (no recursive delete): `target/pwb-xdg-3`, `target/pwb-xdg-4`, `target/pwb-xdg-5`. Each holds `viola/viola-<hash>.sock` and its `.lock`, left by a wrapper the harness ended with SIGTERM (exit -15); no process is left. The scratchpad runs `run1`–`run5` stand under the session scratchpad.

## 6. The answers (appended 2026-11-22T11:44:59Z; the relay is snapped as this chunk's input, cited by the plan)

The operator's own words, given in one dialog and read at 2026-11-22T11:44:50Z, relayed verbatim by the overseer:

1. **F1: «Новое слово paste-cut (Recommended)».** W1. A new `not-delivered` detail, **`paste-cut`** (his name; he
   kept the working name). Exit 13, its own hint, no exit code added. He was shown the cost and the residual (the
   pinned product reads the word as `unknown`, exit 14, until the re-pin). W2 is declined.
2. **F2: «5 секунд (Recommended)».** A compiled 5 s bound on the WHOLE paste. He was shown the arithmetic
   (8.5 + 5 + 40 = 53.5 s under 55 s) and the consequence: a very large text a real claude cannot read within 5 s is
   cut and refused where today it arrived slowly. Claude's read speed is unmeasured.
3. **F3 and F4: «Отказывать до вмешательства (Recommended)».** L1. After a cut, the wrapper refuses every send with
   `paste-cut` until a human key or a prompt or session edge clears it. The hint says a human must look at the
   session. No screen is read and no byte is typed. He was shown that a driver with no human at hand stays refused,
   and what a human key meets under (B). L2, L3 and L4 are declined.
4. **F5: «Датированная заметка к v2-24 (Recommended)».** This entry claims no capability. The wrap adds a dated note
   to v2-24 naming the compiled bound and its tests. Coverage stays 23/37.

The overseer's technical answers:

5. Path (B), as recommended; (A) and (C) are set aside. The writer thread runs under `catch_unwind`, and the obs
   rows naming two run PTY threads say three.
6. The plan names the three by-findings (a second send during a blocked paste, the human's key held, the hook event
   held behind the flight guard). For each it says what this change does and gives a test. The hook event is
   bounded by the same 5 s, or mended if that costs one line; a wider mend is a finding for the wrap.
7. The cut's own writes are bounded too. The closing marker goes through the same writer and never blocks the
   send's return. The plan says what stays queued against a child that never reads, and tests that the wrapper's
   exit still ends with no thread left blocked.
8. L1's state is the wrapper's, in memory. The plan names what sets it, what clears it (by record), that it
   survives no wrapper restart, its place in the refusal order and the event the log gets. A revived wrapper
   starts clear.
9. The fake agent's stop-reading switch is an argv option with its own test-plan row. The cross-process case stays
   under the 20 s lint; unit cases take a short bound as a parameter.
10. The WATCH is read at each run from the rehearsals' own lines. A red on a CONTENDED host is recorded, not re-run;
    the same red on a QUIET host is a finding, and the run stops on it.
11. The test dirs left (`target/pwb-xdg-3`, `-4`, `-5`, the scratchpad runs) are named on the card for the desk. A
    later test of this chunk uses a fresh dir of its own and cleans it by its own teardown.
12. The card: control ids 1–16 characters, each control on an entry filtered to its guard's own cases, an expect
    atom able to fail. A row not in §3 that P4 finds reached is a fork. No round, no live entry. When the plan is
    approved the phase ENDS and nothing else starts.
