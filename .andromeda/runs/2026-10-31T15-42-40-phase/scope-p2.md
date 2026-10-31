# Scope — Exit-cause code catalogue

**Chunk:** `2026-10-31-exit-cause-code-catalogue` · viola-0.1.0 · Epoch 6 — Security hardening (its last entry)

**The working entry:** Exit-cause code catalogue — every exit-21 and exit-1 cause a distinct code across --json, MCP and
process log, matching its human hint.

## What it builds

- Every cause of exit 21 (`instance-unreachable`) and of exit 1 reaches `--json`, MCP and the process log as its own
  distinct closed code, carrying no path and no pid, and each code matches the human `hint:` line of the same cause.
- [inferred] The exit-21 causes, from `verification-matrix.json#v1-18`'s requirement and test-plan's Exit-cause matrix:
  not running · unwrapped name · strict modes · server verification · endpoint lost mid-call.
- [inferred] The exit-1 causes, from the same two: already live · stale heartbeat · squatted name · pinned-hash mismatch ·
  script child · strict-modes or socket-directory refusal · internal error; beside them `viola revive`'s `no-session`,
  `session-live` and `cwd-missing`. P3 closes this list against the code: a cause the code holds and this list misses
  joins it, one the list names and the code cannot reach is named as such.
- The exit-cause table `tests/cross_exit_causes.rs` (test-plan's Exit-cause matrix, an rstest `#[case]` table, one
  negative test per cause): each case asserts the exit code, the `--json` document and a last stderr `hint:` line naming
  that cause, and a hint shared across causes fails the table. As of this take-up the file does not exist; each built
  cause is a case of the binary its row names.
- [inferred] A cause the code holds and no process-log line carries today gets its log form, or its absence is a named
  decision: the strict-modes refusals write no line by rule, because their check runs before the log opens.

## The freight, folded

- CARRY (chunk 2026-10-10-viola-revive, the founder's ruling of 2026-10-01 on that entry: each revive refusal has an exit
  code and a hint that join this catalogue): `viola revive` landed four exit-1 causes with their closed details,
  `strict-modes-failed` · `already-live` · `no-session` (two stderr pairs) · `cwd-missing`, each pair with its own hint,
  tested in root `tests/cli_revive.rs` and not in the `cross_exit_causes.rs` table (test-plan §6 Exit-cause matrix).
  `session-live` is not built (the entry "The board: viola list"). Their `--json` documents landed with chunk
  2026-10-17-cli-machine-contract (the CARRY below).
- [inferred] A staleness in that CARRY, as read at this take-up: `session-live` is written as not built, while test-plan's
  matrix and `.claude/rules/security.md` describe it as landed (chunk 2026-10-16-unwrapped-sessions-on-the-board). P3
  reads which holds.
