# P5 card 1 — «Harness-prefix and identity-floor ledger rows» (2026-11-20-harness-prefix-and-identity-floor-ledger-rows)

**The plan:** `doc:2026-11-20-harness-prefix-and-identity-floor-ledger-rows/plan`, rev 4. Its render is at
`.andromeda/cache/docs/2026-11-20-harness-prefix-and-identity-floor-ledger-rows/plan.md`; open it in an editor.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-harness-prefix-and-identity-floor-ledger-rows`
  is a diff of the chunk's records.

**Inputs:**
- `inputs#I1`: the phase directive;
- `inputs#I2`: the P3-block relay, with the founder's answers 1–5 and the overseer's 6–11;
- the block itself, `.andromeda/runs/2026-11-20T18-17-18Z-phase/founder-forks.md`, with the answers appended and
  P4's two forks.

## 0. HELD — two forks for the founder, written to `founder-forks.md` at P4 (`inputs#I2` item 10)

P4 re-read the picked rows for what the sender prints, waits on and writes. Two consequences reach rows the P3
block's §5 did not name. The plan carries option (a) of each, and **implement starts only on his word on both**.

- **F-P4-1 — the kept stamp's label list.** `(security-plan · decision/live-rig-kept-stamp-owner-only)` names the
  CS, WS and PQ scratch homes. This chunk's start keeps its stamp under `HF` (and `HG` for the conditional second
  start).
  - **(a) The row names `HF` and `HG` beside them (Recommended).**
  - (b) Reuse `PQ` and its kept stamp, which must first be moved aside.
- **F-P4-2 — a user setting can make every version unverified.** The 2.1.328 bundle holds a `crossSessionInbound`
  setting that can turn inbound cross-session messages away or hold them for approval. A user who sets it gets
  `cross-session-prefix` `fail` on every version, so every version reads unverified: transport only, no dialog
  answers.
  - **(a) Accept it as a named residual beside the misaddressing residual (Recommended).**
  - (b) Read `absent` when nothing arrives. Then the row can never fail on the very change it exists to catch.

**Technical leans T1–T5, the overseer's to strike** (`founder-forks.md`):
- **T1:** the send goes between `/compact` and `/clear`. The id is still Run B's first SessionStart id, which
  `resume_target` reads and the agents read just listed live. `/clear` would replace it.
- **T2:** Run B ends by a kill with no key when the box is not up after that turn.
- **T3:** the address stays the session id, the founder's words. The bundle's exact form is a `uds:` socket path.
- **T4:** `SendMessage` is a deferred tool; the sender keeps one allowed tool.
- **T5:** what the sender leaves in the Claude config (below).

## 1. The plan

- **Goal:** two relied rows, 27 → 29, both in this chunk (founder item 1).
  - **`cross-session-prefix` (v2-25).** One more verify child, `claude -p` on Haiku with `--no-session-persistence`
    and `--allowedTools SendMessage`, runs from the print probe's cwd. Its compiled prompt names Run B's session id
    and carries a synthetic text that says it is a viola verify probe. Only the arrived UserPromptSubmit's raw start
    is read, against the four compiled literals. One closed outcome word is kept in the stamp: `harness`,
    `unprefixed`, `no-prompt`, `sender-failed`, `sender-deadline` or `not-run`.
  - **`identity-floor` (v2-26).** The capture arm writes `Env.<k>.json` on SessionStart: `CLAUDE*` names, counts,
    no value, 0600. Verify removes the kept names in memory. The row fails on any unknown name or any unshaped
    `CLAUDE` name. The compiled non-identity list is `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR` and
    `CLAUDE_ENV_FILE`. `IDENTITY_FLOOR` stays eleven.
- **Order:**
  - **A:** the capture arm and the canary test on 27 rows, whole workspace green.
  - **B:** the rows, the sender, the fake agent's sender, receiver and Env replay, the rig labels. Proven on
    test-built synthetic sets, committed nowhere and named synthetic, with three controls: `kept-names-removed`,
    `floor-unknown-fails` and `prefix-match`.
  - **C:** the round and `S1` = `HF`, `--cap 1 --spare 1`.
  - **D:** the landing over `fixtures/claude/2.1.328/`, then every layer.
  - **E:** the operator pass.
- **The live start `S1` (`HF`):**
  - one `viola verify --record` on 2.1.328 by path, in the rig's scratch home under its private config;
  - no key and no byte into any dialog; the credentials file read once; nothing written under the real
    `~/.claude*` or `~/.viola`;
  - green only on `29/29`, `failing: none`, `cross-session-prefix: pass`, `identity-floor: pass`,
    `cross-session: harness`, `env-unknown: 0`, the record and stamp kept, and every teardown word.
