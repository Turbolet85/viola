@history_item n=1 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — the wrapper began serving `hook.event`, re-validated (five hook kinds, object `data`, `prompt-submitted` `text` a string, `origin` harness|human; an invalid event is not appended). If a local command's echo is read as a hook event, it passes this re-validation (contract/channel-frames/hook-event-revalidation, contract/wrapper-channel-methods/hook-event); a new kind or field there is a change to that closed check.

@history_item n=2 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (test seam `FAKE_AGENT_HOOK_PANIC`) a second fake-agent-only seam needed its own Decisions Log entry and the founder's live ratification. Precedent for this chunk: if the fake agent's `--local-command-mode` grows a new environment read or seam, decision/new-env-read-needs-log-entry and decision/test-seams-fake-agent-only require its own entry; it must not reach a release build (decision/release-build-without-test-features).

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the hidden `hook --capture` arm, probe captures, fixture scrub) set how verify's probes capture raw hook payloads (0600 exclusive files, no `VIOLA_*` read, exit 0) and how `--record` scrubs fixtures. A verify probe for a local command's echo reuses this capture arm and scrub (decision/hook-capture-arm, decision/verify-probe-captures-transient); widening the arm was a founder-ratified boundary widening.

@history_item n=4 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — fixed that harness prefixes are only compiled literals matched on the raw start, never built from runtime or upstream text. The same rule binds local-command lists (decision/compiled-classification-sources): whatever counts as a local command's echo must come from compiled `viola-agent-claude` ledger literals, never from the command's own output.

@history_item n=5 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 feed landed off the passthrough under a panic catch; a poisoned model reads `input-not-ready`, which confirmed `send` reports `not-delivered`/`input-not-ready`. If the echo is read off the screen model, it inherits this degrade (decision/vt100-feed-under-panic-catch, decision/vt100-feed-isolated, decision/vt100-inside-catch-unwind): a poisoned model must not confirm a local command.

