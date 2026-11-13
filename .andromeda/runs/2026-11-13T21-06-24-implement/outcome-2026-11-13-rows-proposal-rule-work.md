# Implement outcome — 2026-11-13-rows-proposal-rule-work

**Outcome: green + smoke.** Next → the OPERATOR PASS (entries 33 hygiene · 34 push · 35 the `ci.py conclusion`
read), then `/andromeda-wrap-session`.

## Gates
- P2 firing 1 (the whole block, `{run_dir}/gate-2026-11-13-rows-proposal-rule-work.json`):
  `entries 35 · green 31 · red 0 · recorded 1 · timeout 0 · not-run 3` — one iteration, no fix. Entry 1 recorded
  `tmpfs 32G 10G 22G 32% /tmp`. The three not-run are `not run — leg operator (the letter drives it)` (33, 34, 35).
  Header: `delta docs 222 · rust 26` · `walk-class rust 7 (…) · uncommitted 248`. No deferral (Rust, the lockfile
  and tests all carry delta); pre-push (32) green in 105.69 s.
- No `gate.py guard` entry in the block.
- Matrix: `select * from coverage where chunk = '2026-11-13-rows-proposal-rule-work'` → 0 rows: the chunk owns no
  capability, so no `ref` / `implemented` write was made.

## The six controls (step 12) — each `gate.py control` alone, under a kill deadline
Form: `timeout -k 30 3300 python -X utf8 {tools_dir}/gate.py control --chunk 2026-11-13-rows-proposal-rule-work
--controls-file {run_dir}/control-{k}.toml --run-dir {run_dir} --timeout 1500`, backgrounded; every outer exit 0.
The tool names its clone on its head line, `clone ~/.cache/andromeda-control/{8 hex} · HEAD cb0b16ab · copied 255 ·
removed 0 · working folder {n} files · free {G}` — a fresh 8-hex name per call, every one removed at the end.

| k | control · entry | clone | entry line · control line |
|---|---|---|---|
| 1 | link-replay · 13 | b6a89d02 | `entry 13 in place green · 20.22s` · `holds — in place green, removed red · exit 0 ✗ (exit 100)` |
| 2 | strict-symlink · 10 | 18908c2a | `entry 10 in place green · 9.8s` · `holds — … removed red · exit 0 ✗ (exit 100)` |
| 3 | append-nofollow · 10 | 342ec9d7 | `entry 10 in place green · 9.73s` · `holds — … removed red · exit 0 ✗ (exit 100)` |
| 4 | record-email · 11 | d37650e0 | `entry 11 in place green · 7.63s` · `holds — … removed red · exit 0 ✗ (exit 100)` |
| 5 | hygiene-email · 14 | fae01c38 | `entry 14 in place green · 13.64s` · `holds — … removed red · exit 0 ✗ (exit 100)` |
| 6 | redact-hook · 11 | b800aa4d | `entry 11 in place green · 7.66s` · `holds — … removed red · exit 0 ✗ (exit 100)` |

