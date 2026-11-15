@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (nextest mutants profile and toolchain source) — first set `[profile.mutants]` (`fail-fast` + a 15 s × 2 slow-timeout) because a caught mutant was graded Timeout under plain `fail-fast = true`; the origin of the kill-below-the-timeout reasoning item 2 re-sizes.

@history_item n=2 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — `[profile.mutants]` slow-timeout set to 5 s × 2 with a `package(viola-e2e)` override at 15 s × 2, and cargo-mutants' 20 s auto-timeout floor recorded as measured; why: wait consumers spinning to a 20 s bound under the 20 s floor. This is the 10 s kill item 2 moves and the floor its new margin must stay under (rows mutants-kill-below-cargo-mutants-timeout, root-wait-bound).

@history_item n=3 marker=2026-09-27-browser-verdict-reachability
**2026-09-27-browser-verdict-reachability** — the root `booted_wrapper` bound set to 7 s, `tests/support/watch.rs` `WITHIN`, chosen below the 10 s mutants kill by a remove-the-guard pair (7 s FAIL with the report, 12 s TIMEOUT with the output lost). Item 3's `WITHIN` and its doc comment tying it to the 10 s kill come from here; items 2 and 3 must move together and keep that ordering (rows root-wait-bound, within-7s-wait-bound).

@history_item n=4 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — `run --mutants` builds in its own target dir `target/mutants`, because a copied default `target/` kept `CARGO_BIN_EXE_*` paths and tests drove the unmutated binary (row mutation-own-target-dir). The scratch tree whose path lengthens the socket name (item 1) is this copied tree.

@history_item n=5 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — mutation moved to the epoch boundary: no per-chunk, pre-push or CI gate, `run --mutants` named-only, verdict rule unchanged; the two real-cargo-mutants harness tests kept at the 120 s kill with `retries = 0`. Grounds the chunk's "no mutation run inside the chunk" and the rows mutation-at-epoch-boundary, mutation-not-a-ci-gate, mutation-harness-tests-ungated.

@history_item n=6 marker=2026-09-24-epoch-1-cleanup
**2026-09-24-epoch-1-cleanup** — the counted verdict also needs `unviable <= caught` (`unviable-exceeds-caught`). The mutation-gate-thresholds and mutation-run-failure-conditions rows the next audit's green baseline is judged by.

@history_item n=7 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — the `run --mutants --file` scoped arm (`verdict:"scoped"`, never a leg verdict). Row scoped-mutants-not-the-verdict: a file-scoped run is the inner fix loop, not the verdict. The chunk is barred from running even that, so survivor kills are shown by hand.

@history_item n=8 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (run --mutants --package; the boundary tier's form) — the boundary tier scores a whole member with `--package`. A host-unreachable mutant is "not measured here; owed to …", never "equivalent". On the Linux dev host every mutation run takes a NOCOW btrfs `TMPDIR`. That scratch `TMPDIR` is the one the 7-digit-pid socket path overflows under (item 1, architecture row `mutation-scratch-nocow`, not amended).

@history_item n=9 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** (the mutants profile waits for running tests) — `[profile.mutants]` `fail-fast` became `{ max-fail = 1, terminate = "wait" }` with slow-timeout unchanged. Restated reason: the slow-timeout kill bounds any hang below cargo-mutants' 20 s floor, so a caught mutant ends at its kill line and is never graded Timeout (row mutants-fail-fast-wait). Item 2 must keep `terminate = "wait"` and that bound.

@history_item n=10 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the `profile.mutants` overrides grew from `viola-e2e` alone to also `test(/send_window_/)` and `test(/verify_window_/)`. This is the per-class override structure a slow-class override for item 2 would join.

@history_item n=11 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the `verify_window_` kills moved with the 8.5 s gate bound. Standing rule: a kill line moves only for a floor that is by design, with a planted-hang control recorded both ways. Kept: `WITHIN` 7 s and the mutants profile's own 10 s kill. A constraint on how item 2 may re-size the kill.

@history_item n=12 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `mutants` override order and the `verify_window_` class) — in `[profile.mutants]` the first matching override wins, so the `verify_window_` override stands first, pinned by `mutants_verify_window_override_stands_before_the_harness_package_override`. A planted-hang control read TIMEOUT under mutants and PASS under default. A precedent for the inheritance and first-match premise the scope says to measure at P3, and for adding a class by name rather than its own override.

