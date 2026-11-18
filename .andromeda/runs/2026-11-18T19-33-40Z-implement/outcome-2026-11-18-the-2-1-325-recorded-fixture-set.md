# Outcome — 2026-11-18-the-2-1-325-recorded-fixture-set

**Outcome: green** (no fix iteration; smoke skipped — no boot-path change). Next → the OPERATOR PASS (entries 21
pre-push · 22 the hygiene read, `rec.py land`, the pre-CI commit · 23 the push · 24 the CI read), then the wrap in a
fresh window.

## Files changed
- `crates/viola-e2e/src/rig/mod.rs` — `Arm::VR` (labels `R4` / `R4-2`) and `Arm::MT` (`R4M`); `Plan.tap`; the
  `record-home` refusal for `VR` before the credentials read and the ledger line; the `record` keep before the
  teardown; `run_child` routing `MT` to `arm_mt`; `mcp_config_doc`, `drain`, `arm_mt`, `capture_lines`,
  `tap_fixture`; `finish` reading `mcp-child`; unit cases `the_recording_labels_take_their_arms_and_properties`,
  `the_mcp_config_names_the_tap_in_front_of_viola_mcp` (the `of_label("R4") == None` line removed).
- `crates/viola-e2e/src/rig/isolate.rs` — `Scratch::keep_record`, `names_home_component`, `read_bounded`; unit cases
  `keep_record_copies_the_set_byte_for_byte_and_never_overwrites`,
  `a_path_names_a_home_component_only_for_the_exact_word`.
- `crates/viola-e2e/src/bin/viola-live-rig.rs` — `rehearse --tap`; `live` gives `MT` the sibling `viola-mcp-tap`.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — the three rehearsals (steps 10–12).
- `tests/contract_ledger_probes.rs` — `STAMPED: [&str; 3]` names `2.1.325`.
- `fixtures/claude/2.1.325/` — 28 files, a plain copy of R4's kept record (27) and R4M's `Mcp.tools-list-1.json`.

## Gates (trail `.andromeda/runs/2026-11-18T19-33-40Z-implement/gate-2026-11-18-the-2-1-325-recorded-fixture-set.json`)
- 1 `cargo fmt --all --check` green · 2 `cargo clippy … -D warnings` green (both re-fired green after the `STAMPED` edit)
- 3 `cargo nextest run -p viola-agent-claude --no-fail-fast` green
- 4 `run --unit` green · 5 `run --integration --filter 'binary(rig_rehearsal)'` green
- round 6–12: `round: COMPLETE · legs fired 2/2 · entries 24 · green 7 · red 0` (evidence/round-194010Z.txt)
- 13 `diff -r …` green · 14 `cmp …` green · 15 `grep -c "and the usage budget" …` green (exit 1, last line 0) ·
  16 `test -f …/SessionStart.resume-1.json` green · 17 `run --integration --filter 'binary(cli_verify)'` green ·
  18 `run --e2e --filter '…contract_ledger_probes…mcp_recorded_exchange…'` green (127 passed) ·
  19 `run --integration` green · 20 `run --e2e` green
- control `bt-absent` (entry 3, `--timeout 600`): `holds` — in place green, removed red; the named case
  `ledger::tests::stamps::verified_reads_the_browser_tools_row_absent_as_counting` ran (650/650) and FAILED, no
  TIMEOUT line (evidence/control-1.txt).
- No deferral. No `gate.py guard` entry in the block.

## Live starts (evidence/fixture-starts.ndjson, evidence/fixture-round.md)
R4 19:40:12Z GREEN (24/24, failing none, browser-tools-prompt absent, screen-b..f box, record 27, teardown whole) ·
R4M 19:40:47Z GREEN (mcp-child exit 0, capture-lines 4, tap-fixture written, teardown whole). Spare `R4-2` unspent.
No byte typed into any CLI dialog; nothing written under the real `~/.claude*` or `~/.viola`.

## Deviations
- `arm_mt` waits up to 5 s (`MCP_LEAVE`) after the child exits for the tap and `viola mcp` to leave by themselves
  before the teardown census (the plan's notes expect them to exit on the CLI's close of stdin; without the wait a
  slow exit would read as `1 killed`). Not in the plan's step 7; in-intent, bounded, kills nothing.
- `arm_mt` writes no `viola-build` line (the plan's step 7 list holds none); `stopped-by` is put when a stop came.
- `tap-fixture: none` is read when the capture holds 0 lines (absent or empty), the plan's «no capture».
- The landed set's files are `chmod 0644` after the copy (the kept copies are 0600); bytes unchanged
  (entries 13/14 read them equal).
- Before the round I read the real `~/.claude/.credentials.json`'s `expiresAt` once by hand (140 min left) to check
  the token window — a read beside the rig's own, nothing kept or written; the directive names only the rig's read.
- Scope record: `crates/viola-e2e/tests/rig_rehearsal.rs` · in-intent · serves step 10 · self (research's list omits
  it; the plan lists it). `gate.py scope`: clean — changed 33 · listed 32 · recorded 1.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| R4's `viola verify` and its `claude` children | a leg (entry 9) | terminated — the rig's `census: 0` |
| R4M's `claude -p`, `viola-mcp-tap`, `viola mcp` | a leg (entry 11) | terminated — the rig's `census: 0` |
| the control's clone build and nextest | this run | terminated — `clone removed` |
`gate.py census --comm claude --comm viola --comm viola-mcp-tap --comm viola-live-rig --comm viola-fake-agent --args
viola-rig-1000 --args projects/viola/target/debug/viola`: 16 running, none this run's (the host's other Claude
sessions, the prototype's `viola run` wrappers, `~/.viola/bin/…/viola mcp`, `~/.local/viola-product/bin/viola wait`).

## Decisions & corrections
- An Edit anchored on text the rustfmt PostToolUse hook had reflowed missed (string not found); re-read and re-applied
  (testing.md 2026-09-25's reflow hazard, met again).
- A helper script written into the run dir named absolute host paths; moved to the session scratchpad before hygiene.
- The token-window read by hand above.
