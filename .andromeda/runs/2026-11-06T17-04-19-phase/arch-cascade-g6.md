@carried n=1 label=architecture/occupied-resources fact=fct-nm6raycf
why: The channel-handle no-leak measurement must look for exactly this socket and its listener in the wrapped child's `/proc/<child pid>/fd`, and the refused socket directory's wording is owed the founder's word in this chunk.
quote: - **Linux:** Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (security-plan arch amendment 2): `$XDG_RUNTIME_DIR/viola/`, and `/tmp/viola-<uid>/` only as a fallback; never the `/tmp` root. `run` and `revive`'s start check that directory before any snapshot, child or `Server::bind`, creating it only when absent and checking it again, and the CLI frame verbs, `hook` and `mcp`'s frame tools check the recorded `endpoint`'s directory before they connect; a symlink, a non-directory, a foreign owner or any group or other bit is refused (`socket-dir-refused`) and left as found, the check never changing its mode. The socket is chmod 0600 after the bind.
@carried n=1 run=2 label=architecture fact=fct-wular7qm
quote: - **Linux:** Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root, checked before the bind and before every client's connect and, when refused (`socket-dir-refused`), left as found, never narrowed; the socket is chmod 0600 after the bind; every other process connects to the `endpoint` recorded in the instance snapshot, and both sides admit only a peer of the owning user's euid, the wrapper dropping another before any frame.
@carried n=2 label=architecture/occupied-resources fact=fct-uqyfijk4
why: The mutation audit's survivor is an always-admit `peer_admitted` read at these client seams; it must stay a dated note with no seam added, the euid check kept as stated.
quote: and then the connected peer's euid (`viola_channel::peer_admitted` over `Client::server_euid` / `TokioClient::server_euid`), a foreign euid or one it cannot read reading `server-verify-failed`.
@carried n=3 label=architecture/occupied-resources fact=fct-bzckmn3n
why: `v1-53`'s live run confirms the fake agent's Unix behaviour and the no-leak measurement runs under it, while the re-run release check must keep every test-only binary and feature out of a release build.
quote: - Test-only binaries, never in a release build: `viola-fake-agent` (a `[[bin]]` of the root package behind feature `fake-agent`, the stand-in `claude`;
@carried n=3 run=2 label=architecture fact=fct-4whhx2hw
quote: It is enabled ONLY by the root package's `[dev-dependencies]` entry (`viola-channel = { path = "crates/viola-channel", features = ["test-support"] }`), so no product build carries it, and `scripts/release-check.sh` refuses a release artifact built with it.
@carried n=4 label=architecture/occupied-resources fact=fct-mnor3ks3
why: `v1-13`'s disposition must rest on `release` (incl. `--budget`) and `link` being CLI subcommands outside the five tools `mcp` serves, and the chunk adds no verb.
quote: - Subcommands: `run`, `revive`, `send`, `wait`, `last`, `list`, `answer`, `hook <event>`, `mcp`, `verify`, `pause`, `release` (incl. `--budget`), `link`, `unlink`, `plugin install`. `mcp` (chunk 2026-10-17-mcp-server-for-drivers) is hidden from `viola --help` (`#[command(hide = true)]`: run by the plugin, never by a person) and takes no argument but the global `--home`; it serves the five tools over stdio
@carried n=4 run=2 label=architecture fact=fct-frmh2fi2
quote: MCP tools `send`, `wait`, `last`, `answer`, `list`, which reach a session as `mcp__plugin_viola_viola__<tool>`
@carried n=5 label=architecture/occupied-resources fact=fct-scvtx5bh
why: `v1-13`'s MCP result for `link` and `v1-36`'s link set must be deferred or re-worded against this CLI-only `link` result, its payload unchanged.
quote: - `link` and `unlink` each take `<driver> <driven> [--json]` (one shared `LinkArgs`), the verb's instance being the driven one; each exits 0 with its one result line, or `{"v":1,"ok":<ok>}` under `--json` (`link`: `{since, already_linked}`; `unlink`: `{}`),
@carried n=6 label=architecture/occupied-resources fact=fct-l6ysfimx
why: The socket-directory pair's wording is one of the four PROVISIONAL items whose founder word this chunk must bring, and the live session's `send` / `wait` / `answer` keep these exits.
quote: They exit 21 the same way, past the instance check, on a refused socket directory (`--json`: the `instance-unreachable` document with `detail` `socket-dir-refused`; human: `send_hint` cause `socket-dir`, `hint:` last), and with `detail` `server-verify-failed` on a server of another euid.
@carried n=6 run=2 label=architecture/occupied-resources fact=fct-6oxorhgk
quote: `socket-dir-refused` is a word of it and of `hook-decision`'s `detail` enum.
@carried n=7 label=architecture/occupied-resources fact=fct-7uan73fu
why: The live session in which a driver drives a wrapped builder must start both in one headless `--home`, so the driver's nested `send` / `wait` / `answer` reach the builder through that home.
quote: Each derives the viola home as the grandparent of `VIOLA_DIR`, so a session started with `--home` keeps its hooks, MCP server and nested CLI calls in that home.
@carried n=8 label=architecture/occupied-resources fact=fct-5lqu6sil
why: The chunk's live sessions must run only locally and never under `CI`, and its edit of `crates/viola-e2e/src/harness/session.rs` must keep the session guard's keep-home reads.
quote: `AGENT_RUN_KEEP_HOMES`: `1` keeps `target/e2e-home/` homes for the CI gates. Read by `viola-harness`, by the root test chain (`tests/support/home.rs`) and by viola-e2e's `harness::session` guard (`HarnessSession`'s `Drop`, through `keep_home`), which keeps the session's home under `1`.
  - `AGENT_RUN_KEEP_FAILED`: `1` keeps a failing test's home for post-mortem. Read by the root test chain (`tests/support/home.rs` `keep_decision`), by viola-e2e's `tests/harness_lifecycle.rs` `Booted` guard, which keeps a booted session's home only when the value is `1` and the thread is panicking, and by viola-e2e's `harness::session` guard, whose `Drop` (after its `cleanup`) keeps the home when `AGENT_RUN_KEEP_HOMES=1`, or when this value is `1` and the thread is panicking (chunk 2026-10-17-end-to-end-path-binary-and-harness-legs). `run --mutants` sets it to `0` on the `cargo mutants` command.
  - `CI`: read by `viola-harness` for presence only (`std::env::var_os("CI").is_some()` in the bin and in `run::run`, passed to `run_with` as `ci`). Its one effect: `run --local-live` under it returns `{"v":1,"cmd":"run","ok":false,"reason":"live-in-ci"}`, exit 2, before any build, spawn or suite, so the real `claude` never runs in CI.
