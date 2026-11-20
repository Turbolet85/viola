# Implement outcome — 2026-11-20-permissionrequest-question-answered

**Run:** 2026-11-20T12-55-00Z-implement (re-entry at plan step 16a, plan rev 5) · **Outcome: green** — `S1` live
green within the founder's cap, the record landed, every after-landing layer and the boot smoke green.
**Next → the operator pass** (entries 32 pre-push · 33 hygiene · the land and the pre-CI commit · 34 push · 35 the
`ci.py conclusion` read), run in this same window per the implement directive (`inputs#I9`) and the operator's
invocation (`inputs#I10`); then the wrap, a fresh window's.

## What this run did

- **16a:** `tests/support/verify.rs` — the three `question-permission-1` shapes named SYNTHETIC where built (comments
  only). Entries 7, 8 green.
- **17:** host read QUIET (2026-11-20T12:55:41Z, 0 s stalled).
- **18 the round** (fired 12:55:57Z): entries 12–18 green, `round: COMPLETE · legs fired 1/1`. **`S1` (PQ):**
  `verify-exit: 0`, `pass-count: 27/27`, `failing: none`, `permission-request-question: pass`, screens b–f `box`,
  `keys: none`, `kept-stamp: written`, `record: 30`, `record-left: absent`, `census: 0`, `credentials: removed`,
  `scratch: absent`, `user-config: changed-by-others` (a reading; `lacks changed-by-rig` held). Ledger: one base line
  `PQ` at 12:56:04Z; `PQ-2` unspent. `evidence/live-round.md`, `evidence/round-125557Z.txt`.
- **19:** census `operator` green (`census: 0`); after-reads 20–23 green: `27 ok` (browser-tools-prompt `absent`),
  `live-stamp: unmoved`, the name diff exactly the four lines, count `4`.
- **22 landing:** identity read clean; plain copy of the record over `fixtures/claude/2.1.328/` (28 → 31 files; the
  three Screen files byte-identical). Entry 24 `diff -r` green (no output); entry 25 recorded `permission_mode: auto`.
  `evidence/landing.md`.
- **23:** `STAMPED = ["2.1.328"]`, `DRIFT_ONLY = ["2.1.283", "2.1.287", "2.1.288", "2.1.302", "2.1.325"]`.
- **24 / §V:** `DEFAULT_CLI_VERSION` (harness, fake agent) and `RECORDED_CLI_VERSION` → `2.1.328`; every §V
  "runs on 2.1.328" site moved (ids read from the LANDED set: `FIXTURE_SESSION` `fc611e35-…`, clear-1
  `16e6d6ce-…`); `cli_answer.rs` re-pointed to `question-permission-1` / `plan-1` / `plan-2`; the two Path 4 scripts
  re-pointed.
- **25a:** the drift suite — `DIALOG_STEMS` gains `question-permission`; both replay tests expect exit 0 only on a
  set holding `PermissionRequest.question-permission-1.json`, else exit 1 with the 27th row's step line the one
  failing line and `26 pass  1 fail`.
- **25b:** the hygiene walk — the stem list gains `question-permission`; a set holding any such file must hold the
  twelve plus the three, every other dialog set exactly the twelve.
- **25 the layers:** first firing red (6 tests, every one a value the move or the 27th row moved — table in
  `evidence/after-landing.md`), one fix pass, then green: unit 2900/2900, integration 575/575, E2E 324/324.
- **Smoke** (operator entries 29–31): boot ok, status `ready` with both instances `cli_version 2.1.328`
  `cli_verified true`, cleanup `processes_gone` / `endpoint_gone`.
- **Matrix:** `v2-20` planned → implemented.

## Deviations (each with its scope-record line)

