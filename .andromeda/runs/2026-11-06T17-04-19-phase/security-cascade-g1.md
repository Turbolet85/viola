@carried n=1 label=security-plan/threat-model-summary fact=fct-33o4e7zi
why: Measurement B (one tree keying two `bin/<key>/` folders) must be recorded against the pinned executable's content-hash-keyed folder and acted on only by the founder's word.
quote: `bin/<version>-<hash>/viola` is the pinned executable.
@carried n=1 run=2 label=security-plan/threat-model-summary fact=fct-feql22h5
quote: The pinned binary and plugin folders are never deleted in v1.
@carried n=2 label=security-plan/threat-model-summary fact=fct-jy2pv6x7
why: The chunk's live `viola run` sessions must keep every `CLAUDE*` parent-identity value stripped from the wrapped child, and stand on the directive's own word, never on the earlier dev-host crossings.
quote: The `viola run` wrapper process's environment holds `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and other `CLAUDE*` parent-identity variables, which are stripped from the child's environment (R8 strip list, Occupied Resources: Environment variables).
@carried n=2 run=2 label=security-plan/threat-model-summary fact=fct-jy2pv6x7
quote: neither answer covers a second message, a verify child or any other probe.
@carried n=3 label=security-plan/threat-model-summary fact=fct-t7qeqkiu
why: The founder's word on the socket-directory pair's wording is recorded here, and the IPC boundary text must agree with it.
quote: One endpoint per `viola run`: socket file `<dir>/viola-<h12>.sock`, `<dir>` the per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`) (§ Authentication & Authorization, IPC access control (Unix)).
@carried n=3 run=2 label=security-plan fact=fct-2jp5ovui
quote: Per-instance Unix domain sockets in a verified per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root; no Linux abstract namespace.
@carried n=4 label=security-plan/threat-model-summary fact=fct-t7qeqkiu
why: The live session's driver must reach the wrapped builder only through the channel's existing peer and server checks, and the always-admit `peer_admitted` survivor stays untested by construction with no seam added.
quote: and the peer euid decision past the connect (a refusal `server-verify-failed`, no frame).
@carried n=4 run=2 label=security-plan fact=fct-qtqta3hw
quote: v1 intends to trust every process of the same OS user and no other; there is no per-connection authentication, the self-reported `from` authenticates nothing, and the endpoint name is predictable.
@carried n=5 label=security-plan/threat-model-summary fact=fct-t7qeqkiu
why: v1-13 and v1-36's link clauses are deferred or re-worded with no new channel method, so the `link`/`unlink` methods keep their stated effect.
quote: Whoever can connect can also add or remove a driver link (`link` / `unlink`), whose effect is a `link` / `unlink` line in `events.ndjson` and the snapshot's links only; neither method reads the self-reported `from`.
@carried n=6 label=security-plan/threat-model-summary fact=fct-zyxi3lxw
why: The wheel hypothesis (`CSI ? 6 n`, kitty graphics) may join the closed non-editing list only by a live measurement of its exact grammar on the founder's word.
quote: The stdin classifier's closed non-editing list (the founder's ruling F-W2): only a sequence on it never counts as typing, and every other byte does.
@carried n=6 run=2 label=security-plan/threat-model-summary fact=fct-zyxi3lxw
quote: a reply no terminal has been measured to write is never added on a guess.
@carried n=6 run=3 label=security-plan fact=fct-hscavt6w
quote: only a sequence on the stdin classifier's closed non-editing list never counts as typing, and a reply no terminal has been measured to write is never added on a guess.
@carried n=7 label=security-plan/threat-model-summary fact=fct-bur5z45t
why: v1-13's MCP result for `link` is disposed by deferral or re-wording, never by adding `link`, `unlink` or `release` to the MCP tool list.
quote: Leaving `release`, `pause`, `link` and `unlink` off the MCP tool list is described as an affordance, not an enforcement.
@carried n=7 run=2 label=security-plan fact=fct-kfezk4o5
quote: leaving `release`, `pause`, `link` and `unlink` off the tool list is an affordance, not an enforcement.
@carried n=8 label=security-plan/threat-model-summary fact=fct-qxr7dgpn
why: The founder's word on the board's read rule is recorded here, and the boundary-cli text stating that rule must agree with it.
quote: `viola list`'s board scan holds the home to the read rule (`check_read`) and `instances/` to `check_path`, so a home another user may read is refused whole, with or without an instance;
@carried n=8 run=2 label=security-plan/threat-model-summary fact=fct-qxr7dgpn
quote: `viola run <name> -- <program> <args>` spawns the program as given.
@carried n=9 label=security-plan/threat-model-summary fact=fct-uemrwfo5
why: The live sessions resolve the real `claude` from the calling PATH as an accepted same-user risk, and the portable-pty bump must leave `run`'s own child resolution and version probe intact.
quote: **Trust boundary:** Whatever the calling process's PATH resolves to.
@carried n=9 run=2 label=security-plan/threat-model-summary fact=fct-uemrwfo5
quote: `run` resolves the child executable and runs it once with `--version`.
@carried n=9 run=3 label=security-plan fact=fct-m3uflr5q
quote: the trust boundary is whatever the calling process's PATH resolves to; output parsing is tolerant and failure becomes `unknown`
@carried n=10 label=security-plan/threat-model-summary fact=fct-dvr22577
why: The portable-pty move changes the root `Cargo.lock` graph, which must pass every listed supply-chain check with no ignore-list widening.
quote: **Trust boundary:** `cargo deny check` over `deny.toml` (advisories, licences, sources, C-crate / telemetry-crate / feature bans), the sole-root `deny-sync.toml` tokio ban per sync crate and `viola-channel`'s `cargo tree` form (`scripts/channel-tokio-free.sh`; its Tokio client behind the `tokio` feature), `Cargo.lock`, and `publish = false`.
@carried n=10 run=2 label=security-plan/threat-model-summary fact=fct-dvr22577
quote: and its lockfile gets its own advisory and source audit (see §Dependency Security).
@carried n=10 run=3 label=security-plan fact=fct-tm2e2x5g
quote: the trust boundary is `cargo deny check`, the sync-crate `tokio` bans, `Cargo.lock` and `publish = false`, the test-only `fuzz/` lockfile audited on its own by a ratified exemption
@carried n=11 label=security-plan/threat-model-summary fact=fct-v2lh3e2i
why: The chunk reads CI's one-leg ubuntu jobs green at wrap, the supply-chain and release jobs included, and the Windows remnants removed must leave this Linux-only CI line true.
quote: GitHub Actions `ci.yml`, triggered on push and PR, on the one-leg matrix `[ubuntu-latest]` (every job and artifact name keeps its `${{ matrix.os }}` form); plus `nightly.yml` (weekly `schedule` + `workflow_dispatch`) for `cargo deny check advisories` over the root and fuzz lockfiles, and a time-boxed fuzz run.
@carried n=11 run=2 label=security-plan/threat-model-summary fact=fct-v2lh3e2i
quote: the ubuntu supply-chain job (`cargo deny check`, the `fuzz/Cargo.lock` advisories and sources audit, the sole-root tokio ban then `scripts/channel-tokio-free.sh`, the ban probes, zizmor)
@carried n=11 run=3 label=security-plan/threat-model-summary fact=fct-v2lh3e2i
quote: the release build (`cargo build --release --locked --bin viola` through `scripts/release-check.sh`, which refuses any test-only binary).
@carried n=11 run=4 label=security-plan/threat-model-summary fact=fct-v2lh3e2i
quote: The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin, which holds `src/conpty.rs` and `.github/workflows/windows-mutants.yml`.
@carried n=11 run=5 label=security-plan fact=fct-ju2d5wlo
quote: GitHub Actions `ci.yml` on push and PR over the one-leg `[ubuntu-latest]` matrix, plus the weekly `nightly.yml` advisory and fuzz run
@carried n=12 label=security-plan fact=fct-kjrxk2yb
why: Every gate green includes CI's lint-job rmcp features step and the `deny.toml` rmcp feature bans, which must still hold over the new dependency graph.
quote: rmcp features exactly `server` and `transport-io` in `viola`'s release graph, checked by `scripts/rmcp-features.sh` in CI's lint job (rmcp must be present; the features a non-rmcp package requests must be exactly those two).
@carried n=12 run=2 label=security-plan fact=fct-kjrxk2yb
quote: `transport-streamable-http-server` and `auth` are banned in `deny.toml`.
@carried n=13 label=security-plan fact=fct-2zt7gsrv
why: A cap whose acceptance names a frontend surface is not provable in 0.1.0 and reads deferred only on the founder's word.
quote: The frontend is deferred to 0.2.0; its technology is not chosen.
@carried n=14 label=security-plan fact=fct-y7k647gj
why: The chunk's security-plan re-wordings (the handle-inheritance line, the CI and supply-chain lines) must land only through the directive's store whole-fact form, never as a hand edit of the plan during the phase loop.
quote: Do not edit during phase loop runs (`/andromeda-phase` / `/andromeda-implement`).
@carried n=15 label=security-plan fact=fct-lnwynz4l
why: Each security-plan re-wording this chunk makes must be recorded in the plan's live amendment history.
quote: History: security-plan-amendments.md (live); the log moved verbatim to security-plan-amendments-archive.md (U35).
@binding n=1
**threat-model-summary supply chain ↔ dependency-security**: the portable-pty move off `=0.8.1` must keep the supply-chain trust boundary (security-plan/threat-model-summary fct-dvr22577) green with no new ignore, re-check the one expected `RUSTSEC-2017-0008` ignore whose reason names `portable-pty =0.8.1` (security-plan fct-serp72ds), re-word the pinning line that names `=0.8.1` (security-plan fct-uydjlfhj), and take the version by bump or the named portable-pty-psmux 0.9.7 path (security-plan fct-4r3p2tgk).
@binding n=2
**threat-model-summary IPC boundary ↔ data-protection handle inheritance and bans**: the Linux `/proc/<child pid>/fd` measurement must find no channel socket or listener of the endpoint (security-plan/threat-model-summary fct-t7qeqkiu) in the wrapped child, then re-word the handle-inheritance line off «recorded, not established» (security-plan fct-p7rzex2i); any portable-pty move keeps close-on-exec, channel handles never inheritable (security-plan fct-iftwwapx).
@binding n=3
**threat-model-summary IPC boundary ↔ authentication-authorization**: the founder's word on the socket-directory pair (security-plan/threat-model-summary fct-t7qeqkiu, security-plan fct-2jp5ovui) must leave the IPC access-control row's wording (security-plan fct-dthgjfbs) and the client server-verification row (security-plan fct-phspeqis) agreeing with it; the `peer_admitted` survivor gets no new seam.
@binding n=4
**threat-model-summary MCP and IPC boundaries ↔ security-anti-patterns universal**: v1-13 and v1-36 are deferred or re-worded with `link`/`unlink`/`release` off the MCP tool list (security-plan/threat-model-summary fct-bur5z45t) and no new channel method or listener, which would need its own Decisions Log entry (security-plan fct-yfqtfw75).
@binding n=5
**threat-model-summary CI/CD ↔ dependency-security CI integration**: CI's one-leg ubuntu jobs (security-plan/threat-model-summary fct-v2lh3e2i) read green at wrap means Job 4's four-family `cargo deny check` and fuzz lockfile step (security-plan fct-mi73d5qc), the release check's 5/5 probe and test-only feature refusal (security-plan fct-h3byb6pq) and the single G2 exemption (security-plan fct-wsxv34pw) all pass.
@binding n=6
**threat-model-summary child spawn ↔ input-validation child executable resolution**: the bumped portable-pty must still be handed an absolute child path `viola run` resolved itself (security-plan/threat-model-summary fct-uemrwfo5, security-plan fct-sgmmo3ki), and the vt100 feed stays under `catch_unwind` (security-plan fct-iwuq6h35).
@binding n=7
**threat-model-summary config and filesystem ↔ data-protection pinned-binary bans**: Measurement B's second folder for one tree (security-plan/threat-model-summary fct-33o4e7zi, security-plan/threat-model-summary fct-feql22h5) is recorded only; the pinned binary stays SHA-256 re-hashed against its `<hash>` before reuse (security-plan fct-gmz2hnmb), and acting on it is the founder's.
@binding n=8
**threat-model-summary terminal boundary ↔ the body's terminal vector**: the wheel list is widened only by a live-measured exact grammar on the founder's word (security-plan/threat-model-summary fct-zyxi3lxw, security-plan fct-hscavt6w); the directive's headless rig sessions with no human key make no such widening by themselves.
@binding n=9
**threat-model-summary CLI boundary ↔ bootstrap-phases logging-redaction-wire**: the founder's word on the board's read rule (security-plan/threat-model-summary fct-qxr7dgpn) also covers unwrapped rows printed through `escape_row` (security-plan fct-auxwpgtg), the two lines agreeing after it.
@binding n=10
**the plan's header ↔ the operator directive's store form**: every security-plan re-wording this chunk makes lands only through the store's whole-fact form, never by hand during the phase loop (security-plan fct-y7k647gj), with its amendment recorded in the live history (security-plan fct-lnwynz4l).
