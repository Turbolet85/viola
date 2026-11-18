@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: 4 rows) — verify's child set went from five interactive runs to six (Run F, after Run E). It added `+ decision/verify-own-trust-record` and `+ decision/trust-record-ledger-row`, the 23rd relied row. It also widened `~ decision/claude-first-no-adapter-trait` so the fake agent and the rig may read a Claude config file's trust record. This is the row this chunk re-reads live, and the child set its instruments measure. The card confirm is PROVISIONAL (delegate viola-overseer) and sits inside the founder's rulings R1–R4. Any change to Run F, or to what verify records per run, has to stay inside that ratified child set.

@history_item n=2 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: the rule reworded) — `~ decision/trust-record-ledger-row`: the rule no longer states that the CLI honours a trust record. It now asks the question per CLI version: Run F passes only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box. The reason given is that the one live reading on 2.1.325 read fail. That is the reading S0′ repeats. The pass condition already needs Run A's and Run F's first screens, so the new per-run first-screen word has to match these three conditions.

@history_item n=3 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Standard Contracts: 16 rows) — `~ contract/ledger-stamps/rows` and `~ contract/ledger-stamps/content`: the trust-record row's stamp value may be the trust writer's code word instead of pass or fail. It added `+ contract/trust-writer-words/*`, a closed set of 14 words: `written`, `removed`, `config-absent` (Run F starts no child), `config-unreadable`, `config-link`, `config-not-regular`, `config-foreign`, `config-oversize`, `config-malformed`, `config-dir-unusable`, `home-unknown`, `key-present`, `lock-held`, `lost`. The `--record` witness of where the record went, and any new stamp content, must reuse these closed words and must not change the meaning of a row's value.

@history_item n=4 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Occupied Resources · Conventions · Cross-cutting Patterns · Infrastructure Patterns · Inherited Defaults) — `~ decision/verify-run-e-resumes-run-b-first-session`: the six runs execute in order A to F. `~ decision/verify-removes-only-own-dirs`: verify also removes its own trust record on pass, fail and stop, and only what that run created. `~ decision/env-strip-names-only` and `~ decision/env-strip-reads-names-only`: verify reads `CLAUDE_CONFIG_DIR`'s value only when `claude_env_keep` keeps it (`~ contract/config-json/claude-env-keep`, `~ decision/no-environment-configuration`, `~ decision/env-var-viola-prefix`). Also changed: `~ decision/claude-shapes-in-one-crate` and `~ decision/claude-shapes-isolated`. These are the bounds on the rig's private config and on verify's environment (CLAUDE_CONFIG_DIR is passed to the interactive children), which the P3 list of differences has to name.

@history_item n=5 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Established Decisions: permission-request row) — `~ decision/permission-request-question-stays-null`: the count was replaced by «a further relied row» so it cannot go stale again. Precedent for this chunk: write no relied-row count that a later row would falsify. The relied set is 23 rows, and the floor is the 21 that the live home's stamp passes.

@history_item n=6 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — `~ decision/claude-shapes-in-one-crate` and `~ decision/claude-first-no-adapter-trait` gained a test-only exception: the live rig reads the key names and source word of its own captured UserPromptSubmit payload, never a value. This is PROVISIONAL (delegate the overseer), ratified by the founder's «Разрешить захват в риге». The rig's `V` arm work in this chunk must stay inside this capture exception: closed words only, no screen text, nothing of the founder's config.

@history_item n=7 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — added `+ name/binary/viola-live-rig` and `+ name/other/agent-run-rig` (`AGENT_RUN_RIG`). `~ decision/test-binaries-never-released` and `~ decision/harness-env-unread-by-product` now list them. `~ decision/vt100-screen-model` and `~ decision/vt100-readiness-gate` gained the exception that the test-only rig uses vt100 as the screen model for its child's trust dialog, selector and input box. The `V` arm and its six-word printout extend this rig, and its vt100 use must stay test-only. `~ decision/submitted-form-prompt-match`'s why records the trim reading R2 made live on that rig.

