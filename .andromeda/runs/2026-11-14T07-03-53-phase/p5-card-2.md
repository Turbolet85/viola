# P5 review — 2026-11-14-hyprland-seam «Hyprland seam» (viola-0.2.0, Epoch 1 — Foundation) — card 2

The plan: `.andromeda/cache/docs/2026-11-14-hyprland-seam/plan.md` (rendered from `doc:2026-11-14-hyprland-seam/plan`
rev 7 — open it in an editor). No plan stands in a chunk's folder; writing in that file changes no record until a
`put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-hyprland-seam` diffs the chunk's
records.

## The review folded (`inputs#I3`, the overseer's technical review, snapped at P5)
1. **The recorder now keeps strings only from a closed allowlist** (steps 8 and 9).
   - It keeps a string only where its key stands on a closed per-request allowlist AND the value has the closed form
     the seam parses:
     - `clients` / `activewindow`: `address`, `swallowing`, and each entry of `grouped`, all addresses;
     - `workspaces`: `lastwindow`, an address;
     - `version`: `tag`, in the `v<major>.<minor>.<patch>` form.
   - Every other string, at any depth, becomes the placeholder `viola fixture text`. Numbers, booleans and the JSON
     shape stay.
   - A non-JSON reply is kept only when it is the measured dispatch success word; otherwise it is replaced whole.
   - Step 9's walk asserts the same rule. It gains planted `lastwindowtitle` and unlisted-key cases beside the title
     case.
   - The rejected deny-list scrub is recorded under Constraints. The Expected amendment row is renamed
     `+ decision/hyprland-recorder-string-allowlist`.
   - The founder's answer («titles replaced») stands. This narrows what is committed and widens nothing.
2. **Each control names its case, as nextest prints it** (step 14). The three entries the controls fire now run with
   `--no-fail-fast` (entries 15, 16 and 18, re-baselined).
   - `oversize`: entry `cargo nextest run -p viola-hyprland --no-fail-fast`, case
     `viola-hyprland reply::tests::parse_clients_takes_max_frame_bytes_and_refuses_one_more_as_oversize`.
   - `deadline-kill`: the same entry, case `viola-hyprland read::tests::read_past_the_deadline_kills_and_reaps_its_child`.
   - `title-scrub`: entry `cargo nextest run -p viola-e2e --bin viola-hypr-tap --no-fail-fast`, case
     `viola-e2e::bin/viola-hypr-tap tests::record_replaces_every_string_off_the_allowlist`.
   - `hygiene-title`: entry `cargo nextest run -p viola --features fake-agent --test contract_fixture_hygiene
     --no-fail-fast`, case `viola::contract_fixture_hygiene planted_hyprland_lastwindowtitle_is_rejected`.
   - «Holds» means that case ran and FAILED with the guard removed. A removed firing that is red on another case alone
     is not «holds».
3. **Criterion 1 is aligned to entry 14's `expect`:** exit 0; contains `viola-hyprland v`; lacks `tokio v`,
   `viola-agent-claude v`, `viola-state v` and `viola-channel v`.

## Goal
A new sync library crate `viola-hyprland` is viola's one seam to the compositor. It holds:
- the relied-on requests as a closed set: `clients`, `activewindow`, `workspaces`, `version`, and the dispatches
  `focuswindow` and `workspace`;
- a bounded read of `hyprctl -j`: a direct spawn, no shell, `take(MAX_FRAME + 1)`, killed at 5 s;
- tolerant parsers that drop every text field;
- closed codes-only outcomes;
- two log words, `hyprctl-probe` and `hyprctl-json`.

The reply shapes are measured live at L1 and L2 by the test-only recorder `viola-hypr-tap`. It keeps only the
allowlisted closed forms (everything else is the placeholder), then scrubs a recording or refuses it whole. A fake
`hyprctl` replays the recorded set byte for byte in CI on `ubuntu-latest`. No verb is built, and no capability is
claimed.

## Top acceptance criteria
- The seam's normal graph holds no `tokio`, `viola-agent-claude`, `viola-state` or `viola-channel` (the entry 14
  atoms above). It is the fifth sync crate.
- Reads are bounded:
  - a reply over the cap is refused whole as `oversize`;
  - a child past the deadline is killed and reaped (`read_past_the_deadline_kills_and_reaps_its_child`).
