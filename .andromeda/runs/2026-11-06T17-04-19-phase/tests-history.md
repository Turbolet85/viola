@history_item n=1 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-gq6m6fhp, the recorded supply-chain red) recorded `supply-chain` red on RUSTSEC-2026-0441 (`pin-utils` via `portable-pty =0.8.1` → `nix 0.25.1`) at ci#42030025050, owned by the CARRY on «Version done-check», remedy: move `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils`, ignore list not widened; the `=0.8.1` pin existed for a 0.9.0 Windows bug that left with that chunk. This is item 12's origin and the §9 Supply-chain text this chunk retires once the job is green.

@history_item n=2 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — (amd-gfssmgld, amd-qftp2wt2, amd-vexkxwon) the precedent for an advisory cleared by moving a pin: tokio `=1.53.1` → `=1.53.3` (RUSTSEC-2026-0386 patched) with features unmoved, and the test-plan's every version mention (bootstrap Dev-deps, §8 Time, §7 Time-sensitive row) moved in the same wrap. The `portable-pty` move must likewise update every `=0.8.1` site (§1 viola-pty entity, §5 Module ↔ PTY Tools line, §6 tui driver row).

@history_item n=3 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (amd-iuxhqvcf, the serde_json `unbounded_depth` ban and deny-probes 14/14) set `deny-probes: 14/14 banned, control clean` (13 `probes=(` rows plus `channel-tokio-edge`) and the Supply-chain error line; the portable-pty move must leave this count and line unchanged, and any new ban would move it by that rule.

@history_item n=4 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (amd-2pa3sfd6, the CI gates) the sync-crate `cargo check` target set became four (`viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`), `scripts/channel-tokio-free.sh` joined the sole-root step (`channel-tokio-free: ok`, presence read from the printed line, never the exit), and `scripts/rmcp-features.sh` joined Lint; `viola-pty` stays a listed sync crate, so the new portable-pty graph must stay tokio-free under the sole-root `deny-sync.toml` ban.

@history_item n=5 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — (amd-gm7nfesn) created the Supply-chain stage (cargo deny, the sole-root `deny-sync.toml` tokio ban per crate in `scripts/sync-crates.txt`, `scripts/deny-probes.sh`, zizmor, the weekly `nightly.yml` advisories) and the failure bullets; it recorded the cargo-deny 0.20 CLI form (global `--config`). This is the gate set the item-12 remedy must turn green.

@history_item n=6 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — (amd-5rsyhjff) added `scripts/orphans-check.sh` (+ `--probe`) per lib/bin target, the `fuzz/Cargo.lock` advisories + sources audit (root and fuzz lockfiles in nightly), and the `release` job's `scripts/release-check.sh --probe` then default mode (`--locked --bin viola`), a plain exit-code job. All of these are re-read after the dependency move, and the fuzz lockfile must also stay clean of RUSTSEC-2026-0441.

@history_item n=7 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — (amd-6aaoejju) the release-check probe reads `5/5 refused, control clean` (was 3/3), and the check also fails an artifact built with a test-only feature. This is the closing line the acceptance requires after the `portable-pty` move.

@history_item n=8 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (amd-xafqgfyt, the orphans probe line) the orphans probe line is `orphans-check probes: 2/2 fired, control clean` (was the stale `1/1`). This is the Lint line the extract's `2/2 fired, control clean` rests on.

