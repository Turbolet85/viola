@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s step counter (one static stdout line per ledger row, then `stamped <ver>  <n> pass  <m> fail`) and its exit-1 refusals, the recorded-payload refusal among them; the origin of the step-line / summary rows a new stamped set is recorded and checked under.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — made the recorded-payload refusal the named `unable: a recorded fixture is not clean: <file> <code>` (screen form `<file> row <n>[ seam] <code>`, closed codes, never the content; founder's ruling) and added `verify --help`'s trusted-folder paragraph; this is `contract/viola-verify-refusals/unclean-fixture`, the hygiene a `verify --record` of 2.1.328 must pass, never a hand scrub.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the counter grew to fourteen rows with the four dialog rows; precedent that the summary's example version follows the stamped set and the step-line pattern stays unchanged as rows land.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — seventeen rows (`long-paste-wrapper`, `tag-escaping`, `local-command-clear`); the summary example "follows the one stamped set" — precedent for how a new stamped set (2.1.328) relates to the design's example version.

@history_item n=5 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on `/clear`'s measured post-condition (a `session-start` cause `clear`), not only on `prompt-submitted`; root of `contract/send-readback/clear`, which the recorded set's local-command exchanges must keep confirming.

@history_item n=6 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C/TERM before its rows prints no step line or summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1 (`contract/viola-verify-refusals/interrupted`); binds an interrupted live `--record` start.

@history_item n=7 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — eighteenth row `paste-newline`; the step counter derives from `LedgerRow::ALL.len()`, so the count carries its rule rather than a fixed number.

@history_item n=8 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the step lines count the relied rows (`MM` = `LedgerRow::relied()`), the statusline rows added; origin of `MM` in `contract/viola-verify-output/step-line` and `summary`.

@history_item n=9 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — `resume-session` row added (22); `/compact` still `unconfirmable` in pattern 2 at that time (later superseded by n=11).

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (verify part) `agents-join` row added, `[01/23]`; one more ledger row the recorded set must carry exchanges for.

@history_item n=11 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — a `/compact` send on a verified CLI reads back on a `session-start` of cause `compact` with the session's own id; unverified stays `unconfirmable`; 24th row `local-command-compact`; root of `contract/send-readback/compact` and the 24-row stamp the 2.1.328 set extends.

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's word on verify's interrupted pair (and other Epoch 5 forms): the pair stands as worded, no longer a planning form.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree and stamps refusals) before its log `verify` checks the home, `ledger/`, `bin/`, `plugin/`; a refusal prints only the home-state pair, exit 1; a refused `stamps.json` update prints the same pair (`contract/viola-verify-refusals/home-tree-check`, `stamps-update`, `contract/viola-verify-output/stamps-refused`). Also: a home under a regular file is a strict-modes refusal (`contract/strict-modes/home-under-regular-file`), and the six frame verbs' strict-modes refusal takes the fixed instance-state pair (the `fixed-unable-form` lists). Binds the rig's private-config live starts.

@history_item n=14 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — the stderr rule's fixed `unable: <text>` form named for the exit-1 refusals of `run`, `revive` and `verify`; origin of `contract/cli-refusal-line/fixed-unable-form` and `contract/cli-stderr/fixed-message-refusal`.

@history_item n=15 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `empty-text` to exit 13's detail list with its founder-worded hint; the exit-13 row (`contract/cli-exit-codes/13`) a recorded send exchange can land in.

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the code word (1-64 bytes, lower-case first) through which both clients screen a peer's `refusal` / `detail` (`contract/peer-reply-screening/code-word`); `unknown`'s detail a code word or nothing; the `--json` failures hold no path or upstream text — relevant to what recorded MCP-tap frames may carry.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the catch site) `{"v":1,"error":"internal-error","detail":null}` under `--json`; `run`, `revive`'s start, `mcp` and `hook` silent at the catch site (`contract/cli-exit-codes/1`, `contract/cli-catch-site/silent-verbs`).

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — `LedgerRow::relied()` reads 24 on every build, no OS qualifier, `shim-resolution` passes when `claude --version` answers; verify's `.cmd`/`.bat` refusal retired (verify's refusal codes five); a home under a regular file read with no OS arm; Linux-only target — the relied count and refusal set the 2.1.328 stamp and recording run under.

@history_item n=19 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the readback box's one place is its CLI mirror (`[RB]` / `[  ]` / `[/ ]` of `viola send`); root of `decision/readback-box` and `contract/readback-box/cli-mirror`.

@history_item n=20 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: brand, surface and bans scoped to the CLI; the readback box's one place the CLI mirror; the spinner / live-redraw ban scoped to a verb's output (`decision/no-spinners-in-verb-output`); page tokens retired (the N/A absences in the rows file).

@history_item n=21 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (every area), each verified against the code; the handles in the rows file come from this conversion.

@history_item n=22 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` share the hint for a reason/detail, and a `not-delivered` refusal with an opaque or absent detail prints no hint.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/fixed-phraseology-vocabulary` (`budget-paused` dropped from the vocabulary), `~ contract/cli-exit-codes/10-14` (11 retired, never reused), `~ decision/no-hint-names-viola-release` (why names only human-typing); a recorded fixture or fake-agent literal must not resurrect `budget-paused`.

@history_item n=24 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/send-readback/prompt`: the match tolerates the trailing run of ASCII space/TAB/CR/LF the CLI trims, a leading trim never matches; the readback the recorded `prompt-submitted` exchanges are checked against.

@history_item n=25 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the trailing-only tolerance ratified by the founder; a leading trim waits for a live reading — a recording that shows a leading trim on 2.1.328 does not widen it here.

@history_item n=26 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `+ contract/send-readback/mcp` (a `/mcp` send reads back on a new local-command stdout tag in the session transcript, verified CLI only) and `~ contract/send-readback/unconfirmable` (`/remote-control`, `/cost`, `/usage`; `/mcp` unverified or no declared transcript); the stdout-tag transcript precedent for C3, `/context`'s half-unread post-condition to be read from a readable transcript beside the 2.1.328 recording.

@history_item n=27 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `+ contract/viola-verify-output/trust-record-names-nothing`: the trust-record row writes and removes a trust record in the Claude config, and no verify output names the file, key or value; binds the 2.1.328 recording given the bundle's changed trust dialog (decline text, `--add-dir` trust prompt).

@history_item n=28 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line` (verdict `absent` only on `browser-tools-prompt`, counting as a pass), `~ summary`, `~ trust-record-names-nothing`, `~ summary-line`; PROVISIONAL on the delegate's confirm. The step-line and summary wording a `verify --record` on 2.1.328 prints.

@history_item n=29 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `+ contract/viola-verify-output/browser-tools-line`: the fixed stdout line before the summary when a run met the browser-tools prompt; landed by the chunk that stamped the live home 24/24 on 2.1.328, this chunk's direct predecessor (C1).
