# Outcome — 2026-11-17-standing-live-rig · run 2026-11-17T07-20-47Z-implement

**Outcome: green.** This run re-entered plan rev 8 (revision 1). It wrote the revision's code (steps 5, 6, 7, 11),
re-fired entries 1–9 green, then ran the live round: R1 whole, step 12's alignment of the fake to R1's trust form with
entries 1–9 re-fired green, then the plain build and R2 whole. Live starts: **2 of 2, spare unspent**. No red moved a
plan entry. v2-32 now reads `implemented`. Next: the OPERATOR PASS (entries 14 pre-push, 15 hygiene, 16 push, 17
`ci.py conclusion`), then the wrap.

## Files changed (this run)
- `crates/viola-e2e/src/rig/screen.rs`: `TrustPlace` (`Below(k)` · `Above` · `Unread`), `trust_place(rows)` (the
  option rows from the selector down to the trust row), `Model::trust_place`.
- `crates/viola-e2e/src/rig/keys.rs`: `trust_moves(place)` (`<k>` · `needs-up` · `needs-third-move` · `unread`), the
  `TrustPty` seam, `pass_trust` (moves read first, ≤2 downs each followed by a settle and a read, Enter only on a read
  `trust-yes`, no key at all otherwise), and 5 unit cases (A13).
- `crates/viola-e2e/src/rig/mod.rs`: `r1_keys` through `pass_trust` (a `TrustDriver` over the session), the summary
  `trust-moves` word, `version_token` (the `cli` word is the `x.y.z` token the binary printed, compared equal to
  `--cli`), 1 unit case; the `MAX_DOWN` import dropped.
- `crates/viola-e2e/src/bin/viola-live-rig.rs`: R2's child program is the install root's `claude` by full path.
- Step 12: `src/bin/viola-fake-agent.rs` (`TRUST_OPTIONS = ["No, exit", "Yes, I trust this folder"]`, `TRUST_OPTION =
  1`, rows `{glyph} {option}`); `crates/viola-e2e/tests/rig_rehearsal.rs` A1 re-pinned to that form.
- Earlier in the chunk (run `2026-11-17T06-49-10Z-implement`, kept): the rig's modules, its entry, the rehearsal
  binary, `crates/viola-e2e/Cargo.toml`, `crates/viola-e2e/src/lib.rs`, `src/bin/viola-fake-agent/input.rs`, `Cargo.lock`.
- `gate.py scope`: `scope: clean — changed 13 · listed 12 · recorded 1 (companion 0 · mechanical 1 · in-intent 0 · widening 0) · absorbed 6 · excluded 245`.

## Gates (trail `.andromeda/runs/2026-11-17T07-20-47Z-implement/gate-2026-11-17-standing-live-rig.json`)
- Firing 1, block: `entries 17 · green 9 · red 0 · recorded 0 · timeout 0 · not-run 8` (unit 23 → 29 rig cases).
- Round 1, `--live-legs --entry 11`: `round: COMPLETE · legs fired 1/1 · entries 17 · green 2 · …` (10 probe green, 11
  R1 green) — `evidence/round-072353Z.txt`.
- Firing 2, block after step 12: `entries 17 · green 9 · red 0 · recorded 0 · timeout 0 · not-run 8`, rehearsal
  `"passed":9`; clippy re-checked `viola-e2e` and `viola` (log 2.2).
- Round 2, `--live-legs --entry 13`: `round: COMPLETE · legs fired 1/1 · …` (12 build green, printed
  `d1fb7a4551aecb00`; 13 R2 green) — `evidence/round-072557Z.txt`.
- Controls: none re-fired. No revision-1 edit touched a guarded line (`procs.rs` `sweep`, `isolate.rs` removal and
  token comparison, `keys.rs` grant check, `input.rs` `0x7f` arm all unchanged). The five holds of
  `2026-11-17T06-49-10Z-implement/control-1.toml` stand.
- No gate deferred.

## Live starts (`evidence/live-starts.md`, `evidence/live-starts.ndjson`)
- R1, clock read 07:23:53Z, ledgered 07:23:59Z: `cli: 2.1.325`, `trust: shown`, `selected-first: other-row`,
  `trust-moves: 1`, `keys: down 1, enter 1, x 1, del 1`, `options: 2`, `box-text: empty` (after-x `holds-text`,
  after-del's frame equal to after-enter's), `census: 0`, `credentials: removed`, `scratch: absent`, `user-config: unchanged`.
- R2, clock read 07:25:57Z, ledgered 07:26:06Z: `cli: 2.1.325`, `viola-build: d1fb7a4551aecb00` (equal to entry 12's
  key), `send-1: equal` (leading spaces), `send-2: trailing-trimmed` (trailing spaces + TAB), `send-3: equal` (inner run
  + LF), every reply `ok`, `keys: none`, `census: 0`, `credentials: removed`, `scratch: absent`, `user-config: unchanged`.
- Cap: starts 2 of 2 + spare unspent. No token-window refusal.

