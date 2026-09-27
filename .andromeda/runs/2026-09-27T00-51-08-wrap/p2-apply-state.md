# P2 apply state — wrap of 2026-09-26-local-linux-pre-push-gate (paused at the context alarm, 86%)

Resume: `/andromeda-wrap-session` → Setup 2a sees `report.md` → choose **resume** in THIS run dir
(`.andromeda/runs/2026-09-27T00-51-08-wrap/`). `fanout-results.md` holds all seven verdicts and the validation →
continue at **P2 Apply** with the "remaining" list below, then the cascade, then P3–P7.

## Done
- P1 report: `viola-0.1.0/chunks/2026-09-26-local-linux-pre-push-gate/report.md` (+ evolve records).
- P2 fan-out + validation: `fanout-results.md` (19 accepted, 2 rejected, 5 orchestrator-raised).
- Stray `evidence/guards/__pycache__/guards.cpython-314.pyc` deleted (the wrap commit records the deletion).
- Code-graph refresh ran at Setup (`code-graph-refresh.log`); P4 reads `.andromeda/cache/.refresh-done`.
- **Applied spec bodies (NO sidecar entries written yet — they wait for the cascade sweep):**
  - test-plan (7): §3 `run` step 4 Base (uncommitted-promotion condition) · §5 Module ↔ PTY row (resize window +
    tests) · §9 tool-install paragraph (WSL provisioning) · §3 Internal harness subcommands `pre-push` entry · §3 Closed
    enums `pre-push` bullet · §10 Mutation gate (local union before the push) · §12 Decisions Log entry `2026-09-27`.
  - architecture (6): §Stack CI/CD row (local pre-push gate) · §Established Decisions [Naming] (seam exception) ·
    §Conventions Environment variables (seam exception) · §Cross-cutting Config management (seam sentence) ·
    §Occupied Resources → Environment variables (test-seam bullet) · §Occupied Resources → Repository
    (`target/pre-push/`).

## Remaining P2 apply (in this order)
1. architecture §Infrastructure Patterns → Project directory structure (~:483 `viola-e2e/` comment gains `pre-push`;
   ~:491 add `wsl-provision.sh  # in-distro WSL provisioning: sha256-pinned rustup-init 1.29.1, rust-toolchain.toml,
   cargo install --locked of ci.yml test-job pins (+ --check, --probe)`).
2. architecture §Infrastructure Patterns → CI/CD approach (~:522-525): a bullet for the operator-pass order (stop
   rust-analyzer → `pre-push` on the uncommitted tree → pre-CI commit → guarded push → CI reads; red stops the pass) and
   that the local verdict is the same union CI's `mutants-verdict` computes.
3. security-plan (all ratified by the operator's wrap directive — cite it in each sidecar entry):
   - §Input Validation boundary table (~:217): row for `FAKE_AGENT_PUMP_DELAY_MS` (fake-agent-only, u64 ms, cap 5 000,
     absent from release, not `VIOLA_*`, disables no control).
   - §Security Anti-Patterns → Universal (~:577): carve-out sentence beside the `VIOLA_*` / env rule.
   - §Secret Management → Storage (~:420): the "env vars are not a configuration channel" restatement points at the
     carve-out.
   - Decisions Log: `2026-09-27` entry for the seam (ruling of record).
   - #10 §Dependency Security (~:323/:339): WSL toolchain install (sha256-pinned rustup-init 1.29.1 +
     `cargo install --locked` of ci.yml's pins) as a third install site of the same pins.
   - #11 §Secret Management: `pre-push` runs every WSL call under `env -i` (HOME + PATH only) — no `CLAUDE*` value
     crosses (canary measured 0; WSLENV forwards only `WT_*`).
4. obs-plan: #12 §9 Pipeline integration Mutation row (~:1253) — the local `pre-push` union is the same verdict before
   the push; #13 §8 integration point 6 (~:1208) — Linux clone / home paths never appear in the `pre-push` document or
   the copied-back leg verdict; #17 — read each "env vars are not a configuration channel" restatement (:39 :249 :604
   :606 :1139 :1278 :1384 :1414 :1564): amend only a sentence that reads as exhaustive over what `viola` reads.
5. Cascade: write `cascade-patterns.toml` from every amendment of the pass (base rule `When that commit is HEAD`,
   `a resize is propagated`, `exactly one version source`, `Internal harness subcommands`, `VIOLA_` env rule,
   `not a configuration channel`, `Mutation gate`), run `cascade.py sweep`, disposition every row, then write the four
   sidecars (`{doc}-amendments.md` via `splice.py append`) with the sweep counts, then re-derive the leaves
   (CLAUDE.md GENERATED blocks; `.claude/docs/{stack,conventions,commands,gotchas,tests-summary,security-summary,
   obs-summary}.md`; `.claude/rules/{testing,verification-harness,security}.md` above `## Session Additions`).
6. P2 evolve checkpoint (reconcile), then P3 curation (candidates in report §Decisions & corrections: `wsl --exec`
   never `--`; MSYS2_ARG_CONV_EXCL for `/mnt/…`; distro PATH carries the Windows PATH; `wsl -u root` needs no sudo;
   apt lists go stale → update first; nextest `test(=…)` needs the full `tests::` name; a test's deadline must sit below
   the runner's kill line; `reset --hard` drops `add -A`-staged files so a `clean` guard needs a stray-file case;
   force the window rather than sample a race), P4, P5, P6, P7 (light gate ≈ 15 min: pre-push entry 13 runs).
