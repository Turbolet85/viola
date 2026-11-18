@history_item n=1 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — changed contract/viola-verify-output/step-line, /summary and /exit-0: the verdict word gained `absent` for the browser-tools-prompt row alone, `n pass` counts an absent row, exit 0 when every relied row counts; resolved PROVISIONAL (delegate overseer, «Confirm all 26»). This chunk's clean `viola verify --record` on 2.1.325 is read against these rows (24 relied), and C9 (8/24 on an unanswered host, viola never answers the prompt) and C11 (the `bt-absent` control) rest on this entry.

@history_item n=2 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — changed contract/cli-verbs/run, contract/viola-run/start-output and contract/cli-verbs/revive: on a verified CLI the start writes the CLI's trust record for an untrusted session folder, flag only, adding no refusal, exit, hint, `--json` code or terminal byte; revive keys it by the recorded directory. The live starts of this chunk (recording under the rig's private config, C8's 24-row re-stamp) pass through this trust step; it holds only on a verified CLI, so a stale 22/23-row stamp changes the start's behaviour.

@history_item n=3 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — changed contract/viola-send-output/unconfirmable (now /remote-control, /cost, /usage; /compact, /clear or /mcp on an unverified CLI; /mcp with no readable transcript) and added contract/viola-send-output/read-back-mcp (/mcp read back by a new stdout tag in the transcript). C4's live `/context` measurement and the `/mcp disable` check extend this register; `/context` stands in no row yet, and a new post-condition would change these rows.

@history_item n=4 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — changed contract/viola-send-output/read-back: confirmation by a prompt-submitted equal to the typed text or to it less a trailing run of space/TAB/CR/LF; any leading or inner difference unconfirmed (then PROVISIONAL). The live sends this chunk records are judged by that match.

@history_item n=5 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (both entries) ratified the trailing-only tolerance of contract/viola-send-output/read-back on the founder's word; a leading trim waits for the live reading carried on «Standing live rig with a screen model». A live recording here may meet that reading; any widening is a row change owed to the founder, not taken silently.

@history_item n=6 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify's wireframe grew to 24 relied rows (`[24/24] local-command-compact`), `/compact` read back by its compacted-session post-condition inside a 40 s PROVISIONAL window, else `not-delivered`; `/compact` or `/clear` unconfirmable on an unverified CLI. Precedent for how a landed ledger row moves the verify count and the send outcome rows; C3's 22nd-row re-stamp trap (older stamps read unverified) is the same mechanism.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop; the .cmd/.bat refusal leaves verify) `MM` = `LedgerRow::relied()` length, all rows on every build, growing as owning chunks land rows; the stop is SIGINT/SIGTERM with the plain pair and no stamp; verify's exit-1 pairs are not-found, version, fixture, stop and home, no `.cmd`/`.bat` pair. A StopFailure row added under C2 grows `MM` by this rule; the refusals picked here are exactly that set.

@history_item n=8 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count (`LedgerRow::relied()`), one step line per relied row, exit 0 when every relied row passes (since widened by the browser-tools entry). The rule contract/viola-verify-output/step-line and /summary still carry; a new row (C2) extends it.

@history_item n=9 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the verify example went to 18 rows and `MM` was tied to the ledger length; the ledger-order list gains each row as it lands. Precedent that the verify wireframe and ledger-order list move with every row this chunk might add.

@history_item n=10 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22nd row `resume-session` (`--resume <id> reopens the session under that id`) landed, relied on every OS. C10's hypothesis (Run E spawning nothing, `resume-session` fail) is read on this row in the new recording.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify at 23 rows (`agents-join`); `viola revive` gained the session-live pair and the fixed preflight order holder → logged session → `session-live` (skipped under `--fork`, a failed read passes) → recorded directory. Explains picked rows contract/viola-revive/preflight-order and /session-live-check, which the recorded resume and agents probes exercise.

