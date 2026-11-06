# P5 review card 1 — 2026-11-06-version-done-check («Version done-check», viola-0.1.0)

**The plan** is `doc:2026-11-06-version-done-check/plan` rev 4, rendered to
`.andromeda/cache/docs/2026-11-06-version-done-check/plan.md` to open in an editor.
- No plan stands in a chunk's folder. Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-06-version-done-check` is a
  diff of the chunk's records.

## Goal
Close 0.1.0:
- every gate green on Linux, CI's seven jobs included;
- the masters agree with the code on all 17 CARRYs;
- the 17 open caps ruled by the founder's own word (`inputs#I2`, «Принять всё»): 13 proved here, 4 deferred to 0.2.0.
It adds no product surface.

## What the chunk does
- **The red:** `portable-pty` `=0.8.1` → `=0.9.0` (nix 0.28, serial2). This drops pin-utils (RUSTSEC-2026-0441,
  ci#42255961371) and serial; the RUSTSEC-2017-0008 ignore leaves, a narrowing. Measured in a clone: `cargo deny
  check` all ok, and viola-pty 33 of 33.
- **The remnants:** 47–48 files. The Windows-only code leaves: `plain_drive`, the `EXE_SUFFIX` joins, the `.exe`
  trim, and the Git Bash warning (T2). Remnant comments are re-worded. The census and the `#[cfg]` evaluator are kept,
  each with its reason.
- **Two live starts, the founder's cap** (`inputs#I3`, «Verify = запасная»):
  1. one `viola verify` stamping a rig home on 2.1.287;
  2. one wrapped `viola run`. It reads the start and the child's fd table, holds a permission dialog to the 3600 s
     ceiling with no key, then kills the wrapper and reads the child gone.
  The rig is copied byte for byte from 2026-10-19. A modal is killed with no key, a red is never re-fired.
- **Witness readings and records:** T3 (this session's own wrapping, kinds and counts), v1-09's enumeration, v1-26's
  table, and the census count.
- **Masters, at the wrap only:** F434's two whole-fact edits; T1's five ratified divergences; the census rule; the
  trycmd → literal-pin re-word (CARRY 1); `=0.9.0` everywhere; fct-p7rzex2i's mechanism (CARRY 15); Epoch 7's
  PROVISIONAL marks discharged (his word); the ceiling's PROVISIONAL lifted if the hold reads green.

## Touchpoints
- `Cargo.toml`, `Cargo.lock`, `deny.toml`;
- the remnant files of research §Files to modify;
- `tests/contract_fake_agent_drift.rs`, only if no planted-annotations case stands;
- evidence under `.andromeda/cache/chunks/2026-11-06-version-done-check/evidence/`.

## Test commands (50 entries)
- **Lint:** fmt, clippy, G1, G3, lint-probes, check.
- **The predicted reds** (each with a must-pass control, `p5-controls.md`):
  - `cargo tree -i pin-utils` (8);
  - the pin and ignore line (9);
  - the remnant guard, 75 lines in 47 files (10);
  - `cargo deny check` (15).
- **The moved graph:**
  - viola-pty nextest;
  - msrv unit and check;
  - fuzz deny, the tokio bans, channel-tokio-free, deny-probes;
  - zizmor, orphans, rmcp;
  - release-check probe and build (green, `viola only`).
- **Suites:**
  - `run --unit`;
  - the e2e filter (152 passed at baseline);
  - the integration filter (177 passed);
  - perf and its gate;
  - the smoke boot / status / G2 / G4 / cleanup;
  - `pre-push`.
- **The live record's readers** (37–39): 2 starts, 0 processes left, every reading green. Each is red on the absent
  ledger, with a must-pass and a must-fail control ledger.
- **Operator:** hygiene, push, and seven `ci.py` reads, `supply-chain` now asserted green.
- **Report-only:** the last scheduled nightly run (success, 2026-11-02).

## Concretization previews (current acceptance → what this chunk proves)
- **v1-01** «every push … runs on Linux … a weekly advisory run …» → the pushed sha's seven jobs read
  `verdict: green` (advisories included). Tools are checked first, homes are kept until the gates, uploads are
  scan-gated, and the last scheduled nightly run is recorded.
- **v1-03** «drift … annotations included turns the contract suite red …» → the drift contract replays every set
  green, a planted `annotations` byte reads `dialog_drift` red, and fixture hygiene refuses real paths, names and
  emails.
- **v1-04** «every process role writes …» → `run` / `hook` / `mcp` / `cli`, G4 green, null as a missing key, hook
  stderr and exit, and `logs` merge and filter with torn lines marked. The `ui` role left by the 2026-10-16 ruling.
- **v1-05** «… no setting disables redaction» → G2, G4, the canary, the print and raw-log bans, the forced-panic
  seam, and `cli_controls_not_disableable`'s identity-strip rows plus its completeness case over every config key.
- **v1-07** → coverage 85/95/80 and the five perf rows. Each of the five security-named parser surfaces has a seeded
  proptest and a fuzz corpus that fails on empty. `retries = 0`, and each job ends in one gate verdict.
- **v1-09** (by construction) → `v1-09-amendments.md` names the architecture fact for each of the five standing CLI
  amendments; the three web ones left by the 2026-10-16 ruling.
- **v1-10** → issue and outcome recoverable from `events.ndjson` alone, no wake of `wait`, the client-side refusal
  line. «The page and the tests» reads «the tests» by the 2026-10-16 ruling.
- **v1-13** **re-worded by the founder's word** (`inputs#I2`: «and MCP» dropped): «… the machine-readable and MCP
  results carry the same facts» → every human line of `release --budget` and `link` / `unlink` is built from its
  `--json` result.
- **v1-24** → the named source, never written, redirectable per home, and output passed through unchanged (its
  Windows half left with the Linux-only ruling).
- **v1-26** (manual) → `v1-26-fix-pass.md`: X8, X10 and X11, each with its test green; X13 left with the page.
- **v1-27** → the live wrapped start (screen, no viola byte, `cli_verified:true`, R8 strip, pinned copy, start
  order) and exit on the handle at the killed wrapper, beside each fake-agent witness.
- **v1-35** → Path 6 and the two-instance override; «unknown with its age» on the BAY line.
- **v1-53** → the CI `test` JUnit on `ubuntu-latest`, the census by its rule, and one live `viola run` of real claude
  2.1.287 confirming the Unix behaviour (openpty, the socket directory and modes, a clean fd table, the child gone
  after the kill).

## The words this card rests on
- `inputs#I1`: the operator directive.
- `inputs#I2`: the founder's own word on dialogs 1/7–7/7, and the overseer's T1–T3 (founder-delegated).
- `inputs#I3`: the founder's own word on the live cap.
- No escalate pattern applies: no boundary widening is shipped.

## Sampler
sampler: off

## Refusals met in this run, with the letter line followed
- The bash-guard `cd` arm refused a `cd` into a run dir (P2) and into a prior chunk's evidence dir (P3). Each was
  re-issued once with absolute paths. Letter: host-linux.md «anchor every cross-call path absolutely».
- The bash-guard filter arm refused a `rec.py get … | grep` (P3) and a `matrix.py --help | head` (P5). Each was
  re-issued bare. Letter: SKILL.md Setup 1, «a tool's stdout is never sent … through a filter».
- `fanout.py render --batch classify-p1 --respawn …` exited 2 (an invalid batch choice). It was re-fired as
  `--batch classify`, which rendered `classify-p1-r1`. Letter: fanout-contract.md §The batch 1a, «a part's agent is
  re-spawned from its part».
- No permission or classifier refusal was met.

## P5's landed tx
The outputs land at P5 step 5, after the word. Its `outputs land` line is reported with the run's report.

## Beyond the plan
| # | proposal | kind |
|---|----------|------|
| 1 | v1-11 deferred to 0.2.0. Reason: its subject is the page's live feed, which left 0.1.0; liveness has no log record by design. | deferral (his word, `inputs#I2`) |
| 2 | v1-21 deferred to 0.2.0. Reason: the MCP client's tool-call timeout default is unmeasured, and a `send`'s longest block (48.5 s plus the paste) has no bound under 50 s. | deferral (his word) |
| 3 | v1-34 deferred to 0.2.0. Reason: the harness-prefix and R8 identity-floor rows have no probed row and no ruled probe form. | deferral (his word) |
| 4 | v1-36 deferred to 0.2.0. Reason: `viola list` has no link column by his ruled design, `list --json` omits `links`, and the board or page that shows the set is 0.2.0's. | deferral (his word) |
| 5 | CARRY 10, the terminal-reply hypothesis (`CSI ? 6 n`, kitty graphics), carried to 0.2.0 unmeasured. | carry to 0.2.0 (his word) |
| 6 | CARRY 11's founder-attended tab-close leg carried to 0.2.0; its killed-wrapper leg runs here. | carry to 0.2.0 (his word) |
| 7 | CARRY 5, Measurement B, recorded as known with no action in 0.1.0; the pin re-hash stays the guard. | no action (his word) |
| 8 | If the near-ceiling hold reads red, the ceiling stays PROVISIONAL, owned by 0.2.0's first entry. | conditional owner |

Apply? (yes / review / cancel)
