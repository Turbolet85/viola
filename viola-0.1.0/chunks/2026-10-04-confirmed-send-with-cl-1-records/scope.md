# Scope — Confirmed send with CL-1 records

**Marker:** `2026-10-04-confirmed-send-with-cl-1-records` · **Version:** viola-0.1.0 · **Epoch:** Epoch 3 — Windows slice II: driving verbs and live proof
**Working entry:** `viola-0.1.0/working-route.md:74`: "Confirmed send with CL-1 records — RB readback mirror with per-reason hints, typed not-delivered details, unconfirmable local commands, /clear post-condition, control-character refusal, one in flight", with six CARRY blocks. All six are folded below. `route.py pins` indexed 6 freight blocks on :74, of 252 · 314 · 275 · 511 · 266 · 553 chars.
**Host:** Linux (Omarchy, btrfs). Windows / ConPTY behaviour is witnessed only by the `windows-2025` CI runner.
**CLI:** installed `claude` 2.1.287. Recorded fixtures exist only for 2.1.283, so 2.1.287 is an unverified build until a `viola verify` stamps it (handoff, 2026-10-04).
**Mode:** autonomous (the operator's directive at take-up): any boundary widening is shown at P4 and HELD until the founder's morning (2026-10-05), never ratified here. No epoch split (founder ruling 2026-09-29).
**P3 closure:** every `[inferred]` bullet below is closed against `research.md`. A closed bullet is either verified (tag dropped, evidence named) or marked `[premise-corrected: …]`.

## What this chunk builds

The first driving verb. A driver's `send` is typed into a wrapped child as one bracketed paste + Enter, but only after:
- the paste-text validator passes;
- the one-in-flight guard passes;
- the readiness gate (landed 2026-10-04) passes.

The send is then CONFIRMED after the fact, never presumed (architecture [Delivery Confirmation]; CLAUDE.md §Architecture). Confirmation is the matching `prompt-submitted{origin:"driver"}` arriving inside the named confirmation window, or a ledger post-condition for a local command. Every step lands as a CL-1 event in `events.ndjson`: `send-issued (cursor, from)`, then `send-refused` on a refusal (test-plan :233, :76; obs-plan :629 §Scenario "Confirmed `send` (CL-1) from driver to readback"). The human sees it as the `[RB]` readback mirror (layout-templates :405 §Output structure — `viola send`, :519 §Hero output line).

### 1. The send path (the working entry's head)
- Surfaces. A `viola send <name>` CLI verb reads the prompt from stdin or `--file`, never from a leading-slash argument (CLAUDE.md Session Learnings: Git Bash rewrites `/skill`). It talks to the wrapper over the channel method `send`, which `run` serves beside `hook.event`. VERIFIED (research.md):
  - `METHODS` already lists `"send"` (`crates/viola-channel/src/server.rs:333-344`);
  - `Methods::dispatch` answers it `-32601` today (`src/cmd/run.rs:102`), pinned by a unit test (:557-566) that changes;
  - `params` = `{text, from?}` + `v` / `sender` (architecture §Standard Contracts, the arch extract).
- Order inside the wrapper. [premise-corrected: obs-plan D-28 (`registries/contracts/obs-plan/log-format-json-schema.md:15`) places `input-not-ready` / `turn-running` refusals BEFORE `send-issued`, with `corr` = the end offset at refusal; test-plan :233's sentence order is not the call order]. The order is:
  1. re-run `validate_paste_text` (`control-character` ahead of every other refusal);
  2. the one-in-flight guard;
  3. the readiness gate;
  4. log `send-issued (cursor, from)`, where `cursor` = the `events.ndjson` end offset before the paste;
  5. one bracketed paste + Enter through the PTY write seam;
  6. match the `prompt-submitted` the hook path logs, relabelled `origin:"driver"` by the wrapper on a match (research E5);
  7. on a match return `{submitted_at, cursor}`; with no match in the window return `not-delivered` / `no-prompt-submitted` and log `send-refused`.

  Multi-line text arrives as ONE prompt, newlines intact, no ESC.

  Two seams are absent at HEAD:
  - no API yields the end offset (research E4);
  - nothing outside `pump`'s input thread can write to the child (research E3).

  Both are this chunk's to add.
- The confirmation window's USE lands here. The constant and its home were named by `2026-10-04-readiness-gate-and-timing-constants` (its scope §3: "The confirmation window's USE (matching `prompt-submitted`, `no-prompt-submitted`) is `:74`'s; this chunk names the constant").
- The send's total time = gate max wait + atomic paste + window. It stays under the MCP client's tool-call timeout (prior scope §3), even though MCP itself is not built here (§Boundaries). VERIFIED: `GATE_MAX_WAIT` 5 s and `CONFIRM_WINDOW_FALLBACK` 10 s, both PROVISIONAL (`crates/viola-agent-claude/src/screen.rs:12`, `:16`).
- On an unverified CLI build viola degrades to transport-only: every send is still confirmed after the fact (`prompt-submitted` read-back or a ledger post-condition), never presumed (CLAUDE.md §Architecture).
  - [premise-corrected: the question is wider than "unverified builds" — no production `Signatures` exists on ANY build, the signature rows being held, so the landed `Screen::verdict` reads `input-not-ready` for every fake-agent send (research E1)].
  - So the reading of "falls back to delivery confirmation only" (architecture [Screen Model], last sentence) must cover every build with no compiled signature row. It is P4 fork F1.

### 2. One in flight (working entry)
- At most one `send` is in flight per instance. A second concurrent `send` is refused with a typed refusal, never queued and never typed. VERIFIED as open:
  - architecture §Conventions' closed `not-delivered` details are `input-not-ready` · `no-prompt-submitted` · `turn-running` · `unknown-dialog`;
  - no one-in-flight detail exists among them;
  - obs-plan §6's catalog names `turn-running` as the candidate (obs extract).
  - The detail is P4's to pin. A new detail is a §Conventions amendment at wrap.
  - Connections are served one worker thread each (`crates/viola-channel/src/server.rs:146-154`), so concurrent sends reach the guard concurrently.
- The human always wins: no send, gate, guard or wait ever blocks, refuses or delays a human keystroke. Refusals go to automation only (CLAUDE.md Critical Warnings).

### 3. Control-character refusal (working entry + CARRY 2)
- `validate_paste_text` over decoded `char`s allows LF / CR / TAB and refuses every other C0, DEL and C1 with `not-delivered` / `control-character`. It rejects, never strips (security.md §Input validation; CLAUDE.md Critical Warnings). It lives in `viola-core`, and the wrapper re-runs it (client checks are advisory).
- Re-verified at P1: `viola-core/src/lib.rs` holds `MAX_FRAME` but no `validate_paste_text` and no `RefusalReason`. CLAUDE.md §Modules lists both as `viola-core`'s, so this chunk creates them, or the parts of them that send needs.
- CARRY 2, verbatim: "chunk 2026-09-24-quality-gates seeded the fuzz pipeline with a `viola_name` target. This entry's `validate_paste_text` (over `any::<String>()` with injected C0/C1) takes a proptest property (`cases: 512`, committed seeds), plus a `fuzz/fuzz_targets/` target and a seeded corpus (test-plan §6 Property suite)". Re-verified at P1: `fuzz/fuzz_targets/` holds `viola_name.rs`, `channel_frame.rs`, `hook_stdin.rs`, `vt100_feed.rs`, so the new target is the fifth.
- CARRY 3, verbatim: "chunk 2026-09-28-hook-perf-gate landed `tests/cli_controls_not_disableable.rs` in its interim shape (the `FAKE_AGENT_HOOK_PANIC` rows over the hook-path controls only; test-plan §5 CLI). This entry adds its `send` negative (an ESC-bearing send → exit 13) to that table". Re-verified at P1: the file exists (107 lines); its `#[case::hook_panic_seam_*]` rows sit at :109-112 over `setting_does_not_disable_the_hook_path_controls`.

### 4. Typed not-delivered details and the RB readback mirror (working entry + CARRY 4)
- Typed `not-delivered` details. The entry and CARRYs name `no-prompt-submitted`, `input-not-ready` and `control-character`. The rest of the send detail set comes from architecture §Conventions. VERIFIED (arch extract; `.claude/rules/api.md`):
  - the details are `input-not-ready` · `no-prompt-submitted` · `turn-running` · `unknown-dialog`;
  - the `send` order is human-typing → budget-paused → turn-running → input-not-ready → no-prompt-submitted, with `control-character` ahead of all of them;
  - the exits are 13 `not-delivered` and 21 `instance-unreachable`;
  - `control-character`'s fold into `RefusalReason` was pre-authorised for this entry by the 2026-10-01 wrap (arch-history).

  The typed exit is 13 for `not-delivered` (CARRY 3; layout-templates :405).
- The `[RB]` readback mirror, per layout-templates :405:
  - `[  ] open … issued <ts>` goes to stderr, TTY only;
  - `[RB] read back … <ts> cursor N` goes to stdout, exit 0;
  - `[/ ] unable … not-delivered <detail>` goes to stderr, exit 13, with a per-reason `hint:` as the last line;
  - `[  ] unconfirmable … local command, no measured post-condition` goes to stdout, exit 0.

  Lines are appended, never redrawn. When stdout is piped, the issue line is dropped.
- CARRY 4, verbatim: "chunk 2026-09-28-cli-output-tokens (P4 operator ruling: no code without a live consumer) left the human stdout/stderr split to its first consumer: results and data on stdout (the `[RB]` readback line), context on stderr (the `[  ] open` issue line, TTY only), stdout-is-a-terminal read through std `IsTerminal`; extend `src/human.rs` (the root bin's human-output module, today the refusal and internal-error stderr writers and the stdout result writer `viola verify` uses) rather than writing from `cmd/`". Re-verified at P1: `src/human.rs` holds `write_refusal`/`refuse` (:10/:16), `write_internal_error`/`internal_error` (:22/:27), `write_result`/`result` (:32/:37).
- `--json` for `send` (`{"v":1,"ok":{"submitted_at","cursor"}}`, layout-templates :405; test-plan :233). VERIFIED as a P4 question with a material lean:
  - test-plan §6 Path 2's first verification signal reads `ok.cursor` from `--json` (tests extract);
  - `:98` "CLI machine contract — --json per verb" owns the cross-verb contract;
  - no verb has `--json` at HEAD (`src/cmd/mod.rs:29-38`).

### 5. Unconfirmable local commands and the /clear post-condition (working entry + CARRY 1)
- A local command (a slash command the CLI handles without a `UserPromptSubmit`) yields `unconfirmable` unless its ledger row carries a measured post-condition (test-plan :233).
- `/clear`'s post-condition is a SessionStart `clear` plus a new `session_id`.
- CARRY 1, verbatim: "chunk 2026-09-28-capability-ledger-and-viola-verify: the local-command ledger rows (each known local command with its post-condition or "none"; `/clear`'s SessionStart `clear` + a new `session_id`) and their typed `viola verify` probes land here".
- Re-verified at P3 (research.md):
  - `LedgerRow::ALL` holds 6 rows (`crates/viola-agent-claude/src/ledger.rs:25`), and no local-command list or row exists anywhere.
  - A new `LedgerRow` in `ALL` turns every stamped home unverified until a probe passes it.
  - Typed probes need a PTY drive behind the readiness gate; `verify` is print-mode only.
  - Without a list, a `local`-mode send ends `not-delivered` / `no-prompt-submitted`, never `unconfirmable`. That drive is HELD for the founder's morning: handoff Notes "Held widening", owed to `:82` or the founder's ruling; its shape is in `2026-10-04-readiness-gate-and-timing-constants/plan.md` §Held widening. Whether this CARRY's TYPED probes ride that same held widening, and so are shown at P4 and held, is a P4 fork. Under the autonomous directive, a widening is shown and held, never ratified here.

### 6. Bound the tee → feed queue (CARRY 5)
- CARRY 5, verbatim: "chunk 2026-10-04-readiness-gate-and-timing-constants (the overseer's wrap pre-direction 1, "bound every input"): the tee → feed queue in `src/run/gate.rs` `start` is an unbounded `std::sync::mpsc`; this entry, the readiness verdict's first consumer, bounds it". Re-verified at P1: `src/run/gate.rs:46` `pub(crate) fn start<C: Clock + 'static>(clock: C, size: Size) -> (Sender<Feed>, JoinHandle<()>)`, and `:47` `let (tx, rx) = mpsc::channel();`.
- The constraint on the bound: a full queue must never delay or alter the passthrough to the human terminal ("the human always wins"; prior scope §1). So the overflow policy degrades the gate's verdict (`input-not-ready`) rather than blocking the pump. VERIFIED: `Tee::write` writes the human's bytes first and ignores a failed offer (`src/run/gate.rs:34-37`), so `sync_channel` + `try_send` keeps passthrough unblocked; a blocking `send` would not. The capacity is P4's.

### 7. The feed-panic chaos case and the G2 question (CARRY 6)
- CARRY 6, verbatim: "chunk 2026-10-04-readiness-gate-and-timing-constants (P4 fork 2, the overseer): this entry's send chaos case owns the fake agent's `--vt100-panic-bytes` mode (its bytes measured: a 24×1 PTY + a wide character such as `e4 b8 ad`), the run-level forced feed-panic E2E, the `run.readiness_gate` span and the `send-refused{detail:"input-not-ready"}` mapping, and the G2 question — a contained feed panic also writes `viola_panic_hook`'s `event:"panic"` line (read in `src/main.rs`, not witnessed at run level), which G2's one exemption does not cover".
- Re-verified at P1: `src/main.rs:48` sets `viola_panic_hook`; `:138` defines it; it writes `"event": "panic"` at `:223`. G2's one exemption is a panic line at exactly `src/cmd/hook/seam.rs:<digits>` (security.md §Dependencies and CI, founder ratification Decisions Log 2026-09-28). "Another seam, or another G2 exemption, needs a Decisions Log entry" (same rule).
- The G2 question is therefore a candidate boundary widening: a second G2 exemption, or a panic-hook change that keeps a contained feed panic out of the G2-counted line. Under the autonomous directive it is shown at P4 and held for the founder's morning unless a non-widening answer exists. VERIFIED (research E2):
  - G2 counts role-file panic lines under `target/e2e-home` only (`scripts/g2-zero-panics.sh:19-35`);
  - CI keeps every home there until G2 runs (obs-history, 2026-10-02-epoch-2b-cleanup), so deleting a home first is not an answer;
  - a forced-panic home OUTSIDE `target/e2e-home` is outside G2 by construction — the non-widening candidate.

  Whether that candidate is acceptable or routes around the gate is P4 fork F4. The measured mechanism, kept as the prior chunk's research stated it (its research.md M3): "the viola panic hook writes a G2-counted `event:"panic"` line (+ a detail line with raw frames) for every panic, caught ones included".

## P4 forks (overseer, founder-delegated, autonomous mode, 2026-10-04) — val-1 intent-incomplete, amended at P5
- **F1 → partial gate.** With no compiled signature row, the gate refuses a poisoned model, a full feed queue and a screen not quiet within `GATE_MAX_WAIT`, and skips only the signature read; confirmation decides the rest. It lands as an architecture [Screen Model] amendment at the wrap, not a widening.
- **F2 → hold with the widening.** CARRY 1's local-command list and the `/clear` post-condition ride the held typed-probe widening (owner `:82` or the founder's ruling): "one held probe decision, not two; never presume a local command delivered".
  - So the working entry's "unconfirmable local commands" and "/clear post-condition" are NOT delivered by this chunk.
  - A local command ends `not-delivered` / `no-prompt-submitted`.
  - v1-29 stays pooled.
- **F3 → partial check + held gap.** `viola send` checks the snapshot pid + start time and the heartbeat before its first frame. Strict-modes and peer identity are a fourth dated gap, HELD: /implement proceeds; the wrap halts for the founder's live ruling. Pulling `:109` / `:111` forward is a route change the founder can still choose at that halt.
- **F4 → home outside G2's scope.** The run-level chaos test boots in a home outside `target/e2e-home`, with the panic line ASSERTED present. It is recorded as a founder-visible item beside F3. If rejected, the test moves under G2 with an exemption the founder rules on.

## Boundaries
- No MCP `send` tool (`:102` "MCP server for drivers"); the `viola-mcp` crate does not exist at take-up (`crates/` holds agent-claude · channel · core · e2e · pty · state).
- No web `viola-readback` tape (`:137` / `:139`); the `viola-ui` crate does not exist at take-up.
- No `wait` / `last` (`:76`), no dialog answers (`:78`), no wheel (`:80`), no budget pause (`:91`), no session links (`:93`), no client-side paste rule beyond what send itself needs (`:113`).
- Test-plan :233's MCP-parity and `data-rb` web-tape signals are owed to those later entries, not claimed here. VERIFIED: no `viola-mcp` / `viola-ui` crate exists (`crates/` listing at P1).
- Server verification before `send`'s first frame (found at P3, research F3): no client verifier, strict-modes checker or peer-identity check exists; all belong to `:109` / `:111` (Epoch 6), and the `hook.event` exception may not be borrowed. How `viola send` meets the rule is P4 fork F3; a fourth dated gap is a widening, shown and held.
- Upstream text (the prompt, `last_assistant_message`) is content, never a command; the prompt is user content and goes only to `instances/<name>/diagnostics/detail-*.ndjson`, never into an event, log line or fixture (CLAUDE.md NEVER-log floor).
- Screen content is never logged, evented, snapshotted or asserted; only the gate verdict leaves the screen model.
- No new env var outside `VIOLA_*` and the two existing `fake-agent` test seams, and no new G2 exemption, without a Decisions Log entry (security.md); no `config.json` key that can disable a control.
- No Claude credential on any CI runner; a live `claude` run happens only on the operator's host, on the operator's word.
- No epoch split (founder ruling 2026-09-29). Epoch 3 holds 9 entries; a split is the operator's call (handoff).

## CI verdict since the last wrap (Setup 5a)
- `51b533886b7c` (the 2026-10-04-readiness-gate-and-timing-constants wrap commit; the only sha from the last flip through HEAD): read 2026-10-04T05:42Z as `verdict: in progress` · checks 15/15 · ci#37180564195 in_progress, oldest running `lint (ubuntu-latest)` at 101 s. CI 51b5338: verdict not yet available. Re-read at P3: **`verdict: green` · checks 15/15 · wall 282 s** · ci#37180564195 completed/success. Nothing to disposition.