@history_item n=9 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — (amd-eezt2egh) made the coverage ignore regex separator-agnostic (`crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`, harness `COVERAGE_IGNORE`), recorded as not a widening, and moved the coverage JUnit to `target/nextest/ci/junit.xml`, with MSRV and fuzz through rustup. This is the origin of the Windows-path comment that item 17 rewords: the four trees must stay byte-identical.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-rrxh5sdy, residual Windows reasons and the one-leg perf job) re-worded the coverage ignore class as «a form kept from the Windows target» (no longer a current reason), the perf run as its own `perf` job (was per OS), the harness target-dir relink reason as history, and the OS-gated-tests-never-counted rule as any OS `#[cfg]`. It is the direct precedent for item 17's `COVERAGE_IGNORE` / `COVERAGE_FLOORS` comment edits, and it found residual wording that no sweep pattern caught (a trap for item 17's re-verification).

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-zrj7cxkl, the OS scope) added the §1 OS scope entry: Linux only, CI on `ubuntu-latest`, Windows and macOS code recoverable at tag `multi-os-final` (`ecbe7ab3`). The multi-os-compat required test became the full workspace plus the OS-gate census on `ubuntu-latest`, the perf job one `ubuntu-latest` leg, and the «Linux-only local run as proof for Windows/macOS» rule was retired. This is the scope item 17's remnant removal must reach.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-fngxnwic, the OS-gate census reads another OS as a fault) `faults` gained its third class: a gate naming `windows` or `target_os = "macos"` is a fault even when a row names it, with a parse that catches multi-line `cfg_attr`. Self-check `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named`. The count is 39 functions, 2 gated files, 41 `GATES` rows (was 52/7/59). Item 17's removals must leave the census green, and v1-53 re-reads this count (re-measure and re-word if a gate moves).

@history_item n=13 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (amd-fz3qikah, the census of OS-gated tests) created `tests/contract_os_gated_tests.rs` (the `GATES` table, the walk rules, `andromeda:walks-tree`), a root `contract_` binary run under `run --e2e` in `nextest-e2e`; multi-os-compat row became `CI matrix + contract`. Its count rule (the rows of `GATES`, including `cfg_attr` case rows) is the rule the v1-53 re-read applies.

@history_item n=14 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (amd-qqdv5cf3, run --e2e and the widened E2E regex) built `run --e2e` (suite `nextest-e2e`) with the E2E regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` and the integration layer as its complement; an unmatched `binary()` regex is a parse error. This is why the census and `contract_*` binaries are read in the E2E suite that v1-53's JUnit reading takes.

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-hnqrnlsp, the CI matrix [ubuntu-latest] and two workflows) set the one-leg matrix in `test`, `perf`, `lint` and `release`, each job and artifact keeping its `${{ matrix.os }}` form (so `junit-${{ matrix.os }}` still names the one artifact), the workflows at `ci.yml` and `nightly.yml`, zizmor's declined `concurrency-limits` at 2 low, the `ci` kill line at 26 binaries, and gates enforced in `test` on `ubuntu-latest`. These are the seven jobs that must read green at the wrap.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-zfsdx5lb, the PTY seam on openpty alone) the windows-sys `TerminateProcess` fallback, the ConPTY sideload, `PtyError::Load` and the DA1 answer left. `pty_backend()` is always `openpty`, and Module ↔ PTY reads real openpty spawns (exit on the handle, resize propagated, key after the reported size). This is the seam state the `portable-pty` move must keep. It also bears on item 14's `process-start` row (`openpty` alone) and on item 3 (the sideload test case left with this retirement).

@history_item n=17 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — (amd-6ct2ncg2) replaced the 14-name S6 literal with «the 11-name identity floor measured on the Windows host» (`IDENTITY_FLOOR`) and set the no-EOF chaos mode as measured in CI (on Linux, output outlives the leader only for a holder in its own process group). It is the origin of the Windows-host wording that item 21 / item 17 rewords, and of the no-EOF case the portable-pty move must keep green.

@history_item n=18 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (amd-hvw725mk) placed the differing Linux dev-host reading on `claude` 2.1.287 (12 names, four outside the eleven, three absent) beside the floor's Windows origin, «the floor and the literal oracle stay the eleven». Rewording the Windows-host origin must keep this literal oracle and its rule.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-p5q5fgy2, the R8 strip by exact name) made the persistent exception `config.json` `claude_env_keep` alone and the strip an exact match, the floor and its literal staying eleven. It also set E2 to «the child holds no viola socket or home path» with no Unix qualifier, which is the signal item 15's `/proc` reading witnesses.

@history_item n=20 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — (amd-dvo6fdgz) the E2 premise fix: «fds only 0/1/2 plus the PTY slave» became «the live `/proc/<pid>/fd` table shows nothing of viola's» (fixture files exempt by exact path), as measured `[0,1,2,3,4]`, with portable-pty's `close_random_fds()` closing fds above 2, so a planted leak cannot go red. Item 15 measures exactly this signal, and the `close_random_fds()` clause must be re-measured on the new portable-pty release.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-xxh5u3sr, the channel on a Unix socket) server verification is asserted on Linux, the macOS no-peer-pid carve-out left, and §1 Untestable zones were rewritten so the committed tests hold the real euid read on Linux. `peer_euid_admitted(observed, own)` admits `observed == Some(own)`. This is the zone item 8's survivor is recorded against.

@history_item n=22 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (amd-mgtz4fdl, the euid wiring, untestable and a known survivor) each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed witness. The always-admit `peer_admitted` mutant survives by construction, and «the mutation audit reads that survivor at the route entry "Version done-check"». No seam may be added. This is item 8's origin: it is recorded, not killed.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (amd-rytlxujp, the peer euid decision and the wrapper's drop) held the decision by injection (`peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, the real read cases, `serve_drops_a_refused_peer_before_any_frame`), and §8 states dependency injection with no env, flag or feature seam. These are the exact committed cases item 8's record must name.

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (amd-faby2jqj, the hook's two new fail-open causes; amd-vokupvp6, `server-verify-failed`'s reach) the hook fails open on a server of another euid «its wiring with no committed witness». MCP's mapping gained `peer_admitted` → `server-verify-failed` and `check_endpoint_dir` → `socket-dir-refused`. The plan lines that name these causes must not claim a committed witness of the wiring (cascade bindings on item 8).

