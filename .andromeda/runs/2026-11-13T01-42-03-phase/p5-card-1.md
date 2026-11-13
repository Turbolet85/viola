# P5 review — 2026-11-13-verify-readiness-on-the-input-box · «Verify readiness on the input box»

Written 2026-11-13T02:31:49Z. Plan rev 2 · scope rev 3 · research rev 2. Run dir `.andromeda/runs/2026-11-13T01-42-03-phase`.

**To read or edit the plan:** `.andromeda/cache/docs/2026-11-13-verify-readiness-on-the-input-box/plan.md` (rendered
from the store, rev 2). No plan stands in the chunk's folder. Writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-13-verify-readiness-on-the-input-box`
diffs the chunk's records.

## The plan

**Goal.** Run B's guards read «the input box is up» from a footer literal or from the box's own rows: a `─`-only row
of at least 40 directly above a row opening `❯`, with no modal row. One predicate (`Signatures::input_box_up` in
`viola-agent-claude`) serves four readers:
- verify's guards, and its settle and box waits;
- the `input-box-signature` row;
- `viola run`'s pre-send gate, which shares the literal and the failure (scope rev 3 names it).

A recorded screen keeps a box row as its literal alone. There is no new ledger row, the relied count stays 24, and
every stamp, 2.1.296's included, keeps its reading. The supply-chain red closes with three reasoned `deny.toml` ignores
and rustix 1.1.6.

**Steps.**
1. The box signature, `BOX_RULE_MIN`, the predicate and `verdict` (`screen.rs`).
2. The ledger row on the predicate, the box rows recorded as literals, and `TypedRun.box_only_guards` in the stamp's
   `measured.typed_probe` (`ledger.rs`).
3. Verify's guards, settle and box wait on the predicate, the box-only guards counted (`turns.rs`, `pty.rs`, `runs.rs`).
4. The run gate's box-alone case (`src/run/gate.rs`; no gate code changes).
5. `SIGNATURE_LITERALS` and a refused prompt-row-with-text case (`tests/contract_fixture_hygiene.rs`).
6. The three ignores and rustix 1.1.6 (`deny.toml`, `Cargo.lock`).
7. The live proof: operator entries 30–36, fired only once the founder's per-chunk live-start cap is on record.

**Touchpoints (10, all modify):**
- `crates/viola-agent-claude/src/screen.rs`
- `crates/viola-agent-claude/src/ledger.rs`
- `crates/viola-agent-claude/src/ledger/test_cases.rs`
- `src/cmd/verify/typed/turns.rs`
- `src/cmd/verify/typed/pty.rs`
- `src/cmd/verify/typed/runs.rs`
- `src/run/gate.rs`
- `tests/contract_fixture_hygiene.rs`
- `deny.toml`
- `Cargo.lock`

**Test commands (43 entries).**
- 1–29 for /implement: lint, unit, integration, e2e, the supply-chain probes and the stamped smoke (no mutation).
- 30–36, the live proof: operator entries under the founder's cap.
- 37–43: hygiene, the push, and five CI reads (supply-chain, test, lint, msrv, release).

**Top acceptance criteria.**
- The predicate decides for all four readers: `cargo nextest run -p viola-agent-claude` and `agent-run.sh run --unit`
  green.
- Every stamped recorded set still reads 0 fail; the stamped boot reads `ok:true`.
- `cargo deny check` reads all four families ok, and CI's `supply-chain` reads `verdict: green` on the pushed HEAD.
- `pre-push` is green.
- **matrix#v2-29** (operator, under the cap) — below.

## Concretization preview — the one capability this chunk claims

**v2-29**, method `e2e`.
- **Now:** «On the newest installed claude CLI, one `viola verify` run reads every capability-ledger row pass but the
  account-dependent status-line rate-limits row, its pastes landing in the input box while a transient footer notice
  stands.»
- **Proposed:** «On claude 2.1.322, the newest installed at P4 (a newer one installed by the proof moves it, recorded),
  one `viola verify` of this chunk's default-feature build (1) prints a `[NN/24]` step line for every relied row, none
  reading `fail` but `statusline-rate-limits`, its last line `stamped 2.1.322  …`; and (2) writes a stamp whose
  `measured.typed_probe.box_only_guards` is at least 1 — a Run B paste guard passed on the input box with no footer
  literal on screen, a transient footer notice standing.»
- **Never weaker.** Clause (2) makes «while a transient footer notice stands» an asserted reading, where it was
  unasserted.
- **Risk, named.** The cap stays `implemented`, not verified, in two cases:
  - the founder's live-start cap is not on record before the wrap;
  - no notice stands at the proof, so clause (2) reads false — a usage warning resets with the account's week, and an
    «Update available!» notice needs a release newer than the one installed.

  It is not a gated claim: no route block stands.

