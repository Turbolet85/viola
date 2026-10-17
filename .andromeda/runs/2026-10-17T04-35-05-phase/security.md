# security extract

## Relevance
partial — the chunk is mostly terminal presentation. Security reaches it at four points: control-character escaping of human-mode `wait` / `last` / `list` text, the `NO_COLOR` / `TERM` / `COLORTERM` env reads (a boundary widening), the human-always-wins rule on `viola run`'s stdin/stdout path, and the stderr writers' disclosure floor.

## Constraints
- security-plan §Bootstrap phases (`logging-redaction-wire`) requires human-mode `wait` / `last` output to escape C0/C1 controls while keeping `\n` and `\t`. It also requires each human-mode `list` row to escape every control character (`\n`, `\t`, DEL and C1 included) as `\xHH` through `escape_row`, so the row stays on one line. Whether `wait` / `last` already escape is research's question. The colour layer, and any width-known NAME truncation on the styled TTY board, must sit outside the escaping and must not bypass it. An SGR span may wrap only an already-escaped word.
- security-plan §Secret Management (Storage) requires that environment variables are not a configuration channel. Reading `NO_COLOR` / `TERM` / `COLORTERM` is therefore a new read outside the plan's admitted env set. It needs its own entry in the Security Decisions Log (`security-plan-amendments.md`) and the founder's own ratification before it lands. A delegate's answer or a decisive lean does not count.
- security-plan §Security Anti-Patterns (Universal) requires that no env var, `config.json` key or CLI flag switches off any control. Whatever the colour facts read, they may decide only whether SGR bytes are emitted. They must never disable escaping, redaction, `validate_paste_text` or any check.
- security-plan §Security Anti-Patterns (Universal) requires that a security refusal never blocks the human, and that refusals go only to automation. The unblocked-keys case requires this:
  - a key written while a verified `send` waits in the readiness gate reaches the child;
  - the refusal (`human-typing`, exit 10) goes only to the sending client;
  - no viola-originated refusal byte reaches the human's terminal.
- security-plan §Threat Model Summary (Attack surface, "The human's terminal, `viola run`'s own stdin") requires that only a sequence on the stdin classifier's closed non-editing list never counts as typing. A reply that no terminal has been measured to write is never added on a guess. The keystroke cases in this chunk must not widen that list.
- security-plan §Input Validation (PTY output bytes row) requires the `run` tee to write the human's bytes first. A full feed queue must drop the copy and never block or alter the passthrough. The zero-wrapper-bytes case is measured on that path, and Windows companion-sideload failures also must add no byte to it (per security-plan §Security Anti-Patterns → Universal).
- security-plan §Error Handling (Internal logging) requires that human-mode stderr never carries:
  - upstream text, tool `input` or drift-report values;
  - an anyhow chain holding a serde source (per security-plan §Bootstrap phases, `error-sanitization-wire`).

  This applies to any stderr writer that the chunk adds or regroups, such as the `waiting: <name>` line, verify's `[NN/NN] … pass|fail` step lines and the `human::refuse` sites.

## Patterns to follow
- `escape_row` in `src/human/board.rs` is the established row escaper (per security-plan §Bootstrap phases, `logging-redaction-wire`). The `wait` / `last` escaper is the same class, minus the `\n` / `\t` exemption.
- Refusals print the two fixed `unable:` / `hint:` stderr lines with no path, pid or logged id (per security-plan §Input Validation, Child executable resolution row and Session id row).
- Closed words only on the board: STATUS is `idle` · `busy` · `unknown`, and an absent value is `n/a` (per security-plan §Bootstrap phases, `logging-redaction-wire`). Group labels in `--help` are compiled literals, never runtime text.
- Under `run`, the human's terminal path is fail-open: degradations are recorded as codes in logs, never as terminal bytes (per security-plan §Security Anti-Patterns → Universal, sideload clause).

## Anti-patterns to avoid
- NEVER print `wait` / `last` human-mode output, or `list` rows taken from `claude agents --json`, to a terminal without escaping C0/C1 controls (per security-plan §Security Anti-Patterns → Logging).
- NEVER let an env var or flag (`NO_COLOR` / `TERM` / `COLORTERM` included) act as configuration that switches off a control (per security-plan §Security Anti-Patterns → Universal).
- NEVER expose absolute paths in any external error, and never print an anyhow chain carrying a serde source to stderr (per security-plan §Security Anti-Patterns → Logging).

## Contract bindings
- The env-read widening binds security-plan §Secret Management to architecture's Config management env sentence. One Decisions Log entry each, both behind the same founder ratification, raised as a P4 fork.
- obs-plan §3 / §11's stderr-writer sentences (CARRY 1) bind to security-plan §Error Handling (Internal logging). Whatever writers the wrap names must each meet the stderr floor.
- The escaping rule binds to a11y-plan P6 (injected controls shown escaped) and to design-system cli (SGR spans beside their word). Security owns the escaping. A11y and design own the presentation.
- CI's artifact canary scan (`viola-harness secret-scan`, obs-plan §9) binds to security-plan §Secret Management (Secret scanning in CI). New outer-PTY or stderr captures land under `target/agent-run/` and are scanned.

## Acceptance criteria contributions
- A test types a reply holding ESC, a C1 control (U+009B) and DEL into human-mode `viola wait` / `viola last`. The output shows each one escaped, with `\n` / `\t` kept. A `list` row with injected controls still renders on one line through `escape_row`. The check passes under a colour-capable `TERM` too (per security-plan §Bootstrap phases, `logging-redaction-wire`).
- No `NO_COLOR` / `TERM` / `COLORTERM` read lands without a Security Decisions Log entry and the founder's own ratification. A test shows that no value of those variables changes escaping or any refusal (per security-plan §Secret Management; §Security Anti-Patterns → Universal).
- In the outer-PTY case, a key typed during a verified `send`'s readiness wait reaches the child. The send exits 10 `human-typing` to the client only, and the human's terminal stream carries zero viola-originated bytes (per security-plan §Security Anti-Patterns → Universal).
- If the chunk adds any terminal or colour crate, `cargo deny check` passes. The crate is pinned in `[workspace.dependencies]`, builds no C, and is within the licence `allow` list (per security-plan §Dependency Security). Whether std `IsTerminal` plus the existing windows-sys covers the facts without a new crate is research's question.
