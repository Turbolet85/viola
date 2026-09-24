
## 2026-09-24-observability-gates — internal gate subcommands, exit-aware 10 s readiness, mutants kill below the floor, runner jq and pinned ripgrep, scan-gated uploads
**Section:** §3 `run` step 2 (root `booted_wrapper` readiness) · §3 Internal harness subcommands (new `schema-check`, `secret-scan`; `gate` CI-upload sentence; Closed enums) · §3 Bootstrap phases test-runner-install (`[profile.mutants]`) and ci-tool-install · §9 tool-install paragraph · §9 Test report format
**Change:**
- Root `booted_wrapper` readiness is bounded at 10 s (was 20 s) and exit-aware: it fails at once when `child.try_wait()` reports an exit. Harness `boot` keeps 20 s.
- `schema-check` and `secret-scan` are declared as internal subcommands, with their output shapes, reasons and closed class enum.
- `[profile.mutants]` slow-timeout is 5 s × 2, with a `package(viola-e2e)` override at 15 s × 2. The cargo-mutants auto-timeout floor is recorded as measured.
- jaq leaves CI installation; G2 uses the runner-provided, presence-checked `jq`.
- ripgrep 15.2.0 comes from `scripts/install-ripgrep.sh`.
- The `agent-run-<os>` upload and the "raw junit.xml is never uploaded" clause are retired. In their place are the scan-gated `harness-`, `diag-` and `junit-<os>` uploads and `secret-scan-<os>`.
**Why:** chunk 2026-09-24-observability-gates (report Changes: Symbols / APIs, Schema / config, Dev-tool versions, Harness / gate surface; Spec claims disproved 2). The `2834e4d` mutants red on run `35995290314` was two `wait_ready` consumers spinning to the 20 s bound under cargo-mutants' 20 s floor. Operator decisions at phase P4: runner jq; deadlines fixed by cause and by value.
**Sweep (cascade step 2):**
- Masters, 7 of 7:
  - `bounded at 20 s`: 0 hits after the apply;
  - `period = "15s"`: 1 hit, the amended `:749` override;
  - `jaq 3.1.1`: 2 hits, test `:606` (assertion form, no change) and the amended `:783`;
  - `agent-run-<os>` / `agent-run-${{`: 1 hit, the amended `:657` ("there is no unscanned `agent-run-<os>` upload");
  - `is never uploaded`: 0 hits.
- Other `20 s` sites kept: `:530` (harness boot), the fixture timeout derived from boot, and the "boot's 20 s / 10 s deadlines" line.
- Leaves: `.claude/rules/verification-harness.md` (internal-subcommand list; print-bans-landed wording) re-derived; `.claude/docs/commands.md:10` (jaq install) and `.claude/docs/stack.md:46` (jaq) re-derived; `tests-summary.md` needs no change.
