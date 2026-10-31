# Scope — Exit-cause code catalogue

**Chunk:** `2026-10-31-exit-cause-code-catalogue` · viola-0.1.0 · Epoch 6 — Security hardening (its last entry)

**The working entry:** Exit-cause code catalogue — every exit-21 and exit-1 cause a distinct code across --json, MCP and
process log, matching its human hint.

## What it builds

- Every cause of exit 21 (`instance-unreachable`) and of exit 1 reaches `--json`, MCP and the process log as its own
  distinct closed code, carrying no path and no pid, and each code matches the human `hint:` line of the same cause,
  on each surface the cause reaches. A cause a surface cannot reach, or one a founder ruling keeps off it, is named in
  the plan with that reason (below).
- [premise-corrected: the code holds more causes than `verification-matrix.json#v1-18` lists; the list below is read
  from `src/cmd/` at HEAD (research.md §Cause list)] The exit-21 causes: strict modes · not running (no live endpoint,
  or a failed connect) · an unwrapped name · an endpoint lost mid-call · server verification. The exit-1 causes: already
  live · stale heartbeat · squatted name · pinned-hash mismatch · script child · strict modes (the home tree, at `run`,
  `revive`'s start, `verify` and `update_stamps`) · internal error · `revive`'s `no-session` (two pairs), `session-live`
  and `cwd-missing` · `verify`'s CLI not found, CLI version unreadable, unclean recording and interrupted run.
- [premise-corrected: the code holds no socket-directory refusal: `bind_endpoint`'s `create_private_dir` failure is an
  anyhow error at the catch site, `src/cmd/run.rs:531`, and the verified directory is the route entry "Unix endpoint and
  home hardening"'s] `v1-18`'s socket-directory cause takes no code here; its refusal is that entry's to build. The
  frontend's deferral names `v1-18`'s `ui` start refusal out: `viola ui` is no verb in `src/cmd/mod.rs`.
- The exit-cause table `tests/cross_exit_causes.rs` (test-plan's Exit-cause matrix, an rstest `#[case]` table, one
  negative test per cause): each case asserts the exit code and either, in `--json` mode, the document with stderr
  empty, or, in human mode, a last stderr `hint:` line naming that cause; a hint shared across causes fails the table.
  [premise-corrected: one invocation cannot hold both, `--json` printing no hint, design-system §Surface: cli, Streams]
  The file does not exist at HEAD (`wc`: no such file). Each cause outside it is named by the binary that holds it, or
  by its reason.
- A cause the code holds and no process-log line carries today gets its log form, or its absence is a named decision:
  the strict-modes refusals made before a log opens write no line by Founder Direction 4 (obs-plan §1) and his own
  answers (the masters' `2026-10-20-t06-58-31-wrap` entries).

## The freight, folded

- CARRY (chunk 2026-10-10-viola-revive, the founder's ruling of 2026-10-01 on that entry: each revive refusal has an exit
  code and a hint that join this catalogue): `viola revive` landed four exit-1 causes with their closed details,
  `strict-modes-failed` · `already-live` · `no-session` (two stderr pairs) · `cwd-missing`, each pair with its own hint,
  tested in root `tests/cli_revive.rs` and not in the `cross_exit_causes.rs` table (test-plan §6 Exit-cause matrix).
  `session-live` is not built (the entry "The board: viola list"). Their `--json` documents landed with chunk
  2026-10-17-cli-machine-contract (the CARRY below).
- [premise-corrected: `session-live` is built, `src/cmd/revive.rs:61`, `:91`, `:211`, and `schemas/diag-line.v1.json:78`
  holds the word; the CARRY's sentence is stale] `session-live` joins this catalogue as the fifth `revive` code.
- CARRY (chunk 2026-10-10-budget-governor, pinned by its wrap as a fact this catalogue's revive rows must hold):
  `revive`'s strict-modes refusal now writes no `process-exit` line, because its check runs before the wrapper log opens.
  Of the four exit-1 causes three carry a logged `detail` (`already-live`, `no-session`, `cwd-missing`); the strict-modes
  one is exit 1 and its stderr pair alone, and `Refusal::StrictModes.detail()` still returns `strict-modes-failed` with no
  line carrying it.
- That CARRY's causal claim, re-derived at HEAD: `src/cmd/revive.rs:121` refuses ahead of `open_wrapper_log` at `:136`,
  and `Refusal::StrictModes.detail()` stands at `:70`, where the `--json` document reads it (`:107`); no line carries it.
  The mechanism's reason, that opening a log narrows an existing home, is spot-checked at `src/cmd/client.rs:20`'s comment
  and `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (evidence
  `viola-0.1.0/chunks/2026-10-26-explicit-session-links/evidence/remove-the-guard.md`).
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

- Today every exit-21 cause prints one document, `{"v":1,"error":"instance-unreachable","detail":null}`
  (`src/cmd/client.rs:35`, `:218`, `src/cmd/list.rs:92`, `src/cmd/plugin.rs:54`), MCP returns
  `{"error":"instance-unreachable","detail":null}` with text `refused: instance-unreachable`
  (`crates/viola-mcp/src/result.rs:30`), and a strict-modes refusal's rule writes the same `detail: null`
  (`.claude/rules/security.md`). `v1-18`'s requirement asks a distinct code per cause. The carrier, what a client reads from
  it, and whether it reaches a driver over MCP are the P4 fork: a boundary widening is the founder's live word
  (`inputs#I1` item 5), and the masters' text naming the document moves with his answer.
- A code word crossing a client is read through `viola_core::is_code_word` and `viola_channel::sanitised_fault`, and every
  error body and writer is named in `tests/contract_error_bodies.rs`; new codes stay inside that census, and no code holds a
  path, a pid or upstream text. These documents are the client's own: no exit-21 or exit-1 code crosses a channel reply.
- `viola list`'s refusal and `viola plugin install`'s exit 21 print the `instance-unreachable` document too, and join the
  table. `viola mcp`'s refused home check and its line past `MAX_FRAME` before `initialize` are exit 1 with both streams
  empty: named outside the table by that reason, stdout being MCP's frames (design-system §Platform-Specific Notes), each
  held by its own witness.

## Where it reaches

- New: `tests/cross_exit_causes.rs`, with its `[[test]]` table in `Cargo.toml` and its row on `.config/nextest.toml`'s
  verify-driven kill line.
- Codes and documents: `crates/viola-core/src/lib.rs`, `crates/viola-state/src/liveness.rs`, `src/main.rs` and
  `src/cmd/mod.rs` (the catch site's `--json` form), `tests/support/start.rs` (the start setups the table shares),
  `src/cmd/client.rs` (`refuse_untrusted`, `Unreachable`, `answer_of`, `unreachable`), `src/cmd/send.rs`,
  `src/cmd/wait.rs`, `src/cmd/last.rs`, `src/cmd/answer.rs`, `src/cmd/pause.rs`, `src/cmd/link.rs`, `src/cmd/list.rs`,
  `src/cmd/plugin.rs`.
- The start refusals: `src/cmd/run.rs` (`start`, `collision_check`, the refuse helpers, `Launch`), `src/cmd/revive.rs` (the
  document after a passed preflight), `src/cmd/verify.rs` (the refusals' log codes).
- Hints: `src/human/hints.rs` (the unwrapped-name hint and the endpoint-lost hint).
- MCP: `crates/viola-mcp/src/result.rs`, `crates/viola-mcp/src/tools.rs`.
- The log's closed list: `schemas/diag-line.v1.json`, the `process-exit` `detail` values.
- Tests whose assertions the moved document or code reaches: `tests/chaos_wait_vanish.rs`, `tests/cli_send.rs`,
  `tests/cli_list.rs`, `tests/cli_machine_contract.rs`, `tests/cli_wait_last.rs`, `tests/cli_controls_not_disableable.rs`,
  `tests/cli_plugin_install.rs`, `tests/cli_links.rs`, `tests/cli_error_bodies.rs`, `tests/cli_instance_state.rs`,
  `tests/cli_revive.rs`, `tests/cli_verify.rs`, `tests/security_negatives_server_identity.rs`,
  `tests/security_negatives_strict_modes.rs`, `tests/contract_error_bodies.rs`, `crates/viola-mcp/tests/handlers.rs`,
  `crates/viola-e2e/tests/mcp_tools.rs`.
- [premise-corrected: `src/cmd/unlink.rs` and `src/cmd/release.rs` reach every cause through `link_call` and `wheel_call`,
  and `src/human/mirror.rs` and `crates/viola-channel/src/client.rs` hold the word `instance-unreachable` only as the human
  column and the `wait` `outcome`, research.md §Sweeps] These four take no edit.
- The security-control table (the CARRY above) in `.andromeda/test-plan.md` §6: `viola list`'s row, a wrap amendment.

## Boundaries

- No new exit code: the typed set stays 0, 1, 2, 10–14, 20, 21. Causes are told apart inside exits 21 and 1.
- No exit-code change, and no refusal widened, moved or made to write a line it writes none of today: the checks that run
  before a log opens stay before it. `verify`'s `update_stamps` refusal already ends in its own `process-exit`, which gains
  its code.
- `hook` keeps its contract: exit 0, no body, no stderr, its `hook-decision` detail its only cause record. No code of this
  chunk reaches it; `strict-modes-failed` on `sideload_fallback`, `parse-rejected{ledger-stamps}` and `hook-decision` stays a
  different use of the word and no exit cause.
- The macOS server check stays the recorded gap (route entry "Unix endpoint and home hardening"), and the socket directory's
  verified 0700 is that entry's.
- The other exits' causes (10–14, 20) are not re-opened: their reasons and details are already distinct. The exit-2 lines
  stay named in `NOT_ERROR_BODIES`.
- No `viola release` hint on any row: driver-facing hints never name it (test-plan's matrix).
- The frontend stays deferred to 0.2.0.
- A `verify` failing row is an outcome and no refusal: it takes no code (design-system's `2026-09-28` entry). A failed
  `claude agents --json` read gains no cause, and a ConPTY companion's refusal stays a recorded fallback (`(sampler)`).

## Capabilities

- `verification-matrix.json#v1-18` (every exit-21 and exit-1 cause has a machine-readable code) and
  `verification-matrix.json#v1-39` (the CLI contract, its gap above) are this entry's by their notes. No other markerless
  entry names either (`command grep -n 'v1-18\|v1-39' viola-0.1.0/working-route.md`: this entry's line alone).

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
