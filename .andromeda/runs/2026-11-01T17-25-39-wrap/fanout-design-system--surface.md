proposals:
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 76
    section: "## Surface: cli › ### Component Patterns"
    change: "New list item after this line, at its indent: `unable: the socket directory for <name> cannot be trusted` (the directory of the endpoint is a symlink, not a directory, or not an owned directory with no foreign bit, or it cannot be created; checked before any snapshot, child or bind, never chmodded, left as found) → `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`; the pair a planning form (chunk 2026-11-01-unix-endpoint-and-home-hardening, `inputs#I3`, the overseer's word as the founder's delegate, PROVISIONAL), the founder's own wording owed at Epoch 7's boundary batch."
    sidecar: "Surface: cli: `viola run`'s exit-1 start refusals gain the socket-directory pair, a planning form, the founder's wording owed."
    rationale: "The report adds this pair (report Coverage, the new pair; `inputs#I3` row 1) and `refuse_socket_dir` at `run`; the start refusal list states no such wording."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 84
    section: "## Surface: cli › ### Component Patterns"
    change: "New list item after this line, at its indent: the socket-directory check `viola revive`'s start shares with `run`, before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` → `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` (`run`'s pair, the same planning form); under `--json` the `socket-dir-refused` document, `detail` null, no hint, stderr empty."
    sidecar: "Surface: cli: `viola revive`'s refusal list gains the socket-directory pair of the start it shares with `run`."
    rationale: "Report Symbols: `revive`'s start shares `refused_with(launch.json, socket-dir-refused)`; the revive refusal list holds its start's other check but not this one."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 68
    section: "## Surface: cli › ### Component Patterns"
    change: "New list item after this line, at its indent: the recorded endpoint's directory fails the socket-directory check (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, before any frame): `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` (last on stderr, plain ASCII), the planning form of the start refusal's hint."
    sidecar: "Surface: cli: `instance-unreachable`'s per-cause hints gain the socket-directory cause."
    rationale: "Report Symbols: `CallError::SocketDirRefused` reads hint cause `socket-dir` at the eight frame verbs; the per-cause hint list stops at server verification."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 85
    section: "## Surface: cli › ### Component Patterns"
    change: "Replace «one of five closed words (exit-code row 21)» with «one of six closed words (exit-code row 21)»."
    sidecar: "Surface: cli: exit 21's closed causes five → six in the hints item."
    rationale: "Report Counts: exit 21's closed causes five → six, the sixth `socket-dir-refused`; a row reading «five closed» changes."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 85
    section: "## Surface: cli › ### Component Patterns"
    change: "In the D-20 sentence's list insert `socket-dir-refused` after `server-verify-failed`; after «whose `--json` document reads the same word.» add «`socket-dir-refused` is written to the log by `run` and `revive`'s start at their exit-1 refusal and by the same eight verbs at their exit-21 refusal, whose `--json` document reads the same word; `viola hook` writes it as one `hook-decision` detail.»"
    sidecar: "Surface: cli: the D-20 sentence names `socket-dir-refused` and where it is logged."
    rationale: "Report Coverage: `process-exit` detail at `run` and `revive`'s start and at the frame verbs, one `hook-decision` at the hook; report's Expected amendments names the D-20 sentence."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:66
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 92
    section: "## Surface: cli › ### Component Patterns"
    change: "Replace «beside `not-running`, `unwrapped-name` and `endpoint-lost`» with «beside `not-running`, `unwrapped-name`, `endpoint-lost` and `socket-dir`»."
    sidecar: "Surface: cli: the hint causes gain `socket-dir`."
    rationale: "Report Symbols: `src/human/hints.rs` `send_hint` cause `socket-dir`; this line lists the causes without it."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:19
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 138
    section: "## Surface: cli › ### Component Patterns"
    change: "Add `socket-dir-refused` to the closed codes: replace «`squatted-name` · `internal-error`» with «`squatted-name` · `socket-dir-refused` · `internal-error`»."
    sidecar: "Surface: cli: `revive --json`'s exit-1 codes ten → eleven."
    rationale: "Report Counts: `revive --json`'s exit-1 codes ten → eleven, each list holding all ten extends."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 149
    section: "## Surface: cli › ### Component Patterns"
    change: "Exit row 1: after `run`'s «`<name>'s state files can be written by another user`,» list and after `revive`'s «`the viola home is not private to you`» add « / `the socket directory for <name> cannot be trusted`»; remove `verify`'s parenthetical «(its home tree refused before its log, no step line and no summary, or a `stamps.json` update refused by its strict-modes check)», which line 125 states whole, so the row stays under 2 000 chars (1 991 now, about 1 970 after)."
    sidecar: "Surface: cli: exit row 1 gains the socket-directory refusal at `run` and `revive`; a parenthetical the verify item already states leaves the row, holding it under the line limit."
    rationale: "Report's Expected amendments names exit-code row 1; the row enumerates each start refusal, and the entity shape bars a line over 2 000 chars, so the addition needs the room."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:66
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 157
    section: "## Surface: cli › ### Component Patterns"
    change: "Exit row 21, human cell: replace «(not running / unwrapped / endpoint lost / server verification)» with «(not running / unwrapped / endpoint lost / server verification / socket directory)»."
    sidecar: "Surface: cli: exit row 21's hints name the socket-directory cause."
    rationale: "Report Symbols: frame verbs read `Unreachable::SocketDir` with hint cause `socket-dir`, printing through `unable  instance-unreachable`."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 157
    section: "## Surface: cli › ### Component Patterns"
    change: "Exit row 21, `--json` cell: replace «`endpoint-lost` · `server-verify-failed`;» with «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`;»."
    sidecar: "Surface: cli: exit row 21's closed causes list extends to six."
    rationale: "Report Counts: a row listing the five extends, the list words ending in `socket-dir-refused`; the founder's own word, `inputs#I2` Q1."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 176
    section: "## Surface: cli › ### Component Patterns"
    change: "Replace «(its pid or start time differs, or the check cannot be made)» with «(its pid or start time differs, on Linux and macOS its euid is not this user's, or the check cannot be made; on macOS, with no peer pid, an euid mismatch alone)»."
    sidecar: "Surface: cli: `server-verify-failed`'s reach widens to a refused euid on Linux and macOS, on macOS the euid alone."
    rationale: "Report Counts: `server-verify-failed`'s reach moves to Linux, Windows and macOS, on macOS an euid mismatch alone, each row changes; the founder's own word, `inputs#I2` Q2."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 85
    section: "## Surface: cli › ### Component Patterns"
    change: "Replace the last sentence, «The socket-directory cause and macOS's server check stay without these codes; owner: the route entry …», with «macOS's server check makes no pid comparison, a named carve-out owned by no route entry: there `server-verify-failed` means an euid mismatch alone.»"
    sidecar: "Surface: cli: the socket-directory cause has its code; the route entry's owner clause on the D-20 sentence leaves, macOS's pid gap stays a named carve-out."
    rationale: "Report Retired: the dated gaps owed to the route entry are built; report Counts: each owner clause changes, except macOS's pid gap, which stays named but owned by no route entry."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 98
    section: "## Surface: cli › ### Component Patterns"
    change: "The item now reads: On macOS no peer pid is available, so the server check makes no pid comparison (a named carve-out, owned by no route entry); the client euid check runs there, so the server-verification hint prints on an euid mismatch alone."
    sidecar: "Surface: cli: the macOS pending record closes as a named carve-out; the hint can print there on an euid mismatch."
    rationale: "Report Counts: `verify-word` and `mac-gap`, macOS's pid gap stays named but owned by no route entry; «cannot print there» is false now that the euid check reaches macOS."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 46
    section: "## Surface: cli › ### Component Patterns"
    change: "The item now reads: The board's strict-modes scan holds the home to the read rule on every OS, `instances/` staying on its own check (`check_path`), in the same order: a home that group or other may read refuses the whole board with the home-state pair, whether or not an instance exists (chunk 2026-11-01-unix-endpoint-and-home-hardening; `inputs#I2` Q3, the overseer's technical answer as the founder's delegate, PROVISIONAL, the founder's own word owed at Epoch 7's boundary batch)."
    sidecar: "Surface: cli: `viola list`'s two rules on the home retire for the read rule on every OS, PROVISIONAL; the pending record closes."
    rationale: "Report Retired: `viola list`'s two rules on the home (`inputs#I2` Q3); report Symbols: `board.rs` `scan` takes the home through `check_read`, the whole-board `ScanRefused::Home` unchanged; report Counts names `surface.md:46`."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 176
    section: "## Surface: cli › ### Component Patterns"
    change: "New list item after this line: Exit 21's socket-directory cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a recorded endpoint whose directory is a symlink, not a directory, or not an owned directory with no foreign bit gets no frame and no connect, stays as found, and the verb prints `unable  <name>  instance-unreachable` (`send` its mirror's form) with the socket-directory hint last on stderr, or under `--json` the `instance-unreachable` document with `detail` `socket-dir-refused`, stderr empty; no path, pid or uid (chunk 2026-11-01-unix-endpoint-and-home-hardening; the word the founder's own, `inputs#I2` Q1; the hint a planning form, owed)."
    sidecar: "Surface: cli: exit 21's socket-directory cause recorded beside the server-verification cause."
    rationale: "Report Symbols `src/cmd/client.rs` `connect`, Coverage for the eight frame verbs, and Decisions (`send` prints its mirror's form): an output shape this category holds per cause, new this chunk."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
note: "D-design-tokens holds (every new element reads tokens n/a, no SGR); the body's own standing claims at .andromeda/design-system.md:303 («five closed words»), :334 (revive's ten codes) and :338 (the five causes) fall outside this file, so no proposal here names them."
