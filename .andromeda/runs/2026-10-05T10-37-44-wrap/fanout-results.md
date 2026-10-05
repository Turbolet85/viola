# Fan-out results — wrap of 2026-10-05-real-cli-verify-probes

Seven Explore doc-agents, one parallel batch, prompt from `amendment-flow.md` sent verbatim. The detector counts are
arch 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2, which is 15
and equals the drift-base `doc:` names. Keyed contracts were rendered for arch, test-plan, obs-plan and a11y-plan; the
other three print `n/a` (their line was dropped). Entity probe on every return: `entities=0` (no `&lt;` / `&gt;` /
`&amp;` in any return).

## Verdict lines
- **architecture:** 23 proposals (10 primary, 13 `dependent-of`).
- **security-plan:** `proposals: []`. Stripping removed the per-detector commentary: no drift on input, auth or deps.
  One P2 note: the §Input Validation Constants line (`:243`) lists only the `--version` reads as `MAX_FRAME`
  child-output consumers, not verify's typed PTY runs. Raw twin: `.raw-fanout-security-plan.md`.
- **design-system:** `proposals: []`. Stripping removed the commentary: tokens all `n/a`, and the `/06` counter,
  six-row and `2.1.283` items are count changes outside the token detector. Raw twin: `.raw-fanout-design-system.md`.
- **layout-templates:** 4 proposals (3 primary, 1 dependent).
- **test-plan:** 12 proposals (4 primary, 8 dependent).
- **obs-plan:** 5 proposals (1 primary, 4 dependent). Stripping removed the commentary: D-obs-stack and D-obs-pii
  show no drift, and the three obs `provisional` hits concern the hook deadlines, not the screen constants.
- **a11y-plan:** `proposals: []`. Stripping removed the commentary: no interactive UI, the a11y row schema does not
  use `subject`, and §4 P6 states the generic `[NN/NN]`. Raw twin: `.raw-fanout-a11y-plan.md`.

## Parsed proposals and dispositions

Every disposition reads `apply` · `reject` · `escalate`, with the check that decided it. Validate check 0 (the
re-derivation tell): A16, A22 and A23 carry child flags (`--model haiku`, `--plugin-dir <probe>/plugin`) for the
interactive runs that the report does not state. The collateral is dropped and the report's facts are applied.

