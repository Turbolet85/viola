proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 58
    section: '## Surface: cli › ### Component Patterns'
    change: '    - server verification fails: `the process on <name>''s endpoint is not its recorded wrapper; it was not contacted` (as built, chunk 2026-10-26-server-verification-before-any-frame: the hint cause `server-verify` beside `not-running`, shared by `send`, `wait`, `last`, `answer`, `pause` and `release`; plain ASCII, no SGR, and it holds no pid, start time, path or endpoint)'
    sidecar: 'surface cli-send: the server-verification hint row marked as built, its cause `server-verify` shared by the six frame verbs, plain ASCII with no pid, start time, path or endpoint (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet: `src/human/hints.rs` `send_hint`''s new cause `server-verify` (50-52) prints exactly this string, design-system surface.md cli-send''s, byte for byte; the Coverage row asserts the hint plain ASCII with no SGR and no pid, path or endpoint in any output; the row''s wording stands, it now gains its as-built clause.'
    basis: .andromeda/design-system-detail/surface.md:58
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 58
    section: '## Surface: cli › ### Component Patterns'
    change: '      - On macOS no peer pid is available, so the server check passes with no comparison and this hint cannot print there (a recorded gap, owner: the route entry "Unix endpoint and home hardening").'
    sidecar: 'surface cli-send: the server-verification hint''s macOS gap added under its row (no peer pid, the check passes with no comparison; owner "Unix endpoint and home hardening")'
    rationale: 'The report''s Symbols bullet: `server_verdict` with `peer_pid_reported == false` returns `Matches` with no comparison, the macOS recorded gap, `SERVER_PID_REPORTED` false on macOS; its Retired bullet keeps macOS on the recorded gap, owned by "Unix endpoint and home hardening".'
    basis: .andromeda/design-system-detail/surface.md:58
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 110
    section: '## Surface: cli › ### Component Patterns'
    change: '| 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause''s hint (not running / unwrapped / strict-modes / server verification); as built for `send`, `wait`, `last`, `answer`, `pause` and `release` (chunk 2026-10-26-server-verification-before-any-frame), a server that is not the recorded wrapper (pid or start time differs, or the check cannot be made) gets no frame and the verb prints the server-verification hint last on stderr; `viola list`''s refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder''s own answer, 2026-10-13, relayed verbatim by the Viola overseer) | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), `list`''s refusal and the server-verification refusal included, with stderr empty |'
    sidecar: 'surface cli-exit-codes: exit 21''s server-verification cause marked as built for the six frame verbs, no frame sent, the hint last on stderr or the unchanged document with stderr empty (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet for `src/cmd/{send,wait,last,answer,pause,release}.rs`: a mismatch or an unmakeable check is exit 21 with the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` document, or the `unable  <name>  instance-unreachable` line with the new hint last on stderr; Outcome: `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (12 rows, six verbs x two modes); Schema / config: no exit code or `--json` code added.'
    basis: .andromeda/design-system-detail/surface.md:110
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 73
    section: '## Surface: cli › ### Component Patterns'
    change: 'After the D-20 code list, add: "(`server-verify-failed` is written as landed since chunk 2026-10-26-server-verification-before-any-frame, at the six frame verbs'' exit-21 server-verification refusal; its `--json` document stays `detail:null`, as the not-running cause''s does)"; the rest of the line stands.'
    sidecar: 'surface cli-send: the D-20 code list''s `server-verify-failed` qualified as written by this build, the exit-21 `--json` document still `detail:null` for that cause (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Spec claims disproved bullet, pattern `svf`: design-system-detail/surface.md:73 is a closed-enum list, still true, and an «as landed: written by this build» qualifier is a candidate; Schema / config: this build is the first that writes `server-verify-failed` (`process-exit{exit_code:21, during:"connect"}`), with no `--json` code added.'
    basis: .andromeda/design-system-detail/surface.md:73
note: 'F4 (hook fails open silently, the mcp frame tools return `refused instance-unreachable`) fits no picked entity (Platform-Specific Notes'' detail, for the new-detail pass); the body''s exit-code line (design-system.md:338) holds unmoved; layout-templates.md:141-143 showing only the not-running hint is layout-templates'' own; the log lines'' writing is obs-plan''s.'
