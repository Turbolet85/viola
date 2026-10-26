proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 26
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add to the cli verification item, after the `viola plugin install` clause: the exit-21 server-verification refusal of `send` / `wait` / `last` / `answer` / `pause` / `release` (`unable  <name>  instance-unreachable`, then the `server-verify` hint `the process on <name>''s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty) is pinned by `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (`tests/security_negatives_server_identity.rs`, six verbs x two modes), which runs on the Linux and Windows legs only (the binary compiles out on macOS, its recorded gap), chunk 2026-10-26-server-verification-before-any-frame.'
    sidecar: 'cognitive-accessibility Error recovery verification: the six frame verbs'' exit-21 server-verification refusal and its hint-last / hint-free --json layout named with its Linux-and-Windows test (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'The report adds a new human-mode refusal hint (Symbols bullet, src/human/hints.rs send_hint cause server-verify) on six CLI verbs, exit 21 with the unchanged --json document; Outcome asserts the document under --json with stderr empty and the hint last on stderr in human mode, by the new test binary gated off macOS (Harness bullet). The item currently says its cases run on all three OS legs, which this case does not.'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 35
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Add to the literal-pinned exceptions of the verification item: the `server-verify` hint `the process on <name>''s endpoint is not its recorded wrapper; it was not contacted` (`send_hint`, `src/human/hints.rs`), pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs` on the Linux and Windows legs (compiled out on macOS), chunk 2026-10-26-server-verification-before-any-frame.'
    sidecar: 'cognitive-accessibility Plain language verification: the server-verify hint named among the literal-pinned fixed strings (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'The report adds a fixed CLI string (Symbols bullet: hints.rs prints exactly that text, design-system surface.md cli-send byte for byte) and its pin is the HINT const of the new test file (New text, by line: tests/security_negatives_server_identity.rs 26-27), not trycmd; the item says trycmd pins every fixed CLI string except those it names, so this string is an unnamed exception.'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:14
note: 'D-a11y-surface and D-a11y-obs-schema do not fire: the chunk adds no interactive UI element (Coverage rows read a11y n/a, stderr text, no SGR) and changes no schema (Schema / config: none); no time bound falls on the human, so Timeout extensions is unchanged.'