- No title or class crosses the seam: a planted canary title reaches no line and no error.
- L1 and L2 are recorded on your host, each on its own word, L2 only after `hyprctl locked` does not read `true`.
  Every string in the set is an allowlisted closed form or the placeholder.
- The fake replays every recorded file byte for byte. The fixture class passes its schema, hygiene walk and allowlist
  check. Each parser has a 512-case proptest, and `hyprctl_json` joins the fuzz replay.
- Folded at P5: `fuzz/Cargo.lock`, stale at `4255b3ed`, resolves under `--locked`.
- The four controls each read `holds` on their named case, past fail-fast.

## Touchpoints
- **New:** the crate (`lib.rs`, `reply.rs`, `read.rs`, `proptest-regressions/`), `src/bin/viola-fake-agent/hyprctl.rs`,
  `crates/viola-e2e/src/bin/viola-hypr-tap.rs`, `schemas/hyprland-fixture.v1.json`, `fixtures/hyprland/`,
  `tests/contract_hyprland_seam.rs`, `fuzz/fuzz_targets/hyprctl_json.rs` with its corpus.
- **Modified:** `Cargo.toml`, `Cargo.lock`, `scripts/sync-crates.txt`, `src/obs.rs`, `schemas/diag-line.v1.json`,
  `src/bin/viola-fake-agent.rs`, `tests/support/fake.rs`, the three contract tests, `crates/viola-e2e/Cargo.toml`,
  `fuzz/Cargo.toml`, `fuzz/Cargo.lock`, `.gitattributes`.

## Test commands
The block has 38 entries. 18 are new and each is baseline-run; 14 were seen green at the last wrap; 6 are leg
entries. The reds are the ones the plan predicts: the crate, recorder and test targets are absent, and the fuzz
lockfile is stale. The operator entries are L1, `hyprctl locked`, L2, hygiene, push, and the CI read.

## Concretization preview
None. v2-07, v2-11, v2-12 and v2-14 each need their verb. Nothing is claimed, and the matrix is not touched.

## Live-desktop steps (directive item 2) — each a named operator step waiting on its own per-chunk word
- **L1:** `viola-hypr-tap record` — read-only `hyprctl -j` queries of the running compositor, plus the no-instance
  `version` read (gate entry 3).
- **Before L2:** `hyprctl locked` (gate entry 4). Stop on `true`; never unlock.
- **L2:** `viola-hypr-tap record-dispatch` — `focuswindow` on the window that already has focus, and `workspace` on
  the workspace already shown (gate entry 5).
- No typed key, and no live `claude` start. `hyprctl version` is a print-and-exit read, not a live step.

## Directive item 6 — the block, verbatim where a tool printed it

```
rows-union (verbatim):
architecture: rows 987 · packs 10 · picked 123 (80 by every draw) · standing 20 · 38291 B
security-plan: rows 615 · packs 7 · picked 113 (74 by every draw) · standing 24 · 31708 B
design-system: rows 325 · packs 4 · picked 8 (6 by every draw) · standing 4 · 4535 B
layout-templates: rows 225 · packs 3 · picked 5 (2 by every draw) · standing 0 · 1939 B
test-plan: rows 1375 · packs 14 · picked 219 (131 by every draw) · standing 72 · 54427 B
obs-plan: rows 1058 · packs 11 · picked 279 (151 by every draw) · standing 34 · 64643 B
a11y-plan: rows 241 · packs 3 · picked 20 (9 by every draw) · standing 11 · 6727 B
rows-union: 7 master(s) · picked 767 of 4826 · rule any

task shape (Needs per master's task): not read by me — the letter's ban stands (fanout-contract.md §The batch 5);
  the overseer reads it from the seven task files (`inputs#I3`, closing note).

rows the plan cites (scripted read over the rows and standing files, plan rev 7):
  handles in the plan: 56 · in a rows file: 49 · only in a standing list: 1 · in neither: 6
  standing only: decision/hygiene-reports-class-only ['tests-standing.md']
  neither: test-plan · decision/forced-window-hook-is-test-parameter — unpicked; found by rec.py find at P3
           (dec-gnxmaodo), cited as a row looked up small
  neither: contract/diag-subject/hyprctl-probe, decision/hyprctl-direct-spawn, decision/hyprctl-resolved-from-path,
           decision/hyprland-recorder-string-allowlist, decision/hyprland-seam-crate — this plan's own `+` handles
           (Expected amendments (wrap)), rows that do not exist yet

