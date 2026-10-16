# Implement outcome — 2026-10-16-unwrapped-sessions-on-the-board

**Outcome: green + smoke** (smoke ran as P2 gates), with **one red left open** (below). v1-37 is `implemented`.
The operator pass ran (`chunks/{marker}/evidence/operator-pass.md`): pre-CI commit `de9ee3f`, its CI read red on
`windows-2025` (seven `--record` cases — deviation 12), fixed, the block green again; the fix commit and its CI
read follow in that record.
Next → the **operator pass**: entries 22 (`gate.py hygiene`), then the pre-CI commit, 23 (the push), 24
(`ci.py conclusion`), then `/andromeda-wrap-session`.

## Gates (trail `.andromeda/runs/2026-10-16T20-14-16-implement/gate-2026-10-16-unwrapped-sessions-on-the-board.json`)
- Firing 1 (`--only 1-6`): green 6.
- Firing 2, the live round (`--live-legs`): `round: COMPLETE · legs fired 1/1`; entry 8 read
  `stamped 2.1.287  23 pass  0 fail`; evidence `chunks/{marker}/evidence/round-204738Z.txt`. 6 live starts.
- Firing 3 (whole block): green 17 · red 1 (21, `pre-push`: `cli_answer
  path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human`, see the deviations) → fixed → `--only 21` green.
- Firing 5 (whole block): green 17 · red 1 (11, the open red below) → `--only 11` green (11.3), then after the
  `cross_list_rows` Windows strengthening `--only 1,2,11` green (1.4, 2.4, 11.4).
