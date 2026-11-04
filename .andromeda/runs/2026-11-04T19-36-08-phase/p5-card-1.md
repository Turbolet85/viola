# P5 review card 1 — 2026-11-04-linux-and-macos-parity: "Linux and macOS parity"

**The plan to read:** `.andromeda/cache/docs/2026-11-04-linux-and-macos-parity/plan.md` (rev 2, 433 lines), open it in an
editor. No plan stands in the chunk's folder: that file is the plan's working copy, and writing in it changes no record
until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-04-linux-and-macos-parity`
is a diff of the chunk's records.

## Goal
Make the fake-agent suite one suite on Linux and macOS beside Windows, as a fact the tree holds. P3 measured it nearly there
(ci#41339639367: ubuntu 3446, macOS 3400, Windows 3376 cases, 0 failed, 0 skipped; every one-OS case a per-OS twin or a
named carve-out) save two shared subjects read on one OS. No product source moves; nothing drives a live `claude`
(`inputs#I1` item 5).

## What it does (the plan's 9 steps in short; test code, support and one operator script only)
1. `Frozen` (the wrapper-freeze guard, both OS twins) moves to `tests/support/frozen.rs`; its users in
   `tests/cli_instance_state.rs` take it from there.
2. `run_child_ends_when_its_wrapper_is_terminated` runs on every OS: its terminate step per OS (SIGKILL of the wrapper on
   Unix, measured at P3 on the dev host: the fake-agent child gone at the first 100 ms read; `TerminateProcess` on Windows).
3. The `v1-40` CARRY's witness: under the outer PTY, `viola list` beside a frozen wrapper whose heartbeat is back-dated
   60 s reads exactly `[DIM, BOLD, RESET, DIM, RESET]` — the stale row dim end to end, NAME dim and bold, nothing else
   styled — on both Unix runners, with no 5 s wait (so it fits the `mutants` profile's 10 s kill).
4. `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` widened from Linux to Unix through `ps -o sigignore=`.
5. A reason line on the ten OS-gated tests that carry none, and a census contract, `tests/contract_os_gated_tests.rs`:
   every OS-gated test under `tests/` and `crates/*/tests/` named with its reason in a literal table, red on an unnamed gate.
6. `binary(run_cli)` joins the `ci` profile's 20 s kill line (26 → 27).
7. `scripts/junit-os-sets.py`: the operator pass records the pushed sha's per-OS JUnit counts and set differences, the
   reading `v1-53`'s later claim reads (`inputs#I3`).

## Top acceptance criteria
- The un-gated pin, the `v1-40` witness and the widened hangup case pass on `ubuntu-latest` and `macos-latest` (the pin on
  `windows-2025` too) on the pushed sha, and locally; the witness and the pin fit the `mutants` profile's kill.
- The census passes: every OS-gated test function and file a `GATES` row with a reason; its self-check reads a planted
  unnamed gate as a hit.
- `test`, `perf` and `lint` green on all three OSes on the final pushed sha; the per-OS JUnit reading recorded.
- The preservation guard reads `guard: clean`: no product source, schema, workflow, manifest or lockfile moves.

## Touchpoints
New: `tests/support/frozen.rs` · `tests/contract_os_gated_tests.rs` · `scripts/junit-os-sets.py`. Modified:
`tests/support/mod.rs` · `tests/cli_instance_state.rs` · `tests/cross_exit_causes.rs` · `tests/run_cli.rs` ·
`tests/cross_list_rows.rs` · `tests/cli_fake_agent.rs` · `tests/cli_list.rs` · `tests/cli_plugin_install.rs` ·
`tests/cli_verify.rs` · `.config/nextest.toml`.

## Test commands (24 entries)
`df -h /tmp` first; fmt, clippy (host and Windows-target check), G1, lint-probes, `cargo check`; `run --unit`; the
integration nextest over the six touched binaries; `run --e2e` over `cross_list_rows`, `cross_exit_causes`, the census and
`contract_bounded_readers`; the two new cases under the `mutants` profile; the root waits' count (32); the preservation
guard; the harness smoke (cleanup · boot · status · G2 · G4 · cleanup); `pre-push`; then the operator's hygiene, push, CI
read and per-OS JUnit reading. Baselines (10 new entries): green on 1, 9, 12, 13, 14, 15, 16, 19; the predicted reds on 10
(no `contract_os_gated_tests` binary yet) and 11 (0 tests selected yet), each with its must-pass reading one operand apart;
controls for 12 and 13 over a scratch clone, all as required (`.andromeda/runs/2026-11-04T19-36-08-phase/p5-controls.md`).
No `run --perf`, `--fuzz-replay`, `cargo deny` or mutation entry: no product, parser, dependency or graph change.

## Capabilities — concretization preview
- **No capability is claimed.** `v1-53` stays `planned, unclaimed` on the delegate's answer to the P4 fork (`inputs#I3`,
  the Viola overseer, founder-delegated, technical): «a matrix claim is never weaker than its acceptance: v1-53 asks for a
  live viola run on Linux, and a round made on 2026-10-08 on claude 2.1.287 is not one of this chunk». After the word,
  P5 writes its dated note: this chunk landed the suite-parity witnesses (the census, the per-OS JUnit reading), and its
  live Linux leg is "Version done-check"'s. Its acceptance is untouched.
