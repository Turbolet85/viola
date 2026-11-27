@history_item n=1 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (moved unit tests re-cited after the send and human split) — the pure-move split of `src/run/send.rs` and `src/human.rs` re-cited every moved unit test at its new file (`src/run/send/budget.rs`, `classify.rs`, `relabel.rs`, `src/human/board.rs`), each keeping its name and cases, counted 223 test lines before and after. The direct precedent for this chunk's acceptance (count a moved test module's cases before and after, re-point named paths only); a master site naming a moved test's file is a wrap amendment.

@history_item n=2 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the moved units re-cited and the orphans probe line) — a size split moved code with every case's name kept, so the board units became `src/human/board/` and the stop's cases `src/cmd/verify/typed/`; the Epoch 4 dispositions were re-worded to name code "under `src/cmd/verify/typed/`". Also fixed the orphans probe line to `orphans-check probes: 2/2 fired, control clean`. Precedent for re-citing moved units and for the orphans gate a new module of `viola-pty` or `src/cmd/` must pass.

@history_item n=3 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (2. Test Strategy) — `+ decision/os-gated-cases-in-binary-root`: the `cli_verify` split kept its `cfg(unix)` cases in the root file, a rule binding every later split of a test binary. Constrains where gated cases go if `run.rs`'s or `pump.rs`'s test modules are split into more files.

@history_item n=4 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the census of OS-gated tests) — added `tests/contract_os_gated_tests.rs`, whose `GATES` table names `file`, `test`, `gate` and `reason` per row, and whose `faults` reads an unnamed gate and a row that no longer stands both ways. A gated test that moves file makes its row stale: the census is a register this chunk must re-point (guard of `decision/os-gated-tests-registered` / `decision/os-gated-tests-each-named`).

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault; the OS scope) — `faults` gained a third class, a gate naming `windows` or `target_os = "macos"` even when named, its parse catching a multi-line `cfg_attr`; the count was re-measured (39 functions, 2 files, 41 rows). The same chunk made the product Linux only with one `ubuntu-latest` leg and moved the PTY seam to openpty alone. Grounds `decision/linux-only` and the census rule any moved gated case must still satisfy.

@history_item n=6 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** (Lint row disposed) — the orphans gate `scripts/orphans-check.sh` (+ `--probe`) runs per lib/bin target, `--acyclic` an on-demand review, not a gate. Every module this chunk creates (the restart module of `viola-pty`, the moved window/trust step file) must sit in its `mod` tree or the gate fails the build (`decision/cargo-modules-orphans-gate`, `decision/no-orphan-modules`).

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the CI gates: the channel tree form, rmcp features) — the sync-crate `cargo check` set is `viola-core`, `viola-pty`, `viola-state`, `viola-agent-claude`, read from `scripts/sync-crates.txt`, each checked alone without tokio. The restart code moved within `viola-pty` stays inside a sync crate: no tokio may enter its new module (`decision/sync-crates-without-tokio`, `decision/tokio-banned-per-sync-crate`).

@history_item n=8 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** (Lint row from sync-crates.txt) — the sync-crate check reads one `-p` per crate in `scripts/sync-crates.txt`, an empty list failing, and the sole-root `deny-sync.toml` tokio ban runs per listed crate. The gates `viola-pty`'s split runs under.

@history_item n=9 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (pump resize baseline) — the pump's resize baseline became the spawn size, witnessed by a forced-window test through the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam (6/6 red before, 6/6 green after). The origin of `decision/resize-baseline-is-spawn-size` and its guard `pump_forwards_a_resize_that_lands_before_its_first_look`, a pump test this chunk's `pump.rs` test-module move must carry by name.

@history_item n=10 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (no-EOF measured per OS, harness on outer PTYs) — the no-EOF chaos mode and the rule that exit is read on the handle, never on EOF, with consumers tested against a reader that never ends. Background to `decision/child-exit-read-on-the-process-handle` and `decision/pty-exit-from-wait-not-eof`, which the pump's moved restart path and its tests must keep.

@history_item n=11 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the portable-pty pin at =0.9.0) — `portable-pty` moved to `=0.9.0`; viola-pty ran 33 of 33 and the outer-PTY e2e binaries stayed green, `pty_backend()` still `openpty`. A last recorded viola-pty test count beside which the pump move's before/after case count can be read.

@history_item n=12 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy's no-clobber placement) — the `placed` hook of `place_private_new_with` is a test parameter, not a seam: no env read, no feature, no mockall double, nothing added to the test-seam list. The source of `decision/forced-window-hook-is-test-parameter`; any forced-window hook carried by a moved function stays a parameter.