- **Six reds at the first after-landing firing, fixed in one pass** (`evidence/after-landing.md`):
  - `tests/support/home.rs` · companion · `DIALOG_ROWS` gains `PermissionRequestQuestion` (4 → 5): a stamp with no
    dialog replay fails the 27th row too (`stamped 2.1.328  22 pass  5 fail` read).
  - `tests/cli_verify.rs` · companion · `10 pass  16 fail` → `10 pass  17 fail`: a summary summing to 26 that the
    step 14 / 14a sweep missed (testing.md 2026-11-20 recurrence); its Run C path read: the new row fails beside the
    other interactive rows on a spine-only set. A re-sweep by the sum finds no other summary at 26.
  - `tests/cli_verify/stamps.rs` · listed (step 14) · the record oracle gains the three `question-permission-1`
    names (a record of the step 13b test-built set writes them).
  - `crates/viola-e2e/tests/rig_rehearsal.rs` · companion (step 14) · the VC and V rehearsals' screen-fact literals
    pinned the old default set's ready and trust screens; they now read the 2.1.328 screens the fake draws at the
    new default. §V named only `:1786` in this file.
  - `tests/contract_fake_agent_drift.rs` · in-intent (25a) · `dialog_replay_reads_each_call_s_events_in_order`'s
    oracle gains the three names in Run C order.
- **`tests/cli_answer.rs` module doc**: besides the 2.1.287 capture lines §V named, its line saying a
  PermissionRequest-raised question is left to the human (stale since Phase A) was rewritten to the held-and-answered
  reading.
- **§V line numbers:** four of `cli_answer.rs`'s six fixture sites had moved since P4; re-found by the `fixture(`
  helper, mapped by hook order (the fifth path4 step, the second plan PreToolUse, is `plan-2`).
- **Scope record (rev 5):** 18 lines added at this run — in-intent step 24 ×15 (`src/bin/viola-fake-agent.rs`,
  `src/bin/viola-fake-agent/test_cases_opts.rs`, `tests/support/fake.rs`, `tests/support/start.rs`,
  `tests/chaos_revive.rs`, `tests/cli_send.rs`, `tests/cli_list.rs`, `tests/cross_list_rows.rs`,
  `tests/tui_passthrough.rs`, `tests/run_cli.rs`, `tests/hook_events.rs`, `tests/cli_wheel.rs`,
  `crates/viola-e2e/tests/mcp_tools.rs`, `fixtures/fake-scripts/path4.json`,
  `fixtures/fake-scripts/path4-permission.json`), in-intent 25a `tests/contract_fake_agent_drift.rs`, in-intent 25b
  `tests/contract_fixture_hygiene.rs`, companion `tests/support/home.rs`. `gate.py scope`: `scope: clean — changed
  67 · listed 43 · recorded 24 (companion 4 · mechanical 0 · in-intent 20 · widening 0) · absorbed 29 · excluded
  242`.

## Surfaced / stuck

None.

## Process hygiene

| process | started by | final state |
|---|---|---|
| the round's entries 12–18 (the rig, its `viola verify`, up to 9 `claude` children one at a time) | a leg (gate.py `--live-legs`) | terminated — the rig's own `census: 0`, `scratch: absent`, `credentials: removed`; round summary named no survivor; entry 19 `census: 0` |
| the smoke session `question-smoke` (supervisor, two wrappers, two fake agents) | this run (operator entries 29–31) | terminated — cleanup `processes_gone: true`, `endpoint_gone: true`, `home_removed: true` |
| the gate entries' cargo / nextest trees | this run (through `gate.py`) | terminated (each call exited) |
| the 14 processes `gate.py census` lists (the overseer sessions, the prototype's builders and their `claude`, the pinned product's `viola wait` and `~/.viola`'s `viola mcp`) | NOT DERIVED — none under this repository's `target/` | left running: not this run's — their owners' |

## Decisions & corrections

- The implement directive (`inputs#I9`) and the operator's invocation (`inputs#I10`) bound this run; both snapped.
- Bash guard refusals: 2 — a `cd` into the skill's references dir (re-issued with absolute paths), and a `gate.py
  show` piped into `head` (re-issued as bare calls). Neither routed around.
- Sweep hazard (testing.md 2026-11-20, recurrence): the step 14 count sweep missed a `10 pass  16 fail` summary; the
  sum-by-26 sweep finds it. And a fake-agent default move reaches the rig rehearsals' screen-fact literals, which no
  version literal marks.
