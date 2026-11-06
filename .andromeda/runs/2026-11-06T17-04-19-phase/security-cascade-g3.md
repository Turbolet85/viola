@carried n=1 label=security-plan/dependency-security fact=fct-6nwwowx3
why: The supply-chain red must clear by moving portable-pty off =0.8.1 to a release whose graph drops pin-utils, with cargo deny check advisories and CI's supply-chain job green and the advisories ignore list keeping its one entry.
quote: `cargo deny check advisories` reads red on RUSTSEC-2026-0441 alone (`pin-utils 0.1.0` unmaintained, reached through `portable-pty =0.8.1` → `nix 0.25.1` from `viola-pty`), in CI's `supply-chain` job and locally (as measured at viola-0.1.0/chunks/2026-11-05-linux-only-code-and-ci/report.md). Its owner is the CARRY on "Version done-check", whose remedy moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`; no ignore is widened, the `[advisories]` ignore list keeping its one entry.
@carried n=1 run=2 label=security-plan fact=fct-serp72ds
quote: `ignore = [{ id = "RUSTSEC-2017-0008", reason = "serial via portable-pty =0.8.1; serial ports never opened" }]`. This is the only expected ignore; re-check it whenever the portable-pty pin moves.
@carried n=1 run=3 label=security-plan fact=fct-mi73d5qc
quote: Job 4 (ubuntu) runs `cargo deny check` over all four check families, and fails the build on any advisory, yanked crate or unknown source.
@carried n=1 run=4 label=security-plan fact=fct-4r3p2tgk
quote: Within 7 days of a RustSec/GHSA advisory against a crate and feature viola enables: bump, or swap along arch's named path (portable-pty → portable-pty-psmux 0.9.7).
@carried n=2 label=security-plan/dependency-security fact=fct-zj6omeqt
why: The moved portable-pty pin must stay an exact workspace.dependencies pin with Cargo.lock committed, keep the channel handles close-on-exec, and keep PtyError's Display fixed.
quote: The `portable-pty =0.8.1` pin carries no Windows reason: portable-pty 0.9.0's Windows garbage-read bug reaches no code viola builds, and "Version done-check" measures moving off the pin.
@carried n=2 run=2 label=security-plan fact=fct-uydjlfhj
quote: `Cargo.lock` and `fuzz/Cargo.lock` are committed. `[workspace.dependencies]` pins every third-party version of the root graph (portable-pty `=0.8.1`)
@carried n=2 run=3 label=security-plan fact=fct-p7rzex2i
quote: **Handle inheritance:** channel handles never leak into the `claude` child: on Linux the property rests on close-on-exec descriptors (recorded, not established; its measurement is owed to the working-route entry "Version done-check"). Any portable-pty swap (portable-pty-psmux 0.9.7 is the named candidate) must keep this property.
@carried n=2 run=4 label=security-plan fact=fct-y26eifc2
quote: The `Display` impls on the product error enums (`thiserror`, or hand-written for `PtyError`; §Error Handling lists them) must not interpolate upstream text or absolute paths.
@carried n=3 label=security-plan/dependency-security fact=fct-qfxjrvyr
why: The new portable-pty graph must pass cargo deny's licences and sources families with the allow list exact, any exception the graph no longer reaches dropped, and any new non-listed licence crate given its own exception and Decisions Log entry.
quote: A `[[licenses.exceptions]]` entry naming a crate absent from the graph is dropped, never kept: cargo-deny warns `license-exception-not-encountered` for it, and such an exception narrows nothing and widens nothing.
@carried n=3 run=2 label=security-plan fact=fct-32d4ty7r
quote: the `allow` list stays exactly `["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]` (arch §Build system). `0BSD` is admitted only per crate, by one `[[licenses.exceptions]]` for `doctest-file`, the one 0BSD crate interprocess 2.4.4 pulls into the Linux graph (as measured at `cargo deny check licenses` reading `licenses ok`, chunk 2026-11-05-linux-only-code-and-ci). It is never added to `allow`, so any other 0BSD crate still fails the gate, and each new exception needs its own Decisions Log entry (operator ratification `2026-09-25`).
@carried n=3 run=3 label=security-plan fact=fct-l55i2g5k
quote: `[sources]`: `unknown-registry = "deny"`, `unknown-git = "deny"` (crates.io only; this also keeps CVE-2026-5222's sparse-registry class out of reach).
@carried n=4 label=security-plan/dependency-security fact=fct-o63eny7e
why: The C-building crate bans, the per-sync-crate tokio bans and the channel-tokio-free tree form must still pass over the new transitive crates, each proven live by the deny probes at wrap.
quote: The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for the one target triple `x86_64-unknown-linux-gnu` in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system). `viola-channel`, whose Tokio client (tokio `=1.53.3`, `io-util`) sits behind its `tokio` feature, left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features tokio` control one; CI supply-chain step `Tokio ban per sync crate (sole root)`). Every ban, the tree form included (probe `channel-tokio-edge` and its channel control; `14/14 banned, control clean`, counted as the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe, as measured at viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md), is proven live by `scripts/deny-probes.sh`.
@carried n=4 run=2 label=security-plan fact=fct-cavsczz7
quote: The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph in `deny-sync.toml`, run per sync crate as the sole root, `viola-channel` covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh`; every ban is proven live by `scripts/deny-probes.sh`.
@carried n=5 label=security-plan/dependency-security fact=fct-pgy5bb7v
why: deny.toml is reached whole, so its serde_json unbounded_depth feature ban and the deny-probes row proving it must still fire at wrap.
quote: `deny.toml`'s `[[bans.features]]` also bans serde_json's `unbounded_depth` feature, its reason that every parse of external bytes keeps serde_json's default recursion limit; the `scripts/deny-probes.sh` row `serde_json-unbounded-depth` proves the ban fires, and the test `serde_json_unbounded_depth_is_banned_in_deny_toml` reads it in `deny.toml`.
@carried n=5 run=2 label=security-plan fact=fct-mrnvctku
quote: `serde_json` denies `unbounded_depth` (every parse of external bytes keeps serde_json's default recursion limit), its firing proven by `scripts/deny-probes.sh`.
@carried n=6 label=security-plan/dependency-security fact=fct-i5izpzxc
why: CI's supply-chain job read green at wrap includes the fuzz lockfile's own advisories and sources audit step.
quote: The lockfile is audited separately: the ci.yml `supply-chain` step `Fuzz lockfile audit (advisories, sources)` runs `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` (into `target/supply-chain/deny-fuzz.json`), and the weekly `nightly.yml` `advisories` job runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories`.
@carried n=6 run=2 label=security-plan fact=fct-blte3oxa
quote: its lockfile is audited separately for advisories and sources, and the C-build ban does not apply to it.
@carried n=7 label=security-plan/dependency-security fact=fct-fguujmtu
why: The G2 zero-panics gate, the release check and zizmor must read green at wrap, G2 keeping its one exemption whose seam stays named only in its own file.
quote: CI's G2 zero-panics gate (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs) has exactly one exemption. It does not count a role-file `panic` line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` (the `FAKE_AGENT_HOOK_PANIC` seam's forced panic).
@carried n=7 run=2 label=security-plan/input-validation fact=fct-qyyrikqm
quote: Named in product source only in its own file; compiled out of every release build; not a `VIOLA_*` variable; configures nothing, disables no control and widens no redaction
@carried n=7 run=3 label=security-plan fact=fct-wsxv34pw
quote: CI's G2 zero-panics gate has exactly one exemption, a role-file `panic` line whose `panic_location` is exactly the `FAKE_AGENT_HOOK_PANIC` seam's, compared as a whole string, its `--probe` run before every G2 check; any further G2 exemption needs its own Decisions Log entry.
@carried n=7 run=4 label=security-plan fact=fct-h3byb6pq
quote: It refuses any executable other than `viola`, and any artifact whose `features` include a test-only feature (`test-support`, `fake-agent`): `release-check: FAILED — test-only feature {feature} in {target}`. `--probe` runs first and must read 5/5 refused with a clean control.
@carried n=7 run=5 label=security-plan fact=fct-ivw6qaje
quote: `zizmor .github/workflows/` runs on the ubuntu leg and fails on unpinned actions, `excessive-permissions`, template injection and cache poisoning.
@carried n=8 label=security-plan/bootstrap-phases fact=fct-f326yvjk
why: CI's test job read green at wrap must pass its artifact canary scan, every scan-gated upload waiting on it and no matched bytes printed or written.
quote: The CI `test` job does run obs-plan §9's artifact canary scan: `viola-harness secret-scan` over test-home diagnostics, the harness capture except `run --mutants`' `target/agent-run/chunk.diff`, and the nextest JUnit report, checked against this plan's NEVER-log floor. That one file is repository source text by construction (it trips the scan's own needles whenever it touches the scan's source), so it is skipped by exact path and the `harness-<os>` upload drops it (`!target/agent-run/chunk.diff`): it is never scanned and never uploaded, and a `chunk.diff` anywhere else under the capture is still scanned. It never prints or writes the matched bytes, and every scan-gated upload (`diag-`, `junit-`, `harness-<os>`) waits for it to pass.
@carried n=8 run=2 label=security-plan fact=fct-rwpm3oj4
quote: the CI `test` job runs obs-plan §9's artifact canary scan against this plan's NEVER-log floor, never printing or writing the matched bytes, and every scan-gated upload waits for it to pass
@carried n=9 label=security-plan/bootstrap-phases fact=fct-sawbqqxr
why: The always-admit peer_admitted survivor at the three client seams must stay untested by construction with no seam added, and the live driver session must pass through the existing directory, peer-euid and server checks.
quote: the `peer_creds` decision is `peer_admitted`, the wrapper dropping a refused peer in `accept_loop` before any frame, and the directory verification is `check_socket_dir` and `check_endpoint_dir` in `viola-state`, called at the bind and the three client seams, none inside `Client::connect`.
@carried n=9 run=2 label=security-plan/input-validation fact=fct-l7hy7pzk
quote: A passed check is followed by the liveness pre-check `live_endpoint` (the endpoint, `pid` and `started_at`), `check_endpoint_dir` on the parent of that endpoint (a refusal is exit 21, `socket-dir-refused`, no frame), the connect, the peer euid decision `peer_admitted` (a refusal reads `server-verify-failed`) and then the server check before the frame (IPC client-side server verification row).
@carried n=9 run=3 label=security-plan fact=fct-pz6zubwd
quote: Linux socket hardening (socket directory, chmod 0600, peer euid check) and client server verification in `viola-channel`
@carried n=10 label=security-plan/input-validation fact=fct-nuclnamd
why: The dialog hold near the 3600 s ceiling must be read live, never reconfigured, and no value may make the hold zero or unbounded.
quote: The dialog hold is the top-level key `dialog_hold_s`, read once at the wrapper's start through `run::dialog::read_hold` right after the thresholds: a JSON whole number of seconds that fits `DialogHold` (60–3600, `DialogHold::try_new`; default 60, `DIALOG_HOLD_DEFAULT`), else the hold takes 60 and is reported as one `parse-rejected{parser:"config-json", detail:"malformed"}` line; a document that is no object, or whose `v` is not 1, gives 60 and no rejection from this read; no value makes the hold zero or unbounded, and no env var or flag sets it (a boundary widening the founder answered himself, chunk 2026-10-19-dialog-hold-for-the-driver, `inputs#I2` Q1).
@carried n=10 run=2 label=security-plan/input-validation fact=fct-p4cpzbge
quote: the reply read is bounded at `DIALOG_HOLD_MAX + 5 s` (3605 s: the ceiling, the wrapper replying `null` at its own configured hold's end)
@carried n=10 run=3 label=security-plan fact=fct-wsshf6pn
quote: no value switches the budget gate off or makes the dialog hold zero or unbounded
@carried n=11 label=security-plan/input-validation fact=fct-q67nxnld
why: After the bump viola run must still hand portable-pty an absolute child path it resolved itself, and the live sessions take the real claude from the calling PATH only as the accepted same-user risk.
quote: This is same-user trust and an accepted risk (Decisions Log). Its output is untrusted: it is bounded by `MAX_FRAME`, and a row's `name` is never a `ViolaName` or `target` in CLI `list` or the MCP `list` tool. Human-mode CLI `list` escapes C0/C1 controls in it before printing (§ Logging). `viola run` resolves its child program itself and hands portable-pty an absolute path: a given path is used as given and a bare name is looked up on `PATH`; a program not found is `ProgramNotFound` (under `viola revive --json` the document `internal-error`, before `spawn_child`).
@carried n=11 run=2 label=security-plan fact=fct-sgmmo3ki
quote: `claude` resolved from the calling process's PATH, a same-user trust and accepted risk, its output untrusted and bounded; `viola run` resolves its child itself, a given path as given and a bare name on `PATH`, and hands portable-pty an absolute path
@carried n=12 label=security-plan/input-validation fact=fct-g433mics
why: crates/viola-pty is reached whole by the portable-pty move, so host_size must still never yield a zero size and the PTY output must still feed vt100 under catch_unwind with passthrough unblocked.
quote: Feed through `std::panic::catch_unwind`: a tee on `run`'s pump output writes the human's bytes first and hands a copy to a feed thread, which runs every feed and resize under the catch. A caught panic poisons the screen model until the host size changes, with one `parse-rejected{parser:"vt100-feed", detail:"panicked"}` line per poisoning and no screen text; a poisoned model reads `input-not-ready`, which confirmed `send` reports as `not-delivered`/`input-not-ready`, and byte passthrough to the human terminal continues. vt100 0.16.2 panics are reachable at real small sizes (as measured at chunk 2026-10-04-readiness-gate-and-timing-constants: a 24×1 screen + a wide character; 1×1 `?u`; 1×2 `abc`); `viola_pty::host_size` never yields a zero size.
@carried n=12 run=2 label=security-plan fact=fct-juszneez
quote: PTY output bytes (vt100) — fed under `catch_unwind`, the human's bytes written first; a caught panic or a dropped copy poisons the screen model, which then reads `input-not-ready`, while byte passthrough to the human terminal continues; the feed queue is bounded and a full queue never blocks the passthrough.
@carried n=13 label=security-plan/data-protection fact=fct-uxsbnoo5
why: Measurement B must be recorded as the founder's item with no cause claimed and acted on only by his word, the pinned binary still re-hashed with SHA-256 before reuse.
quote: Clean release builds of one tree can give two files whose loaded sections are equal, so one tree can key two `bin/<key>/` and `plugin/<key>/` folders; the reading stands at architecture §Established Decisions [Deployment / Distribution], no cause is claimed, and acting on it is the founder's.
@carried n=13 run=2 label=security-plan/data-protection fact=fct-zvqsbdzq
quote: Before reusing an existing `bin/<version>-<hash>/viola`, `viola run` re-hashes the file with SHA-256 and compares the truncated result with `<hash>`. On a mismatch it refuses to start (exit 1).
@carried n=13 run=3 label=security-plan fact=fct-7aw7emh6
quote: the pinned binary is re-hashed before use.
@carried n=14 label=security-plan/data-protection fact=fct-uz6mlfv5
why: Each of the chunk's at most two live sessions must rest on its own answer (the directive's founder word), never on an earlier reading's permission.
quote: It is no product path and no standing permission: another such reading needs its own answer.
@carried n=14 run=2 label=security-plan fact=fct-f5gbcl67
quote: it is no product path and no standing permission: another such reading needs its own answer.
@carried n=15 label=security-plan/input-validation fact=fct-cysd5aat
why: The founder's word on the board's read rule must be recorded against the board read as it stands, unwrapped rows never a ViolaName or target and printed through escape_row.
quote: **Identifiers:** an unwrapped row's `name` (its `sessionId`) is never a `ViolaName` and never a valid `target`; human-mode `list` prints it through `escape_row`.
@carried n=15 run=2 label=security-plan/input-validation fact=fct-wkbf5hxn
quote: `viola list`'s `viola_state::board::read`, after `board::scan` has checked every instance in name order (a replayed snapshot gives an item only for a beat at most `STALE_AFTER` old, and the read writes no file)
@binding n=1
**dependency-security (the portable-pty move) ↔ data-protection (handle inheritance)**: the release the chunk moves to must keep the channel handles close-on-exec; the `/proc/<child pid>/fd` measurement under the fake agent must be taken on the moved graph, and only that measurement re-words the Handle-inheritance line off «recorded, not established» (security-plan/dependency-security fct-6nwwowx3) (security-plan/dependency-security fct-zj6omeqt) (security-plan fct-p7rzex2i) (security-plan fct-iftwwapx).
@binding n=2
**dependency-security (the portable-pty move) ↔ input-validation (child resolution, PTY output)**: viola-pty's spawn on the new release must still take the absolute path viola run resolved, `host_size` must never yield a zero size, and the vt100 feed must stay under `catch_unwind` with passthrough never blocked (security-plan/dependency-security fct-zj6omeqt) (security-plan/input-validation fct-q67nxnld) (security-plan/input-validation fct-g433mics).
@binding n=3
**dependency-security (advisories) ↔ dependency-security (licences, sources, bans)**: the graph that drops pin-utils must pass all four cargo deny families and the deny probes, the RUSTSEC-2017-0008 ignore re-checked (its reason names `=0.8.1`) and no ignore added, a licence exception the graph no longer reaches dropped, and the pin recorded exactly in `[workspace.dependencies]` with `Cargo.lock` committed (security-plan/dependency-security fct-6nwwowx3) (security-plan fct-serp72ds) (security-plan/dependency-security fct-qfxjrvyr) (security-plan/dependency-security fct-o63eny7e) (security-plan fct-uydjlfhj).
@binding n=4
**dependency-security (the portable-pty move) ↔ bootstrap-phases (error-sanitization-wire)**: a `PtyError` change the new release forces must keep its hand-written `Display` free of upstream text and absolute paths (security-plan/dependency-security fct-zj6omeqt) (security-plan fct-y26eifc2).
@binding n=5
**input-validation (config dialog hold) ↔ input-validation (hook stdin reply bound)**: the live hold near the 3600 s ceiling is read inside the hook's 3605 s reply-read bound, under a config value inside 60–3600, never by reconfiguring the hold or setting it by env var or flag (security-plan/input-validation fct-nuclnamd) (security-plan/input-validation fct-p4cpzbge).
@binding n=6
**bootstrap-phases (auth-scaffolding-baseline) ↔ input-validation (CLI frame verbs)**: the `peer_admitted` survivor at the three client seams stays untested by construction with no seam added, and the live driver session runs through the existing `check_endpoint_dir`, peer euid and server checks (security-plan/bootstrap-phases fct-sawbqqxr) (security-plan/input-validation fct-l7hy7pzk).
@binding n=7
**data-protection (Measurement B) ↔ data-protection (pinned-binary re-hash)**: one tree keying two `bin/<key>/` folders is recorded with no cause claimed, the SHA-256 re-hash before reuse standing as the guard either way (security-plan/data-protection fct-uxsbnoo5) (security-plan/data-protection fct-zvqsbdzq).
@binding n=8
**dependency-security (G2, release check) ↔ input-validation (the FAKE_AGENT_HOOK_PANIC seam)**: G2's one exemption holds only while the seam stays named in its own file `src/cmd/hook/seam.rs`, compiled out of every release build as the release check proves, so the remnant removal must leave both green (security-plan/dependency-security fct-fguujmtu) (security-plan/input-validation fct-qyyrikqm) (security-plan fct-h3byb6pq).
@binding n=9
**dependency-security (supply-chain job) ↔ bootstrap-phases (secret-scanning-ci-gate)**: CI's seven jobs read green at wrap take in the supply-chain job's fuzz lockfile audit and the test job's canary scan gating every scan-gated upload (security-plan/dependency-security fct-i5izpzxc) (security-plan/bootstrap-phases fct-f326yvjk).
