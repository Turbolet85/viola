# Outcome — 2026-11-21-cross-session-prefix-ledger-row · implement run 2026-11-21T13-20-00Z-implement

**Outcome: stuck** — the live start `XS` read red with a cause the plan's table reads as AMBIGUOUS, and the
overseer's directive (inputs#I4) stops implement before the landing and the operator pass. Nothing is retried; no
second start is made; `XS-2` stays shut. The working tree is preserved (nothing committed).

## What stuck
- `XS` (entry 28, claude 2.1.328 by path, VR arm, ledgered 2026-11-21T13:48:27Z, base line 1 of `--cap 1 --spare 1`):
  `verify-exit: 1` · `pass-count: 28/29` · `failing: cross-session-prefix` · `cross-session-prefix: fail` ·
  `cross-session: no-prompt` · `sender-tools: search send` · `send-result: refused` · `latency_ms` null · every other
  relied row pass (`browser-tools-prompt: absent`) · `keys: none` · `census: 0` · `credentials: removed` ·
  `scratch: absent` · `record: none` · `kept-stamp: written`.
- The cause table (plan §Implementation notes): `sender_tools` holds `send`, `send_result` `refused` → **C2 or C4 —
  the cross-session gate off in the sender, or the session id unresolved; ambiguous: both answer a failure; the start
  that splits them is the operator's.** `search` before `send` excludes C1/C3 (the deferred tool was fetched and
  called); the tool's own refusal excludes C5.
- After-reads: 33 `29 short` (red: 29 rows, one failing) · 34 recorded (the block above) · 29 `kept-floor: unchanged`
  · 30 `live-stamp: unmoved` · 31 `stale-sockets: before 141 after 146` (a reading) · 32 `census: 0`. 35, 36 not
  fired (no record dir on a red start). Full record: `.andromeda/cache/chunks/2026-11-21-cross-session-prefix-ledger-row/evidence/live-round.md`.

## Gates reached
- Phase A, entries 1–8: green (fmt, clippy, agent crate, root bin, the canary binary, `run --unit`,
  `run --integration` 595 passed, `run --e2e` 334 passed).
- Phase B, entries 9–18: green (entry 12 red once on two rig counts this chunk's step 17 moves — FLOOR_ROWS 24→25 and
  the summary's unread lines 10→14 — fixed and re-fired green).
- Controls: `prefix-match` holds · `no-payload` holds · `unread-shape` holds
  (`evidence/controls.txt`; the guard's own case failed in each removed log, no TIMEOUT line).
- Round: 19–27 green/recorded, 28 red. Not reached: 35–40 (the landing and its reads), the operator pass (41–47).
- Expected reds not fired by design: the five stamped-home `cli_verify` cases read `cross-session-prefix` fail over
  the committed 2.1.328 set (no recorded arrival until a landing), measured by hand during phase B.

## Files changed
29 files under research's lists plus one scope-record line:
- `crates/viola-agent-claude/src/ledger/test_cases_framing.rs` · companion · serves step 15 · self

## Deviations, with justification
1. The fake sender delivers only to a Run B whose receiver left `<pid>.listening` in the inbox: without it every
   verify case run without the framing replay waited the 30 s cross-session window past the 20 s test kill (plan
   step 13 named no such gate).
2. The `no-payload` control forced the payload filing on inside `capture_names_only` (the case calls that function
   directly; testing.md 2026-09-25 ext. 2026-10-07); the plan's table names the call-site return.
3. Phase A is the block's first EIGHT entries and phase B entries 9–15 (+16–18 the control entries); the plan's prose
   says seven and 8–14. Fired by the entries' phase notes.
4. The whole live round is refused at pre-flight (operator entry 32 and the unkeyed 33, 34 inside the span): fired
   `--live-legs --entry 28` (steps 19–27, then XS), 33/34 by `--entry`, 32 by `--operator`, and the round-keyed
   after-reads 29–31 by hand, verbatim from the block.
5. `sender_tools` / `send_result` stamp the string `none` where no file stood, `null` where the sender never started.
6. In the ledger unit fixture the cross-session arrival sits after the clear's SessionStart (index 12), so no earlier
   capture index moves; the reading counts only the prompts before it.
7. `src/run/auto_verify.rs`'s tests needed no edit: they read `LedgerRow::relied()`, not a literal count.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the round's entries 19–28 (XS's rig, its verify, its claude children) | a leg | terminated — round census, rig `census: 0`, entry 32 `census: 0` |
| the three control clones' firings | this run (gate.py control) | terminated — each `clone removed` |
| the 13 processes `gate.py census` lists (other projects' sessions, the prototype's wrappers, the product's waits, the pinned product's `viola mcp`) | NOT DERIVED — none of this run's | left running: not this run's — their owners |

## Decisions & corrections
- The overseer's word to go on (inputs#I5): host read from 2026-11-21T13:47:33Z read `QUIET - 0 s stalled`; the round
  fired.
- Guard refusal: the Bash guard refused a `cat` heredoc append to a test file; re-issued once through the Edit tool.
- Standing learning met: a whole re-record moves fixture session ids (testing.md 2026-11-21) — not reached, no landing.
