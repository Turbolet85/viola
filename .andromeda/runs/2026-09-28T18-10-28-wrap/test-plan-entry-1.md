
## 2026-09-28-capability-ledger-and-viola-verify — verify's lines pinned by literal asserts, the fake agent's print mode, the hygiene walk over the first recorded set
**Section:** §1 Test Scope Summary (the arch CI/CD quotation; harness readiness); §3 `boot` readiness; §5 Integration CLI; §7 Fake agent (modes, `start` receipt) and Fixture hygiene
**Change:**
- §5 CLI: `tests/cli_verify.rs` (12 cases) pins `viola verify`'s six `[NN/06]` step lines, the `stamped` summary, the `unable:`/`hint:` pairs and `error: internal error` by literal asserts; the trycmd verify case arrives with a redaction-heavy consumer (the "`viola-verify` step counter" left the trycmd bullet).
- §7 Fake agent: print mode `-p/--print <prompt>` — start receipts, the four spine `default` fixtures (prompt set), `ok` on stdout, exit 0, never raw, no stdin; `DEFAULT_CLI_VERSION` stays 2.1.0. The `start` receipt follows the raw-mode guard only in interactive mode.
- §7 Fixture hygiene: a run-time directory walk (`claude_fixtures`, with its own walker test), not rstest `#[files]`; it also covers `fixtures/claude/*/*.json` (the 2.1.283 set) against `schemas/claude-fixture.v1.json` (was "join with the first recorded fixture").
- §1/§3: the `boot` line-3 readiness check joins with "Verify-stamped test homes and harness" (was "Capability ledger and viola verify", which landed with the harness unchanged); §1's arch quotation reads the real CLI only locally, `verify` in CI against the fake agent.
**Why:** rstest 0.27 refuses an empty glob at compile time (`rstest_macros-0.27.0/src/parse/rstest/files.rs:635`, measured at implement); the rest is the chunk's landed surface (report Changes). `contract_ledger_probes.rs` and the per-set insta pin stay in §5/§6 unchanged: the "Verify-stamped test homes and harness" and "Fake-agent drift contract" entries own them.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
