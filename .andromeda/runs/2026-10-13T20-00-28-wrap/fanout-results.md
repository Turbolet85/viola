# Fan-out results — 2026-10-13-interrupted-verify-cleanup

Path check: before `tree 0f58249e5ed9 index afa0f9c23493 head c22161e791ff`; after `tree 557414446cb8`, index and
head unchanged; `git diff-tree` printed exactly seven `A` lines, one `fanout-{doc}.md` per agent. No fix, no twin, no
re-spawn. Each receipt's count equals its file's.

architecture: 7 proposals · fanout-architecture.md
note: the Project directory structure tree lists `verify/typed.rs` but not `verify/stop.rs`; [CLI Version Compatibility] (`:91`) describes Run B's Ctrl-C ending without the stop path.
  1. §Occupied Resources → Filesystem (`:413`, the interactive probe dirs) — apply (playbook "Accurate this-chunk addition", routine; expected amendment 1)
  2. §Occupied Resources → Filesystem (`:412`, `ledger/probes/<pid>/`) — apply (routine; dependent of 1)
  3. §Occupied Resources → Filesystem, `diagnostics/` (verify's interrupted self exit; a stopped child's exit without `child_exit_status`) — apply (routine)
  4. §Conventions → CLI exit codes, the `1` item — apply (routine; expected amendment 3)
  5. §Occupied Resources → Claude Code integration names (`:371`, the owed-move clause retired) — apply (routine; expected amendment 2)
  6. §Infrastructure Patterns → Crate dependency direction (the root's `libc` `sigaction` and `windows-sys` `SetConsoleCtrlHandler` roles) — apply (routine; expected amendment 4)
  7. §Infrastructure Patterns → CI/CD approach (the matrix 9 → 10 items, root 4 → 5) — apply (routine; playbook "A count in a master that carries no rule" — the key states its rule, `- package:` items; report Counts)
  8. raised by main — §Infrastructure Patterns → Project directory structure: the `src/cmd/verify/` lines gain `stop.rs` beside `typed.rs` — apply (routine, the agent's note; report Crates / modules)
  9. raised by main — [CLI Version Compatibility] (`:91`, Run B's ending) and its stamping sentence: a run stopped by Ctrl-C or TERM is ended by a kill with no key, starts no later run, and writes no stamp — apply (routine; check 4 read at offset 2736; expected amendment 1's `stop-signals` site)

security-plan: 0 proposals · fanout-security-plan.md
note: names security-plan `:357` (the matrix count) as stale, and expected amendment 5 (`:131`, `:278`, `:282`, `:283`) as plan-carried with no proposal.
  1. raised by main — §Data Protection, Interactive probe dirs (`:283`): removed on a pass, a fail and a Ctrl-C / Ctrl-Break / SIGINT / SIGTERM stop; SIGKILL, a crash, a failed install and a Windows close / logoff / shutdown leave them by hand — apply (check 5; expected amendment 5)
  2. raised by main — §Data Protection, Probe captures (`:282`): the same exit paths and exceptions — apply (check 5; dependent of 1)
  3. raised by main — §Data Protection, Code-bearing artefacts (`:278`): the clause owing the `--settings` move to "Interrupted verify cleanup" retired, Runs C and D read `SETTINGS_FLAG` — apply (check 5; expected amendment 5)
  4. raised by main — the Run B ending (`:131`): a stopped run is ended by a kill with no Ctrl-C — apply (routine; dependent of architecture 9)
  5. raised by main — §Dependency Security, CI integration (`:357`): a ten-item matrix, the root `viola` package five of them — apply (routine; report Counts)

design-system: 0 proposals · fanout-design-system.md
note: expected amendment 8 is outside D-design-tokens.
  1. raised by main — §Surface: cli → Exit-code phraseology, row 1: a `viola verify` stopped before it finished, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` — apply (check 5; expected amendment 8; a planning form on the operator's word, `inputs#I2`, the founder's wording owed)
  2. raised by main — §Surface: cli → Component Patterns 5 (`verify`'s pattern): the interrupted form — apply (check 5; dependent of 1)

layout-templates: 3 proposals · fanout-layout-templates.md
note: `:590`, `:520`, `:535`, `:539`, `:541`–`:545` hold unchanged.
  1. §Surface: cli → Output structure — `viola verify` (`:496`) — apply (routine; expected amendment 9)
  2. §Surface: cli → Component — Primary content block 2 (`:588`) — apply (routine; dependent of 1)
  3. §Surface: cli → Component — Footer / terminator (`:598`) — apply (routine; dependent of 1)

test-plan: 2 proposals · fanout-test-plan.md
note: D-tests-framework and D-tests-obs-harness read no drift; `:1226`'s «all nine jobs» is a past run's reading and stays.
  1. §5 Integration Test Strategy → CLI, the `tests/cli_verify.rs` bullet (`:656`) — apply (routine; expected amendment 11)
  2. §9 CI Integration → Pipeline structure, the Mutation row (`:1163`) — apply (routine; report Counts)

obs-plan: 4 proposals · fanout-obs-plan.md
note: D-obs-stack and D-obs-pii read no drift; `:308`, `:621` and the two keys name `run` / `revive` causes and stay.
  1. §6 Log Coverage → `detail` code catalog (`:834`), `interrupted`, 11 → 12 — apply (routine; expected amendment 6)
  2. §4 Edge flows, the `verify` bullet (`:739`) — apply (routine; expected amendment 7)
  3. §6 Log Coverage → Child / shell spawns, a stopped child's `process-exit` — apply (routine; dependent of 2)
  4. §4 Edge flows, the `--capture` sub-bullet (`:740`) — apply (routine; expected amendment 7)

a11y-plan: 0 proposals · fanout-a11y-plan.md
note: expected amendment 10 is outside both detectors.
  1. raised by main — §1 Surface: cli → Notes (`:90`): verify's interrupted exit-1 refusal is one `unable:` line and one `hint:` line last — apply (check 5; expected amendment 10)
  2. raised by main — §8 Error recovery, CLI (`:838`): the interrupted pair takes pattern 2, and the stop path adds no time bound — apply (check 5; dependent of 1)

## Checks
- **1, playbook.** Every proposal and raised item matches "Accurate this-chunk addition" (routine): its symbols and
  values are in the report's Changes. The matrix counts also match "A count in a master that carries no rule" — each
  site states its rule, so the count is moved with the rule kept. Not a boundary widening: the stop takes no data
  (`stops_on` 0 | 1, Unix SIGINT / SIGTERM), adds no IPC, file or env crossing, and its only effect is the existing
  guards' removal on a new exit path. The security-plan detector read no drift.
- **2, cross-contradiction.** None: every edit adds the stop path to a section; no two proposals pull one section
  apart.
- **3, intent.** The scope record's one line (companion, the operator's word) is justified; the entry-9 deviation is
  the operator's direction, recorded.
- **4, absence needs evidence.** The retired claim «every exit path» was swept over the seven masters and the
  registries (`every exit path|on every exit`): 5 hits. architecture `:412` and `:413` are proposals 2 and 1;
  security-plan `:282` and `:283` are raised 2 and 1; obs-plan `:740` is proposal 4. Long lines were read by offset
  (`cascade.py window`: security-plan `:131` @1071, architecture `:91` @2736).
- **5, expected amendments.** All eleven are covered: 1 → architecture 1 / 2 / 9 · 2 → architecture 5 · 3 →
  architecture 4 · 4 → architecture 6 · 5 → security-plan 1–4 · 6 → obs-plan 1 · 7 → obs-plan 2–4 · 8 →
  design-system 1–2 · 9 → layout-templates 1–3 · 10 → a11y-plan 1–2 · 11 → test-plan 1. No `claim false` citation
  row.
- **6, disproved claims.** The report's one entry (the plan's preservation guard against the contract test) is routed:
  plan text, the operator's wrap direction recording it as a deviation, and the pipeline lesson to curation (P3). No
  master states it.

Rejected for a source the report does not carry: 0.
