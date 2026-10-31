# security · Secret Management — category read

## Carried
- (.andromeda/security-plan-detail/secret-management.md:9) The value of an inherited credential must stay in the wrapper's environment, so no error body on the three surfaces, and no diagnostics record written beside one, may hold it.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.
- (.andromeda/security-plan-detail/secret-management.md:9) The witness's planted stripped value must sit under a name this rule removes: a persistent-set name is kept, and only an identity-floor name is removed whatever the persistent set says.
  > R8 (operator ruling 2026-09-25, Decisions Log): every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) is removed except a name in the persistent set — Windows registry `Environment` value names, Unix `config.json` `claude_env_keep` — and the identity floor (`IDENTITY_FLOOR`, the 11 names measured on the Windows host, both messaging variables among them) is removed whatever the persistent set says.
- (.andromeda/security-plan-detail/secret-management.md:9) A sanitised error path must not start reading a stripped variable's value: the strip and the logs stay names-only, and the proof must hold that a name on `process-start` is no value on an error surface.
  > The strip, the registry reader and the logs handle names only and never read a value; `process-start{subject:"claude-child"}` names the stripped (`env_stripped_known`) and kept (`env_kept`) names.
- (.andromeda/security-plan-detail/secret-management.md:8) Driving a failure for a witness must use the listed variables or a test-side setup, because the environment outside `VIOLA_*` plumbing is this closed set and no variable may select or shape an error body.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds, and the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM`, read in every build by `viola list`'s human mode only to decide whether and how deep SGR is written (they configure nothing, disable no control and widen no redaction;
- (.andromeda/security-plan-detail/secret-management.md:15) Each surface's witness must run against its own `--home`, never a host home.
  > Tests use their own `--home` directory.
- (.andromeda/security-plan-detail/secret-management.md:16) The planted value must be set by the witness itself on the child it drives, because a credential on the host never crosses the launcher and so proves nothing under the local gate.
  > launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root, so no host environment value — the inherited `CLAUDE*` credentials included — crosses into a child.
- (.andromeda/security-plan-detail/secret-management.md:16) The planted value must be a test constant, since carrying a real host value into a gate child is a boundary widening that needs its own Decisions Log entry and the founder's live word.
  > Any assignment whose value comes from the host is a boundary widening and needs its own Decisions Log entry;
- (.andromeda/security-plan.md:344) This floor names the forbidden value for the chunk's proof, and the diagnostics files that receive an error's full detail fall under it as well as the external bodies.
  > Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value to any log, diagnostic, snapshot, `events.ndjson` or fixture.
- (.andromeda/security-plan.md:343) Any wording this chunk moves in a restatement of the floor must stay in step with the consolidated list, which is the only form allowed to lead.
  > This list is the consolidated NEVER-log floor. Data Protection § Logs, Error Handling and Anti-Patterns § Secrets / § Logging restate parts of it and must not diverge from it:
- (.andromeda/security-plan.md:476) The ban's sinks include stderr and `diagnostics/`, so human-mode stderr and the diagnostics path must hold no stripped value on the failures the witnesses drive.
  > NEVER write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log or stderr, `events.ndjson`, snapshots, `diagnostics/` or fixtures (consolidated floor: Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan.md:396) The proof's secret class must cover at least both messaging variables, the only secrets v1 names.
  > The inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `CLAUDE_CODE_MESSAGING_SOCKET`.
- (.andromeda/security-plan.md:376) The witness's planted value must be a synthetic canary, never a token, key or credential literal.
  > **Never in code:** No token, key or credential literal in source or in the embedded plugin files (`hooks.json`, `.mcp.json`, `plugin.json` carry only exec paths).
- (.andromeda/security-plan.md:478) The same ban stands for the chunk's new test and product code.
  > NEVER hardcode credentials in source or in the `include_str!` plugin files.
- (.andromeda/security-plan.md:380) Any `v1-47` leg resting on "a secret scan after every end-to-end test" must read this obs-owned scan, step and condition as named, and no second scanner may stand in for it.
  > What CI does run is the obs-owned artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`, `if: always()`, obs-plan §9 step 3). Its classes:
- (.andromeda/security-plan.md:381) A planted stripped value counts as proof for the scan only under this class, so the chunk's stripped-value claim must match these needles.
  > the stripped `CLAUDE*` canaries (the twelve `claude-stripped` needles);
- (.andromeda/security-plan.md:386) Witness output written under `target/agent-run/` falls inside this scan, so a witness's failure message must not print a planted needle, and the scan's silence about matched bytes must be kept.
  > It reads every file under `target/agent-run/` except `run --mutants`' `target/agent-run/chunk.diff`, which is skipped by exact path and excluded from the `harness-<os>` upload, so it is never uploaded. It never prints or writes the matched bytes, and the test-home uploads run only when it succeeds.
- (.andromeda/security-plan.md:337) The scan must be held to this plan's floor, with the scan-gated uploads waiting on its pass, and no repo secret scanner is added by this chunk.
  > **secret-scanning-ci-gate:** no repo secret scanner in v1, there being no secrets in the repo or CI; the CI `test` job runs obs-plan §9's artifact canary scan against this plan's NEVER-log floor, never printing or writing the matched bytes, and every scan-gated upload waits for it to pass;
- (.andromeda/security-plan.md:519) A witness that needs a new way to force a fault must not add an environment seam without its own Decisions Log entry, and no such seam may switch off a control.
  > The carve-outs, the two test-only seams and the terminal colour facts, switch off nothing; NEVER add another such seam or environment read without its own Decisions Log entry.

## Bindings
- **Stripped-value floor ↔ body §Error Handling**: the value this category forbids is one of the things every external body must omit, and the full-detail sink of those bodies stays under the floor; `(.andromeda/security-plan.md:344)` with `(.andromeda/security-plan.md:409)` and `(.andromeda/security-plan.md:416)`.
- **Category detail ↔ body §Secret Management**: the detail's credential and R8 rule is the group under the body's broad production line, and the two must read alike; `(.andromeda/security-plan-detail/secret-management.md:9)` with `(.andromeda/security-plan.md:372)`.
- **Artifact canary scan ↔ obs-plan §9 and the test plan's CI integration**: obs owns the scan and tests own the witness bodies, while this plan holds the floor it is read against and the classes it holds; `(.andromeda/security-plan.md:380)` with `(.andromeda/security-plan.md:337)`.
- **Scan and floor ↔ `verification-matrix.json#v1-47`**: the capability's secret-scan and never-log legs stand only if this scan and floor are what the chunk's proof reads; `(.andromeda/security-plan.md:380)` with `(.andromeda/security-plan.md:343)`.
- **Environment closed set ↔ body §Input Validation seam rows**: the seams named in this category are the rows that bound them, and a witness using `FAKE_AGENT_HOOK_PANIC` stays inside its closed value; `(.andromeda/security-plan-detail/secret-management.md:8)` with `(.andromeda/security-plan.md:152)` and `(.andromeda/security-plan.md:519)`.
- **Gate launcher ↔ the operator directive's founder-word rule**: a host value carried into a gate child is a boundary widening, asked of the founder live at P4 as well as logged; `(.andromeda/security-plan-detail/secret-management.md:16)`.
