# Codebase Research — 2026-09-29-fake-agent-drift-contract

## Scope
- **Depth:** deep · **Reads:** 16 · **Globs/Greps:** 17
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`, read in full (auto-loaded), 6 Session Additions applied. The fake-agent section and the "must not drift from recorded fixtures" line bind here. The 2026-09-27 `test(=tests::name)` addition applies to any inline-test filter.
  - `.claude/rules/testing.md`, read in full (auto-loaded), 20 Session Additions. Of these, 2026-09-24 (wait on the exact line), 2026-09-25 (guard neutralisation), 2026-09-27 (force a timing window, never sample it) and 2026-09-28 (never raise a bound) apply.
- **Platform issues consulted:** none. No runner-only bullet: CI on `2d8bc53` (ci#36571737771) was still in progress at Setup, with no red relayed.

## Files inspected
- `src/bin/viola-fake-agent.rs` (full, 721 lines @ `2d8bc53`) — the fake agent. `hook_commands` (:156-182) takes every `type:"command"` entry of every group and ignores `matcher` (its own test at :693-694 feeds a `"matcher":"m"` group and expects its command run). `payload` (:185-194) re-serialises the fixture object with `to_string()`. `run_hook` (:230-262) receipts `hook` with `event, command_absolute, ran, exit_code, stderr_len, stdout_hex` and no stdin bytes. `receipt_size` (:463-473) is called only at :492 (before the first read) and :494 (after each byte read). `print_turn` (:477-484) fires SessionStart → UserPromptSubmit → Stop → SessionEnd `default`. `main` (:511) makes no `restrict_dll_search` call.
- `crates/viola-agent-claude/src/hook.rs` (:80-214, :216-446):
  - `HARNESS_PREFIXES` (:90) is `["<agent-message from=", "<task-notification>"]`. `prompt_origin` (:151-157) is a raw `starts_with` with no trim. `data_of` (:134-137) classifies on the raw prompt, then `prompt_text` unwraps and unescapes.
  - `unescape_tags` (:201-214) drops the backslash of `<\` before an ASCII letter or `/`.
  - The case table `prompt_origin_reads_the_raw_prefix` (:391-401) pins `<\task-notification>x` → `human`.
- `crates/viola-agent-claude/src/ledger.rs` (:55-214) — `CAPTURE_EVENTS` (:63-68) is the spine order, and `spine_in_order` (:199-207) is the `spine-hooks` post-condition. Fixtures are written only when every row passes, so the recorded ORDER is this row's measured fact, not a property of the fixture files.
- `src/cmd/verify.rs` (:291-323) — `record` writes each scrubbed payload as `payload.to_string()` + `'\n'`.
- `fixtures/claude/2.1.283/*.json` (4 files, read whole) — SessionStart, UserPromptSubmit (prompt = the probe prompt), Stop and SessionEnd `default`. Every file ends in `0a` (`tail -c1 | od`). None carries a `tool_name`.
- `schemas/claude-fixture.v1.json` (full) — tolerant, `required: [hook_event_name]`, with the nine-event enum.
- `tests/contract_fixture_hygiene.rs` (full, 249) — `claude_fixtures_pass_hygiene` (:46-59) walks every `fixtures/claude/*/*.json` at run time against `schemas/claude-fixture.v1.json` (the absolute-path, username and schema classes, each with a planted red at :106-138). The fixture schema check therefore exists, and it iterates every version.
- `tests/contract_ledger_probes.rs` (full, 71) — `recorded_versions` (:24-34) plus verify over each set, with 6 literal row ids.
- `tests/support/fake.rs` (full, 115) — `FAKE`, `RECORDED_CLI_VERSION`, `receipt`/`of_kind`/`wait_for`, `write_plugin` (one event, no matcher), `write_fixture` (synthetic) and `unhex`.
- `tests/tui_pty_seam.rs` (:58-75) and `tests/tui_passthrough.rs` (:120-169) — the two consumers of the `size` receipt. Both drive a key after the resize to make the child re-read its size (`tui_passthrough.rs:149` comment: "Each key makes the child re-read its size").
- `plugin/hooks/hooks.json` (full) — seven exec-form groups with no `matcher` key.
- `Cargo.toml` (:9-135, :180-193) — feature `fake-agent = []`; root `[[test]]` entries carry `required-features = ["fake-agent"]`; dev-deps are interprocess, jsonschema, rstest, serde_json, sha2, sysinfo and tempfile. **insta is not a dependency anywhere** (`grep -rn insta Cargo.toml crates/*/Cargo.toml` → 0; `grep -c 'name = "insta"' Cargo.lock` → 0). `serde_json` is `=1.0.151` with `preserve_order`.
- `D:/dev/projects/additional/viola-lab/prototype/src/hook.rs` (:81-100, :192-206; outside this repository, read-only) — the prototype's `harness_injected`:
  - it trims the start, then matches `<agent-message `, `<task-notification>`, `<\cross-session-message` or `<cross-session-message`;
  - its comment: "A cross-session message (another Claude session's SendMessage) arrives escaped as `<\cross-session-message` (measured 2026-09-28/29 on andromeda-worker, overseer1's F115)";
  - its test: `<\cross-session-message from="uds:x" from-name="overseer1">` → harness, and `typed <\cross-session-message later in a line` → not harness.
- `refs/viola-brief.md` (:57, :149, :234-242) — M2, the harness prefixes. M4: the CLI escapes typed tag text, and the builder "also reported the harness rewriting the prefixes `<agent-message from=` and `<task-notification>` inside a subagent's returned text".
- `.andromeda/test-plan.md` (:400-408, :866, :1100, :1328-1341, :1365) — the §6 contract suite wording. S8 `annotations` is placed in the **question answer path**: PreToolUse `updatedInput.answers` + `annotations`.
- `.andromeda/architecture.md` (:45, :63, :76, :91, :357) — the S8 row, "free text and `annotations`", lands **with dialog answers**. Harness prefixes and tag escaping land with the first live test. The PreToolUse matcher is `AskUserQuestion|ExitPlanMode`, and it joins with the dialog chunk.
- `claude --version` → `2.1.283 (Claude Code)`: the installed CLI equals the recorded set.

## Graph impact (from the code-graph query; trace `tree-query-2026-09-29-fake-agent-drift-contract.json`, plane rust)
- **prompt_origin** — 1 caller, `hook/data_of` @ `crates/viola-agent-claude/src/hook.rs:136`. The change stays inside the crate. The root caller `cmd/hook/handle` reaches it only through `normalise` (@ `src/cmd/hook.rs:138`).
- **payload** (fake agent) — 1 production caller, `Agent::fire` @ `src/bin/viola-fake-agent.rs:223`, plus its unit test (:711-714). The `viola-e2e` `perf.rs` `payload` hits are a name collision (a different fn in `crates/viola-e2e/src/harness/run/perf.rs`), and they are out of scope.
- **receipt_size** — 2 call sites, both in `read_stdin` @ `src/bin/viola-fake-agent.rs:492,494`.
- **hook_commands** — 1 production caller, `Agent::fire` @ `:209`, plus its unit test (:697-704).
- **fire** — called from `submit` (:271), `run_steps` (:315), `print_turn` (:479-482) and `main` (:544).
- `normalise` callers are the in-crate tests plus `cmd/hook/handle` (unchanged signature).
- No signature changes cross a crate boundary.

## Patterns detected
- **The recorded order is a ledger fact** (`ledger.rs:62-68`, `:199-207`). The contract's expected hook sequence is SessionStart → UserPromptSubmit → Stop → SessionEnd, written as a literal in the test per testing.md "Oracles are literals". It is never imported from `CAPTURE_EVENTS`.
- **The recorder's byte form** (`src/cmd/verify.rs:319-320`): `to_string()` + `\n`. With `preserve_order`, the fake agent's `payload` for UserPromptSubmit re-serialises to the same bytes minus that trailing `\n`.
- **Run-time version walk** (`tests/contract_ledger_probes.rs:24-34`, `tests/contract_fixture_hygiene.rs:27-44`): the per-version iteration to copy. An empty walk fails.
- **Whole-object receipt pins** (`tests/cli_fake_agent.rs:317-318`, `tests/cli_instance_state.rs:377-378`). A new field on the `ran:true` `hook` receipt moves both. The `ran:false` pin at `tests/cli_fake_agent.rs:390` does not move if the field rides only a hook that ran.
- **Key-driven size oracle** (`tests/tui_pty_seam.rs:69`, `tests/tui_passthrough.rs:164-166`): both consumers send a key after the resize. A change-driven `size` receipt keeps them green, because they wait on the `size` line itself. They would simply stop needing the key.

## Conventions to follow
- **Test-only placement:** fake-agent changes stay in `src/bin/viola-fake-agent.rs` (feature `fake-agent`). New root contract tests go in `tests/contract_<topic>.rs` with a `[[test]]` entry carrying `required-features = ["fake-agent"]` (`Cargo.toml:14-114`).
- **Classifier tables:** rstest `#[case::label]` inline in `hook.rs`'s `mod tests` (:391-401). Harness-tag literals are written in the test.
- **Receipt discipline:** one `write_all` per line, `"v":1` plus a kebab `kind` (`viola-fake-agent.rs:108-119`). Codes and hex only.
- **Waits:** `fake::wait_for` over the receipt, bounded by `WITHIN`. No sleeps in tests.

## Mechanism re-derivations (closure of folded mechanism claims)
- **CARRY 2 "a resize with no later key writes no receipt"** (measured-marked, read at `90aba7c`) — VERIFIED at `2d8bc53`. `receipt_size` runs only at `viola-fake-agent.rs:492` and after `stdin.read` returns a byte (:493-494). `read` blocks, so a resize with no following byte writes no `size` line.
- **Payload equality (the contract's load-bearing equality)** — for SessionStart, Stop and SessionEnd, `fire` sends `fs::read(fixture)` unchanged (`payload(fixture, None)` returns it as is, :186), so stdin bytes == fixture bytes. For UserPromptSubmit with the recorded prompt, `payload` yields `obj.to_string()` with no trailing `\n`, while the recorder wrote `to_string()` + `\n` (`verify.rs:319-320`) and the file ends `0a`.
  - So at HEAD, UserPromptSubmit stdin bytes ≠ fixture bytes by exactly the final `\n`: **a live drift the contract will witness red before its fix.**
  - The claim that re-serialising a `preserve_order` object reproduces the recorder's bytes rests on both sides using serde_json `=1.0.151`'s `to_string`. It stays a HYPOTHESIS until the contract test's first run shows it.
- **The harness-origin claim (scope item 5)** — the tag literal, and the claim that the CLI injects it escaped, are corroborated only by the prototype's comment and test (outside this repo; measured 2026-09-28/29 on andromeda-worker per overseer1's F115, CLI version unstated there) and by the overseer's relay.
  - No recorded fixture and no ledger row witnesses it. `verify`'s probe is `claude -p` over the four spine events and cannot produce a cross-session message.
  - At HEAD an escaped `<\…` prefix is by design "typed by the user" (`hook.rs:149-150`; arch :49; brief M4). The measured injected form is ESCAPED, and a human who types the plain tag at a prompt's start produces the same escaped bytes (M4). So a classifier that files `<\cross-session-message` as harness also files such a typed prompt as harness.
  - The plain form `<cross-session-message` cannot come from typing, because the CLI escapes typed tags (M4). But it has no measurement of its own.
- **Observation (unmeasured at the hook layer):** in this orchestrating session, on the host's Claude Code 2.1.283, a subagent's hand-back reaches the model framed `Another Claude session sent a message:` followed by `<agent-message from="…">`. Whether the UserPromptSubmit `prompt` carries that preface was not measured. If it does, M2's `starts_with("<agent-message from=")` would miss it. This is a live-measurement question for "First live test and self-drive" (:76), which owns the harness-prefix ledger row.

## New files to create
- `tests/contract_fake_agent_drift.rs` — the drift contract. For each recorded `fixtures/claude/<ver>/` set, it runs the fake agent in print mode with the recorded prompt under a test plugin registering the four spine events. It asserts:
  - the `hook` receipts' event order equals the literal spine order;
  - each receipt's stdin bytes equal that event's recorded fixture bytes;
  - an empty walk fails.

## Files to modify
- `src/bin/viola-fake-agent.rs` — `payload` keeps the fixture's trailing newline. The `ran:true` `hook` receipt carries the stdin bytes as hex. Beyond that, what changes hangs on P4's forks: the matcher evaluation, and/or a change-driven `size` receipt.
- `crates/viola-agent-claude/src/hook.rs` — `HARNESS_PREFIXES` and `prompt_origin` for the cross-session-message tag (the form is P4's fork), plus the case table.
- `Cargo.toml` — a `[[test]]` entry for `contract_fake_agent_drift` with `required-features = ["fake-agent"]`.
- `tests/cli_fake_agent.rs` — the whole-object `ran:true` `hook` receipt pin (:317-318) gains the stdin field.
- `tests/cli_instance_state.rs` — the whole-object `ran:true` `hook` receipt pin (:377-378) gains the stdin field.

## Open questions
- Harness origin for the cross-session tag: is the ESCAPED form (the measured one, which a human can also produce by typing the plain tag at a prompt's start) classified `harness`, or only the plain form, or is item 5 deferred to the live-measured harness-prefix row (:76)? → blocks: plan-decision. It is a security boundary question, the "escaped-tag-means-typed" design (security history 2026-09-27-wrapper-channel: a widening needs a live founder answer).
- Matcher evaluation (CARRY 1): its premise, "where recorded `tool_name`s exist", is false at HEAD. Build it now against synthetic tool-bearing fixtures, or re-carry it to the dialog-answers entry, where PreToolUse `AskUserQuestion|ExitPlanMode` is the first consumer and tool-bearing fixtures get recorded? → blocks: plan-decision.
- Receipt `size` truth (CARRY 2): a change-driven `size` receipt (a watcher in the fake agent), or amend test-plan §7's wording to "at start and before a byte read after a change"? → blocks: plan-decision.

## Notes
- test-plan §6 says the contract is "pinned with insta", but insta is absent from the workspace (`grep -rn insta Cargo.toml crates/*/Cargo.toml` → 0 dependency lines). The recorded fixture files are themselves the pinned artifact the contract compares against. P4 weighs a new dev-dependency against a §6 wording amendment.