@history_item n=13 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the verify_window_ kill at 60 s) — `[profile.mutants]` holds four overrides, first match winning: `verify_window_` 20 s × 3 first, then `package(viola-e2e)`, `send_window_`, `send_under_the_paste_hint_` at 15 s × 2. The paste-hint class was pinned by `mutants_paste_hint_override_holds_the_class_above_the_default_kill`. Planted-hang TIMEOUT/PASS pairs were recorded, and each kill moved with its pair, "never by raising the default". The closest precedent for item 2's ~5 s class (rows send-window-paste-hint-30s-kill, verify-window-60s-kill, viola-e2e-30s-mutants-kill).

@history_item n=14 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — `WITHIN` is "shared by the root waits on a child", counted by the pattern `Instant::now() + WITHIN` (22 in 16 then); `tests/support/{…,cli.rs,…}` named as the one shared helpers. The count's rule item 3 must re-read if it changes `WITHIN` or adds a per-call bound.

@history_item n=15 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the fake agent's resume payload and ten options; the root waits at 26 in 19 and the kill path's endpoint wait) — the kill path waits for the wrapper's exit, then the child gone, then `holder_gone`. `wait_endpoint_gone` after a kill was disproved (a killed wrapper leaves its socket file). This is the root-wait design that shares `WITHIN` (row killed-wrapper-endpoint-wait).

