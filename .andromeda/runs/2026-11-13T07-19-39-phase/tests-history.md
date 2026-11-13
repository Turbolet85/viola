@history_item n=1 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** (internal gate subcommands) — declared `secret-scan` as an internal harness subcommand with its output shape, reasons and a closed class enum. Item 9 removes the `cookie` class, so it edits a closed enum this entry set up. Every site that lists the classes has to move together.

@history_item n=2 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** (the GUI surfaces leave 0.1.0) — kept the `token-query` / `gui-token` / `cookie` scan classes and the `cookie_file` field as harness leftovers, owned by "Remove the browser leg". This is the origin of the `cookie` class that item 9 / `v2-06` now removes.

@history_item n=3 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (§3 harness: the GUI scan forms leave) — `secret-scan` classes dropped to `claude-stripped`, `cookie` (the case-insensitive `cookie:` header), `content-canary`, `mode` and `unreadable`. The `token-query`, `gui-token` and `viola_<digits>=` forms were removed, and an older session record with `ui` / `cookie_file` still parses. This is the direct precedent for item 9: a scan class taken out while every other class keeps catching. The companion Log format entry kept the "no line may contain a `Cookie` header" rule. This chunk's tests retire that rule's witnesses, and the master change belongs to the wrap.

@history_item n=4 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the after-every-test error-body check, replaced and not built) — made `viola-harness secret-scan` the holder of the no-`Cookie`, stripped-value and canary clauses, after each job's end-to-end run, and recorded that `secret_scan.rs` reads `cookie:` whatever its case. Item 9 removes the cookie half of these clauses, and the stripped-value and canary halves must keep their holders.

@history_item n=5 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration) — built `tests/contract_error_bodies.rs`: its tables, its census over `PRODUCT_ROOTS` with the `andromeda:walks-tree` token, and the guards `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing` and `census_finds_an_unnamed_writer_and_an_interpolated_message`. Item 1 replaces its hand scanner `blanked`. These guards and the census self-check must give the same verdicts after the swap.

@history_item n=6 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration) — built `tests/contract_bounded_readers.rs`. Its census leaves comments, strings and test items out, takes a `take(` in the call's own statement, and matches an `EXCEPTIONS` needle against the whole source line. It has the self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one`, which reads red at exactly `crates/viola-state/src/fs.rs:354` when the bound is undone. That census holds the `code_only` scanner that item 1 replaces. The masking flags (comments, strings) and the whole-line needle rule are behaviour the shared scanner must keep.

@history_item n=7 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the bounded readers' cap-driving witnesses) — the five `BOUNDARIES` rows (statusline shell-out, the child pipe drain, the thresholds / dialog-hold / `claude_env_keep` readers) now name cap-driving cases. These are the witnesses `boundaries_each_name_a_bounded_read_that_stands` resolves through the census. They must still resolve after the scanner swap and the file splits.

@history_item n=8 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the census of OS-gated tests) — built `tests/contract_os_gated_tests.rs`: the `GATES` table, its walk over `tests/` and `crates/*/tests/`, and the attribute-run reading of `#[cfg]` / `#[cfg_attr]` outside string values. Its tests are `os_gated_tests_are_each_named_with_their_reason`, `gates_table_rows_are_unique` and the self-check with eight planted cases. Item 1 replaces this census scanner. Splitting files moves gated tests, and every moved gate needs its `GATES` row re-pointed.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — `faults` reads three ways: unnamed gates, stale rows, and gates naming `windows` / `target_os = "macos"`. Its parse catches a multi-line `cfg_attr` predicate that a line grep cannot see, and the self-check builds the Windows word from a `WINDOWS` const. The count is 39 functions + 2 files = 41 `GATES` rows. The shared scanner must keep the multi-line predicate reading. The trap: a planted literal must not trip the census itself.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only) — Viola is Linux only, and the Windows/macOS code, tests and CI are out of the tree, recoverable at `multi-os-final`. This is the basis for item 10 (the dead `#[cfg(not(unix))]` constants in `crates/viola-e2e/src/harness/run.rs`) and for `decision/linux-only`.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (mutation without the Windows leg or the host scratch) — `scratch.rs` / `HOST_SCRATCH` were deleted. The dev host's `TMPDIR` scratch stays an operator fact, and the harness sets none. Item 8 names the host's quota'd temp root in one place, so it must not turn this into a harness-set `TMPDIR` without a recorded decision. The two real-cargo-mutants harness tests run ungated.

