## Relevant amendment history
- **2026-10-05-real-cli-verify-probes** — ten ledger rows, verify's interactive runs and the screen fixture class. This is the immediate base this chunk extends:
  - boot step 4 runs verify with `--screens --turn-stop --trusted-root`;
  - `--local-live` checks ten literal `LEDGER_ROWS` (unit-proven, live firing owed to "First live test and self-drive");
  - `cli_verify` has 22 cases on `[NN/10]`;
  - `contract_ledger_probes` stamps 2.1.287 and 2.1.288 at `10 pass  0 fail`, with 2.1.283 drift-only and every dir on exactly one list;
  - the hygiene walk adds the 2.1.288 set, `Screen.*.json` against `claude-screen.v1.json`, and the email / seam-split-username refusals with named codes;
  - nextest `profile.mutants` gives `test(/verify_window_/)` 15 s × 2. A new verify-window case for the dialog re-probe falls under it.
  - It re-homed the relayed dialog captures, the `permission` e2e case and its wake witness to this entry by name. Every `/10` literal, list and owner site this chunk moves was set here.
- **2026-10-04-dialog-answers-by-dialog-id** — relayed dialog fixtures, the 2.1.287 default, Paths 3 and 4 as landed. This introduced what this chunk supersedes:
  - the relayed `fixtures/claude/2.1.287/` `PreToolUse.*` / `PermissionRequest.*` set and `RELAYED.md` (`tool_input.plan` redacted, reviewed before commit), ruled R1 because print mode raises no dialog hook. The contract suite checks both the recorded spine and the relayed tier.
  - the moves to `DEFAULT_CLI_VERSION` / `RECORDED_CLI_VERSION` / `stamped_home` / `Wrapper::boot` 2.1.287;
  - matcher evaluation through `hook_commands(…, tool)`;
  - Path 4's landed `cli_answer.rs` cases over `path4.json`, with `permission` unit / insta only and its e2e and Path 3 wake witness owed here.
  It fixes what the re-probe must replace, and which default-version literals move if the recorded set's version changes.
- **2026-10-04-dialog-answers-by-dialog-id** (five perf rows on an unstamped session) — the perf session went from stamped to unstamped (`stamp: false`), so the `pre-tool-use` row times the unverified-CLI dialog path, and `gate --require perf` requires all five rows by name. Closing R2 changes what "verified" means for a dialog decision. Re-check that the perf row still times the path it claims, and that the session's stamp state stays deliberate.
- **2026-09-29-fake-agent-drift-contract** (the fake-agent drift contract, byte for byte; matchers and S8 moved):
  - It replaced insta-pinned hook sequences with `contract_fake_agent_drift`: receipt `stdin_hex` byte for byte against each recorded fixture, walked per set at run time, where an empty walk fails.
  - insta stays for decision bodies only, and fake-agent transcripts get no snapshot.
  - The S8 `annotations` assertion and the matchers moved to the dialog-answers chunk.
  It is the rule new dialog fixtures must fall under. A `--record` refresh moves the fixture files, never a snapshot copy.
- **2026-09-29-verify-stamped-test-homes-and-harness** (stamped root seam) — root `stamped_home` runs verify against the fake agent. It is `stamped` only on exit 0 plus a last line `stamped <ver>  <n> pass  0 fail`, and otherwise panics with codes. `StampedHome::unstamped` serves the unverified-path tests. Growing the row count, or making dialog rows a stamp precondition, flows through this seam. The rule that stamps come only from verify (§7 Seed strategies) is its stated basis.
- **2026-09-29-verify-stamped-test-homes-and-harness** (run --local-live specified) — this specified `--local-live`:
  - `live-in-ci` refusal under `CI` (exit 2, before any build);
  - the build into `target/harness`, then one verify against the real `claude`;
  - suite `local-live` passes on the literal row ids each naming one `  pass` line plus a `0 fail` summary;
  - closed codes `build` / `verify-exit-<n>` / `verify-exit-none` / `row-missing`;
  - a new closed value takes a Decisions Log entry.
  The harness literal this chunk may move, and the precedent for recording any new code.