@history_item n=16 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify_stopped_by_a_signal_…` landed, run through `verify_started` and signalled once the receipt shows its lines. The receipt watch reuses `fake::wait_for`, so the `WITHIN` count was unchanged. That is the `fake::wait_for` / `WITHIN` path item 3's red runs through (`tests/support/watch.rs:54`), and it backs the scope's premise that the bound is shared by every `fake::wait_for` caller.

@history_item n=17 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` (Run E). This is the case red twice in the audit's coverage suite on `timed out waiting for Run E's start` (item 3, M2).

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the bootstrap phases and test data bootstrap without the Windows arms) — the latest count of root waits: `Instant::now() + WITHIN` at 31 sites in 23 files. `unconnectable` / `holder_gone` read a refused connect or an absent path. This is the count's last reading, which item 3 moves by its rule if it touches the waits.

@history_item n=19 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the root waits' count at 32 sites in 24 files) — `WITHIN` sites re-counted by the stated `grep` rule at a wrap. A precedent for restating the count when item 3 changes the bound.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness, because nextest's default profile has no kill line (an unbounded witness hung 27 min). Row bounded-remove-the-guard-witness. The directive's `gate.py control` runs for the hypr-tap survivors must sit under a kill deadline for this reason.

@history_item n=21 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (the Windows listener's admission negatives) — Kept: the tests' `viola-test-chan-<pid>-<label>` endpoint names stay architecture's registered namespace. This is the name item 1 changes in `tests/hook_fail_open.rs`. A new name must stay inside that registered namespace or be squared with architecture.

@history_item n=22 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the hook fail-open matrices' impostor rows; the impostor's listener) — `tests/hook_fail_open.rs` gained the impostor rows (`case::impostor`, `question_impostor`, `permission_impostor`) that bind a test-owned `viola_channel::Server` on a test endpoint (`Impostor`). These are the cases whose labels set the longest socket name item 1 must bound (rows hook-fails-open, server-identity-verified, hook/impostor-server).

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a test's endpoint directory is made private) — a test placing an endpoint in its own directory makes it through `viola_state::fs::create_private_dir`, never a bare `tempfile::tempdir()`, because tempfile leaves 0755 and `check_endpoint_dir` refuses it. `tests/hook_fail_open.rs` is a named site. Rows private-dir-for-test-endpoints, test-endpoint-dirs-made-private, socket-directory-check: item 1's shorter socket path must keep its private directory.

@history_item n=24 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (a home a test makes before viola) — a test-made home goes through `create_private_dir`, never `create_dir_all`, or the read rule refuses it (row test-homes-made-private). Item 5's home-naming helper in `tests/support/home.rs` must keep this.

@history_item n=25 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `prepare_home_base`, called from `TestHome::new`, holds the e2e-home base contract (a link with an absolute target, owner-only). On the dev host `target/e2e-home` links to tmpfs `/tmp/viola-e2e-home-<uid>`. This is the helper chain item 5 binds the record-home naming rule into, and it bears on where test homes and paths land (rows e2e-home-link-base, e2e-home-base-checks, e2e-homes-under-one-root).

@history_item n=26 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — survivors disposed by a nextest case in the package that owns the mutated file, some after a behaviour-preserving restatement. Each was shown red under a hand-applied mutant and green restored, with no mutation run; the next boundary audit confirms. Also: two cargo-mutants jobs under one `TMPDIR` shared a report file, a scratch trap. This is item 4's method precedent (row mutants-tested-by-owning-package).

@history_item n=27 marker=2026-10-13-epoch-4-survivors-ii
**2026-10-13-epoch-4-survivors-ii** — the 16 `typed.rs` survivors were killed after named-predicate / generic-seam restatements inside the file, each red under its hand-applied mutant, with no mutation run. A precedent for the restatements item 4 may need to reach `main`'s match arms.

@history_item n=28 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (`prepare` killed on every host, the Epoch 5 survivors disposed) — the Epoch 5 survivors were each disposed in the owning package: 10 killed and 6 after restatement, red under the hand-applied mutant, reverted byte-exact, none equivalent. This is the disposal form the next audit reads for item 4.

@history_item n=29 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange) — the precedent test-only tap `viola-mcp-tap` (`record` / `fixture`, scrub and refusal, the four closed codes), whose shape `viola-hypr-tap` (item 4's file) follows.

@history_item n=30 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — introduced the test-only `viola-hypr-tap`. Recorded Hyprland replies are refreshed only by it, under the same scrub, with a dirty set refused whole and a closed per-request keep-list held by the hygiene walk. The fake agent was added as hyprctl, and CI never touches the real hyprctl. These are the recorder's behaviours item 4's survivor-killing tests drive (rows fixtures-recorded-by-live-verify, recorder-scrub, dirty-payload-refuses-recording, fake-agent-as-claude, ci-never-real-claude).

@history_item n=31 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** — the `except` on fakes-for-external-services: viola-hyprland's own unit tests use a planted hyprctl stub, because cargo-mutants runs only the mutated package's tests. Direct precedent for item 4: `viola-hypr-tap`'s survivors must be killed by `viola-e2e`'s own tests, not root ones.

@history_item n=32 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the euid wiring, untestable and a known survivor) — a mutant no committed test can kill is written into the test plan (§1 untestable zone, §10 mutation gate) with its reason. This is the precedent for item 4's alternative: stating with a reason why the recorder's `main` is outside the tier (row mutation-run-verdict's except).

@history_item n=33 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the always-admit survivor as 0.1.0's known limit) — the known survivor stands as the version's known limit, read at each epoch-boundary audit. This sets the wording form a tier-text ruling on `main` would take.

@history_item n=34 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** (`run --mutants` leaves host-excluded missed mutants out) — the verdict reads `missed − host-excluded == 0 && timeout == 0 && unviable <= caught`, and a missed mutant is never hand-argued away. The audit judges item 4's survivors against that rule (row mutation-gate-thresholds).

@history_item n=35 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (mutation without the Windows leg or the host scratch) — `mutants.out/` sits at the repo root, the harness sets no `TMPDIR`, and the dev host's `TMPDIR` scratch stays an operator fact. The two real-cargo-mutants harness tests run ungated. Item 1's scratch path is this operator `TMPDIR`, not a harness value.

@history_item n=36 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — a measured corrupt-profile mechanism: an instrumented child killed during its exit-time profile write leaves a short profile, so the start test's child became uninstrumented `whoami`. Folded reds are closed by cause, never by a green re-run. This is the origin of row killed-test-child-uninstrumented, which bears on keeping the coverage suite green that the next audit reads (M1).
