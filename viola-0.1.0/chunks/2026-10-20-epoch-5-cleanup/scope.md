# Scope — 2026-10-20-epoch-5-cleanup

**Working entry:** "Epoch 5 cleanup — the debts the Epoch 5 boundary's audit and diagnosis named, each closed so the
next boundary's audit reads it closed" (`viola-0.1.0/working-route.md`, under `### Epoch 6 — Security hardening`).

**Intent.** Close the ten debts the Epoch 5 boundary named — the code audit
(`.andromeda/runs/2026-10-19T21-08-30-code-audit/proposals.md`) and the evolve diagnosis
(`.andromeda/runs/2026-10-19T20-57-02-evolve-diagnose/proposals.md`) — so the next boundary's audit reads each one
closed. The freight is the Viola overseer's placement as the founder's delegate, PROVISIONAL until his word, except
items (8) and (10), which are his own word (`.andromeda/runs/2026-10-20T06-58-31-wrap/inputs/I1-batch-e5-directive.md.txt`
§4; `.andromeda/runs/2026-10-19T23-02-00-wrap/inputs/I1-boundary-wrap-e5-directive.md.txt`). Its place directly after
"Newer claude CLI rows" is his own word, «Оставить после»
(`viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/inputs/I6-relay-1.md.txt`).