@history_item n=12 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — verify at 17 rows; the examples name 2.1.287, «the one stamped set», and the failing example is the chunk's own first record round. Precedent: the stamped version shown in the layout tracks the recorded fixture set, so a 2.1.325 set may move the example version.

@history_item n=13 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the fixed dirty-recording refusal became `unable: a recorded fixture is not clean: <file> <code>` (screen `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`; never the content), hint unchanged; `viola verify --help` gained the trust-folder / external CLAUDE.md paragraph. This is contract/viola-verify-refusals/fixture-not-clean, the scrub every file of the new set must pass (a payload holding `/home/` refused, C1/C6 record-home rule).

@history_item n=14 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify at 14 rows; Kept: verify never answers the CLI-native external-import dialog, the probe's own dialogs answered by its hook. Precedent for C9's boundary that viola never answers or seeds the browser-tools prompt.

@history_item n=15 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set verify's output structure: step lines, the last stdout line `stamped <ver>  <n> pass  <m> fail`, a failing run exiting 1 with the stamp still written, usage `viola verify [--record <DIR>] [-- <program> [args…]]`, and the unable/hint refusal pairs (CLI not found, version unreadable, dirty recording). Origin of the picked verify rows (usage, failing-row, summary, refusals) that the `--record` run lands under.

@history_item n=16 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a run stopped by Ctrl-C or TERM before its rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, stdout empty, no summary, no stamp, exit 1; the summary closes a completed run only. Picked rows contract/viola-verify-output/stop and contract/viola-verify-refusals/stopped; a live record round killed mid-run leaves no stamp.

@history_item n=17 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch: verify's interrupted pair stands as worded (planning-form mark gone), among other ratifications. The stop pair's wording is fixed, not open to this chunk.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree refusals of verify and the run and revive starts) verify refuses a widened home, `ledger/`, `bin/` or `plugin/` before its log, or a refused stamps write, with the home pair, stdout empty, no step line or summary; run/revive starts refuse a widened home tree before the wrapper log, no child. Picked rows contract/viola-verify-refusals/home-not-private and contract/run-revive-start/home-tree-refused; the record home C6 builds must be private or the live verify refuses.

@history_item n=19 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb run only by the plugin; its five tools are the driver's machine view (success the `--json` ok payload, refusal `isError: true` `refused: <code>`, codes only, no tool naming release/pause/link/unlink). The `Mcp.tools-list-1.json` re-record through `viola-mcp-tap` (C1) records these tools and their descriptions, bound by the picked mcp rows.

@history_item n=20 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — created `viola revive`'s output structure: a passed start prints nothing, `--list` rows `<ts>  <cause>  <id>` with cause `startup`/`clear`/`resume`/`compact` else `unknown`, each refusal exit 1 with one unable/hint pair last on stderr (the no-logged-session pair among them). Origin of the picked revive rows the resume probes touch.

@history_item n=21 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `viola wait` wireframe gained `dialog-expired`, and the session-end annotation now reads «every kind without a dialog_id takes the turn-ended form». That wording stands in picked row contract/viola-wait-output/turn-ended; a recorded StopFailure / turn end (C2) is read through it.

@history_item n=22 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2) changed contract/viola-list-json/document (item and skipped shapes, no `budget`), retired exit 11 and made exit 14 read an older wrapper's `budget-paused` as `unknown`. Recorded payloads replayed by the fake agent must carry no budget member or budget-paused reply as live shape.

@history_item n=23 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built `viola --help` as the grouped table with exactly `Usage: viola [--home <dir>] <verb> [args]` first (picked row contract/viola-help/usage-line). Bears only if the chunk touches verify's help paragraph.

@history_item n=24 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (226), each verified against the code; relied rows kept or cut by the founder. Every picked row's handle dates from this migration; amendments since name rows by handle.

@history_item n=25 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface (absence/web-surface), Linux the one target, no interactive prompt in a verb. Confirms the Linux-only reading of the recording host and that no verb gains an interactive prompt (C9).
