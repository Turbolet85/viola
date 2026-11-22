@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (nextest mutants profile and toolchain source) — created `[profile.mutants]` with its slow-timeout kill because a caught mutant was graded Timeout under plain fail-fast; the origin of the `mutants` profile whose 15 s default kill item 6 reads against `contract_fake_agent_drift`'s replays.

@history_item n=2 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** (mutants kill below the floor) — root waits and the mutants kill were set by cause below cargo-mutants' 20 s floor after two `wait_ready` consumers spun to their bound; the precedent that every kill line sits below the floor (row decision/mutants-kill-below-cargo-mutants-timeout), which item 5's `ABORT_DEADLINE` vs the test's kill must keep.

@history_item n=3 marker=2026-09-24-epoch-1-cleanup
**2026-09-24-epoch-1-cleanup** (an unviable swamp is red) — added `unviable <= caught` to the verdict (rows decision/mutation-run-failure-conditions, contract/mutants-verdict/unviable-exceeds-caught); a killing test for a survivor must not be read by a mutation run inside this chunk, but the next boundary audit judges by this rule.

@history_item n=4 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — moved mutation off every chunk, pre-push and CI gate to the epoch-boundary audit and on demand (rows decision/mutation-at-epoch-boundary, decision/mutation-gate-cadence, decision/mutation-not-a-ci-gate); why this chunk runs no mutation run and its survivor kills are read by the next boundary audit.

@history_item n=5 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (run --mutants --package; boundary tier's form) — the boundary tier scores a whole member with `run --mutants --package`, a mutant the host cannot reach is "not measured here", and the Linux dev host takes a NOCOW btrfs `TMPDIR`; the form the audit used to read the 7 viola-agent-claude and 3 viola-pty survivors (contract/mutants-verdict/package).

@history_item n=6 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** (the mutants profile waits for running tests) — `[profile.mutants]` fail-fast became `terminate = "wait"`, the slow-timeout kill bounding any hang below the 20 s floor; context for item 6's kill reading on the `mutants` profile.

@history_item n=7 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — standing rule: a stalled-start red is a finding about the backing, stopped on and reported, never re-run for green; no kill, `WITHIN` or retry moved. Binds items 4 and 5, both reds measured under a neighbour's IO stall (no retry, no loosened bound without measurement).

@history_item n=8 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `verify_window_` kills moved in proportion to a by-design floor, each with a planted hang shown killed under the moved bound and passing with none; standing rule: a kill line moves only for a floor that is by design, with that control recorded both ways. The precedent item 6 (15 s kill on the drift replays) must meet if any kill is touched.

@history_item n=9 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the mutants override order and the verify_window_ class) — the first matching `[profile.mutants]` override wins, `verify_window_` placed first, the order pinned by `mutants_verify_window_override_stands_before_the_harness_package_override`; a new override for the drift replays (item 6) must respect that order and its pin.

@history_item n=10 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the shared root helpers; first whole-member viola-e2e score) — shared `tests/support` helpers and no copy per file (row decision/one-shared-test-helper); first whole viola-e2e score (718 mutants, 89 min) — mutating viola-e2e whole is explicitly out of this chunk.

@history_item n=11 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — host-excluded missed mutants leave the missed count and are named in `host_excluded` (rows decision/host-unreachable-mutants-excluded, decision/mutants-verdict-counted-by-harness, contract/run-document/host-excluded); how a survivor reachable only on another host is disposed rather than killed.

@history_item n=12 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — the prior survivors chunk's disposal form: each survivor killed by a nextest case in the package owning the mutated file, some after a behaviour-preserving restatement, each shown red under its hand-applied mutant and green restored with no mutation run; also recorded viola-pty's `PortablePty::resize` miss under two jobs sharing one `TMPDIR`. The exact precedent for items 2 and 3 (agent-claude and pump.rs kills owned by their own crates, row decision/mutants-tested-by-owning-package).

@history_item n=13 marker=2026-10-13-epoch-4-survivors-ii
**2026-10-13-epoch-4-survivors-ii** — the 16 `typed.rs` survivors (`framing_turns` 5 among them) disposed after restatements (named predicates, `probe_until`, generic `SharedPty`, plain ending functions), killed by unit cases; the history behind item 8's "one step function for `framing_turns`' per-paste probes".

