# security extract

## Relevance
relevant: the chunk adds the IPC surface the plan rates highest-impact (a `send` channel method that types into the PTY), the bracketed-paste breakout control, a new refusal path and a bounded input queue.

## Constraints
1. **New channel method needs a Decisions Log entry.** security-plan §Security Anti-Patterns → Universal requires a Security Decisions Log entry before any new channel method is added. The entry maps the method to the IPC controls in §Authentication & Authorization (protected DACL / 0700 socket directory, `accept_remote(false)`, `MAX_FRAME` framing, the `v` check).
   - The `send` method is new. The only method on the record is `hook.event` (security-plan-amendments-archive `2026-09-28` "hook.event served"), and every other method answers `-32601`.
   - Writing that entry for `send` is P4's job. It is shown, and it is held if it widens anything.
2. **Server verification and strict-modes before the first `send` frame.** security-plan §Authentication & Authorization (row "IPC client-side server verification" and row "`~/.viola/` access control") and §Security Anti-Patterns → Authentication require both checks before the first frame of any method, `send` named explicitly:
   - pid (or euid + pid) + sysinfo start time, checked against a `snapshot.json` that passed strict-modes in the same process;
   - Windows SQOS open;
   - on mismatch, `instance-unreachable`, exit 21.

   The `hook.event` interim gap is ratified for that frame only. The archive `2026-09-28` entry says "No other frame, verb or process may take this exception". So `viola send` cannot borrow it.
   - Open for research: whether `viola-channel` already has a client verifier and a strict-modes check, or whether both still belong to Epoch 6 ("Server verification before any frame", "Home and code-bearing file integrity").
   - If both are absent, sending without them is a boundary widening, and under the autonomous directive it is shown at P4 and held.
3. **Paste-text validation, exact set.** security-plan §Input Validation (row "Paste text") requires `viola_core::validate_paste_text(&str) -> Result<(), CoreError>` to run over decoded `char`s.
   - Allowed: LF, CR and TAB.
   - Refused: every other C0 (ESC above all), DEL, and C1 (U+0080–U+009F), with `not-delivered` / `control-character`.
   - Reject, never strip.
   - It runs client-side, and again in the wrapper before the PTY write. The wrapper's check is authoritative.
   - It is checked before the Conventions refusal order.
   - §Bootstrap phases (input-validation-library-install) assigns the validator and the `control-character` detail to this route entry.
4. **Bound every input.** security-plan §Input Validation (row "PTY output bytes (vt100)") records the tee → feed `std::sync::mpsc` as unbounded and "owed a bound … by confirmed `send`".
   - The vt100 feed and resize stay under `catch_unwind`.
   - A poisoned model reads `input-not-ready`, which `send` reports as `not-delivered` / `input-not-ready`.
   - Byte passthrough to the human terminal continues.
   - Per §Security Anti-Patterns → Universal, the overflow policy may degrade the verdict but never block or delay the human.
   - The CLI `send` stdin / `--file` read is capped under §Input Validation §Constants (`MAX_FRAME` on every external reader).
5. **The human always wins, and refusals stay sanitised.** security-plan §Security Anti-Patterns → Universal requires every refusal (control-character, one in flight, input-not-ready, no-prompt-submitted) to go to automation only. No refusal may block, refuse or delay a human keystroke.
   - §Error Handling requires `result.refusal` to carry the `not-delivered` / `<detail>` codes only: no upstream text (the prompt), no absolute path, no anyhow chain holding a serde source.
   - The same applies to the CLI's `[/ ] unable … hint:` stderr lines.
6. **No switch or seam can disable a control.** security-plan §Security Anti-Patterns → Universal: no `config.json` key, `VIOLA_*` env var or CLI flag may switch off `validate_paste_text`.
   - No new env-var test seam may be added without its own Decisions Log entry.
   - §Dependency Security (the G2 bullet): G2 has exactly one exemption, `src/cmd/hook/seam.rs:<digits>`, compared as a whole string. A second exemption for a contained feed panic needs its own Decisions Log entry.
   - The CARRY 6 G2 question is therefore a P4 widening fork unless the test design avoids a G2-gated home.
7. **Local-command list and `from`.** security-plan §Security Anti-Patterns → Code Patterns requires local-command lists and screen signatures to come only from compiled `viola-agent-claude` ledger rows, never from runtime or upstream text. That covers the unconfirmable / `/clear` classification.
   - §Security Anti-Patterns → Authentication: `from` in `send-issued` is self-reported, never an identity.

## Patterns to follow
- **Wrapper re-validation (`hook.event` precedent).** Follow the pattern of security-plan-amendments-archive `2026-09-28`:
  - re-validate the params in the wrapper (closed types, `v` check, `MAX_FRAME` framing);
  - answer `-32603` with exactly `"internal error"` / `data: null` on an internal fault (§Error Handling).
