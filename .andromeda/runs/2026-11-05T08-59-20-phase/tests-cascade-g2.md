@carried n=1 label=test-plan/test-strategy fact=fct-7osf2h3d
why: The chunk removes these `cfg(not(target_os = "macos"))` gates, so every case named here must compile and run ungated on Linux, and the macOS carve-out must leave the unit level's text.
quote: Runs on all 3 OSes, except the two real-cargo-mutants harness tests (§10 Mutation gate) and the server check's captured-span cases, compiled out on macOS, where the server check reads no peer pid: each frame verb's `request_refuses_an_unverified_server_inside_its_dispatch_span`, `hook_refuses_an_unverified_server_inside_hook_handle`, the test-only `cmd::client::refused_inside` helper they share (under `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`) and the `last.rs`, `pause.rs` and `unlink.rs` test modules, gated whole (a module is gated whole only when its every case is gated: `src/cmd/link.rs` gates its server-check case alone, and its reply-shape and line cases run on macOS); the decision table `server_verdict_rows` and the macOS `server_pid` case still run there, and so do the socket-directory check's captured-span cases `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` and `hook_refuses_a_loosened_socket_directory_inside_hook_handle`, which read the span recorder `Seen` from the test module `refused_seen` in `src/cmd/client.rs`, outside the macOS-gated `refused_inside`.
@carried n=1 run=2 label=test-plan fact=fct-njccdctv
quote: on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS.
@carried n=2 label=test-plan/test-strategy fact=fct-zx23mmpw
why: The chunk removes the Windows and macOS `perf` jobs of ci.yml, so the perf gate must stand as the ubuntu `perf` job alone, its five rows and its `max` gate unchanged.
quote: hyperfine 1.20.0 gates the hook deadlines on `max` (the `--perf` arm, the five hyperfine rows — `pre-tool-use` included, on an unstamped perf session — and the per-OS `perf` CI job are built; §10).
@carried n=3 label=test-plan/test-strategy fact=fct-cz5jn5fs
why: The chunk removes the audit's Windows mutation leg (`windows-mutants.yml`, `tests/contract_windows_mutation_scope.rs` and the Windows-host scratch), so the mutation level must stand as the on-demand Linux run, still with no CI mutation gate.
quote: `/andromeda-code-audit` runs cargo-mutants 27.1.0 per unit, `run --mutants` [`--file`] runs it `--in-diff`, and `run --mutants --package <member>` runs it over one whole member (the boundary-tier form), both through `--test-tool=nextest` on demand (named only, never in the default or `--all`). No per-chunk, pre-push or CI mutation gate.
@carried n=3 run=2 label=test-plan fact=fct-elca5edh
quote: cargo-mutants writes `mutants.out/outcomes.json` (under the harness-set output dir: the host mutation scratch `<repo parent>/viola-mutants-scratch/` on a Windows host, the repository root elsewhere)
@carried n=4 label=test-plan/test-strategy fact=fct-vekwchud
why: This contract case carries a Unix gate the census counts, so the chunk must keep it running on Linux and drop or keep its gate by the scope's `cfg(unix)` rule, its census row re-pointed or retired with it.
quote: (`fake_agent_statusline_replay_matches_every_recorded_statusline_set`, Unix; the synthetic `statusline_drift_names_a_changed_byte_a_swapped_order_and_a_short_replay`)
@carried n=5 label=test-plan/test-strategy fact=fct-hckuzazj
why: The shared root helpers lose their Windows arms (`icacls`, the debugger attach), and their Unix arms must keep the same names and behaviour for every root test that calls them.
quote: `home.rs` holds the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened. `frozen.rs` holds `Frozen`, the one freeze guard (`attach(pid)` and `Drop`): on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop; on Windows a debugger attach, detached on drop.
@carried n=6 label=test-plan/test-strategy fact=fct-hckuzazj
why: `SOCKET_ROOT_VAR`'s macOS arm leaves, so the test-owned socket root must read `XDG_RUNTIME_DIR` unconditionally.
quote: `tests/support/home.rs` holds the test-owned socket root: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`
@carried n=7 label=test-plan/test-strategy fact=fct-hckuzazj
why: Whether `scripts/agent-run.ps1` leaves with the Windows CI steps that alone run it is the scope's open fork, and the shim pair named here must follow that decision.
quote: **Harness shims:** `scripts/agent-run.{sh,ps1}`.
@carried n=7 run=2 label=test-plan fact=fct-yqn2bvnu
quote: `scripts/agent-run.sh` (POSIX) and `scripts/agent-run.ps1` (PowerShell) are identical thin shims. They run `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]` and forward its exit code and stdout unchanged. All logic lives once, in Rust, so both shells have byte-identical semantics on all 3 OSes.
@carried n=8 label=test-plan/test-strategy fact=fct-df3r2j7c
why: The census is re-pointed or retired with its gates: re-pointed, it must stay a root `contract_` binary run in `nextest-e2e`; retired, its file and this home must leave together.
quote: `tests/contract_os_gated_tests.rs` is a root `contract_` binary of the sync suites, auto-discovered with no `[[test]]` table; the E2E layer's regex selects it, so it runs under `run --e2e`, in suite `nextest-e2e`.
@carried n=9 label=test-plan/test-strategy fact=fct-gldxhydz
why: The chunk retires the multi-os-compat matrix and re-points or retires the census, and while the census stands every gate the removal leaves must be a current `GATES` row, never a stale one.
quote: | multi-os-compat | CI matrix + contract | §9 Matrix builds; §6 Contract suite (root `contract_os_gated_tests.rs`: every OS-gated test function and gated file under `tests/` and `crates/*/tests/` a `GATES` row with its reason, an unnamed or stale gate failing the run) |
@carried n=9 run=2 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=10 label=test-plan/test-strategy fact=fct-34amt7nu
why: The Windows channel negatives leave and the macOS and Unix gates on the server-identity, socket-directory and hook impostor negatives drop, so each surviving IPC negative must run on Linux beside its controls.
quote: §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial; root `security_negatives_server_identity.rs`, off macOS: a CLI frame verb against a server that is not the recorded wrapper; root `security_negatives_socket_dir.rs`, Unix: a socket directory at 0755 or a symlink to 0700 refusing a start, the eight CLI frame verbs and the hook, each left as found; root `channel_links.rs`: raw `link` / `unlink` frames with a bad name or pair, each `-32602` with no line written) + Security sweep (the hook fail-open impostor rows, off macOS) |
@carried n=11 label=test-plan/test-strategy fact=fct-mqdaf5bf
why: The Windows protected-DACL negative leaves with its `cfg(windows)` arm, while the umask negative and the other V7 negatives must keep running on Linux unchanged.
quote: a start under umask 000 making every home path owner-only (Unix); a Windows `--home` outside the profile taking the protected DACL, read back through `icacls /save`;
@carried n=12 label=test-plan/test-strategy fact=fct-peucakqx
why: The chunk trims the Windows target dependencies from the manifests and their `deny.toml` entries, so every V9 gate named here must still pass on the trimmed manifests, the weekly advisories staying in `nightly.yml`.
quote: | Supply chain (V9) | CI gates | §9 Lint stage (sync-crate `cargo check`, `scripts/orphans-check.sh`) + Supply-chain stage (cargo deny incl. the `fuzz/Cargo.lock` audit, sole-root tokio ban, ban probes, zizmor) + Release build stage (`scripts/release-check.sh`) + `nightly.yml` weekly advisories (root and fuzz lockfiles) |
@carried n=13 label=test-plan/test-strategy fact=fct-brzfiy4i
why: The chunk fixes two harness cases that pass or fail by the runner's free disk, so they must decide on a free-space figure the test injects, never by a retry or an `#[ignore]`, and the MSRV unit run must be green.
quote: | discipline (no screen parsing / no blanket approval / no flakes) | all | §6 Scenario 4 negative; §11 |
@carried n=13 run=2 label=test-plan fact=fct-2cc4sa5w
quote: NEVER ignore flaky tests; quarantine and fix immediately
@carried n=13 run=3 label=test-plan fact=fct-uptme7ma
quote: MSRV 1.96, edition 2024), executed through cargo-nextest 0.9.146 (process-per-test, JUnit).
@carried n=14 label=test-plan/unit-test-strategy fact=fct-o57nkgzg
why: The Windows pipe and macOS `$TMPDIR` golden vectors leave with their arms, while the Linux `$XDG_RUNTIME_DIR` form and the `/tmp` fallback vectors must stay byte-identical, no Linux wire contract moving.
quote: with the Windows `\\.\pipe\` form and the Unix per-user 0700 directory forms `$XDG_RUNTIME_DIR/viola/viola-<h12>.sock` (Linux) and `$TMPDIR/viola/viola-<h12>.sock` (macOS), plus the `/tmp/viola-<uid>/viola-<h12>.sock` fallback (security IPC access control, Unix).
@carried n=15 label=test-plan/unit-test-strategy fact=fct-o57nkgzg
why: Where `PEER_EUID_REPORTED`'s `cfg!(unix)` loses its sibling, the decision must keep refusing a peer whose euid cannot be read on Linux, held by injected euids, with no env, flag or feature seam forcing a refusing decision.
quote: The peer euid decision and read: a peer `peer_euid_admitted` cannot read is refused where the euid is reported (`PEER_EUID_REPORTED`, `cfg!(unix)`), held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`;
@carried n=16 label=test-plan/unit-test-strategy fact=fct-o57nkgzg
why: This `cfg(windows)` unit case leaves with the Windows arm, so the plan must name the Linux evidence for no inherited channel handles rather than a case that no longer exists.
quote: `#[cfg(windows)]` handle non-inheritance, which backs E2 and the §12 open question. After the `viola-channel` server creates a pipe instance and a client connects, `GetHandleInformation` (windows-sys) on both handles (via `AsRawHandle`) reports `HANDLE_FLAG_INHERIT` clear. This is the only Windows evidence; there is no child-side probe.
@carried n=16 run=2 label=test-plan fact=fct-2hayha6u
quote: the connected stream's server pid on each OS, and Windows handle non-inheritance.
@carried n=17 label=test-plan/unit-test-strategy fact=fct-o57nkgzg
why: The Windows and macOS server-pid arms leave, so the pid cases must hold the Linux `SO_PEERCRED` reading, and the none-case with `SERVER_PID_REPORTED` is the closed-word fork P4 decides.
quote: The connected stream's server pid, `Client::server_pid` and `TokioClient::server_pid` (`peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none), asserted on each OS by `client_server_pid_is_the_serving_process` and `client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`crates/viola-channel/src/client.rs`) and their Tokio twins `tokio_client_server_pid_is_the_serving_process` and `tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`tokio_client.rs`, behind the `tokio` feature); `SERVER_PID_REPORTED` is false on macOS.
@carried n=18 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: With macOS gone no OS reports no peer pid, so the plan must decide whether `server_verdict_rows` keeps this row as a pure-table case or retires it with `SERVER_PID_REPORTED`, the Linux verdict unchanged.
quote: where no peer pid is reported (`peer_pid_reported` false, macOS) the verdict is `Matches` with no comparison, the recorded gap.
@carried n=19 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The `cfg(windows)` read-rights case leaves with its arm, and whatever of `windows_read_verdict` and its cases stands outside a gate is listed and decided with the Windows strict-modes code it serves, `unix_read_verdict` and its case unchanged.
quote: and `windows_read_verdict` is the write rule plus a refusal of any allow ACE granting a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to an untrusted SID, inherit-only ACEs included (`windows_read_verdict_refuses_any_read_right_to_another_sid`, `windows_read_verdict_takes_the_trusted_sids_and_keeps_the_write_rule`, and the `cfg(windows)` `read_rights_are_the_named_flags`).
@carried n=20 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The SDDL descriptor stubs and the real `windows-2025` grants leave with windows-sys and the Windows strict-modes arm.
quote: Windows strict-modes feeds synthetic security descriptors built from SDDL strings (via windows-sys) to the descriptor-check function. This covers a foreign-owner SID, a NULL DACL, an inherit-only GENERIC_WRITE ACE, a read grant on `diagnostics/`, and `CREATOR OWNER` treated as the user. These are the stubs for the cases hosted runners cannot provision (test-scope Sec 1). A Users write grant on an instance tree and a Users read grant on `diagnostics/` are also made for real on `windows-2025` (the instance check, below).
@carried n=20 run=2 label=test-plan fact=fct-7vfmbe3m
quote: Unix strict-modes and Windows strict-modes (the write and the read rule) over synthetic SDDL descriptors and, on `windows-2025`, real grants,
@carried n=21 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The `cfg(windows)` instance-tree cases and their oracle leave, and the `cfg(unix)` cases whose only sibling left lose their gate, each keeping its Linux refusal.
quote: Unit cases `strict_instance_*`: five widened modes (`cfg(unix)`), a tree viola wrote passes, a home with no instance directory passes, and a path that cannot be statted is unreadable. Three `cfg(unix)` root cases in `tests/cli_revive.rs` hold the refusal: two on `viola revive` (a widened instance tree; and, since chunk 2026-10-10-budget-governor, a group-writable home, refused and left as found) and one on `viola revive --list`; one in `tests/cli_links.rs`, `link_verbs_refuse_a_widened_instance_before_any_log`, holds it on `viola link` / `viola unlink` (§5 On-disk). On Windows the instance tree is covered by `cfg(windows)` cases green on `windows-2025` (as measured at ci#40403389629): a tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` refuses a home made with `create_dir_all` before viola, its refused entry an inherited write grant to a SID strict-modes does not trust, whichever SID holds it (its oracle below).
@carried n=21 run=2 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
quote: The plain-home case's Windows oracle is the `cfg(windows)` test helper `strict::win::tests::inherited_untrusted_writes(home)`
@carried n=22 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The instance check's and the diagnostics check's Windows rule arms leave, so `events.ndjson` and a diagnostics file must stay held to the Unix rule on Linux.
quote: its `snapshot.json` (write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows);
@carried n=22 run=2 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
quote: a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes
@carried n=23 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The off-Unix arm of the socket directory check and its case leave, and `check_socket_dir` must run ungated on Linux, never chmodding.
quote: `strict::check_socket_dir`, which passes an absent path and never chmods, by `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found` and, off Unix, `check_socket_dir_off_unix_reads_nothing`;
@carried n=24 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The board scan's `windows-2025` read-grant case leaves, while the board must keep the home on the read rule on Linux through its remaining case.
quote: held by `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`.
@carried n=25 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
why: The Windows forced-window placement case and the Windows read-only-file arm leave, while their Unix twins must keep running on Linux as they are.
quote: the forced-window cases `place_private_new_placed_copy_execs_at_once` (Unix) and `place_private_new_placed_copy_runs_at_once` (Windows) (§5);
@carried n=25 run=2 label=test-plan/unit-test-strategy fact=fct-vbcc7bad
quote: the refusal of a longer target under a read-only directory runs on Unix only, Windows using a read-only file.
@carried n=26 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
why: The case-insensitive Windows arm of the strip leaves, while the strip must stay exact on Linux and the floor literal must stay the eleven names written into the test.
quote: every inherited `CLAUDE*` name is removed (ASCII case-insensitive on Windows, exact on Unix) unless it is in the persistent set, and a floor name is removed whatever the persistent set says. A Linux dev-host reading on `claude` 2.1.287 stands beside the Windows one and differs (12 names in a hook's environment, four outside the eleven, three of the eleven absent; architecture §Occupied Resources → Environment variables); the floor and this literal stay the eleven.
@carried n=26 run=2 label=test-plan fact=fct-jfqpalye
quote: NEVER import the product's own `IDENTITY_FLOOR`, ledger rows or refusal ordering as the test oracle.
@carried n=27 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
why: The npm-shim to `claude.exe` resolution is Windows text the removal may leave naming gone code, so the plan must list whether it and its fixture case leave or stay.
quote: npm-shim → `claude.exe` resolution over fixture shim files in a tempdir.
@carried n=28 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
why: The `.cmd` / `.bat` refusal (`BatchScriptChild`) is the scope's open fork, so its held messages and outputs must be kept or retired with P4's decision on whether it names a removed arm.
quote: `resolve_program` returns `Result<PathBuf, AgentError>`, `AgentError` holding `BatchScriptChild` and `ProgramNotFound`, their messages and outputs held by `refusal_messages_are_fixed`, `tests/cli_program_resolution.rs` and `tests/cli_verify.rs`.
@carried n=29 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
why: The Windows relied set and the Windows-only verdict leave, and the Linux relied set must stay the 24 rows so `verified` and the older-stamp oracle read the same on Linux.
quote: the verdict `verified` reads `LedgerRow::relied()`: 24 rows on Unix, 21 on Windows, `agents-join` then `local-command-compact` appended last, both relied on every OS
@carried n=29 run=2 label=test-plan/unit-test-strategy fact=fct-hpqcf7r7
quote: and one of the eighteen, `resume-session`, `agents-join` and `local-command-compact` verifies on Windows only.
@carried n=30 label=test-plan/unit-test-strategy fact=fct-id225cf7
why: The helper's macOS gate leaves, so it and every frame verb's and the hook's captured-span case must compile and run ungated on Linux, the `refused_seen` split no longer an OS carve-out.
quote: each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS, §2 Unit);
@carried n=30 run=2 label=test-plan/unit-test-strategy fact=fct-id225cf7
quote: the span recorder `Seen` stands in the test module `refused_seen`, out of the macOS-gated `refused_inside`, so these two run on macOS.
@carried n=30 run=3 label=test-plan fact=fct-fn3ybwy5
quote: (`connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span`, `hook_refuses_a_loosened_socket_directory_inside_hook_handle`, both running on macOS too)
@binding n=1
**test-strategy multi-os-compat row ↔ test-strategy census binary home**: re-pointing or retiring the census decides both the `GATES` table every remaining OS gate must be a row of and the `contract_` binary's place in `nextest-e2e`; the two move together (test-plan/test-strategy fct-gldxhydz) (test-plan/test-strategy fct-df3r2j7c).
@binding n=2
**test-strategy Unit level ↔ unit-test-strategy root bin, viola-state and viola-channel**: the macOS carve-out is one chain, the `refused_inside` gate, the no-peer-pid `server_verdict_rows` row and `SERVER_PID_REPORTED`, so the chunk drops the gates and decides the closed word together, every captured-span case then running on Linux (test-plan/test-strategy fct-7osf2h3d) (test-plan/unit-test-strategy fct-id225cf7) (test-plan/unit-test-strategy fct-vbcc7bad) (test-plan/unit-test-strategy fct-o57nkgzg).
@binding n=3
**test-strategy IPC and filesystem rows ↔ unit-test-strategy viola-channel and viola-state**: the Windows channel and DACL negatives leave with their unit-level Windows cases and the off-macOS server-identity negatives run on Linux, while every Linux negative, exit code and wire contract stays as it is (test-plan/test-strategy fct-34amt7nu) (test-plan/test-strategy fct-mqdaf5bf) (test-plan/unit-test-strategy fct-vbcc7bad).
@binding n=4
**test-strategy harness shims ↔ the body's harness contract and the CI plan**: the decision on `scripts/agent-run.ps1` moves the shim pair named in the directory pattern, the harness contract's "on all 3 OSes" and ci.yml's `pwsh shim` steps together (test-plan/test-strategy fct-hckuzazj) (test-plan fct-yqn2bvnu).
@binding n=5
**test-strategy discipline row ↔ the body's unit framework and the CI plan's `msrv` job**: the two e2e-home free-space cases decide on an injected figure so the MSRV 1.96 unit run on ubuntu is green, with no retry and no `#[ignore]` (test-plan/test-strategy fct-brzfiy4i) (test-plan fct-uptme7ma) (test-plan fct-2cc4sa5w).
@binding n=6
**test-strategy and unit-test-strategy OS text ↔ the test-plan master**: the 3-OS legs, per-OS jobs and Windows and macOS carve-outs carried here leave the master only as amendments the wrap writes, which this chunk's plan names as expected (test-plan/test-strategy fct-gldxhydz) (test-plan/test-strategy fct-zx23mmpw) (test-plan fct-njccdctv).
@binding n=7
**test-strategy supply-chain row ↔ the manifests' trimmed Windows dependencies**: cargo deny, the sole-root tokio ban, the deny probes and the release check must each still end their required line after `Cargo.toml` and `deny.toml` lose the Windows entries (test-plan/test-strategy fct-peucakqx).