### architecture
| id | detector | section | change (summary of the returned line) | dep | disposition |
|---|---|---|---|---|---|
| A1 | D-arch-resources | §Occupied Resources → Filesystem | register Run A `<OS temp>/viola-verify-*` and Run B `<cwd>/.viola-verify-<pid>/`, both 0700, removed on every exit path, `/.viola-verify-*/` gitignored | — | apply (playbook "Accurate this-chunk addition"); the widening itself is E1 |
| A2 | D-arch-resources | §Occupied Resources → Repository | register `schemas/claude-screen.v1.json` (modal\|ready\|turn, cols 80, 24 rows) | — | apply (accurate addition) |
| A3 | D-arch-resources | §Occupied Resources → Repository | `fixtures/claude/<v>/` gains `Screen.<phase>.json` (signature rows only, refused whole and named); stamped sets 2.1.287 + 2.1.288, 2.1.283 drift-only; relayed-fixture supersession owned by "Dialog rows and re-probe" | — | apply (accurate addition; check 5 [CLI Version Compatibility] floor) |
| A4 | D-arch-resources | §Occupied Resources → Filesystem `diagnostics/` | verify logs four spawn pairs (`version-probe`, `verify-probe`, `verify-pty-probe` ×2) | — | apply |
| A5 | D-arch-resources | §Standard Contracts → Ledger stamps envelope | `measured` gains `typed_probe {ready_settle_ms, turn_settle_ms, prompt_latency_ms, max_turn_gap_ms}`; `run` reads no number from it | — | apply; the example-line collateral ("the example covers the ten rows") is dropped, the example stays a valid illustration |
| A6 | D-arch-resources | §Occupied Resources → `ledger/stamps.json` | measured values now include `typed_probe` | A5's | apply |
| A7 | D-arch-resources | §Occupied Resources → Binary | fake agent `--trusted-root` / `--screens` / `--turn-stop` (argv, no env); verify drives them in CI | — | apply |
| A8 | D-arch-resources | §Infrastructure Patterns → CI/CD approach | CI verify drives print mode plus the interactive screen modes | dep | apply |
| A9 | D-arch-resources | §Infrastructure Patterns → Project directory structure | `schemas/` lists `claude-screen.v1.json` | dep | apply |
| A10 | D-arch-resources | §Infrastructure Patterns → Project directory structure | `fixtures/claude/<v>/` holds screen fixtures too | dep | apply |
| A11 | D-arch-resources | §Infrastructure Patterns → Project directory structure | `src/cmd/` note for `verify/typed.rs` | — | apply (the tree already annotates sub-files, `client.rs`; not registry over-reach) |
| A12 | D-arch-decisions | §Established Decisions → [Screen Model] | `SIGNATURES` compiled with two modal literals; constants compiled and validated per version (no longer PROVISIONAL); full gate on a verified CLI; `Screen::rows()` verify-only; trust inherited from a trusted parent, external imports keyed on the git root | — | apply (check 5 floor; disproved claim 2) |
| A13 | D-arch-decisions | §Design Philosophy | partial gate on unverified builds only | dep | apply |
| A14 | D-arch-decisions | [Human Takeover / Wheel] | `GATE_MAX_WAIT` 5 s no longer provisional | dep | apply |
| A15 | D-arch-decisions | [Delivery Confirmation] | window compiled, validated by `confirm-window`; local-command rows owed to "Local-command and paste-framing rows" | — | apply (check 5 floor) |
| A16 | D-arch-decisions | [CLI Version Compatibility] | ten rows; `check(row, &Probes)`; verify's Run A / Run B; `--record` screens + named refusal; owed rows re-pointed to the two new entries | — | apply, the child-flag collateral dropped (check 0) |
| A17 | D-arch-decisions | [CLI Version Compatibility] rows list | cross-session `prompt` measurement owed to "Local-command and paste-framing rows" | dep | apply |
| A18 | D-arch-decisions | Cross-cutting → Capability ledger | the dialog exception rides the ten-row stamp until "Dialog rows and re-probe" | dep | apply |
| A19 | D-arch-decisions | §Stack → Screen model row | vt100 also serves verify's interactive runs; `Screen::rows()` the only row-text exit, verify alone | — | apply |
| A20 | D-arch-decisions | §Infrastructure Patterns → Crate dependency direction | viola-agent-claude's landed API: ten rows, `SIGNATURES`, typed-probe helpers, constants not provisional | dep | apply |
| A21 | D-arch-decisions | §Infrastructure Patterns → Crate dependency direction | root bin lists `tempfile` (`=3.27.0`, Run A's dir) | — | apply (Dependencies bullet carries it) |
| A22 | D-arch-decisions | [Plugin Scope] | verify's two interactive runs join the print probe as unwrapped exceptions; Run B runs the user's global hooks and status line | — | apply, collateral dropped (check 0); the widening itself is E1 / E2 |
| A23 | D-arch-decisions | §Occupied Resources → Claude Code integration names | register verify's interactive children | dep | apply, collateral dropped (check 0) |

### layout-templates
| id | detector | section | change | dep | disposition |
|---|---|---|---|---|---|
| L1 | D-layout-surface | §Surface: cli → `viola verify` wireframe | ten step lines `[01/10]`…`[10/10]`, summary `10 pass` | — | apply (check 5 floor) |
| L2 | D-layout-surface | same, ledger-order prose | ten rows listed, `MM` = 10 | dep | apply |
| L3 | D-layout-surface | same | `viola verify --help` paragraph region | — | apply (check 5 floor) |
| L4 | D-layout-surface | §Surface: cli → refusal lines | the named `--record` refusal replaces the fixed text | — | apply; the widening itself is E3 |

### test-plan
| id | detector | section | change | dep | disposition |
|---|---|---|---|---|---|
| T1 | D-tests-obs-harness | §3 → 5-command implementation (`--local-live`) | ten literal row ids, unit-proven; live firing owed to "First live test and self-drive" | — | apply (check 5 floor) |
| T2 | D-tests-obs-harness | §3 → 5-command (`boot` step 4) | verify with `--screens --turn-stop --trusted-root <ws root>`, ten rows | — | apply |
| T3 | D-tests-obs-harness | §3 → 5-command (`boot` step 5) | supervise passes `--screens --trusted-root <cwd>` | — | apply |
| T4 | D-tests-obs-harness | §5 Integration (CLI) | `cli_verify` 22 cases, `[NN/10]`, interactive runs, named refusal, no run dir left | dep | apply |
| T5 | D-tests-obs-harness | §5 Integration (CLI) | `contract_ledger_probes`: 2.1.287 + 2.1.288 stamped at ten, 2.1.283 drift-only | dep | apply (check 5 stamp walk) |
| T6 | D-tests-obs-harness | §7 Fake agent | the three new options | dep | apply (check 5 floor) |
| T7 | D-tests-obs-harness | §7 Fixture hygiene walk | 2.1.288 set; `Screen.*.json` walked against `claude-screen.v1.json` | dep | apply |
| T8 | D-tests-obs-harness | §7 Fixture hygiene | email-shaped token and seam-split username refused | dep | apply |
| T9 | D-tests-obs-harness | §7 Fixture hygiene (recorder) | screens signature-rows only, scrub-as-detector, named refusal | dep | apply |
| T10 | D-tests-obs-harness | §2 Test pyramid, Contract row | screen fixtures checked against `claude-screen.v1.json` | dep | apply |
| T11 | D-tests-obs-harness | §4 Unit (viola-agent-claude) | signatures also feed verify's settle and record helpers; full gate on a verified CLI | dep | apply |
| T12 | D-tests-framework | §3 → Bootstrap phases (nextest) | `profile.mutants` gains `test(/verify_window_/)` 15 s × 2 | — | apply |

### obs-plan
| id | detector | section | change | dep | disposition |
|---|---|---|---|---|---|
| O1 | D-obs-instrumentation | §4 Edge flows → `verify` | four child spawn pairs | — | apply (check 5 floor) |
| O2 | D-obs-instrumentation | §6 Child / shell spawns | subject set gains `verify-pty-probe`; four spawns | dep | apply (check 5 floor) |
| O3 | D-obs-instrumentation | §6 Child / shell spawns, schema | `$defs.subject.enum` lists `verify-pty-probe` | dep | apply |
| O4 | D-obs-instrumentation | §6 event table → `process-start` | subject list gains `verify-pty-probe` | dep | apply |
| O5 | D-obs-instrumentation | §4 Edge flows → `verify`, CI | CI verify covers the two typed runs | dep | apply |

### Raised by the orchestrator (check 5, the plan's expected amendments no detector proposed)
| id | doc | section | change | disposition |
|---|---|---|---|---|
| R1 | security-plan | Threat Model Summary → Child process spawning | verify's two PTY children against the live `claude`; no byte into either CLI dialog; Run A killed, Run B Ctrl-C ×2 then kill | **escalate E1** (playbook "Boundary widening": a new subprocess crossing) |
| R2 | security-plan | Data Protection → Probe captures / Repository fixtures | the two probe dirs (the user's cwd and the OS temp root), signature-only screen fixtures, the seam and email refusal, the named refusal codes | **escalate E1 / E3** (writes outside the viola home; an error body naming a file and a code) |
| R3 | security-plan | Data Protection (accepted risk) | the Run B residual: a synthetic-prompt transcript under `~/.claude/projects/`; the user's global hooks and status line run in Run B | **escalate E2** |
| R4 | security-plan | Threat Model / Secret Management (dev-host prerequisite) | the dev host's external-imports answer, set by the overseer on the founder's amended live ruling | **escalate E2** |
| R5 | security-plan | Anti-Patterns → Universal (`:604`) | the dated gap now rides the ten-row stamp (counter `/10`) until "Dialog rows and re-probe" | apply (accurate addition; the gap's widening was ratified R2, its owner re-pointed by the split) |
| R6 | security-plan | Input Validation → Constants (`:243`) and child-output row (`:237`) | `MAX_FRAME` also caps verify's typed PTY output | apply (accurate addition, a cap) |
| R7 | design-system | §Surface: cli → `viola verify` | `[NN/10]` counter, ten rows, the help paragraph, `2.1.283` example re-read | apply (check 5 floor) |
| R8 | a11y-plan | §4 P6 | the counter | no change: `:349` and `:587` state the generic `[NN/NN]` with no literal `/06` |

### Disproved claims (check 6)
1. The record command's `--home "$h/home"` (plan entries 7 and 8) is disposed as a recorded plan correction (chunk
   artifact) plus curation (the `/home/` component hazard). The masters were swept for `--record` command text and
   none quotes it.
2. "A probe dir under a trusted parent starts with no modal" is applied in A12 (trust is inherited; external imports
   are keyed on the git root).
3. The plan's gate order is a deviation and no master states it, so it is disposed.

## Escalations
- **E1** (R1, R2, A1, A22, A23): verify's PTY crossing and its two probe dirs.
- **E2** (R3, R4): the Run B residual and the dev-host external-imports answer.
- **E3** (R2 half, L4): the named `--record` refusal.

## Escalation resolutions (2026-10-05, AskUserQuestion at this wrap's P2)
- **E1 ratified:** the founder's existing live rulings R-S2 (~00:00Z), "two runs, never accept" (~07:00Z, re-affirmed
  ~08:50Z) and the two dirs (08:25Z), each answered after the widening was shown, relayed by the overseer. The
  operator's word: "record them as the founder rulings, relayed by the overseer". R1, R2 (dir half), A1, A22 and A23
  apply.
- **E2 accepted and recorded:** the overseer's word, relayed by the operator: the founder saw this residual class at
  the trust ruling, and the flag edit was his amended live ruling (~09:05Z). Both are recorded as accepted, with the
  dev-host prerequisite. R3 and R4 apply.
- **E3 ratified:** the founder ruled live at this wrap (2026-10-05, through the overseer's AskUserQuestion, with the
  closed code set and the no-content rule shown). It is recorded as the founder's live ruling. R2 (refusal half) and
  L4 apply.
- **Open escalations: 0.**
