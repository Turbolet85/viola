@history_item n=1 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — added `+ decision/api-error-turn-end-shown-as-turn-end`: an API-error turn ends as a plain `turn-ended` with a closed `cause` word, never the error text, with no new output wording. This is the chunk that registered `StopFailure` with no ledger row and no probe (carry C1). The live payload this chunk records must hold that row: no distinct status, colour or hint, and the error text shown nowhere.

@history_item n=2 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — `~ decision/api-error-turn-end-shown-as-turn-end`: the trace grew from the one word `api-error` to one closed cause word of nine in `--json` and MCP `wait` event data, with human output unchanged. This is the nine-word map that the live payload's `error` word is read against (carry C1). If the reading shows a word outside the map, or one that reaches human output, this row must change.

@history_item n=3 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** (two entries) — `~ decision/api-error-turn-end-shown-as-turn-end` state went from provisional to standing, and the delegate's confirm of the nine-word rule was ratified on the founder's own word. The row this chunk rests on is now standing, not provisional, so any change the live reading forces needs the founder again.

@history_item n=4 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `verify`'s step counter as one static stdout line per ledger row plus a `stamped <ver>  <n> pass  <m> fail` summary, with "the count grows as owning chunks land rows", and a failing row printing no stderr word. This is the precedent for the settlement outcome "a capability-ledger row with its `viola verify` probe".

@history_item n=5 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the step counter moved to the relied count: `MM` is `LedgerRow::relied()`'s count and the summary's n + m equals `MM`. A StopFailure ledger row would change `MM`. Whether it is relied decides whether the counter and the summary move (rows `contract/viola-verify-output/step-line`, `summary`, `summary-line`).

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — `LedgerRow::relied()` reads 24 on every build, with no OS qualifier, and the summary sample is `stamped 2.1.287  24 pass  0 fail`. This is the current count a new StopFailure row would extend. Also from this marker: Linux-only target, CI on `ubuntu-latest` alone.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the 24th ledger row (`local-command-compact`) landed, and the step lines read `[NN/24]`. This is the most recent precedent of a ledger row and its probe landing with a design amendment that names the row's words and the new count.

@history_item n=8 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `+ contract/viola-verify-output/trust-record-names-nothing`: a verify row that writes into the Claude config file got a surface promise that no output names the file, key or value. This is a precedent for a probe touching the CLI's config: a StopFailure probe under a private config or a closed `ANTHROPIC_BASE_URL` may need a like promise that no line names its endpoint or config.

@history_item n=9 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (two entries) — `~ contract/viola-verify-output/step-line`, `~ summary`, `~ trust-record-names-nothing`, `~ summary-line`: the verdict word `absent` was added for the `browser-tools-prompt` row alone, counting as a pass, and the summary's `n` counts it (delegate confirm, PROVISIONAL). This precedent shows that a row whose condition may not be reached live needed a third verdict word. A StopFailure row whose class is unreachable would face the same choice, and the closed `pass`/`absent`/`fail` set currently names only that one row.

@history_item n=10 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `+ contract/viola-verify-output/browser-tools-line`: verify's stdout gained one fixed line before the summary when a run met the browser-tools prompt. This is the latest change to verify's output shape, and a new row's wording must sit beside it. It is also the chunk that re-stamped the live home 24/24.

@history_item n=11 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify stopped by Ctrl-C/TERM prints no step line and no summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1. The source of `contract/viola-verify-refusals/interrupted` and `stopped-or-refused`. Its wording was ratified at 2026-10-20-t06-58-31-wrap. A probe that waits on a lost connection must keep this pair intact.

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted `unable:` / `hint:` pair as worded, along with other Epoch 5 marks. The pair is the founder's own wording, not a planning form, so this chunk may not reword it.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals) — verify checks its home tree before its log, and a refusal prints only the home-state pair, with no step line or summary, exit 1. A refused `stamps.json` update prints the same pair. This is the source of `contract/viola-verify-output/stopped-or-refused` and `contract/viola-verify-refusals/stamps-update`, which a re-stamp with a new row passes through.

@history_item n=14 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the fixed `a recorded payload still holds a path or a username` became `unable: a recorded fixture is not clean: <file> <code>` (closed codes, never the content), and verify gained real-CLI probes. This is the source of `contract/viola-verify-refusals/unclean-fixture`. A live StopFailure payload recorded through the rig's scrub, if kept as a fixture, falls under this check (`home-path` · `absolute-path` · `username` · `email`).

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — added that `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too. This is the source of `decision/silent-catch-sites` and `contract/cli-catch-site/silent-verbs`. A `StopFailure` delivered through `viola hook` must keep the hook silent: exit 0, nothing on stderr (`contract/hook-verb/exit-code`, `contract/hook-verb/stderr`).

@history_item n=16 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `wait`/`last` result lines: every non-dialog kind takes the `turn-ended` form, and `session-end` got its form. This is the source of `contract/viola-wait-output/event-form`, which a StopFailure-ended turn prints through (as `turn-ended`, with no cause in human output).

@history_item n=17 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — narrowed the time form to "every other kind without a `dialog_id`", which is the wording `contract/viola-wait-output/event-form` now holds. This is a trap: an event kind's form is decided by whether it carries `dialog_id`, so a StopFailure turn end must stay a kind without one.

@history_item n=18 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity) — `~ decision/fixed-phraseology-vocabulary`: `budget-paused` was dropped from the fixed vocabulary (`open`, `RB`, `unable`, `human-typing`, `not-delivered`). A new ledger row's step-line words or any new verdict word must fit this closed vocabulary, and the retired word is never reused.

@history_item n=19 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the spinner / progress-bar / live-redrawing ban was scoped to "in a verb's output" (`decision/no-spinners-in-verb-output`), and the CLI status guards (no-spinner `busy`, `turn-ended`) were kept. A verify probe that waits on a lost-connection retry still prints only static step-counter lines (`decision/one-progress-pattern`, `decision/step-counter-only-progress-pattern`).
