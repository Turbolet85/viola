@history_item n=1 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Brand Identity: 4 rows) — changed `~ contract/cli-verification/verified` to "a stamp reading every relied row `pass` prints `verified`; a stamp lacking a relied row, or reading one `fail`, does not" and `~ contract/cli-verification/unverified-cli` to "no stamp reading every relied row `pass` runs transport only". This is the gate the chunk opens automatically. A stamp written before the relied set grew stays unverified until it is re-stamped. So an automatic verify has to write a whole relied-set stamp, and a RED stamp (one row `fail`) still leaves the version `unverified-cli`.

@history_item n=2 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added `+ contract/viola-verify-output/trust-record-names-nothing`. No step line, summary or refusal of `viola verify` may name the Claude config file, the trust record's key or its value. An automatic verify started by `viola run` writes and removes that trust record with no human present, and its output and any recorded failure must stay inside this promise.

@history_item n=3 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (3 rows, then 1 row) — changed `~ contract/viola-verify-output/step-line`, `~ contract/viola-verify-output/summary`, `~ contract/viola-verify-output/summary-line` and `~ contract/viola-verify-output/trust-record-names-nothing`. The verdict word `absent` is written only on the `browser-tools-prompt` row and counts as a pass there, and the summary's `n` counts it. An automatic verify's verdict, and "which rows failed" for a RED run, must read `absent` as a pass. The step-line confirm was the delegate's, PROVISIONAL.

@history_item n=4 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — added `+ contract/viola-verify-output/browser-tools-line`. When a run met the browser-tools prompt, a fixed stdout line prints before the summary: "viola never answers it: answer it once in claude, then run viola verify again". This conflicts with an unattended verify. If `viola run` runs verify itself, it must decide where this line goes (the child holds the terminal) and whether an `absent` or prompt-met run counts as done.

@history_item n=5 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — made verify's step counter count the relied ledger rows, `MM` = `LedgerRow::relied()` (was every ledger row), and made the summary's n + m equal `MM`. "Relied" is what a stamp must cover for `verified`, and the automatic verify inherits this count rule.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build; the .cmd/.bat refusal retired; the colour decision order's rung 7 retired; home under a regular file without an OS arm) — `relied()` is now 24 on every build, and `shim-resolution` passes when `claude --version` answers. `revive --json`'s closed codes went back to ten (`batch-script-child` removed), and verify lost its `.cmd`/`.bat` refusal. These set the code lists and the colour order that a shared run/revive version gate must keep.

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set the base shape of `verify`'s output: a static stdout step line for each row, the `stamped <ver>  n pass  m fail` summary, its exit-1 `unable:` refusals each with its own hint (CLI not found, version unreadable), and a failing row printing no stderr word. If the automatic form reuses verify, it gets these outputs and refusals, and they are what tell the user which rows failed.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added `viola verify --help`'s static paragraph: run it from a folder you trust, and an unapproved external CLAUDE.md import blocks the probe. It also named the unclean-fixture refusal. An unattended verify from `viola run`'s cwd meets the same trust and import preconditions, so these are failure modes of the automatic form.

@history_item n=9 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, and exits 1 (`contract/viola-verify-refusals/interrupted`). An automatic verify that is interrupted or killed while `viola run` waits must decide whether that counts as a failed attempt.

@history_item n=10 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the interrupted pair's wording, served-verbs-only `--help`, and the width source. A new verb or flag for a launcher (one of the P3 candidates) falls under the served-verbs-only help rule.

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals; run's and revive's start refusals at the home-tree check; a home under a regular file; the six frame verbs' strict-modes pair) — before its log, `verify` checks the home, `ledger/`, `bin/` and `plugin/`. A `stamps.json` update refused by its strict-modes check prints the home-state pair and exits 1 (`contract/viola-verify-refusals/stamps-update`, `contract/viola-verify-output/stamps-refused`). `run`'s and `revive`'s start make the same home-tree check before any log. The stamp writer `update_stamps` and its strict-modes check are what an automatic verify writes through. A refused update in the live home has no human to read the pair.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict-modes readings without an OS arm) — a home under a regular file is a strict-modes refusal on every OS: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21). This explains `contract/strict-modes/home-under-regular-file`, which applies to a verify started from `run`.

@history_item n=13 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — gave `revive`'s exit-1 refusals in a fixed order that stops at the first, with run's already-live and stale pairs unchanged. `revive` prints nothing once the child starts, and a passed `revive` is the same passthrough as `viola run` (0.0, no colour). The stderr fixed-form list became run, revive and verify. If the version gate is shared, any automatic-verify step at revive's start has to fit into this order and its silence.

@history_item n=14 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added revive's `session-live` pair (`contract/viola-revive-refusals/session-live`, read after the logged session, skipped under `--fork`, a failed read passes). The verify count went to 23. This is precedent for a start-time read placed inside revive's fixed cause order.

