# P5 review — 2026-10-13-paste-newline-ledger-row · Paste newline ledger row

## Goal
An LF typed inside a bracketed paste, which every multi-line `send` relies on, becomes ledger row 18,
`paste-newline`, probed by a fifth Run B paste and stamped on live `claude` 2.1.287 by one `--record` round of five
starts. The founder's own answers to the three P4 forks (`inputs#I2`, relayed verbatim by the operator): a fifth Run B
paste (R-L29), a cap of 5 live starts, and the six other owed shapes moving to a new entry directly after this one.

## Top acceptance criteria
- `LedgerRow::ALL` holds 18 rows, the last `paste-newline`; a stamp of the 17 older ids, all pass, reads unverified.
- Run B pastes exactly five compiled texts (probe prompt, long, tag-like, the LF text, `/clear`), each only into a
  settled input box, none after a stop.
- One live round on 2.1.287 (exactly 5 starts, fired once, never retried) prints `stamped 2.1.287  18 pass  0 fail`; its
  `paste-3` prompt equals the compiled LF text byte for byte; the census finds no probe-dir `claude` left.
- `fixtures/claude/2.1.287/` gains `UserPromptSubmit.paste-3.json` and nothing else changes there; the 2.1.287 set
  stamps 18/0 in CI; drift and hygiene contracts pass it.
- `viola verify` prints eighteen `[NN/18]` lines, plain ASCII, the interrupted pair unchanged; six spawn pairs.
- CARRY 2's four mutants each read red by an assertion when hand-applied, and green once restored
  (`evidence/carry2-kills.md`); no mutation run.

## Touchpoints
- New: `fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json` (recorded live, copied alone); `evidence/`.
- Modify: `crates/viola-agent-claude/src/ledger.rs`, `src/cmd/verify/typed.rs`, `src/cmd/verify.rs` (tests),
  `src/bin/viola-fake-agent.rs` (CARRY 2 cases only, if needed), `src/run/version_gate.rs`, `tests/cli_verify.rs`,
  `tests/contract_ledger_probes.rs`, `tests/support/verify.rs`, `crates/viola-e2e/src/harness/run.rs`.

## Test commands (24 entries)
fmt · clippy · G1 · lint probes · `run --unit` · the 3 `check_paste_newline_` cases by name · the round
(`test -x` probe → the record entry, 5 starts → the census) · the fixture guard · `run --integration` over
`cli_verify`, `contract_ledger_probes`, `contract_fake_agent_drift`, `contract_fixture_hygiene`, `cli_version_gate` ·
the 17-literal probe (75 → 0) · the preservation guard · `cargo deny` · harness smoke + G2 + G4 · `pre-push` ·
operator: hygiene, push, CI read. Ten new entries baselined at HEAD `19483111dbe9`; three guards controlled both ways
over scratch clones (`p5-controls.md`).

## Concretization preview
- No capability is claimed. `v1-34` stays pooled: its requirement also needs the harness-prefix and R8
  identity-floor rows, owed with no entry (the founder, 2026-10-07T09:43Z). After your word a dated not-claimed
  note is written on `v1-34`: the LF row lands at eighteen; the two rows stay owed; the six other shapes move to "Owed
  ledger rows".

## Beyond the plan
1. **Route insertion** — a new entry "Owed ledger rows", placed directly after "Paste newline ledger row" in
   `### Epoch 5 — Driver surface`, carrying this entry's CARRY 4, 5, 6, 7 and 8 verbatim (`--resume`, `/compact`,
   the statusline override and `/bin/sh`, `rate_limits`, `claude agents --json`). Each shape needs a child-set
   widening ruling and a live cap of its own at take-up. Authority: the founder's own answer to fork 3
   (`inputs#I2`).
2. **Deferral** — CARRY 2's cargo-mutants grading of `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs` moves
   to the Epoch 5 boundary audit (`/andromeda-code-audit`). This chunk shows the kills by hand-applied mutants
   instead, because `inputs#I1` excludes the witness run the CARRY named.
3. **Owner for a part left out** — CARRY 3 (the three `viola-state` retry-loop mutants graded by the 10 s kill on
   `windows-2025`) is not this entry's work. It is carried to the Epoch 5 boundary audit's `windows-mutants.yml`
   dispatch through the handoff's dispatcher note, with no route entry. Reason: its re-reading needs a dispatch
   (`inputs#I1`), and two Linux cases already kill its six mutants.
4. **Deferral** — the tmpfs `--local-live` home is re-stamped at eighteen rows by its next `run --local-live` firing
   (five starts, under a cap of its own), not in this chunk (the cap is five, `inputs#I2`). Until then, a live session
   from that home reads `cli_verified:false`.
5. **Owner for a part left out** — the LF row's Windows reading (stamped here on Linux 2.1.287 only) goes to the
   existing entry "Windows-only live measurements".
