@history_item n=1 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (the /mcp echo precedent) — changed `~ contract/driver-send/local-command` from exact equality to classification by name; widened `~ decision/local-command-post-condition` to add `/mcp`, confirmed only by a new stdout tag in the transcript past its pre-paste end; widened `~ contract/send-result/unconfirmable` and `~ decision/local-command-unconfirmable` to cover an echo-confirmed command with no remembered or readable transcript end; retired `− decision/local-command-exact-match`. This is the exact template `/context` follows with the `CommandStdout` post-condition. The post-condition rule names `/clear`, `/compact` and `/mcp` one by one, so `/context` must join that enumeration.

@history_item n=2 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (the fake agent and harness surface) — added `+ contract/fake-agent-cli/transcript` (`--transcript` seeds a 0600 transcript and sets the launch SessionStart's transcript_path) and `+ contract/fake-agent-input/echo-command` (under `--local-command-mode` a submitted echo-confirmed command, and under `--framing` the exact verify echo probe command, appends one stdout tag line and fires no hook). Also widened `~ decision/fake-agent-replays-recorded-bytes` and `~ contract/fake-agent-hooks/stdin-payload` for that set field, and added `+ decision/stamping-verify-passes-transcript`, because without a transcript Run B's echo probe is skipped and the local-command-echo row fails. The fake agent's rehearsal of `/context` extends both input rows: as written, the `--framing` half names one "exact verify echo probe command". A second echo probe needs that row amended, and every stamping helper must keep passing `--transcript`.

@history_item n=3 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the delegate's confirm of the local-command-delivery rows (`contract/driver-send/local-command`, `decision/local-command-exact-match`, `decision/local-command-post-condition`, `contract/send-result/unconfirmable`), so they no longer stand PROVISIONAL. Widening the echo-confirmed set to `/context` touches founder-ratified rows. Those rows change only by a fresh amendment carrying its own authority.

@history_item n=4 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — landed `send`'s local verdicts. `local` is exit 0 `{confirmed:false, detail:"unconfirmable", cursor}`, with send-issued then send-confirmed{confirmed:false}, no send-refused, no window, and the command text in no role log. Pinned the human line `[  ] unconfirmable  <name>  local command, no measured post-condition`, with a post-condition-confirmed command printing `[RB] read back`. Kept off-list slash text at exit 13 `no-prompt-submitted`, and added the `send_local_command_` unit family in `src/run/send/classify.rs`. These are the rows behind `contract/send-result/unconfirmable`, `contract/viola-send-human-mode/unconfirmable`, `contract/viola-send-human-mode/read-back` and `contract/viola-send/off-list-slash-text`. `/context` moves off the slash-off-list path onto these outcomes.

@history_item n=5 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — before any local-command row was compiled, set `local` to exit 13 `not-delivered`/`no-prompt-submitted`, never presumed delivered. This is the origin of "a listed local command is never presumed delivered". The precedent: a slash command's outcome depends on whether it is on the compiled list. `/context` today takes that off-list exit-13 path, and joining the list changes its observable outcome.

@history_item n=6 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (twenty-four rows) — added a local-command ledger row end to end. Relied counts rose and `cli_verify` `[NN/MM]`, `STEPS_PASS`, `ROW_IDS`, `contract_ledger_probes` `ROW_IDS`, `LEDGER_ROWS` and `LIVE_PASS` were all re-sized. Run B gained a compiled paste (`/compact` before `/clear`) with no kill line moved, plus a new framing variant and replay contract case. The older-stamp oracle was extended, and the no-framing and tag-turn-modal summary literals were recounted. This is the nearest full precedent for every count and literal a new `/context` row moves.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (`/compact`'s own window) — `/compact` on a verified CLI waits only for its post-condition inside `COMPACT_CONFIRM_WINDOW` (40 s, provisional, guard `send_compact_confirm_window_is_its_own_bound`). Every other confirmed send keeps the 10 s fallback, so whether `/context` takes its own window or the fallback is a decision this chunk must state. It also brought four `…_compact_*` classify functions and the unverified-CLI-unconfirmable E2E case, the pattern for `/context`'s unit and E2E cases.

@history_item n=8 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — set `/compact` to unconfirmable at once until its row landed (`send_compact_on_a_verified_cli_is_unconfirmable_at_once`, later retired). It also dropped the master's case count for `send_local_command_` (the code's `#[case]` rows are the count) and recorded a live firing of every relied row. Precedent: a command listed before its post-condition is measured answers unconfirmable, and no master states case counts.

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — built the local-command rows and the framing tier. Run B's added pastes recorded `UserPromptSubmit.paste-<n>` / `SessionEnd.clear-<n>` / `SessionStart.clear-<n>`, which the drift contract replays byte for byte under `--framing`, and a set without them must exit 1. Recorded the standing sizing rule: the 20 s kill stays on a measured verify reading. Run B's paste sequence and the framing replay are where a `/context` probe paste lands.

@history_item n=10 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — added a ledger row with a Run B paste and recorded a trap: a live home stamped at the old row count reads `row-missing` under `--local-live` until its next re-stamp. It also says every count carries its rule (`LedgerRow::ALL`). A new relied row makes every stamped version and the tmpfs live home read unverified or `row-missing` until re-stamped. The scope leaves that relied/not-relied question open.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — introduced the relied set (`LedgerRow::relied()`), step lines `[NN/MM]` with MM the relied count, and `contract_ledger_probes` listing each relied id once. The scope's open question, whether `/context`'s row is RELIED, is decided against this mechanism: `decision/version-gate-needs-all-relied-rows` and `contract/verify-output/step-line` both key on it.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set) — `LedgerRow::relied()` became all twenty-four rows on every build. `STEPS_PASS`, `ROW_IDS` and `LEDGER_ROWS` became `[&str; 24]` ungated, and the older-stamp oracle was restated. This is the current shape the new row's count moves, with no per-OS split left.