@history_item n=8 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/screen-signatures-compiled`: a screen is ready when no row holds a modal literal and either a footer literal stands or the box's own rows do (a rule row of at least 40 rule chars above a prompt-char row). `~ decision/verify-box-wait-for-literal`: after a paste turn, a quiet screen keeps waiting unless it holds the box or a modal. `~ contract/ledger-stamps/measured` gained `box_only_guards`. `~ decision/screen-fixtures-signature-rows-only` was also changed. This is the predicate the first-screen word (`trust` · `imports` · `box` · `none`) must be computed from. It is also the direct precedent for adding a reading to `measured`: additive, no `v` bump, and run reads none of it.

@history_item n=9 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — `SIGNATURES.input_box` gained `(shift+tab to cycle)`, because 2.1.302 hides `for agents` 15 s after start. `input-box-signature` now reads three screens (ready, turn, and the screen after the long paste), and a long turn that never settles fails it. The fake agent gained `--long-turn-screen`. Trap: a CLI's footer can change between versions and silently push rows 8–23 to fail from `input-box-signature` on. That is exactly S0's pattern, and the per-run first-screen word exists to tell this case apart from a trust stop.

@history_item n=10 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled, ten ledger rows, verify's interactive runs) — compiled the modal literals `Yes, I trust this folder` and `Yes, allow external imports`. Run A runs untrusted in an OS-temp dir and is killed; Run B runs trusted in `<cwd>/.viola-verify-<pid>/`. No key goes into a CLI dialog. `Screen::rows()` is verify-only. Measured on 2.1.288: trust is inherited from a trusted parent, and the imports approval is keyed on the git root. These are the compiled literals behind the `trust` / `imports` words. The trust and git-root key rules are the reason the `V` arm puts Runs B–E in a git-initialised, seeded repository and Run F outside any repository.

@history_item n=11 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows for verify's interactive runs and screen fixtures) — `measured` gained `typed_probe` with run reading no number from it. `--record` screens keep signature rows only. The `--record` refusal names the file and a closed code, never the content. verify's spawn pairs are logged, and the interactive probe dirs are 0700 and removed. Any new first-screen word and the `--record` witness must follow the same signature-only and closed-code rules (`decision/run-never-reads-screen-content`, `decision/screen-rows-read-only-by-verify`, `decision/verify-record-only-clean`).

@history_item n=12 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify went to four interactive runs (C and D). `measured` gained `dialog_probe`, which is additive, needs no `v` bump, and run reads none of it. This is a second precedent for growing `measured` per run without touching the relied rows or run's version gate.

@history_item n=13 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E · Run E's resources and a killed PTY child's exit status) — Run E resumes Run B's first SessionStart id and spawns nothing without one. A verify is several `claude` starts. A child killed at a stop closes its spawn pair with `duration_ms` and a `child_exit_status` (the PTY pump's exit code). This bears on the scope's closed facts («the child's exit before the read», each run's wall): the spawn pairs already record exit and duration per interactive run. Run E with no child is the `null` first-screen case.

@history_item n=14 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (CLI Version Compatibility: the agents-join row and Run B's agents read) — added the relied row `agents-join` and one `claude agents --json` read inside Run B once its first turn settles. This was ratified as a widening of the child set. It is one of rows 8–23 that S0 failed downstream of Run B. The floor predicate reads it, and `decision/verify-child-set` counts this read.

@history_item n=15 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the twenty-fourth ledger row · Run B's three transcript files) — Run B got its sixth paste (`/compact`), each paste going only into the settled input box. `typed_probe` gained `compact_latency_ms` (measured at 21 413 ms live), additive with no `v` bump. Run B leaves three transcripts. This bears on the timings a ~40 s whole verify implies: a Run B that really reached the box takes far longer than 40 s. Precedent: a new `measured` reading is additive.

@history_item n=16 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's pastes go only into rows holding an input-box literal and no modal. After the long and tag-like turns, the wait is bounded by `PROBE_DEADLINE` from that turn's Stop. Measured: 2.1.287's footer holds no input-box literal for 8.0 s after a long paste. These are the waits that bound how soon a screen is read. They are among the timing candidates for runs stopping short of the box.

@history_item n=17 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` is 8.5 s and `QUIET_PERIOD` is 300 ms (`decision/gate-timings-compiled`). Trap for later chunks: any case that reads a verified, literal-less screen through a clock that never advances now waits forever. This bears on bounding the 52 s `verify_window_without_screens_fails_every_interactive_row` case and its 60 s `verify_window_` kill: with no screens, each interactive run waits out its deadlines.

@history_item n=18 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a stopped verify kills and reaps its child with no key, starts no later run, writes no stamp, and exits 1 with `detail:"interrupted"`. The drop guards remove only what that run created. This is the stop path the rig's standing stop (Ctrl-C into its pty, or a no-key group kill) drives. The stop must leave no stamp and no trust record.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's stop: `SIGINT` / `SIGTERM` alone · the relied set: all rows on every build · the R8 strip: exact names) — verify's stop handles SIGINT/SIGTERM only; SIGKILL and a crash leave the dirs behind. `LedgerRow::relied()` is every row on every build. The R8 strip removes inherited `CLAUDE*` names matched exactly, `claude_env_keep` is the one pass-list, and the identity floor of eleven names always goes. These define the environment of verify's children (`decision/child-env-claude-strip`, `decision/r8-identity-floor`) that the P3 list must compare against the rig's own wrapped start.