- **2026-09-29-verify-stamped-test-homes-and-harness** (fake-agent default 2.1.283; perf session stamped; H2 owner re-named) — the fake agent answers at the version its fixtures were recorded at, so the wrapper's version gate reads a stamped version. That is the precedent for moving the default when a new recorded dialog set lands at another version.
- **2026-09-28-capability-ledger-and-viola-verify** — verify's lines pinned by literal asserts, the fake agent's print mode, the hygiene walk over the first recorded set:
  - `tests/cli_verify.rs` pins the step lines, the `stamped` summary and the `unable:`/`hint:` pairs by literal assert_cmd asserts;
  - print mode fires only the four spine `default` fixtures, the reason dialogs need the interactive run;
  - the hygiene walk is a run-time directory walk, not rstest `#[files]`, because rstest 0.27 refuses an empty glob.
  A new re-probe step or refusal joins the same way.
- **2026-09-24-fake-agent-and-test-data-fixtures** (fake-agent contract as built, consumer-first) — this set the operator rule: no fixture or payload shape is invented before a recorded fixture exists. It also set the payload path `<fixtures>/<cli-version>/<Event>.<variant>.json` and the class-only hygiene checker. The recorded dialog payloads, such as an ordinary-tool `PermissionRequest.<variant>.json`, define the shapes, not the plan.
- **2026-09-24-fake-agent-and-test-data-fixtures** (fixture naming and `--fixtures` root) — fixtures use the CLI's PascalCase hook event name (`PreToolUse.ask-question.json`) with no mapping table, and the fake agent joins `<cli-version>` under the parent `--fixtures` root. New dialog fixture names follow this form, and code wins where a doc invented a shape.
- **2026-10-04-wait-and-last** — Path 3 as landed records its surfaces with owed status. The dialog kinds' end-to-end wake witness was owed and later split: question and plan landed, and `permission` came to this entry. The `permission` wake witness closes that owed item, and the scenario text must record it as landed.
- **2026-10-04-running-turn-refusal** (Path 5 and Path 2 span the running turn) — a running turn refuses `send` (exit 13 `turn-running`), so every test that sends twice must end the first turn with a scripted Stop. The fake agent's interactive submit fires none. This is a trap for a `path4.json` permission case that submits a second prompt.
- **2026-10-04-running-turn-refusal** (the turn-running control negative) — the controls table now has five negatives, `answer` on an unstamped home → 12 among them. The chunk's new negative (`answer` on a home whose stamp lacks the dialog rows → 12 `unverified-cli`) extends this row, and no `FAKE_AGENT_HOOK_PANIC` setting may disable it.
- **2026-10-04-running-turn-refusal** (the fake agent quiesces its hooks before it exits) — the fake agent exits on `\x03` or stdin EOF only after a running hook finishes, and starts no other. A dialog hook parked by a gated script delays its exit, which bears on teardown in new dialog-tier tests.
- **2026-10-04-the-wheel** (the one-Rust-test example selects through --integration) — `--e2e` selects nothing and is a usage error (exit 2). A plan gate written with `--e2e` failed this way, so gate commands for `cli_answer` / `cli_verify` cases must use `run --integration --filter …`.
- **2026-09-29-sideloaded-conpty** (the Windows x64 seeded-home carve-out) — §7's rule that on-disk state is never hand-written has only ruled exceptions (chaos, `seed_conpty`). Any shortcut that writes `ledger/stamps.json` or a dialog-row stamp by hand would be a new carve-out needing a ruling. The extract's "stamps only from verify" constraint rests on this rule.
- **2026-09-24-observability-gates** — this declared the internal `schema-check` (G4) and `secret-scan` subcommands, their output shapes and closed class enum. They are the checks the extract binds for any new verify or diagnostic lines and for the canary's allowed homes.
- **2026-09-29-t15-07-57-wrap** — the test-plan Decisions Log left the body for the archive. The in-force items now stand in the body, and later closed-enum changes are recorded as amendments in place of a Decisions Log entry (2026-10-03 precedent). That is where a new `local-live` code or verify refusal code introduced here gets recorded.