## Forks (no dialog — the founder is away; `inputs#I5`, each answered by the directive's delegate rule, PROVISIONAL)

- **F1 — the supply-chain form.** Answered: three reasoned ignores; rmcp held at its ratified `>=3.4.1, <3.5`.
  - Each advisory is confined to a disabled feature: `auth`; `transport-streamable-http-server`; the
    streamable-HTTP clients, which `rmcp-features.sh` keeps out of the release graph.
  - The security plan's own response for that class applies (security-plan · decision/cve-sla-disabled-feature).
  - Alternative, measured feasible: rmcp `>=3.9.0, <3.10`. A scratch `cargo check -p viola-mcp -p viola-e2e
    --all-targets` is clean on 3.9.0 (adds `const-oid 0.10.2`). It moves the founder-ratified range.
- **F2 — the readiness signature.** Answered: the box signature added beside the footer literals, a box row recorded
  as its literal alone.
  - The founder ruled the second footer literal in himself (chunk 2026-10-19-newer-claude-cli-rows), so a change to
    the readiness literals is his fork.
  - The wrap may read it as a boundary widening (rul-r74hshif); it is listed under Expected amendments.

## Validation

- **Mechanical checks.**
  - Sections all present; the 10 modify paths exist; no master is a touchpoint.
  - Fence parses: `gate.py run --dry-run`, 43 entries, exit 0.
  - `planlint`: hits 0. Its one listed `unchanged` claim on `Signatures` (line 37) is true: the four existing literals
    stay and two are added.
  - Lists parse: `scope lists: parsed 10 items`.
  - Snapshot citations: `inputs.py verify` — 5 entries, unchanged 4 · n/a 1 · drifted 0 · uncited 0.
  - The redirect target in entry 32 is gitignored (`git check-ignore -q` exit 0).
  - Size: 554 lines, above the ~400 guideline; the 43-entry gate block is most of it.
- **Baselines** (`gate.py baseline`: seen green 17 · not seen 0 · new 12 · leg 14):
  - green: 1 (/tmp 22G free); 7 (572/572); 10 (integration 511 passed); 11 (e2e 276 passed); 15 (fuzz advisories
    ok); 23–25 and 28 (the stamped smoke, boot 5.09 s).
  - red, the red the plan predicts: 12 (no ignore yet); 13 (rustix 1.1.5 under tempfile 3.27.0); 14 (advisories
    FAILED on the four ids, bans · licenses · sources ok).
- **Controls** (`p5-controls.md`, a `git clone --shared` at `2f9eaddb`):
  - 12, 13 and 14 read as on the real tree unmodified, and green over the step-6 change (three ignores, rustix 1.1.6).
  - The verify readers 33–35 read red on the recorded failing 2.1.302 output and green on a control copy carrying the
    subject.
- **Validation-1** (plan against `doc:…/scope` rev 3 and the working entry): aligned, with one wording point for you.
  - The entry says «never keyed on a footer literal». The plan keeps the two footer literals as one arm beside the box:
    readiness no longer depends on a footer literal, but it still reads one when one stands.
  - Read literally, the entry asks for F2 option 2 (replace the literals). That would stop every stamped recorded set
    reading ready until re-recorded live.
  - The plan takes F2 option 1. Your word decides whether the entry's wording or the delegate answer stands.

## Report (directive item 4): every non-zero exit, refusal and unexpected behaviour this run

**Non-zero exits and refusals.**
- P1: `rec.py put --edit-file` exit 3, nothing written — an `old:` anchor crossed a line the scope draft had wrapped;
  re-fired, landed rev 2.
- Bash guard refusals (each re-issued once in the passing form):
  - `cd` into the run dir;
  - `cd` into the overseer tree;
  - `gate.py run … | tail`, a pipeline tool's output into a filter.
- PostToolUse `stamp-ahead` refused the plan's estimated `Generated:` stamp (+9 min); corrected from `date -u`.
- Research probes:
  - a local advisory-db lookup loop exited 1 — the local db predates the four advisories; they were read from run
    ci#43344540425's log instead;
  - a `python3 -c` envelope probe exited 1 (`TypeError`), wrongly assuming `data.<ver>`; the true shape is
    `data.versions.<ver>`.
- Pickers: three draws failed `[EVERY_ROW]`, each writing a near-miss key for its pack's key. Each was re-spawned once
  and passed:
  - `arch-pick-p8`: `decision/strict-check-before-log` for `decision/strict-modes-check-before-log`;
  - `tests-pick-p9`: `decision/synthetic-fixture-text-only` for `decision/synthetic-fixture-text`;
  - `obs-pick-p6`: `contract/hook-decision/socket-dir-refused` for `contract/hook-decision-detail/socket-dir-refused`.
