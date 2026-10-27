# security · cascade read · group 1 of 3

## Carried
- (.andromeda/security-plan-detail/threat-model-summary.md:18) The statusline command, its two sources, `config.json` and the stamps decide what code runs or whether dialogs are answered, so each read of them must be capped, tolerant and listed in the enumeration.
  > `snapshot.json` stores `statusline_command`, the user-written shell string that `hook statusline` reads through `VIOLA_DIR` and runs through a shell. This is the only shell-out in viola.
  (.andromeda/security-plan-detail/threat-model-summary.md:19)
  > `<home>/statusline-source.json` (the Claude settings shape; viola only reads it) and, for the default home only, the user's `~/.claude/settings.json` are the inputs that decide `statusline_command` at each start.
  (.andromeda/security-plan-detail/threat-model-summary.md:16)
  > `config.json` holds budget thresholds.
  (.andromeda/security-plan-detail/threat-model-summary.md:21)
  > `ledger/stamps.json` holds the verified CLI versions. A stamp is what switches on dialog answering.
- (.andromeda/security-plan-detail/threat-model-summary.md:44) The channel frame's input half is a boundary the enumeration must list with every served method, each frame capped, parsed at default depth and its params closed on both sides.
  > Methods: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, and `hook.event` notifications.
  (.andromeda/security-plan-detail/threat-model-summary.md:58)
  > Input validation is limited to JSON-RPC parsing, the `v` check (`-32602`) and the `ViolaName` nutype.
- (.andromeda/security-plan-detail/authentication-authorization.md:12) Each served method and the `hook.event` notification must keep its own params check in the wrapper, the per-method closed-params property the proof enumerates.
  > the nine served methods (`send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`) and the `hook.event` notification, each behind the IPC rows below and its own params check (§ Input Validation, Channel frames).
  (.andromeda/security-plan.md:106)
  > RBAC / permissions — capability-based, all or nothing: any caller that passes the OS check gets the full surface; no roles, the only principal being the local OS user.
- (.andromeda/security-plan-detail/authentication-authorization.md:21) The channel server is reached whole, so the frame cap the chunk proves must stay behind the peer check: a foreign-euid peer is dropped before any frame is read.
  > On every accept, `stream.peer_creds()`; if `euid() != geteuid()`, drop the connection before reading a frame.
  (.andromeda/security-plan.md:112)
  > IPC server-side peer check (Unix) — on every accept, a peer whose euid differs from the server's is dropped before a frame is read.
- (.andromeda/security-plan-detail/authentication-authorization.md:24) The sync and Tokio clients are reached whole, so a bound added there must keep the server check before the first frame of every method and the SQOS open with no default-connect fallback.
  > **Timing:** all of this runs before the first frame of any method is written on the connection, including `hook.event` notifications (UserPromptSubmit, Stop and PostToolUse carry prompt text, `last_assistant_message` and tool data) and `wait`/`last` requests, not only `send`, `answer` and `hook.dialog`.
  (.andromeda/security-plan-detail/authentication-authorization.md:24)
  > The client never falls back to interprocess's default connect, which opens without SQOS and lets the server impersonate (measured: `SecurityImpersonation`) (§ Authentication).
  (.andromeda/security-plan-detail/authentication-authorization.md:25)
  > every client runs the instance's strict-modes check before it reads the snapshot (`viola hook`'s event and dialog paths through `canonical` and `trusted` in `src/cmd/hook.rs` before obs init; CLI `send` through `client::refuse_untrusted` after its text read)
  (.andromeda/security-plan-detail/authentication-authorization.md:25)
  > `send` is served by the wrapper as of chunk 2026-10-04-confirmed-send-with-cl-1-records, under the channel row's controls (frame cap, depth, version, closed `send` params).
  (.andromeda/security-plan.md:114)
  > before the first frame of any method, the client runs the instance's strict-modes check, a refusal writing no frame and no line, and only then verifies the server against its one snapshot read
- (.andromeda/security-plan-detail/authentication-authorization.md:30) `release`'s params are a channel method's params, so the closed type check over them, a string `from` refused before any budget is acted on and a `from` of another type `-32602` with `data: null`, is part of the closed-params proof.
  > The wrapper rejects a `release` whose `params` carry a string `from` with `-32602` `"invalid params"` and `data: {"reason":"release-from-driver"}` (`viola_channel::ProtocolError::ReleaseFromDriver`), and logs one `release-from-driver` line (`from_trust:"self-reported"`); a `from` of another type is `-32602` with `data: null`.