@history_item n=25 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** — (amd-3bg7fcwk, the viola-channel untestable zone narrowed) narrowed the untestable zone to the other-euid peer, asserted at the peer-credential check boundary (no second OS account). (amd-dknbq254) also added the Windows `CreateNamedPipeW` control-pipe cases, whose wording (fct-hhwc4gim) item 17 must remove or keep with a reason.

@history_item n=26 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-kqu5zghj, the security sweep, control negatives and exit-cause matrix on Linux) removed the Windows listener remote-client and DACL bullets, SQOS, the ConPTY companion and `.cmd` cases from Security control negatives and Chaos, and made the impostor rows and server identity ungated. Any `CreateNamedPipeW` wording still standing in e2e-test-strategy.md is a residual that item 17's follow-through must catch.

@history_item n=27 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — (amd-5gt53dby) the founder's ruling that no per-chunk, pre-push or CI mutation gate exists: `run --mutants` is named-only and mutation runs at the epoch boundary. This is why mutation is not among this chunk's gates and item 8 is read without a mutation run.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-nssv2hvq, mutation without the Windows leg or the host scratch) `mutants.out/outcomes.json` sits at the repo root, the boundary audit has no Windows leg, and `windows-mutants.yml`, `HOST_SCRATCH` and `scratch_bytes` left. The host-unreachable rule names a `cfg` item or a `cfg!` const. This is context for item 8's survivor record and for any remnant in harness mutation code.

@history_item n=29 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** — (amd-wm276zs2, nextest the only supported runner) a plain `cargo test` is unsupported: three root-bin cases are known-cause reds under it, with no owner. The «every gate green» re-read must go through nextest / `agent-run`, never a plain `cargo test`.

@history_item n=30 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (amd-lhoufd4i) the test job gates `coverage,doctest`, `junit-${{ matrix.os }}` uploads the one `target/nextest/ci/junit.xml`, nextest `retries = 0`, `#[ignore]` is no parking place, and serialising tests is banned. (amd-amvotjij) also fixed `gate::SUITES` at eight and the session record at `{v, session, home, instances, supervisor_pid, supervisor_started_at}`. That field set is the one that must hold after the `crates/viola-e2e/src/harness/session.rs:52` remnant is removed.

