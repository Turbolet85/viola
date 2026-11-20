@history_item n=1 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** — the source of `decision/permission-request-question-stays-null`. `permission` runs end to end, and a `question` first raised by PermissionRequest stays `null` to the human. On 2.1.288 and 2.1.287 a PermissionRequest `allow` WITH `updatedInput` would satisfy it, but that was read only statically from the installed bundles and never run. It also says a late `answer` to such a question is refused `unknown-dialog`. Why: the ledger is the single gate, and a new row, its probe and the re-stamp cost more live sessions than the cap had left. Kept: viola's bare PermissionRequest `allow` on ExitPlanMode stays ignored. This chunk retires this decision and turns `contract/answer-refusals/unknown-dialog` around. The static reading has to become a live probe.

@history_item n=2 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (the dialog tier served) — `hook.dialog` `{kind, data, hook_event, tool?, input?, continuation?}` → `{dialog_id, response}`. A PermissionRequest that repeats the armed PreToolUse dialog's tool with an equal `tool_input` is that dialog's continuation: no event, no new `dialog_id`, answered only from an armed `plan` revise. The `permission` answer has no suggestion slot (founder ruling F3). PreToolUse (matcher `AskUserQuestion|ExitPlanMode`) and PermissionRequest are registered sync in `hooks.json`. The question-by-PermissionRequest path sits on this continuation and kind-map logic (`contract/hook-to-kind-map/permissionrequest`, `contract/channel-methods/hook-dialog-continuation`).

@history_item n=3 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (decisions on the six-row stamp) — a non-null dialog decision was allowed to flow on a stamp that had no dialog rows. That was a dated gap (founder ruling R2), closed later by the dialog rows. It is the precedent that a dialog decision body must be gated by its own ledger row, which this chunk's new row must follow.

@history_item n=4 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the direct precedent for a dialog-body ledger row. It added `question-answer`, `plan-approve-revise`, `question-notes` and `dialog-concurrency`. It also added Run C (`-dialogs/`, `--settings <DIALOG_SETTINGS>`, three dialog prompts) and Run D (plan mode), whose dialogs are answered only by the probe hook. It added the hidden `--answers <DIR>`: a dialog event's ordinal selects a closed `ProbeAnswer`, and `probe_body` prints one `decision_body`. The S7 row moved to `allow` + `updatedInput` = the tool's own input, because a bare `allow` left the plan dialog up live. The registry half adds the dialog probe plugin, `<Event>.<stem>-<n>.json` variants and the fake agent's `--dialogs`. The new probe will most likely extend Run C, `ProbeAnswer` and the dialog variants. That is a widening of verify's child set, and it needs the founder's ratification.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the 18th row (a new Run B paste) is the template for adding a relied row. Older all-`pass` stamps read unverified, so every stamped home reads `cli_verified:false` until it is re-verified. It renumbered «the hypothetical PermissionRequest `updatedInput` body row» to the 19th. The widening is ratified by the founder's own answer; a delegate's confirm is PROVISIONAL. Trap: adding a row unverifies every existing stamp.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the relied set as `LedgerRow::relied()`, and step lines `[NN/MM]` with `MM` the relied count. The PermissionRequest body row was renumbered to the 22nd. Any row count in the masters moves when this chunk lands.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E: a widening of the verify child set, ratified by the founder's P4 answer. A stamp of the older ids reads unverified on every OS. `--record` writes `SessionStart.resume-1.json`. This is the precedent for a new interactive run and for its spawn pair and probe dir in `diagnostics/`.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (CLI Version Compatibility) — the `agents-join` row. `Probes` gained a capture, and the row fails when nothing is captured. «The next row is the 24th». This is the pattern for a new `Probes` field and its pass predicate.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the 24th row `local-command-compact`, with Run B's sixth paste and `typed_probe` gaining a measured field (additive, no `v` bump). The 2.1.287 set grew a file and was stamped `24 pass`. «The next row would be the 25th». This is a precedent for re-stamping and for the fixture-count rules (`ls fixtures/claude/<v> | wc -l`).

@history_item n=10 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — reworded `decision/permission-request-question-stays-null` from «25th row» to «24th row» when `statusline-rate-limits` left. It also added `decision/protocol-version-bump-rule`'s exception and `decision/mixed-version-tolerance`: a stamp that holds a retired row verifies on the relied rows alone.

