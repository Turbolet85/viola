# security · Secret Management — category read

## Carried
- (.andromeda/security-plan-detail/secret-management.md:9) Every new or moved cause record (a `process-exit` `detail`, a cause's new log form, a diagnostics line, a code word) must hold no inherited credential value; a code or log form may be added only as a closed word, never built from the environment.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.
- (.andromeda/security-plan.md:476) The human `hint:` lines and any stderr pair of a catalogued cause fall under this ban, as do the log forms: no hint, code or line may carry a stripped `CLAUDE*` value.
  > NEVER write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log or stderr, `events.ndjson`, snapshots, `diagnostics/` or fixtures (consolidated floor: Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan-detail/secret-management.md:8) The exit-cause table must reach each cause (endpoint lost mid-call, internal error, a `claude` not on `PATH`) without a new environment read or test seam: the seams named here are the set, and a cause no existing means reaches is named as such, not given a new variable.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds
- (.andromeda/security-plan-detail/secret-management.md:15) `tests/cross_exit_causes.rs` and every case moved in the existing suites must run under a test-owned `--home`, never the operator's real home.
  > **Development:** Same as production. Tests use their own `--home` directory.
- (.andromeda/security-plan.md:380) The chunk's CI runs must pass this scan with the new cause log forms and the new table's homes in reach of it: no new form may trip a `claude-stripped` needle, the `cookie:` class or the content canary in a home-level role file, and diagnostics files stay `0600`.
  > What CI does run is the obs-owned artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`, `if: always()`, obs-plan §9 step 3). Its classes:
  > - the stripped `CLAUDE*` canaries (the twelve `claude-stripped` needles);
  > - the `cookie:` header, case-insensitive;
  > - the content canary, which fails only in home-level role files;
  > - a non-`0600` diagnostics file, Unix only.
- (.andromeda/security-plan.md:386) A red scan on this chunk's runs withholds the test-home uploads, so the chunk's evidence must not lean on an upload the scan held back.
  > It never prints or writes the matched bytes, and the test-home uploads run only when it succeeds.

## Bindings
- **Credential values in cause records ↔ Bootstrap phases `logging-redaction-wire`**: the category's credential rule (.andromeda/security-plan-detail/secret-management.md:9) and the § Secrets ban (.andromeda/security-plan.md:476) restate the consolidated NEVER-log floor (.andromeda/security-plan.md:344) and must not diverge from it; the catalogue's log forms and hints answer to all three.
- **No new environment read ↔ Security Anti-Patterns § Universal**: the seam set (.andromeda/security-plan-detail/secret-management.md:8) binds with the ban on another seam or environment read without its own Decisions Log entry (.andromeda/security-plan.md:519); a seam the exit-cause table would need is that entry first.
- **Canary scan ↔ obs-plan §9 and test-plan's Exit-cause matrix**: the scan the chunk's CI must pass (.andromeda/security-plan.md:380) is obs-owned, and the new table's homes and log forms are what it reads; the table's fixtures stay synthetic under its own `--home` (.andromeda/security-plan-detail/secret-management.md:15).
