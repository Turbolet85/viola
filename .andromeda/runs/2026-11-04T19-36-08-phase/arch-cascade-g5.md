@carried n=1 label=architecture/conventions fact=fct-5uakjii2
why: every CLI --json refusal document must carry these same refusal reasons and closed detail words on Linux, macOS and Windows, a field one OS cannot fill reading its documented null, never an OS-only word.
quote: **`RefusalReason` (serialised kebab-case):** `human-typing` · `budget-paused` · `unverified-cli` · `not-delivered` · `unknown`. `unknown` is `#[serde(other)]`, so an older client can still read a refusal from a newer wrapper. `detail` values are kebab-case strings from a closed set per reason, extended only in `viola-core`:
@carried n=1 run=2 label=architecture fact=fct-u5xw5irn
quote: viola omits absent fields on write, and readers treat missing and `null` the same.
@carried n=2 label=architecture/conventions fact=fct-jpo3rq7m
why: the one-schema rule must hold exit 21's closed cause set identical on every OS, so a Unix-only cause such as `socket-dir-refused` stays a word of the one shared set and no OS adds another.
quote: **Unreachable cause:** `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir`, each word through `as_str`, `SocketDir`'s `socket-dir-refused`) is exit 21's closed cause set. It has no serde path: the two clients write every word. It is no `RefusalReason`.
@carried n=3 label=architecture/conventions fact=fct-vuikun3q
why: the scope adds no exit code and no OS-only word, so the revive --json refusal codes stay this closed list and the typed exit-code set stays whole on every OS, `hook` keeping its exit-0 contract.
quote: `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`; `client::refused_document`) with stderr empty
@carried n=3 run=2 label=architecture fact=fct-jdhp6aqu
quote: `2` usage error · `10` `human-typing` · `11` `budget-paused` · `12` `unverified-cli` · `13` `not-delivered` · `14` `unknown` refusal · `20` protocol fault from the wrapper · `21` instance unreachable
@carried n=3 run=3 label=architecture fact=fct-nppkdemz
quote: **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.
@carried n=4 label=architecture/conventions fact=fct-2y3oelpa
why: the start's refused socket directory is a Unix-only refusal the Unix suite must read the same on Linux and macOS, its document word and its process-exit line holding the one schema and carrying no path, pid or uid.
quote: Exit `1` also covers `viola run` and `viola revive`'s start refusing the socket directory (`socket_dir_ready` in `src/cmd/run.rs`, before any snapshot, child or `Server::bind`): the stderr `unable:` / `hint:` pair (its wording: design-system, Surface: cli), or under `viola revive --json` the document with code `socket-dir-refused`, and one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`; no path, pid or uid.
@carried n=5 label=architecture/conventions fact=fct-7s455w3m
why: the server check differs per OS, so a parity test must read the peer-euid arm on both Unix OSes and the pid and start-time arm on Linux only, each writing no frame and the same `server-verify-failed` document.
quote: Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, a server that is not the recorded wrapper (on Unix a connected peer whose euid is not this user's or cannot be read, `peer_admitted`; on Linux and Windows the connected peer's pid or OS start time not the snapshot's, or that check not made): no frame is written, and the verb prints `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` under `--json` with stderr empty
@carried n=6 label=architecture/conventions fact=fct-fw4invxz
why: the scope keeps macOS's recorded gap, so on macOS `server-verify-failed` must mean an euid mismatch alone and no test may read a pid or start-time comparison there.
quote: An exception: `server-verify-failed` is claimed on Linux, Windows and macOS, where on macOS it means an euid mismatch alone; macOS compares no peer pid, `v1-18`'s named carve-out, owned by no route entry.
@carried n=7 label=architecture/conventions fact=fct-wm75dacy
why: the frame verbs' refused socket directory is a Unix-only arm whose process-exit line must conform to the one diag-line schema with the same fields and words on Linux and macOS.
quote: Exit `21`'s refused socket directory: on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, `src/cmd/client.rs` `connect`'s refused check is `CallError::SocketDirRefused` to `Unreachable::SocketDir`, writing one `process-exit` at WARN with `exit_code` 21, `detail` `socket-dir-refused` and `during` `connect`, and no `channel-request`.
@carried n=8 label=architecture/conventions fact=fct-gvna3tj2
why: the MCP frame tools' server check differs per OS, and the Unix suite must read its peer-euid arm returning the one `instance-unreachable` result shaped like CLI --json on every OS.
quote: A frame tool (`send`, `wait`, `last`, `answer`) whose connected server is not the recorded wrapper (on Unix its peer euid not this user's or unreadable, `peer_admitted`; else its pid or OS start time not the snapshot's, or the check cannot be made; either reads `UnreachableCause::ServerVerify`; the start time read on `tokio::task::spawn_blocking`) returns the `instance-unreachable` `isError` result, text `refused: instance-unreachable`, with no frame written and no log line.
@carried n=8 run=2 label=architecture fact=fct-5nrjpmo2
quote: a refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`
@carried n=9 label=architecture/conventions fact=fct-qfrfkv27
why: the MCP frame tools' refused socket directory is a Unix-only result whose `socket-dir-refused` detail must read the same closed word on Linux and macOS.
quote: A frame tool (`send`, `wait`, `last`, `answer`) meeting a refused socket directory: MCP's `frame`, after `live_endpoint`, runs `check_endpoint_dir` and answers `result::unreachable(UnreachableCause::SocketDir)`, the `instance-unreachable` `isError` result with `detail: "socket-dir-refused"`, with no frame written and no line by rule, `isError` the record.
@binding n=1
**conventions (the server-verify exit-21 arm and its macOS exception) ↔ contract-instance-snapshot and res-ipc-endpoints**: the server check compares the connected peer's pid and OS start time with the snapshot's `pid` and `started_at` on Linux and Windows only, and both Unix sides admit only the owning user's euid; the parity suite must read the euid arm on Linux and macOS and the pid arm on Linux, never on macOS (architecture/conventions fct-7s455w3m) (architecture/conventions fct-fw4invxz) (architecture fct-ld365mzq) (architecture fct-wular7qm)
@binding n=2
**conventions (the socket-directory refusals, start and frame verbs, CLI and MCP) ↔ res-ipc-endpoints**: the per-user 0700 socket directory checked before the bind and every connect, left as found when refused, is the Unix-only check whose refusal must read `socket-dir-refused` in one closed word on both Unix OSes (architecture/conventions fct-2y3oelpa) (architecture/conventions fct-wm75dacy) (architecture/conventions fct-qfrfkv27) (architecture fct-wular7qm)
@binding n=3
**conventions (the process-exit lines of the refusals) ↔ res-repository and res-filesystem (the diag-line schema)**: each `process-exit` a Unix-only refusal writes must conform to the obs-owned default-deny `schemas/diag-line.v1.json`, pinned by `tests/contract_diag_schema.rs`, with the same fields on every OS (architecture/conventions fct-2y3oelpa) (architecture/conventions fct-wm75dacy) (architecture fct-gn6wx4l2) (architecture fct-5jecrew3)
@binding n=4
**conventions (the closed refusal, cause and exit-code sets) ↔ contract-cli-json-output and the data model conventions**: every CLI --json and MCP document must carry the same closed words on every OS, a field one OS cannot fill omitted on write and read as null, never an OS-only word, and the typed exit-code set gains no code (architecture/conventions fct-5uakjii2) (architecture/conventions fct-jpo3rq7m) (architecture/conventions fct-vuikun3q) (architecture fct-ksmkl4jd) (architecture fct-u5xw5irn)
