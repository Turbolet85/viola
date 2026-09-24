
## 2026-09-24-observability-gates — lint bans, gate tools, new target/ paths, scan-gated CI uploads
**Section:** §Stack and Technologies (Code quality row) · §Occupied Resources → Repository · §Infrastructure Patterns → Build system (Lint), Project directory structure, CI/CD approach (Setup steps, Jobs wired today, target job 2)
**Change:**
- The Code quality row names:
  - the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros;
  - ripgrep 15.2.0 for G1/G3;
  - runner `jq` for G2;
  - jsonschema 0.57.0 for the harness `schema-check`.
- Registered paths: `target/secret-scan/hits.json`, `target/tools/ripgrep/bin/`, `target/lint-probes/`.
- The tree gains `clippy.toml`, `scripts/lint-probes.sh` and `scripts/install-ripgrep.sh`.
- The Lint bullet and target job 2 carry `--features fake-agent` and the bans; the raw `event!` grep and `tests/contract_lints.rs` are named.
- Setup steps name `scripts/install-ripgrep.sh` (install-action has no ripgrep manifest) and runner `jq`.
- `test` job: follows the obs-plan §9 gate order with scan-gated uploads, replacing the unscanned `agent-run-<os>` upload.
- `lint` job: wires target jobs 1–2 plus the ripgrep install, G1 and G3, and the lint probes on Linux.
**Why:** chunk 2026-09-24-observability-gates (report Changes: Schema / config, Dev-tool versions, Harness / gate surface, Counts moved; Spec claims disproved 1). New gitignored `target/` resources registered in the same form as `target/deny-probes/`.
**Sweep (cascade step 2):**
- Masters, 7 of 7:
  - `all-targets -- -D warnings` (featureless clippy): 0 hits after the apply;
  - `agent-run-<os>`: arch 0 hits;
  - the CLAUDE.md lint line already carried `--features fake-agent`.
- Leaves: `.claude/docs/stack.md` Code quality row (verbatim mirror) re-derived; `.claude/docs/commands.md` re-derived. CLAUDE.md `GENERATED:setup:*` was recomputed against the amended Stack, Occupied Resources and Infrastructure: no stale line (pointer table, warnings and workflow unchanged).
