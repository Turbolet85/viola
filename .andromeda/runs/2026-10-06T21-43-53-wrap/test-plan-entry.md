## 2026-10-06-local-command-and-paste-framing-rows — seventeen rows, the framing tier, the fake agent's two options, Path 2 step 3 as built
**Section:** §1 (Critical paths, confirmed send; the contract-test trigger) · §2 pyramid (Contract row) · §4 viola-agent-claude · §5 CLI (`cli_verify.rs`, `contract_ledger_probes.rs`) · §6 Path 2 (step 3, its verification bullets) · §7 (Fixture library, Seed strategies, Fake agent, Fixture hygiene) · §3 → 5-command implementation · §3 → Bootstrap phases
**Change:**
- Counts: seventeen `[NN/17]` step lines and `LEDGER_ROWS: [&str; 17]` (was fourteen); one stamped set, `2.1.287`, ending `17 pass  0 fail`; `2.1.283` and `2.1.288` drift-only. Boot step 4 and the contract pass `--dialogs --framing`. The live `run --local-live` firing is owed at seventeen rows.
- The framing tier joins the recorded set: `UserPromptSubmit.paste-<n>.json`, `SessionEnd.clear-<n>.json`, `SessionStart.clear-<n>.json`, recorded by Run B's three added pastes, four under `2.1.287`; the drift contract replays them byte for byte under `--framing`, and a set without them is run without the option and must exit 1.
- Fake agent: seven argv options (was five): `--framing` and the test-only hold `--paste-hint-ms` (capped at 8 000 ms). A replayed `paste-<n>` variant keeps its recorded `prompt`.
- §4: the unwrap's eight framing cases and the three named arm cases; a stamp of the fourteen older ids reads unverified.
- §6 Path 2 step 3: was "longer than the fixture's paste-wrap threshold, so the fake agent emits the form"; now the recorded `paste-1` variant replayed under `--framing` (the fake agent has no threshold); the wrapped half is `send_long_text_wrapped_by_the_cli_is_confirmed`, the literal-tag half is not built.
- `unconfirmable` for a local command is owed to "Local-command send outcomes" (was "Local-command and paste-framing rows"): the row is compiled, `send` does not consume it yet.
- Bootstrap key: the `verify_window_` class holds two cases; verify's quiet waits are ten (was seven when the kill was sized); no bound moved.
**Why:** the plan's inventory follows what the chunk landed and what its P4 split moved to the next entry. The 20 s kill stands on a measured reading: the longest verify-driven test read 9.98 s on the ubuntu leg in ci#37534758441.
**Kept:** `.config/nextest.toml` and `tests/support/verify.rs` still say "seven 300 ms settles" in a comment; the wrap touches no source, the correction is pinned on the next route entry.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