@history_item n=31 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (amd-hwlpuxvg, boot's two flags, harness_session, status over viola list) `boot --statusline-echo` writes a command `<bin dir>/viola-fake-agent[.exe] statusline-echo …`, and `agents_mode` joined `session.json` / `supervise.json`. The `[.exe]` text and the session record are EXE_SUFFIX / `session.rs` sites that item 17 touches, and each harness command must still print one JSON document.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-n33ar24f, one harness shim, agent-run.sh) `scripts/agent-run.sh` is the one shim, `boot` copies the fake agent as `bin/claude` (no `[.exe]`), viola makes the home 0700, and `secret-scan`'s `mode_check` is `unix`. This is the precedent for removing the remaining `EXE_SUFFIX` joins in `crates/viola-e2e/src/harness/` while keeping `boot` and the assert_cmd target-dir lookup.

@history_item n=33 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-62z53en5, the bootstrap phases and test data bootstrap without the Windows arms) set the kill line at 26 binaries, made libc a plain `viola-e2e` dependency (windows-sys pin gone), made `Instant::now() + WITHIN` 31 sites in 23 files, and had `unconnectable` / `holder_gone` read Unix only. These are counts that remnant edits under `tests/` or the harness may move, each by its stated rule.

@history_item n=34 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (amd-jqjfbovx) the §5 home-resolution bullet names the `resolve_home_*`, `canonical_home_*` and `plain_drive_*` units of `src/cmd/mod.rs`. If item 17 removes `plain_drive`, its unit name leaves this plan line with the code (or stays with its reason).

@history_item n=35 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — (amd-o6hxlf3v) measured that no manifest holds trycmd and no `tests/cmd/` exists, and pinned `viola --help` by the `HELP` literal (`help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`) in `tests/cli_output_plain.rs`. «The remaining trycmd / assert_cmd tooling claims (§3, §5, §6, §7, §10) stand until "Version done-check" decides tool or text». This is item 1's origin and the literal that item 17's edits at lines 104 and 116 must keep holding.

@history_item n=36 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — (amd-4cs5rgtp) the `viola list` board is pinned by literals in `tests/cli_list.rs` and the units, «not a trycmd case (was: trycmd)», trycmd keeping the send mirror and help (help later moved too). This is the literal-assert precedent that item 1's «text» option generalises.

@history_item n=37 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (amd-xdupmfpc) `tests/cli_verify.rs` pins verify's lines by literal asserts, and «the trycmd verify case arrives with a redaction-heavy consumer». That promised case (fct-lzdm6e7w) and its snapbox redactions stand or are re-worded with item 1's verdict.

@history_item n=38 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — (amd-r32l3vjd) created `tests/cli_output_plain.rs` (no ESC under the colour variables, the SGR parser self-check under the outer PTY). (amd-fakhmvq4, 2026-10-17-optional-plugin-install) later reshaped `help_lists_only_served_verbs` to read its usage program as `viola` or `viola.exe`. The `viola.exe` arm is the line-104/116 Windows remnant that item 17 removes, and the plan text at fct-lzdm6e7w follows it.