@history_item n=14 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` (SIGINT/SIGTERM) via `verify_started`, with a cwd and `TMPDIR` the case owns; the stop witness shape item 7's SIGKILL measurement sits beside (contract/verify-stop/signal).

@history_item n=15 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — step lines count `MM` = `LedgerRow::relied()`'s length, and `STEPS_PASS`, `ROW_IDS`, `LEDGER_ROWS` each restate it as literals across `cli_verify`, `contract_ledger_probes` and `--local-live`; the scattered relied-count literals item 9 proposes to hold from one source.

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (twenty-four rows) — each added row moved the relied count through `[NN/MM]`, `STEPS_PASS`, `ROW_IDS`, `LEDGER_ROWS`, `LIVE_PASS`, the older-stamp oracle and the per-OS summaries at once; the drift item 9 and the wrap's count-literal detector address.

@history_item n=17 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the verify_window_ kill) — `[profile.mutants]` holds the ordered overrides with the paste-hint class pinned by `mutants_paste_hint_override_holds_the_class_above_the_default_kill`; both floors moved with their planted-hang pair, "never by raising the default" — the line item 6 must keep (rows decision/send-window-paste-hint-30s-kill, decision/mutants-kill-below-cargo-mutants-timeout).

@history_item n=18 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the headroom floor, the hook hangup named) — `HEADROOM_FLOOR_MIB` refusal for the tmpfs e2e-home base and the fake agent's children in their own process groups (row decision/fake-agent-own-process-group); host-stall context for items 4 and 5.

@history_item n=19 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the Epoch 5 survivors disposed) — 16 survivors each disposed by a case in the owning package, red under the hand-applied mutant and green reverted byte-exact, no mutation run, none equivalent; the most recent precedent of a cleanup chunk closing an audit's survivors so the next audit reads them closed.

@history_item n=20 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (scoped log captures through with_scoped_default) — every root-bin scoped capture goes through the pinned dispatch helper (row decision/scoped-capture-pinned-dispatch); a new mend test in `slot.rs` (item 1) that captures a log line must use it.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness (an unbounded one hung 27 min under the default profile); binds each mend's red-under-removal witness here.

@history_item n=22 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another) — `contract_fake_agent_drift`'s agents replay plants every copy before any thread starts a child (ETXTBSY, row decision/fake-agent-copies-planted-first); a trap in the very binary whose replay timing item 6 measures.

@history_item n=23 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — `contract_fake_agent_drift` compares, per recorded set walked at run time, each hook's stdin byte for byte, an empty walk failing (rows decision/fake-agent-matches-recordings, decision/fixture-drift-byte-compare); what the two replay tests item 6 reads actually do.

@history_item n=24 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (pump resize baseline) — the pump's baseline is the spawn size, witnessed through the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` forced-window seam; existing pump test machinery beside which item 3's `pump.rs` survivor kills land.

@history_item n=25 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration) — `tests/contract_bounded_readers.rs` `BOUNDARIES` names each external reader's bounded read and witness, a census failing an unnamed one (row decision/bounded-readers); a change to verify's agents read (item 4) must keep its row and witness standing.

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the bounded readers' cap-driving witnesses) — the child pipe drain shared by `claude --version` and `claude agents --json` is held by `drain_keeps_cap_bytes_of_an_endless_pipe` (row decision/bounded-readers-keep-the-cap); the cap side of the agents read whose time bound item 4 measures.

