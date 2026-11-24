@history_item n=1 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (6. E2E Test Strategy: 3 row(s)) — added decision/adopt-proven-live (real adopt, resuming a real ended session and refusing a real live one, is proven only by the gated live reading R5; the fake-agent rehearsal of `scripts/r5-adopt.sh` never counts as live). It also put adopt's start on contract/cli-human/internal-error's silent list, and extended decision/session-live-read-fails-open to adopt. Why it matters: this chunk serves R5a, which read red twice. A changed spawn folder or early-exit path is proven here over the fake agent only; the live proof stays R5's, fired later through `2026-11-24-viola-adopt`'s gated door. Any new early-exit output must not break adopt's start being silent at the internal-error catch site.

@history_item n=2 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (5. Integration Test Strategy: 1 row(s)) — put `viola adopt` on contract/strict-modes-refusal/no-line: a widened home or instance dir is refused before any log. Why it matters: any new adopt refusal or early-exit record must keep the strict-modes refusal lineless, ahead of the log it would write.

@history_item n=3 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (1. Test Scope Summary: 1 row(s)) — decision/real-claude-local-only gained «the gated live-reading scripts» as a local-only route to the real claude, never fired by a gate in live form. Why it matters: if `scripts/r5-adopt.sh` changes, its live arm stays local-only and gated. Its `--rehearse` form, which must reach the changed path with no claude, is the only form any gate may run.

@history_item n=4 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (7. Test Data & Fixtures: 1 row(s)) — added decision/child-argv-read-from-proc: a claim about the child's argv is read from the live child's `/proc` cmdline, because the fake agent's start receipt records no argv. Why it matters: a test proving where adopt starts its resumed child, or with which `--resume` argv, should read the live child's `/proc` and not trust the receipt. The receipt does carry a `cwd` kind.

@history_item n=5 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (3. Test Harness Contract: 1 row(s)) — decision/verify-driven-tests-20s-kill gained an except: a verify-driven test named `verify_window_` takes that class's 80 s kill. Why it matters: new stamped-home adopt cases sit on the 20 s `ci` kill line unless named into the verify_window_ class. Precedent for where a new adopt test binary's bound comes from.

@history_item n=6 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. E2E Test Strategy: 2 row(s)) — decision/hint-names-its-cause gained an except: viola up's `boot-time-unread` is held by its unit pair alone, because no case can drive it. It also added decision/window-reopen-proven-live (the R4 gated rig). Why it matters: precedent for a new closed exit cause with its own hint. Every cause needs a negative case asserting the exit, the `--json` document and a last hint naming it, and the only escape is a cause no case can drive. This applies to the early-exit word this chunk adds.

@history_item n=7 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. E2E Test Strategy: 3 row(s)) — added decision/live-rig-root-outside-repositories: a live rig's live run refuses a root inside a repository and runs its stand-in from a target dir of its own (PROVISIONAL). It also added decision/child-fd-table-read-after-launch-settles and the strict-modes scan for viola up. Why it matters: `scripts/r5-adopt.sh` types adopt in its own scratch `$base/typed-in`. Any change to where the rig runs adopt must keep the rig root outside every repository.

@history_item n=8 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (6. E2E Test Strategy: 1 row(s), live-rig-clean-start) — added decision/live-rig-clean-start (PROVISIONAL). A live rig's stand-in takes the wrapper's leading arguments, the rig unsets every inherited `VIOLA_*` before starting anything, and every board read runs with no claude on PATH. Why it matters: the R5 script's `run_env` passes only PATH and inherits the caller's real home. Any edit to the script's environment or folder must keep these three clean-start rules.

@history_item n=9 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (6. E2E Test Strategy: 2 row(s)) — added decision/window-raise-proven-live (R3 is a gated live reading with a rehearse form that never counts as live). It also put viola focus on decision/strict-modes-before-any-log, later ratified on fork F-A. Why it matters: this is the gated-reading precedent R5 follows. A rehearse form proves the script's path, never the real CLI's behaviour.

@history_item n=10 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (6. E2E Test Strategy: 1 row(s)) — added decision/live-failing-reading-not-verification: a live reading that fails a ledger row is recorded as read and never as verification, and a confounded live reading leaves the row unverified and its live re-read owed. Why it matters: R5a's two red reads (resumed=no, child exit 1 in about 0.8 s) were confounded by folder and home (hypotheses A/B). They stand as readings, and the fix's proof is the owed re-fire, not a reinterpretation.

@history_item n=11 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (3. Test Harness Contract: 1 row(s); 7. Test Data & Fixtures: 2 row(s)) — added decision/verify-under-test-owned-user-home and the fake agent's `--trust-config` / `--trust-record-blank` options. Why it matters: a test whose adopt start writes a trust record must aim the write at a test-owned config. The fake agent's trust decision comes from `--trust-config`, which a resume-folder test may need in order to model an untrusted spawn folder.

