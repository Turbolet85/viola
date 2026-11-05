@carried n=1 label=architecture/conventions fact=fct-vuikun3q
why: the typed exit-code set and hook's always-exit-0 must come out of the removal unchanged on Linux, and the `.cmd`/`.bat` refusal recorded here is Windows-naming text the plan must list and decide against the removal.
quote: `viola run` refusing a `.cmd`/`.bat` child program before any spawn (`process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}`, the two fixed `unable:`/`hint:` stderr lines)
@carried n=1 run=2 label=architecture fact=fct-jdhp6aqu
quote: `0` ok · `1` internal error, and the start, revive-preflight and verify refusals
@carried n=1 run=3 label=architecture fact=fct-nppkdemz
quote: **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.
@carried n=2 label=architecture/conventions fact=fct-tg3zks37
why: verify's closed refusal codes include `batch-script-child`, a word naming a Windows behaviour that the plan must list and decide, while the Linux codes stay unchanged.
quote: `viola verify`'s refusals each carry their code on the `process-exit` it writes under a parsed `VIOLA_NAME` (`src/cmd/verify.rs` `Ended::Refused`): `batch-script-child` · `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside `interrupted`.
@carried n=2 run=2 label=architecture/conventions fact=fct-vuikun3q
quote: `the claude CLI is a .cmd or .bat script` / `pass the real executable, not a .cmd or .bat shim`
@carried n=3 label=architecture/conventions fact=fct-fw4invxz
why: the macOS carve-out of `server-verify-failed` must lose its code with the macOS arms, and the plan must list this text as naming code that no longer exists.
quote: An exception: `server-verify-failed` is claimed on Linux, Windows and macOS, where on macOS it means an euid mismatch alone; macOS compares no peer pid, `v1-18`'s named carve-out, owned by no route entry.
@carried n=4 label=architecture/conventions fact=fct-7s455w3m
why: the client's server check must keep its Linux euid and pid/start-time halves, the euid half's Unix gate made unconditional with no seam forcing a refusing decision, while the Windows half leaves.
quote: a server that is not the recorded wrapper (on Unix a connected peer whose euid is not this user's or cannot be read, `peer_admitted`; on Linux and Windows the connected peer's pid or OS start time not the snapshot's, or that check not made): no frame is written
@carried n=4 run=2 label=architecture/conventions fact=fct-gvna3tj2
quote: (on Unix its peer euid not this user's or unreadable, `peer_admitted`; else its pid or OS start time not the snapshot's, or the check cannot be made; either reads `UnreachableCause::ServerVerify`; the start time read on `tokio::task::spawn_blocking`)
@carried n=5 label=architecture/conventions fact=fct-jpo3rq7m
why: exit 21's closed cause set must stay as it is on Linux once the Unix-gated checks that produce `ServerVerify` and `SocketDir` lose their gates.
quote: `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir`, each word through `as_str`, `SocketDir`'s `socket-dir-refused`) is exit 21's closed cause set.
@carried n=6 label=architecture/conventions fact=fct-2y3oelpa
why: the socket-directory refusal is a Linux socket rule the chunk re-asserts, and it must keep its exits and words on every path when its Unix gate is made unconditional.
quote: Exit `1` also covers `viola run` and `viola revive`'s start refusing the socket directory (`socket_dir_ready` in `src/cmd/run.rs`, before any snapshot, child or `Server::bind`)
@carried n=6 run=2 label=architecture/conventions fact=fct-wm75dacy
quote: Exit `21`'s refused socket directory: on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, `src/cmd/client.rs` `connect`'s refused check is `CallError::SocketDirRefused` to `Unreachable::SocketDir`, writing one `process-exit` at WARN with `exit_code` 21, `detail` `socket-dir-refused` and `during` `connect`, and no `channel-request`.
@carried n=6 run=3 label=architecture/conventions fact=fct-qfrfkv27
quote: A frame tool (`send`, `wait`, `last`, `answer`) meeting a refused socket directory: MCP's `frame`, after `live_endpoint`, runs `check_endpoint_dir` and answers `result::unreachable(UnreachableCause::SocketDir)`
@carried n=7 label=architecture/conventions fact=fct-czy74f6t
why: the strict-modes refusals must keep their exits, codes and no-log behaviour on Linux while each check's Windows half leaves and its Unix half loses its gate.
quote: Exit `1` also covers `viola run`'s start and `viola revive`'s start arm refusing the home tree, checked before the wrapper log opens (`checked_exe`, then `refuse_tree`, in `src/cmd/run.rs`)
@carried n=7 run=2 label=architecture/conventions fact=fct-zvpgaziz
quote: Exit `1` also covers `viola verify` refusing the home tree (`check_home_tree` with no name and no key: the home, `ledger/`, `bin/` and `plugin/`, each that exists), checked before its `cli` log opens
@carried n=7 run=3 label=architecture/conventions fact=fct-u4uyh6oz
quote: Exit `21` also covers the refused strict-modes check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`, the verb's first step, after its text read for `send`)
@carried n=7 run=4 label=architecture/conventions fact=fct-pfqpxroc
quote: A frame tool (`send`, `wait`, `last`, `answer`) first runs `viola_state::strict::check_instance` on the home and the target instance, before `live_endpoint`: a refusal returns the `instance-unreachable` `isError` result with `detail: "strict-modes-failed"`, text `refused: instance-unreachable`, and writes no frame and no line.
@binding n=1
**Exit codes (conventions) ↔ Hook contract and CI/CD (body)**: the typed set `0, 1, 2, 10–14, 20, 21` and hook's always-exit-0 must hold unchanged on Linux after the removal (architecture/conventions fct-vuikun3q) (architecture fct-nppkdemz), and the `batch-script-child` refusal both name is the CI/CD decision's `.cmd`/`.bat` rule the plan must list and decide (architecture fct-bvzxxcy7).
@binding n=2
**Server-verify cause (conventions) ↔ Instance snapshot contract and trust boundary (body)**: the Linux pid/start-time server check against the snapshot's `pid` and `started_at` must stay while the macOS carve-out and the Windows half lose their code (architecture/conventions fct-7s455w3m) (architecture/conventions fct-fw4invxz) (architecture fct-ld365mzq), and the euid check made unconditional must keep admitting only the owning user with no seam forcing a refusing decision (architecture fct-ao5mmzh3).
@binding n=3
**Socket-directory refusal (conventions) ↔ IPC endpoints (Occupied Resources)**: the `socket-dir-refused` exits on start, CLI verbs and MCP frame tools must follow the per-user 0700 socket directory rule unchanged once its Unix gate is made unconditional (architecture/conventions fct-2y3oelpa) (architecture/conventions fct-wm75dacy) (architecture fct-wular7qm).
@binding n=4
**Strict-modes refusals (conventions) ↔ Filesystem rows (Occupied Resources)**: the home-tree and instance checks behind exit 1 and exit 21 must keep their Unix half (no group or other bit) as their only rule, the Windows DACL half leaving with the code (architecture/conventions fct-czy74f6t) (architecture/conventions fct-u4uyh6oz) (architecture fct-cicfj3bq).
