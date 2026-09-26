# Codebase Research — 2026-09-26-ci-chunk-base-and-union-verdict

## Scope
- **Depth:** moderate · **Reads:** 9 (gate.rs 1–230 + 300–470, run/mutants.rs 25–335, secret_scan.rs 1–310, ci.yml 1–205, viola-pty/src/lib.rs 370–445, viola-harness.rs 180–195, the pty chunk's `evidence/operator-pass.md` 1–85, the cookbook) · **Globs/Greps:** 9
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (loaded on touching `crates/viola-e2e/**`), 3 Session Additions applied: `--in-diff` mutates only touched lines (a kill is witnessed by the new test's PASS, never by a green job); a diff with `#[cfg(unix)]` bodies runs on this host as `run --mutants --leg windows-2025`, verdict from the CI union; never pipe `agent-run.sh boot`.
- **Platform issues consulted:** not applicable — no failure signature exists to search: scope carries no runner-only bullet, every run since the last flip is green (Setup 5a: 36171941029, 36172787056), and the plan's CI-reading entries read this chunk's own future runs. The one recorded CI verdict (run 36165685381) was read from its own downloaded artifacts; its red is the union rule itself, not a platform fault.

## Files inspected
- `crates/viola-e2e/src/harness/gate.rs` (1–230, 300–470) — `gate(artifacts, require, legs)` (:57) dispatches `("mutants", Some(legs))` to `union` (:117–153): every named leg's `mutants-verdict-<leg>.json` must exist (else `artifact-missing`, and no mutant is judged); per name, red iff no leg `caught` and some leg `missed|timeout`. Unviable never enters the predicate. Tests: `gate_union_is_red_only_where_no_leg_caught_the_mutant` (:411), `gate_union_needs_every_named_leg` (:445), helper `leg()` (:398) writes `{"v":1,"leg","verdict":"counted","mutants":[{name,outcome}]}`.
- `crates/viola-e2e/src/harness/run/mutants.rs` (25–335) — `resolve_base` (:34): `AGENT_RUN_CHUNK_BASE` (trimmed, non-empty), else `git merge-base HEAD origin/main`; `None` unless the result is a commit. `mutants()` (:227) deletes the stale `chunk.diff` and the leg file first, then `base-missing` on `None` (:238–242), writes `target/agent-run/chunk.diff` (:233, :242), classifies `no-rust-delta` / `test-only-rust-delta` / `counted`. `leg_verdict` (:199) reduces `outcomes.json` to names + outcomes (`caught|missed|timeout|unviable`, :187–195). `mutants_suite` (:90) keeps the per-leg `unviable > caught` red.
- `crates/viola-e2e/src/harness/secret_scan.rs` (1–310) — `Roots::of` (:47): `e2e_home`, `agent_run = target/agent-run`, `junit`. `scan` (:62) walks EVERY file under `agent_run` via `collect_files` (:78, :205 — recursive, symlinks not followed) with the Critical classes, cookie pairs and GUI tokens; the content canary only in role files. Planted-secret tests exist: `secret_scan_flags_every_critical_class_even_in_a_detail_file` (:308), `secret_scan_descends_into_the_capture_tree` (:353), `secret_scan_hit_locates_without_the_matched_bytes` (:422).
- `.github/workflows/ci.yml` (1–205) — `mutants` job: checkout `fetch-depth: 0` (:155), env `AGENT_RUN_CHUNK_BASE: ${{ github.event.pull_request.base.sha || github.event.before }}` (:166) passed through `env:` to `run --mutants --leg "$LEG"`. `mutants-verdict` job: `needs: mutants`, `if: always()`, ubuntu, a checkout of the SAME sha (no fetch-depth), download of both leg files, `gate --require mutants --mutants-legs ubuntu-latest,windows-2025` (:200). `test` job: the `harness-<os>` upload ships the WHOLE `target/agent-run/` (:110–117), gated on `secret-scan` success. No a11y steps exist in `ci.yml` today (grep `e2e-web|playwright|--browser` over ci.yml: 0 hits; `e2e-web/` absent from the tree), so the a11y extract's step-order constraint has nothing to preserve yet. No `concurrency:` key.
- `crates/viola-pty/src/lib.rs` (370–445) — `HostTerminal::enter` twice: `#[cfg(windows)]` at :393–394, `#[cfg(unix)]` at :417–418 (fn-level attribute); `Drop` uses STATEMENT-level `#[cfg(windows)]` / `#[cfg(unix)]` (:435, :442). `HostTerminal` has no `Default`.
- `crates/viola-e2e/src/bin/viola-harness.rs` (180–195) — `gate_cmd(ws, require, artifacts, mutants_legs)` holds `ws` (the workspace root) and calls `gate(&artifacts, &require, legs)`; the env read is at :150.
- `viola-0.1.0/chunks/2026-09-25-pty-wrapper-on-windows/evidence/operator-pass.md` (1–85) — run 36165685381 on 17ea8c7, base 0253507: legs green, `mutants-verdict` RED on exactly two names, `lib.rs:395:9` and `lib.rs:420:9` `replace HostTerminal::enter -> Option<Self> with Some(Default::default())`; the leg readings there were "from the tree, not a separate measurement".

## Measured at this phase
- **The recorded breach shape** (downloaded: `gh run download 36165685381 -n mutants-verdict-{ubuntu-latest,windows-2025}` → `.andromeda/runs/2026-09-26T19-33-34-phase/run-36165685381/`): ubuntu-latest `counted`, 186 mutants (caught 139 · missed 17 · unviable 30); windows-2025 `counted`, 186 (caught 152 · missed 4 · unviable 30). `lib.rs:395:9 … Some(Default::default())`: windows **unviable**, ubuntu **missed**. `lib.rs:420:9 … Some(Default::default())`: ubuntu **unviable**, windows **missed**. The union witness replays these two records verbatim.
- **Outcomes alone cannot carry the rule.** A windows-only body reads `missed` on ubuntu because it is compiled out; a shared body missed by a leg that compiles it also reads `missed`. So "unviable somewhere, missed elsewhere" is the breach shape AND the negative's shape; only knowing which legs COMPILE the mutated line separates them. `unviable` on a leg does prove that leg compiles the line (a build failure needs compilation).
- **cfg shapes in lib/bin code** (`grep -rhoE '#!?\[cfg\([^]]*\)\]' src crates/*/src --include=*.rs | sort | uniq -c`): `test` 27 · `unix` 14 · `windows` 13 · `not(unix)` 3 · `not(windows)` 2 — five predicate forms, no `target_os`, no `feature`, no `all/any`. cfg-gated `mod x;` declarations: 0 (two greps: same-line and previous-line). `cfg_attr`: 1, `#[cfg_attr(test, mockall::automock)]` (viola-pty/src/lib.rs:113), which gates no code. `#![cfg]` inner attributes: 0.
- **The `-G` edge** (scratch probe `base_probe.sh`, a throwaway repo: flip → `chore(b): operator pre-CI commit` → code-only fix → fix editing a `· complete ·` line → wrap): the pickaxe from HEAD reads the flip until the complete-line edit, then MOVES to that fix commit (`008462c`); the pickaxe bounded at the oldest pre-CI commit's parent holds the flip (`b8fe9c2`) through every commit of the pass; after the next wrap both read the new flip. The hypothesis is measured true and the bounded rule holds.
- **The pickaxe over this repo** (`git log --format='%h %ad %s' -G ' · complete · ' -- .andromeda/master-route.md`): 11 hits, exactly the 11 chunk wrap commits (b0236ca … fcca1ce); neither 0-pending adaptation commit (9df9e45, e486544) nor any pre-CI commit matches.
- **Parser availability** (`Cargo.lock`): `syn` 2.0.119 and 3.0.6, `proc-macro2` 1.0.107, `quote` 1.0.47 already resolved (proc-macro deps); none is a `[workspace.dependencies]` entry (`grep syn|proc-macro2 Cargo.toml`: 0). Licences MIT OR Apache-2.0 (in `deny.toml` `allow`); no ban names them (`deny.toml` `[bans] deny`: cc, libsqlite3-sys, openssl-sys, opentelemetry-*, sentry, tracing-appender).
- **Local residue** (`ls target/agent-run/`): `artifacts/` plus dozens of `cli-drive-*` session dirs; the scan walks all of it.

## Graph impact (rust plane, `db_state: fresh`; trace `tree-query-2026-09-26-ci-chunk-base-and-union-verdict.json`, 48 rows)
- **resolve_base** — 1 production caller, `mutants()` @ mutants.rs:239; re-exported @ run.rs:26; 4 test sites in `resolve_base_needs_a_real_commit` (mutants.rs:388–394).
- **union** — 1 caller, `gate()` @ gate.rs:63. **gate** — 1 production caller, `gate_cmd()` @ viola-harness.rs:194; 15 test sites in gate.rs (:281–477). A signature change of `gate` threads through all 16.
- **scan** — `secret_scan()` @ secret_scan.rs:59 + 13 test sites (:302–484). **collect_files** — `scan()` :78, `gui_tokens()` :179, itself :214. **secret_scan** — `main()` @ viola-harness.rs:233.
- **chunk_diff** — `mutants()` :240, run.rs:25 (re-export), 2 test sites. **mutants** — `tool_arms()` @ run.rs:181.
- Every caller is inside `viola-e2e`; no product crate is touched (crate-edge check not needed: `viola-e2e` is a leaf test-only crate, arch §Module Boundaries).

## Patterns detected
- **Fresh artifact, never stale** (mutants.rs:233–236): the leg deletes `chunk.diff` and its leg file before any early exit, so a stale file never reads as this run's.
- **Fixed-code breaches** (gate.rs:53–55): `{"gate","suite","detail"}`, detail = a code, a count or a mutant name.
- **Temp git repos in unit tests** (`git_repo()` run.rs:274, `mini()` run.rs:297): the base rule's tests build history there; nothing nests cargo in nextest (the runner seam).
- **Per-OS function pairs are avoided** in harness code (secret_scan.rs:231–244 comment): one body per OS, or cargo-mutants leaves the other body unkillable. New harness code keeps that shape.

## Conventions to follow
- **Harness env vars** are `AGENT_RUN_`-prefixed and read only by `viola-harness` (viola-harness.rs:150); `AGENT_RUN_CHUNK_BASE` is the registered one.
- **Pinned workspace deps**: a new dependency is a `[workspace.dependencies]` pin consumed as `x.workspace = true` (crates/viola-e2e/Cargo.toml).
- **viola-e2e prints by design** and carries its own `[lints]` (verification-harness.md §Exemptions).

## New files to create
- none required by research; tests land inline (`#[cfg(test)]` modules in gate.rs, mutants.rs, secret_scan.rs), as every existing harness test does.

## Files to modify
- `crates/viola-e2e/src/harness/run/mutants.rs` — `resolve_base`: the master-flip rule (bounded at the oldest pre-CI commit's parent), the env value as explicit override; tests.
- `crates/viola-e2e/src/harness/gate.rs` — `union`: judge each mutant only on the legs that compile its line; `gate` gains the source root; the 15 test call sites.
- `crates/viola-e2e/src/bin/viola-harness.rs` — `gate_cmd` passes `ws.root` to `gate` (:194).
- a new cfg module in `viola-e2e` (harness) — which legs compile a `file:line` (P4 names it).
- `crates/viola-e2e/Cargo.toml` + root `Cargo.toml` `[workspace.dependencies]` — the parser pin (P4's decision).
- `crates/viola-e2e/src/harness/secret_scan.rs` — the scan skips the mutation leg's `chunk.diff`; tests.
- `.github/workflows/ci.yml` — drop the event-derived `AGENT_RUN_CHUNK_BASE` (:166); `harness-<os>` upload excludes `target/agent-run/chunk.diff` so the scan still covers everything uploaded (:115).
- Companion sweep (`grep -rn` over `crates/ src/ scripts/ .github/`, re-derived): `AGENT_RUN_CHUNK_BASE`: 4 hits · 4 changed (ci.yml:166 removed; viola-harness.rs:150 read kept; the mutants.rs:33 and run.rs:80 doc comments restated) · `mutants-legs|mutants_legs`: 7 hits · 0 changed · 7 no-change (the CLI surface and its parsing are unchanged: ci.yml:200, tests/cli.rs:83, viola-harness.rs:68/183/188/237/238); the `gate(...)` call at viola-harness.rs:194 changes as a `gate` caller (graph) · `chunk.diff`: 1 file (mutants.rs, no change) · secret_scan.rs gains its first mention.
- Expected amendments (wrap, not touchpoints): test-plan §3 `run` step 4 Base, §3 `gate` union bullet, §3 `secret-scan` Scope, §6 Canary bullet, §9 Mutation row, §10 Mutation gate, §12 entry; architecture §CI/CD approach (base + union sentences) and §Occupied Resources (`AGENT_RUN_CHUNK_BASE` meaning); obs-plan §9 Mutation row + §9 step 3 scope.

## Open questions
- How the gate learns which legs compile a mutant's line: a `syn`-based cfg evaluator over the checked-out source (gate-side, leg file unchanged), or a text scan, or leg-side annotation → blocks: plan-decision (P4).
- Whether `AGENT_RUN_CHUNK_BASE` stays as an explicit override or is retired → blocks: plan-decision (P4).