@history_item n=27 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (agents-join, Run B's agents read) — Run B makes its one agents read once its first turn has settled; an oversize or malformed answer reads unknown (rows decision/agents-json-fallback-unknown, decision/agents-parser-fails-to-unknown) and the fake agent answers from files beside its exe; the read item 4 gives a bound of its own.

@history_item n=28 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `WITHIN` 7 → 12 s and the mutants default kill 10 → 15 s, a stated departure from "never by raising the default" taken on the operator's answer (rows decision/root-wait-bound, decision/within-7s-wait-bound, decision/mutants-kill-below-cargo-mutants-timeout); the 15 s kill item 6 reads, and why raising any timeout is now out of bounds.

@history_item n=29 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — one verify stamp per build copied under a lock keeps each stamped-home test under the 20 s kill (rows decision/one-verify-stamp-per-build, decision/verify-driven-tests-20s-kill); context for the boot supervisor test's timing (item 5).

@history_item n=30 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — root viola children tied to the test thread by a parent-death kill, the supervisor untied only under the CLI boot (row decision/supervisor-ordinary-child, decision/no-untied-viola-spawn); the lifecycle of the boot supervisor test item 5 restructures.

@history_item n=31 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — RUSTSEC-2026-0543 (chrono) ignored on the unmaintained-upstream ground, "re-read each epoch" (rows decision/no-advisory-ignored, decision/no-ignored-advisories); in-test supervisor SIGKILL / stamping verify SIGTERM; `decision/no-untied-harness-boot` census; `decision/tied-verify-kill-witness` (a verify orphaned by a kill removes its own dirs, so processes and dirs cannot tell it). Directly the advisory freight and the verify-kill measurement design of item 7.

@history_item n=32 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify writes a trust record for its own temp dir and a SIGTERM stop removes it (contract/verify-stop/signal); every test verify runs under a test-owned user home with an empty owner-only config (decision/verify-under-test-owned-user-home) and the verify-start census; the exact setup item 7 needs to measure SIGKILL's leftover project entry without touching the real config.

@history_item n=33 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — run and revive tests under a test-owned user home, harness wrappers under a session user home, trust-start folders under /tmp outside any repository (rows decision/run-under-test-owned-user-home, decision/trust-start-folders-outside-repository, contract/fake-agent-cli/trust-config); constraints on item 7's test-owned Claude config.

@history_item n=34 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — the `verify_window_` class kill moved 60 → 80 s in ci and mutants (heir decision/verify-window-80s-kill); the current override set any item-6 change sits in.

@history_item n=35 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify step lines read pass|absent|fail and the version gate counts absent on the browser-tools-prompt row alone (rows decision/version-gate-needs-all-relied-rows, decision/ledger-row-verify-probe, contract/verify-output/step-line, summary-line); ledger.rs verdict logic the agent-claude survivor kills (item 2) may touch.

@history_item n=36 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — the gate reads the box up by its rule and prompt rows with no footer literal (contract/readiness-gate/gate-max-wait); first rmcp advisory ignore exception (decision/no-advisory-ignored); screen.rs predicate behaviour the item-2 `screen.rs:126/:135` kills test.

@history_item n=37 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — the gate's verdict reads compiled shapes over the compiled literals (decision/screen-signatures-only); the screen.rs surface item 2's survivors sit in, unit cases asserting nothing beyond compiled literals (decision/no-assertion-beyond-compiled-literals).

@history_item n=38 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Epoch 4 boundary wrap) — ratified every Epoch 4 PROVISIONAL mark and brought rows to the built tree: `decision/screen-signatures-only`'s shape parenthetical, `decision/live-rig-reads-compiled-shapes`, `decision/run-start-verify-and-trust-after-bind`, fake-args file (contract/fake-agent-cli/argv-only-options); the masters this chunk starts from, issued by the same wrap that produced its directive.

@history_item n=39 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — the paste ends within `PASTE_BOUND`, a cut closes the bracket with no Enter and ends the send paste-cut; refusal order gained the two paste-cut rungs; `--stop-reading` (rows contract/driver-send/typed-text, contract/viola-send/paste-cut, during-a-blocked-paste, decision/send-refusal-order, contract/fake-agent-cli/stop-reading); the paste writer whose three `pump.rs` survivors item 3 kills, and the send path whose `SendSlot::confirm` item 1 mends.

@history_item n=40 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — the claim moved to trailing-trim match (decision/send-confirmation-trailing-trim-match, contract/viola-send/records); the confirmation logic around `SendSlot::confirm`'s claimed line (item 1).

@history_item n=41 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `decision/stamping-verify-passes-transcript` (every stamping helper passes `--transcript` for its stamp to hold the echo row) and the echo-confirmed unconfirmable cases; a convention any helper restated by item 9's single relied-count source must keep.

@history_item n=42 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — `decision/stamping-verify-test-owned-runtime-dir` (every 29-row stamp needs a test-owned runtime dir) and the drift check's send-tool exception in `contract_fake_agent_drift` (decision/fixture-drift-byte-compare); the latest change to the relied set (29) and to the drift binary item 6 reads.

@history_item n=43 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all rows on every build; mutation without the Windows leg) — `LedgerRow::relied()` ungated, literals `[&str; N]` ungated; no Windows mutation leg or host scratch, `mutants.out` at the repo root; simplifies item 9 and shapes how survivors are read.

@history_item n=44 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the always-admit survivor; advisories with no ignore; planted annotation drift) — the client-seam always-admit survivor stands as the version's known limit (decision/mutation-run-verdict except); `[advisories] ignore = []` was reached then; `dialog_drift_names_a_changed_annotation` added to `contract_fake_agent_drift`; precedent for disposing a survivor by record rather than a kill, and for the chrono re-read.

@history_item n=45 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — floor names no longer counted in rows; `decision/strip-control-oracle-keeps-guarded-names` (a remove-the-guard control's oracle must keep the guarded names); `contract/claude-fixtures/env-names` — `ledger.rs`'s `env_names_fault` survivors (item 2, first) sit on this env-names surface.

@history_item n=46 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — a keyed `viola run` start runs verify, still the sole stamps writer (decision/single-writer-state-files); the start's automatic verify is a second caller of Run B's agents read (item 4).

@history_item n=47 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — one committed live rig, rehearsals in CI, live starts by hand (decision/standing-live-rig); this chunk runs no live or rig start of a real claude, and splitting `rig/mod.rs` is deliberately out.
