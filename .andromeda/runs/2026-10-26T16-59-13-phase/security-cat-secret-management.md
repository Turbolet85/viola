# security · Secret Management — category read

## Carried
- (.andromeda/security-plan-detail/secret-management.md:8) The strict-modes checks this chunk adds (the hook's read through `VIOLA_DIR`, every CLI / MCP entry point) must take no new environment read and no env-var switch, and a test-side way around the check for a seeded home (CARRYs 6, 13) must not be an environment seam.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds, and the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM`, read in every build by `viola list`'s human mode only to decide whether and how deep SGR is written
- (.andromeda/security-plan-detail/secret-management.md:9) Every snapshot, diagnostics file and ledger stamp this chunk writes, checks or newly guards (the DACL check before `diagnostics/detail-*.ndjson`, the single snapshot/stamp writers, any refusal record) must carry no inherited `CLAUDE*` credential.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.
- (.andromeda/security-plan-detail/secret-management.md:15) The chunk's negative cases (a 0770 `--home`, a widened instance DACL, a seeded Windows home) must each run in a test's own `--home`, never the user's real home.
  > Same as production. Tests use their own `--home` directory.
- (.andromeda/security-plan-detail/secret-management.md:16) Any gate this chunk runs through the local pre-push launcher (its stamped-home gate, CARRY 11's two clean release builds if run there) must launch under `env -i` with exactly `HOME` and `PATH`; an added host-valued assignment is a boundary widening that halts for the founder's word.
  > launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root, so no host environment value — the inherited `CLAUDE*` credentials included — crosses into a child. No other assignment is made: every launcher call carries exactly `HOME` and `PATH`, every PATH component a constant under the home or a system dir.
- (.andromeda/security-plan.md:373) The per-start `plugin/` rewrite must keep the embedded plugin files carrying only exec paths, with no credential literal.
  > **Never in code:** No token, key or credential literal in source or in the embedded plugin files (`hooks.json`, `.mcp.json`, `plugin.json` carry only exec paths).
- (.andromeda/security-plan.md:377) The owner-only home and the diagnostics DACL work must keep CI's artifact canary scan passing, its non-`0600` diagnostics class and its home-level role-file content canary included.
  > What CI does run is the obs-owned artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`, `if: always()`, obs-plan §9 step 3). Its classes:
  > - the stripped `CLAUDE*` canaries (the twelve `claude-stripped` needles);
  > - the `cookie:` header, case-insensitive;
  > - the content canary, which fails only in home-level role files;
  > - a non-`0600` diagnostics file, Unix only.
- (.andromeda/security-plan.md:473) A refusal, diagnostic or snapshot line the new checks produce must hold no R8-stripped `CLAUDE*` value.
  > NEVER write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log or stderr, `events.ndjson`, snapshots, `diagnostics/` or fixtures
- (.andromeda/security-plan.md:474) Test homes the chunk's cases create (seeded homes, widened-mode homes) must never be committed.
  > NEVER commit `.env` files, local `--home` test directories or signing material to git.
- (.andromeda/security-plan.md:516) The home strict-modes check this chunk extends to every entry point must not be switchable off by `config.json`, a `VIOLA_*` variable or a CLI flag, and no new seam or environment read may be added for it without its own Decisions Log entry.
  > NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan (the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check, `validate_paste_text`): environment variables are not a configuration channel. The carve-outs, the two test-only seams and the terminal colour facts, switch off nothing; NEVER add another such seam or environment read without its own Decisions Log entry.

## Bindings
- **No env channel ↔ Input Validation / Anti-Patterns Universal (the body)**: the strict-modes check at every entry point is a control no env var disables; a seeded-home accommodation (CARRYs 6, 13, test-plan §5 carve-out) must be made in the test seed, not as a new seam (.andromeda/security-plan-detail/secret-management.md:8), (.andromeda/security-plan.md:516)
- **Credentials never serialised ↔ Data Protection (owner-only home, diagnostics 0600) and obs-plan §9 secret-scan**: the files this chunk guards are the ones the canary scan reads; its non-`0600` diagnostics class is the Unix witness of the owner-only rule (.andromeda/security-plan-detail/secret-management.md:9), (.andromeda/security-plan.md:377)
- **Tests' own `--home` ↔ test-plan §5 / §6 (the 0770 `--home` negative, the Windows `RUNNER_TEMP` case)**: every negative home is a test's own and uncommitted (.andromeda/security-plan-detail/secret-management.md:15), (.andromeda/security-plan.md:474)
- **Plugin files carry only exec paths ↔ Anti-Patterns Data Protection / Universal (rewrite every start, absolute pinned path)**: the rewrite keeps the content to exec paths (.andromeda/security-plan.md:373)
- **pre-push `env -i` ↔ Dependency Security (pre-push `tools` stage) and the scope's widening halt**: a host-valued assignment is a widening (.andromeda/security-plan-detail/secret-management.md:16)
