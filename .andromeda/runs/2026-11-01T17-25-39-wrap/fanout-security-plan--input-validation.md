proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 42
    section: "## Input Validation"
    change: "In the last sentence replace «runs `check_path` on the home and on `instances/`» with «runs `check_read` on the home (the read rule) and `check_path` on `instances/`, on every OS»; the order, the whole-board `ScanRefused::Home` and no file opened stay as written; append «The read rule on the home is the overseer's technical answer as the founder's delegate, PROVISIONAL, the founder's own word owed at Epoch 7's boundary batch (chunk 2026-11-01-unix-endpoint-and-home-hardening, `inputs#I2` Q3); `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home.»"
    sidecar: "§Input Validation, Home resolution: `viola list` holds the home to the read rule on every OS, `instances/` staying `check_path`; PROVISIONAL on the delegate's answer (`inputs#I2` Q3)."
    rationale: "report.md:26 retires `viola list`'s two rules on the home and report.md:32 names input-validation.md:42 under pattern `board-rule`; report.md:18 gives `check_read` on the home and `check_path` on `instances/`, on every OS, with the three write-rule keepers; report.md:50 and report.md:94 give Q3's authority."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 39
    section: "## Input Validation"
    change: "Extend the closed list of `viola revive --json`'s exit-1 codes with `socket-dir-refused`, so it holds the eleven codes; every other word stays."
    sidecar: "§Input Validation, `revive --json` codes: `socket-dir-refused` joins the closed exit-1 list, ten to eleven."
    rationale: "report.md:29 moves `revive --json`'s exit-1 codes ten to eleven and each list holding all ten extends; this line holds all ten; report.md:14 has `revive`'s start share the refusal."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: "## Input Validation"
    change: "In **Frame verbs:** replace «and then, on Linux and Windows, the server check before the frame (IPC client-side server verification row)» with «and then `client::connect`'s checks in order, before the frame: the socket directory, the peer euid and the server check, the pid and start time on Linux and Windows, macOS comparing no peer pid, the named carve-out (**At `connect`:** below; IPC client-side server verification row)»."
    sidecar: "§Input Validation, Frame verbs: the socket-directory and peer-euid checks stand between `live_endpoint` and the server check."
    rationale: "report.md:15 gives `connect`'s order; report.md:31 widens `server-verify-failed` to macOS as an euid mismatch alone; report.md:33 keeps macOS's pid gap named and owned by no route entry; report.md:110 has macOS's comparison absent and named."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: "## Input Validation"
    change: "After «`viola mcp`'s `send` / `wait` / `last` / `answer` tools run `check_instance` before `live_endpoint`; a refusal returns `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line» add «; their `frame` then runs `check_endpoint_dir`, a refusal `instance-unreachable` with `detail` `socket-dir-refused`, then the peer euid and the server check, a refusal `server-verify-failed`, each with no frame and no line, `isError` the record»."
    sidecar: "§Input Validation, MCP frame tools: `check_endpoint_dir` after `live_endpoint`, the peer euid beside the server check."
    rationale: "report.md:17 gives MCP `frame`'s check after `live_endpoint`, the `SocketDir` and `ServerVerify` causes, no frame, no line, `isError` the record; report.md:73 records the same."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 41
    section: "## Input Validation"
    change: "Replace «through `client::refuse_untrusted`, the one check they share with the six frame verbs» with «through `client::refuse_untrusted`, the instance check they share with the six frame verbs, and `client::connect`'s checks after it (**At `connect`:** below)»."
    sidecar: "§Input Validation, `link` / `unlink` arguments: the verbs also take `connect`'s checks, one of its eight callers."
    rationale: "report.md:15 counts `link` and `unlink` among `connect`'s 8 callers, so a socket-directory or euid refusal reaches them too; report.md:71 applies the check at the eight CLI frame verbs; «the one check they share» no longer reads whole."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 41
    section: "## Input Validation"
    change: "New line after this one: « **Socket directory:** `viola_state::strict::check_socket_dir` takes one `symlink_metadata` of the path: an absent path passes, an unstattable one is `Refused::Unreadable`, a symlink `Refused::Symlink`, not a directory `Refused::NotADirectory`, else `unix_read_verdict`; off Unix it returns `Ok(())`. It never chmods, so a refused directory stays as found. `check_endpoint_dir` runs it on a recorded endpoint's parent. `viola run` and `revive`'s start run it through `socket_dir_ready` (the check, the parent created only when absent, the check again) before any snapshot, child or `Server::bind`: a refusal or a failed create is exit 1 with `process-exit` `detail` `socket-dir-refused`, and under `revive --json` that code, no bind and no child. No path, pid or uid on any of it. The closed word is the founder's own answer (chunk 2026-11-01-unix-endpoint-and-home-hardening, `inputs#I2` Q1).»"
    sidecar: "§Input Validation, CLI row: the socket-directory check and the start's refusal, `socket-dir-refused`."
    rationale: "report.md:11 gives the check and its verdicts; report.md:14 and report.md:95 give the start's order and the one code; report.md:70 gives the line and no path, pid or uid; report.md:93 gives Q1."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 41
    section: "## Input Validation"
    change: "New line after the **Socket directory:** line: « **At `connect`:** the three client seams, `src/cmd/client.rs` `connect` (8 callers: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`), `src/cmd/hook.rs` `connect` and `viola-mcp`'s `frame`, each run `check_endpoint_dir` on the recorded endpoint, then read `peer_admitted(client.server_euid())`, then the server check. `peer_euid_admitted` takes only the owning user where the euid is reported and refuses a peer it cannot read there; off Unix none is reported, the DACL and the pid comparison staying the controls. On the CLI a refused directory is exit 21, `instance-unreachable` with `detail` `socket-dir-refused`, one WARN `process-exit` with `during` `connect` and no `channel-request`; a refused euid reads `server-verify-failed`. In `viola hook` a refused directory is `NotSent::SocketDir` / `Asked::SocketDir`, a refused euid `NotSent::ServerUnverified`: exit 0, empty streams, no frame, one `hook-decision` at WARN. A foreign euid reading `server-verify-failed` on Linux and macOS is the founder's own answer (`inputs#I2` Q2).»"
    sidecar: "§Input Validation, CLI row: the client seams' three checks in order, and what each refusal reads on the CLI and in the hook."
    rationale: "report.md:12 gives the decision and the Windows carve-out; report.md:15, report.md:16 and report.md:17 give the three seams; report.md:71 and report.md:72 give the lines; report.md:93 gives Q2."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
note: "No D-security-input, D-security-auth or D-security-deps violation in this category: the recorded endpoint's parent and the peer euid are checked per report.md:70-75 and report.md:23 adds no dependency; line 42's kept parenthetical «each through `symlink_metadata`» will now cover `check_read`, whose read call the report does not state, so the orchestrator should confirm it stands."
