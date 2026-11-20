@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (verify's six-row step counter and its exit-1 phraseology) set `viola verify`'s pattern: one static stdout step line per ledger row, then the `stamped <ver>  <n> pass  <m> fail` summary, with "the count grows as owning chunks land rows"; a failing row prints no stderr word. This chunk's new relied row for the PermissionRequest question `allow` body is another row of that kind, and it lands under this pattern (contract/viola-verify-output/step-line, summary, summary-line, fail-word).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the step counter grew from 6 to 10 rows as probes landed, and verify's exit-1 refusals got the named unclean-fixture form (`a recorded fixture is not clean: <file> <code>`, never the content). The new probe's recorded fixtures under `fixtures/claude/<version>/` must pass that cleanliness screen (contract/viola-verify-refusals/unclean-fixture).

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the step counter went to `[01/14]` with the four dialog rows named (`question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency`). This is the direct precedent: the existing `question-answer` row covers the answer path this chunk extends to a question first raised through PermissionRequest. The new row sits next to these dialog rows and needs its own words.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — verify went to 17 rows, each new row one static ASCII step line named with its words, and the example version followed the one stamped set. This is a precedent for how a row-adding chunk rewords the counter and the summary example.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the counter derives from `LedgerRow::ALL.len()` (18 rows), and "the count now carries its rule". The design rows state counts by rule (`MM`) rather than by a fixed number, so adding a row should not need a new literal count.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the count of relied rows (`LedgerRow::relied()`), not of all rows, and the summary's n + m equals `MM`. The new row must be relied (the scope gates the dialog decision on it), so it enters `MM`, and every stamp of the earlier set falls short of it.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22nd row (`resume-session`) was added at the end of the ledger order, relied on every OS. This is a precedent for appending a relied row and moving the counter and summary sample.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (verify bullet) `[01/23]`, with `agents-join` added last to the member list, is another precedent for a relied row added at the end of the order.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify went to `[NN/24]` with a row that gates a behaviour on a verified CLI (`/compact` read back only on a verified CLI, `unconfirmable` otherwise). This chunk follows the same shape: the question `allow` decision is gated on a verify stamp that counts every relied row. It also bears on decision/no-optimistic-send-confirmation and decision/no-success-before-confirmation.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count on every build) `LedgerRow::relied()` reads all rows on every build, with no OS-split relied set; `shim-resolution` passes when `claude --version` answers. The new row is relied with no OS qualifier. A sub-entry of the same marker (the .cmd/.bat refusal retired) removed verify's `.cmd`/`.bat` refusal, so verify's exit-1 refusals are the remaining ones (contract/viola-verify-refusals/*).

@history_item n=11 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `+ contract/viola-verify-output/trust-record-names-nothing`: a new ledger row whose probe has a side effect got a design row promising that its output names no file, key or value. This is a precedent for whether the new probe needs a design row of its own. The probe's output must likewise never print answer or option text (decision/no-internals-in-errors, decision/hint-never-quotes-text).

@history_item n=12 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ …/summary`, `~ …/trust-record-names-nothing`, `~ …/summary-line`: the verdict word `absent`, written only on the `browser-tools-prompt` row, counts as a pass. The step and summary rows now name that row explicitly. A new relied row must not disturb that special case, and must say whether it has an `absent` reading of its own (for example, when no run met a question raised through PermissionRequest) or only `pass`/`fail`. These changes were the delegate's PROVISIONAL confirm.

@history_item n=13 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `+ contract/viola-verify-output/browser-tools-line`: verify gained one fixed stdout line before the summary. This is the most recent re-stamp chunk. The scope leaves the live home's re-stamp to that re-pin step, and this chunk re-stamps only in the rig's scratch home.

@history_item n=14 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/cli-verification/verified` and `~ contract/cli-verification/unverified-cli`: a stamp that lacks a relied row, or reads one `fail`, no longer reads `verified`, so the CLI runs transport only as `unverified-cli` until re-stamped. This is the constraint behind this chunk's re-stamp: adding the relied row makes every existing stamp (2.1.328's 24- and 26-row stamps) read `unverified-cli` until it is re-stamped against the larger set.

@history_item n=15 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — gave `viola wait`'s dialog lines `dialog unknown` for an event without a `dialog_id`, and the `\xHH` message-mode escaping. If a question raised through PermissionRequest reaches `wait` as `question`, it uses this form (contract/viola-wait-output/dialog-form, contract/cli-message-escaping/control-characters).