@history_item n=11 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — reworded the stays-null row to «22nd row», and left one `--capture` arm (`decision/hooks-fail-open` except). Removing the stamps' two status-line row ids kept every own-format `v` at 1. Any change of this chunk to the stamps envelope is judged against this exception.

@history_item n=12 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added Run F as a sixth interactive run (`decision/verify-child-set`) and the `trust-record` row with its closed writer words in the stamps. It finally reworded the stays-null row to «a further relied row», with no count, so that it would not go stale again. Most recent precedent: a new relied row with its own run, ratified as a widening (delegate confirm PROVISIONAL), with `decision/verify-own-trust-record` as a bounded write exception.

@history_item n=13 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — the verified rule became «every relied row counts», with `absent` accepted on `browser-tools-prompt` alone. Step lines read `pass|absent|fail`, and the summary counts rows that count. The new row will be pass/fail only, unless this chunk argues otherwise. It must enter the same counting rule (`decision/version-verified-all-relied-rows`, `contract/ledger-stamps/*`).

@history_item n=14 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — the latest relied row, `decision/local-command-context-probe`: Run B's eighth paste, where a failing read is fail, never absent. It is ratified inside the founder's «Разрешить пробу» and is PROVISIONAL until his own word. This is the nearest precedent for the ratification path and the counts this chunk will extend. The tree now holds 26 relied rows (scope fact).

@history_item n=15 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — a relied row with its own verify probe, plus `hook.event` gaining an optional param (`transcript?`). This is the precedent for widening a channel frame additively when a new decision body needs more hook context.

@history_item n=16 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the configurable `dialog_hold_s` (60–3600) and `dialog-expired`, which wakes `wait`. The hook's reply bound is `DIALOG_HOLD_MAX + 5 s`, and the dialog hooks' `hooks.json` `timeout` is 3615. `pending_dialog` is written before its event line. A held question raised by PermissionRequest falls under the same hold, expiry and null-at-end (`decision/dialog-hold`, `decision/viola-ends-the-dialog-wait`).

@history_item n=17 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — the dialog hold became standing (no longer provisional), measured at the ceiling. `contract/dialog-hold-end/null-reply` now says expiry ends only the hook's wait, with the CLI's dialog still drawn for the human. This is the «expired hold keeps today's fall-back» boundary in the scope.

@history_item n=18 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the wheel as landed. A move to the human hands back the pending dialog (`null` at once) on its worker thread, and `answer` is refused `human-typing` while the human holds the wheel (`decision/human-wheel-renders-dialogs`). The new answer path must keep both.

@history_item n=19 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the hook's one connect) — a dialog hook whose server check fails sends no `hook.dialog` and exits 0 with no body, so the dialog renders for the human (`decision/dialog-hook-fails-open-on-server-check`). The PermissionRequest question path inherits this fail-open.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the hook's instance check runs before anything under the home on the event and dialog paths. `update_stamps` runs a strict-modes check before and under its lock, and verify refuses a widened home tree before its log. Re-stamping goes through this writer (`decision/stamps-writer-checks-before-and-under-lock`, `decision/verify-refuses-untrusted-home-or-stamps`).

@history_item n=21 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive PTY runs at 80×24. «No key into a CLI dialog» is a founder ruling (`decision/verify-never-types-into-dialog`): the new probe must answer only through the probe hook, never by keystroke. Stamps went per version, and the signature-only `--record` screens started here.

@history_item n=22 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** — the typed-input probe against the live installed `claude` was ratified to run locally only, never on a CI runner (`decision/real-cli-probe-local-only`). The S3/S7/S8 and dialog-concurrency rows were owed to the dialog rows. Any live re-stamp of this chunk stays local.

@history_item n=23 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — 2.1.288 was not re-recorded under the live cap, so it could no longer stamp every row and went drift-only. Trap for the re-stamp choice (2.1.328 alone or 2.1.329 too): a version left un-recorded falls out of the stamped list.

@history_item n=24 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — a new CLI version (2.1.302) got its own fixture set recorded whole by one `viola verify --record` round and was stamped at the relied set beside 2.1.287. This is the precedent for adding 2.1.329's set and for `contract_ledger_probes`' `STAMPED` list.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: all rows on every build) — `LedgerRow::relied()` is every row of `ALL` on every build, with no per-OS filter. A new row is relied everywhere, and `MM` in the step lines is the full count.

