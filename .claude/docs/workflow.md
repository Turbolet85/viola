# Development Workflow

_Extracted from architecture.md and project conventions by `/andromeda-setup-project`. Reference for how to actually work on this project day-to-day._

## Git workflow
- **Branching:** one long-lived build branch per version — `build/viola-0.1.0` for this version, then `build/viola-X.Y.Z`. Chunks commit on it; no per-chunk feature branches.
- **Commit format:** conventional commits (`feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `test:`, `perf:`, `build:`, `ci:`); wrap commits each chunk.
- **Main branch:** `main`
- **Never force push** to main; the version branch is pushed at every wrap commit (the remote matches local HEAD on exit).
- **PRs:** CI runs on push and PR (`ci.yml`; `nightly.yml` runs the weekly advisory check and the 120 s-per-target fuzz time-box on `schedule` / `workflow_dispatch` from the default branch). The `mutants` legs (ubuntu-latest, windows-2025) diff against `AGENT_RUN_CHUNK_BASE` = the PR base sha, or else the push's `github.event.before` (this project pushes one build branch, so the push trigger is what fires per chunk). The `mutants-verdict` job gates their union. No `concurrency:` block, so no push's run is cancelled.

## Andromeda workflow

This project uses the Andromeda pipeline for architecture, planning, and implementation.

**Initial planning (done once, in fixed order):**
1. `/andromeda-arch` — architecture (creates `.andromeda/architecture.md` + `project.yaml`)
2. `/andromeda-security` → `/andromeda-design` → `/andromeda-tests` → `/andromeda-obs` → `/andromeda-a11y` — the specialist plans
3. `/andromeda-route` — break the project into versions and an ordered build route
4. `/andromeda-setup-project` — populate CLAUDE.md and `.claude/` from the plans

**Per chunk (main development loop):**
1. `/andromeda-new-session` — load context, run the health check, propose the next action
2. `/andromeda-phase` — promote and plan the next chunk from the working route
3. `/andromeda-implement` — execute the plan (writes code and tests, runs the gates green; it never commits)
4. `/andromeda-wrap-session` — reconcile, capture learnings, commit, push
5. Repeat per chunk; when the version's verification matrix is complete, `/andromeda-route` starts the next version

**The first consumer is the build itself:** viola's own loop is driven overseer → builder, first by hand, then through the prototype, and — once the Epoch-3 live test passes — through viola.

## Daily session lifecycle

**At the start of a session:**
- Read `.claude/session-handoff.md`; `/andromeda-new-session` shows the dashboard, runs the health check and proposes the next action.

**During work:**
- Follow the current chunk's plan; drive verification through `scripts/agent-run.*` (never a manual smoke step).
- When Claude makes a mistake, correct it — `/andromeda-wrap-session` will capture the correction as a learning later.

**At the end of a session (or mid-session after a milestone):**
- `/andromeda-wrap-session` — commits work, updates session-handoff.md, analyzes the session for learnings and curates them into the right tier (CLAUDE.md, `.claude/rules/*`, or `.claude/docs/session-learnings.md`).

**Don't skip session boundaries.** They keep CLAUDE.md and project knowledge fresh over time.

## Verification discipline
- Every gate is agent-runnable with a machine-readable verdict: nextest JUnit, Playwright JSON, `mutants.out/outcomes.json` (locally) and the per-leg `mutants-verdict-<leg>.json` (CI), hyperfine JSON, llvm-cov JSON, one harness JSON document per command, and `viola-harness gate` as the one verdict per CI job.
- CI (three OSes) is the authority for OS-specific code: a Linux-only or Windows-only local run is not proof for the other OSes.
- The real `claude` CLI runs only locally (`agent-run run --local-live`, `viola verify`); CI uses the fake agent and recorded fixtures. Changed real-CLI behaviour means a local fixture refresh + the contract suite.
- Zero flakes: a flaky test keeps the chunk red until the root cause is fixed in that chunk.

## Code review

- **Locally:** the **code-reviewer** agent is installed at `.claude/agents/code-reviewer.md` — invoke via trigger phrases like "review this" or "does this make sense?"
- **Architecture review:** for significant changes, read `.andromeda/architecture.md` Established Decisions to check alignment; an amendment rides the implementing chunk's wrap reconcile.

## Release process
v1 has no release stage: `cargo install --path .` locally. v1.x adds a dist 0.33.0 release workflow with cargo-auditable, signing and self_update (with `signatures`) — never with `rust-cache`.

## Troubleshooting workflow
- Tests failing? `scripts/agent-run.sh logs` for the merged event + diagnostics stream; check `.claude/rules/testing.md`
- Hooks not running? Check `.claude/settings.json` and that `jq` is on PATH (hooks exit 0 silently without it)
- CLAUDE.md looks stale? Run `/andromeda-new-session` to see the health check, possibly re-run `/andromeda-setup-project`

## Philosophy

This project uses a **reference-based** CLAUDE.md architecture — a thin CLAUDE.md index with deeper content in `.claude/docs/` (on-demand) and `.claude/rules/` (path-scoped). See `section-markers.md` in the Andromeda skills for the ownership convention between generated and user-curated sections.
