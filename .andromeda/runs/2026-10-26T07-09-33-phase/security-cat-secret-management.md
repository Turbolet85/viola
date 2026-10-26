# security · Secret Management — category read

## Carried
- (.andromeda/security-plan-detail/secret-management.md:8) The server verification must be switched or faked by no environment variable: a test seam that forces a mismatch, or any env read that skips the check, is outside the closed set of non-`VIOLA_*` reads and needs its own Decisions Log entry.
  > environment variables are not a configuration channel — the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds, and the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM`
- (.andromeda/security-plan-detail/secret-management.md:9) Every line the new check writes on a mismatch or a check it cannot make (diagnostics, events, a refusal) must carry no inherited `CLAUDE*` value.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger.
- (.andromeda/security-plan-detail/secret-management.md:9) The hook client (`hook.event`, `hook.dialog`) gains its verification inside the environment Claude Code hands a hook, which holds both messaging variables, so the hook's fail-open mismatch path must read and record names only, never a value.
  > the environment `claude` 2.1.287 hands a hook, read in a child started with every inherited `CLAUDE*` name removed, holds 12 names, both messaging variables among them
- (.andromeda/security-plan-detail/secret-management.md:15) The mismatch and unverifiable-server tests must run against their own `--home`, never the user's `~/.viola`.
  > - **Development:** Same as production. Tests use their own `--home` directory.
- (.andromeda/security-plan-detail/secret-management.md:16) The chunk's gates run under the pre-push launcher with exactly `HOME` and `PATH`; a test of the verification must not need a host value crossing into a child, and any such assignment is a boundary widening held for the founder's word.
  > launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` from the repository root, so no host environment value — the inherited `CLAUDE*` credentials included — crosses into a child. No other assignment is made: every launcher call carries exactly `HOME` and `PATH`, every PATH component a constant under the home or a system dir.
- (.andromeda/security-plan-detail/secret-management.md:16) A host-valued assignment added for a verification test is a widening, never an implementation choice.
  > Any assignment whose value comes from the host is a boundary widening and needs its own Decisions Log entry
- (.andromeda/security-plan.md:369) The broad line the verification honours on every client path.
  > **Production:** no stored secrets in v1, and environment variables are not a configuration channel; the inherited `CLAUDE*` credentials stay only in the `viola run` wrapper's environment, removed from the child by the R8 rule (the identity floor whatever the persistent set says) and never serialised into events, snapshots, diagnostics or the ledger; the strip handles names only and never reads a value.
- (.andromeda/security-plan.md:377) The artifacts the new mismatch tests leave under `target/agent-run/` must pass the CI canary scan, its `claude-stripped` needles and the 0600 diagnostics class included.
  > What CI does run is the obs-owned artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`, `if: always()`, obs-plan §9 step 3). Its classes:
  > - the stripped `CLAUDE*` canaries (the twelve `claude-stripped` needles);
  > - the `cookie:` header, case-insensitive;
  > - the content canary, which fails only in home-level role files;
  > - a non-`0600` diagnostics file, Unix only.
- (.andromeda/security-plan.md:516) No `config.json` key, `VIOLA_*` variable or CLI flag may switch the server verification off.
  > NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan (the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check, `validate_paste_text`): environment variables are not a configuration channel.
- (.andromeda/security-plan.md:343) The pid and start time the check compares may go to `diagnostics/` only, never into the CLI exit-21 output, the MCP `instance-unreachable` result or channel `error.data`.
  > Pids, `agent_session_id`, `started_at` and `budget.json` values may be written to `diagnostics/`.

## Bindings
- **No env seam for the check ↔ body §Input Validation test-seam rows and §Security Anti-Patterns Universal**: a mismatch-forcing test seam would be a third `fake-agent` seam beside the two the body names; (.andromeda/security-plan-detail/secret-management.md:8) and (.andromeda/security-plan.md:516) — a new seam halts for a Decisions Log entry.
- **Credentials never serialised ↔ body §Bootstrap phases `logging-redaction-wire` and obs-plan §9 canary scan**: the hook client's new mismatch path writes no `CLAUDE*` value, and the scan's `claude-stripped` needles prove it on CI; (.andromeda/security-plan-detail/secret-management.md:9), (.andromeda/security-plan.md:377).
- **Pid / start time in diagnostics only ↔ body §Error Handling (MCP `instance-unreachable` with `detail: null`, fixed CLI messages)**: the compared values stay out of every external error; (.andromeda/security-plan.md:343).
- **Pre-push `env -i` launcher ↔ body §Dependency Security pre-push row and the scope's widening hold (`inputs#I1` item 5)**: a host-valued assignment a verification test needs is a widening for the founder's live word; (.andromeda/security-plan-detail/secret-management.md:16).