- CARRY (chunk 2026-10-10-budget-governor, pinned by its wrap as a fact this catalogue's revive rows must hold):
  `revive`'s strict-modes refusal now writes no `process-exit` line, because its check runs before the wrapper log opens.
  Of the four exit-1 causes three carry a logged `detail` (`already-live`, `no-session`, `cwd-missing`); the strict-modes
  one is exit 1 and its stderr pair alone, and `Refusal::StrictModes.detail()` still returns `strict-modes-failed` with no
  line carrying it.
- [inferred] That CARRY's causal claim, kept with its marker: «because its check runs before the wrapper log opens». P3
  re-verifies the mechanism at HEAD, and whether `Refusal::StrictModes.detail()` still stands after chunk
  2026-10-31-sanitised-error-surfaces.
- CARRY (chunk 2026-10-11-the-board-viola-list, the review card the founder approved,
  `chunks/2026-10-11-the-board-viola-list/inputs/I3-relay-3.md.txt`,
  `chunks/2026-10-11-the-board-viola-list/inputs/I8-board-card-owners.md.txt`): `viola list` (exits 0 / 21 / 1, its exit 21
  the whole-board strict-modes refusal) joins the exit-cause matrix and the security-control table.
- CARRY (chunk 2026-10-16-unwrapped-sessions-on-the-board, its P5 review card's row, the operator's own yes,
  `chunks/2026-10-16-unwrapped-sessions-on-the-board/inputs/I2-relay-2.md.txt`): the unwrapped-name `instance-unreachable`
  hint (`<name> is not a wrapped session; only sessions started with viola run take commands`, design-system cli pattern 2)
  is not built; that chunk proves the exit 21 only (`tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable`).
- CARRY (chunk 2026-10-17-cli-machine-contract, the Viola overseer's ledger call at its wrap,
  `chunks/2026-10-17-cli-machine-contract/inputs/I3-relay-1.md.txt` Q2): `viola revive --json` landed the exit-1 document
  `{"v":1,"error":"<code>","detail":null}` for its strict-modes check and its preflight (`strict-modes-failed` ·
  `already-live` · `no-session` · `session-live` · `cwd-missing`); a refusal `run`'s start makes after a passed preflight
  (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) still prints
  no document under `revive --json` — owed here, its codes joining this catalogue. `verification-matrix.json#v1-39` was
  un-claimed at that wrap with this gap named; this entry claims it with the gap closed.
- watch: `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`'s margin, both cases red at the 7 s `WITHIN` of
  `tests/support/fake.rs`'s `wait_for` (7.535 s and 7.188 s) in a pre-push re-run under a host record of `CONTENDED`, green
  in the block's second firing and the next pre-push, about 4.3 s alone in five rounds; hypothesis: host contention,
  inferred from the two cases' place in the run, the nextest log holding no wall clock per test; no bound moved
  (`chunks/2026-10-31-sanitised-error-surfaces/evidence/operator-pass.md`;
  `chunks/2026-10-31-sanitised-error-surfaces/inputs/I3-wrap-sanitised-errors-directive.md.txt` item 4); retires: a
  recurrence, or 3 green runs (0/3; since 2026-10-31-sanitised-error-surfaces). Tallied on this chunk's CI reads
  (`inputs#I1` item 7).

## The contract this chunk moves

- [inferred] Today every exit-21 cause prints one document, `{"v":1,"error":"instance-unreachable","detail":null}`, MCP
  returns `{"error":"instance-unreachable","detail":null}` with text `refused: instance-unreachable`, and a strict-modes
  refusal's rule writes the same `detail: null` (`.claude/rules/security.md`). `v1-18`'s requirement asks a distinct code
  per cause. Carrying the code in `detail` or in `error`, and what a client reads from it, is a contract choice that
  amends the architecture, design-system, security-plan and test-plan text naming that document; P3 reads the seams and
  P4 raises the choice.
- [inferred] A code word crossing a client is read through `viola_core::is_code_word` and `viola_channel::sanitised_fault`
  (chunk 2026-10-31-sanitised-error-surfaces), and every error body and writer is named in
  `tests/contract_error_bodies.rs`; new codes stay inside that census, and no code holds a path, a pid or upstream text.
- [inferred] Whether a distinct code tells a same-user process more than `detail: null` does today, against the
  security-plan's error rules, is read in P3. A boundary widening is the founder's live word, asked as a P4 dialog
  (`inputs#I1` item 5).
- [inferred] `viola list`'s refusal and `viola plugin install`'s exit 21 print the `instance-unreachable` document too, and
  `viola mcp`'s refused home check is exit 1 with nothing on stdout or stderr; each joins the table or is named outside it
  by a reason.

## Where it reaches (premises for P3, all [inferred])

- New: `tests/cross_exit_causes.rs`.
- Codes and documents: `crates/viola-core/src/lib.rs`, `src/cmd/client.rs` (`refused_document`, `refuse_untrusted`, the
  liveness pre-check, the server check), `src/cmd/send.rs`, `src/cmd/wait.rs`, `src/cmd/last.rs`, `src/cmd/answer.rs`,
  `src/cmd/pause.rs`, `src/cmd/release.rs`, `src/cmd/link.rs`, `src/cmd/unlink.rs`, `src/cmd/list.rs`, `src/cmd/plugin.rs`,
  `src/cmd/mcp.rs`.
- The start refusals: `src/cmd/run.rs` (its start refusals, `checked_exe` / `refuse_tree`, the collision check),
  `src/cmd/revive.rs` (the document after a passed preflight), `src/cmd/verify.rs` (its refused home tree).
- Hints: `src/human/hints.rs` (the unwrapped-name hint, and each cause's own), `src/human/mirror.rs`.
- MCP: `crates/viola-mcp/src/result.rs`, `crates/viola-mcp/src/tools.rs`.
- The channel's mid-call reading: `crates/viola-channel/src/client.rs` (the vanished server read as
  `instance-unreachable`).
- The log's closed list: `schemas/diag-line.v1.json`, the `process-exit` `detail` values.
- Tests whose assertions the moved document reaches: `tests/cli_machine_contract.rs`, `tests/cli_revive.rs`,
  `tests/cli_list.rs`, `tests/cli_links.rs`, `tests/cli_verify.rs`, `tests/cli_plugin_install.rs`,
  `tests/security_negatives_strict_modes.rs`, `tests/security_negatives_server_identity.rs`,
  `tests/contract_error_bodies.rs`, `tests/cli_error_bodies.rs`, `crates/viola-mcp/tests/handlers.rs`,
  `crates/viola-e2e/tests/mcp_tools.rs`.
- The security-control table (the CARRY above) in `.andromeda/test-plan.md` §6's scenarios: `viola list`'s row.

## Boundaries

- [inferred] No new exit code: the typed set stays 0, 1, 2, 10–14, 20, 21. Causes are told apart inside exits 21 and 1.
- [inferred] No exit-code change, and no refusal widened, moved or made to write a line it writes none of today, unless P4
  names it: the checks that run before a log opens stay before it.
- [inferred] `hook` keeps its contract: exit 0, no body, no stderr, its `hook-decision` detail its only cause record.
- [inferred] The macOS server check stays the recorded gap (route entry "Unix endpoint and home hardening"), and the socket
  directory's verified 0700 is that entry's: this chunk gives its refusal a code only if the code holds that refusal today.
- [inferred] The other exits' causes (10–14, 20) are not re-opened: their reasons and details are already distinct.
- No `viola release` hint on any row: driver-facing hints never name it (test-plan's matrix).
- The frontend stays deferred to 0.2.0: `v1-18`'s `ui` start refusal, if the code holds none, is named out by that ruling.

## Capabilities

- `verification-matrix.json#v1-18` (every exit-21 and exit-1 cause has a machine-readable code) and
  `verification-matrix.json#v1-39` (the CLI contract, its gap above) are this entry's by their notes. P4 reads each cap's
  requirement and the route lines naming it before the lean.

## The CI read (Setup 5a)

- `16ef6b48` (the last wrap's flip): green · checks 15/15 · wall 797 s · ci#41198134788. The WATCH above is the tally's
  first reading: that run is the wrap commit's, the WATCH's own origin chunk.
- `df4ca577` and `404f7496`: not on the remote, noted — the spine door's two local commits, which go out with this chunk's
  normal pushes (`inputs#I1` item 9).

## The operator's directive (`inputs#I1`)

- The sampler is forced on this phase and on this chunk's wrap (item 1).
- Classifiers at effort high with three draws, read from the batch's check line and `pick`'s trail and said on the P5 card;
  anything reading `max` or fewer draws stops the phase and is reported (item 2).
- Every subagent and Workflow transcript is kept and the run dir left intact (item 3).
- A dead draw takes the contract's one re-spawn; a second death on the same agent, any other hard cascade failure, or
  agents doing the orchestrator's work (F352) stops the phase, reported with the run dir (item 4).
- A boundary widening, a live-session cap or a route split is the founder's live word, asked as a P4 dialog (item 5).
- No epoch boundary inside Epoch 6; `df -h /tmp` is read before a stamped-home gate (item 6).
- The spine lives in the store (`.andromeda/db/`); nothing edits a rendered route, matrix or `coverage.md` by hand, and
  anything the store changes under the skill is reported at once (item 8).