@history_item n=6 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (send's input validation as landed; the feed bound closed) `validate_paste_text` runs client-side in `viola send` and first in the wrapper's `send`; the paste is one bracketed `write_all`; the tee→feed queue bounded at 256 with a dropped copy poisoning the model. The send path this chunk changes keeps that order and the one-paste write (decision/paste-validation-on-both-sides, decision/send-text-is-one-bracketed-paste, decision/vt100-feed-bounded-queue).

@history_item n=7 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (run's strict stamps read) `run`'s version gate reads `ledger/stamps.json` through `read_stamps_strict`; a refusal, unreadable or malformed file is `cli_verified:false`. `src/run/version_gate.rs` is in this chunk's surfaces: a listed command on an unverified CLI already reads `unconfirmable`, and any new echo ledger row's stamp is read only through this strict path (decision/version-gate-strict-stamps-read, contract/strict-modes-refusal/version-gate).

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive PTY children were set as direct spawns under R8 at 80×24, pasting only compiled literals, read only against compiled signature literals, never a byte into a dialog, a modal start killed with no key, each a founder-ratified widening. A verify probe for the echo must stay inside these rules (decision/verify-children-direct-spawn, decision/verify-never-answers-dialogs, decision/verify-signature-only, decision/verify-real-cli-transcripts-accepted).

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B gained compiled pastes including `PROBE_LOCAL_COMMAND` = `/clear` (opening a second CLI session), each only into a settled input box; stamp 14 → 17 rows. Direct precedent: adding a local-command paste or row to verify is a boundary widening of the verify child set that needed the founder's own live answer, and changes the stamp's relied set and its transcript residual.

@history_item n=10 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — the typed text (validated, then trailing LF removed) became the only text `send` uses: local-command classification, the confirmation match, `text_bytes` and the paste. Local-command recognition in this chunk must classify the typed text, not the received text; any other removal from a sent text needs its own ruling (decision/refused-text-never-stripped, decision/reject-dont-strip-paste-text).

@history_item n=11 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (trailing CR and LF removed; an empty typed text refused) a listed local command followed by LF, CR or CRLF is that command; an empty typed text is refused `empty-text` client-side and in the wrapper before the first wheel read. Classification of a local command and its echo match rest on this typed form, and the refusal order stays (decision/empty-typed-text-refused, contract/send-refusal/empty-text, contract/channel-refusal/empty-text).

@history_item n=12 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — inner CR / CRLF typed as one LF after validation; the typed text is what is classified and pasted. Typing any character the received text did not hold, other than this LF, needs its own ruling — a trap if the echo path tempted a rewrite of the command text (decision/typed-text-line-ending-normalisation, decision/bridge-send-one-bracketed-paste, decision/refuse-control-characters).

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — added the human's-terminal stdin vector: a typing byte takes the wheel and the next automation send is refused. A local-command send stays behind this refusal (contract/send-refusal/human-typing); the echo path changes no wheel rule.

@history_item n=14 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — Run B's fifth compiled paste and a new ledger row (`paste-newline`), stamp 17 → 18; the paste a widening ratified by the founder's own answer to a P4 fork. Precedent for this chunk's possible echo ledger row with a verify probe: a new relied row moves the stamp counter and makes an older all-pass stamp read `cli_verified:false`.

@history_item n=15 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — a relied-on CLI behaviour owed a ledger row became measured, stamped rows with fixtures, and the relied set and stamp counter moved (`/21`). Precedent: an undocumented CLI echo this chunk relies on becomes a ledger row with a probe, joins the relied set, and changes what a stamp must cover (decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp).

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's sixth paste `/compact` (`LOCAL_COMMANDS[2].0`), its answer waited up to `PROBE_DEADLINE`, its backup-file residual measured, stamp 23 → 24 rows with `local-command-compact` relied. Nearest precedent: a per-local-command post-condition row and probe were a founder-ratified widening of the verify child set; this chunk's echo row/probe would follow the same path and residual accounting.

@history_item n=17 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — Run B's pastes go into rows holding *an* input-box literal (two compiled literals now); and a scratch footer reading of a newer CLI (one start, no key, scrubbed footer rows only) was accepted as no standing permission. If the echo's 2.1.325 shape needs a live reading, decision/scratch-footer-reading-residual says another such reading needs its own answer.

@history_item n=18 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (MCP tool input validation and error bodies as landed) the MCP `send` tool validates raw arguments, runs `validate_paste_text` then the empty-typed-text check before any frame, and returns closed codes only. The MCP `send` outcome this chunk touches only as it reads the wrapper's reply stays under these rules (contract/mcp-tools/send, decision/mcp-handlers-validate-raw-arguments, contract/mcp-tool-inputs/invalid-params).

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the bounded-reader census `product_readers_are_each_bounded_or_named` and the vt100 passthrough witness after a caught feed panic. Any new reader the echo needs (a probe capture, child output) must be bounded in its statement or named in the census (decision/bounded-readers-census, decision/max-frame-cap).

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the clients' screen of a peer's reply) clients take a refusal or detail only as a validated code word and a fault only through `sanitised_fault`; the error-body census names every writer. A new or changed send outcome detail for local commands reaches CLI `--json` and MCP only through this screen and must be a closed code word (decision/client-reads-closed-values-only, decision/client-reply-code-words-only, decision/reply-code-words-validated, decision/every-error-writer-named).

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the statusline shell-out, the settings override and the relied set hold on every build) `LedgerRow::relied()` became all twenty-four rows on every build, counter `/24`. A new echo ledger row extends that one relied set and counter with no per-OS split.

@history_item n=22 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (statusline-rate-limits reads the account's plan) a relied row read red on an unchanged CLI (`23 pass  1 fail`) because it measured the account, not the CLI. Trap for an echo row: it must measure CLI behaviour stably, or a stamp reads `cli_verified:false` and listed local commands fall back to `unconfirmable`.

@history_item n=23 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — added decision/input-box-readiness-from-literals (one predicate for verify and run's pre-send gate, compiled literals only, never under a modal) and narrowed screen fixtures to signature rows. If the echo is a screen signal, it is read by compiled literals through that model and recorded only as signature rows (decision/verify-screen-read-against-literals).

@history_item n=24 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added decision/live-round-rig-keys-own-pty (each live round needs the founder's word anew) and decision/live-session-screens-kept-local. The scope's possible live start to read the echo on 2.1.325 falls under both: ask before the review card, keep screen frames local, commit only closed-word summaries.

@history_item n=25 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — StopFailure registered as a named exception to the unverified-CLI gate (no ledger row, no verify probe, owned by the 2.1.325 recorded fixture set); hook.event re-validation gained the closed turn-end cause set; API-error text dropped at the parse. Precedent for how this chunk decides whether a relied echo gets a ledger row or a named exception, and for dropping upstream text at the parse.

@history_item n=26 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — delivery confirmation widened from exact match to decision/delivery-confirmation-trailing-tolerance; decision/typed-text-line-ending-normalisation lost its "exact equality" why. The scope leaves this match untouched for ordinary sends; the local-command echo path must not loosen it further (leading trim still waits on a live reading).

@history_item n=27 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance (leading trim deferred to "Standing live rig with a screen model") and StopFailure's no-ledger-row exception (decision/unverified-cli-gate now standing without the provisional mark). Both stand as the confirmation and gate rules this chunk builds beside.
