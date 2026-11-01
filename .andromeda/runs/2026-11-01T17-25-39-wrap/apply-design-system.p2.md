# Apply — design-system · part 2 of 2

## Verdicts
- fanout-design-system--cascade-new-surface.md 1 · applied · .andromeda/design-system-detail/surface.md:140+#1
- fanout-design-system--cascade.md 1 · applied · .andromeda/design-system-detail/surface.md:46#1
- fanout-design-system--cascade.md 3 · applied · .andromeda/design-system-detail/surface.md:68+#1
- fanout-design-system--cascade.md 4 · applied · .andromeda/design-system-detail/surface.md:76+#1
- fanout-design-system--cascade.md 5 · applied · .andromeda/design-system-detail/surface.md:84+#1
- fanout-design-system--cascade.md 6 · applied · .andromeda/design-system-detail/surface.md:85#1
- fanout-design-system--cascade.md 7 · applied · .andromeda/design-system-detail/surface.md:85#2, .andromeda/design-system-detail/surface.md:85#3
- fanout-design-system--cascade.md 8 · applied · .andromeda/design-system-detail/surface.md:85#4
- fanout-design-system--cascade.md 9 · applied · .andromeda/design-system-detail/surface.md:92#1
- fanout-design-system--cascade.md 10 · applied · .andromeda/design-system-detail/surface.md:92+#1
- fanout-design-system--cascade.md 11 · applied · .andromeda/design-system-detail/surface.md:98#1
- fanout-design-system--cascade.md 12 · applied · .andromeda/design-system-detail/surface.md:98+#1
- fanout-design-system--cascade.md 13 · applied · .andromeda/design-system-detail/surface.md:138+#1
- fanout-design-system--cascade.md 14 · applied · .andromeda/design-system-detail/surface.md:138#1
- fanout-design-system--cascade.md 15 · applied · .andromeda/design-system-detail/surface.md:149#1
- fanout-design-system--cascade.md 16 · applied · .andromeda/design-system-detail/surface.md:157#1
- fanout-design-system--cascade.md 17 · applied · .andromeda/design-system-detail/surface.md:157#2
- fanout-design-system--cascade.md 18 · applied · .andromeda/design-system-detail/surface.md:176#1

## Edits
- line: .andromeda/design-system-detail/surface.md:46
  old: «The board's strict-modes scan holds the home to two rules: it checks the home by the write rule, then each instance's set, which holds the home to the read rule. So a home that group or other may read but not write passes as the empty board when no instance exists, and with an instance refuses the whole board with the instance-state pair naming that instance: which pair the refusal prints depends on whether an instance exists. Pending, owner: a CARRY on the working-route entry «Unix endpoint and home hardening».»
  new: «The board's strict-modes scan holds the home to the read rule on every OS and `instances/` to the write rule, then each instance's set: a home another user may read refuses the whole board with the home-state pair, with or without an instance; the order and the no-open rule hold.»
- after: .andromeda/design-system-detail/surface.md:68
  anchor: «    - server verification fails: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`»
  add: «    - the socket directory is refused: the hint of `run`'s socket-directory pair below»
- after: .andromeda/design-system-detail/surface.md:76
  anchor: «    - `unable: the viola home is not private to you` (a home-level refusal of the home-tree check made at start before any log, of the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/`, each that exists; no child) → `hint: the viola home must be readable only by you; viola does not change its permissions`; an instance-level refusal of the same check prints `unable: <name>'s state files can be written by another user` → `hint: viola will not read them; make the viola home and its files owner-only`»
  add: «    - `unable: the socket directory for <name> cannot be trusted` (before any snapshot, child or bind) → `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`»
- after: .andromeda/design-system-detail/surface.md:84
  anchor: «    - the home-tree check of its start, after a passed preflight and before any log (no child): a home-level refusal → `unable: the viola home is not private to you` → `hint: the viola home must be readable only by you; viola does not change its permissions`; an instance-level refusal → the instance pair above; under `--json` the `strict-modes-failed` document»
  add: «    - a refused socket directory at its start: `run`'s socket-directory pair above»
- line: .andromeda/design-system-detail/surface.md:85
  old: «one of five closed words»
  new: «one of six closed words»
- line: .andromeda/design-system-detail/surface.md:85
  old: «`strict-modes-failed`, `server-verify-failed`, `already-live`»
  new: «`strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`, `already-live`»
- line: .andromeda/design-system-detail/surface.md:85
  old: «whose `--json` document reads the same word.»
  new: «whose `--json` document reads the same word. So is `socket-dir-refused`, at those eight verbs' exit 21 and at `run`'s and `revive`'s start.»
