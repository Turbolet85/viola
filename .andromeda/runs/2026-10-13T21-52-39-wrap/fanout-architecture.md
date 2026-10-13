proposals:
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [CLI Version Compatibility]'
    change: 'The ledger (`viola_agent_claude::ledger`, a closed set) holds eighteen rows, the list ending `local-command-clear` · `paste-newline`; add chunk 2026-10-13-paste-newline-ledger-row to the widening chunks; `paste-newline` (an LF inside a paste stays inside the one prompt submitted) is checked by `pasted(&probes.trusted, PROBE_NEWLINE_PASTE).is_some()`: the first trusted-run UserPromptSubmit whose prompt normalises to the whole text, so a CLI that submitted at the LF leaves no capture equal to it.'
    sidecar: 'CLI Version Compatibility: ledger seventeen → eighteen rows, `paste-newline` the eighteenth (chunk 2026-10-13-paste-newline-ledger-row).'
    rationale: 'Report Changes → Symbols / APIs: `LedgerRow::PasteNewline` is the eighteenth and last member of `LedgerRow::ALL` (`[Self; 18]`), its check arm stated; Counts / qualifiers moved: seventeen rows → eighteen, `cli_verified` gated by eighteen. The locked decision still names seventeen rows ending `local-command-clear`.'
    basis: '.andromeda/architecture.md:91'
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [CLI Version Compatibility]'
    change: 'verify "checks the eighteen rows" and prints one `[NN/18]` step line per row (`MM` from `LedgerRow::ALL.len()`, the last `[18/18] paste-newline …`), then the `stamped <ver>  N pass  M fail` summary.'
    sidecar: 'CLI Version Compatibility: verify checks eighteen rows, `[NN/18]` step lines.'
    rationale: 'Report Changes → Symbols / APIs (`viola verify` stdout: `MM` is now 18, derived from `LedgerRow::ALL.len()`) and Counts / qualifiers moved (`[NN/17]` → `[NN/18]`).'
    basis: '.andromeda/architecture.md:91'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [CLI Version Compatibility]'
    change: 'A non-`null` dialog decision needs a stamp whose eighteen rows pass: a stamp holding exactly the seventeen older ids (or the fourteen), all `pass`, reads unverified, so every home stamped before chunk 2026-10-13-paste-newline-ledger-row reads `cli_verified:false` until its next `viola verify`.'
    sidecar: 'CLI Version Compatibility: the dialog-decision stamp gate reads eighteen rows; a seventeen-id stamp reads unverified.'
    rationale: 'Report Changes → Symbols / APIs: `version_gate::stamps_verdict` over `ledger::verified` reads the stamp against all eighteen ids; a stamp holding only the seventeen older ids, all `pass`, now reads `cli_verified:false`.'
    basis: '.andromeda/architecture.md:91'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [CLI Version Compatibility]'
    change: 'The PermissionRequest `allow` WITH `updatedInput` body has no ledger row, and adding one needs a 19th row (not "an 18th row"), its `viola verify` probe and a re-stamp of every stamped version.'
    sidecar: 'CLI Version Compatibility: the hypothetical question-body row is now the 19th, the 18th being `paste-newline`.'
    rationale: 'Report Changes → Symbols / APIs: the eighteenth slot of `LedgerRow::ALL` is taken by `paste-newline`, so the sentence naming "an 18th row" for another body is stale.'
    basis: '.andromeda/architecture.md:91'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [CLI Version Compatibility]'
    change: 'The bullet list of ledger rows gains one bullet for the paste newline row (row `paste-newline`): an LF typed inside a bracketed paste stays inside the one prompt submitted, probed by Run B''s compiled `PROBE_NEWLINE_PASTE` and, as measured on live `claude` 2.1.287 on the Linux dev host at chunk 2026-10-13-paste-newline-ledger-row (`evidence/live-round.md`, one round, five starts), submitted as ONE prompt equal to the 113-byte text byte for byte; not measured on Windows.'
    sidecar: 'CLI Version Compatibility: ledger-row list names the `paste-newline` row and its Linux 2.1.287 measurement.'
    rationale: 'Report Changes → Symbols / APIs (`LedgerRow::PasteNewline`, `PROBE_NEWLINE_PASTE`) and Cross-cutting claims (the live CLI: one prompt equal to the text, Linux 2.1.287 only). The decision''s row list enumerates the landed rows (long-paste wrapper, tag escaping, local commands) and omits the new one.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [CLI Version Compatibility]'
    change: 'Run B gets five bracketed pastes, all compiled texts: `PROBE_PROMPT`, then `PROBE_LONG_PASTE` (1 500 bytes, one line), `PROBE_TAG_PASTE` (200 bytes), `PROBE_NEWLINE_PASTE` (113 bytes, ASCII, exactly one LF at neither end, no CR) and `PROBE_LOCAL_COMMAND` (the list''s `/clear`), each of the four added ones only into the settled input box the turn before left, none after the stop, each waited for one more UserPromptSubmit and one more Stop (`FRAMING_TURNS` `[(&str, &str); 4]`: `paste-1`, `paste-2`, `paste-3`, `clear-1`); the widening is the founder''s own answer at P4, relayed verbatim (`inputs#I2`).'
    sidecar: 'CLI Version Compatibility: Run B four → five compiled pastes, the newline text before `/clear` (the founder''s P4 answer).'
    rationale: 'Report Changes → Symbols / APIs (`typed.rs` `framing_turns`: added pastes `[PROBE_LONG_PASTE, PROBE_TAG_PASTE, PROBE_NEWLINE_PASTE]` then the local command; Run B pastes five compiled texts; `FRAMING_TURNS` now four entries) and Counts / qualifiers moved (four compiled pastes → five). Expected amendments names this site in other words at architecture.md:91.'
    basis: '.andromeda/architecture.md:91'
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: '`viola verify` runs in CI against the fake agent over all four typed runs and all eighteen ledger rows.'
    sidecar: 'CI/CD approach: fake-agent verify covers eighteen ledger rows.'
    rationale: 'Report Counts / qualifiers moved: seventeen rows → eighteen; sweep `rows17` lists `contracts/architecture/ci-cd-approach.md:20`; Harness / gate surface: `boot` stamps at eighteen rows.'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md:20'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Crate dependency direction'
    change: 'The pure `ledger` module holds the eighteen rows, Run B''s compiled paste texts (`PROBE_NEWLINE_PASTE` among them) and the local-command list (`LOCAL_COMMANDS`, `framing_stem`, `framing_variants`).'
    sidecar: 'Crate dependency direction: `ledger` holds eighteen rows.'
    rationale: 'Report Counts / qualifiers moved (seventeen → eighteen rows); sweep `rows17` lists `contracts/architecture/crate-dependency-direction.md:6`; Symbols / APIs: `pub const PROBE_NEWLINE_PASTE` new in the ledger module.'
    basis: '.andromeda/registries/contracts/architecture/crate-dependency-direction.md:6'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Cross-cutting Patterns → Capability ledger'
    change: 'The dialog answer mapping is gated by its own rows inside the eighteen-row `cli_verified` (not "seventeen-row").'
    sidecar: 'Capability ledger: `cli_verified` is eighteen-row.'
    rationale: 'Report Counts / qualifiers moved: `cli_verified` is gated by eighteen rows; sweep `rows17` lists architecture.md:472.'
    basis: '.andromeda/architecture.md:472'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Cross-cutting Patterns → Capability ledger'
    change: 'Two relied-on dependences have no row yet (`viola revive`''s and the statusline pass-through''s three shapes), not three: `send`''s LF typed inside a bracketed paste now has its row `paste-newline` with its `viola verify` probe (Run B''s newline paste), landed at chunk 2026-10-13-paste-newline-ledger-row and stamped on Linux 2.1.287 only (`stamped 2.1.287  18 pass  0 fail`, `evidence/live-round.md`); its Windows reading goes to the existing working-route entry "Windows-only live measurements".'
    sidecar: 'Capability ledger: the inner-LF dependence rowed (`paste-newline`); two relied-on dependences remain without a row; its Windows leg on "Windows-only live measurements".'
    rationale: 'Report Counts / qualifiers moved ("the LF typed inside a paste now has its row"); Cross-cutting claims (live 2.1.287 Linux only); inputs#I3 card row 5 sends the Windows reading to "Windows-only live measurements"; Expected amendments: Cross-cutting Patterns (Capability ledger), sites `inner-lf-owed` architecture.md:472.'
    basis: '.andromeda/architecture.md:472'
  - detector: D-arch-decisions
    severity: warning
    section: '§Cross-cutting Patterns → Capability ledger'
    change: '`viola revive`''s `claude --resume <id>` row stays owed to `v1-34`, now on the working-route entry "Owed ledger rows" (minted directly after "Paste newline ledger row" in Epoch 5, carrying its CARRY 4–8 verbatim; the founder''s answer to fork 3, `inputs#I2`, and the operator''s word, `inputs#I4`), no longer on "Paste newline ledger row".'
    sidecar: 'Capability ledger: the `--resume` row owed on "Owed ledger rows".'
    rationale: 'Report Changes → Inputs I3 row 1 (route insertion "Owed ledger rows" carrying CARRY 4–8: `--resume`, `/compact`, the statusline override and `/bin/sh`, `rate_limits`, `claude agents --json`) and I4 (minted at this wrap); Counts / qualifiers moved ("the Owed ledger rows entry carries the rest"); sweep `owed-rows` architecture.md:472.'
    basis: '.andromeda/architecture.md:472'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Cross-cutting Patterns → Capability ledger'
    change: 'The statusline rows owed: `rate_limits` with its `viola verify` probe, the settings override and the Unix shell on the working-route entry "Owed ledger rows" (not "Paste newline ledger row"), the Windows legs on "Windows-only live measurements".'
    sidecar: 'Capability ledger: the statusline settings-override, Unix-shell and `rate_limits` rows owed on "Owed ledger rows".'
    rationale: 'Report Changes → Inputs I3 row 1 (CARRY 6 the statusline override and `/bin/sh`, CARRY 7 `rate_limits`, moved verbatim to "Owed ledger rows") and I4.'
    basis: '.andromeda/architecture.md:472'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Cross-cutting Patterns → Capability ledger'
    change: 'The founder''s own answer of 2026-10-13 keeps the build as it is and joins the `rate_limits` row and its probe to the owed rows, which now sit on "Owed ledger rows" (carried there from "Paste newline ledger row" by the founder''s fork-3 answer, `inputs#I2`).'
    sidecar: 'Capability ledger: the budget-gate `rate_limits` row''s owning entry is "Owed ledger rows".'
    rationale: 'Report Changes → Inputs I3 row 1 and I4: "Paste newline ledger row" closed with this chunk and its CARRY 7 (`rate_limits`) moved verbatim to "Owed ledger rows".'
    basis: '.andromeda/architecture.md:472'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Delivery Confirmation]'
    change: 'Every multi-line `send` relies on the LF-inside-a-paste shape, which is gated by the ledger row `paste-newline` and its `viola verify` probe (Run B''s `PROBE_NEWLINE_PASTE`), landed at chunk 2026-10-13-paste-newline-ledger-row and stamped on live `claude` 2.1.287 on the Linux dev host (`evidence/live-round.md`: one prompt equal to the 113-byte text byte for byte); its Windows reading is owed on "Windows-only live measurements" — replacing "it has no ledger row and no `viola verify` probe: its row is owed … Paste newline ledger row".'
    sidecar: 'Delivery Confirmation: the inner-LF shape every multi-line send relies on is rowed (`paste-newline`).'
    rationale: 'Report Counts / qualifiers moved (the LF typed inside a paste now has its row), Cross-cutting claims (live 2.1.287, Linux only), inputs#I3 row 5; sweep `inner-lf-owed` architecture.md:49 ×3.'
    basis: '.andromeda/architecture.md:49'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Standard Contracts → Channel methods (`list`)'
    change: 'The `claude agents --json` join-field ledger row and probe are owed to the working-route entry "Owed ledger rows" (not "Paste newline ledger row"); the read itself stays owed to "Unwrapped sessions on the board".'
    sidecar: 'Channel methods `list`: the join-field row owed on "Owed ledger rows".'
    rationale: 'Report Changes → Inputs I3 row 1: CARRY 8 (`claude agents --json`) moved verbatim to "Owed ledger rows", minted at this wrap (I4); sweep `inner-lf-owed` lists architecture.md:291.'
    basis: '.andromeda/architecture.md:291'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Repository'
    change: 'The committed set `2.1.287` holds spine + 3 screens + 12 dialog variants + 5 framing variants (`paste-1`, `paste-2`, `paste-3`, `clear-1` ×2; 29 files), the new `UserPromptSubmit.paste-3.json` recorded live by `viola verify --record` and scrubbed.'
    sidecar: 'Repository: 2.1.287 fixture set gains `UserPromptSubmit.paste-3.json` (four → five framing variants, 29 files).'
    rationale: 'Report Changes → Files (new fixture `fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json`), Schema / config (one framing variant added, 621 bytes, scrubbed) and Counts / qualifiers moved (framing variants four → five, 29 files); sweep `framing-variants` architecture.md:422.'
    basis: '.andromeda/architecture.md:422'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Repository'
    change: '`2.1.287` is stamped at eighteen rows by `contract_ledger_probes` (`18 pass  0 fail`), not seventeen.'
    sidecar: 'Repository: 2.1.287 stamped at eighteen rows by `contract_ledger_probes`.'
    rationale: 'Report Outcome: `contract_ledger_probes` stamps 2.1.287 at `18 pass  0 fail`; sweep `rows17` architecture.md:422.'
    basis: '.andromeda/architecture.md:422'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Binary, subcommands and exit codes'
    change: '`viola-fake-agent`''s `--framing`: a pasted compiled paste text fires its recorded `paste-1` / `paste-2` / `paste-3` UserPromptSubmit variant with its bytes unchanged (the new stem reached through `framing_stem`, no option added; the twelve argv options stand).'
    sidecar: 'Test-only binaries: fake agent `--framing` replays `paste-3` too.'
    rationale: 'Report Changes → Symbols / APIs: `framing_stem` gives the newline text the stem `paste-3` and the fake agent reaches it with no change of its own; no new fake-agent option. Sweep `framing-variants` architecture.md:362.'
    basis: '.andromeda/architecture.md:362'
    dependent-of: D-arch-resources
note: 'No new IPC method, endpoint, port, env var, crate or dependency (report Symbols / Crates / Dependencies), so neither detector found stack drift. Not proposed, for lack of a report statement: whether Run B''s `box_wait` (stated at architecture.md:91 for the rows after the long and tag-like turns) also covers the rows after the new newline turn; `contracts/architecture/project-directory-structure.md:108` names the variants by pattern with no count and needs no change; `stamped 2.1.287  17 pass  0 fail` at architecture.md:91 is a dated record of an earlier chunk and stands.'