- **Step 25, the founder's red rule, by the rig's own words:**
  - The ONE red that opens `HG` (`--cap 2`): `identity-floor: fail` with `env-unknown` ≥ 1 alone, every other row
    `pass`, and the record not refused. Then the names go to him, he classifies them, the code is mended through
    the revision door, and `HG` re-records.
  - Any other red is a finding: implement stops.
  - `HF-2` is spent only on a refusal before the ledger line, or on a host read CONTENDED by a neighbour.
- **Before `S1`** (`inputs#I2` item 9). Each is an entry, or true by construction with its code line:
  - entries: the install read by path, the census, the build hash, the kept record and stamp ABSENT, the live stamp
    sha recorded, and `XDG_RUNTIME_DIR` owner-only;
  - by construction: the scratch home (run-id dirs on tmpfs) and the `.viola-verify-*` dirs.

  After it: the census keyed `operator`, the kept stamp `29 ok`, the env names (report-only), the live stamp
  unmoved, and the name diff at exactly 3.
- **Touchpoints:**
  - 2 new files plus the re-recorded set: the canary test, `Env.recorded-1.json` and
    `UserPromptSubmit.cross-session-1.json`;
  - 32 files to modify: the agent crate's ledger, lib and hook; `src/cmd/hook.rs`; verify and its typed runs; the
    fake agent; the cli_verify, contract and support tests; the rig; the secret-scan table; `Cargo.toml`; nextest's
    override; the diag schema's subject enum.
- **Test commands:** 38 entries. Phase A 7, phase B 7, round 6 + `S1` + census, after-reads 6, after-landing 3, boot
  smoke 3, operator pass 4.
- **Acceptance (top):** v2-25 and v2-26 as below; both rows' pass and fail tables; three controls «holds»; Run B's
  kill with no key; every relied literal at 29; `S1` green within the cap; the set a plain copy; all layers, pre-push
  and CI green.

## 2. Concretization previews (the never-weaken checkpoint; claims written only after the word)

- **v2-25.**
  - Today: «The prefix the CLI puts on a message sent in from another session is a stamped ledger row whose probe
    reads it live, a second session sending into the probe's session.»
  - Proposed: «verify's relied row `cross-session-prefix` reads `pass` on claude 2.1.328 live in the rig's scratch
    home (`S1` `HF`: `cross-session-prefix: pass`, `cross-session: harness`, `pass-count: 29/29`, kept stamp
    `29 ok`). Its probe: a print-mode `claude -p` sender's one `SendMessage` to Run B's session id arrives as a
    UserPromptSubmit whose raw start is a compiled harness literal. The recorded
    `UserPromptSubmit.cross-session-1.json` is replayed by the fake agent in CI (`contract_ledger_probes`, 2.1.328
    `29 pass 0 fail`).»
- **v2-26.**
  - Today: «Which CLI-identity environment names a wrapped session keeps is a stamped ledger row whose probe reads
    names only; no value reaches any capture, log or fixture.»
  - Proposed: «verify's relied row `identity-floor` reads `pass` on claude 2.1.328 live in the rig's scratch home
    (`S1` `HF`: `identity-floor: pass`, `env-unknown: 0`). Every `CLAUDE*` name the CLI adds to a hook child is on
    the eleven-name floor or the compiled three-name non-identity list, and the kept stamp's `env_probe.added` lists
    them. The canary-value test `security_negatives_env_capture` is green in CI: no value in any capture, log, stamp
    or record, kept names absent from both. The recorded `Env.recorded-1.json` is replayed by the fake agent in CI.»
- Neither is gated. Both stand on the founder's item 5. A red `S1` verifies neither: the cap then stays pooled, or
  the HG revision re-claims it.

## 3. What the sender leaves (`inputs#I2` item 6) and the residual rows

- **No transcript** (`--no-session-persistence`).
- **No new project dir:** the sender runs in the print probe's cwd, whose project dir verify already leaves.
- **Unmeasured:**
  - a session-registry entry;
  - an input-history line;
  - a socket under `$XDG_RUNTIME_DIR/cc-socks/` while it runs.
- **Carried by:** `(security-plan · decision/verify-real-cli-transcripts-accepted)`, its architecture twin, and the new
  misaddressing residual row (Expected amendments). F-P4-2 (a) adds the `crossSessionInbound` residual to that row.

## 4. The run, verbatim where a tool printed it (`inputs#I1` item 11)