@history_item n=12 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (`run --mutants --package`) — on the Linux dev host every mutation run takes a NOCOW btrfs `TMPDIR`, because of the `/tmp` quota and the reflink exec-bit loss. This is the measured basis behind item 8's "quota'd `/tmp` tmpfs" (Epoch 6 L5). The boundary tier scores a whole member with `run --mutants --package`.

@history_item n=13 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — on the dev host `target/e2e-home` is an operator-set link to an owner-only tmpfs (`/tmp/viola-e2e-home-<uid>`). `ensure_e2e_home` / `prepare_home_base` check the linked base, and deletions go through the link, a boundary widening the founder ratified. This is the standing contract behind `decision/dev-host-e2e-home-on-tmpfs`, `e2e-home-link-base` and `e2e-home-deletion-through-link`, and item 8 must not break it.

@history_item n=14 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the headroom floor) — a linked base with less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` is refused. The decision is the pure `headroom_refusal`, with one compiled constant and no env/flag/config. This is the code of item 5's two `free_bytes` survivors (`crates/viola-e2e/src/harness/mod.rs`). It is also the earlier precedent for an epoch cleanup, alongside its named mutant survivors.

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the e2e-home headroom read on an injected figure) — `ensure_e2e_home` passes the free-bytes reading to a private function, so `harness::tests::e2e_home_*` drive it on an injected figure. This is the seam item 5 uses to kill the `free_bytes` survivors with a test, adding no env, flag or feature.

@history_item n=16 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — there is no per-chunk, pre-push or CI mutation gate: mutation runs at the epoch-boundary audit and on demand through `run --mutants`. The verdict rule (`missed == 0`, `timeout == 0`, `unviable <= caught`) is unchanged. This matches the directive that this chunk makes no mutation run. Rows `decision/mutation-at-epoch-boundary` and `mutation-gate-cadence` rest on it.

@history_item n=17 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — sets the precedent for closing boundary-audit survivors without a mutation run. Each survivor is disposed by a nextest case in the package that owns the mutated file, some after a behaviour-preserving restatement, each shown red under its hand-applied mutant and green restored. A ledger goes to `evidence/survivors.ndjson`, and the next boundary audit confirms the kills. Item 5 follows this form.

@history_item n=18 marker=2026-10-13-epoch-4-survivors-ii
**2026-10-13-epoch-4-survivors-ii** — 16 survivors were disposed after restatements (named predicates, generic over the seam with a test-only recording double, plain functions), each killed by a case in that module's tests with no mutation run. This is the pattern for item 5's restatements, and `decision/mutants-tested-by-owning-package` holds it.

@history_item n=19 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (`prepare` killed on every host, the Epoch 5 survivors disposed) — 16 survivors were disposed, 10 killed and 6 after a restatement, each red under its hand-applied mutant and green reverted byte-exact. None was marked equivalent. This is the latest cleanup precedent for item 5. Here item 5 marks `stdin.rs:15` `CHUNK` as equivalent, which departs from "none equivalent" and needs its reason recorded.

@history_item n=20 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** (host-excluded missed mutants) — a missed mutant whose span sits under a `cfg` proven false for the host is left out as `host_excluded`. Anything the reader cannot read stays counted. This is the rule behind `decision/host-unreachable-mutants` and `host-excluded-mutants`. Item 5's "recorded exclusion" of the `viola-mcp-tap` must be a recorded scope exclusion, not this host exclusion.

@history_item n=21 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the always-admit survivor as 0.1.0's known limit) — the client-seam always-admit survivor (matrix#v1-44) stands as the version's known limit, read at each epoch-boundary audit. This is the one standing tolerated survivor (`decision/mutation-run-verdict` except clause). Item 5 must not fold it in with the named survivors.

@history_item n=22 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange) — the test-only `viola-mcp-tap` records `Mcp.<variant>.json` through the `--record` scrub and refusal, with four closed codes naming the file only. `mcp_fixtures` walks it, and `tests/mcp_recorded_exchange.rs` replays it. This is the tap whose 19 survivors item 5 closes by tests or a recorded scope exclusion. Rows `decision/mcp-exchange-recorded-by-tap` and `mcp-tap-refuses-dirty-exchange` bind its behaviour.

@history_item n=23 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap's tests) — this entry sets up `CappedLines` at `MAX_FRAME`:
- A line past the cap passes no byte and ends the session with one `parse-rejected{mcp-stdin, oversize, count:1}` WARN line.
- The unit `stdin::tests` cover the six `capped_lines_*` cases.
- The `serve` child cases and `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` cover the session end.

Item 5 targets the `lib.rs:95` (assert the WARN line) and `stdin.rs:84` (a buffer smaller than the pending line) survivors in exactly this code.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`viola mcp`'s exit at a line past the cap before `initialize`) — the unit child exits 11 while `viola mcp` itself exits 1, with both streams empty and one `parse-rejected` line. The holders are `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` and the E2E case. This is the context for the viola-mcp `lib.rs` WARN-line survivor.

@history_item n=25 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (scoped log captures through `with_scoped_default`) — the root bin's scoped captures go through `test_support::with_scoped_default`, so they don't lose a line whose callsite another thread registered first. The crate-level captures, including viola-channel and viola-state, run on a bare `with_default`. They have never been seen red and have no owner. This is a trap for item 5's new viola-mcp test asserting the WARN `parse-rejected` line. `decision/one-global-tracing-install` binds it.

@history_item n=26 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy's no-clobber placement) — the unit cases cover `fs::place_private_new` (the mode set before any byte, `Placed::Present`, the missing-dir failure) and `pin::place_absent` (same bytes reused, other bytes `HashMismatch`). The `placed` hook is a test parameter, not a seam. This is the surrounding code and test form for item 5's viola-state `fs.rs:123` / `pin.rs:91` error-kind-guard survivors.

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (a home a test makes before viola) — a test home made before viola goes through `viola_state::fs::create_private_dir`, never `create_dir_all`, which the read rule refuses at 0755. This is the rule behind `decision/test-homes-made-private`. It is also why the 19 Epoch 6 tests reddened, and item 6's one viola-made-home helper must make homes this way.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-widening helpers) — `tests/support/home.rs` holds `widen` · `grant_read` · `unwiden` · `still_widened`. Widened homes are set by the test on the real file and refused by the product's own check. These are the existing shared home helpers, so item 6's helper belongs beside them and must not be a copy.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness. The G4/G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, because one unbounded witness hung 27 minutes under nextest's default profile. This is `decision/bounded-remove-the-guard-witness`, and item 7's one remove-the-guard helper must carry the bound.

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a test's endpoint directory is made private) — a test placing an endpoint in its own directory makes it through `create_private_dir`, never a bare `tempfile::tempdir()`, which reads 0755 and is refused by `check_endpoint_dir`. On first run, nine `handlers` cases were refused. This rule (`decision/private-dir-for-test-endpoints`) extends item 6's home helper to endpoint dirs.

@history_item n=31 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the bare temp-dir home and the liveness-check case) — a bare temp dir reads 0755 and the read rule refuses it. `tool_inputs_that_pass_reach_the_liveness_check` now starts from a viola-made `empty_home`. The trap: an oracle moved by a shared body can hide a test that never reached its subject. Item 6's re-homed tests must still reach what they test.

@history_item n=32 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the shared root helpers named) — the helper list is `tests/support/{home.rs,…,cli.rs,ndjson.rs}`, and no root test file outside `tests/support/` defines its own copy. This is the origin of `decision/one-shared-test-helper` / `shared-e2e-helpers`, which items 1, 6 and 7 extend. The scope names the test plan's one-shared-test-helper decision as touched.

@history_item n=33 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the Frozen guard's one home) — `Frozen` moved whole to `tests/support/frozen.rs`, its sole home, declared in `tests/support/mod.rs`, and the helper set and the no-copy rule were extended. This is the precedent for lifting local twins into one support module, which items 1, 6 and 7 repeat.

@history_item n=34 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the root `cross_` binary and the shared start helpers) — `tests/support/start.rs` shares the start setups, because a root test file holds no copy of its own of what two files share. This is the existing home for start helpers next to which item 6/7's new helpers land.

@history_item n=35 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (moved unit tests re-cited after the send and human split) — a pure-move split of `src/run/send.rs` and `src/human.rs`. Every test kept its name and cases (223 test lines before and after), and the masters re-cited the new paths. This is the precedent for item 3's splits (test names and case count unchanged), and cited paths must follow the move.

@history_item n=36 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the moved units re-cited) — a size split moved the board units to `src/human/board/` and the typed module to `src/cmd/verify/typed/`, keeping every case's name. The orphans probe line was corrected to `2/2 fired`. This is the latest over-800-line split precedent. Item 3's splits create new modules, which `decision/cargo-modules-orphans-gate` / `no-orphan-modules` must still pass.

@history_item n=37 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (`run --e2e`, the E2E regex) — the E2E layer is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, and the integration layer is its complement. If item 3 splits `tests/cli_verify.rs` or adds a support binary, the new binary's prefix decides its layer (`decision/e2e-layer-by-binary-prefix`, `integration-binary-surface-prefix`).

@history_item n=38 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the bootstrap phases) — `[profile.ci]`'s 20 s kill line lists twenty-six verify-driven binaries, counted by the `binary(…)` names on that filter line. `cli_verify` is on it. If `tests/cli_verify.rs` splits into more than one binary, each new binary must join the filter line and the key file's list and count. Otherwise its verify-driven tests lose `decision/ci-stamped-home-kill-line`.

@history_item n=39 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the `verify_window_` kill at 60 s) — the `verify_window_` override stands first at 60 s in both `ci` and `mutants`, held by name. Tests in `tests/cli_verify.rs` moved by item 3 must keep the `verify_window_` prefix so they keep their kill line (`decision/verify-window-60s-kill`).

@history_item n=40 marker=2026-10-10-session-links
**2026-10-10-session-links** (links in the replay) — the replay recovers `links` through `viola_state::replay::apply_link_line` with inline `link_rule_*` cases, and a `link` line with no string `ts` gets `since: "unknown"`. This is one side of item 2's `src/run/links.rs` ↔ `src/run/snapshot.rs` duplicate. The merged function must keep this rule and `decision/link-since-kept`.

@history_item n=41 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the served `link` / `unlink` methods) — this entry names the `src/run/links.rs` unit cases (`links_set_method_*`) and `methods_route_link_and_unlink_to_the_link_set`, together with the `-32602` / `-32603` contracts. They pin the `links.rs` side of item 2's merge (`contract/link-method/*`), and it must leave their verdicts unchanged.

@history_item n=42 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check on its event and dialog paths) — `canonical` / `trusted` moved into `src/cmd/hook.rs`, shared by both arms and run before obs init. `hook_refused_by_the_instance_check_fails_open_and_writes_nothing` holds the order. Item 2's `hook.rs` 1075↔1152 merge sits in this event/dialog-path code. The instance-check-first order (`decision/hook-instance-check-first`, `hook-instance-check-before-config`) must survive.

@history_item n=43 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the read rule and the instance check's widened set) — this entry sets `check_instance`'s set and its three call sites (`hook.rs` `trusted`, `client.rs` `refuse_untrusted`, `revive.rs`), along with the write and read rules in `strict.rs`. This is the code beside item 4's dead `check_present` (`crates/viola-state/src/strict.rs`). Deleting it or wiring it must leave `decision/strict-modes-two-rules` intact.
