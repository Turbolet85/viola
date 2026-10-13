proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: '§3 → 5-command implementation (run, --local-live)'
    change: 'Suite local-live checks eighteen literal row ids (LEDGER_ROWS: [&str; 18], the seventeen named before, then `paste-newline` last); the eighteen-row literals are unit-proven (harness::run::tests, the LIVE_PASS proof reads eighteen); the live firing at seventeen rows (chunk 2026-10-08-first-live-test-and-self-drive, `stamped 2.1.287  17 pass  0 fail`) stays as that record, and the tmpfs live home stamped at seventeen rows reads `row-missing` until its next `run --local-live` firing re-stamps it at eighteen (five starts, under a cap of its own).'
    sidecar: 'test-plan §3 → 5-command implementation: local-live LEDGER_ROWS seventeen → eighteen ids (`paste-newline` last); the seventeen-row live firing kept as a dated record; the live home reads row-missing until re-stamped (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'The report changes the harness surface: "run --local-live checks eighteen literal ids (LEDGER_ROWS), so the tmpfs live home stamped at seventeen rows reads row-missing until its next firing re-stamps it" (Harness / gate surface; P5 card row 4, inputs#I3), and Counts / qualifiers moved names LEDGER_ROWS at eighteen. The key still states [&str; 17] and seventeen ids, so test-plan §3 lags the harness one-sidedly; obs-plan §3 holds only the generic [NN/MM] form (report sweep: obs-plan.md:739, no count), so no obs-plan change is owed.'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:53 (report sweep rows17 ×4, ledger-rows-lit)'
  - detector: D-tests-obs-harness
    severity: warning
    section: '§3 → 5-command implementation (boot, step 4)'
    change: 'boot step 4: Run B with its four added compiled pastes (the long text, the tag-like text, the newline text, then `/clear`), and verify stamps all eighteen ledger rows.'
    sidecar: 'test-plan §3 → 5-command implementation boot step 4: Run B three → four added compiled pastes; seventeen → eighteen ledger rows (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Harness / gate surface: "boot stamps through viola verify against the fake agent at eighteen rows (green, 5.36 s)"; Symbols / APIs: framing_turns is now [PROBE_LONG_PASTE, PROBE_TAG_PASTE, PROBE_NEWLINE_PASTE] then the local command; Counts / qualifiers moved: seventeen → eighteen rows, Run B four → five compiled pastes.'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:8 (report sweep rows17, runb-pastes)'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: '§5 Integration Test Strategy (tests/cli_verify.rs)'
    change: 'cli_verify pins `--framing` for Run B''s four added pastes; the eighteen `[NN/18] <row id> <row words>  pass|fail` step lines (the last `[18/18] paste-newline …`); a run without the framing replay failing only `local-command-clear` (the three paste rows pass on the fake agent''s echo; `stamped 2.1.0  17 pass  1 fail`); and once the screen drawn after the tag-like paste''s turn alone shows a modal, Run B pastes neither the newline text nor the local command (`16 pass  2 fail`, `paste-newline` failing with `local-command-clear`).'
    sidecar: 'test-plan §5 cli_verify: [NN/17] → [NN/18], Run B three → four added pastes, the paste rows passing on echo two → three, the tag-turn-modal case now withholds the newline paste too (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Counts / qualifiers moved: step lines [NN/17] → [NN/18], Run B four → five compiled pastes; Deviations: "the tag-turn-modal case now reads 16 pass 2 fail, with paste-newline failing too — the newline text sits behind the same input-box guard as /clear"; Spec claims disproved: the eighteen-row oracle "stamped 2.1.0  17 pass  1 fail" of verify_without_framing_fails_only_the_clear_row; Outcome (design / layouts / a11y) eighteen [NN/18] lines.'
    basis: '.andromeda/test-plan.md:656 (report sweep rows17, runb-pastes, step-counter)'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: '§5 Integration Test Strategy (contract_ledger_probes.rs)'
    change: 'contract_ledger_probes must list each of the eighteen literal ledger row ids exactly once and end `stamped <ver>  18 pass  0 fail`.'
    sidecar: 'test-plan §5 contract_ledger_probes: seventeen → eighteen literal row ids, `17 pass  0 fail` → `18 pass  0 fail` (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Outcome (tests): "contract_ledger_probes stamps 2.1.287 at 18 pass 0 fail; drift and hygiene pass it (integration 94/94)"; Counts / qualifiers moved: the ledger seventeen → eighteen rows.'
    basis: '.andromeda/test-plan.md:666 (report sweep rows17 ×2)'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy (the ledger framing arms)'
    change: 'The ledger''s four framing arms (`long-paste-wrapper` · `tag-escaping` · `paste-newline` · `local-command-clear`), each reading its Run B capture; `paste-newline` (`pasted(&probes.trusted, PROBE_NEWLINE_PASTE)`: the first trusted-run UserPromptSubmit whose prompt normalises to the whole text) in its own three cases, `check_paste_newline_passes_on_the_one_prompt_holding_the_lf`, `check_paste_newline_fails_when_the_lf_split_the_prompt` and `check_paste_newline_fails_without_a_matching_capture`.'
    sidecar: 'test-plan §4: the ledger framing arms three → four, `paste-newline` and its three check_paste_newline_ unit cases named (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'The chunk adds a new code path, LedgerRow::PasteNewline (Symbols / APIs; FRAMING_ROWS gains PasteNewline, Deviations step 2), unit-tested by three check_paste_newline_ cases (Coverage of new surfaces; Outcome: filter reads passed 3, failed 0). §4 names only three framing arms and their three cases, so the new arm has no tier statement in the plan.'
    basis: 'crates/viola-agent-claude/src/ledger.rs:2506-2553 (report New text, by line: the three check_paste_newline_ rows)'
  - detector: D-tests-coverage
    severity: warning
    section: '§2 Test Strategy (contract-test-against-sandbox trigger, Required test type)'
    change: 'Live viola verify refreshes the framing variants through Run B''s four added pastes (`paste-<n>`, `clear-<n>`; the newline text recorded as `paste-3`), locally.'
    sidecar: 'test-plan §2 contract trigger: Run B three → four added pastes, the newline paste recorded as paste-3 (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Symbols / APIs: FRAMING_TURNS now four in paste order, paste-3 the newline text, framing_variants names its capture paste-3 for --record; Schema / config: the 2.1.287 set gains UserPromptSubmit.paste-3.json. The §2 tier statement must cover the new paste path at the contract tier.'
    basis: '.andromeda/test-plan.md:408 (report sweep runb-pastes, framing-variants ×3)'
  - detector: D-tests-coverage
    severity: warning
    section: '§2 Test Strategy (Contract tier row)'
    change: 'The framing-tier fixtures are recorded by Run B''s four added pastes as `UserPromptSubmit.paste-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` (stems `paste`, `clear`; five under `2.1.287`, none under `2.1.288`).'
    sidecar: 'test-plan §2 Contract row: Run B three → four added pastes; 2.1.287 framing variants four → five (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Counts / qualifiers moved: "The 2.1.287 stamped set''s framing variants: four → five (paste-1, paste-2, paste-3, clear-1 ×2). The set holds 29 files (28 before)"; Run B four → five compiled pastes.'
    basis: '.andromeda/test-plan.md:438 (report sweep runb-pastes, framing-variants)'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures (Recorded hook payloads row)'
    change: 'Run B''s framing payloads as `UserPromptSubmit.paste-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` (five today, under `2.1.287` only).'
    sidecar: 'test-plan §7 Recorded hook payloads: framing payloads four → five under 2.1.287 (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Schema / config and Counts / qualifiers moved: the 2.1.287 framing variants four → five with UserPromptSubmit.paste-3.json, recorded live and scrubbed.'
    basis: '.andromeda/test-plan.md:1067 (report sweep framing-variants)'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures (fake agent: bracketed pastes)'
    change: 'Under `--framing` a compiled paste text of verify''s Run B fires its recorded `paste-1` / `paste-2` / `paste-3` variant instead, its bytes unchanged.'
    sidecar: 'test-plan §7 fake agent pastes: the --framing variants paste-1 / paste-2 → paste-1 / paste-2 / paste-3 (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Symbols / APIs: "framing_stem gives the new text the stem paste-3 … The fake agent reaches the new stem through framing_stem with no change of its own."'
    basis: '.andromeda/test-plan.md:1078 (report sweep framing-variants)'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures (fake agent modes, --framing)'
    change: '`--framing` (Run B: a pasted compiled paste text fires its recorded `paste-1` / `paste-2` / `paste-3` UserPromptSubmit variant with its bytes unchanged, …).'
    sidecar: 'test-plan §7 fake agent --framing option: paste-3 named beside paste-1 / paste-2 (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Symbols / APIs: FRAMING_TURNS gains paste-3 (the newline text) and the fake agent reaches it through framing_stem; no new fake-agent option (no new IPC method, endpoint, port, env var, CLI flag or fake-agent option), so only the variant list changes.'
    basis: '.andromeda/test-plan.md:1093 (report sweep framing-variants)'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures (the fixture hygiene walk)'
    change: 'The walk covers `2.1.287`''s five recorded framing variants, `UserPromptSubmit.paste-1.json`, `UserPromptSubmit.paste-2.json`, `UserPromptSubmit.paste-3.json`, `SessionEnd.clear-1.json` and `SessionStart.clear-1.json`.'
    sidecar: 'test-plan §7 hygiene walk: 2.1.287 framing variants four → five, paste-3 listed (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Files: tests/contract_fixture_hygiene.rs is a recorded companion serving fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json (its four-variant list was a shape-pinned site, Spec claims disproved); Outcome: contract_fixture_hygiene green.'
    basis: '.andromeda/test-plan.md:1101 (report sweep framing-variants ×2)'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → Bootstrap phases (derive for route / setup-project) (quality-gate-config-emit, profile.ci overrides)'
    change: 'Keep the 2026-10-06 sizing sentence as its record, and add: since chunk 2026-10-13-paste-newline-ledger-row Run B makes four added turns (the newline paste joined); the integration durations read 34.69 s against 35.4 s at P5 and no kill line moved.'
    sidecar: 'test-plan §3 → Bootstrap phases: Run B three → four added turns recorded beside the 2026-10-06 sizing, no kill line moved (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Counts / qualifiers moved: Run B three added pastes → four added pastes then /clear, and a verify run under the fake agent gains one turn (hooks 35 → 37, Stop 8 → 9); Outcome: "The integration durations were read as the plan asked: 34.69 s against 35.4 s at P5; no kill line moved."'
    basis: '.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10 (report sweep runb-pastes)'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy (Path 2, a text ending in newlines / the inner-LF control)'
    change: 'The control that an LF typed inside a bracketed paste is submitted unchanged (`rule-inner-lf`) is now covered by the ledger row `paste-newline` (Run B''s `paste-3` probe, stamped `18 pass  0 fail` on live `claude` 2.1.287 on the Linux dev host at chunk 2026-10-13-paste-newline-ledger-row); its Windows reading is owed to "Windows-only live measurements".'
    sidecar: 'test-plan §6 Path 2: the inner-LF-in-a-paste shape no longer unrowed — the paste-newline ledger row covers it (Linux 2.1.287), Windows owed to "Windows-only live measurements" (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Counts / qualifiers moved: "the LF typed inside a paste now has its row"; Cross-cutting claims: claude 2.1.287 on Linux submitted the 113-byte text holding one inner LF as one prompt equal to it byte for byte, measured on Linux only; P5 card row 5 (inputs#I3): the Windows reading goes to "Windows-only live measurements". The sentence saying no ledger row or verify probe covers the shape is retired.'
    basis: '.andromeda/test-plan.md:752 (report sweep inner-lf-owed ×3)'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures (fake agent modes, the resume SessionStart payload)'
    change: 'No reading became a fixture or a ledger row; the recorded payload is owed to the route entry "Owed ledger rows" (the `--resume` shape it carries from "Paste newline ledger row").'
    sidecar: 'test-plan §7: the owed recorded resume SessionStart payload re-pointed from "Paste newline ledger row" to "Owed ledger rows" (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Inputs I3, P5 card row 1: a new route entry "Owed ledger rows", directly after "Paste newline ledger row", carries its CARRY 4–8 verbatim (`--resume`, `/compact`, the statusline override and `/bin/sh`, `rate_limits`, `claude agents --json`); I4: it is minted from the P5 card as the founder answered (inputs#I2). This chunk lands only the LF row, so the payload still owed now sits on the new entry.'
    basis: '.andromeda/test-plan.md:1093 (report sweep inner-lf-owed)'
    dependent-of: D-tests-coverage
note: 'D-tests-framework reads no drift (nextest, assert_cmd, rstest, the harness commands unchanged). Report sweep hits left unchanged as true unrelated claims: test-plan.md:566 (the seventeen typed_text cases of 2026-10-09), :1056 and :1076 (the generic paste-<n> form, no count). The §4 framing-arms site is not in the report sweep, so its proposal names the section without a line. The last proposal assumes the orchestrator mints "Owed ledger rows" this wrap (inputs#I4); if it does not, hold that proposal.'
