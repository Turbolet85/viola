@carried n=1 label=architecture/stack-and-technologies fact=fct-kxiazeyf
why: The chunk moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`, so the PTY row's pin text must follow the move while the `pty` seam stays, `pty_backend()` still reads `openpty` and libc's `=0.2.189` pin stays compatible with the new graph.
quote: | PTY layer | portable-pty `=0.8.1` (pinned) behind viola's own `pty` seam; libc `=0.2.189` for the host tty's raw mode, behind `HostTerminal`; the host's input is `std::io::stdin()` (`viola_pty::host_stdin()`) and its size the `TIOCGWINSZ` read (`viola_pty::host_size()`) | Hosts the unmodified `claude` in openpty (`pty_backend()` always `openpty`) |
@carried n=1 run=2 label=architecture fact=fct-zegjs5rw
quote: portable-pty `=0.8.1` behind viola's own `pty` seam
@carried n=2 label=architecture/stack-and-technologies fact=fct-7z3oulbd
why: The new `portable-pty` graph must bring no async runtime into the sync crates, so Tokio stays built only inside `mcp` and the tokio-free checks the chunk re-runs stay green.
quote: Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.3 built only inside `mcp` | No async runtime on the hot `hook` path.
@carried n=2 run=2 label=architecture fact=fct-xclilru3
quote: A sync std-thread core; Tokio 1.53.3 only in `mcp`
@carried n=3 label=architecture/stack-and-technologies fact=fct-nuruygpl
why: The moved `portable-pty` and every crate its graph brings must build at the declared 1.96 floor so the `msrv` job, one of CI's seven, stays green without the floor or the toolchain pin moving.
quote: workspace `rust-version = "1.96"`, edition 2024 | One native binary `viola`. 1.96 is the declared floor (the security toolchain floor; sysinfo 0.39.6 needs 1.95, std `File::lock` 1.89, rmcp 1.88). CI checks the floor in the `msrv` job: 1.96 is installed by `rustup toolchain install 1.96` and selected with `RUSTUP_TOOLCHAIN=1.96`, which outranks `rust-toolchain.toml`, so the pin is untouched.
@carried n=4 label=architecture/stack-and-technologies fact=fct-iw73rjlh
why: The `Cargo.lock` move under the new `portable-pty` graph and the Windows-remnant removal must keep libc the one OS-binding crate, with no viola crate depending on windows-sys.
quote: | OS bindings | libc, a plain (not target-gated) `[dependencies]` entry of the root, `viola-channel`, `viola-pty`, `viola-state` and `viola-e2e`; no crate depends on windows-sys | The one OS-binding crate of the Linux-only build |
@carried n=5 label=architecture fact=fct-igbvtfv6
why: Each Windows remnant item 17 names is removed or kept with its reason under this rule, the build and tests staying Linux-only with the removed code recoverable only at the tag.
quote: **Linux only.** viola targets Linux: the core on any current distribution, built and tested on every commit in CI on `ubuntu-latest` against the fake agent, and the window features of 0.2.0 on Hyprland; the Windows and macOS code is out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).
@carried n=6 label=architecture/stack-and-technologies fact=fct-jsy3xdhu
why: The channel-handle no-leak measurement reads the wrapped child's fds for this Unix socket and its listener, and the socket-directory pair's wording owed the founder's word is this directory check's refusal.
quote: libc gives the uid for the per-user socket directory and the socket is chmod 0600 after the bind; the directory itself is read by `viola_state::strict::check_endpoint_dir` (the recorded endpoint's parent, through `check_socket_dir`, one std `symlink_metadata`) before a bind (`viola run`, `revive`'s start) and before each client's connect (the CLI frame verbs, `hook`, MCP's `frame`), a symlink, a non-directory, another owner or a group or other bit refused and never chmodded, an absent path passing | One endpoint per `viola run`: a Unix domain socket |
@carried n=7 label=architecture/stack-and-technologies fact=fct-vp2lxvsd
why: The mutation survivor the chunk carries is an always-admit read of this euid at the three client seams, which stays a dated note with no seam added.
quote: Both clients also read the serving process's euid from the same creds, `peer_creds().ok()?.euid()` (`Client::server_euid`, `TokioClient::server_euid`).
@carried n=8 label=architecture/stack-and-technologies fact=fct-itjxddea
why: Measurement B, the founder item this chunk carries, is one tree keying two pinned copies through this 16-hex content-hash key.
quote: SHA-256 of the exe bytes, truncated to the first 8 bytes (16 hex), for the `bin/` and `plugin/` `<version>-<hash>` keys, and to re-hash an existing pinned copy (read in 64 KiB chunks) before every reuse.
@carried n=9 label=architecture/stack-and-technologies fact=fct-dy7nnp4c
why: The live dialog hold near the PROVISIONAL 3600 s ceiling reads the upper bound this newtype enforces.
quote: `DialogHold` (whole seconds 60–3600)
@carried n=10 label=architecture/stack-and-technologies fact=fct-r7pka372
why: The chunk folds or ratifies each open one-enum-per-crate divergence against this per-crate `thiserror` rule with `PtyError` the hand-written exception.
quote: | Error types | thiserror 2.0.20 (per crate; `viola-pty`'s `PtyError` is hand-written); anyhow 1.0.104 (bin edge only) |
@carried n=11 label=architecture fact=fct-vf75c7sp
why: The Inherited Defaults Errors line is one of the three places the derive census and the divergences' fold or ratification must be re-worded to agree with the code.
quote: Errors: thiserror 2.0.20, one enum per crate (except `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are open divergences, with `PinError`, `StampsReadError` and `Refused` the three the census adds, [Error Handling]), anyhow 1.0.104 only in the root bin; typed CLI exit codes; `viola hook` always exits 0.
@carried n=12 label=architecture/stack-and-technologies fact=fct-cnnrnbfw
why: The killed-wrapper leg the chunk carries, and the fake-agent reading standing meanwhile, are judged by the pid + start-time liveness beside the heartbeat.
quote: | Process liveness | sysinfo 0.39.6 (pid + process start time) + `claude agents --json` | Enriches the primary signal, the heartbeat file |
@carried n=13 label=architecture/stack-and-technologies fact=fct-qeiflw5v
why: v1-13's disposition rests on this five-tool set holding no `link` or `release` result, and the live driving session reaches a builder through these tools.
quote: stdio MCP server implementing spec 2026-07-28 (compatible with 2025-11-25): tools `send · wait · last · answer · list`.
@carried n=14 label=architecture/stack-and-technologies fact=fct-sb4r6c4a
why: Every gate green on Linux includes CI's seven jobs on `ubuntu-latest`, `supply-chain` among them, read green at this chunk's wrap.
quote: | CI/CD | GitHub Actions matrix `[ubuntu-latest]` (`test`, `perf`, `lint`, `release`; every job and artifact name keeps its `${{ matrix.os }}` form); toolchains installed by `rustup toolchain install`: from `rust-toolchain.toml`, from `fuzz/rust-toolchain.toml` in the fuzz jobs, and `1.96` in the `msrv` job (no toolchain action)
@carried n=14 run=2 label=architecture fact=fct-f53f3qjo
quote: CI: GitHub Actions on `ubuntu-latest` against the fake agent.
@carried n=15 label=architecture/stack-and-technologies fact=fct-lj7tqt64
why: The chunk re-runs fmt, clippy, check, `cargo deny check` with its per-sync-crate tokio ban and the release check green, and edits the mutants host reader's `windows` key and the coverage-floor comments these tools serve.
quote: rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`; ripgrep 15.2.0 (PCRE2; the obs G1/G3 gate tool, installed by `scripts/install-ripgrep.sh`); `jq` (runner-provided: G2's `scripts/g2-zero-panics.sh`, `scripts/release-check.sh` and `scripts/orphans-check.sh`, which refuse with `tool-missing: jq`); hyperfine 1.20.0 (the perf rows' timing runner behind `run --perf`, `-N --warmup 3 --runs 30`; `cargo install --locked` in the CI `perf` job and on the dev host, no lockfile resolves it, as measured on the Windows dev host at chunk 2026-09-28-hook-perf-gate `evidence/hyperfine-host.md`); jsonschema 0.57.0 (test harness `schema-check`, G4); syn `=2.0.119` (`default-features = false`; features `full`, `parsing`, `printing`, `visit`) and proc-macro2 `=1.0.107` (feature `span-locations`), direct dependencies of the test-only `viola-e2e` alone (the `#[cfg]` reader behind `run --mutants`' host exclusion, `harness::run::mutants::host`; no product crate depends on either); cargo-deny 0.20.2 (advisories, licences, sources, bans: C crates, telemetry crates, feature bans; the tokio ban via `deny-sync.toml` per sync crate); zizmor 1.30.1 (GitHub workflow linter); cargo-modules 0.27.0 (`cargo install --locked` in the `lint` job; the orphans gate per lib/bin target in CI, the dependency-graph review on demand); cargo-llvm-cov 0.9.1 with the rustup component `llvm-tools-preview` (the coverage runner behind `run --coverage`; `viola-harness gate` enforces the coverage floors)
@binding n=1
**stack PTY row ↔ established-decisions PTY decision ↔ the body's Stack line**: the `portable-pty` pin text moves as one in the Stack row (architecture/stack-and-technologies fct-kxiazeyf), the PTY decision that names this entry as the owner of the move (architecture fct-omwrdnlc) and the body's one-line stack (architecture fct-zegjs5rw), and the new release must keep child exit on the process handle and `pty_backend()` reading `openpty`, the value the obs-plan `process-start` row is brought to.
@binding n=2
**stack Tokio and code-quality rows ↔ the Concurrency KEYSTONE ↔ Occupied Resources' sync crates**: the new `portable-pty` graph under `viola-pty` must pass the sole-root `cargo deny` `tokio` ban via `deny-sync.toml` (architecture/stack-and-technologies fct-lj7tqt64) and the sync-crate check of the KEYSTONE (architecture fct-l5jpezvw), `viola-pty` being listed in `scripts/sync-crates.txt` (architecture fct-26g75mkn), with Tokio built only inside `mcp` (architecture/stack-and-technologies fct-7z3oulbd).
@binding n=3
**stack Error types row ↔ established-decisions Error Handling ↔ conventions error-types line ↔ Inherited Defaults Errors line**: each divergence's fold or ratification and the re-measured derive count are worded alike in the Error Handling decision (architecture fct-sha7o35z), the Conventions line that states nine derives (architecture fct-nhsg4k7g) and the Inherited Defaults Errors line (architecture fct-vf75c7sp), under the per-crate rule of the Stack row (architecture/stack-and-technologies fct-r7pka372).
@binding n=4
**stack Domain newtypes row ↔ established-decisions Hook Contract ↔ occupied-resources config and hook timeout**: the live hold near 3600 s reads the `DialogHold` bound (architecture/stack-and-technologies fct-dy7nnp4c), and the ceiling's PROVISIONAL mark is lifted or kept as one value across the Hook Contract decision (architecture fct-4nt6qtyp), `config.json` (architecture fct-i4wacyrp) and `schemas/config.v1.json` (architecture fct-6e4hf2ai), inside the dialog hooks' `timeout` 3615 (architecture fct-h57b2ldw).
@binding n=5
**stack Content hash row ↔ established-decisions Deployment ↔ occupied-resources pinned copy**: Measurement B is stated against the 16-hex key (architecture/stack-and-technologies fct-itjxddea), the Deployment decision that owes it to this entry (architecture fct-c3wssf3l) and the `bin/<version>-<hash>/` row (architecture fct-hnee7xdr), acting on it being the founder's.
@binding n=6
**stack Wrapper IPC rows ↔ occupied-resources Linux endpoint ↔ the security-plan no-leak wording**: the no-leak measurement under `/proc/<child pid>/fd` is taken on the Unix socket and listener of the Stack row (architecture/stack-and-technologies fct-jsy3xdhu) and the Linux endpoint row (architecture fct-wular7qm) before the security-plan wording leaves «recorded, not established»; the survivor's euid read (architecture/stack-and-technologies fct-vp2lxvsd) stays a dated note, no seam added.
@binding n=7
**stack MCP server row ↔ established-decisions MCP ↔ v1-13's disposition**: `link` and `release` (budget included) stay CLI-only (architecture fct-tkwxvt3j) outside the five tools (architecture/stack-and-technologies fct-qeiflw5v), so v1-13's MCP clause is deferred with that reason or re-worded on the founder's word.
@binding n=8
**body Linux only ↔ stack CI, OS bindings and code-quality rows ↔ cross-cutting Process and path discipline**: each item-17 remnant leaves under the Linux-only rule (architecture fct-igbvtfv6) with no `windows` or `target_os = "macos"` gate left (architecture fct-fa2klbw5), libc staying the one OS-binding crate (architecture/stack-and-technologies fct-iw73rjlh), the mutants host reader's `windows` key and the coverage floors' «per OS» comments following (architecture/stack-and-technologies fct-lj7tqt64), and CI staying `[ubuntu-latest]` alone (architecture/stack-and-technologies fct-sb4r6c4a).
@binding n=9
**stack Language / runtime row ↔ stack PTY row**: the chosen `portable-pty` release is one whose graph builds at the 1.96 floor (architecture/stack-and-technologies fct-nuruygpl), so the move off `=0.8.1` (architecture/stack-and-technologies fct-kxiazeyf) leaves the `msrv` job green with neither the floor nor `rust-toolchain.toml` moved.
