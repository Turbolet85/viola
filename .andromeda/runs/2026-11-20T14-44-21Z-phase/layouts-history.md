@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — landed `viola verify`'s output structure (one step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` last line, a failing-row run exit 1 that still writes the stamp, usage `viola verify [--record <DIR>] [-- <program> [args…]]`) and its fixed exit-1 refusal pairs; the automatic verify started by `viola run` either reuses this output whole or must say where it goes, and a RED automatic verify still writes a stamp under this rule, which bears on the chunk's "no loop on a failed version" record (contract/viola-verify-output/failing-row, /usage, /summary).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added verify's `--help` paragraph: run it from a folder you trust in Claude Code, an unapproved external CLAUDE.md import blocks the probe; an automatic verify launched from `viola run`'s session folder inherits that folder's trust and imports, a trap for the form chosen at P3.

@history_item n=3 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — gave verify its stopped form: Ctrl-C or TERM before the rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, stdout empty, no summary, no stamp, exit 1; an automatic verify interrupted inside `viola run` meets this pair and its hint naming the manual verb (contract/viola-verify-refusals/stopped, contract/viola-verify-output/stop).

@history_item n=4 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch made verify's interrupted pair his wording (planning-form mark gone) and ratified `--help`'s served-verbs-only table; any re-wording of the stop pair for an automatic run, or a new help row, moves founder-worded text.

@history_item n=5 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — set `MM` to the relied count (`LedgerRow::relied()`), one step line per relied row and exit 0 when every relied row passes; the automatic stamp must be the same relied-count stamp, not a subset (the live home's 24-row 2.1.328 stamp vs the tree's 27 relied rows is the scope's live example).

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS; the .cmd/.bat refusal leaves revive and verify) — `MM` is all relied rows on every build, the stop is `SIGINT` / `SIGTERM` unconditional, and verify's exit-1 pairs are the not-found, version, fixture, stop and home pairs; revive's `--json` start codes lost `batch-script-child`. These are the refusal pairs an automatic verify can surface (contract/viola-verify-refusals/*).

@history_item n=7 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `/summary`, `/exit-0`: the step word gained `absent` for the browser-tools-prompt row, and exit 0 became "every relied row counts"; what counts as a GREEN automatic verify (and so a stamped, verified start) is this rule, the `absent` reading marked PROVISIONAL by the delegate overseer at the time.

@history_item n=8 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run`, `~ contract/viola-run/start-output`, `~ contract/cli-verbs/revive`: on a verified CLI the run/revive start writes the claude trust record and the step adds no refusal, exit code, hint, `--json` code or terminal byte; the nearest precedent for a start step gated on the version being verified, and the constraint that `viola run` prints nothing of its own — an automatic verify that prints step lines, refusals or a summary before the session starts widens this row and must be said whole at P3.

@history_item n=9 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's output structure: a passed start prints nothing of its own while the child holds the terminal, each refusal is exit 1 with one `unable:` and one `hint:` last, the verb in `setup:    run, revive, verify, plugin install`; if revive's start shares the version gate, the automatic verify inherits these output rules (contract/viola-revive/start-output, /refusal-shape).

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — fixed revive's preflight order (holder, logged session, `session-live` via one `claude agents --json` read skipped under `--fork`, a failed read passing, then the recorded directory) and added the session-open-elsewhere pair; an automatic verify placed in revive's start must state where it falls in this order (contract/viola-revive/preflight-order, /session-live-check).

@history_item n=11 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `viola revive` `--json`: each refusal one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr, human pairs byte-identical; a new failure of the automatic verify reached through revive needs a closed code here or stays inside an existing one (contract/viola-revive-json/refusal-document).

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `viola revive --json` post-preflight gap closed; a stale holder's code; the `cli` catch site's `internal-error` document) — the start `revive` shares with `run` prints one exit-1 document per post-preflight refusal (`pinned-hash-mismatch`, `squatted-name`, `internal-error` for a program not found or a start error), a `pump_child` error prints none, and a `revive` start stays outside the catch site; `revive --json` is the machine road to `run`'s start causes, `run` having no `--json` — any new start refusal the automatic verify adds lands in this closed set (contract/run-revive-start-json/*).

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — verify checks its home tree before its log and maps a refused stamps write to the home pair; run/revive starts check the home tree before the wrapper log opens, no log, no child; an automatic stamp write from `viola run` meets both checks and their order (contract/viola-verify-refusals/home-not-private, contract/run-revive-start/home-tree-refused).

@history_item n=14 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the `run` and `revive` starts) — added the start's socket-directory refusal before any snapshot, child or bind, and `socket-dir-refused` to revive's `--json` start codes; fixes what runs before the child in the start, where an automatic verify would sit (contract/run-revive-start/socket-dir-refused, contract/run-revive-start-json/socket-dir-refused).

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ contract/cli-exit-codes/11` retired never reused, `~ contract/viola-send-output/unable` exit 13 (or 10/12/14); exit 12 `unverified-cli` is the refusal the automatic stamp clears, and no new refusal of this chunk may reuse 11 (contract/cli-exit-codes/11, /12).

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` and `/clear` read back by post-condition only on a verified CLI, `[  ] unconfirmable` on an unverified one; a version stamped automatically flips these outcomes for the running session, so when the session turns verified matters.

@history_item n=17 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/viola-send-output/unconfirmable`: `/mcp` joined the commands unconfirmable on an unverified CLI version; same verified-gate dependency as the chunk's "goes on verified".

@history_item n=18 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/viola-send-output/unconfirmable`: `/context` joined the unverified-CLI unconfirmable list; the current wording of the row this chunk's verified state changes the outcome of.

@history_item n=19 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the founder-confirmed census scoped "no interactive prompt and no pager" to the verbs (decision/no-prompt-or-pager, decision/flat-verbs); a consent question asked by `viola run` before the first automatic verify would breach it, so the P3 consent form must tell without asking, or use config or a flag.

@history_item n=20 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built `viola --help` as the grouped table over served verbs only, pinned by the `HELP` literal and `help_lists_only_served_verbs`, and the plain-output conditions; a separate verb or flag form (a P3 candidate) changes the help table or must stay out of it (contract/cli-verbs/help, contract/viola-help/verb-groups, decision/plain-output-modes).

@history_item n=21 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:  link, unlink` help row built) — the help is five rows, `setup:    run, revive, verify, plugin install`; the latest precedent of a served verb joining the table, its tests `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`.

@history_item n=22 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb in no help row with the `HELP` literal unchanged; the precedent for a launcher-called verb that adds no human surface (decision/no-banner names `viola mcp` as having none).

@history_item n=23 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows verified against the code, the cli surface's relied rows kept or cut by the founder; every picked row above takes its current wording from this conversion, and later changes are `+`/`~`/`−` entries on those handles.