**The union** (`rec.py rows-union`, take-up window):
```
architecture: rows 972 · packs 10 · picked 312 (167 by every draw) · standing 24 · 91754 B
security-plan: rows 623 · packs 7 · picked 222 (168 by every draw) · standing 15 · 64076 B
design-system: rows 324 · packs 4 · picked 55 (40 by every draw) · standing 4 · 14170 B
layout-templates: rows 223 · packs 3 · picked 43 (27 by every draw) · standing 0 · 9961 B
test-plan: rows 1355 · packs 14 · picked 283 (181 by every draw) · standing 61 · 73926 B
obs-plan: rows 1032 · packs 11 · picked 195 (107 by every draw) · standing 52 · 48833 B
a11y-plan: rows 235 · packs 3 · picked 102 (53 by every draw) · standing 18 · 26130 B
rows-union: 7 master(s) · picked 1212 of 4764 · rule any
```
- **The rows were read by search, not whole:** the sum is 328 850 B, past 250 000 B.
- **Every cited row is in the union or its standing list:** 56 cited, 0 missing (a scripted check of each
  `(doc · key)` against that master's rows and standing files). It found one miss before the fix: the P3 block and
  the plan's first revs named `test-plan · decision/env-names-only`, which is obs-plan's row. Fixed in the plan at
  rev 4; the block's §5 line is left as written.

**`[RETYPED]` lines and re-spawns** (fanout `check`, take-up window):
```
security-pick-p1.d3 · [RETYPED] line 67: decision/agents-probe-logging-redacted is no row of the pack — read as decision/agents-probe-log-redaction, the pack's row 67
security-pick-p2.d3 · [RETYPED] line 2: contract/strict-modes-refusal/cli-frame-verb is no row of the pack — read as contract/strict-modes-refusal/cli-frame-verbs, the pack's row 2
tests-pick-p6.d2 · [RETYPED] line 50: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 50
re-spawned 1 agent(s) · NOT_NEEDED_REACHED 1 · wall 28 s
```
The re-spawn was the layouts task writer, batch `task-r1`; its raw twin is `.raw-layouts-task.md`.
`draws-dead 0 · batches-on-one 0`.

**Non-zero exits and refusals:**
- P3: `gate.py scope --lists-only` printed `UNPARSED: research.md:52 — not the derived form (§Scope)`. Fixed then;
  re-read `scope lists: parsed 23 items`.
- P4: `inputs.py snap` of `founder-forks.md` → exit 3, `--source lies in this project's own repository, under
  .andromeda/ — the pipeline's own tree`. The plan cites the file by its run-dir path.
- The Bash guard refused two `cd` into a reference or run dir and one pipe of a tool's output into a filter. Each was
  re-issued once in the guard's form.
- **Baselines** (new entries, untouched tree), each the reading the plan predicts:
  - green on the untouched tree: 3 and 12 (1122 passed) and 11 (99 passed);
  - red on an absent subject: 4 and 14 (exit 101, no such test target), 13 (exit 4, 0 selected), 23, 25, 26, 27
    and 28;
  - recorded: 24.
- **Must-pass and fail-input controls** for 23, 25, 26 and 27 over a scratch clone at `b3eb184c`: every pass input
  green, every fail input red (`.andromeda/runs/2026-11-20T18-17-18Z-phase/p5-controls.md`).
- No `INDETERMINATE` and no `NOT DERIVED` line. planlint: `hits: 0`. Its 12 (d) listing (3 claims) was read and
  each holds. Check 4 (17) lists 0 of 23.

**Checks:**
- 1–5, 7–12: pass.
- **6, size, WARN:** about 725 lines of prose outside the fence (1016 rendered, 291 fence). Two rows and a new child
  in one chunk is the founder's item 1.
- **Validation-1:** aligned with the scope and the working entry. `hook.rs` is touched for the prefix ids alone; the
  four literals do not change.

**Where a letter was wrong or silent:**
- (a) The P3 block placed the sender «after its framing pastes» and also named «Run B's first SessionStart» id.
  `/clear`, the last framing paste, replaces that session. Resolved as lean T1. It is my own block's fault, filed as an
  evolve friction.
- (b) The block's §5 named `env-names-only` under test-plan; it is obs-plan's.
- (c) The precedent snapped `founder-forks.md` as an input; the inputs tool now refuses a run-dir file. The letter
  is silent on which wins; the plan cites it by path.
- (d) The evolve `validate` checkpoint and P5 step 5's land fire «on every P5 outcome». A stop at the card, before
  the word, is no outcome, so neither has fired. They fire at the word.
- (e) No master row was found reached outside §5 except F-P4-1 and F-P4-2.

## Beyond the plan

none

---
Apply? (yes / review / cancel)