## Standing constraints (the phase directive, inputs#I1)
- No mutation run, no witness run and no workflow dispatch inside this chunk — the founder's standing direction
  (inputs#I1 §1, restating the entry's own clause). Every mutation-related item is worded by what the NEXT boundary's
  audit will read (CARRY 2's ungraded root unit, the Windows baselines, the 16 survivors), never by a mutation result
  this chunk must produce. [verified: inputs#I1 §1; the entry's clause]
- A guard test keeps its remove-the-guard demonstration as a one-off reading, made over a scratch clone where the
  classifier refuses the in-tree edit. [verified: inputs#I1 §1; testing.md 2026-09-25, extended 2026-10-20
  (`--project-root {clone}`)]
- The chunk is sized against one window first; ten items that do not fit get a split proposed at P4 with a recommended
  partition. The order stands: this entry comes right after "Newer claude CLI rows". [verified: inputs#I1 §2; research
  §Sizing reads the ten items past one window — the split is research's first Open question]
- `df -h /tmp` is read before any gate that boots stamped homes: `/tmp` is a 32 GB tmpfs with a per-user quota that
  tripped twice on 2026-10-19/20, most of it other live sessions' task output, which is not this chunk's to remove.
  [verified: inputs#I1 §4; `findmnt -T /tmp` reads `tmpfs … usrquota`, size 32 868 264 KiB]
- Two letters newer than 07:56Z bind the plan: a preservation guard that lets recorded companions through is ONE
  `gate.py guard` call; an OS-sensitive diff names its other-OS reading in the `ci.py` entry's note. [verified:
  inputs#I1 §5]

## The split (inputs#I2, the founder's own word, P4)
- This chunk builds items (1), (2), (3), (6), (8), (9), (10). Items (4), (5), (7) leave it for a new working-route entry
  «Epoch 5 cleanup II», directly after this one; the wrap writes that insertion on his word («Два чанка», live,
  2026-10-20T08:32Z). Their bullets below stay as folded, read only to size them.

## What it builds — the ten items
1. **The root package's unmutated baseline passes the nextest `mutants` profile's 10 s kill.**
   - Measured by the audit at HEAD b6ddc198: `contract_fake_agent_drift
     fake_agent_dialog_replay_matches_every_recorded_dialog_set` ran 10.577 s unkilled; `cli_send
     send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` and `…_on_a_verified_cli::case_1_hint` ran
     9.0–9.3 s; so neither CARRY 2 nor the root `src/` code of eight Epoch 5 chunks was graded. [verified: the audit's
     pointer stands (`proposals.md` §Informational, root unit) — premise-corrected at HEAD: the replay case walks three
     dialog sets now, `2.1.302`'s landed at `4eaca1d3` after `b6ddc198`, so 10.577 s is a floor, not today's wall]
   - The direction names a profile override like `verify_window_` / `send_window_`, or a faster replay. CARRY 2's
     grading moves to the next boundary (the chunk makes the baseline pass; it runs no mutation).
   - Re-verified: the three tests exist at HEAD (`tests/contract_fake_agent_drift.rs:218`, `tests/cli_send.rs:964`).
2. **`verify_window_without_screens_fails_every_interactive_row` has headroom under its 45 s kill.**
   - Measured 42.95 s. [verified: the audit's §Informational line, 42.952 s alone, 43.635 s in the baseline; the case
     at `tests/cli_verify.rs:1251`]
3. **The Windows mutation leg's baselines pass.**
   - Run 39326811932: `viola-e2e` `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not`
     fails `Ok(())` against `Err(Writable)`; `cli_budget path6_the_boundaries_and_the_window_word` hits the 10 s kill;
     `viola-pty` built 78 unviable mutants against 11 caught. [verified against the recorded run: the audit's dispatch
     table; runner-only — not reproducible on this host by construction, and the chunk dispatches nothing]
   - Re-verified: `crates/viola-e2e/src/harness/run/mutants/scratch.rs:258`, `tests/cli_budget.rs:191` exist.
   - With no mutation run or dispatch in the chunk, "pass" is worded as what the next boundary's Windows leg will read;
     the chunk's own proof is local. [verified: inputs#I1 §1]
4. **Each of the audit's 16 Informational survivors is killed or recorded.**
   - [inferred] Hypothesis (the entry's words, kept verbatim): "the three `viola-mcp` ones are killed only by
     `crates/viola-e2e/tests/mcp_tools.rs`, outside the package run — not measured". Re-verified: that file exists.
     Not re-derivable at P3: a hand-applied mutant is implement's, and the chunk runs no mutation.
5. **Size.**
   - `crates/viola-agent-claude/src/ledger.rs` stands without its in-file test module. [verified: the module opens at
     `:1518` of 4275 lines, 2758 test lines]
   - `src/cmd/verify/typed.rs` and `src/human/board.rs` stand split by concern. [research: `board.rs` is 313 product
     lines and 753 test lines; `typed.rs` 894 and 534 — their size is mostly test mass too]
   - `viola-mcp` → `serde` is no unused-dependency candidate (`crates/viola-mcp/Cargo.toml:21`
     `serde.workspace = true`).
   - [premise-corrected at the fold: `wc -l` at HEAD d35644a5 reads `ledger.rs` 4275, `typed.rs` 1428, `board.rs`
     1065 lines — the entry's 3732 / 1199 / 949 were the audit's reading at b6ddc198.]
6. **`tests/support/home.rs` `dialogless_home` derives its summary from `LedgerRow::relied()`, never from a
   literal.** "red only at pre-push in three chunks" [verified: the diagnosis P6 rows — resume-and-compact, unwrapped,
   compact]. Re-verified: `dialogless_home` at `tests/support/home.rs:405`, one caller (`tests/cli_answer.rs:793`); the
   test's own oracle stays the literal in `tests/cli_verify.rs:1315` / `:1317`.
7. **An E2E contract case replays a recorded live `tools/list` exchange**, so the gate sees the class only the live
   start saw (`ttlMs` / `cacheScope`). [verified: `live-mcp.md` Start 1, the rmcp client accepted what the CLI refused —
   research: no recording of the exchange exists ("read, not copied"); one needs a new live start]
8. **The chronic truncated `viola hook` `.profraw` at the coverage merge has its cause found** — who kills the
   profile writer (seen since Epoch 3). The founder's own word, «Искать причину»
   (`viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/inputs/I10-relay-2.md.txt`). The WATCH below stands until
   this item owns it.
   - [inferred] hypothesis (research's, unmeasured): the fake agent spawns each hook in its own process group
     (`src/bin/viola-fake-agent.rs:631`), so a hangup of its terminal — a test killing the wrapper, whose PTY then
     closes — reaches an in-flight `viola hook` mid its exit-time profile write; the fake agent's stdout holder already
     leaves that group for exactly this SIGHUP (`:1018`). The recorded file is a `viola hook` cut off after
     `Client::notify` (`viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/profraw-red.md`). A cause
     reading is two-sided, at implement.
9. **The gates that boot stamped homes read the `/tmp` usrquota tmpfs's headroom first.**
   - "each parallel test home pins a ~100 MB debug `viola`" [verified: `target/debug/viola` 101 163 992 B; 32 test
     threads on this host]; "the switch wrap's light gate went red on EDQUOT, measured at
     `.andromeda/runs/2026-10-19T19-16-36-wrap`" [verified: its `relay-3.md`, 24 G used of 32 G at the red, 13 G after
     the founder's cleanup].
   - [premise-corrected: the per-user quota is unreadable from the session's shell (`quotactl_fd` → EPERM), and `df` /
     `statvfs` read the filesystem (14 134 MiB free now), not the quota; EDQUOT struck at about 8 G free by `df`. A
     headroom read is a filesystem-free floor set from that reading, not a quota read]
10. **The restructure's web leftovers**, as the next audit and a reader of the masters see them at `0807ee3c` — the
    founder's own word, «Вычистить в cleanup» (`.andromeda/runs/2026-10-20T06-58-31-wrap/inputs/I1-batch-e5-directive.md.txt`):
    - design-system's Brand Identity still cites cut motion values and cut page components (its lines 16, 21, 41,
      45 — re-verified at HEAD: `<viola-atis>`, `<viola-event-feed>`, the `web-spa (viola ui)` motion row, the
      single-source motion paragraph);
    - a11y-plan's `Standard (1)` tier rests on the terminal surfaces alone (`.andromeda/a11y-plan.md:18–19`);
    - the web-only `.claude/rules/frontend.md`, `.claude/rules/a11y.md` and the `.claude/docs/services/viola-ui.md`
      stub stand (re-verified present; `a11y.md`'s `paths:` name only web files, so it loads on no file of the tree);
    - [premise-corrected: no `reserved` mark stands in obs-plan §3's log-format copy — `grep -n -i 'reserved'` reads 0
      in both log-format key files; the mark went with 2026-10-16-remove-the-browser-leg (obs history). obs's standing
      web leftover is the §4 scenario heading `GUI view: page load, sessions and links, live SSE tape …`
      (`.andromeda/obs-plan.md:647`) over its deferral stub]
    - four rule files' `paths:` (`a11y.md`, `api.md`, `frontend.md`, `testing.md`) still name `e2e-web/**` /
      `crates/viola-ui/**` (re-verified: each names one);
    - `CLAUDE.md`'s services pointer still lists `viola-ui` (`:100`, inside `GENERATED:setup:deeper-topics`);
    - the old text stays in the sidecars for 0.2.0.
    - The masters are the spec sources: phase never amends them, so item (10)'s master-body edits and the `CLAUDE.md`
      generated block are the chunk's wrap (cascade) work; the rule and doc files are implement edits. [verified: the
      sidecar contract; `CLAUDE.md` markers]
    - Out of reach: the axum `http2` ban and the GUI scan forms stay the founder's separate item (arch, security and
      obs histories, 2026-10-20-t06-58-31-wrap).

## The second CARRY (PROVISIONAL)
- This entry's own wrap proposes the judgment-base growth: a `drift-base.md` design-system detector over the cli
  wording passages (As-built clauses, refusal pairs, output wording), and the recurring step-2 sweep classes
  (`cli_verify` count literals, retired surface names) moved into detector scope. It is a wrap act, so the plan names
  it as an expected wrap proposal, not an implement task. [verified: `drift-base.md` is wrap-owned]

## Watches (observations, no obligation)
- watch: the macOS readiness stall, `wrapper <name> not ready` after `process-start{subject:"self"}` only, inside
  `run.start` before the version gate, its phase not named; witness the stall report `wait_ready` writes into the
  panic; closed UNMET on the founder's «Закрыть + WATCH»
  (`viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/inputs/I9-relay-1.md.txt`) (0/3; since
  2026-10-19-newer-claude-cli-rows).
- watch: a `viola` process's truncated `.profraw` refused by `llvm-cov` at `pre-push`'s coverage merge (header
  corrupt), every test passing; recurred at the light gate of 2026-10-19-newer-claude-cli-rows's wrap (`32.log`, 2974
  of 2974 passed) (0/3; since 2026-10-19-newer-claude-cli-rows). Its cause is item (8). Recurred at P5's baseline run
  of `pre-push` on the untouched tree (2026-10-20T08:48Z, 2974 of 2974 passed, one 90 112 B `viola` profile refused;
  `.andromeda/runs/2026-10-20T07-58-07-phase/p5-prepush-red.md`): its retire condition «a recurrence» is met, and the
  red is item (8)'s subject.

## CI verdicts read at Setup (the last flip through HEAD)
- `0807ee3c` — green · checks 15/15 · ci#39383206773 · wall 603 s.
- `d35644a5` — CI d35644a5: verdict not yet available (in progress, ci#39389641659) — never read as green; no red to
  disposition.

## Boundaries
- No mutation run, witness run or workflow dispatch (above). No spec-master edit by phase.
- No product behaviour change: every item is test, harness, gate, file layout or documentation work; a product source
  split (item 5) moves code without changing it. [verified for items (6), (8), (9), (10): the fake agent is a
  test-only bin behind `fake-agent`, the keepers are test and harness code]
- Surfaces touched: `tests/` (support, contract, cli suites), `.config/nextest.toml` (the `mutants` profile),
  `crates/viola-e2e/` (the mutants scratch, the keeper, the MCP contract case), `crates/viola-agent-claude/src/ledger.rs`,
  `src/cmd/verify/typed.rs`, `src/human/board.rs`, `src/bin/viola-fake-agent.rs`, the stamped-home keepers, and the
  rule, doc and master files item (10) names. [research: the recommended partition reaches only `tests/support/`,
  `tests/cli_instance_state.rs`, `crates/viola-e2e/` (the keeper and its manifest), the fake agent, the census script
  and item (10)'s files]