@history_item n=13 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (run --mutants --package; the boundary tier's form) — `run --mutants --package <member>` scores one whole member; viola-e2e scenarios run only against viola-e2e's own mutants. With `decision/mutants-tested-by-owning-package`, a moved `viola-pty` function's tests must stay in `viola-pty`, and a root function's in the root package, or its mutants go unkilled.

@history_item n=14 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** (8. Mocking & Stubbing Discipline) — `decision/fakes-for-external-services` gained the except that viola-hyprland's own unit tests use a planted stub because cargo-mutants runs only the mutated package's tests. The measured trap behind keeping each moved test in its owning crate.

@history_item n=15 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** (derived whole-chunk base) — the mutation base is derived from history as the last master flip before the oldest pre-CI commit of a pending chunk, so every run mutates the whole chunk. Origin of `decision/mutation-base-whole-chunk`: the scope notes a move changes the sites the next audit's diff reads, so the moved files are all of this chunk's mutation diff.

@history_item n=16 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — no per-chunk, pre-push or CI mutation gate; `run --mutants` runs only when named and at the epoch-boundary audit, the verdict rule unchanged. Grounds the scope's "no mutation run planned or run" against the older `decision/mutation-testing` row (per chunk) that the rows file still holds beside `decision/mutation-at-epoch-boundary`.

@history_item n=17 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** (host-excluded missed mutants) — a missed mutant whose whole span sits under a `cfg` proved false for the host leaves the missed count and is named in `host_excluded`; `survived` = missed − host-excluded + timeout. Origin of `decision/host-excluded-mutants`, `decision/mutants-survived-count` and the verdict-by-counts rows the next audit reads over the moved code.

@history_item n=18 marker=2026-10-13-epoch-4-survivors-ii
**2026-10-13-epoch-4-survivors-ii** (the 16 typed.rs survivors disposed) — survivors were disposed by behaviour-preserving restatements inside one file, each killed by a case in that module, shown red under its hand-applied mutant with no mutation run made. Precedent: a restatement that moves a site keeps its killing case beside it, the epoch-boundary audit confirming the kills.

@history_item n=19 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (`prepare` killed on every host, the Epoch 5 survivors disposed) — the boundary audit's survivors were each disposed by a nextest case in the package owning the mutated file, red under the hand-applied mutant and green reverted byte-exact, none recorded equivalent. The disposition form the moved functions' current kills (326 caught · 0 missed at `328c22d8`) rest on.

@history_item n=20 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the always-admit survivor as 0.1.0's known limit) — the always-admit `peer_admitted` survivor stands as the version's known limit, read at each epoch-boundary audit. The except of `decision/mutation-run-verdict`; the next audit reads it, not this chunk.

@history_item n=21 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** (10. Quality Gates & Coverage Targets; 3. Test Harness Contract) — `+ decision/equivalent-survivors-known-limit` (two equivalent survivors left as code for the next boundary's audit), `+ decision/harness-abort-wait-below-ci-kill`, and `decision/verify-window-80s-kill` widened to the fake agent dialog and framing replay tests. Known-limit register and kill-line rows beside this chunk's moves.

@history_item n=22 marker=2026-11-27-epoch-5-cleanup
**2026-11-27-epoch-5-cleanup** (3. Test Harness Contract: 2 row(s)) — the `verify_window_` class's filter line, identical in the `ci` and `mutants` profiles, now takes six verify-driven tests by name beside its prefix; `decision/verify-driven-tests-20s-kill`'s except covers the named members. The sibling chunk this one split from: a test named one by one on `.config/nextest.toml`'s filter line must keep the name the filter matches if its module moves, or it silently loses the 80 s kill.

@history_item n=23 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract) — `WITHIN` moved 7 s → 12 s and the mutants default kill 10 s → 15 s, below cargo-mutants' 20 s floor, a stated departure from fixing slow cases by removing slow work. The current values of `decision/mutants-kill-below-cargo-mutants-timeout` and the root wait bound that the moved tests run under unchanged.

@history_item n=24 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the served `link` / `unlink` methods) — names `methods_route_link_and_unlink_to_the_link_set` (`src/cmd/run.rs`) beside the `src/run/links.rs` units. A test-plan citation of a unit test by `src/cmd/run.rs`: if that case leaves the file with `run.rs`'s test module, the citation is an expected re-point.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the four readers on the replay) — `run`'s collision check and the other readers moved onto `read_snapshot_or_replay`, with the rule `read_snapshot(` under `src/`, outside test modules, held to no product caller. `decision/snapshot-read-through-replay`'s census: code moved out of `src/cmd/run.rs` keeps that rule, and a moved test module must stay a test module for the census to skip it.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the start's and verify's home-tree checks before any log) — Path 1's start runs `check_home_tree` before the wrapper log opens; `tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log` is the guard of `decision/run-checks-before-child-spawn` and `decision/widened-home-tree-refused-at-start`. The start order in `run.rs` this chunk's window/trust move must leave byte-for-byte in behaviour.