- `v1-40` reads verified: the witness adds evidence; its ledger note is the wrap's (an expected amendment).

## The cascade (directive items 1 and 8)
- **Sampler:** `sample: off · phase · 2026-11-04-linux-and-macos-parity · not forced — the sampler runs on --force alone`
  (`sampler: off`). The classifiers ran as the letters set them, Haiku · high, three draws a listing.
- **Classify check lines:** `classify-p1`: `classify: 207 agents · 207 pass · yes 824 of 1048 (draw 1 744 · draw 2 749 ·
  draw 3 746 · one draw only 70)`; `classify-p2`: `classify: 3 agents · 3 pass · yes 1 of 1 (draw 1 1 · draw 2 1 · draw 3 1
  · one draw only 0)`. No dead draw.
- **pick's trail:** arch `yes 145 · no 47 · 9 strong reader(s) · draws 3 · one draw only 8`; security `yes 120 · no 21 · 8
  · draws 3 · one draw only 5`; design `yes 151 · no 60 · 9 · draws 3 · one draw only 34`; tests `yes 129 · no 29 · 8 ·
  draws 3 · one draw only 10`; obs `yes 228 · no 63 · 12 · draws 3 · one draw only 12`; a11y `yes 52 · no 4 · 3 · draws 3
  · one draw only 1` (each `split at 64 KB or 25 entities`).
- **`fanout.py check` refusals and re-spawns:** stage 1 13/13 pass; history 7/7 pass (three `not a marker` rows, all
  `2026-10-16T14-58-10-wrap`, a wrap run's name, read). Strong: `strong-p1` 30 of 45 on its first check
  (`QUOTES_VERBATIM 4, CARRIED_FROM_PICKED 2, BODY_IN_GROUP 12, BODY_UNDER_ENTITY 3`); after `strong-p2` (4/4) one
  `fanout.py move` over both parts (moved 32, held 8), `strong-p1` re-checked 41 of 45. Re-spawned once, `strong-p1-r1`
  (`CARRIED_FROM_PICKED`, `QUOTES_VERBATIM`): `arch-cascade-g2` passed, `design-cascade-g7` passed (it failed only on a run
  the move carried in from `design-cascade-g1` with that reader's mis-quote), `design-cascade-g8` passed,
  `arch-cascade-g9` failed the same two checks again (labels `architecture/crosscut-*` where the check reads
  `architecture/cross-cutting-patterns`) — the phase stopped and reported; the operator ruled it set aside (`inputs#I2`).