@history_item n=20 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the statusline rows left the ledger (relied rows went 23 → 21 before trust-record and local-command-echo) and Run B lost its probe `--settings`. `~ decision/claude-flag-literals-in-agent-crate`: verify alone reads `--settings`. `~ decision/protocol-version-bump-rule`: the stamps' removed row ids kept `v` at 1. This explains the 21-row floor and the argv verify's children now get, both of which the P3 list of differences reads.

@history_item n=21 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns) — `~ decision/mixed-version-tolerance`: a stamp holding a retired ledger row verifies on the relied rows alone. This bears on reading S0′'s stamp against the live home's 2.1.325 stamp under the floor predicate (`decision/version-verified-all-relied-rows`, `decision/stamps-merge-own-version-only`).

@history_item n=22 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits row reads the account's plan state) — on the rig home, an unchanged CLI's verify failed a row (`23 pass 1 fail`) because the account's payload differed. That red was ruled a finding and never re-run for green. Trap: a row reading below `pass` on the rig's private home can come from the home or the account, not the CLI. The P3 candidates and a spent spare must name such a cause.

@history_item n=23 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** (plugin install as built) — its live round took none of its readings: with `proj/` its own git repo, both wrapped starts were held by the folder-trust dialog, and a start was left unspent. This is a measured precedent that a git-initialised cwd raises the trust dialog. It bears directly on the `V` arm git-initialising its scratch root and seeding the record at the root's key.

@history_item n=24 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/wheel-starts-with-driver`: a key the human types into the folder-trust dialog before the session starts takes the wheel. This is a live reading of how the trust dialog meets the wrapper. It stands beside `decision/no-typing-into-trust-dialogs` and `decision/verify-never-types-into-dialog`: no rig key and no verify key may go into either dialog.

@history_item n=25 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules) — `src/cmd/verify/typed.rs` is the module root over `src/cmd/verify/typed/`: `runs.rs` (Runs A–E, `trusted_args`), `turns.rs` (`poll_until`, `wait_for`, `agents_read`, `framing_turns`), `pty.rs` (`settled`, `box_wait`), `run.rs` (`Run`, `ctrl_c_ending`, `ended_by`). The per-run first-screen word and the child-exit fact go into these modules.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before verify's log · the stamps writer's strict-modes check) — verify runs `check_home_tree` before its `cli` log. `update_stamps` runs `check_stamps` before it creates `ledger/` and again under its lock, and a refusal reads and writes nothing. This is `decision/verify-refuses-untrusted-home-or-stamps`, whose why notes that the verify window test expects the widened stamps byte-equal. Any change to the `verify_window_` cases or to stamp contents must keep that.

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes on its `process-exit`) — under a parsed `VIOLA_NAME`, verify's `process-exit` carries a closed `Ended::Refused` code (including `unclean-fixture`) beside `interrupted`. A failing row is no refusal and carries no code, and the fixture's file enters no code (`contract/verify-process-exit/*`, `contract/verify-exit-detail/*`). The new readings must not turn a failing row into a refusal code.

@history_item n=28 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the landed ledger rows, `viola verify`'s probe, the stamps envelope) — the origin of the closed ledger, the stamps envelope (`writer:"verify"`, per-version `rows` plus `measured`, a merge that keeps unknown versions and fields), and `--record` writing scrubbed fixtures only at 0 fail. A failing row still stamps and exits 1. These are the base contracts (`contract/ledger-stamps/*`, `contract/viola-verify-exit-codes/1-relied-row-failed`, step and summary lines) that S0′'s reading is taken against.

@history_item n=29 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, the harness CI read) — the harness's `run --local-live` is the one real-`claude` verify home, refused under `CI` before any build or spawn (`decision/no-live-claude-in-ci`, `decision/real-cli-probe-local-only`). verify's spawn pairs are logged per child. The live start S0′ is local only, and CI keeps verifying against the fake agent.

@history_item n=30 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `~ decision/root-waits-share-one-bound`: root test waits share the one 12 s `WITHIN` bound (it was 7 s). Every waiting case this chunk adds or re-bounds takes this shared bound and stays under the 20 s lint.

@history_item n=31 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the screen model is the pure `viola_agent_claude::screen::Screen`. A caught feed panic or a dropped feed message poisons it until the host size changes, and a poisoned model reads not ready (`decision/poisoned-screen-model`). A poisoned screen is a run outcome the closed first-screen word set (`trust` · `imports` · `box` · `none`) must place without inventing a new word.

@history_item n=32 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers, a cross-cutting pattern held by a census) — every product reader of external bytes is bounded in its own statement or named in `tests/contract_bounded_readers.rs`'s `EXCEPTIONS`, and the census goes red otherwise (`decision/bounded-readers`). Any reader this chunk adds, such as one behind the `--record` witness reading the Claude config, must land bounded.
