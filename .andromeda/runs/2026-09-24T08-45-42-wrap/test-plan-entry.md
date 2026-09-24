
## 2026-09-24-fake-agent-and-test-data-fixtures — mutation verdict for Rust-free diffs
**Section:** §3 `run` step 4 (Classification bullet added; Verdict scoped to the `counted` arm) · §3 `run` Output format (`mutants` object) · §3 Closed enums (`mutants.verdict`) · §10 Mutation gate · §12 Decisions Log (new entry)
**Change:**
- The harness classifies `chunk.diff` by its `diff --git` headers.
- A diff with no `.rs` path (an empty one included) never reaches cargo-mutants. It passes as `{"tested":0,"verdict":"no-rust-delta","diff":"target/agent-run/chunk.diff","files":N}`.
- A Rust delta deletes a stale `mutants.out/outcomes.json` first and reports `"verdict":"counted"`. With no fresh `outcomes.json` it is red.
- New closed value `counted` | `no-rust-delta`.
- §10's "tests-only diff = `{"tested":0}`" is retired.

**Why:** report Spec claims disproved 3: cargo-mutants 27.1.0 exits 0 on a Rust-free diff and leaves `mutants.out/` untouched. CI run 35973118026 (sha dc01bd9) read `outcomes-missing`, and the phase P5 baseline on the dev host read a stale `"tested":8`. Operator P1 constraint. Sweep over all seven masters (`"tested":0}`, `touches only tests`, `parse \`mutants.out/outcomes.json\``): 2 test-plan sites amended (:557 Verdict, :1445 §10); test-plan :553 (base-missing rationale) and obs-plan :1237 (artifact list) no change.

## 2026-09-24-fake-agent-and-test-data-fixtures — fake-agent contract as built, consumer-first
**Section:** §7 Fake agent · §7 Fixture hygiene · §7 seed table (recorded-payload row) · §3 `run` step 2 · §12
**Change:**
- Hooks are read from `<plugin-dir>/hooks/hooks.json`, not `plugin/` + `settings.json`. Only an absolute exec-form command runs, and matchers are not yet evaluated.
- Payload `<fixtures>/<cli-version>/<Event>.<variant>.json`, with only `prompt` set for UserPromptSubmit.
- Receipt kinds and fields listed; script schema `schemas/fake-script.v1.json`.
- Modes built vs deferred: `--vt100-panic-bytes`, `statusline-echo`, `agents --json` land with their consumers.
- The hygiene walk covers `fixtures/fake-scripts/*.json`, with the class-only checker. The `fixtures/claude` walk joins with the first recorded fixture.
- Only the sync root chain exists (`viola-test-*` homes); the `viola_e2e::fixtures` copy lands with its first consumer, and the root `stamped_home` is an interim no-stamp seam.

**Why:** report Symbols / APIs, Schema / config, Crates / modules; operator P4 "consumer-first, no shapes invented before a recorded fixture"; plan expected amendments. Sweep `plugin/\` and \`settings.json\``, `exists twice`, `In both copies`, `FAKE_CLAUDE_AGENTS_MODE`, `statusline-echo`, `vt100-panic-bytes` over all seven masters:
- :1324, :542 amended.
- :229, :349 no change (`viola run` rewriting its own plugin folder).
- :514, :1101, :1149, :1315 no change (sequencing: target state; owners pinned as CARRYs at this wrap's route-resolve).

## 2026-09-24-fake-agent-and-test-data-fixtures — fixture naming and `--fixtures` root (operator-resolved escalations)
**Section:** §2 File naming · §7 seed table · §3 `boot` step 5 · §6 Path 4 step 1 and scenario step 3
**Change:**
- Fixtures are named `<Event>.<variant>.json` with the CLI's PascalCase hook event name (`PreToolUse.ask-question.json`), with no mapping table.
- `boot` passes the parent `--fixtures <root>/fixtures/claude`, and the fake agent joins `<cli-version>`.
- `--plugin-dir` comes from `viola run` (architecture §Occupied Resources).

**Why:** the fan-out escalated two doc-vs-code shape conflicts. The operator (wrap P2) resolved them with "code wins where the doc invented a shape; no mapping tables". Sweep `hook-event>`, `pre-tool-use\.ask`, `fixtures/claude/<ver>` over all seven masters:
- 4 sites amended (:467, :1048, :1133, :1315 row).
- :519 amended (`--fixtures`).
- :828, :930, :954, :1296 no change (directory references to a version dir, not the argument).