- line: .andromeda/design-system-detail/surface.md:85
  old: «The socket-directory cause and macOS's server check stay without these codes; owner: the route entry "Unix endpoint and home hardening".»
  new: «On macOS `server-verify-failed` means an euid mismatch alone: no peer pid is compared, a named gap no route entry owns.»
- line: .andromeda/design-system-detail/surface.md:92
  old: «`unwrapped-name` and `endpoint-lost` (»
  new: «`unwrapped-name`, `endpoint-lost` and `socket-dir` (»
- after: .andromeda/design-system-detail/surface.md:92
  anchor: «- The server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, the eight verbs that write a frame, as the hint cause `server-verify` beside `not-running`, `unwrapped-name` and `endpoint-lost` (`not-running` is the hint key of the log's `instance-dead`): it prints the server-verification line above byte for byte, plain ASCII with no SGR, and it holds no pid, start time, path or endpoint.»
  add: «- A refused socket directory at `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` or `unlink` exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the hint of cause `socket-dir`, no strict-modes pair; under `--json` the `instance-unreachable` document with `detail` `socket-dir-refused`, stderr empty.»
- line: .andromeda/design-system-detail/surface.md:98
  old: «the server check passes with no comparison and the server-verification hint cannot print there (a recorded gap; owner: the route entry "Unix endpoint and home hardening").»
  new: «the server check compares no pid (a named gap, owned by no route entry); it compares the peer's euid, and a mismatch alone prints the server-verification hint there.»
- after: .andromeda/design-system-detail/surface.md:98
  anchor: «- On macOS no peer pid is available, so the server check passes with no comparison and the server-verification hint cannot print there (a recorded gap; owner: the route entry "Unix endpoint and home hardening").»
  add: «- OPEN — the socket-directory pair's wording is a planning form · owner: the Epoch 7 boundary batch»
- line: .andromeda/design-system-detail/surface.md:138
  old: «`squatted-name` · `internal-error`»
  new: «`squatted-name` · `socket-dir-refused` · `internal-error`»
- after: .andromeda/design-system-detail/surface.md:138
  anchor: «- **`revive --json`'s documents:** one exit-1 document for every refusal of the start it shares with `run`, its closed codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; `run` itself prints none of these documents.»
  add: «- **`revive --json`'s socket-directory refusal:** one stdout document with the code `socket-dir-refused`, no hint, stderr empty; a failed create of the directory and a refused check read that one code.»
- after: .andromeda/design-system-detail/surface.md:140
  anchor: «- **A `pump_child` error under `revive --json`:** prints no document: the child has held the terminal and viola writes nothing past it.»
  add: «- **A peer the wrapper refuses:** dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.»
- line: .andromeda/design-system-detail/surface.md:149
  old: «`<name>'s state files can be written by another user`, or a `viola revive` refusal `unable: <name>'s state files can be written by another user` / `<name> has no logged session to resume` / `<name> has no logged session with that id` / `<name>'s session is open in another claude` / `<name>'s recorded directory is missing` / `the viola home is not private to you` (or `run`'s `already live` pair)»
  new: «`<name>'s state files can be written by another user` / `the socket directory for <name> cannot be trusted`, or a `viola revive` refusal `unable: <name> has no logged session to resume` / `<name> has no logged session with that id` / `<name>'s session is open in another claude` / `<name>'s recorded directory is missing` (or `run`'s `already live`, home, instance and socket-directory pairs)»
- line: .andromeda/design-system-detail/surface.md:157
  old: «(not running / unwrapped / endpoint lost / server verification)»
  new: «(not running / unwrapped / endpoint lost / server verification / socket directory)»
- line: .andromeda/design-system-detail/surface.md:157
  old: «`endpoint-lost` · `server-verify-failed`; the `list`»
  new: «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`; the `list`»
- line: .andromeda/design-system-detail/surface.md:176
  old: «(its pid or start time differs, or the check cannot be made)»
  new: «(its pid or start time differs, or the check cannot be made; on Linux and macOS also a peer whose euid is not this user's or cannot be read, on macOS the euid alone)»

## Patterns
[[pattern]]
id = "ds2-01"
fixed = 'holds the home to two rules'

[[pattern]]
id = "ds2-02"
fixed = 'one of five closed words'

[[pattern]]
id = "ds2-03"
fixed = 'stay without these codes'

[[pattern]]
id = "ds2-04"
fixed = 'passes with no comparison'