Each summary: `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`. Each removed
red was read from its log and is a test assertion, never a build error (evidence/control-{1..6}.txt):
link-replay `tests/cli_links.rs:268`; strict-symlink the six link cases of step 5; append-nofollow
`fs.rs:383`; redact-hook `hook.rs:326`.
**Two partial witnesses**: nextest's default fail-fast ends a removed firing at its first red, so
- record-email's red came from the older screen-scrub cases (`screen_text_fault` now reaches the email check
  through `text_fault`); step 2's own case `unclean_refuses_an_email_in_a_string_or_a_key_after_a_path` never ran
  (261/610). Isolating it needs a seventh control (keep `screen_text_fault`'s own arm) — outside the six, not fired.
- hygiene-email read red on the agents, hook-payload and MCP planted cases; the status-line and fake-script email
  cases were not reached (62/93).

## Smoke
Recorded from P2: entries 26–31 (cleanup · boot `p-rules-smoke` · status · G2 · G4 · cleanup) green; the final
cleanup `processes_gone true · endpoint_gone true · killed []`. Not driven a second time.

## Files changed (26, every one in research's lists; scope `clean`, record 0)
New: `tests/support/digits.rs`. Modified: `src/human/hints.rs`, `src/cmd/send.rs`, `src/cmd/answer.rs`,
`src/cmd/run.rs`, `tests/cross_exit_causes.rs`, `tests/cli_answer.rs`, `crates/viola-agent-claude/src/ledger.rs`,
`crates/viola-agent-claude/src/ledger/test_cases_checks.rs`, `tests/support/hygiene.rs`,
`tests/contract_fixture_hygiene.rs`, `crates/viola-state/src/strict.rs`, `crates/viola-state/src/fs.rs`,
`tests/tui_pty_seam.rs`, `tests/tui_channel_fds.rs`, `tests/contract_os_gated_tests.rs`,
`crates/viola-agent-claude/Cargo.toml`, `crates/viola-agent-claude/src/hook.rs`,
`crates/viola-agent-claude/src/dialog.rs`, `crates/viola-mcp/Cargo.toml`, `crates/viola-mcp/src/tools.rs`,
`Cargo.lock`, `tests/support/mod.rs`, `tests/security_negatives_{server_identity,socket_dir,strict_modes}.rs`.

## Deviations from the plan, with justification
1. **Step 9 — three MCP input types do not derive `Redact`.** veil 0.3.0 refuses `#[derive(Redact)]` on a type
   with no `#[redact]` field (a compile error: "`#[derive(Redact)]` does nothing by default, you must specify at
   least one field to redact"). `WaitInput`, `LastInput` and `ListInput` hold no content field, so they keep
   `JsonSchema` alone, as before; `SendInput` (`text`) and `AnswerInput` (`response`) derive it. The plan's "the
   five input types derive `Redact`" is wrong against the pinned crate. Probe 23 still reads `tools.rs`.
2. **Step 9 — `hook::Known` redacts `prompt` and `last_assistant_message`;** the unit case covers it beside
   `Normalised`. `Known` was named in the step; the plan's field list names those two.
3. **Step 1 — `tests/cross_exit_causes.rs`.** `unverified-cli` (exit 12) is not an exit-1 / exit-21 cause, so it
   sits in its own `REFUSALS` table read by `no_two_codes_share_a_hint` and `no_hint_names_release`, and
   `every_code_is_a_code_word` checks its exit in `10..=14`, rather than widening the `1 | 21` assert.
4. **Step 1 — `tests/cli_answer.rs`.** The pinned human-mode refusal is `not-delivered  unknown-dialog` with its
   hint: no new `not-delivered` detail is reachable from `answer` end to end (the wrapper's answer refusals are
   control-character, human-typing, unverified-cli, unknown-dialog); the new details are pinned by the unit table
   of `answer_hint`.
5. **Step 8** — the 0-2 membership assert on the `fds` receipt goes with the receipt read, as the step says; the
   `/proc` table asserts stand unchanged (no membership assert added on the table).
6. **Plan Implementation notes** state the `digit_runs` and `fn parse_name` hits correctly; the operator's phase
   directive (inputs#I1 item 1) said `fn digit_runs` was defined twice — three at HEAD (the plan and scope already
   corrected this).

## Surfaced
Nothing surfaced as a spec↔reality gap. For the wrap: obs-plan · decision/payload-types-derive-redact as written
("the hook and MCP payload types derive veil `Redact`") cannot hold for a payload type with no content field under
veil 0.3.0 — deviation 1.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `p-rules-smoke` harness session (supervise, wrapper, fake agent) | this run (entries 26–31) | terminated — cleanup `processes_gone true`, `killed []` |
| six `gate.py control` clones and their builds | this run | terminated — each `clone removed` |
| 22 processes in `gate.py census` (the host's `claude` overseers and builders, the prototype's and product's `viola` sessions and `wait`s, `andromeda-batch` agents) | NOT DERIVED — none is this repository's build (no `projects/viola/target` or `andromeda-control` arg) | left running — not this run's |

## Decisions & corrections
- The operator's directive for this run: no live `claude` start and no cargo-mutants run (none made); the six
  controls each a separate `gate.py control` call under a kill deadline (`timeout -k 30 3300`, the tool's
  `--timeout 1500`); every long run journalled in `{run_dir}/journal.md` before it was waited on.
- Guard refusals met: bash-guard refused a `cd` into the skill's references dir and a `cat` heredoc to a file; each
  re-issued once (absolute paths; the Edit tool).
- A self-made stall: a stray `cat > /dev/null` blocked a script on stdin to the Bash tool's 120 s bound; stopped
  (TaskStop) and re-run from a scratchpad file. Nothing was written by the stalled call.
- Standing learning met: testing.md 2026-11-13 (a control clone holds no build output) — entries 13 and 14 are
  `-p viola --features fake-agent`, which builds the root package, and both reproduced in their clones.
