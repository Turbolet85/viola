@history_item n=1 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Brand Identity: 2 rows) added `+ contract/send-readback/mcp` (a `/mcp` send reads back on a verified CLI once a new local-command stdout tag appears in the transcript after the send, no new word) and rewrote `~ contract/send-readback/unconfirmable` (no-post-condition commands `/remote-control`, `/cost`, `/usage`; `/compact` and `/mcp` on an unverified CLI or with no declared transcript). This is the direct precedent `/context` copies: "the send readback contract holds one row a command", so a `/context` stdout-tag confirmer likely needs its own `send-readback` row and an amended `unconfirmable` row naming `/context` on an unverified CLI or undeclared transcript.

@history_item n=2 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` read back by its measured post-condition on a verified CLI and `unconfirmable` on an unverified one; `verify` gained its 24th row (`local-command-compact`), the step counter, `MM` and the summary sample moved with it. Precedent for a local command's post-condition landing with its own ledger row and the verify step list growing by one row with its words.

@history_item n=3 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on a measured post-condition too (first `/clear`), not only on a matching `prompt-submitted`; no new word: a post-condition-confirmed send prints `[RB] read back`, the unconfirmable line reuses the open box. Constrains `/context`: its confirmation must print the existing words (`decision/no-success-before-confirmation`, `contract/viola-send-mirror/read-back`, `contract/viola-send-mirror/unconfirmable`).

@history_item n=4 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the step counter moved with a new relied row (`resume-session`), and pattern 2's `[  ] unconfirmable … local command, no measured post-condition` was stated to cover `/compact` unchanged. The unconfirmable line is the fallback a `/context` send prints where its post-condition is not measured.

@history_item n=5 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — verify's step counter grew to seventeen with `local-command-clear` (/clear starts a new session and submits no prompt), each new row one static ASCII step line with its words. The first local-command ledger row: naming precedent for a `local-command-context`-style row.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify prints one step line per RELIED ledger row, `MM` being `LedgerRow::relied()`'s count, the summary's n + m equal to `MM`. Whether the `/context` row is relied decides whether `MM` and the summary move (`contract/viola-verify-output/step-line`, `contract/viola-verify-output/summary-line`).

@history_item n=7 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the step counter derives from the ledger's row set (`LedgerRow::ALL.len()`), "the count now carries its rule": a new row moves the count without a hand-written figure, matching the scope's boundary of adding no documentation figure.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count on every build) `LedgerRow::relied()` reads 24 on every build, no OS qualifier; the Linux-only target also narrows the cli platform. A new relied row is relied on the one build, so no per-OS split for `/context`.

@history_item n=9 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — (Surface: cli: 3 rows + 1 row) `~ contract/viola-verify-output/step-line`, `~ contract/viola-verify-output/summary`, `~ contract/viola-verify-output/summary-line`, `~ contract/viola-verify-output/trust-record-names-nothing`: the verdict word `absent` on the `browser-tools-prompt` row, counted as a pass on that row alone. The most recent change to the verify rows a new `/context` probe row will print through; a new row ends `pass` or `fail` unless it, too, needs a row-specific word (which would amend these rows).

@history_item n=10 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `+ contract/viola-verify-output/trust-record-names-nothing`: a verify row that touches the Claude config got its own surface promise that no output names the file, key or value. Precedent: a probe with a privacy-sensitive side (the `/context` stdout tag holds session content) may need a row saying verify prints none of it (`decision/no-internals-in-errors`, `decision/hint-never-quotes-text`).

@history_item n=11 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `+ contract/viola-verify-output/browser-tools-line`: a fixed verify stdout line before the summary; added because no design-system row stated that wording. The re-stamp chunk the scope names for the relied/re-stamp question; precedent that any new verify output wording needs its own row.

@history_item n=12 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — origin of `verify`'s step counter (one static stdout line per ledger row, `[NN/MM] <row> <words>  pass`, then `stamped <ver>  <n> pass  <m> fail`), the count growing "as owning chunks land rows"; a failing row prints no stderr word. The pattern a `/context` row joins (`decision/step-counter-only-progress-pattern`, `decision/one-progress-pattern`).

@history_item n=13 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's counter grew with four real-CLI probe rows and gained its `--help` paragraph; the step-line/summary pattern unchanged. Precedent for live-CLI probe rows extending the counter.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree and stamps refusals) a `stamps.json` update refused by its strict-modes check prints the home-state pair, exit 1, no step line or summary. Stands under `contract/viola-verify-output/stamps-refused`, `contract/viola-verify-refusals/stamps-update` and `contract/fixed-refusal-pairs/home-state`: the live stamp of the `/context` row writes `stamps.json` through this path.

@history_item n=15 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `viola verify` stopped by Ctrl-C or TERM prints no step line or summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1 (ratified at 2026-10-20T06-58-31-wrap). Holds for an interactive `/context` probe run (`contract/viola-verify-refusals/interrupted`).

@history_item n=16 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for the same reason or detail; `send`'s `unverified-cli` prints its hint (`contract/refusal-hints/unverified-cli`). A `/context` send on an unverified CLI falls under `unverified-cli` / `unconfirmable` per this keying.

@history_item n=17 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the readback box's one place is its CLI mirror, the Signature Test checking the `[RB]` / `[  ]` / `[/ ]` column of `viola send` alone (`decision/readback-box`, `decision/readback-box-signature-check`); a `/context` readback draws only there.

@history_item n=18 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0, brand, surface and bans scoped to the CLI) the readback box's one place is the CLI mirror; the spinner / progress-bar / live-redraw ban reads "in a verb's output"; status guards (refused send folded, `turn-ended`, nonzero `skipped`) kept. Rows `decision/no-spinners-in-verb-output`, `decision/refused-send-folds-into-send-line`, `decision/turn-ended-never-fills-readback` the send path rests on.

@history_item n=19 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — (registry migration) lifted the Squint Test rule that `open` and `unconfirmable` look identical by design, only the word cell telling them apart (`decision/squint-test`'s except, `contract/readback-box/unconfirmable`); a `/context` send left unconfirmable prints this same drawing.

@history_item n=20 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code; "relied: 45 kept · 12 cut". The register every row this chunk adds or amends extends; a master holds what a planner must respect, the code the rest.

@history_item n=21 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/fixed-phraseology-vocabulary` (`budget-paused` dropped), `~ contract/cli-exit-codes/10-14` (11 retired, never reused), `~ decision/no-hint-names-viola-release` (why narrowed to human-typing). Rows the `/context` send path prints under; no new vocabulary word or exit code is expected.

@history_item n=22 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — the `empty-text` row, hint and exit-13 word: the exit-13 detail list ending `control-character` / `empty-text` (`contract/cli-exit-codes/13`), the not-delivered refusals a `/context` paste can still meet.

@history_item n=23 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the founder's wording of the `input-not-ready` hint; the not-delivered hint a `/context` send that never lands prints, unchanged.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the code word and the screen on a peer's reply (`contract/peer-reply-screening/code-word`, `contract/peer-reply-screening/string-detail-not-code-word`); exit 2's fixed stderr lines with no `--json` document (`decision/json-one-document-on-stdout`'s except); clean `--json` failure documents (`decision/json-failure-documents-clean`). Any new `detail` a `/context` outcome carries must be a code word and keep these documents clean.

@history_item n=25 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — message mode spelled out: every control character but `\n` and `\t` prints as `\xHH`, `--json` serde-escaped (`contract/cli-message-escaping/control-characters`, `contract/cli-message-escaping/json`); bears if `/context`'s tag content ever reaches `wait` / `last` output.