gate.py control: none ran at P5 — the four controls are implement's one-shot run (plan step 14), each on its named
  case past fail-fast; their entry lines and clone names land in the chunk's evidence then.

[RETYPED] lines (fanout.py check, verbatim):
arch-pick-p10.d2 · [RETYPED] line 90: decision/anyhow-only-root-binary is no row of the pack — read as decision/anyhow-only-root-bin, the pack's row 90
tests-pick-p9 · [RETYPED] line 63: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 63
tests-pick-p13.d3 · [RETYPED] line 98: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 98
[EVERY_ROW] re-spawns: 0 (156 of 156 draws passed first try; draws-dead 0)

non-zero exits, refusals, unexpected readings:
- bash-guard refused 4 calls, each re-issued once in the guarded form: three `cd` into a skill or run dir; one pipe of
  a tool's output into a filter (that refused call also carried a plan `put`, re-fired bare: "landed … rev 4").
- `cargo tree --manifest-path fuzz/Cargo.toml --locked …` → exit 101 "cannot update the lock file … because
  --locked was passed" — fuzz/Cargo.lock stale at 4255b3ed (the veil edge of viola-agent-claude, added by
  2026-11-13-rows-proposal-rule-work, missing); found because the baseline run of `agent-run.sh run --fuzz-replay`
  rewrote the file (restored with git checkout after the reading). Folded into this chunk (directive item 5).
- gate entry 14 (rev 2 form `grep -c … expect ['exit 1','last line 0']`) read GREEN VACUOUSLY on the absent crate:
  cargo tree exited 101, grep -c printed 0 → re-spelt to assert the crate line (control-14.md).
- baseline reds the plan predicts: 14 (exit 1), 15 / 16 / 17 (exit 101, targets absent; 15 and 16 re-run in their
  `--no-fail-fast` form, the same reading), 24 (exit 101, lock stale), 26 (exit 2, files absent).
- my own scratch script `baselines.py` failed once on an assertion (an apostrophe in entry 14's reading); re-run with
  the TOML basic-string form.
- plan defect caught at review (`inputs#I3` item 1): step 8's deny-list scrub (two keys) would have committed
  `lastwindowtitle`, classes, tags and nested names; turned into a closed keep-list.
- letter silent: plan-template and validation.md do not say that `agent-run.sh run --fuzz-replay` rewrites
  fuzz/Cargo.lock in the tree; a stale lockfile passes CI's supply-chain job unseen.
- UNPARSED / INDETERMINATE / NOT DERIVED: none printed by any tool this run.

keyed contracts / NOT YET MEASURED: the plan needs no new keyed contract; its new rows are Expected amendments. No
  NOT YET MEASURED mark stands on a row this plan reads.
```

## Forks answered at P4 (`inputs#I2`)
- **Q1 transport:** `hyprctl -j`, direct spawn. This is the founder's own word, relayed verbatim by the overseer, on
  the escalate pattern «Boundary widening — a subprocess/IPC boundary gains a new crossing».
- **Q2 recorder:** titles replaced. Also the founder's own word, relayed verbatim, on the same pattern. The P5 review
  narrows it to the closed allowlist and widens nothing.
- **Q3 placement:** inside `viola-hyprland`. The overseer's technical answer (delegate overseer).
- The wrap halts on Q1 and Q2 once, as one answer card.

## Warnings
- Plan size: 587 lines in all (`wc -l`). 268 of them are the `[[gate]]` fence, so the prose is 319 lines, inside the
  guide.
- `HYPRCTL_DEADLINE` (5 s) is PROVISIONAL (see Beyond the plan).

## Beyond the plan
| proposal | owner |
|---|---|
| The `exec` window-placement dispatch and the `monitors` query, left out of the relied set | «`viola open`» — the existing working-route entry «`viola open` — a terminal window per session beside its parent; closed flag allowlist, no shell; window identity and origin in its own state, logged codes-only» |
| Settling the PROVISIONAL `HYPRCTL_DEADLINE` (5 s) against a measured need | «`viola open`» (the seam's first product caller), same entry |

Apply? (yes / review / cancel)
