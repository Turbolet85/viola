proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'Append to the CLI verbs entity, after the `viola plugin` clause: `send`, `wait`, `last`, `answer`, `pause` and `release` (chunk 2026-10-26-server-verification-before-any-frame) each refuse an endpoint whose serving process is not the recorded wrapper before any frame, exit 21, with the `unable  <name>  instance-unreachable` line and the server-verify hint last on stderr (plain ASCII, no SGR), or under `--json` the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; on macOS the refusal cannot occur (no peer pid, the recorded gap); their `--json` code and exit code sets are unchanged'
    sidecar: 'a11y-scope-summary CLI verbs entity: the six frame verbs'' exit-21 server-verify refusal recorded as built (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'Report Changes, Symbols / APIs: `src/cmd/{send,wait,last,answer,pause,release}.rs` each go through `client::connect`; a mismatch or an unmakeable check is exit 21 with the unchanged document or the `unable  <name>  instance-unreachable` line with the new hint last on stderr; Retired: macOS stays on the recorded gap'
    basis: 'src/human/hints.rs:50'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add to the CLI item, after the `viola verify` interrupted-pair clause: CLI `send`, `wait`, `last`, `answer`, `pause` and `release` refuse an endpoint whose serving process is not the recorded wrapper with exit 21 (chunk 2026-10-26-server-verification-before-any-frame): `unable  <name>  instance-unreachable` on stderr, then the hint `the process on <name>''s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty and no hint'
    sidecar: 'cognitive-errors pattern: the server-verify refusal of the six CLI frame verbs added (F1)'
    rationale: 'Report Changes, Symbols / APIs: `send_hint`''s new cause `server-verify` prints exactly that string (design-system surface.md cli-send byte for byte); the six verbs exit 21 with the unchanged document or the `unable` line with the hint last on stderr'
    basis: 'src/human/hints.rs:50'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 26
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add to the cli verification item: the server-verify refusal is pinned not by trycmd but by the `HINT` literal in `tests/security_negatives_server_identity.rs` (`security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` over the six verbs in two modes, and its control `security_negative_cli_verb_reaches_the_recorded_wrapper`), which asserts the hint is the last stderr line, plain ASCII with no SGR, and free of any pid, start time, path or endpoint digits; the binary is gated `not(target_os = "macos")`, so it runs on windows-2025 and ubuntu-latest only (chunk 2026-10-26-server-verification-before-any-frame)'
    sidecar: 'cognitive-errors verification: the server-verify refusal''s literal pin and its two-leg reach added (F2)'
    rationale: 'Report Changes, Harness / gate surface: new test binary `security_negatives_server_identity` (root, `#![cfg(not(target_os = "macos"))]`); New text, by line: `const HINT` at 26-27, the mismatch function at 175-263 and its control at 265-289; Coverage: the hint plain ASCII, no SGR, no pid, path or endpoint asserted'
    basis: 'tests/security_negatives_server_identity.rs:26'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 26
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add after the cli verification item: on macOS the server-verify refusal cannot occur (no peer pid; the check reads as passed, the recorded gap), and its output-discipline case compiles out there; its macOS witness is owed to "Unix endpoint and home hardening"'
    sidecar: 'cognitive-errors verification: the server-verify refusal''s macOS gap and its owed witness recorded (F3)'
    rationale: 'Report Changes, Symbols / APIs: `server_verdict` with `peer_pid_reported == false` reads `Matches` with no comparison, the macOS recorded gap; Retired and P5 card row 3: the macOS server-verification cases join "Unix endpoint and home hardening", this chunk''s negatives compile out on macOS'
    basis: 'crates/viola-state/src/liveness.rs:123'
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 35
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Add to the literal-pinned exceptions after the `viola plugin install` clause: and the server-verify hint (`src/human/hints.rs`, cause `server-verify`), pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs` (chunk 2026-10-26-server-verification-before-any-frame)'
    sidecar: 'cognitive-plain-language verification: the server-verify hint named among the fixed strings pinned by literal rather than trycmd'
    rationale: 'Report Changes, Symbols / APIs: `src/human/hints.rs` gains the fixed hint string; New text, by line: `tests/security_negatives_server_identity.rs` 26-27 `const HINT`; no trycmd case was added (Harness / gate surface: none changed), so the claim that trycmd pins every fixed CLI string outside the named literal exceptions no longer holds'
    basis: 'tests/security_negatives_server_identity.rs:26'
note: F4 (hooks and MCP fail open with no human-facing byte) fits the not-assertable plugin entity, which is not picked, so the new-detail pass places it; the body's §8 Error recovery Verification («on all three OS legs») and Plain language Verification (the literal exceptions list) broad lines carry the same gaps as the detail edits above.