- Legs not fired by `run`: 7–9 (the round, fired once above), 22–24 (`operator`, the pass's).
- No deferral: every changed-surface gate ran; Rust delta in every firing.
- Beside the block: the Windows target's clippy, check only (`CARGO_TARGET_DIR=target/wincheck`), exit 0, twice
  (the workspace, then `cross_list_rows` after its edit).

## The open red (not closed by a green re-run)
`viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, firing 5's entry 11 (log
`$TMPDIR/andromeda-gate/2026-10-16-unwrapped-sessions-on-the-board/implement-2026-10-16T20-14-16/11.2.log`):
`assertion left == right failed: stdout: {"v":1,"ok":{…}} left: Some(0) right: Some(10)` — the second send was
delivered where the case asserts `human-typing`. The case races the fake agent's 3 s paste-hint hold against the key
the test writes after the send's channel request.
- Same tree, other firings: green in firing 3's entry 11 and in `pre-push` 21.2 and 21.3 (21.3 later in the same firing).
- Host record (`hostwatch.py read`): the red window load mean 40.3, cpu-some mean 33 %, verdict QUIET; the green
  windows load mean 37.1 and 41.2 — no difference that explains it.
- Two-sided, after the red, this tree against `git archive HEAD` (`92264a7f254c`) in `target/control-92264a7/`:
  the case alone 5/5 and 5/5 green; `binary(cli_send)` alternating 3/3 and 3/3 green; the entry's filter
  alternating (control without `cross_list_rows`) 2/2 and 2/2 green. 20 runs, no reproduction on either side.
- This chunk changes nothing on the case's path (the send, the gate, the paste-hint hold); the fake agent's
  registry writes one file at each UserPromptSubmit and Stop, before the hook, outside the hold.
- Basis for "not this chunk's": UNOBTAINABLE locally — the red reproduces on neither tree. Cause: not known. The
  wrap pins its owner.

## Files changed (53 by `gate.py scope`: 37 listed, 16 recorded, 0 widening; scope `clean`)
New: `crates/viola-agent-claude/src/agents.rs`, `crates/viola-agent-claude/proptest-regressions/agents.txt`,
`src/run/agents.rs`, `fuzz/fuzz_targets/claude_agents_json.rs`, `fuzz/corpus/claude_agents_json/` (6 seeds),
`schemas/claude-agents-fixture.v1.json`, `fixtures/claude/2.1.287/Agents.recorded-1.json`, `tests/cross_list_rows.rs`.
Modified (listed): the agent crate's `lib.rs` and `ledger.rs`; `viola-state`'s `board.rs` and `snapshot.rs`;
`src/run/send/slot.rs`, `src/run/mod.rs`, `src/run/version_gate.rs`, `src/cmd/list.rs`, `src/human/board.rs`,
`src/cmd/revive.rs`, `src/cmd/verify.rs`, `src/cmd/verify/typed.rs`, `src/bin/viola-fake-agent.rs`,
`schemas/diag-line.v1.json`, `fuzz/Cargo.toml`, `.gitattributes`, `.config/nextest.toml`, `tests/cli_list.rs`,
`tests/cli_revive.rs`, `tests/cli_verify.rs`, `tests/contract_ledger_probes.rs`, `tests/contract_fixture_hygiene.rs`,
`tests/contract_fake_agent_drift.rs`, `tests/support/fake.rs`. Listed and unchanged: `fuzz/Cargo.lock` (a `[[bin]]`
adds no package).
Recorded (`chunks/{marker}/scope-record.md`): 15 companions — the 12 `InstanceSnapshot` literal sites and
`src/run/send.rs` (step 4), `crates/viola-e2e/src/harness/run.rs` (its `LEDGER_ROWS` and `LIVE_PASS`, step 10),
`tests/support/home.rs` (`dialogless_home`'s summary, step 10); 1 in-intent — `tests/support/cli.rs` (step 14).

Counts, each read by its rule: verify spawn pairs 8 (`cli_verify` literal); relied rows 23 / 20
(`LedgerRow::relied()`); the fixture set 33 (`ls fixtures/claude/2.1.287 | wc -l`); fuzz targets 6
(`ls fuzz/fuzz_targets | wc -l`); the corpus 6; `process-exit` detail 13 (the schema enum's members); the nextest
kill line 19 `binary(…)` names.

## Deviations from the plan, with their justifications
1. **Step 1:** `agents::parse` returns `AgentsList { rows, skipped, tolerated }`, not `Vec<AgentsRow>`: the step asks
   that skipped elements and tolerated keys be counted, and a bare `Vec` carries no count.
2. **Step 2:** proptest wrote `proptest-regressions/agents.txt` from a real shrunk failure — an oracle defect, not a
   parser one: a float (`6.137…e-208`) does not round-trip through serde_json's text, so comparing a row with the
   pre-serialisation value failed. The oracle now compares against the input re-parsed from its bytes; the seed is
   kept and replays green.
3. **Step 3:** the over-the-cap reading is a new `run_capped(…, cap)` that `run_bounded` now calls with `MAX_FRAME`
   (its two callers' readings unchanged); the agents read passes `MAX_FRAME + 1` and `parse` reads `oversize`. The
   spawn pair and the `parse-rejected` line are written by one helper, `run::agents::read_logged`, which each caller
   (list, revive, verify) calls at its own site; `run_bounded` stays unlogged.
4. **Step 4:** the snapshot's `agent_session_id` is written **before** the `session-start` line can be read, not
   after it: written after, it raced every reader that waits on that line (cli_list's `path1` and `e5` cases read
   the snapshot mid-rewrite, both red). Before is the wheel's own precedent in the same function.
5. **Step 6:** `list` looks `claude` up from its current directory; an unreadable current directory is a failed
   read (the board as the disk gave it).
6. **Step 7:** NAME's width counts characters, not bytes, so a non-ASCII session id keeps the columns (the format
   width counts characters).
7. **Step 11:** Run B's read is made once its first turn has settled (`typed.turn` holds a screen), before the
   framing pastes, not directly before `/clear` — still inside the plan's window. Placed before `/clear` it tied
   the agents row to the framing pastes: a run that stopped pasting early failed the agents row too (two
   `cli_verify` cases). The capture is carried to `verify` in memory (`typed::measure` returns it), not written to
   the captures dir.
8. **Step 13:** the fake's registry row is written just before its launch SessionStart fires, so a reader of the
   `session-start` line already finds it. Rows of a fake killed by SIGKILL (verify's Runs C, D, E) stay in
   `sessions/` beside the binary and are left out of every listing by the start-time check; the fake removes only
   its own row, as the plan says.
9. **Step 14:** a plain `support::cli::spawn` of `list` runs with `PATH` set to an empty bin dir beside the home, so
   `cli_budget`, `chaos_forward_compat` and `chaos_torn_append` (they call it) read no host session without edits;
   `spawn_on` / `viola_on` take a planted bin dir. `viola revive` tests already put the fake as `claude` first on
   `PATH` (`claude_dir`), and its copy now carries the planted agents files.
10. **Step 15:** `cli_revive`'s `assert_refused` counts the wrapper's own `process-exit{subject:"self"}` only (its doc
    already said so): the agents read adds an `agents-probe` exit line. The joined `cli_list` case holds its turn
    by a `viola send` the fake submits and never stops, not a script. `cross_list_rows`' Windows terminal case reads
    each escaped name as text under a 200-column terminal (ConPTY re-renders the stream with escapes of its own;
    the no-ESC assertion is Unix-only).
12. **Step 13, after the first CI read (red on `windows-2025`):** the fake's registry row carries the launch
    SessionStart fixture's recorded `cwd` (`~` without one), not its real working directory: the real one leaked the
    runner's checkout path into `verify --record`'s kept row, which `--record` refused (`absolute-path`), seven
    `cli_verify` cases. Guarded in `contract_fake_agent_drift` (red with the real cwd, green restored).
11. **The plan's integration filter** did not select `cli_answer`, whose `tests/support/home.rs` `dialogless_home`
    pinned `18 pass  4 fail`; only `pre-push` found it (now `19 pass  4 fail`, `16` on Windows).

## The P4 / P12 leg (step 17) — `chunks/{marker}/evidence/live-p4.md`
2 starts (7 and 8 of 8), ledgered before each (`evidence/live-sessions.ndjson`). **P12: follows** — after `/clear` the
agents row of the session's pid names the new id. **P4: two writers** — a second `claude --resume <id>` of a live id
started under the same id; two live rows carried it. With them standing, `viola revive p4a` was refused
`session-live`: exit 1, the founder's pair, the hint last, one `process-exit{detail:"session-live"}`, no child.
`session-live` therefore stays a refusal; no warning conversion is owed.

## Live starts: 8 of the founder's 8
6 in the record round (20:46:00Z) and 2 in the leg (21:10:50Z, 21:11:49Z), each ledgered with `date -u` before it
(`live-starts.md`). No red round, no re-fire.

## Process hygiene (measured against the host's process list at 21:13Z)
| process | started by | final state |
|---|---|---|
| the record round's `viola verify` and its six `claude` children | the round (entry 8) | terminated — round census: no `claude` in a probe dir; entry 9 green |
| the harness smoke session `p-usb-smoke` (supervisor, wrapper, fake agent) | the smoke entries (16) | terminated — entry 20 read `processes_gone` and `endpoint_gone` true |
| the rig hosts `p4a` and `p4b`, their `viola run` wrappers and `claude` children | this run (the leg) | terminated — each rig `exit_code 0`, `killed false`; census 21:12:42Z: none left |
| `viola revive p4a` | this run (the leg) | exited 1 at once; started no child |
| the two-sided reruns (harness `run --integration`, both trees) | this run | terminated — each run's document read, no process of this repository left |
| `claude` pid 3231528 (cwd: the repository) | not this run — the implementing session's own CLI | left running: it is this session |

A census of every process whose executable lies under this repository: none.

## Left on disk (not removed)
- `target/control-92264a7/` (the control tree and its build) and `target/p4-live-20261016T211033Z/` (the leg's two
  session dirs): under the ignored `target/`.
- `target/e2e-home/viola-record-20261016T204612Z/` (the round's stamped home, the leg's home): on the tmpfs.
- The session scratchpad's `control/`, `control.tar` and `p4-rig/`: a recursive `rm` of the first two was refused
  by the host's deny rule; they stay there.

## Decisions & corrections
- The founder's live-start cap for this chunk is 8 (his dialog 2026-10-16T19:54:09Z, relayed in the invocation):
  6 + 2, each ledgered with `date -u` before it, a red round never re-fired — held: 8 used, none re-fired.
- The operator's invocation: run each tool bare and read it as printed; journal each long run in the run dir before
  the wait (`journal.md`); run the operator pass with the `ci.py` conclusion read; stop ready for the wrap.
- Guard refusals: a `cat` heredoc with a file target (re-issued through Write / Edit), a `cd` that moves the cwd
  (twice, re-issued with absolute paths), and a recursive `rm` refused by the host's deny rule (not re-issued; the
  targets named above).
- A field path into the matrix was probed before the contract's schema was read (`KeyError: 'acceptance'`); the
  contract was then read and the value taken from `verification.acceptance`.
- Standing learnings met: `testing.md` 2026-10-16 (an oracle copied from the first run's output — the cli_verify
  summaries and spawn-pair lists were taken from the run's own output); `testing.md` 2026-10-06 (an acceptance held
  only in substance on Windows was closed by extending the witness); `verification-harness.md` 2026-09-29 (a local
  red judged two-sided, left open while its cause is unknown).
