@history_item n=1 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — set the two refusals `viola send` makes itself before any frame: `not-delivered  control-character` and `not-delivered  empty-text`, each a `[/ ] unable` line on stderr, exit 13, its hint last, one document under `--json`, with the wrapper making the same two refusals for a frame sent straight to it. The signature sentence also gained "or exit 13 when the client refuses the text before any frame". This is the precedent for this chunk's "validated as received, never stripped" (contract/send-refusals/control-character, contract/send-refusals/empty-text, decision/wrapper-repeats-send-refusals, contract/viola-send-output/unable). Trap: it states nothing about whether the issue line prints on a client-side refusal, because nothing measured it. If a whitespace-only text gets its own decision, it lands in this block and must keep client/wrapper parity and its own hint.

@history_item n=2 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — gave the `[RB] read back` line a second trigger: a send is confirmed by the matching `prompt-submitted` or, for `/clear` on a verified CLI, by its new-session post-condition. It kept `clear.txt`'s `not-delivered  input-not-ready` example because the refusal rungs apply to a listed command as to any text. A trim-tolerant match widens what "the matching `prompt-submitted`" means (contract/viola-send-output/read-back). The local-command list (`classify.rs`) reads the exact typed text and must not loosen with it.

@history_item n=3 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` is read back by a `session-start` of cause `compact` inside its own 40 s window, else `not-delivered` / `no-prompt-submitted`. `[  ] unconfirmable` covers `/remote-control` and unverified `/compact` or `/clear`. The verify wireframe went to 24 rows (`local-command-compact` last). This chunk must not move these post-condition outcomes (contract/viola-send-output/read-back-compact, read-back-clear, unconfirmable), and any new ledger row extends the verify list after `local-command-compact`.

@history_item n=4 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `viola send` specimen's `input-not-ready` hint became the founder's wording: `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session` (it was `viola wait …, then send again`). This is the readiness-gate outcome this chunk's turn-race fix sits beside (contract/viola-send-output/input-not-ready-hint). A `turn-running` refusal from the gate must keep its own distinct hint and not reuse this one.

@history_item n=5 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `send` keeps its `[/ ] unable` mirror while `wait` / `last` share the `unable` word column, and every cli verb prints `error: internal error` from the one catch site. This is the origin of decision/shared-refusal-word-column and decision/turn-ended-not-send-outcome. A `turn-running` refusal from the readiness-gate race prints in this mirror form; the turn ending stays `wait`'s result, never a send outcome.

@history_item n=6 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp`'s tools are a driver's machine view: each success or refusal is the CLI verb's `--json` document less `v`, a refusal `isError: true` with `refused: <code>`, bodies codes only, and no tool names `release` / `pause` / `link` / `unlink`. This is where contract/mcp-tools/send, contract/mcp-tool-results/* and decision/mcp-hides-wheel-and-handoff-verbs come from. The scope keeps the MCP surface's shape untouched: the trim-tolerant `ok` must reach MCP `send` only through the existing document, and a wheel fix must not surface `release` in a hint or description.

@history_item n=7 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2) changed contract/viola-send-output/unable from exit 13 (or 10/11/12/14) to 13 (or 10/12/14) and retired exit 11 for good (contract/cli-exit-codes/11: never reused). Exit 14 also reads an older wrapper's `budget-paused` reply as `unknown`. A new send refusal (for example `turn-running`) must map to an existing code and never to 11.

@history_item n=8 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — added contract/viola-answer-output/not-delivered-hints: `viola answer`'s hint table carries the send hints for five more `not-delivered` details. A new or changed send `not-delivered` detail or hint in this chunk is shared with `answer`'s hint table and must stay consistent with it (decision/hint-for-every-cause).

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the no-hint exception now names the `unknown` refusal as "whose detail is a code word or nothing"; `unknown`, `wrapper fault` and `internal error` still take no hint. This is the exception clause of decision/hint-for-every-cause. A new send cause needs a closed code word and its own hint, or it collapses to `unknown`.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the unwrapped-name and endpoint-lost hints; exit 21's `detail` cause; the catch site's `internal-error` document) gave every exit-21 cause its own hint and its own `detail` code, so a driver can tell the causes apart, and made the catch site print `{"v":1,"error":"internal-error","detail":null}` under `--json`. This is the precedent behind decision/hint-for-every-cause and contract/cli-json-mode/one-document, which any new send cause or detail must follow.

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the six frame verbs' exit-21 instance pair; verify's home pair) placed `viola send`'s strict-modes refusal after its text read and before any role file or log opens. It also added verify's home pair (contract/viola-verify-refusals/home-not-private). It fixes the order of send's client-side checks: text validation comes before the instance check, which the "validated as received" work must keep.

@history_item n=12 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — the frame verbs, `send` included in its `[/ ] unable` mirror, refuse exit 21 with no frame written when the server check fails, each cause with its own hint last. This is one of send's pre-frame refusal rungs, alongside the text refusals and the readiness gate this chunk touches.

@history_item n=13 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the verify wireframe gained `long-paste-wrapper`, `tag-escaping` and `local-command-clear`, three ledger rows that probe how the CLI submits a pasted text. This is the precedent for how undocumented CLI submission behaviour gets a ledger row with a `viola verify` probe. The scope leaves P3/P4 to decide whether the trim needs one, and the verify output rows would grow with it.

@history_item n=14 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — added the `paste-newline` row (an LF inside a paste stays inside the one prompt submitted) and gave `MM` its rule rather than a fixed number. It is the nearest precedent for `typed_text`'s CR/LF handling, and a trim row would extend the same ordered list (contract/viola-verify-output/step-line).

@history_item n=15 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — made `MM` the relied count (`LedgerRow::relied()`), one step line per relied row and exit 0 when every relied row passes (contract/viola-verify-output/step-line, exit-0). A new ledger row for the CLI's trim must be relied, or not, by this rule.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop on one OS; the .cmd/.bat refusal leaves revive and verify) set `MM` to all 24 rows on every build, ending at `local-command-compact` and growing as chunks land rows. The stop became `SIGINT` / `SIGTERM` alone, and verify's exit-1 pairs are now not-found, version, fixture, stop and home. This is the current baseline that a trim ledger row would extend (contract/viola-verify-output/*, contract/viola-verify-refusals/*).

@history_item n=17 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added verify's `--help` paragraph (run from a trusted folder; an unapproved external CLAUDE.md import blocks the probe) and the named refusal `a recorded fixture is not clean: <file> <code>` (contract/viola-verify-output/help-paragraph, contract/viola-verify-refusals/fixture-not-clean). This applies to any new recorded fixture or probe this chunk adds for the trim.

@history_item n=18 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — landed `viola verify`'s output structure (step lines, the `stamped` summary, a failing run exits 1 and still stamps) and its `unable:` / `hint:` pairs (CLI not found, unreadable version). This is the origin of contract/viola-verify-output/summary, failing-row, plain-ASCII and the not-found and version-unreadable refusals.

@history_item n=19 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added verify's stopped form: stdout empty, no summary, no stamp, exit 1, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` (contract/viola-verify-output/stop, contract/viola-verify-refusals/stopped, contract/viola-verify/summary). It must stay intact if a probe is added.

@history_item n=20 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the layout master's text into rows, each verified against the code, with divergences settled "as the code shows" or "as written". This is the source of every picked row's current wording. A row that the code no longer matches after this chunk needs a row amendment (`~ {handle}`), not a body edit.

@history_item n=21 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census: no web surface, viola renders no page (absence/web-surface), and the terminal fleet console is no verb's output. This confirms the chunk's send changes are CLI/MCP only.