- Baselines red by prediction: 12, 13, 14. Controls red by design: 33 and 35 on input A, 34 on input A.
- CI at Setup 5a: 7 of 9 shas red on `supply-chain` / nightly `advisories`. Runs ci#43346505444 and ci#43345611887 were
  `in_progress`, their failed check already completed.

**UNPARSED / INDETERMINATE / NOT DERIVED lines:** none printed by any tool this run.

**`not a marker` rows** (`fanout.py check`, history batch, 8 rows): `2026-11-12T03-23-49-rows` in the arch, security,
design, layouts, tests, obs and a11y history files, and `2026-11-08T20-32-52-wrap` in design. These are run-dir names
heading sidecar entries, not chunk markers. Read, not failed.

**The tally — its arm and pack lines, verbatim** (`rec.py rows-packs`, then `rows-union`):
```
architecture: rows 987 · packs · packs 10 · pack 100 · 274159 B
security-plan: rows 614 · packs · packs 7 · pack 100 · 176458 B
design-system: rows 325 · packs · packs 4 · pack 100 · 77497 B
layout-templates: rows 224 · packs · packs 3 · pack 100 · 52598 B
test-plan: rows 1374 · packs · packs 14 · pack 100 · 361722 B
obs-plan: rows 1059 · packs · packs 11 · pack 100 · 287316 B
a11y-plan: rows 241 · packs · packs 3 · pack 100 · 67218 B
rows-packs: 7 master(s) · 4824 row(s) · 52 pack(s) · 3 draw(s) a pack · pin 803
architecture: rows 987 · packs · packs 10 · picked 194 (124 by every draw) · standing 22 · 55316 B
security-plan: rows 614 · packs · packs 7 · picked 126 (87 by every draw) · standing 4 · 35050 B
design-system: rows 325 · packs · packs 4 · picked 62 (45 by every draw) · standing 0 · 14765 B
layout-templates: rows 224 · packs · packs 3 · picked 51 (39 by every draw) · standing 2 · 10034 B
test-plan: rows 1374 · packs · packs 14 · picked 226 (126 by every draw) · standing 44 · 56258 B
obs-plan: rows 1059 · packs · packs 11 · picked 318 (131 by every draw) · standing 36 · 72384 B
a11y-plan: rows 241 · packs · packs 3 · picked 60 (32 by every draw) · standing 12 · 16334 B
rows-union: 7 master(s) · picked 1037 of 4824 · rule any
```
(each `@tally` reads `arm=packs pack=100 draws=3 draws_dead=0 batches_on_one=0`; census:
`pick · 156 agents · first try 153 · wall 1332 s · in 2 parts`.)

**Where the row read's letters could not be followed as written, or the read behaved unexpectedly.**
- The pick render passed the 500,000 B margin and came in two parts (`pick-p1` 83 agents, `pick-p2` 73), as
  §The batch 1a provides. No letter orders a part's re-spawn against the next part. I ran `pick-p1-r1` after
  `pick-p1`'s check and before `pick-p2`.
- Every master went `packs`; none went `whole`. The line form `· packs · packs {k}` repeats the word, because the arm
  word and the count's label are the same word.
- Coverage is wide: obs-plan picked 318 of 1059, 119 on one draw. Many of those rows (MCP sinks, the panic gate, OTel)
  bind no part of this chunk, and the planner read 72 KB in two pages.
- P3 corrected the scope to name the run gate In. The letter re-runs the row read when a correction adds «a surface no
  task named», but the task files are process files the letter forbids reading. I judged from the rows files instead:
  the run gate's rows are picked in four masters (architecture · decision/send-waits-out-paste-hint, test-plan ·
  contract/readiness-gate/gate-max-wait, a11y-plan · decision/human-key-takes-the-wheel, design-system ·
  contract/refusal-hints/not-delivered-input-not-ready). On that basis I took the surface as named, and did not re-run
  the read.
- `rec.py tree doc:{marker}/plan --outline` printed `outline (0 files)` for a plan with 12 sections; the plain
  `tree` listed them.

## Beyond the plan

| proposal the word would approve | owner |
|---|---|
| The unpin of `vb.sh`'s 2.1.296 shim after the proof | the operator (outside every route entry; `inputs#I1` item 1) |
| A box-only screen fixture replayed by the fake agent in CI: none can be recorded while `statusline-rate-limits` fails on this account (`--record` writes only at 0 fail), and none is hand-written | «Status-line interception removed» (it retires the rate-limits row, after which a clean `--record` can carry one) |
| v2-29 left `implemented` if the cap or a standing notice is missing at the proof — no route insertion, the wrap reports it owed | this chunk's wrap |

Apply? (yes / review / cancel)