@history_item n=39 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (amd-bb4rhxnd, the link verbs' result lines and outputs pinned) the `viola link` / `viola unlink` result lines joined `--help` and the board as layouts pinned by literal (`link_verbs_print_the_design_lines`), and `help_lists_only_served_verbs` reads no `hook`, `help` or bare `plugin`. This is the third literal family that item 1's masters-agree verdict must name together.

@history_item n=40 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (amd-e54mtnrn, Path 5 records the classifier's tables; the real CLI has two local routes) the closed non-editing list is pinned at the unit tier (8 reply cases, 23 `REPLIES`, 27 negative controls), and its real-terminal reading on foot 1.28.0 is a local live measurement, not a suite. A route entry's live readings are that chunk's own local measurement, never a test layer or CI. This binds item 10's hypothesis and both of this chunk's live sessions.

@history_item n=41 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (amd-tvc747xe) recorded the live `wait --json` / `last` and `question` → `viola answer` readings on `claude` 2.1.296 as «a local measurement and no suite» in that chunk's `evidence/switch-readings.ndjson`. This is the form the «overseer drives a builder» session and the 3600 s hold reading must take.

@history_item n=42 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — (amd-npoy5txr) `run --local-live` refuses under `CI` (`live-in-ci`, exit 2, before any build). This is the harness guard that keeps a real-`claude` run out of CI, which the live sessions and item 11's real-CLI read rest on.

@history_item n=43 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — (amd-6grdlnwe) the dialog hold's clock-driven unit cases (`read_hold`, the 120 s hold, expiry's one `dialog-expired`, the hook bound `DIALOG_HOLD_MAX` plus grace, the timeout margin), `dialog_hold_s` as the tenth `SETTINGS` row, `ALL` 16 and `WAIT_WAKE` 6. The founder answered the widening himself, and the card confirm was PROVISIONAL (later cleared in the Epoch 5 batch, amd-3zgedmee). Item 2's live hold near 3600 s reads these bounds.

@history_item n=44 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (amd-rhpvycfa, a start's socket directory refusal; amd-ijyzrs56, the socket directory's security negatives; amd-pj6zrvnq, its unit cases; amd-gelnrgex, exit 21's sixth cause; amd-xv5rttjq, the IPC boundary's directory check; amd-4tx4tode, its fixtures) established the socket-directory pair (0755 or a symlink to 0700): log word `socket-dir-refused`, hint cause `socket-dir`, `Form` fixture, `socket_dir_verdict` / `check_socket_dir` / `check_endpoint_dir` cases, and the start's exit 1 with no bind or child. Each rests on «the wrap's card confirmed by the overseer as his delegate, PROVISIONAL». Item 7 brings this wording to the founder, and every oracle moves together.

@history_item n=45 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (amd-634nmyhl, `viola list` on the read rule) a home another user may read refuses `viola list` (exit 21, `ScanRefused::Home`), held by `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` and `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance`, «the overseer's technical answer as the founder's delegate, PROVISIONAL … the founder's own word owed in the batch at Epoch 7's boundary». This is item 7's board-read-rule item.

@history_item n=46 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (amd-liu2datt, a terminated wrapper's child on every OS) added `run_child_ends_when_its_wrapper_is_terminated`, now `kill -KILL` alone (amd-mgo44gby, 2026-11-05-linux-only-code-and-ci). This is the fake-agent reading that item 11 keeps standing, and the portable-pty move must keep it green.

@history_item n=47 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (amd-jv5yg5mg, the hangup case on Linux only) `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` is `cfg(target_os = "linux")` and reads the SIGHUP disposition through `ps -o sigignore=`. It is one census row, and it is the Unix behaviour (with amd-tirjkxrl's SIGHUP-ignoring exit and amd-75ste4oe's per-command process groups) that v1-53's live `viola run` confirms.

@history_item n=48 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (amd-75ste4oe, the headroom floor, the hook hangup named) every command the fake agent spawns runs in its own process group on Unix, and Stack adjustments gained the measured hangup-cut-profile mechanism (0 refused of 800). That is the profraw census fact (fct-s6rhzqu4) behind the `test` job's coverage merge staying green. E4 reads «none in 0.1.0».

@history_item n=49 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — (amd-jsj5zfbp) recorded the corrupt-profile mechanism, with `scripts/profraw-census.sh` as a dev-host witness, and a red folded and closed by cause, never by a green re-run, «nextest `retries`, the coverage ignore regex and the gate's required suites» kept. (amd-ut3mpx3z, 2026-09-29-h2-conpty-resize-probe) holds the `LLVM_PROFILE_FILE` exception for the harness self-tests. These are traps if the coverage merge reads red while gates are re-read.

@history_item n=50 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (amd-cnisrb36, the MCP tests as landed) `tools/list` is exactly the five tools (`mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`), and Path 6's MCP form was owed then and landed as `mcp_tools_path6_budget_paused_send`. Item 4's v1-13 / v1-36 dispositions must keep the five tools, never `release` or `link`.

@history_item n=51 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (amd-x467ldvw, E3 built; amd-uscs6qzr, the served link methods) built `viola link` / `viola unlink` and the served methods (`e3_two_links_and_a_send_leave_one_link_record`, the `links_set_method_*` units). These are «their CLI halves are built» for item 4. Its halt answers stay PROVISIONAL (amd-jxumzis5 Kept).

@history_item n=52 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (amd-g7bh2c4z) Path 6's cli and ipc halves landed (`path6_release_budget_lifts_the_pause`, etc.), and `release --budget` returns `{budget_paused:false, override_until}`, with the MCP form owed elsewhere. This is the built CLI half of v1-13's budget-release clause that item 4's disposition names.

@history_item n=53 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — (amd-gzxcxwvf, the GUI surfaces and their tests leave 0.1.0) the founder's 2026-10-16 ruling cut the api-service and web-spa surfaces, and E4 is a stub «The frontend is deferred to 0.2.0; its technology is not chosen». This is the basis for the boundary that a cap naming a frontend surface reads `deferred` only on the founder's word.

@history_item n=54 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (amd-3hn5t2j2, the error-body enumeration in the contract suite) created `tests/contract_error_bodies.rs` with guards `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`. An item-6 fold or item-16 re-count must keep these green. (amd-w6r7zjmy) folding `Refusal` into `AgentError`, messages unchanged, is the precedent for a one-enum-per-crate fold.

@history_item n=55 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (amd-wd5hieih, the tree, diagnostics, stamps and pin checks at the unit tier) introduced `StampsError::StrictModes` (`update_stamps_refuses_a_widened_ledger_and_leaves_it`), `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key` and `PinError::StrictModes`. (amd-v4kdvsdc, 2026-11-05) later removed `PinError::StrictModes`. These are the stamps and pin oracles that item 6's fold or ratification and item 5's Measurement B touch.

@history_item n=56 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — (amd-sreor2sv, the pinned copy's no-clobber placement) added `fs::place_private_new` and `pin::place_absent` with `PinError::HashMismatch` (`place_absent_over_a_copy_another_start_placed`) and the `placed` hook as a test parameter, not a seam. This is the `PinError` variant that item 6 names, and the pinned copy behind item 5's two `bin/<key>/` folders.

@history_item n=57 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (amd-3feqch4b, the `viola mcp` stdin line cap's tests) added the `serve` child cases (`serve_ends_the_session_at_a_line_past_max_frame`, `serve_on_a_closed_stdin_is_the_initialize_error`, each killed at 5 s), and (amd-btzw2oby, 2026-10-31) `ServeError::Initialize`, exit 11 for the unit child and 1 for `viola mcp`. These are where item 6's `ServeError` fold lands and where the test-child marker of item 3 (`MCP_SERVE_TEST_CHILD`) is used. (amd-hwmom55r) a `HELD`-variable child in `handlers.rs` is a second test-child marker of the same kind.

@history_item n=58 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (amd-itsyxvxp) built `run --perf` and the per-OS `perf` job and recorded «host max 72.8–73.7 ms» on the Windows dev host (fct-vxnii5yh). (amd-7m34ynih, 2026-10-04) set five rows required by name (`perf::ROWS`), a missing row being red. Item 17's follow-through brings §10's «per OS» `session-end` row and the Windows reading in line with the one `ubuntu-latest` leg.

@history_item n=59 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-aqxz4ezn, the e2e-home headroom read on an injected figure) the msrv red of ci#41930831242 was closed by ci#42030025050's `msrv` green, with `ensure_e2e_home`'s reading injected. The MSRV 1.96 job is one of the seven, and the new portable-pty release must build there.

@history_item n=60 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-ee2rxbze, the relied set, all twenty-four rows on every build) `LedgerRow::relied()` is 24 on every build and the literals are ungated. The H2 key-after-resize question was re-pointed to the open residuals targeted `another-os` (the «Windows-only live measurements» entry retired). This is context for item 17's remnant re-verification and for the plan's remaining Windows-owned clauses.

@history_item n=61 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — (amd-puxntbrv) the scan-gated `harness-`, `diag-` and `junit-<os>` uploads behind `secret-scan`, with no unscanned upload. This is the tests ↔ obs binding the extract cites: artifacts leave CI only behind obs-plan §9's scan.

@history_item n=62 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (amd-3fwhzts6, remove-the-guard witnesses bounded by `WITHIN`) a control whose neutralised guard lets a product process start needs a bounded witness (an unbounded one hung 27 minutes under the default profile). This is a trap for item 15's measurement or any control this chunk runs.