- **`arch-cascade-g9` dead:** its four facts read by main through `rec.py get` and carried into the plan as bindings under
  `architecture/cross-cutting-patterns` — fct-k6htnrfj (config management), fct-wfge3rsz (the capability ledger),
  fct-qip7sr3o (bounded readers), fct-7ok4646z (sanitised error surfaces) — the plan's Provenance.
- **Tally:** arch false-yes .303 · security .125 · design .497 · tests .248 · obs .193 · a11y .038; 6 masters, 1049
  entities, 825 classify-yes, standing 30 entities listed, draws dead 0.

## Refusals, and reaches for an old path (directive item 8)
- `fanout.py render --batch strong-p1 … --respawn …` refused by its usage, exit 2, nothing written: the re-spawn takes
  `--batch strong` and the tool derives the part (`fanout-contract.md` §Re-spawn's «the re-spawn is `{part}-r{n}`» read
  as the batch name).
- `inputs.py snap --step phase:P2` refused by its usage (the step set is P1 · P3 · P4 · P5), exit 2, nothing written; the
  ruling was snapped `phase:P3` (`inputs#I2`), where main read the four facts.
- `rec.py put --edit-file` refused a premise-closure edit file with multi-line pairs, exit 2, nothing written; the closure
  went through the working copy and `put --expect-rev 2` (SKILL.md Setup 5's edit path).
- Two `rec.py query` calls guessed a column (`input_row.chunk`, `coverage.notes`), Binder Error, exit 2; the views'
  columns read and the reads re-made.
- The Bash guard refused three calls, each re-issued once in the form it passes: a `cd` into the skill's references (P1),
  a `rec.py get` piped into `head` and a `cd` into the run dir (P2, the g9 diagnosis). The stamp-ahead hook refused an
  estimated `Generated:` stamp in the plan; re-written from `date -u`.
- No reach for an old path: every master read went through `rec.py read` · `find` · `get` · `render` · `tree`, the chunk's
  documents through `rec.py put` / `render`, position and coverage through `rec.py query`.
- One departure from the letter, recorded at the plan checkpoint: P4 read the 49 cascade extracts (642 KB with the six
  standing lists) by a structural index of their 1168 carried items' `why:` lines and read the 82 naming this plan's
  subjects, not each extract whole.

## Validation
Mechanical checks 1–12 pass (re-run in full after the baseline write): sections present; modify paths exist, no master in
the modify-set, every expected amendment names a master or the `matrix#v1-40` ledger-note form; new-file parents exist,
the minted names absent from architecture's resources; the fence parses, 24 entries; `planlint` 0 hits; snapshot
citations clean (3 entries, 0 uncited); `target/wincheck` ignored; step 4's mechanism read at
`crates/viola-state/src/board.rs:275` and `crates/viola-state/src/liveness.rs:27`. Validation-1: aligned with the scope.
Directive item 8's `landed tx` line and `rec.py check`'s line come after the word, at P5's landing, in the report.

## Beyond the plan
1. A CARRY on the existing entry **"Version done-check"**: `v1-53`'s one live `viola run` on Linux, its claim reading this
   chunk's suite-parity witnesses (the census, the pushed sha's per-OS JUnit reading) (`inputs#I3`).
2. The CARRY from chunk 2026-10-08-first-live-test-and-self-drive (a terminal answering `CSI ? 6 n` with `CSI ? r;c R`, or
   the kitty graphics query; its hypothesis that such a terminal loses the wheel at a live start) moves to **"Version
   done-check"**: it needs a live terminal and a real `claude`, and a shape joins the list only on the founder's word.
3. The CARRY from chunk 2026-10-10-viola-revive's legs this chunk cannot take — the founder-attended tab-close in a Linux
   terminal on the dev host, and a real `claude` read on Unix ending with its killed wrapper — moves to **"Version
   done-check"** (`inputs#I1` item 5).

Apply? (yes / review / cancel)
