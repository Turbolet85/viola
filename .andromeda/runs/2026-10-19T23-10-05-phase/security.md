# security extract

## Relevance
partial: the chunk changes no IPC, home or listener surface. It does touch the stamp that switches on dialog answering, `viola verify`'s live probe children, the `send` confirm path and the recorded fixtures. All four are security-planned surfaces.

## Constraints
- A non-`null` dialog decision flows only when every row of the relied set (`LedgerRow::relied()`, counter `/24` on Unix) passes. Only `viola verify` writes `ledger/stamps.json`. A `20 pass  4 fail` stamp must keep reading `cli_verified:false`, and `answer` must keep exiting 12 `unverified-cli` (per security-plan §Security Anti-Patterns → Universal). A row that resolves as (c) is never stamped through: the gate never treats a failing relied row as a pass. The plan fixes the twenty-four rows by name, so adding, removing or un-relying a row is a plan amendment, not a probe fix (per security-plan §Security Anti-Patterns → Universal).
- `viola verify`'s interactive children are bounded as follows (per security-plan §Threat Model Summary, Attack surface → Child process spawning and PATH resolution):
  - Run B gets exactly six compiled-literal pastes (`PROBE_PROMPT`, `PROBE_LONG_PASTE`, `PROBE_TAG_PASTE`, `PROBE_NEWLINE_PASTE`, `PROBE_COMPACT_COMMAND`, `PROBE_LOCAL_COMMAND`).
  - Each added paste goes only into a settled input box with no modal. The input-box wait is bounded by `PROBE_DEADLINE`.
  - No byte is ever typed into any dialog, and a modal start is killed with no key.
  - An uninterrupted verify makes six starts and eight spawn pairs.
  - A corrected probe or oracle that adds a paste, start, run, spawn pair or key, or changes a paste's literal, crosses a founder-ratified boundary and is a P4 fork. It is not a lean.
- Screen signatures, harness prefixes and the local-command list (`/clear`, `/compact`) come only from compiled `viola-agent-claude` ledger rows. They are never built from runtime or upstream text, so a re-measured 2.1.302 shape enters as a new compiled literal (per security-plan §Security Anti-Patterns → Code Patterns).
- If a (b) resolution changes the `send` reliance, the paste-text rule stays fixed (per security-plan §Input Validation, Paste text row):
  - `validate_paste_text` runs first on the received text, and a refused character is rejected, never stripped.
  - `typed_text` rewrites or removes only CR and LF.
  - The refusal order stays `control-character` then `empty-text`.
  - The delivery-confirmation match stays an exact equality with no tolerance on the submitted side.
  - Loosening that match to follow a changed CLI echo is a widening, not a fix.
- verify's PTY output is capped at `MAX_FRAME` into a `Screen` fed under `catch_unwind`, and is read only against the compiled signature literals. Run B's one `claude agents --json` read uses `take(MAX_FRAME + 1)`, the R8 strip and `VERSION_DEADLINE`, and its capture is held in memory only (per security-plan §Input Validation, Child process output row).
- Any `fixtures/claude/2.1.302/` recording goes through `viola verify --record`'s scrub, built from synthetic prompts only (per security-plan §Data Protection, Repository fixtures):
  - The home prefix becomes `~` and the user word becomes `<user>`.
  - The whole recording is refused on a surviving `home-path`, `absolute-path`, `username` or `email`.
  - Screen fixtures keep only the rows that hold a compiled signature literal.
  - Refusals name the file and the code, never the content.
- The live round's probe dirs (`<cwd>/.viola-verify-<pid>*/` and the OS-temp Run A dir) are 0700 and guard-removed. viola writes nothing under `~/.claude`. The CLI's transcripts are an accepted residual only at the measured class: Run B three files, Runs C and D one each, Run E none. A changed probe that adds a transcript class falls outside the accepted risk (per security-plan §Data Protection, accepted risk for verify's trusted runs B, C, D and E).

## Patterns to follow
- Probe pastes and oracles as compiled constants in `src/cmd/verify.rs` and `crates/viola-agent-claude/src/ledger.rs`. Answers to Run C/D dialogs go only through the hidden `hook --capture --answers` arm, `ledger::probe_body` → `dialog::decision_body` (per security-plan §Input Validation, Hook stdin row).
- Capture files are claimed exclusively through `viola_state::fs::create_private_new`, then written at 0600 through `replace_private` (per security-plan §Input Validation, Hook stdin row).
- `ledger/stamps.json` is written only by `viola verify`, under its `.lock`, at 0600 (per security-plan §Data Protection, Code-bearing artefacts). Its `update_stamps` read is the dated `2026-09-28` stamps-read gap. Whether the live round's home choice (`~/.viola` vs a `--home`) changes anything there is a P4 question, not a security change.
- Run E's id goes only through `ledger::resume_target` → `is_session_id` → `resume_args(id, false)`, as one argv element of a direct spawn (per security-plan §Input Validation, Session id row).

## Anti-patterns to avoid
- Building a screen signature, prefix or local-command match from a captured 2.1.302 screen or payload at runtime, instead of from a compiled literal (per security-plan §Security Anti-Patterns → Code Patterns).
- Making a row pass by stripping characters from the submitted or typed text, or by adding tolerance to the exact match (per security-plan §Security Anti-Patterns → Input).
- Committing a 2.1.302 fixture recorded from a non-synthetic prompt or one not checked for home paths and usernames (per security-plan §Security Anti-Patterns → Data Protection).

## Contract bindings
- security ↔ tests: recorded `fixtures/claude/2.1.302/` files are replayed by the fake agent in CI. The scrub and signature-only rule (§Data Protection, Repository fixtures) is the floor the test fixtures inherit.
- security ↔ obs: verify's captures and refusals stay on the NEVER-log floor (§Bootstrap phases, `logging-redaction-wire`). Captured screen text, prompts and R8-stripped `CLAUDE*` values reach no log line or fixture beyond the scrubbed signature rows.
- security ↔ architecture: the capability ledger. A per-row (b) change moves the product reliance and the probe together inside `viola-agent-claude` (§Security Anti-Patterns → Code Patterns; §Security Anti-Patterns → Universal, the relied set).

## Acceptance criteria contributions
- A home stamped with exactly the 2.1.302 `20 pass  4 fail` reading still yields `cli_verified:false` and `answer` exit 12 `unverified-cli`. Only a stamp with every relied row `pass` yields `cli_verified:true` (per security-plan §Security Anti-Patterns → Universal).
- An uninterrupted `viola verify` after the change still writes six starts and eight spawn pairs, and Run B's paste set is still the six compiled literals. Any added start, paste or key carries a recorded founder answer (per security-plan §Threat Model Summary, Attack surface → Child process spawning and PATH resolution).
- Any `send` change keeps the paste-text rule (per security-plan §Input Validation, Paste text row):
  - the refusal order `control-character` then `empty-text`;
  - reject-not-strip;
  - the exact-equality delivery confirmation with no submitted-side tolerance.
  - Whether the existing unit/property cases already pin these is research's question.
- Every committed `fixtures/claude/2.1.302/` file passes `viola verify --record`'s scrub with no `home-path` · `absolute-path` · `username` · `email` refusal, and every screen fixture holds only compiled-signature rows (per security-plan §Data Protection, Repository fixtures).