@history_item n=16 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog-expired` takes the dialog form (`<kind>  <name>  dialog <id>  cursor <n>`) because the expired-hold record carries `dialog_id`. The scope keeps "an expired dialog hold keeps today's conservative fall-back", so the question raised through PermissionRequest must keep this expiry line and form.

@history_item n=17 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the `dialog-expired` wait line and verify's interrupted `unable:` / `hint:` pair as worded. Both are standing wording this chunk must not move (contract/viola-wait-output/dialog-form, contract/viola-verify-refusals/interrupted).

@history_item n=18 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — set verify's interrupted run to no step line, no summary, the fixed pair, and exit 1. A new probe must keep that behaviour when Ctrl-C or TERM stops a run (contract/viola-verify-output/stopped-or-refused).

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — two sub-entries apply. (verify's home-tree and stamps refusals) verify checks its home tree before its log, and a refused `stamps.json` update prints the home-state pair and exits 1; the re-stamp goes through this path (contract/viola-verify-refusals/stamps-update, home-tree-check). (the six frame verbs' strict-modes refusal) `answer` runs the instance strict-modes check before any frame and prints the instance-state pair, exit 21 (contract/cli-refusal-line/fixed-unable-form).

@history_item n=20 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `answer`'s not-delivered, human-typing and unverified-cli refusals carry the same hint `send` holds for the same reason or detail, and an opaque detail or none gets no hint. Any refusal the new path adds to `answer` (unknown-dialog, unverified-cli) follows this rule (contract/refusal-hints/not-delivered-unknown-dialog, unverified-cli, human-typing).

@history_item n=21 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — four sub-entries apply. Exit 2's fixed `UNUSABLE_RESPONSE` line for an oversize `viola answer` gets no `--json` document (contract/cli-exit-codes/2, decision/json-one-document-on-stdout). Exit 20's closed `detail` and `release-from-driver` (decision/no-hint-for-faults, decision/no-hint-names-viola-release). `unknown`'s detail is a code word or nothing, and it gets no hint. The `--json` failure witness holds no ESC, path or upstream text (decision/json-failure-documents-clean, guard `tests/cli_error_bodies.rs`). The answers the driver carries are upstream or driver content and must never reach an error document or text.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — four sub-entries apply. Exit 21's document names its cause (contract/exit-21-json-cause/endpoint-lost). The `endpoint-lost` hint (contract/refusal-hints/endpoint-lost). The catch site's `internal-error` document, with `run`, `revive`'s start, `mcp` and `hook` silent there under `--json` too (contract/cli-catch-site/*, decision/silent-catch-sites). The `hook` path that carries the PermissionRequest decision stays silent and exits 0 (contract/hook-verb/*).

@history_item n=23 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder ratified the wording of the `endpoint-lost` hint, so contract/refusal-hints/endpoint-lost is standing wording, which `answer` prints when its wrapper is lost mid-call.

@history_item n=24 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the server-verification cause and hint cover eight frame verbs) `answer` is one of the eight frame-writing verbs that print the server-verify hint byte for byte (contract/refusal-hints/server-verify). An `answer` that carries answers still goes through that identity check before any frame.

@history_item n=25 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — several sub-entries apply. `~ decision/fixed-phraseology-vocabulary` dropped `budget-paused`. `~ contract/cli-exit-codes/10-14` and `~ contract/cli-exit-codes/11`: 11 is retired and never reused. `~ decision/no-hint-names-viola-release`'s why now names only the human-typing refusal. Any new refusal or exit code this chunk adds must not reuse 11, and must use only the fixed vocabulary.

@history_item n=26 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — set the rule that no hint names `viola release` and the fixed `unable: <text>` form for exit-1 refusals (decision/no-hint-names-release, contract/cli-stderr/fixed-message-refusal). Any hint on the answer path must keep to them.

@history_item n=27 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer: the `DIALOG` word of a pending dialog in the attention token with its kind word plain, and stale rows dim (contract/cli-colour-tokens/attention, dim, stale). A question raised through PermissionRequest shows on the board as `DIALOG question` under this rule, and now clears once the driver answers.

@history_item n=28 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the cocked strip keeps its slot and shows `DIALOG` plus the kind, amber on the CLI (decision/dialog-pending-cocked-strip). This is the board's register for a pending dialog, which the new answer path resolves.

@history_item n=29 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (brand, surface and bans scoped to the CLI) the spinner, progress-bar and live-redraw ban is scoped to verb output, and the brand is CLI-only. The new probe's verify output stays static step lines (decision/no-spinners-in-verb-output, decision/step-counter-only-progress-pattern).

@history_item n=30 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows verified against the code; the code holds the rest. Wording this chunk changes (for example, the ledger's row list and count, or the verify probes) is amended through `~`/`+` row entries, not through prose.