@history_item n=12 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (7. Test Data & Fixtures: 2 row(s); 3. Test Harness Contract: 2 row(s)) — contract/fake-agent-cli/trust-config now follows the CLI's walk (the cwd's exact key, then each parent up to the git root). It also added decision/trust-start-folders-outside-repository (trust-start folders under `/tmp`, because a workspace folder keys at its git root), decision/run-under-test-owned-user-home and decision/harness-wrappers-under-session-user-home. Why it matters: moving adopt's spawn folder changes which folder the start trusts. Tests of that folder must sit outside any repository and run under a test-owned user home, or they key at the workspace root or write the operator's real config.

@history_item n=13 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (1. Test Scope Summary: 2 row(s)) — added decision/run-start-verify-and-trust-after-bind: between the endpoint bind and the first snapshot, a start runs any automatic verify, then on a verified verdict writes an untrusted spawn folder's trust record. It also added decision/human-never-blocked's verify-window except. Why it matters: adopt shares run's start. The spawn folder this chunk changes is the folder whose trust record that step writes, and an early exit after it must be judged against that order.

@history_item n=14 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E Test Strategy: 1 row(s) ×2) — contract/event-log/start-wheel-record gained the `human-input` wheel line where trust keys pass through, and decision/start-lines-read-from-start-offset was added (a start's lines are read from its own offset, never line 1, because a reused name keeps earlier starts' lines). Why it matters: the early-exit record of an adopt start, under a name that may be reused, must be read from that start's own offset. An early exit may also land before session-start, so the start's record list can end short.

@history_item n=15 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive in the test plan: its exit-1 causes, the replay's first reader, the instance check's cases) — set revive's exit-1 causes in preflight order, made revive the first replay reader, and recorded that a replayed snapshot carries no cwd, so revive over it is refused `cwd-missing`. Why it matters: revive (and up) resume in the snapshot's recorded cwd while adopt alone has no such record (a P3 measurement owed). The cwd-missing cause and its preflight order are the pattern an adopt resume-folder rule sits beside.

@history_item n=16 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the fake agent's resume payload and ten options; the root waits at 26 in 19 and the kill path's endpoint wait) — under `--resume <id>` the fake agent sets source `resume` and session_id on the launch SessionStart. It also recorded that a killed wrapper leaves its socket file, so the kill path waits on `holder_gone`. Why it matters: the fake agent's `--resume` path is how adopt's resumed child is driven in tests. A child that exits at once must be told apart from a resumed one by these records, and a test that kills a wrapper must wait on `holder_gone`.

@history_item n=17 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the strict-modes cases without a log line, the gate's negatives) — made revive's strict-modes refusal the stderr pair alone with no log line on either arm, the check first and ahead of the wrapper log, with cases asserting no line added. Why it matters: precedent and trap for any new adopt refusal or early-exit outcome. A refusal before the log writes no line, while one after it writes one `process-exit`, and the cases must say which.

@history_item n=18 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data: agents files, test-scoped PATH; tests: revive's session-live) — added revive's `session-live` reading and `revive_whose_agents_read_fails_starts`. The fake agent was placed as `claude` in a test-scoped bin dir answering from files beside its exe, and every test-started list/revive runs under a test-scoped PATH. Why it matters: adopt shares the live-elsewhere reading. Its tests must keep the test-scoped PATH so they never read the host's real sessions, which this phase is also forbidden to touch.

@history_item n=19 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (tests: revive's machine form, the completeness case and the home-resolution cases landed) — gave revive `--json` with one `revive_json_*` case per code, and a test-started viola drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN`. Why it matters: a new adopt closed word needs its own `--json` document case beside the existing refusal tables, and tests must drop the inherited `VIOLA_*` names.

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table is built) — built `tests/cross_exit_causes.rs`: one negative test per cause asserting exit, `--json` document and last `hint:` line, each logged cause's `process-exit` detail, and literals in the file. The self-checks are `no_two_codes_share_a_hint`, `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`. Why it matters: the early-exit closed word, hint and output line this chunk adds must join this table or name its own binary, and must pass every self-check (code-word form, unique hint, no `release`).

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json`'s ten codes and the closed gap) — a refusal from the start that `run` makes after a passed preflight prints one exit-1 document under `revive --json`, before `spawn_child`. A `pump_child` error prints no document, because the child has held the terminal. Why it matters: a child ending at once is past `spawn_child`, in exactly the region where revive prints no document. Adopt's early-exit word must decide its document and output against this line.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder reads `stale-heartbeat`; the closed causes at the unit tier and the two built hints; the bare temp-dir home and the liveness-check case) — `stale-heartbeat` became its own exit-1 cause, and hints were built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`. It also recorded a trap: an oracle moved by a shared body can hide a test that never reached its subject. Why it matters: a new early-exit hint lands in `hints.rs` with its unit pin. Its negative case must prove it reached the early exit rather than an earlier refusal sharing the same exit 1.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's sixth cause and `revive`'s eleventh code) — added `socket-dir-refused` to revive's closed `--json` codes (ten→eleven) with its own hint and self-checks reading the word. Why it matters: precedent for growing a closed code list. The count, the table and the self-checks move together, and the counted register must be re-read.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (program resolution on PATH, the .cmd/.bat refusal retired) — revive's closed `--json` refusal codes went from eleven to ten (`batch-script-child` retired), and verify's from six to five. Why it matters: this is the current register of revive codes that an adopt early-exit word sits beside. Count words in the masters move by their rule.

