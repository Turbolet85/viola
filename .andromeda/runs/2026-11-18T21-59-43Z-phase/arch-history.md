@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the ledger, verify's probe, the stamps envelope; the version gate split) landed `viola verify` as the one stamps writer (`ledger/stamps.json`, `writer:"verify"`, merge keeps unknown versions, a failing row still stamps and exits 1, `--record` only at 0 fail), the `--version` first-line contract, and `run`'s version gate reading stamps into `cli_version`/`cli_verified`. This chunk's live 24-row verify of `~/.viola` writes its 2.1.325 entry through exactly this path, and the re-pinned product's gate reads it; a failed live verify still replaces that version's entry (the scope's hypothesis rests here).

@history_item n=2 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — registered `run --local-live` (`target/e2e-home/viola-live-<pid>/home`) as the one real-`claude` verify home and `CI` as a harness-only read refusing live runs. Precedent for keeping real-CLI verify local only (`decision/real-cli-probe-local-only`, `decision/no-live-claude-in-ci`); this chunk instead verifies the founder's REAL home, a different target than any harness home.

@history_item n=3 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (decisions on the six-row stamp, the strict stamps read) made `run`'s version gate read stamps through `read_stamps_strict`, a strict-modes refusal recording `cli_verified:false` with `parse-rejected{ledger-stamps, strict-modes-failed}` (`decision/version-gate-strict-stamps-read`). The re-pinned product's gate reads the live home's new stamp this way, so the live home's tree modes must pass.

@history_item n=4 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** — the founder ratified the typed-input `viola verify` probe running the installed `claude` locally on the dev host only, never with a credential on CI. The standing basis for every live verify start this chunk makes under the founder's cap.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (screen signatures compiled, ten rows, verify's interactive runs; registry rows) compiled `SIGNATURES`, made `run` pass the full gate only when `cli_verified`, added verify's PTY runs (Run A untrusted, killed with no key; Run B trusted in `<cwd>/.viola-verify-<pid>/`), `Screen::rows()` verify-only, signature-only `--record` screens, and the CLI-transcript residual. Origin of `decision/verify-never-types-into-dialog`, `decision/screen-rows-read-only-by-verify` and `decision/verify-transcripts-accepted-residual`, which bound the browser-tools line and every live start here.

@history_item n=6 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — added Runs C and D (plansDirectory inside the run dir, +2 transcripts per verify, dialogs answered only by the probe hook). Origin of `decision/verify-plan-files-in-run-dir`; part of what a live verify of the real home leaves under `~/.claude/projects/`, which this chunk's plan must list as files written.

