# security extract

## Relevance
relevant — the chunk adds a new child-spawn path (verify's PTY child of the live `claude`), a new fixture class (recorded screen text), new stamped ledger rows that gate non-`null` dialog decisions, and closes a dated security gap (the six-row stamp).

## Constraints
- **The live leg stays local, credential-free CI.** Per security-plan §Threat Model Summary (CI pipeline bullet), the real `claude` CLI runs only locally. In CI, `viola verify` runs only against the fake agent, and its real-CLI probe and `--record` run only locally. The new PTY probe must keep that split: CI stamps the new rows from replayed fixtures and never holds a Claude credential (R-S2 restates this). Whether the live leg is refused under `CI`, the way the harness's `run --local-live` is, is research's question.
- **Recorded screen text gets the full fixture scrub.**
  - Per security-plan §Data Protection (Repository fixtures), every string verify records is scrubbed before any write: the home prefix becomes `~` and the user word becomes `<user>`.
  - The whole recording is refused when a path or username survives (`unable: a recorded payload still holds a path or a username`, exit 1, nothing written).
  - Probe prompts must be synthetic: `PROBE_PROMPT` and any typed probe input are compiled constants.
  - Screen rows are a new fixture class. Whether the existing scrub reaches them, and whether the startup screen also shows content the path/user-word scrub does not cover, are research's questions. That content includes the probe-dir cwd under the home, an account email and an organisation name.
- **Probe dir hygiene.** Per security-plan §Data Protection (Probe captures):
  - The probe plugin and captures live under `ledger/probes/<pid>/` (0700), with captures 0600.
  - A drop guard removes the whole probe dir on every exit path of `verify`, now including PTY-child failure, deadline expiry and kill.
  - Only a scrubbed `--record` copy leaves the dir.
  - Disk modes are set explicitly through the shared helpers, never left to the umask (§Authentication & Authorization, home/file modes).
- **Bound every read of the PTY child.**
  - Per security-plan §Input Validation (Constants: `MAX_FRAME`, child output row), every child output reader goes through `take(MAX_FRAME)`, parsing is tolerant and failure reads as `unknown`. This covers the PTY output stream feeding the screen model and each capture read.
  - The capture arm stays as ratified (§Input Validation, Hook stdin row, capture arm): absolute existing `<DIR>`, `take(MAX_FRAME + 1)`, raw 0600 write, no `VIOLA_*`, no channel, exit 0.
  - The vt100 model is fed only under `catch_unwind` (§Security Anti-Patterns §Code Patterns).
- **Stamp authority.**
  - Per security-plan §Security Anti-Patterns §Universal, only `viola verify` writes `ledger/stamps.json`, under its `.lock`, with 0600 (§Data Protection).
  - A non-`null` dialog decision requires a stamp for the child's CLI version, read through `read_stamps_strict`.
  - The dated gap is that a decision flows on the six-row spine stamp until `working-route.md:84`. It closes only when the S3 / S7 / S8 / dialog-concurrency rows join `LedgerRow::ALL`, and a stamp lacking them stops enabling those decisions.
  - `update_stamps`' strict-modes exception (Decisions Log `2026-09-28`, stamps read) stays as is. The chunk widens none of the founder's dated exceptions.
- **Compiled-only classifier inputs.**
  - Per security-plan §Security Anti-Patterns §Code Patterns, screen signatures, harness prefixes and local-command lists come only from compiled `viola-agent-claude` ledger rows, never from runtime or upstream text. W1's measured signature and W2's local-command map are compiled literals, never read from a recording at runtime.
  - The harness-prefix set is exactly four literals in that ban. A fifth prefix (for example the CARRY 3 hypothesis preface) is a plan amendment, not a code-only change.
- **Child environment and spawn.**
  - Per security-plan §Secret Management, R8 strips inherited `CLAUDE*` names from the child, with `IDENTITY_FLOOR` stripped regardless, and handles names only.
  - The plan states R8 for `viola run`'s child. Whether it also applies to verify's PTY child (the scope's open sub-question, and whether today's print-mode probe strips at all) is research's question and an arch/security ruling, not an assumption.
  - Either way, no R8-stripped value may reach the ledger, a fixture or a log (§Bootstrap phases, `logging-redaction-wire`).
  - The child is spawned directly, never through sh/bash/cmd, and is never a `.cmd`/`.bat` PTY child (§Security Anti-Patterns §Data Protection).

## Patterns to follow
- Reuse verify's existing `--record` scrub-and-refuse path (security-plan §Data Protection, Repository fixtures) for the screen fixtures, rather than adding a second scrubber.
- Reuse the probe-dir drop guard and the `hook --capture` arm unchanged as the PTY probe's capture sink (security-plan §Data Protection, Probe captures; §Input Validation, Hook stdin row).
- Reuse `viola-pty`'s `PasteHandle` one-write bracketed paste for the typed probe input, so the paste text still passes `validate_paste_text` (security-plan §Input Validation; CLAUDE.md input-bound invariant).
- `send`'s new `unconfirmable` outcome carries a fixed code and `detail` only: no upstream text, no path (security-plan §Error Handling, External responses).
- Recorded screen text and user content go only to `instances/<name>/diagnostics/` detail files or the scrubbed fixture, never to a process-log line or stderr (security-plan §Bootstrap phases, `logging-redaction-wire`).

## Anti-patterns to avoid
- NEVER commit `fixtures/claude/*` recorded with non-synthetic prompts, or before checking them for home paths and usernames (security-plan §Security Anti-Patterns §Data Protection). This applies with extra force to screen captures, which render the cwd and banner.
- NEVER build screen signatures, harness prefixes or local-command lists from runtime or upstream text (security-plan §Security Anti-Patterns §Code Patterns).
- NEVER run the real-CLI probe or `--record` in CI, and never put a Claude credential on a runner (security-plan §Threat Model Summary, CI pipeline bullet).

## Contract bindings
- **obs:**
  - A new `process-start` / verify `subject`, or a redefined `verify-probe`, is a `diag-line.v1.json` change. It needs an obs Decisions Log entry, and the line must name env names only, never values (security-plan §Secret Management R8 ↔ obs-plan §6).
  - Screen text stays out of process-log lines (`logging-redaction-wire` ↔ obs detail catalog).
- **tests:**
  - The fake agent replays the scrubbed screen fixtures, so CI stamps the new rows credential-free.
  - CI's `viola-harness secret-scan` canary runs over the new fixtures and diagnostics (security-plan §Bootstrap phases, secret-scan line ↔ test-plan §10 / obs-plan §9).
- **architecture:**
  - The new ledger rows and the S3/S7/S8 re-probe close the dated gap in security-plan §Security Anti-Patterns §Universal and CLAUDE.md's "one dated exception". Both texts, and the `security-summary.md` leaf, need the matching amendment at wrap.
  - security-plan §Threat Model Summary's "Child process spawning" vector lists no verify PTY spawn, and R-S2 is the authority for adding it. Whether that list is amended is a drift/wrap question.

## Acceptance criteria contributions
- `viola verify --record` on a screen capture whose text holds the home path or the user word writes the scrubbed form, or refuses whole with exit 1 and nothing written. A test pins both arms over screen-row input, and no committed `fixtures/claude/2.1.288/` file holds `/home/`, `/Users/`, `\Users\`, a drive path or the user word (per security-plan §Data Protection, Repository fixtures).
- The live PTY probe is not reachable in CI: the CI legs stamp the new rows from fake-agent replay alone, and the live leg is refused when it detects CI (per security-plan §Threat Model Summary, CI pipeline bullet).
- After the chunk, a stamp missing the S3 / S7 / S8 / dialog-concurrency rows does not enable a non-`null` `hook.dialog` decision. `LedgerRow::ALL` includes those rows, and a test refuses `answer` non-`null` on a pre-chunk six-row stamp (per security-plan §Security Anti-Patterns §Universal).
- `ledger/probes/<pid>/` is gone after every `verify` exit path, including PTY deadline and kill, and no R8-stripped `CLAUDE*` value appears in the ledger, fixtures or diagnostics. The secret-scan canary passes (per security-plan §Data Protection, Probe captures; §Bootstrap phases, `logging-redaction-wire`).