@history_item n=25 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. E2E Test Strategy: 6 row(s)) — exit 11 was retired and «never renumbered or reused», and an older wrapper's retired word reads `unknown`, exit 14. Why it matters: a new early-exit outcome must take an existing or new exit number without reusing retired 11. An unknown word falls back to exit 14 (contract/cli-exit-codes/14).

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration in the contract suite) — `tests/contract_error_bodies.rs` lists every CLI document and error writer, with `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing`. Why it matters: a new adopt early-exit document or writer must be named in the enumeration with a witness, and its body must hold no path. That matters because the spawn folder is a path.

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the start's and verify's home-tree checks before any log; E5 built) — revive's start runs run's home-tree check before the wrapper log. The replay's product readers became six, and a revive over a replayed snapshot (no cwd) is refused. Why it matters: adopt shares this start sequence. Where an early-exit record or a release of the instance dir lands must respect the check-before-log order and the replay's absent cwd.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (a home a test makes before viola: `create_private_dir`, the plain home refused) — a test that makes a home or folder before viola makes it through `create_private_dir`, because a plain `create_dir_all` home is refused by the read rule. Why it matters: a test planting a session's original folder or a home for the resume-folder cases must make it private, or its oracle reads a strict-modes refusal.

@history_item n=29 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (7. Test Data & Fixtures: 4 row(s)) — the fake agent exits 1 on Enter on the trust form's No, and holds its launch SessionStart behind the trust form under `--screens` with an untrusted cwd. Why it matters: this is the existing fake-agent way to make a child end at once with exit 1, a stand-in for the early exit this chunk must report and record.

@history_item n=30 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E Test Strategy: 2 row(s)) — replaced decision/live-readings-not-a-test-layer with decision/standing-live-rig: one committed test-only rig whose rehearsals run in CI and whose live starts run by hand, each ledgered. Why it matters: the tier frame `scripts/r5-adopt.sh`'s rehearse/live split sits in. Any change keeps the rehearse form CI-safe and the live form local and ledgered.

@history_item n=31 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the fake agent's exit ignores SIGHUP) — on Unix the fake agent's exit ignores SIGHUP from its first step, so a hangup does not cut its exit-time coverage write. Why it matters: a test whose fake child exits at once must not kill it mid-exit, or it leaves a cut-short coverage profile.

@history_item n=32 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (7. Test Data & Fixtures: 1 row(s), argv-only-options) — the fake agent reads a fake-args file beside its exe ahead of argv, argv winning, and never the environment. Why it matters: an early-exit or folder-sensitive fake-agent mode for adopt tests must come from argv or that file, never an env seam.

@history_item n=33 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (5. Integration Test Strategy: 3 row(s)) — every long-lived viola child a root test spawns is tied to its test thread by a parent-death kill. decision/no-untied-viola-spawn is held by a census. Why it matters: a new adopt test spawning `viola adopt` must use the tied spawn helpers, or the census fails.

@history_item n=34 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: 1–2 row(s)) — moved the root wait bound and `WITHIN` to 12 s and the mutants default kill to 15 s, below cargo-mutants' 20 s floor. Why it matters: a new test waiting for an adopt child's exit or early-exit record uses `WITHIN` (12 s) under a deadline, never a sleep.

@history_item n=35 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (5. Integration Test Strategy: 1 row(s)) — decision/fresh-home-per-test: a stamped home is a copy of its build's verify stamp, never a link, and nothing is shared between tests. Why it matters: adopt cases that need a verified CLI (the trust step after the bind) start from `stamped_home`'s copy.

@history_item n=36 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (6. E2E Test Strategy: 1 row(s)) — decision/single-writer-state-files: only the wrapper writes `snapshot.json` and only verify writes `stamps.json`, the verify a keyed start runs included. Why it matters: recording an adopt early exit, or releasing its instance dir, must not make a non-wrapper writer of the snapshot.

@history_item n=37 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Test Scope Summary: 1 row(s) ×2) — the start's event order gained an optional `window` line, and the replay's fixed field set gained `window` (links, agent_session_id, wheel, lineage, launch, window). Why it matters: these are the current registers of start records and replay fields. An adopt early-exit event or a recorded resume folder that extends them moves these rows and their readiness probe (contract/boot-readiness/events).

@history_item n=38 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the CLI tests over std::process::Command, the integration tier) — CLI tests run `std::process::Command` against `CARGO_BIN_EXE_viola`. Every fixed string, human layouts included, is pinned by a literal in its test file (no trycmd). Why it matters: the new adopt early-exit output line and hint are pinned as literals in their test files.
