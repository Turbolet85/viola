@history_item n=1 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `RefusalDetail::empty-text` as a new `not-delivered` detail, with its own hint and a place at the end of the exit-13 detail list; the hint wording came from the founder, and the plan's own list had to raise it because no detector covers this vocabulary. This is the closest precedent for a new closed word when the paste is cut: it needs a detail row, a hint keyed by detail (or an explicit no-hint), and the exit-13 list extended (rows contract/cli-exit-codes/13, contract/refusal-hints/*).

@history_item n=2 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — reworded the `not-delivered · input-not-ready` hint to `... send again, and if it repeats a human must look at the session`. The old text sent drivers to `viola wait`, which woke on nothing when no turn was running. A hint for a cut paste must likewise not point at a wait that cannot end, and the founder words hints (contract/refusal-hints/not-delivered-input-not-ready).

@history_item n=3 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — changed contract/refusal-hints/refusal-line: `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint. Any new paste-bound detail therefore binds `answer`'s hint too, or it prints no hint.

@history_item n=4 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — made `unknown`'s detail a code word or nothing (exit 14, no hint), and defined the code word with both clients screening it (CLI `reply_of`, MCP `of_reply`). A code word outside the closed refusal list stays visible and exits 14. It also closed exit 20's `--json` detail (fixed `wrapper fault` message, MCP the same), gave exit 2 fixed stderr lines with no document (an unusable `viola send`), and added the `tests/cli_error_bodies.rs` witness (no ESC, path or upstream text). These set how the closed word at the bound reaches the CLI and the MCP `send` tool, and what an older client reads when it does not know the word (rows peer-reply-screening/*, wrapper-fault-detail/*, decision/json-failure-documents-clean, decision/json-one-document-on-stdout).

@history_item n=5 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — retired exit 11 (`budget-paused`) and recorded that it is never reused; an older wrapper's `budget-paused` reads `unknown`, exit 14. It also cut `budget-paused` from decision/fixed-phraseology-vocabulary, gave decision/no-hint-names-viola-release a reason that cites the human-typing refusal alone, and kept WHEEL at 23. A new paste-bound word must not take exit 11, joins the fixed vocabulary, and its hint must not name `viola release`.

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — put exit 21's cause into `detail` as one of a closed set of words. Widening that set took the founder's own word after the widening was shown to him, and the delegate's confirm was PROVISIONAL. The entry also added the `endpoint-lost` hint for a wrapper lost while the call was open, and listed D-20's log detail codes. Two precedents follow: adding a closed word at a time bound is a widening that needs the founder's word, and a call cut mid-flight already has its own wording.

@history_item n=7 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder ratified the `endpoint-lost` hint wording and the reading of an unwrapped name from its instance directory. This is the standing wording a new "call cut" hint should stay consistent with.

@history_item n=8 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — added `socket-dir-refused` as exit 21's sixth cause, a log code and revive's eleventh code. The closed word was the founder's own answer, given after the widening was shown to him, and its pair's wording was a planning form until his batch. The wrapper's silent drop of a refused peer writes no line and moves no wheel. This is a precedent for adding a closed word and for "no wheel moved" at a cut.

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — when `send`'s strict-modes check refuses (checked after reading its text), it prints the fixed instance-state pair, not the `[/ ]` mirror, and writes no frame. This shows where `send`'s refusals take the mirror form (contract/viola-send-mirror/unable) and where they do not, which a paste-bound refusal must place itself against.

@history_item n=10 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — defined a match in contract/send-readback/prompt as the sent text, or the sent text with its trailing whitespace run removed; a prompt trimmed at the start does not match. This bears on what a cut paste leaves in the input box, and on whether a later send or the human's Enter can be read back as a match.

@history_item n=11 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only match in contract/send-readback/prompt; a leading trim waits on a live reading. The confirmation rule a cut paste meets is standing, not provisional.

@history_item n=12 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on a matching `prompt-submitted` or on a measured post-condition, never optimistically. A send cut at the bound must therefore stay unfilled and end `unable` (decision/no-optimistic-send-confirmation, decision/no-success-before-confirmation).

@history_item n=13 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — widened contract/send-readback/unconfirmable to local commands with no post-condition and to `/mcp` on an unverified CLI or with no declared transcript, and added `/mcp`'s echo as a confirmer. This is the row a cut local-command paste would otherwise fall under.

@history_item n=14 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added `/context` to contract/send-readback/unconfirmable and gave it its own post-condition row. This is the current form of the unconfirmable row in the picked set.

@history_item n=15 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — added the `paste-newline` ledger row: an LF inside a paste stays inside the one prompt submitted. This is the paste-framing behaviour the bounded paste write must not break.

@history_item n=16 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — added the `long-paste-wrapper` and `tag-escaping` rows (a long paste unwraps to the text as pasted). Long pastes are exactly where a paste write blocks on a full PTY buffer, so the bound must keep these framings intact.

@history_item n=17 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — added decision/api-error-turn-end-shown-as-turn-end: a closed `cause` word, never the error text, with no new output wording. It was provisional on the delegate's word. This is a precedent for a closed word that reaches `--json` and MCP without upstream text.

@history_item n=18 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — grew that cause into a closed set of nine words reaching `--json` and MCP `wait`, with no change to human output. The founder ratified it at 2026-11-17T05-21-35Z-wrap. This is the precedent for widening a closed set across the CLI and MCP together.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — gave the catch site a `--json` document, `internal-error`; `run`, `revive`'s start, `mcp` and `hook` stay silent there. Faults print no hint (rows decision/no-hint-for-faults, contract/cli-catch-site/silent-verbs). A paste failure must be a typed refusal, not a fault.

@history_item n=20 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — scoped the ban on spinners, progress bars and live-redrawing dashboards to a verb's output (decision/no-spinners-in-verb-output). A blocked paste shows nothing in progress: `send` prints its static open line and then its result.

@history_item n=21 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — made `viola mcp` a hidden verb run by the plugin, whose stdout carries MCP frames only (contract/mcp-verb/stdout). The MCP `send` tool's result at the bound travels through these frames.

@history_item n=22 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — spelled out message mode, `\xHH` escaping, and that `--json` stays serde-escaped (contract/cli-message-escaping/json). This governs any text a cut-paste result might carry; the closed word itself carries none.

@history_item n=23 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows verified against the code. This is the baseline every picked row descends from; a row this chunk changes is amended by its handle.
