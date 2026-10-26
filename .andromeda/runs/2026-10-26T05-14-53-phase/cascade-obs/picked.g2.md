### ci-integration#1.5 · values · ci · ## 9. CI Integration · lines 14–14
| OTel trace export | N/A (no exporter in v1) | — | — |
### ci-integration#2.1 · values · ci · ## 9. CI Integration · lines 20–22
| Stage | Telemetry produced | Consumer |
|-------|---------------------|----------|
| Lint / typecheck | clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below); `cargo check` of sync crates without Tokio; `cargo deny check` (ubuntu) incl. veil `toggle` ban and bans on `opentelemetry-otlp` / `opentelemetry-stdout` / `sentry` / `tracing-appender` / tracing-subscriber `env-filter` feature | CI annotations + job exit code |
### ci-integration#2.2 · values · ci · ## 9. CI Integration · lines 23–23
| Unit tests | In-memory `Vec<u8>` writer tests parse emitted lines with serde_json against `schemas/diag-line.v1.json` (tests-owned bodies) | nextest JUnit |
### ci-integration#2.4 · values · ci · ## 9. CI Integration · lines 25–25
| E2E tests | `agent-run logs` / `status`; zero-`event:"panic"` gate G2 and schema gate G4 (below) | uploaded artifacts + gate exit codes |
### ci-integration#3.3 · procedure · ci · ## 9. CI Integration · lines 40–53
- **G4, schema conformance.** §10 counts schema-conformance failures as a build failure. G1–G3 do not measure them, and the Unit tests row checks only in-memory lines, so G4 checks real process output.
  - Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).
  - It runs as its own step, `id: schema-conformance`, `if: always()` after the last test step. It exits non-zero on any failing line and prints `file`, line number and the failing schema keyword, never the line's content (detail lines carry the canary). The check body and the JSON Schema validator crate are tests-owned; obs owns both schemas.
  - A line that is not JSON is skipped and counted, as in G2, because chaos cases kill writers mid-line and a restarted writer appends after the torn line. Interleaved writes are caught by the concurrent-append check (§3 Logging stack), not by G4.

```sh
# G1: whole workspace from its root (root bin `viola` + crates/* + tests/ + fuzz/); target/ is skipped through .gitignore
set +e; rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .; test $? -eq 1
# G2: first fail if no home-level role file exists (an empty scope would pass vacuously); the start point ends in a slash, so a scope that is a link to a directory is descended (the Linux dev host's `target/e2e-home`; on a real directory the walk is the same); `awk 1` ends every file's last line with a newline; -R + fromjson? skip a torn line (not a panic record) instead of failing with jq exit 2
test -n "$(find target/e2e-home/ -path '*/diagnostics/*.ndjson' ! -name 'detail-*' -print -quit)"
find target/e2e-home/ -path '*/diagnostics/*.ndjson' ! -name 'detail-*' -exec awk 1 {} + | jq -R -n -e --arg seam src/cmd/hook/seam.rs '[inputs|fromjson?|select(.event=="panic")|select(((.panic_location|type)=="string" and (.panic_location|test(":[0-9]+$")) and ((.panic_location|sub(":[0-9]+$";""))==$seam))|not)]|length==0'
# G3: no abort panic strategy (profile `panic` key, `-C panic` rustflag, CARGO_PROFILE_*_PANIC env) in Cargo.toml, cargo config or workflows (§7); default regex engine; only rg exit 1 passes
set +e; rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' "(panic|_PANIC)\s*[:=]\s*[\"']?abort" .; test $? -eq 1
```
### obs-anti-patterns#1.1 · exception · logs · ## 11. Obs Anti-Patterns (NEVER do these) › ### Logs · lines 8–8
- NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (`viola-harness` and the fake agent are exempt, §3 obs-ci-gate-wire). They bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing. Enforced by `[workspace.lints.clippy]` `print_stdout`, `print_stderr`, `dbg_macro` = `deny` under the §9 clippy step. Only the output modules that own `--json`, human CLI stderr and the hook decision body may carry a local `#[allow]`, and only where they use a print macro — `run`'s and `revive`'s pre-spawn refusals, `revive --list`'s, `verify`'s, `list`'s and `plugin install`'s refusals write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1, chunk 2026-10-17-optional-plugin-install), which uses no print macro and carries no `#[allow]` — plus the fake agent's crate-level `#![allow]` in `src/bin/viola-fake-agent.rs` (a root-package bin, §3 obs-ci-gate-wire).