## Deviations from the plan, with justifications
1. **`trust-moves: unread`, a fourth word.** Step 5 rev 1 closes the word at `<k>` · `needs-up` · `needs-third-move`.
   A trust frame with no selector row (or no trust row) has no place to read. It is built as `unread`: no key typed,
   and the start reads `trust-unpassed`, the same no-key path as the other two.
2. **The step-12 comparison rests on the rehearsal cases, not a by-hand fake summary.** A by-hand `rehearse r1` for
   a side-by-side summary was refused by the permission classifier. Entry 4's green A1/A2 assertions pin the fake's
   `trust`, `selected-first`, `trust-moves`, `selected`, `options` and `box-text` words, and they equal R1's
   (`live-starts.md` table).
3. **The `selected` summary word** after the trust pass is written as `trust-yes` by `r1_keys` once `pass_trust`
   returns: `pass_trust` returns Ok only after Enter typed on a frame read `trust-yes`.
- Scope record: `` `Cargo.lock` · mechanical · serves `crates/viola-e2e/Cargo.toml` · self `` (the stopped run's line, unchanged).

## Observed, not acted on
- R1's kept trust frame (gitignored `target/live-rig/104770-1794900238/screens/001-start.txt`) holds the scratch cwd
  path `/tmp/viola-rig-1000/<run>/cwd` that the CLI draws. It holds no user name or home path, and A6 reads the
  rehearsal's frames only. The kept screens are local by decision (security-plan · decision/live-session-screens-kept-local).
- R2's first frame read `box-text: holds-text` with the box up and nothing typed (R1's empty box read `empty`). It
  touches no R2 reading and was not investigated.

## Operator pass, before the pre-CI commit (rewritten after the first print)
- Entry 14 `bash scripts/agent-run.sh pre-push`, firing 1: **red** · exit 1, 2 of 3484 failed, both this chunk's own and
  outside the plan's block (`evidence/operator-14-firing-1.txt`):
  - `cli_verify runs::verify_record_screens_keep_only_signature_rows`: the fake drew its compiled trust rows in place
    of the set's recorded `Screen.modal`;
  - `contract_lints test_deadlines_sit_below_the_nextest_kill_line`: `rig_rehearsal.rs` `WITHIN 60s vs kill 30s`.
- Fixed in listed files:
  - `src/bin/viola-fake-agent.rs` `draw_trust(opts, selected)` draws over the recorded modal: the trust row as
    recorded, `❯ No, exit` directly above at its text column;
  - `crates/viola-e2e/tests/rig_rehearsal.rs` `WITHIN` 18 s, the rig `--settle 6 --deadline 15`.
- Re-read: block firing 3 `entries 17 · green 9 · red 0`; `cli_verify` + `contract_lints` 42 passed; entry 14 firing 2
  **green**, 3484 passed, coverage gate `"breaches":[]` (`evidence/operator-14-firing-2.txt`).
- No plan entry moved, so the run's stop rule did not fire. Report rewritten (rev 2) over `new-text-2.md`.

## Surfaced
- No spec↔reality gap.

## Process hygiene
`gate.py census --comm viola-live-rig --comm viola-fake-agent --comm viola --comm claude --args projects/viola/target/debug/viola --args viola-rig` → `census: 16 running`:

| process | started by | final state |
|---|---|---|
| `viola-live-rig` R1 and R2, their `claude` / `viola run` children | a leg (rounds 1 and 2) | terminated — each rig's own teardown read `census: 0`; the round's census held no survivor; `gate.py census` lists none |
| `viola-fake-agent` and `viola-live-rig rehearse` (entry 4, twice) | this run (gate entries) | terminated — none listed |
| `cargo build --bin viola` (entry 12) | a leg (round step) | terminated |
| 16 listed rows: 3 `viola wait`, 5 `viola run` / `claude` builder pairs of other projects, 4 overseer `claude` sessions, 1 `~/.viola/bin/0.1.0-…/viola mcp` | NOT DERIVED — the host's own sessions, not this run's | left running: not this run's |

The by-hand rehearsal script `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/3de5294f-…/scratchpad/rehearse_r1.sh` never ran (refused).

## Decisions & corrections
- The operator's rules for this run (`/andromeda-implement` args): the cap 2 + 1 spare, each start ledgered with the
  clock time before it is made, no timer, end every process, «A red that moves a plan entry stops you BEFORE the
  operator pass». Applied: both clock reads made before each round, no red moved an entry, no wake set.
- A permission-classifier refusal (verdict `[Undermining Oversight]`) of a by-hand fake rehearsal script: not retried
  and not routed around by another tool. The rehearsal cases' assertions served instead (deviation 2).
- An evolve append estimated its `ts` ahead of the clock and was refused (`REFUSED ts: … later than the clock`). It
  was re-fired from `date -u`.
- Two bash-guard refusals at Setup (`cd` into a skill dir; a pipeline tool's output piped into `grep`), each re-issued once.

## Outcome basis
This run's own window, and the stopped run's `2026-11-17T06-49-10Z-implement/outcome-2026-11-17-standing-live-rig.md`
(its deviations absorbed by plan rev 8, phase run `2026-11-17T07-09-35Z-phase`).