@history_item n=13 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/version-gate-needs-all-relied-rows` gained "a retired row it also holds changing nothing" when `statusline-rate-limits` was retired. `− decision/budget-…` rows and `send-refusal-order` lost the budget rung. The version gate's rule is the one a new relied row tightens: every stamp lacking the new row reads unverified.

@history_item n=14 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (`statusline-rate-limits`'s live red) — a live `viola verify` read a ledger row red because the row measured the account's plan state, not the CLI. The row was then judged no CLI behaviour to rely on. This is a trap for a `/context` row: its post-condition must be a CLI behaviour measured live, not something that varies by account or session.

@history_item n=15 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — changed `~ contract/verify-output/step-line`, `~ contract/verify-output/summary-line`, `~ decision/version-gate-needs-all-relied-rows`, `~ contract/local-live-suite/pass` and `~ decision/ledger-row-verify-probe` so a row may read `absent`, counting, on the browser-tools-prompt row alone. A `/context` row lands into this verdict vocabulary. Unless the chunk widens the exception, it must read pass, never absent, to count.

@history_item n=16 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — created verify's interactive runs (`--screens --turn-stop --trusted-root`), the ten-row `LEDGER_ROWS`, the screen fixture class and the hygiene walk over it. It owed local-command `unconfirmable` to a later entry. This is the foundation of `decision/ledger-row-verify-probe`, `decision/fixtures-recorded-by-live-verify` and `decision/ledger-probe-contract-suite` that the new probe extends.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the `agents-join` row added a Run B read after its first turn settles, before the framing pastes. That moved the spawn-pair count, the summary literals and the `[NN/MM]` lines. Where the `/context` paste sits in Run B's order relative to the framing pastes and the agents read follows this ordering precedent.

@history_item n=18 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — made `2.1.287` and `2.1.302` stamped sets of `contract_ledger_probes`, added the long-turn modal case and its summary literals, and walked the newer set's variants in the hygiene contract. Every stamped recorded set (now including later stamped sets) must hold the new row's probe outcome. Otherwise it moves to the drift-only list under `decision/ledger-probe-contract-suite`, every fixture dir on exactly one list.

@history_item n=19 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — found the `--local-live` parenthetical row list stale after three framing rows landed, and named all ids of `LEDGER_ROWS`. Trap: any master text that enumerates ledger row ids must gain `/context`'s id in the same chunk.

@history_item n=20 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — the last ledger row added. `~ contract/verify-stop/signal` now removes verify's own trust record. It added `+ decision/no-verify-start-outside-test-home` (a census tying every test verify to a test-owned home), `+ decision/verify-under-test-owned-user-home` and `+ decision/live-failing-reading-not-verification`, under which a failing or confounded live reading is recorded as read, never as verification. Any new verify test for the `/context` probe falls under the census. The live stamp of the row is held to the read-versus-verified rule, which matters because cry-qklc2wb6 rests on one reading.

@history_item n=21 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — `~ decision/record-only-at-zero-failing-rows` gained the rig-only first-screen exception. `decision/verify-window-60s-kill` became `decision/verify-window-80s-kill` because the class's floor grew with verify's runs. A further Run B paste lengthens the `verify_window_` floor, and the kill moves only for a by-design floor with planted-hang controls both ways.

@history_item n=22 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — moved the `verify_window_` kill to 60 s and set the paste-hint class's mutants kill, each with planted-hang TIMEOUT/PASS controls. It stated that a slow case is fixed by removing slow work, never by raising the default (`decision/send-window-paste-hint-30s-kill`). This is the standing method if the added `/context` paste pushes a verify-driven case toward its kill.

@history_item n=23 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — added `+ decision/one-verify-stamp-per-build`, widened `~ decision/fresh-home-per-test` (a stamped home is a copy of its build's stamp) and moved `~ decision/verify-driven-tests-20s-kill`'s why to the one stamp per build. The new row is stamped once per build, and this is why a longer stamping verify does not hit each stamped-home test's 20 s kill.

@history_item n=24 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `~ decision/root-wait-bound` and `~ decision/within-7s-wait-bound` moved to 12 s, `~ decision/mutants-kill-below-cargo-mutants-timeout` to a 15 s default kill, and it added `+ decision/record-home-names-no-home`. These are the current numeric bounds for any new root wait and the recording-home rule for a `--record` of the new probe's fixtures.

@history_item n=25 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `decision/live-readings-not-a-test-layer` became `+ decision/standing-live-rig`. It widened `~ decision/real-claude-local-only` (live through viola verify and the rig's live starts), the screen exceptions (`decision/no-child-screen-assertions`, `decision/never-parse-rendered-child-screen`, `decision/discipline-at-every-layer`, `decision/no-screen-parsing`) and the fake agent's trust, keystroke and DEL contracts. The live stamp of `/context` on 2.1.328 rides this rig's start under the founder's start cap. The rig's DEL/box rehearsal is the subject of carry cry-4olcx4il (the intermittent `rig_rehearsal_reads_the_box_empty_after_x_and_del` red, acted on only at its next red).

@history_item n=26 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the rig) — `~ decision/standing-live-rig` gained the SF arm's local-only contained rehearsal. The except of `decision/no-child-screen-assertions`, `decision/never-parse-rendered-child-screen`, `decision/no-screen-parsing` and `decision/discipline-at-every-layer` now reads "by compiled literals and the box row's cell attributes, never text". It added `+ contract/fake-agent-cli/turn-fail` and `+ contract/fake-agent-cli/settings-hooks`. Carry cry-kutofxml (the placeholder hint's cell attribute read past the cursor's cell, attributes only) rides this chunk's rig start under exactly these excepts.

@history_item n=27 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled) — kept `decision/hook-payloads-recorded-by-verify`'s except on one contained live reading on 2.1.328, claiming nothing for other words or versions and making no recorded fixture. It is the precedent for how one live reading is worded: cry-qklc2wb6's `/context` mechanism rests on one reading (`context-tags: 1`), which must not be written as verification.

@history_item n=28 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** — `~ decision/record-home-names-no-home` extended to the live rig's record arm, which refuses a home-named scratch base with exit 3, uncounted, before any credentials read. This applies if the new row's live stamp records fixtures through the rig.

@history_item n=29 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — added `+ decision/run-under-test-owned-user-home` and `+ decision/harness-wrappers-under-session-user-home`. It changed `~ contract/fake-agent-cli/trust-config` to the CLI's trust walk. Any new E2E `/context` send case booting a wrapper must run under a test-owned user home.

@history_item n=30 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — `~ decision/wrapper-drop-guard` (long-lived children tied to the test thread by a parent-death kill), `+ decision/no-untied-viola-spawn` (a census) and `+ decision/tied-child-spawn-thread`. New root send or verify tests for `/context` are held by the untied-spawn census.

@history_item n=31 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — `~ decision/supervisor-ordinary-child` (the in-test session ties the supervisor by SIGKILL and its stamping verify by SIGTERM), `~ decision/rstest-case-tables` (viola-e2e and viola-core use labelled tables) and `+ decision/tied-verify-kill-witness`. These are the case-table form and the stamping-verify tie any harness-side change for the new row keeps.

@history_item n=32 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ contract/event-log/start-wheel-record` (trust keys passing through the wrapper write a human-input wheel record before session-start) and `+ decision/start-lines-read-from-start-offset`. A trap for the rig's live `/context` start and any reused-name reader: read a start's lines from its own offset.

@history_item n=33 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ contract/readiness-gate/gate-max-wait` (the box read up on its rule and prompt rows with no footer literal) and `+ decision/screen-fixture-box-rows-as-literals`. The `/context` probe paste goes through verify's readiness wait and its screen fixtures under these rules.

@history_item n=34 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — `--local-live` alone runs no other suite and stamps the first `claude` on `PATH`. The live firing was recorded as a dated reading. The suites reach the real CLI only through `run --local-live` / `viola verify`, and a chunk's live readings are its own local measurement, not a suite. This is the precedent for recording the row's live stamp on 2.1.328 as a dated reading.

@history_item n=35 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — specified `run --local-live`: `live-in-ci` refusal, build then verify, and the closed failure codes (`build`, `verify-exit-<n>`, `verify-exit-none`, `row-missing`), each literal row id naming one pass line. It also set the stamped root seam (`stamped` only on exit 0 and `… 0 fail`). These are the rows `contract/local-live-suite/*` and `decision/live-verify-never-in-ci` that the new relied id joins.