- **Shared validator in `viola-core`.** Call it from both the CLI client and the wrapper (§Input Validation §Boundary: "the CLI, MCP and the wrapper all call the same function"). The wrapper's call is the enforcing one (§Security Anti-Patterns → Input: never trust client-side checks alone).
- **Fixed-message errors.** `CoreError` / `Refusal` `Display` impls use fixed messages, and fields holding payloads are excluded from `Display` (§Error Handling, External responses).
  - Full detail, the prompt text included, goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) (§Data Protection → Logs; §Bootstrap phases, error-sanitization-wire).
- **Gate verdict only.** The screen model stays the existing `catch_unwind` feed thread, and only the verdict leaves it (§Input Validation, row "PTY output bytes (vt100)"; §Security Anti-Patterns → Code Patterns "NEVER feed vt100 outside `catch_unwind`").
- **New fuzz target.** The `validate_paste_text` fuzz target goes in the separate `fuzz/` workspace with a synthetic seeded corpus. §Bootstrap phases (secret-scanning-ci-gate) admits the nightly `fuzz/artifacts/` upload only while the corpus is synthetic. §Input Validation (Parser surfaces) names `validate_paste_text` as a fuzz / property surface.

## Anti-patterns to avoid
- NEVER write `send.text` into the PTY when it holds a refused control, NEVER strip one silently, NEVER refuse LF/CR/TAB, and NEVER run the check over raw bytes (security-plan §Security Anti-Patterns → Input).
- NEVER write the `send` frame before server verification against a strict-modes-checked snapshot, and NEVER extend the `hook.event` exception to `send` (§Security Anti-Patterns → Authentication; archive `2026-09-28`).
- NEVER add the `send` channel method, a G2 exemption or an env-var seam without its own Security Decisions Log entry (§Security Anti-Patterns → Universal; §Dependency Security, the G2 bullet).

## Contract bindings
- **security ↔ obs (NEVER-log floor).**
  - `send-issued (cursor, from)` and `send-refused` carry codes only.
  - The prompt reaches only `diagnostics/detail-*.ndjson`.
  - The artifact canary scan (`viola-harness secret-scan`, §Bootstrap phases, secret-scanning-ci-gate) must stay green over the new events and the CLI's stderr.
  - The event shape is obs's.
- **security ↔ architecture (origin).** Wrapper re-validation of `hook.event` admits `origin` ∈ `harness` · `human` only (archive `2026-09-28`; §Input Validation, row "Channel frames").
  - `prompt-submitted{origin:"driver"}` therefore cannot arrive self-asserted over `hook.event`. It must be assigned by the wrapper when it matches its own in-flight send.
  - How the matching assigns `driver` is a question for architecture [Delivery Confirmation] and research.
- **security ↔ tests (two CARRYs).**
  - CARRY 3's ESC-bearing `send` → exit 13 row in `tests/cli_controls_not_disableable.rs` is §Security Anti-Patterns → Universal's "no switch disables `validate_paste_text`" witness.
  - CARRY 2's property + fuzz target is the §Input Validation Parser-surfaces coverage.
- **security ↔ CI (G2).** The CARRY 6 forced feed panic interacts with G2's single exemption (§Dependency Security, the G2 bullet). `scripts/g2-zero-panics.sh --probe` must still read red for every non-exempt location.

## Acceptance criteria contributions
- `validate_paste_text` must hold the exact set over decoded `char`s (per security-plan §Input Validation, row "Paste text"):
  - it accepts LF, CR, TAB and valid multi-byte UTF-8 whose continuation bytes fall in 0x80–0xBF;
  - it refuses ESC (`ESC[201~`), every other C0, DEL and U+0080–U+009F with `not-delivered` / `control-character`;
  - the wrapper re-runs it before any PTY write, so a frame that bypasses the client is refused and nothing is typed.
- `viola send` writes no channel frame before pid / euid + start-time verification against a strict-modes-checked `snapshot.json` (per security-plan §Authentication & Authorization, row "IPC client-side server verification").
  - A mismatch or squatter yields `instance-unreachable`, exit 21.
  - If the verifier is not yet built, this criterion is replaced by a P4-shown, founder-held widening, never silently skipped.
- Under a full tee → feed queue or a poisoned model, the human's terminal bytes pass through undelayed and `send` reports `not-delivered` / `input-not-ready` (per security-plan §Input Validation, row "PTY output bytes (vt100)"; §Security Anti-Patterns → Universal).
- Refusal outputs carry codes only (per security-plan §Error Handling; §Security Anti-Patterns → Logging). This covers:
  - channel `result.refusal`;
  - the CLI `[/ ] unable` / `hint:` stderr;
  - `send-refused`.

  None of them may contain the prompt text, an absolute path or a serde source, and the CI secret-scan reads clean.
