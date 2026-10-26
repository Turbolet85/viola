proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 58
    section: '## Surface: cli › ### Component Patterns'
    change: '- server verification fails: `the process on <name>''s endpoint is not its recorded wrapper; it was not contacted` (as built, chunk 2026-10-26-server-verification-before-any-frame: `send`, `wait`, `last`, `answer`, `pause` and `release` print it on Linux and Windows when the process serving the endpoint is not the snapshot''s recorded pid and start time, or the check cannot be made, and write no frame; on macOS no peer pid is read and the check passes uncompared, so it does not print there)'
    sidecar: 'surface.md cli-send: the server-verification hint marked as built for the six CLI frame verbs (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'Report Symbols / APIs: hints.rs send_hint''s new cause server-verify prints this string byte for byte, and the six verbs each request through client::connect, a mismatch or an unmakeable check exiting 21 with the hint last; server_verdict returns Matches uncompared when no peer pid is reported (macOS). The line stated the cause before any verb could reach it; Expected amendments names an as-built qualifier as the candidate.'
    basis: src/human/hints.rs:50
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 110
    section: '## Surface: cli › ### Component Patterns'
    change: 'in the exit-21 row, inside «(not running / unwrapped / strict-modes / server verification)» add after «server verification»: «, as built for `send` · `wait` · `last` · `answer` · `pause` · `release` on Linux and Windows, chunk 2026-10-26-server-verification-before-any-frame»'
    sidecar: 'surface.md cli-exit-codes: the exit-21 row''s server-verification cause marked as built for the six CLI frame verbs'
    rationale: 'Report Symbols / APIs (the six verbs: exit 21, the unchanged instance-unreachable document under --json, the new hint last on stderr) and Expected amendments (gaps, surface.md:110, an as-built qualifier a candidate).'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:15
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 73
    section: '## Surface: cli › ### Component Patterns'
    change: 'after `server-verify-failed` in the obs-plan D-20 list add « (written since chunk 2026-10-26-server-verification-before-any-frame: `process-exit` at exit 21 with `during:"connect"` in the CLI log, and `hook-decision` in the hook log)»'
    sidecar: 'surface.md cli-send: the D-20 detail code server-verify-failed marked as written by this build'
    rationale: 'Report Schema / config: this build is the first that writes server-verify-failed (process-exit exit_code 21, during connect, into cli-<name>.ndjson; hook-decision into hook-<name>.ndjson); Spec claims disproved (pattern svf) names surface.md:73 still true with an as-landed qualifier a candidate.'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:21
note: 'D-design-tokens and D-design-cli-wording read clean: every new surface is tokens n/a, and the one new output literal (src/human/hints.rs 50-52) equals surface.md:58 byte for byte, no wording replaced; no Retired gap is stated in this file.'
