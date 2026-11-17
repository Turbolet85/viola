@history_item n=1 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (decision/standing-live-rig, the screen and approval exceptions) — retired decision/live-readings-not-a-test-layer for its heir + decision/standing-live-rig (one committed test-only rig `viola-live-rig` in viola-e2e, rehearsals over the fake agent in CI, live starts by hand under the founder's start cap per chunk, each ledgered with its clock time, live arm refusing under CI; guard `rig_rehearsal`), and added the rig exceptions to decision/no-child-screen-assertions, decision/never-parse-rendered-child-screen, decision/discipline-at-every-layer, decision/no-screen-parsing (the child's screen read by compiled literals into closed words only) and decision/no-blanket-approval-tests (a read trust dialog passed under its key grant); decision/real-claude-local-only now names the rig's live starts. This chunk takes every reading on that rig: any new arm (wake, latency, imports modal, verify-record) must stay inside these exceptions, and any key into a new modal, like the external-imports one, needs its own grant; the exceptions do not cover it.

@history_item n=2 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (fake agent trust form; the trim read live) — + contract/fake-agent-screens/trust-dialog; under `--screens` with an untrusted cwd the launch SessionStart fires only after Enter on the trust row; exit 1 on the form's No; typed keys draw a box rule and prompt row, DEL erases. The why of decision/send-confirmation-trailing-trim-match was rewritten from R2's live reading. These are the fake-agent surfaces a rehearsal of any new rig arm stands on. R2's `box-text: holds-text` reading comes from the same round, and an imports-modal rehearsal would have to model its modal on the fake agent the same way.

@history_item n=3 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only send tolerance, with a leading trim waiting for "the live reading carried on «Standing live rig with a screen model»", and ratified StopFailure's no-ledger-row registration, its live reading owned by «The 2.1.325 recorded fixture set». A precedent for this chunk: a live reading is carried to a named owner rather than folded into a fixture or row here. The scope's boundary says the same: no fixture is re-recorded here, and no ledger row is built.