@history_item n=26 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (statusline-rate-limits reads the account's plan state) — a relied row whose probe read depended on the account went red on an unchanged CLI and was later retired. Trap: the new probe must read the CLI's behaviour, not an account- or environment-dependent state.

@history_item n=27 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified StopFailure as registered with no ledger row and no verify probe, the one named exception to `decision/capability-ledger-gate` and `decision/cli-capability-ledger`. This chunk takes the opposite path (row and probe) for the PermissionRequest body. The exception's except-cells stand beside any wording change.

@history_item n=28 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** — settled the StopFailure exception on one contained live reading (outcome B, no row). It also set the current `decision/claude-first-no-adapter-trait` / `decision/claude-shapes-in-one-crate` / `decision/claude-shapes-isolated` except-text. The decision body's Claude shape must stay in `viola-agent-claude` (`dialog.rs`) under that rule.

@history_item n=29 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — the founder's ruled limit, «a shape no send relies on needs no probe» (`decision/no-probe-for-unrelied-shapes`, the `capability-ledger-gate` except). The PermissionRequest `allow` + `updatedInput` body is relied on once it is sent, so it falls outside that limit and needs its row.

@history_item n=30 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — plugin precedence was ruled to be no ledger row (`decision/plugin-precedence-no-ledger-row`), because no send relies on it. This is a counter-example that marks where the ledger rule stops.

@history_item n=31 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — the closed ledger, the stamps envelope and its merge rule, the stamps' single writer, and `--record` writing only at 0 fail. It added the hidden `hook --capture` arm, a founder-ratified exception that only verify's probe plugin may register. The new probe's capture and answer arms live under that ratification (`contract/probe-capture/payload-file`).

@history_item n=32 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — the hidden rig-only `--record-first-screens` exception to `decision/verify-record-only-clean` / `decision/recorded-fixtures-scrubbed`, and `measured.first_screen` / `first_settle_ms`. These are the rig-side tools for reading a failing live probe of this chunk.

@history_item n=33 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — the live home's re-stamp and the product re-pin. The scope keeps the live home's re-stamp with that step, and this chunk re-stamps only in the rig's scratch home. It also widened the rig's test-only Claude-shape exception (VH key names).

@history_item n=34 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `viola-live-rig` and `AGENT_RUN_RIG` (test-only, never read by viola), and vt100 as the rig's screen model. This is the instrument for any live start this chunk shapes at P3.

@history_item n=35 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `decision/verify-box-wait-for-literal` and the input-box rows rule. These bind how a new or extended verify run waits before and after its dialog prompts.

@history_item n=36 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — the last chunk. It changed `decision/screen-signatures-compiled` to «no row is a standing modal», with the quote exception (PROVISIONAL, delegate overseer). This chunk's scope is pinned at its commit `1e9f1ea3`.

@history_item n=37 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — verify's `process-exit` refusal codes (`program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, plus `interrupted`) and «a failing row is no refusal». A new failing row adds no code.

@history_item n=38 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `unknown`'s detail is a code word or null, and refusal details come from a closed set. Error bodies carry no upstream text. Answer text and option text crossing into the decision body is content, never a command, and never reaches a human line or an error body.

@history_item n=39 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — `question` data gained `option_details` and `header`. Option text is upstream content carried into the event log, `hook.dialog` and `wait`, never a human line (`decision/option-text-never-in-human-line`). The answers the driver sends back for a PermissionRequest question key on these `question` strings.

@history_item n=40 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP `answer` tool (`{target, dialog_id, response}`), re-validating raw arguments. A schema violation is `invalid-params`. MCP `wait` is capped at 50 s. Only touched if the answer shape must change (scope).

@history_item n=41 marker=2026-10-10-session-links
**2026-10-10-session-links** — an `answer` from another instance records the implicit link once every refusal has passed and the kind matched, before the answer is taken. A link that cannot be recorded is `-32603` and leaves the dialog held (`contract/channel-methods/answer-from-link`). A PermissionRequest question answer runs through the same order.

@history_item n=42 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — every product reader of external bytes is bounded in its own statement or named as an exception, held by `tests/contract_bounded_readers.rs`, with the `serde_json` `unbounded_depth` ban. Any new read of a PermissionRequest payload or probe capture must land bounded, or the census goes red.