@carried n=8 run=2 label=architecture fact=fct-rw5zqusn
quote: - Test-only, read by no `viola` build: `MCP_SERVE_TEST_CHILD`, a marker `viola-mcp`'s unit tests set by `Command::env` on the child they spawn, read only by that crate's `#[cfg(test)]` child entry `serve_child_entry`; outside the `VIOLA_*` namespace, which is product plumbing.
@carried n=8 run=3 label=architecture fact=fct-e4sufzw6
quote: - `target/e2e-home/viola-session-*/home`, `target/e2e-home/viola-test-*/home` and `target/e2e-home/viola-live-<pid>/home`: every harness and test home but the one `viola-chaos-*` home, gitignored and kept in CI for the obs gates;
@carried n=8 run=4 label=architecture fact=fct-ziwerfk3
quote: - `target/nextest/ci/junit.xml`: the nextest `ci` profile's JUnit, gitignored. The `junit-<os>` upload (behind the secret scan) carries it alone.
@carried n=9 label=architecture/occupied-resources fact=fct-3jytrhfq
why: The live dialog hold near the ceiling must set `dialog_hold_s` in the session home's `config.json` within 60–3600 before the wrapper starts, since the wrapper reads it once at its start.
quote: `dialog_hold_s`: the dialog hold, a JSON whole number of seconds from 60 to 3600, default 60, read by the wrapper once at its start through `run::dialog::read_hold` right after the thresholds, any other value taking 60 with one `parse-rejected{parser:"config-json", detail:"malformed"}` WARN line, so no value makes the hold zero or unbounded).
@carried n=9 run=2 label=architecture fact=fct-i4wacyrp
quote: and `dialog_hold_s` (60–3600 s, default 60);
@carried n=9 run=3 label=architecture fact=fct-h57b2ldw
quote: Registered today: all nine (PreToolUse and PermissionRequest since chunk 2026-10-04-dialog-answers-by-dialog-id, `timeout` 3615).
@carried n=10 label=architecture/occupied-resources fact=fct-6u55z4cp
why: Measurement B (one tree building two files) keys two pinned copies, a founder item, and `PinError` is one of the divergences that must take a fold or a ratification.
quote: `Present` (a concurrent first start placed the copy first) re-hashes the copy that stands: equal bytes are reused, and different bytes are `PinError::HashMismatch`, the file left as found and the start refused `pinned-hash-mismatch` (exit 1).
@carried n=10 run=2 label=architecture fact=fct-hnee7xdr
quote: where `<hash>` is the first 16 hex of the SHA-256 of the exe's content;
@carried n=11 label=architecture/occupied-resources fact=fct-6oxorhgk
why: The move of `portable-pty` off `=0.8.1` must keep a signalled child's exit code reported, so a stopped interactive run still closes its pair with a `child_exit_status`, and the move's verdict is read in the `supply-chain` artifact.
quote: a child of an interactive PTY run killed at a Ctrl-C or TERM stop closes its pair with `duration_ms` and a `child_exit_status`, the PTY pump's exit code, since portable-pty reports a code for a signalled child, as measured at chunk 2026-10-14-resume-and-compact-ledger-rows by its resume-run stop case;
@carried n=11 run=2 label=architecture fact=fct-gwc5vubx
quote: - `target/supply-chain/`: `deny.json` (the root graph), `deny-fuzz.json` (the `fuzz/Cargo.lock` advisories and sources audit) and `zizmor.json` from the CI `supply-chain` job, uploaded as artifact `supply-chain` (7 days, admissible by content: obs-plan §8 item 6), gitignored.
@binding n=1
**res-filesystem (`diagnostics/` signalled-child code) ↔ the PTY decision and the supply-chain artifact**: the release `portable-pty` moves to must keep reporting a code for a signalled child (architecture/occupied-resources fct-6oxorhgk), beside the PTY decision's `openpty` backend and process-handle exit (architecture fct-omwrdnlc), and its graph must read RUSTSEC-2026-0441 green in `target/supply-chain/deny.json` with no ignore widening (architecture fct-gwc5vubx).
@binding n=2
**res-env-vars (test-only marker row) ↔ scope item 3**: `PTY_SIDELOAD_TEST_CASE`, re-measured at HEAD, must either take a row beside `MCP_SERVE_TEST_CHILD` in Environment variables or be stated as taking none (architecture fct-rw5zqusn), outside the `VIOLA_*` namespace and read by no `viola` build.
@binding n=3
**res-filesystem (`config.json` `dialog_hold_s`) ↔ the Hook Contract decision and the registered hook timeout**: the live hold near the 3600 s ceiling is set through `config.json` (architecture/occupied-resources fct-3jytrhfq) and must stay inside the PreToolUse / PermissionRequest `timeout` 3615 (architecture fct-h57b2ldw) before the ceiling's PROVISIONAL mark in the Hook Contract (architecture fct-4nt6qtyp) is lifted.
@binding n=4
**res-ipc-endpoints (the Linux socket) ↔ the security-plan no-leak line**: the wrapped child's `/proc/<child pid>/fd` must hold neither the `viola-<h12>.sock` socket nor its listener (architecture/occupied-resources fct-nm6raycf) under the fake agent (architecture/occupied-resources fct-bzckmn3n) before security-plan's «recorded, not established» line is re-worded.
@binding n=5
**res-binaries (pinned copy) ↔ the Deployment / Distribution and Error Handling decisions**: Measurement B's two-copies-per-tree result stays a founder item against the content-hash key (architecture/occupied-resources fct-6u55z4cp, architecture fct-c3wssf3l), and `PinError` takes its fold or ratification in the one-enum-per-crate rule (architecture fct-sha7o35z).
@binding n=6
**res-binaries (subcommands, `link`) ↔ the MCP decision ↔ the matrix's `v1-13` / `v1-36`**: `link` and `release --budget` are CLI-only (architecture/occupied-resources fct-mnor3ks3, architecture/occupied-resources fct-scvtx5bh, architecture fct-tkwxvt3j), so each clause naming their MCP result or a board link set must be deferred with its reason or re-worded, on the founder's word, no verb or tool added.
@binding n=7
**res-ipc-endpoints (peer euid check) ↔ test-plan §1's untestable zone and matrix#v1-44**: the always-admit `peer_admitted` survivor (architecture/occupied-resources fct-uqyfijk4) must stay a dated note, no seam added to make it killable.
@binding n=8
**res-env-vars (`CI`, the live homes) ↔ the CI/CD decision**: both live sessions run locally in `target/e2e-home/viola-live-<pid>/home` homes (architecture fct-e4sufzw6), never under `CI` (architecture/occupied-resources fct-5lqu6sil, architecture fct-bvzxxcy7), driver and builder in one `--home` (architecture/occupied-resources fct-7uan73fu), and `v1-53`'s claim reads the `ubuntu-latest` JUnit alone (architecture fct-ziwerfk3).
