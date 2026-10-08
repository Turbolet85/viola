# The prototype's driving guide against the product — the gap list (for the route)

**39 items; 10 stop a switch as the guide stands, in the same five groups research M9 named.** The switch of
the overseer's own driving is not made in this chunk (inputs#I7); this list is for the entry that makes it.

How it was made. Research M9 counted 25 items, 5 stopping, from an explorer's read that was not kept item by
item. This list was re-derived at implement (2026-10-08) by a read-only explorer over the guide's verbatim copy
(inputs#I5, cited as L…) and the product's source, with a `file:line` for every product-side claim. The count
differs by finer splitting, not by a new kind of stop: the ten stops fall into M9's five groups. Checked here
against the source by hand: items 12, 18, 20, 27 and the snapshot's `pending_dialog` of item 6. One claim of
the explorer is corrected (item 10). The other cites are the explorer's and were not each re-read. Not read:
the scripts outside this repository (`vb.sh`, `vbr.sh`, `vstart.sh`, `waitm.sh`, `scr.sh`, `clear-worker.sh`);
what they do is taken from the guide's words.

Classes: STOP (stops a switch as the guide stands) · DIFF (works, differently) · SAME · NONE (no longer needed).

| # | the guide relies on | the product at HEAD | class |
|---|---|---|---|
| 1 | the binary by a path outside PATH (L8-9) | the `viola` bin of this repository (`Cargo.toml:12-14`); the live launcher used `target/release-check/release/viola` | DIFF |
| 2 | `--plugin-dir` passed by the start line (L10) | `run` writes the embedded plugin and puts `--plugin-dir` first itself (`src/run/mod.rs:124-128`) | NONE |
| 3 | state folder `~/.viola/sessions/<name>/` (L11) | `<home>/instances/<name>/`, home `~/.viola` or `--home` (`src/cmd/mod.rs:140-147`) | DIFF |
| 4 | `run` deletes the session folder, so `vstart.sh` archives it (L11) | `run` never deletes; a gone name's dir is reused and its log appended (`src/cmd/run.rs:217-230`, `:291-309`) | NONE |
| 5 | `events.ndjson` with Claude's event names and `viola:*` actions (L11-12) | the same file name; a closed kebab-case kind set, no Claude names (`crates/viola-core/src/lib.rs:46-93`) | DIFF |
| 6 | state files `status`, `wheel`, `dialogs/` (L12) | one `snapshot.json` with `wheel` and `pending_dialog` (`crates/viola-state/src/snapshot.rs:57`), a `heartbeat` file; no status field | DIFF |
| 7 | the `sent/` record of a local command, removed by hand (L12, L69-71) | no `sent/`; `send-issued` / `send-confirmed` / `send-refused` lines without the text (`src/run/send.rs:389-405`) | NONE |
| 8 | `out.raw` and `scr.sh` to see a CLI modal and a ready input box (L12-15, L31-32, L60-63) | no stored screen and no verb that prints one (`src/cmd/mod.rs:36-61`); the pre-send gate reads an in-memory screen model and refuses `input-not-ready` on a verified CLI (`crates/viola-agent-claude/src/screen.rs:143-156`) | **STOP** |
| 9 | session name `viola-builder`, cwd the repository (L13) | the name fits `ViolaName`; the child's cwd is `run`'s | SAME |
| 10 | the founder's start line `vb.sh` / `vstart.sh`, `--dry-run`, `vbr.sh <id>` to resume (L19-25) | `viola [--home H] run <name> -- <program> [args]`, no other flag (`src/cmd/run.rs:34-42`); resume arguments pass after `--`. Corrected: `run` does not need a terminal on stdin (research M5; a start with stdin at `/dev/null` ran here, 07:32Z). Route: "viola revive" (`working-route.md:107`) | **STOP** (the scripts; the capability exists) |
| 11 | the founder answers the trust menu by hand and the wheel still reads `viola` (L26-27) | any editing key moves the wheel to `human` (`src/run/wheel.rs:306-313`); only `release` returns it | DIFF |
| 12 | `list` after start and before every send: state `idle`, wheel `viola` (L27, L31) | no `list` verb (`src/cmd/mod.rs:36-61`; 0 `List` in the enum). The holder word is `driver`. `send` reads the wheel, the turn and readiness itself (`src/run/send.rs:299-359`). Route: "The board: viola list" (`working-route.md:115`) | **STOP** (the verb; `send`'s refusals cover the purpose) |
| 13 | `send <name> '<text>'`, the text an argument (L33) | the text from stdin or `--file`; a second word is a usage error (`src/cmd/send.rs:28-48`) | DIFF |
| 14 | `MSYS_NO_PATHCONV=1` before a `/skill` text (L33) | the text is never an argument | NONE |
| 15 | `wait <name> --timeout 3000` (L34) | `--timeout-ms`, `--after <cursor>`, `--json` (`src/cmd/wait.rs:18-32`); no deadline without the flag | DIFF |
| 16 | wait's first line: `turn-end` with the final text, `question`, `permission`, `plan`, `exited`, `timeout` (L35-36) | `turn-ended  <name>  <time>  cursor <n>`, no text; the text from `last` or `wait --json`; dialog lines carry `dialog <id>`; `session-end`; `timed out` (`src/human.rs:125-152`) | DIFF |
| 17 | a plain-text prompt at a turn end is answered with a `send` (L37) | the same | SAME |
| 18 | `answer <name> 'Label' --note '…'`, all answers in one call (L37-39) | `answer <name> <dialog_id>` with one JSON object on stdin or `--file`: `{"answers": {"<question>": "<answer>"}, "annotations": …}` (`src/cmd/answer.rs:24-37`); the question texts only from `wait --json`; no completeness check; `unverified-cli` without a stamp | DIFF |
| 19 | `answer … allow` / `deny <msg>` (L39) | `{"behavior": "allow"}` / `{"behavior": "deny", "message": "…"}` | DIFF |
| 20 | `answer … allow <n>`, a permission suggestion (L39) | not carried: "a deliberate v1 limit" (`crates/viola-agent-claude/src/dialog.rs:172-173`); plain allow and deny remain | **STOP** |
| 21 | `answer … approve` / `revise <what>` (L39) | `{"behavior": "approve"}` / `{"behavior": "revise", "message": "…"}` | DIFF |
| 22 | delivery confirmed by reading the log for a `UserPromptSubmit` with the text (L40-41) | `send` returns `[RB] read back … cursor <n>` only after the matching `prompt-submitted` is on disk; else exit 13 `no-prompt-submitted` | NONE |
| 23 | background turns: `wait` again, the harness re-wakes the builder (L42-43) | a harness prompt starts a turn without moving the wheel; its `turn-ended` wakes `wait` | SAME |
| 24 | `waitm.sh`: skips short progress turn ends, prints the screen before a dialog, keeps a pid file (L44-50, L75-78) | none here; building blocks `wait --after <cursor> --json`; the screen part has no counterpart (item 32) | **STOP** |
| 25 | a restarted waiter loses nothing: `wait` "reads the current state" (L50) | `wait` without `--after` starts at the log's end at the call (`src/run/wait.rs:127-148`): pass the cursor | DIFF |
| 26 | `clear-worker.sh <name> --then '<skill line>'` (L54) | none here; two sends do it: `/clear` is confirmed by the send itself (item 37) | **STOP** (the script; the capability exists) |
| 27 | a dialog held 3500 s, set in the session's `config.json` and by `vstart.sh` (L55-56, L100) | `DIALOG_DEADLINE` is a compiled 60 s, "PROVISIONAL, unmeasured" (`crates/viola-core/src/lib.rs:16-19`); `run` has no such flag; no route entry names a longer or configurable deadline | **STOP** |
| 28 | at expiry the prototype denies with a message, logs a held dialog, `wait` reports `held`, the answer goes later as a prompt (L55-56) | at the deadline the reply is `null`: the CLI's own menu is the human's; no event at expiry, no `held` kind; a late `answer` is `unknown-dialog` (`src/run/dialog.rs:272-300`, `:343-352`) | **STOP** |
| 29 | Esc on a CLI modal "does not take the wheel" (L60-63) | a lone Esc is an editing key and moves the wheel to `human` (`src/run/wheel.rs:358-369`) | DIFF |
| 30 | harness-injected prompts log their own event and never take the wheel (L64-65) | the same behaviour: `prompt-submitted` with `origin: "harness"` | SAME |
| 31 | the long-paste wrap and its tag escaping are normalised (L66-68) | the same (`crates/viola-agent-claude/src/hook.rs:187-250`) | SAME |
| 32 | the screen before a dialog, because the transcript lags (L72-77) | none: a dialog event carries the questions or the plan only; no verb reads the screen; no route entry names one | **STOP** |
| 33 | tag-like text in a send must not take the wheel (L79-82) | the in-flight send's own prompt is matched on the typed text and relabelled `driver` | SAME |
| 34 | wheel unexpectedly `human`: grep the log, then the overseer runs `release` (L83-85) | `wheel` lines `{holder, cause}`; `release` is the human's verb, refused `release-from-driver` when it carries `from` (`src/run/wheel.rs:193-200`); no hint names it | DIFF |
| 35 | rebuilding under a running session: new hook processes pick up the new binary (L87-91) | hooks run the pinned copy by absolute path, so a rebuild takes effect at the next `run` | DIFF |
| 36 | short one-line sends; newlines or about 800 B get wrapped (L105-106, L148-154) | the same mechanism: one bracketed paste and Enter, trailing LFs dropped, the paste hint waited out | SAME |
| 37 | `/clear` between skills, then reading `SessionStart` for `source: clear` (L111-118) | a send of `/clear` is confirmed by `session-start{cause: "clear"}` with a new session id, on a verified CLI | DIFF |
| 38 | the builder's `ctx NN%` read from the screen before the wrap (L119-124) | none. Route: "Statusline pass-through" (`working-route.md:109`) names rate-limit readings, not a context percentage | **STOP** |
| 39 | a second send while `busy` waits 120 s and exits 1 (L154) | refused at once `not-delivered turn-running`, exit 13 | DIFF |

## The ten stops, by M9's five groups

| M9's group | items | what would close it |
|---|---|---|
| a dialog held until the overseer answers | 27, 28 | a longer or configurable dialog deadline and a record of an expired hold; no route entry names either |
| `list` before every send | 12 | "The board: viola list" (`working-route.md:115`) |
| the screen read (modals, the screen before a dialog, `ctx NN%`) | 8, 32, 38 | no route entry names a screen read; the gate covers part of item 8 |
| `allow <n>` | 20 | a deliberate v1 limit; a ruling, not a route entry |
| the start line and the three scripts | 10, 24, 26 | a rewrite outside this repository, the overseer's and the founder's |

## Measured in this chunk and bearing on a switch

- A live CLI in a real terminal (foot) loses the wheel to the terminal's own replies at its start
  (`evidence/terminal-replies.md`). Until the classifier's list covers them, a product-wrapped builder in a
  terminal window refuses every driver `send`. This stops a switch before any of the ten above.
- Item 11 and item 29 change what the founder does at a start: after any key of his, the wheel is his until he
  runs `release`.