@history_item n=4 marker=2026-11-16-scheduled-wake-count
**2026-11-16-scheduled-wake-count** — + contract/dialog-data/scheduled-wakes (Stop's `session_crons` read by length alone, null when absent or not an array, a non-array a drift entry to the instance detail file), + contract/turn-ended/scheduled-wakes (reaches events.ndjson, CLI `wait --json` and MCP wait only), ~ contract/event-data/turn-ended (four keys). R3b's «a real wake reads 1» reads these rows live; that chunk claimed only the measured 0 and synthetic counts.

@history_item n=5 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the delegate's confirm on the scheduled-wake rows (contract/dialog-data/scheduled-wakes, contract/event-data/turn-ended, contract/turn-ended/scheduled-wakes), the granular API-error cause rows and the local-command rows. The rows R3b reads stand on his own word, so a live reading that differs from them is a finding, not a row edit this chunk can make.

@history_item n=6 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — ~ contract/viola-wait/non-waking-records (a harness-origin prompt-submitted never wakes a parked wait); + decision/one-verify-stamp-per-build (stamped homes copy one stamp per build pair under a lock, the stamp dir's own sweep rule); ~ decision/fresh-home-per-test; ~ decision/verify-driven-tests-20s-kill why (one verify stamp per build keeps each stamped-home test under the 20 s kill); ~ contract/fake-agent-cli/script (Notification and UserPromptSubmit steps). This bears on R3a, a fired wake's prompt-submit, and on how a wake's prompt is filed. It is also the rationale the nextest kill-line carry must respect when `cli` and `harness_tied` join the 20 s line.

@history_item n=7 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — + decision/record-home-names-no-home (a home given to `viola verify --record` comes from the shared record-home helper and fixture, with no path component named `home`; guard `record_home_names_no_path_component_home`); decision/root-wait-bound and decision/within-7s-wait-bound moved to 12 s; mutants default kill 10 s → 15 s (decision/mutants-kill-below-cargo-mutants-timeout), recorded as a stated departure from "never raise the default". R4's live record home is that row's live half. Any new bounded wait in the rig or the harness falls under the 12 s WITHIN and the 15 s mutants kill.

@history_item n=8 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ contract/event-log/start-wheel-record (folder-trust keys passing through the wrapper write a wheel{human, human-input} line before session-start), + decision/start-lines-read-from-start-offset (a start's first lines are read from its own byte offset, never line 1 of a reused instance's events.ndjson). Every live reading of a start's records (R3a's payload, R3b's turn-ended, R3c's latency) must read from the start's own offset. It must also expect the trust-key wheel line on the rig's live starts.

@history_item n=9 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — + decision/tied-verify-kill-witness (a kill witness waits until verify holds a child inside a viola-verify run dir and reads the stamp's absence; guard `killed_test_leaves_no_stamping_verify`), + decision/no-untied-harness-boot (census `viola_e2e_census_finds_no_untied_boot`), ~ decision/supervisor-ordinary-child (an in-test session ties the supervisor by parent-death SIGKILL and its stamping verify by SIGTERM), ~ decision/no-advisory-ignored / decision/no-ignored-advisories (two ignore grounds). These are the `harness_tied` and `harness_lifecycle` cases whose kill-line placement the nextest carry decides. A shortened readiness deadline in `harness_lifecycle` must keep these tied-boot and witness rules.

@history_item n=10 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — ~ decision/wrapper-drop-guard (every long-lived viola child of a root test is tied to its spawning thread by a parent-death kill), + decision/no-untied-viola-spawn (census `cross_killed_test_census_finds_no_untied_viola_spawn`), + decision/tied-child-spawn-thread, ~ decision/supervisor-ordinary-child. Any new test or rehearsal case this chunk adds that spawns viola must use the tied builder, or the census goes red.

@history_item n=11 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (the `[profile.ci]` overrides born) — `[profile.ci]` gained the `verify_window_` override and the 20 s (10 s × 2) line over the verify-driven binaries, with no test-side bound on a verify a test drives (decision/verify-driven-tests-20s-kill). The rationale was that verify's designed floor is not a regression. This is the origin of the kill line the nextest carry says omits `harness_lifecycle`, `cli` and `harness_tied`.

@history_item n=12 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the standing rule set here: a kill line moves only for a floor that is by design, with a planted-hang control recorded both ways (TIMEOUT under the bound, PASS without it). This is a constraint on the kill-line fork for `harness_lifecycle`, whose 20.644 s case waits out boot's 20 s deadline by design. A bound of its own needs that two-sided control, and the case is never left to the kill line.

@history_item n=13 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `mutants` override order and the `verify_window_` class) — the harness test `verify_window_boot_with_an_unknown_cli_version_is_verify_failed` joined the `verify_window_` class by name (60 s under `ci`, was the profile's 120 s), the override order pinned by `mutants_verify_window_override_stands_before_the_harness_package_override`. A precedent for giving a viola-e2e harness case a bound by its own name or class: the operator chose the class's name over an override of its own.

@history_item n=14 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the `verify_window_` kill at 60 s) — `[profile.mutants]` is four overrides, first match winning (`verify_window_` 60 s, then `package(viola-e2e)`, `send_window_` and the paste-hint class at 30 s), each moved with its planted-hang pair and never by raising the default. The viola-e2e harness case takes the class's 60 s kill. A new override for `harness_lifecycle` sits in this ordering, with the same pair. This entry also added `HEADROOM_FLOOR_MIB` on the linked e2e-home base.

@history_item n=15 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — root readiness was made exit-aware and bounded apart from harness `boot`, which kept its 20 s (contract/harness-boot/timeout), and the mutants `package(viola-e2e)` override came in (decision/viola-e2e-30s-mutants-kill: the harness's own tests wait out its 20 s boot deadline by design). This is the history behind the inferred premise that boot's 20 s readiness deadline is a parameter `harness_lifecycle`'s case can shorten without changing the product.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the bootstrap phases; the CI matrix) — the `ci` kill line lists twenty-six verify-driven binaries (the count's rule: the `binary(…)` names on that filter line of `.config/nextest.toml`), and the root waits read 31 sites in 23 files. When `cli`, `harness_tied` and possibly `harness_lifecycle` join, the count moves by that rule, and the bootstrap-phases key file's spelled-out count and list must move with it. Earlier entries recorded this every time a binary joined (`2026-11-04-linux-and-macos-parity` 27, `2026-11-01-unix-endpoint-and-home-hardening` 26).

@history_item n=17 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — `harness_session` was made a plain function and drop guard that keeps the spawned `supervise`; boot's flags were built; the E2E regex was widened; the verify-driven override's list was recounted (a binary already on the line was missing from the list). This is where the viola-e2e boot model (decision/e2e-boot-model-by-cleanup) that `cli`, `harness_tied` and `harness_lifecycle` boot through comes from. It is also a trap: recount the filter line, not the prose list.

@history_item n=18 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (`run --local-live` specified) — under `CI`, `--local-live` is `live-in-ci`, exit 2, before any build (contract/run-reason/live-in-ci, decision/live-verify-never-in-ci); it builds, then runs one verify against the real `claude` through the runner seam (decision/local-live-runner-seam), with failure codes `build`, `verify-exit-<n>`, `verify-exit-none`, `row-missing` and the pass rule (contract/local-live-suite/*). R4's live `viola verify --record` is the record-taking sibling of this arm. Its CI refusal and its seam discipline are the model for that reading.

@history_item n=19 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the first live firing was recorded, and because `verify` names no program it stamps the first `claude` on `PATH`. A route entry's live readings of a real session are that chunk's own local measurement, not a test layer and never in CI. A trap for this chunk: PATH `claude` is now 2.1.326, which no stamp covers, so each reading must name its binary, and a reading or stamp on 2.1.326 must be counted apart.

@history_item n=20 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — the `target/e2e-home` keeper (`Workspace::ensure_e2e_home`, `prepare_home_base`) runs before `--local-live`'s verify; on the dev host the base is a link to tmpfs `/tmp/viola-e2e-home-<uid>`. Standing rule: a stalled-start red is a finding about the backing, stopped on and reported, never re-run for green. This applies to any live home R4 takes under that base. A later entry (`2026-10-13-paste-newline-ledger-row`) adds the trap that a tmpfs live home stamped at fewer rows reads `row-missing` until re-stamped.

@history_item n=21 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive runs and the screen fixture class: screens keep signature rows only, checked scrub-as-detector and refused whole with a named code (decision/screen-fixture-scrub-as-detector, contract/verify-recording-refusal/unable-message), and the hygiene walk refuses an email-shaped token and a seam-split username (decision/fixture-hygiene-walk). These are the refusal paths a live `verify --record` (R4) can meet. An unclean recording refuses the whole record (decision/dirty-payload-refuses-recording, decision/record-only-at-zero-failing-rows).

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes) — `verify` writes its refusal code on `process-exit` under a parsed `VIOLA_NAME`. A recording refused as unclean reads `unclean-fixture`, with the fixture's file name entering no code (contract/verify-exit-detail/unclean-fixture, contract/process-exit-detail/unclean-fixture). A failing row is no refusal and carries no code (decision/verify-failing-row-no-refusal). `verify` takes no `--json` (decision/verify-takes-no-json). This is what R4's live record reads back when it refuses, or when a row fails on 2.1.326.

@history_item n=23 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — + decision/stamping-verify-passes-transcript (the harness boot and the stamped-home helpers pass `--transcript`, a convention every new stamping helper keeps for its stamp to hold the 22nd row); the fake agent's `--transcript` and echo-command rows. A record-home or stamping helper added for R4 must keep this convention.

@history_item n=24 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ contract/readiness-gate/gate-max-wait (the input box reads up on its box rule row directly above a prompt row, with no footer literal needed), + decision/screen-fixture-box-rows-as-literals (a recorded box rule row is its rule chars alone and the prompt row the prompt char alone; guard `planted_box_row_beyond_its_literal_is_rejected`). This is the reading behind R2's unexplained `box-text: holds-text` on an empty, untyped box. An investigation of that reading rests on this box-row rule, and a live record of 2.1.326 screens must keep it.

@history_item n=25 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — the long-turn screen arm, a second compiled input-box literal `(shift+tab to cycle)` beside `for agents` (decision/screen-signatures-only), the 2.1.302 recorded set stamped in the suite, and the readiness stall report (decision/readiness-stall-report). A precedent for how a newer CLI's screen reading enters the compiled literals and a recorded set. This applies to the external-imports modal's shape and to R4 on a newer `claude`.

@history_item n=26 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `--tag-turn-screen` and `verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal`: a modal on the turn screen stops verify's paste. This is the standing precedent for a modal raised mid-run; the external-imports modal reading must say what a start or verify reads when it shows or does not show.

@history_item n=27 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the gate reads `viola_core::SPINE_DEADLINE` (1.0 s) and the hook's `CONNECT_DEADLINE` (750 ms) sits below it (decision/hook-connect-deadline, contract/viola-hook/deadline, decision/hook-deadline-timing). R3c, the window before a CLI-started turn's UserPromptSubmit reaches the wrapper, is measured against these hook timings. Later entries (`2026-11-01-unix-endpoint-and-home-hardening`, `2026-10-26-home-and-code-bearing-file-integrity`) recorded hook rows at about 2–3 ms max with the checks in the timed path.

@history_item n=28 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — a running turn refuses `send` 13 `turn-running` (contract/viola-send/turn-running, decision/send-refusal-order), and every test that sends twice must end the first turn with a scripted Stop. R3c's race is the window this refusal cannot close for a turn the wrapper has not yet learned of. R3b's procedure (one start, two sends) must end the first turn before the second.

@history_item n=29 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the fake agent's resume payload) — one payload is set without a recorded fixture, and the live payload on `claude` 2.1.287 held 10 key names against the rewritten 5. No reading became a fixture or a ledger row, and the recorded payload was carried to an owner. A precedent for R3a: a live payload shape (a fired wake's `source` field) is captured as closed words, not folded into fixtures or the fake agent here.

@history_item n=30 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — harness prefixes widened to four tags on the raw start (contract/prompt-submitted/origin-harness), and the drift contract compares receipts byte for byte (decision/fake-agent-matches-recordings, decision/fixture-drift-byte-compare). R3a reads a fired wake's UserPromptSubmit payload, which these rules classify and replay. Classifying a wake prompt stays out of scope (v2-36), but the reading must name what origin such a prompt gets today.

@history_item n=31 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — StopFailure registered with no recorded payload, no ledger row and no verify probe, its tests on a synthetic payload (decision/hook-payloads-recorded-by-verify except), and + decision/send-cases-copy-recorded-fixtures (a purely synthetic fixtures root fails any send after the first turn). A trap for any rehearsal this chunk adds that sends twice over the fake agent, as R3b's two-send procedure would.

@history_item n=32 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — every test-started `list` / `revive` runs under a test-scoped `PATH` so no test reads the host's live sessions (decision/tests-never-read-host-sessions); files beside the exe, no env seam. A constraint on rig rehearsals and on any helper that plants the fake agent as `claude`: only shared code may plant it.

@history_item n=33 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a witness bounded by a time limit, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes) (decision/bounded-remove-the-guard-witness). This applies to any remove-the-guard control this chunk records for its kill-line or rig change.

@history_item n=34 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the feed-panic passthrough) — the first exception to decision/no-child-screen-assertions and decision/never-parse-rendered-child-screen: the forced vt100 panic case byte-searches the outer terminal's drained stream (guard `chaos_feed_panic_refuses_send_and_keeps_passthrough`). The rig exception sits beside it. Any new screen reading must take one of these two named forms, never a parse of the rendered child screen.

@history_item n=35 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (live readings as local measurements) — the live round recorded the child's fd table and a live `statusline-rate-limits` red as local live measurements, no suite, each routed to a named owner rather than changing a row. This is the form R3a–R4 and the imports-modal reading take in the chunk's evidence: closed words, the binary named, and an owner for anything not built here.

@history_item n=36 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision / contract / absence), each verified against the code. Every later entry names its change by `+` / `~` / `−` handle. This is the form any test-plan change this chunk proposes must take (for example ~ decision/verify-driven-tests-20s-kill or the kill-line binaries list).