- (.andromeda/security-plan-detail/authentication-authorization.md:34) Every bounded read the chunk adds or moves under the home must come after that entry point's strict-modes check, never before it.
  > **Strict-modes check:** in every process that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`
  (.andromeda/security-plan.md:118)
  > a strict-modes check refuses a trusted path another user can write, a home, instance directory or `diagnostics/` another user can read (Unix any group or other bit; Windows an allow ACE granting a read right to another SID, inherit-only included) or a symlinked `diagnostics/` folder or file, in every process that reads a trusted file, before its first read or open under the home.
- (.andromeda/security-plan-detail/authentication-authorization.md:38) The two statusline sources and `budget.json` sit outside the trusted-file list, so their only control is the capped, tolerant, checked read the chunk enumerates, a failed `budget.json` read being no reading.
  > Both decide the command `hook statusline` later runs, and their controls are the read-only, capped, tolerant read of § Input Validation (a boundary widening the founder ratified, live, 2026-10-10, relayed verbatim by the operator).
  (.andromeda/security-plan-detail/authentication-authorization.md:38)
  > The read runs `check_path` on the home and on `budget.json` first, and a refused or unreadable file is no reading, on which the gate does not pause.
- (.andromeda/security-plan-detail/authentication-authorization.md:44) `run` and `revive`'s start are reached whole, so every read at their start comes after `check_home_tree`, and the pinned key joined into `bin/<key>/` and `plugin/<key>/` is the one `checked_exe` takes from the exe's bytes.
  > `run` and `revive`'s start run it through `checked_exe` (the exe's bytes read once, its `pin_key` taken) before the wrapper log opens; a refusal exits 1 with the home or the instance `unable:` / `hint:` pair (`refuse_tree`; `revive --json` the `strict-modes-failed` document), no log, no line and no child.
- (.andromeda/security-plan-detail/authentication-authorization.md:72) The client half of the paste rule must refuse with exit 13 before the strict-modes, liveness and server checks and before any frame, `send`'s instance check coming after its text read.
  > The client's own exit-13 refusals come first, and every frame verb's strict-modes refusal comes before its liveness and server checks: `client::refuse_untrusted` runs `check_instance` first (`send` after its text read)
- (.andromeda/security-plan-detail/authentication-authorization.md:81) The wrapper-side name check on channel `link` / `unlink` params is witnessed by this test, which the proof's per-boundary enumeration must name.
  > `channel_link_names_are_validated_by_the_wrapper` sends raw bad-name, foreign-`driven` and self-pair `link` / `unlink` frames past every CLI check and reads `-32602` with no line written;
- (.andromeda/security-plan-detail/threat-model-summary.md:66) The paste boundary's rule must hold at every client and authoritatively in the wrapper: the typed text's CR / LF shape, and an empty typed text refused `empty-text` with nothing written.
  > A bridge send is one `ESC[200~…ESC[201~` + CR write of the typed text: the text from stdin, `--file` or MCP `send.text`, with every CR LF pair and every other CR inside it as one LF, and without its trailing CR and LF characters (Established Decisions: Delivery Confirmation; Human Takeover / Wheel). A text whose typed text is empty is refused `not-delivered` / `empty-text` and nothing is written.
  (.andromeda/security-plan-detail/threat-model-summary.md:67)
  > Text containing one would end the paste early, and the rest would reach the `claude` TUI as typed keystrokes.
  (.andromeda/security-plan.md:37)
  > **Vector:** CLI input, the `send` text written into the PTY as one bracketed paste of the typed text: an embedded paste terminator or other control sequence would end the paste early, and the rest would reach the `claude` TUI as typed keystrokes.
- (.andromeda/security-plan-detail/threat-model-summary.md:74) The terminal boundary is a row of the threat-model set the enumeration is checked against: its type rule is the classifier's closed list, and no bound the chunk adds may refuse or delay a human byte.
  > Every byte the terminal writes to `viola run`'s stdin: the human's keys, and the terminal's own focus reports, mouse reports and replies to the child's queries. A byte that is typing takes the wheel from the driver (`wheel{holder:"human",cause:"human-input"}`), and the next automation `send` is refused `human-typing` (Established Decisions: Human Takeover / Wheel).
  (.andromeda/security-plan-detail/threat-model-summary.md:75)
  > The stdin classifier's closed non-editing list (the founder's ruling F-W2): only a sequence on it never counts as typing, and every other byte does.
- (.andromeda/security-plan-detail/threat-model-summary.md:82) Every hook event's stdin and `hook statusline`'s are capped boundaries the enumeration lists, parsed tolerantly, never read as a command, failing open, and read only after the instance check.
  > JSON on stdin for SessionStart, UserPromptSubmit, PreToolUse (`AskUserQuestion|ExitPlanMode`), PermissionRequest, Stop, SessionEnd, Notification, PostToolUse and PostToolUseFailure, plus `hook statusline` with statusline JSON (Established Decisions: Hook Transport; Occupied Resources).
  (.andromeda/security-plan-detail/threat-model-summary.md:84)
  > Parsing is tolerant in `viola-agent-claude`: no `deny_unknown_fields`, serde_path_to_error for drift reports (Established Decisions: Validation).
  (.andromeda/security-plan-detail/threat-model-summary.md:85)
  > Upstream text is treated as content, never as commands (R1).
  (.andromeda/security-plan-detail/threat-model-summary.md:86)
  > Any failure fails open to the human: exit 0 with no body.
  (.andromeda/security-plan-detail/threat-model-summary.md:88)
  > Every hook path (event, dialog and `hook statusline`) runs the instance's strict-modes check on the canonicalised `VIOLA_DIR` instance before anything under the home is read or opened, the `--capture` arms ahead of it; a refused or absent instance exits 0 with no `config.json` read, no role file, no stdin read, no line and no frame.
  (.andromeda/security-plan.md:41)
  > **Vector:** Hook stdin payloads from Claude Code, exec-form `viola hook <event>`: parsed tolerantly, upstream text treated as content, never as commands, and any failure failing open to the human; `hook statusline` runs `statusline_command` through a shell, its integrity set by filesystem permissions on the viola home.
- (.andromeda/security-plan-detail/threat-model-summary.md:97) The MCP tool inputs are a boundary the enumeration lists; the schemas are advisory, so each handler must check its raw arguments itself and refuse before any frame, after its instance check.
  > Tool input schemas (schemars) and the `ViolaName` newtype check the input.
  (.andromeda/security-plan-detail/threat-model-summary.md:95)
  > Tools `send`, `wait`, `last`, `answer` and `list`, called by the driver session's model (Established Decisions: MCP).
  (.andromeda/security-plan-detail/threat-model-summary.md:98)
  > Before its frame, each of `send`, `wait`, `last` and `answer` runs the instance's strict-modes check (`check_instance` on the home and the instance directory) ahead of the liveness pre-check and the server check; a refusal is `instance-unreachable` with `detail: null`, no frame and no line.
  (.andromeda/security-plan.md:43)
  > tool input schemas and the `ViolaName` newtype check the input
- (.andromeda/security-plan-detail/threat-model-summary.md:111) The CLI's stdin / `--file` inputs and the home-choosing `--home` / `VIOLA_DIR` are boundaries the enumeration lists, read under the cap after the strict-modes check.
  > `answer` takes its `response` JSON from stdin or `--file`.
  (.andromeda/security-plan-detail/threat-model-summary.md:110)
  > `--home <dir>` and `VIOLA_DIR` choose which state tree, snapshot, `endpoint`, `statusline_command` and ledger stamps are used (Cross-cutting Patterns: Config management).
  (.andromeda/security-plan-detail/threat-model-summary.md:113)
  > Same-user invocation, clap parsing, the `ViolaName` nutype, and tolerant `config.json` parsing.
  (.andromeda/security-plan-detail/threat-model-summary.md:114)
  > Before its first read or open under the home, `run`, `revive`'s start and `verify` hold the home tree to the strict-modes check (`check_home_tree`; a refusal exit 1, no log opened), and the frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` hold the instance to it (`check_instance`; a refusal exit 21)
- (.andromeda/security-plan-detail/threat-model-summary.md:122) The home's cross-process readers are files the chunk caps; each must tolerate torn lines, and a replayed snapshot must still carry no endpoint, pid, start time or command.
  > Readers across processes: `snapshot.json`, `events.ndjson`, `budget.json`, `config.json`, `ledger/stamps.json`, `heartbeat`.
  (.andromeda/security-plan-detail/threat-model-summary.md:127)
  > Readers tolerate torn lines (they skip and count them, and the next append heals a torn last line).
  (.andromeda/security-plan-detail/threat-model-summary.md:127)
  > a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so it yields no frame and runs no command (Standard Contracts: Snapshot envelope).
- (.andromeda/security-plan-detail/threat-model-summary.md:144) The `claude agents --json` and `--version` outputs, and verify's PTY output, are child-output readers the enumeration lists, each capped and parsed tolerantly with failure `unknown`.
  > `list` and `mcp` spawn `claude agents --json`, resolving `claude` from their own PATH with npm-shim → `claude.exe` resolution.
  (.andromeda/security-plan-detail/threat-model-summary.md:145)
  > `run` resolves the child executable and runs it once with `--version`.
  (.andromeda/security-plan-detail/threat-model-summary.md:148)
  > its output capped at `MAX_FRAME` into a `Screen` fed under `catch_unwind`
  (.andromeda/security-plan-detail/threat-model-summary.md:150)
  > Output parsing is tolerant, and failure becomes `unknown`.
  (.andromeda/security-plan.md:49)
  > output parsing is tolerant and failure becomes `unknown`

## Bindings
- **threat-model boundaries ↔ input validation**: the enumeration's rows are the threat model's boundaries, IPC frames (.andromeda/security-plan-detail/threat-model-summary.md:44), paste (.andromeda/security-plan-detail/threat-model-summary.md:66), terminal (.andromeda/security-plan-detail/threat-model-summary.md:74), hook stdin (.andromeda/security-plan-detail/threat-model-summary.md:82), MCP (.andromeda/security-plan-detail/threat-model-summary.md:95), CLI (.andromeda/security-plan-detail/threat-model-summary.md:111), filesystem (.andromeda/security-plan-detail/threat-model-summary.md:122) and child spawn (.andromeda/security-plan-detail/threat-model-summary.md:144), each carrying the cap, depth and type rule the Input bans set (.andromeda/security-plan.md:451), (.andromeda/security-plan.md:452), (.andromeda/security-plan.md:453).
- **auth strict-modes order ↔ the size caps on home files**: every capped read of the home's files (.andromeda/security-plan-detail/threat-model-summary.md:122) comes after the entry point's check (.andromeda/security-plan-detail/authentication-authorization.md:34), (.andromeda/security-plan-detail/authentication-authorization.md:44), (.andromeda/security-plan-detail/threat-model-summary.md:114), the statusline sources and `budget.json` controlled only by that capped read (.andromeda/security-plan-detail/authentication-authorization.md:38).
- **auth channel peer and server checks ↔ the frame cap**: the server's frame cap sits behind the peer drop (.andromeda/security-plan-detail/authentication-authorization.md:21) and the clients' bounded exchange behind the server check before the first frame (.andromeda/security-plan-detail/authentication-authorization.md:24), the request size limit holding on both sides (.andromeda/security-plan.md:469).
- **paste boundary ↔ client refusal order ↔ the paste ban**: the paste rule (.andromeda/security-plan-detail/threat-model-summary.md:66), (.andromeda/security-plan.md:449), (.andromeda/security-plan.md:450) is refused client side with exit 13 before any check or frame (.andromeda/security-plan-detail/authentication-authorization.md:72) and again in the wrapper (.andromeda/security-plan.md:447).
- **closed channel params ↔ names before path joins**: each method's params check (.andromeda/security-plan-detail/authentication-authorization.md:12), `release`'s closed `from` (.andromeda/security-plan-detail/authentication-authorization.md:30) and the `link` / `unlink` name witness (.andromeda/security-plan-detail/authentication-authorization.md:81) carry the name ban (.andromeda/security-plan.md:448) into the wrapper.
- **hook stdin ↔ fail open**: the hook boundary's failure (.andromeda/security-plan-detail/threat-model-summary.md:86) and every bound added on it (.andromeda/security-plan.md:480) stay exit 0 with no body and no stderr, the statusline command never run past a failed check (.andromeda/security-plan.md:460).