@history_item n=27 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (1. Test Scope Summary: 2 row(s); 4 row(s)) — `+ decision/run-start-verify-and-trust-after-bind` (the automatic verify after the bind, then on a verified final verdict the untrusted spawn folder's trust record, before the first snapshot) and `decision/human-never-blocked` gained the start's automatic verify window as its except. The exact steps this chunk moves out of `src/cmd/run.rs`; their order is the behaviour a pure move must keep.

@history_item n=28 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Test Scope Summary: 1 row(s), run-start-events order) — a start under `--origin` appends one window line with source wrapper after launch and before session-start; the same chunk added `decision/window-placement-proven-live` and made the fake agent stand as foot. The window step being moved writes that line; `contract/run-start-events/order` must read the same after the move.

@history_item n=29 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (3. Test Harness Contract) — `contract/boot-readiness/events` reads event lines 1–4 (wheel, lineage, launch, session-start); the harness readiness probe reads them. The harness boot path the plan's smoke trio exercises after `run.rs`'s shared start moves.

@history_item n=30 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** (6. E2E Test Strategy; 5. Integration Test Strategy) — `decision/window-placement-proven-live` names R7 as the one proof that a placing open keeps the view; `+ contract/verify-exit-detail/untrusted-folder` and `+ decision/verify-failing-row-from-fixture-set`. The last chunk to edit the window step: its R7 is live-only (out of this chunk's bounds), its fakes the tests a moved window step must keep green.

@history_item n=31 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (7. Test Data & Fixtures; 3. Test Harness Contract) — the trust step writes the Claude config, so `+ decision/run-under-test-owned-user-home`, `+ decision/harness-wrappers-under-session-user-home` and `+ decision/trust-start-folders-outside-repository` (trust folders under `/tmp`, a folder inside the workspace keying at its git root). The trust step's tests that move with it keep these homes.

@history_item n=32 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. E2E Test Strategy) — `decision/hint-names-its-cause` gained the except that viola up's `boot-time-unread` is held by its unit pair alone; `+ decision/window-reopen-proven-live` (R4 gated, R4b refusing); `decision/strict-modes-before-any-log` gained viola up's board scan. The rows binding `upped`'s plan/act split: the refusal order and the unit pair must survive it.

@history_item n=33 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (6. E2E Test Strategy: 3 row(s)) — adopt's start joined the silent roles at the internal-error catch site and shares revive's session-live reading. `run.rs` is the shared start of run, revive and adopt; the moved steps reach all three.

@history_item n=34 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json`'s ten codes and the closed gap) — a `pump_child` error prints no document, the child having held the terminal; a refusal `run`'s start makes after the preflight prints one exit-1 document under `revive --json`. Source of `contract/revive-json/pump-child-error` and `decision/revive-json-no-document-after-terminal`, on the start path the move touches.

@history_item n=35 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration in the Contract suite) — `tests/contract_bounded_readers.rs` holds `BOUNDARIES` rows naming each reader's bounded read in its file, checked by `boundaries_each_name_a_bounded_read_that_stands`, and an `EXCEPTIONS` needle matched on the call's source line. A reader moved out of `run.rs` or `pump.rs` makes its row point at the wrong file: a register to re-point.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration in the contract suite) — `tests/contract_error_bodies.rs` names every product error writer with a writer and witness that stand (`product_error_writers_are_each_named`), its census walking `PRODUCT_ROOTS`. A writer moved to another file must be re-pointed in its table (`decision/error-writers-each-named`).

@history_item n=37 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (5. Integration Test Strategy: 3 row(s)) — `+ decision/no-untied-viola-spawn` (a census over every test source, recursively) and `+ decision/tied-child-spawn-thread`. A moved test module that spawns viola must stay tied for the census.

@history_item n=38 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (5. Integration Test Strategy: 2 row(s)) — `+ decision/no-verify-start-outside-test-home`, a census over test sources with a closed exempt list, proven red on a planted untied start. Moved tests that start verify stay inside its reach.

@history_item n=39 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (run --e2e, the E2E regex widened, the integration complement) — the E2E regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, the integration layer its complement, the unit layer lib and bin targets (`decision/test-layer-filtersets`). Inline test modules split off a source file stay in the unit layer only while they remain `cfg(test)` modules of the crate (`decision/unit-tests-inline`).

@history_item n=40 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the shared root helpers named) — the shared root helpers live in `tests/support/`, and no root test file outside it defines its own copy. A constraint on any test helper a move would duplicate.