@history_item n=7 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — (seventeen rows, Run B's pastes; registry rows for framing variants) compiled `LOCAL_COMMANDS`, measured `/clear`'s new-session post-condition, and set the rule that a stamp of an older id set reads unverified; the fake agent's `--framing`/`--paste-hint-ms` options. The rule that makes every 21-/22-/23-row stamp of the live home read unverified until this chunk's 24-row re-stamp, and the list `/context` is not on.

@history_item n=8 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `send` consumes the local-command list by exact equality; a listed command with post-condition none, or any on an unverified CLI, answers `ok` `unconfirmable`; `/clear` claims a session-start with a new id. Basis of C4's exposure (an unverified live home runs `/clear`, `/compact`, `/mcp` sends unconfirmable) and of what measuring `/context`'s post-condition would have to feed (`decision/local-command-post-conditions`).

@history_item n=9 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` 8.5 s over the CLI's 8.0 s paste hint, `QUIET_PERIOD` 300 ms, validated per version by the quiet-period row (`decision/gate-timings-compiled`). Trap carried: a verified literal-less screen read through a frozen clock waits forever — relevant to the fake-agent test of verify's browser-tools line.

@history_item n=10 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — test homes' base `target/e2e-home` linked to `/tmp/viola-e2e-home-<uid>` with a strict real-directory check; standing rule: a stalled-start red is a backing finding, never re-run for green. Binds this chunk's TEST starts (each under a test-owned home, `decision/linked-e2e-home-base-strict`).

@history_item n=11 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's Ctrl-C/TERM stop: no stamp, exit 1 `interrupted`, probe dirs removed only by drop guard (never by pattern), SIGKILL leaving them by hand; verify reads `SETTINGS_FLAG` and `PROGRAM`. Governs what a stopped or killed live verify of the real home leaves and writes (`decision/verify-probe-dirs-drop-guarded`, `contract/verify-process-exit/interrupted`).

@history_item n=12 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `[NN/MM]` step lines with `MM` the relied count; "every home stamped before this chunk reads `cli_verified:false` until its next `viola verify`". The recurring re-stamp debt pattern this chunk discharges for the live home.

@history_item n=13 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — (the resume row, Run E; Run E's resources) Run E resumes Run B's first session in its own `-resume/` dir, a verify being six `claude` starts then; `/compact` listed. Origin of `decision/verify-run-e-resumes-run-b-first-session`; part of the live start count and files the founder-forks block must price.

@history_item n=14 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (the agents-join row and Run B's agents read) one `claude agents --json` read inside Run B, the eighth spawn pair; every home stamped before reads unverified until re-verified. Part of `decision/verify-child-set` the live verify runs whole.

@history_item n=15 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the plugin registers viola mcp) `.mcp.json` registers server `viola` with the pinned path and `["mcp"]` (`contract/plugin-mcp-json/viola`); tools reach a session as `mcp__plugin_viola_viola__*`. The server the re-pinned product's plugin points at, and the MCP surface beside which `/mcp disable viola-probe-absent` acts on the real config.

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (`/compact` confirmed by its same-session SessionStart; the twenty-fourth row; Run B's three transcript files) the compact post-condition, its 40 s window, Run B's sixth paste and Run B leaving three transcript files. Precedent for measuring a local command's post-condition live (as `/context` here) and for what Run B writes under `~/.claude/projects/` (`decision/compact-confirmed-by-same-session`, `decision/compact-confirm-window`).

@history_item n=17 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the VIOLA_DIR home step ratified) the CLI verbs' `VIOLA_DIR` step and canonicalisation ratified (`decision/viola-home-resolution`, `decision/session-home-from-viola-dir`). Which home a live verify by path resolves to — the real `~/.viola` default — rests on this order.

@history_item n=18 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (verify's print-mode probe leaves an empty CLI project dir) the founder accepted the print probe's empty `~/.claude/projects/` dir into the transcript residual. The switch itself is the precedent for the operator step this chunk names and stops before (the builder's session moved onto a re-pinned product).

@history_item n=19 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the second input-box literal; the 2.1.302 fixture set) a newer CLI hid `for agents`, so `(shift+tab to cycle)` was added and 2.1.302 re-stamped live 24/0, avoiding `run` refusing `input-not-ready` on a verified version. Nearest precedent for stamping a new CLI version live on the dev host before the product relies on it.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (home-tree check before the start's and verify's log; the pin key taken once; the stamps writer's strict-modes check) `check_home_tree` refuses before any log (verify: home, `ledger/`, `bin/`, `plugin/`), `pin_key`/`pin_bytes` pin the very bytes checked, and `update_stamps` checks strict modes before and under its lock. A widened real `~/.viola` refuses the live verify with exit 1 and writes no stamp (`decision/verify-refuses-untrusted-home-or-stamps`, `decision/stamps-writer-checks-before-and-under-lock`, `decision/run-pins-checked-bytes`).

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — (no-clobber placement; one tree can build two files; placed once, a lost placement re-hashing) the pinned copy is placed with `persist_noclobber` and never replaced, a mismatch refusing `pinned-hash-mismatch`; Measurement B: six cold builds of one tree gave two files, keying two `bin/<key>/` folders. The re-pin rebuild "from a named commit" may key a new copy per build; the kept `viola.2026-10-19` and the rollback copy must survive (`decision/pinned-copy-no-clobber-placement`, `decision/no-deletion-of-pinned-copies`).

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`viola verify`'s refusal codes on its `process-exit`) verify writes its `Ended::Refused` code (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`) beside `interrupted` under a parsed `VIOLA_NAME`; a refused home tree writes no line; a failing row is no refusal; verify takes no `--json`. The surface any new verify output line (the browser-tools words) must not disturb.

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the relied set on every build; one binary name `viola`; verify's stop `SIGINT`/`SIGTERM` alone) every ledger row is relied on every build, the binary and pinned copy are `viola` alone (0700), CI on `ubuntu-latest` only. Fixes the step-line `MM` and the pinned path the re-pin places.

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (Measurement B known, no action in 0.1.0; the statusline-rate-limits row reads the account's plan state; dialog-hold ceiling) Measurement B left with the pinned copy's re-hash as the guard; a live verify of an unchanged CLI failed a row on account state (`23 pass  1 fail`), the dev home's stamps unmoved; an unverified CLI replies `null` to dialogs at once. Traps for this chunk: a live red can be the account's or host's state (the unanswered browser-tools prompt, C5), not the CLI's.

@history_item n=25 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `decision/screen-signatures-compiled` reads the box's own rule and prompt rows beside footer literals; `decision/verify-box-wait-for-literal` waits for the box or a modal; `box_only_guards` in `measured`. The readiness verify's live runs settle on, and the screen reading a browser-tools line must sit beside.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (`decision/mixed-version-tolerance` widened; `decision/run-start-order` lost the budget step) a stamp holding a retired ledger row verifies on the relied rows alone. The old live-home stamps (2.1.296 … .325) carry retired statusline ids; this rule decides how they read until re-stamped.

@history_item n=27 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — removed `--settings` at start and the statusline arm (`absence/status-line-interception`), kept the `--settings` literal for verify only (`decision/claude-flag-literals-in-agent-crate`), and widened `decision/protocol-version-bump-rule` (the stamps' two status-line row ids removed with `v` kept 1). Why the live home's 21-row 2.1.325 stamp holds ids no build relies on, and why the re-pinned product passes the child no settings layer.

@history_item n=28 marker=2026-11-15T18-01-44-wrap
**2026-11-15T18-01-44-wrap** — (the widened protocol-version-bump exception ratified) the stamps' removed status-line ids with `v` 1 stand on the founder's own word. Settles that the live home's stamps envelope needs no migration before the re-stamp.

@history_item n=29 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — the previous live round on the product: dialog-hold ceiling read live and made standing; `decision/wheel-starts-with-driver` names trust keys typed before session start. Nearest precedent for a founder-capped live round on the trimmed product.

@history_item n=30 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — registered StopFailure with no ledger row and no verify probe (the `except` now on `decision/capability-ledger`, `decision/cli-capability-ledger`, `decision/capability-ledger-gate`). The one relied behaviour the 24-row re-stamp does not cover.

@history_item n=31 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified StopFailure's no-row registration, "its live reading owned by «The 2.1.325 recorded fixture set»"; ratified the submitted-form trailing tolerance. Confirms StopFailure stays out of the stamp this chunk writes.

@history_item n=32 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added the 22nd relied row `local-command-echo` (Run B's seventh paste `/mcp disable viola-probe-absent`, passing on a new stdout tag), classification by name plus rest, stdout-echo confirmation by transcript tag count, `hook.event` `transcript?`. The origin of C3/C6: every earlier stamp reads unverified, `/context` was never sent live, and whether that paste writes the real Claude config is unmeasured (`decision/local-command-echo-probe`, `decision/local-command-echo-confirmation`, `decision/local-commands-by-ledger`).

@history_item n=33 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — (the delegate's confirm ratified — 2026-11-16-local-command-delivery, all sections) the local-command-echo rows, echo confirmation and `unconfirmable` widening stand on the founder's word. The rows this chunk re-stamps are ratified, not provisional.

@history_item n=34 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — the test-only `viola-live-rig` and `AGENT_RUN_RIG`; vt100 as the rig's screen model (`decision/vt100-readiness-gate` except, `decision/vt100-screen-model`). The rig's private-config readings are what C4 says this live-home verify must now repeat under the real home.

@history_item n=35 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (Run F and the 23rd row; verify writes and removes its own trust record; CLAUDE_CONFIG_DIR's value read when kept; the trust writer's code words in the stamp) six interactive runs A–F, `decision/verify-own-trust-record` (never creating an absent config, removing only its own record), `decision/trust-record-ledger-row`, `contract/ledger-stamps/rows` admitting writer words. A live verify of the real home writes into and removes from the founder's real `~/.claude.json`; a `config-absent` there fails the row with Run F starting no child.

@history_item n=36 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — the hidden rig-only `--record-first-screens` (whole scrubbed rows, failed verify only), `measured.first_screen`/`first_settle_ms`, the rig seeding its private config. The first-screen words are how a live verify records meeting the browser-tools prompt; the flag is rig-only, so the real-home verify does not pass it.

@history_item n=37 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — a wrapped start on a verified CLI writes the trust record of an untrusted spawn folder (`decision/run-start-order`), `run`/`revive` reading a kept `CLAUDE_CONFIG_DIR`, `key-present`/`removed` mapped to lost at a start; PROVISIONAL (delegate). The re-pinned product gains this write once its stamp verifies — a new effect on the founder's real config the re-pin turns on.

@history_item n=38 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — (verified = every relied row counts, `absent` on browser-tools-prompt alone; `first_screen` gains `browser-tools`; step line `pass|absent|fail`; summary counts an absent row; the 24th row `decision/browser-tools-prompt-ledger-row`; the rig reads the `cachedChromeExtensionInstalled` key name only) all PROVISIONAL (delegate overseer). The contracts this chunk's verify line on the prompt must leave unchanged (stamp and exit), the 24-row count it stamps, and the key-name-only precedent its PREREQ gate (`claudeInChromeDefaultEnabled` by name) follows.
