@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=108
last: - 2026-11-04: When a test's OS reader (a `ps` keyword, a `/p
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=9 class=stale
leaf-text: six agent-driven surfaces across three required OSes
source: fct-cyzmyvps «Test-scope Sec 2 has 6 agent-driven surface entries (cli verbs, hook exec, tui passthrough, the IPC channel, MCP stdio and library crates) on Linux (CI on `ubuntu-latest`).»
replace-with: six agent-driven surfaces on Linux (CI on `ubuntu-latest`)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=13 class=stale
leaf-text: `scripts/agent-run.{sh,ps1}` are thin shims over
source: fct-yqn2bvnu «**Harness implementation:** `scripts/agent-run.sh` (POSIX) is the one thin shim.»
replace-with: `scripts/agent-run.sh` (POSIX) is the one thin shim over
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=20 class=retired
leaf-text: — viola creates it, except on Windows x64 where `seed_conpty` first hard-links the ConPTY companions into a root test home's `bin/<key>/conpty/` from `target/conpty-seed/<key>/` (`Wrapper::boot`, piped and outer-PTY starts; never `run_viola_unseeded` or `conpty_sideload`, which keep the product's write; `seed_conpty` makes the home and the seed dir through `viola_state::fs::create_private_dir` first, so a seeded home passes the instance check; its companions are hard links to one shared seed file, so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded), and except
source: fct-ze6ljnvw «the Windows and macOS code, tests, fixtures and CI jobs are not in the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: — viola creates it, except
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: 24 on Unix and 21 on Windows
source: fct-lzdm6e7w «(`ROW_IDS: [&str; 24]`, every row relied on every build)»
replace-with: all 24 on every build
@leaf_edit n=5 leaf=.claude/docs/tests-summary.md line=26 class=retired
leaf-text: Run B's `paste-3` paste (its Windows reading owed to "Windows-only live measurements")
source: fct-qbytqn26 «cover that shape, stamped on live `claude` 2.1.287 on the Linux dev host there, `stamped 2.1.287  18 pass  0 fail`»
replace-with: Run B's `paste-3` paste, stamped on live 2.1.287 on the Linux dev host
@leaf_edit n=6 leaf=.claude/docs/tests-summary.md line=27 class=stale
leaf-text: (`tests/cli_wait_last.rs`, `tests/chaos_wait_vanish.rs`) on all three CI OSes
source: fct-mdqnbyad «`tests/cli_wait_last.rs` covers cli and ipc-internal (the wrapper channel) in CI on `ubuntu-latest`»
replace-with: (`tests/cli_wait_last.rs`, `tests/chaos_wait_vanish.rs`) in CI on `ubuntu-latest`
@leaf_edit n=7 leaf=.claude/docs/tests-summary.md line=28 class=stale
leaf-text: as landed, `tests/cli_answer.rs` over `fixtures/fake-scripts/path4.json` on all three CI OSes
source: fct-p2i2onck «`tests/cli_answer.rs` over `fixtures/fake-scripts/path4.json` covers cli and ipc-internal (the wrapper channel) in CI on `ubuntu-latest`»
replace-with: as landed, `tests/cli_answer.rs` over `fixtures/fake-scripts/path4.json` in CI on `ubuntu-latest`
@leaf_edit n=8 leaf=.claude/docs/tests-summary.md line=29 class=stale
leaf-text: as landed `tests/tui_wheel.rs` + `tests/cli_wheel.rs` on all three OSes (Windows: an injected mouse report takes the wheel, F-W3)
source: fct-3r2a46vd «cover tui, cli and ipc-internal (the wrapper channel) in CI on `ubuntu-latest`, the focus case holding the full assertion»
replace-with: as landed `tests/tui_wheel.rs` + `tests/cli_wheel.rs` in CI on `ubuntu-latest`
@leaf_edit n=9 leaf=.claude/docs/tests-summary.md line=30 class=retired
leaf-text: `tests/cli_instance_state.rs`; the marker equality on Unix only, no override and no command on Windows; `budget.json`
source: fct-ze6ljnvw «the Windows and macOS code, tests, fixtures and CI jobs are not in the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: `tests/cli_instance_state.rs`; `budget.json`
@leaf_edit n=10 leaf=.claude/docs/tests-summary.md line=30 class=stale
leaf-text: green on all three CI runners
source: fct-kylcwwrw «landed at chunk 2026-10-11-the-board-viola-list as `path6_list_shows_the_reading_and_the_pause` (`tests/cli_budget.rs`), green in CI.»
replace-with: green in CI
@leaf_edit n=11 leaf=.claude/docs/tests-summary.md line=30 class=retired
leaf-text: A case that needs a home viola created on every OS starts from `stamped_home`; a home `seed_conpty` made passes the instance check on Windows x64, and one a test makes with `create_dir_all` is refused
source: fct-bxlupfu3 «A home made with `create_dir_all` before viola is refused (0755) by the read rule. A case that needs a home viola created starts from `stamped_home`»
replace-with: A case that needs a home viola created starts from `stamped_home`; a home a test makes with `create_dir_all` is refused by the read rule
@leaf_edit n=12 leaf=.claude/docs/tests-summary.md line=33 class=stale
leaf-text: the server-verification cause in `tests/security_negatives_server_identity.rs` on Linux and Windows
source: fct-ndx7lm2p «beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs`»
replace-with: the server-verification cause in `tests/security_negatives_server_identity.rs`
@leaf_edit n=13 leaf=.claude/docs/tests-summary.md line=33 class=stale
leaf-text: (the Unix socket directory refusal in `tests/security_negatives_socket_dir.rs`, Linux and macOS)
source: fct-hhwc4gim «As landed: `tests/security_negatives_socket_dir.rs`, a Unix-only binary»
replace-with: (the Unix socket directory refusal in `tests/security_negatives_socket_dir.rs`, a Unix-only binary)
@leaf_edit n=14 leaf=.claude/docs/tests-summary.md line=42 class=stale
leaf-text: the 2 missed in `prepare`, which now takes its host flag from `mutate()` (`scratch::HOST_SCRATCH`) so both arms are reached on every host
source: fct-ocdr3xdn «Its 2 missed are in `prepare` (`crates/viola-e2e/src/harness/run/mutants/scratch.rs`, deleted with the Windows scratch wipe and recoverable at the tag `multi-os-final`)»
replace-with: the 2 missed in `prepare` (`crates/viola-e2e/src/harness/run/mutants/scratch.rs`, deleted with the Windows scratch wipe and recoverable at the tag `multi-os-final`)
@leaf_edit n=15 leaf=.claude/docs/tests-summary.md line=42 class=stale
leaf-text: the 2 `prepare` rows as recorded (killed on every host since)
source: fct-ocdr3xdn «the 2 `prepare` rows above stood as recorded (read killed on the Linux dev host, above)»
replace-with: the 2 `prepare` rows as recorded (read killed on the Linux dev host)
@leaf_edit n=16 leaf=.claude/docs/tests-summary.md line=42 class=retired
leaf-text: On a Windows host the run's temp copies and `mutants.out/` live in the host mutation scratch `<repo parent>/viola-mutants-scratch` (guarded, wiped first, `scratch_bytes` reported); each run's
source: fct-ocdr3xdn «a NOCOW (`chattr +C`) btrfs dir set by the operator (an environment fact; the harness sets no scratch dir of its own)»
replace-with: Each run's
@leaf_edit n=17 leaf=.claude/docs/tests-summary.md line=42 class=retired
leaf-text: its Windows leg the dispatch-only, report-only `windows-mutants.yml` (`--package <member> --file …` per matrix item on `windows-2025`: ten `mutants (<label>)` jobs, the root package split by file into five, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`; run 38036448183, the one dispatch made outside the audit, on the founder's word for chunk 2026-10-10-windows-mutation-grade, read all nine green over 642 mutants with 30 host-excluded)
source: fct-ocdr3xdn «The boundary audit has no Windows leg: `windows-mutants.yml` and the `cfg(windows)` code it measured on `windows-2025` are out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: with no Windows leg (`windows-mutants.yml` and the `cfg(windows)` code out of the tree, recoverable at the tag `multi-os-final`)
@leaf_edit n=18 leaf=.claude/docs/tests-summary.md line=42 class=stale
leaf-text: the two real-cargo-mutants harness self-tests are compiled out on macOS (runner-side, measured)
source: fct-ocdr3xdn «The two real-cargo-mutants harness tests (`run_mutants_passes_when_the_change_is_tested`, `run_mutants_reports_survivors_of_an_untested_change`) run ungated, never `#[ignore]`»
replace-with: the two real-cargo-mutants harness self-tests run ungated, never `#[ignore]`
@leaf_edit n=19 leaf=.claude/rules/testing.md line=18 class=retired
leaf-text: (Unix modes; on Windows `icacls`, its exit asserted)
source: fct-t5v77dkt «`tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes).»
replace-with: (Unix modes)
@leaf_edit n=20 leaf=.claude/rules/testing.md line=18 class=stale
leaf-text: (`SOCKET_ROOT_VAR`, `TMPDIR` on macOS, else `XDG_RUNTIME_DIR`;
source: fct-jfjmhkwu «`tests/support/home.rs`, under `/tmp`: `SOCKET_ROOT_VAR` (`XDG_RUNTIME_DIR`), `SocketRoot`»
replace-with: (`SOCKET_ROOT_VAR`, `XDG_RUNTIME_DIR`;
@leaf_edit n=21 leaf=.claude/rules/testing.md line=18 class=stale
leaf-text: (`attach(pid)` and `Drop`: on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop; on Windows a debugger attach, detached on drop)
source: fct-hckuzazj «`frozen.rs` holds `Frozen`, the one freeze guard (`attach(pid)` and `Drop`): `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop.»
replace-with: (`attach(pid)` and `Drop`: `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop)
@leaf_edit n=22 leaf=.claude/rules/testing.md line=19 class=stale
leaf-text: (rmcp client; a numbered Path binary, e.g. `path6_budget_harness.rs`) on all three CI OSes, plus
source: fct-shyrfs6v «OS: `ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs), the one leg of `strategy.matrix.os` in `test`, `perf`, `lint` and `release`»
replace-with: (rmcp client; a numbered Path binary, e.g. `path6_budget_harness.rs`) on `ubuntu-latest`, CI's one leg, plus
@leaf_edit n=23 leaf=.claude/rules/testing.md line=19 class=omission
leaf-text: a `GATES` row with its reason, an unnamed or stale gate failing the run)
source: fct-gldxhydz «an unnamed or stale gate failing the run, and a gate whose predicate names `windows` or `target_os = "macos"` a fault even when a row names it»
replace-with: a `GATES` row with its reason, an unnamed or stale gate failing the run, and a gate whose predicate names `windows` or `target_os = "macos"` a fault even when a row names it)
@leaf_edit n=24 leaf=.claude/rules/testing.md line=33 class=retired
leaf-text: so viola creates it (Windows protected DACL);
source: fct-ze6ljnvw «the Windows and macOS code, tests, fixtures and CI jobs are not in the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: so viola creates it;
@leaf_edit n=25 leaf=.claude/rules/testing.md line=33 class=retired
leaf-text: whose home the product refuses (Unix by the read rule; Windows by an inherited write grant to a SID strict-modes does not trust, read by its right whichever SID holds it) and CI's
source: fct-54gkcbby «never `create_dir_all`: a plain home is one the product refuses by the read rule.»
replace-with: whose home the product refuses by the read rule, and CI's
@leaf_edit n=26 leaf=.claude/rules/testing.md line=33 class=retired
leaf-text: The one carve-out: on Windows x64 root tests seed the ConPTY companions through `tests/support/home.rs` `seed_conpty` before `viola run` (test-plan §5); `run_viola_unseeded` starts and `tests/conpty_sideload.rs` never seed, so the product's first-start write stays covered; `seed_conpty` makes the home and the seed dir through `viola_state::fs::create_private_dir` first, so a seeded home passes the instance check; its companions are hard links to one shared seed file, so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded; a case that needs a home viola created on every OS starts from `stamped_home`. The second (the founder's ruling, 2026-10-04):
source: fct-wwnk7u4j «One carve-out: `tests/chaos_feed_panic.rs` alone boots outside the scans.»
replace-with: A case that needs a home viola created starts from `stamped_home`. The one carve-out (the founder's ruling, 2026-10-04):
@leaf_edit n=27 leaf=.claude/rules/testing.md line=34 class=stale
leaf-text: save the server-identity impostor cases' test-built snapshot, Linux and Windows, recording
source: fct-rgx56qfp «The server-identity impostor cases (`served` in `tests/security_negatives_server_identity.rs`, `Impostor::serve` in `tests/hook_fail_open.rs`) write an `InstanceSnapshot` themselves»
replace-with: save the server-identity impostor cases' test-built snapshot, recording
@leaf_edit n=28 leaf=.claude/rules/testing.md line=38 class=stale
leaf-text: (the child's own bytes on Unix, no viola literal on every OS)
source: fct-ktlsbc3h «a byte search of the drained master reader's stream (the child's own bytes, no viola literal), never a parse of the rendered screen.»
replace-with: (the child's own bytes, no viola literal)
@leaf_edit n=29 leaf=.claude/rules/testing.md line=39 class=retired
leaf-text: An oracle for a runner-inherited Windows grant names no SID (`windows-2025` runners differ by host in which SID holds a temp dir's inherited write): it reads the grant by its right, beside a control that cannot pass vacuously. A test-started
source: fct-ze6ljnvw «the Windows and macOS code, tests, fixtures and CI jobs are not in the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: A test-started
@leaf_edit n=30 leaf=.claude/rules/testing.md line=46 class=retired
leaf-text: `mutants.out/outcomes.json` — in the host mutation scratch `<repo parent>/viola-mutants-scratch` on a Windows host — never the exit code alone
source: fct-hgewdmj2 «The run keeps the inherited temp dir and writes `mutants.out/` at the repository root.»
replace-with: `mutants.out/outcomes.json` at the repository root, never the exit code alone