@history_item n=15 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive --json` its first exit-1 stdout documents `{"v":1,"error":"<code>","detail":null}`, with no hint and stderr empty. This is the machine form any new start refusal or failure of a shared gate must follow.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json`'s documents and its ten codes) — every refusal of the start that `revive` shares with `run` prints one exit-1 document from a closed code list. `run` itself prints none, and a `pump_child` error prints none (`decision/no-json-after-child-held-terminal`). A new start outcome (verify failed or ran) would need a new closed code, and this entry shows what a widening costs: the founder's own word.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — under `--json` the catch site prints `{"v":1,"error":"internal-error","detail":null}`, counts `--json` only before `--`, and `run`, `revive`'s start, `mcp` and `hook` stay silent (`contract/cli-catch-site/*`, `decision/silent-catch-sites`). An anyhow error from an automatic verify inside `run`'s start falls under this silence.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's own code; the pending arch block spent, the log's codes) — gave `stale-heartbeat` and `already-live` their own printed and logged codes (`contract/viola-run-start-refusals/*`). It closed the obs-plan D-20 log detail code list and noted that revive's strict-modes refusal writes no log line (`contract/viola-revive-refusals/strict-modes-no-log`). This is the register a recorded failed attempt or an automatic-verify log code would extend.

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive; `socket-dir-refused`; `revive --json`'s eleventh code; a refused socket directory at the eight frame verbs) — added run's start refusal `the socket directory for <name> cannot be trusted` (before any snapshot, child or bind) with its hint, revive's shared pair, and the `socket-dir-refused` revive code and log code (`contract/viola-run-start-refusals/socket-dir`, `contract/log-detail-codes/socket-dir-refused`, `contract/refusal-hints/socket-dir`). This is the latest precedent for a step added to the shared start before the child. The wording was first PROVISIONAL.

@history_item n=20 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — the founder confirmed the socket-directory pair's wording and its OPEN mark was retired. This shows how a planning-form start refusal gets settled at a boundary batch.

@history_item n=21 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 20's fixed `wrapper fault`; `unknown`'s detail; exit 2's fixed stderr lines; the `--json` failures the witness drives) — explains `decision/no-hint-for-faults`, the exceptions to `decision/json-one-document-on-stdout`, and `decision/json-failure-documents-clean` (guard `tests/cli_error_bodies.rs`). An automatic verify's failure output must hold no path, ESC or upstream text and print one document under `--json`.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause) — the Streams rule changed so that the typed `refusal` / `error` fields and their `detail` are the whole answer under `--json`, and exit 21's cause is its `detail`. This explains `decision/no-hint-under-json`, which also binds the `unverified-cli` (exit 12) refusal object.

@history_item n=23 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — changed `~ contract/refusal-hints/refusal-line`: `send` and `answer` now print the same hint for the same reason, and send's `unverified-cli` gained its hint (`contract/refusal-hints/unverified-cli` "run viola verify for this CLI version"). This chunk changes what that hint means, because a version may now be stamped without a human running verify.

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity; Iconography; Surface: cli 4 and 10 rows) — changed `~ decision/fixed-phraseology-vocabulary` (`budget-paused` dropped), `~ contract/cli-separators/space-hyphen-space` (example now `skipped 2 - 0 - 1`), `~ contract/cli-exit-codes/10-14` and `~ contract/cli-exit-codes/11` (11 retired, never reused). Any new word or exit code for an automatic verify must come from the fixed vocabulary, and exit 11 cannot be reused.

@history_item n=25 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the census scoped the "no interactive prompt, no TUI mode and no pager" rule and the spinner / progress-bar / live-redrawing ban to a verb's output (`decision/no-interactive-prompt-or-pager`, `decision/no-spinners-in-verb-output`). An automatic verify's consent or notice "before the first one" cannot be an interactive prompt or show progress, and verify's step counter stays the one progress pattern.

@history_item n=26 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer and the decision order, with `--json`, then `viola run`, then `NO_COLOR` as rungs. `--help` lists only the served verbs, grouped. This explains `contract/cli-colour-decision-order/*` and `contract/help-output/verb-groups`. Nothing may print while `viola run`'s child runs, and a new verb or flag must sit in the help groups.

@history_item n=27 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — made `mcp` a hidden verb (`#[command(hide = true)]`) that the plugin runs and a person never does, so it is in no `--help` row. This is the precedent for the "separate verb a launcher calls" candidate form.

@history_item n=28 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:` help row built; the fixed instance-state pair for link/unlink) — completed the five help groups `board / traffic / wheel / handoff / setup: run, revive, verify, plugin install` and extended the fixed `unable: <text>` form list (`contract/viola-help/verb-groups`, `contract/cli-stderr/fixed-message-refusal`). A new setup verb or a fixed-form refusal would extend these closed lists.

@history_item n=29 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board shows the CLI cell as `unknown` or the version, and `list`'s exit-21 refusal joined the fixed-form list. This explains `decision/list-is-the-board` and `verify` acting on no strip. Any verified/unverified display after an automatic stamp reads through the board.

@history_item n=30 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without its `color` feature, so all styled bytes are viola's own (`decision/clap-without-colour-feature`). A new flag or verb for the automatic form inherits this.

@history_item n=31 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `--json` output stays serde-escaped (`contract/cli-message-escaping/json`). Any new automatic-verify `--json` field follows it.

@history_item n=32 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — a `/compact` send is read back by its post-condition only on a verified CLI, and stays `unconfirmable` on an unverified one. The verify count went to 24. This shows what turning verified unlocks in the readback, and that a session may change from unverified to verified only at an explicit gate.

@history_item n=33 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — changed `~ contract/send-readback/unconfirmable` and added `+ contract/send-readback/mcp`: `/mcp` is confirmable only on a verified CLI. This adds to what "goes on verified" changes for a session's sends, and matters for the candidate form where the session turns verified when the stamp lands.

@history_item n=34 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (the never-built page's design tokens retired) — every web or graphical surface left, which explains the absences (`absence/rendered-page`, `absence/web-page-surface`). An automatic verify's notice has only the CLI's stderr and stdout.

@history_item n=35 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each checked against the code, with the founder's fates. Every row this chunk changes is amended as a `+` / `~` / `−` row with its why, as the later entries show.
