@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set the `viola verify` output structure: one `[NN/MM] <row id> …` step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` summary, a failing-row run exiting 1 with the stamp still written, the usage `viola verify [--record <DIR>] [-- <program> [args…]]`, and verify's fixed `unable:`/`hint:` pairs; `MM` grows as owning chunks land rows. This is the base of contract/viola-verify-output/* that the two new ledger rows (harness prefixes, R8 identity floor) extend: they add step lines, raise `MM`, and run under `--record`.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the wireframe went to ten rows; it added the static, path-free `--help` paragraph (run from a folder trusted in Claude Code; an unapproved external CLAUDE.md import blocks the probe), and it turned the dirty-recording refusal into `a recorded fixture is not clean: <file> <code>` (codes `home-path` · `absolute-path` · `username` · `email`, never the content). This one bears directly on the identity-floor probe ("env NAMES, never values"): the `--record` scrub and its closed code set are what a captured env name holding private text would meet (contract/viola-verify-refusals/fixture-not-clean, contract/viola-verify-output/help-paragraph). A new probe that needs a second session must keep the help paragraph static.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify output went to fourteen rows, with the four dialog rows added to the ledger order. Kept: verify never answers the CLI-native external-import dialog, and the probe's own dialogs are answered by its hook, not by verify typing. This is a precedent for a probe whose child needs input: a second session sending into a verify child must keep that line of no typing by verify into native dialogs.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the wireframe went to seventeen rows (long-paste-wrapper, tag-escaping, local-command-clear), and the examples name the one stamped version set. This is the precedent for landing ledger rows that change how a prompt's raw text is read (paste wrapper and tag un-escaping), the nearest kin to the harness-prefix row, which reads a prompt's raw start. The examples follow the stamped set.

@history_item n=5 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added verify's interrupted form: Ctrl-C or TERM before the rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, with stdout empty, no summary and no stamp, exit 1. The summary closes only a completed run. A longer verify, with a second session and maybe a model turn, must still honour this stop contract (contract/viola-verify-output/stop, contract/viola-verify-refusals/stopped) and clean up any second process.

@history_item n=6 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the wireframe went to eighteen rows. `MM` was a literal count; now it is `LedgerRow::ALL.len()`, so the count carries its rule. Each new row adds a step line in ledger order, and the stated count follows the rule rather than a hand-edited number.

@history_item n=7 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count, `LedgerRow::relied()`'s length (21 then), and exit 0 means every relied row passes. This is the precedent for deciding whether a new row is relied, and for the step-line and exit-0 rows that count only relied rows. The chunk's two rows are to be relied rows.

@history_item n=8 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the verify example went to 22 relied rows, with the failing example summing to the total and the Usage's row list ending at the new row. This is the pattern every row-landing chunk followed in the verify wireframe and usage list (dated 2026-10-16 in the history).

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (verify sub-change) verify went to 23 relied rows with `[23/23] agents-join …`, `stamped 2.1.287  23 pass  0 fail`, the failing example `21 pass  2 fail` and the Usage list ending `agents-join`. This is a row-landing precedent: a probe that reads a second `claude` surface (`claude agents --json`) beside the verify child.

@history_item n=10 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify went to twenty-four step lines (`local-command-compact`). `/compact` became read back on a verified CLI by its compacted-session post-condition, else unconfirmable. This shows that a row's verification gates send behaviour (contract/viola-send-output/read-back-compact, contract/viola-send-output/unconfirmable). Adding relied rows changes which stamped versions count as verified, and so which read-backs apply.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop) `MM` is now all relied rows on every build, with no per-OS relied set, "growing as owning chunks land their rows", and the stop is `SIGINT` / `SIGTERM` unconditionally. (.cmd/.bat refusal) verify's exit-1 pairs are now not-found, version, fixture, stop and home, with no `.cmd`/`.bat` pair. (per-OS wording) `viola <verb> --help` names the program by its executable. These set the current relied-count rule and the closed refusal set the new probes must fit (contract/viola-verify-refusals/*, contract/viola-verb-help/usage). The identity-floor row has no per-OS carve-out.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree refusals of verify and the run and revive starts) verify's strict-modes check refuses the home, `ledger/`, `bin/` or `plugin/` before its log opens, or refuses the stamps write, with the home pair, exit 1, stdout empty, no step line and no summary. A new probe's artefacts (a second session's state, captured env names, the stamp's relied-id set) live under that checked tree (contract/viola-verify-refusals/home-not-private).

@history_item n=13 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted pair wording, along with other Epoch 5 marks: its "planning form" mark is gone. The stop pair is fixed wording, and a chunk that lengthens verify must not reword it.

@history_item n=14 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (226 rows; "a master holds what a planner must respect and the code holds the rest"). Every picked row (contract/viola-verify-output/*, contract/viola-verify-refusals/*, the decisions) dates from this migration. That makes it the baseline for any row this chunk must amend, such as the step-line wording or the relied count.

@history_item n=15 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ contract/viola-verify-output/summary`, `~ contract/viola-verify-output/exit-0`: the verdict word gained `absent`, for the browser-tools-prompt row alone; `n pass` counts an absent row, and exit 0 means every relied row counts. This is PROVISIONAL (delegate overseer). It is the latest precedent of a ledger row changing the verify output rows. A new row that may fail to meet its condition in a run (a harness prompt never seen, an env name never captured) would need the same kind of amendment, because `absent` is reserved to that one row.

@history_item n=16 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/viola-run/start-output`: before the child, a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`). This is PROVISIONAL until the founder's word. The automatic stamp at the start runs every relied probe, so the new probes (a second session and maybe a model turn) run inside `viola run`'s start too, and their output and duration land on that passed-start surface. This is the chunk's immediate predecessor at `b3eb184c`.

@history_item n=17 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/viola-run/start-output`, `~ contract/cli-verbs/run`, `~ contract/cli-verbs/revive`: on a verified CLI the start writes the trust record and adds no refusal, exit code, hint, `--json` code or terminal byte. "Verified" rests on the stamp's relied-id set, which this chunk grows. The verify help paragraph's trusted-folder clause and what a probe leaves in the user's Claude config are traps the scope names.

@history_item n=18 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `/clear` is read back by its new-session post-condition on a verified CLI version (contract/viola-send-output/read-back-clear). This is one of the picked read-back rows gated on "a verified CLI version", which this chunk's added relied rows redefine.

@history_item n=19 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/viola-send-output/unconfirmable`, `+ contract/viola-send-output/read-back-mcp`: `/mcp` is read back by its transcript stdout tag on a verified CLI, and is unconfirmable on an unverified one. This picked row's verified/unverified split moves with the relied-row set.

@history_item n=20 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/viola-send-output/unconfirmable`, `+ contract/viola-send-output/read-back-context`: `/context` is read back by its stdout tag on a verified CLI, and is unconfirmable on an unverified one or before its transcript exists. This is the current wording of the two picked rows whose verified-version condition the new rows feed.

@history_item n=21 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses control-character and empty texts before any frame, and the wrapper makes the same two refusals for a frame sent straight to it (decision/wrapper-repeats-send-refusals). This is a constraint on the harness-prefix probe if its second session sends into the verify child through a viola endpoint or a frame.

@history_item n=22 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — exit 21 gained the server-verification cause with its own hint, last on stderr. This is the root of decision/hint-for-every-cause and decision/hint-last-on-stderr: any new refusal cause a probe introduces needs its own distinct hint naming the next verb, never `viola release`, and never quoting text or naming a path or a pid.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing, and `unknown`, `wrapper fault` and `internal error` take no hint. This is the exception clause of decision/hint-for-every-cause. A captured env name or value must never reach a refusal detail as free text.

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2) `~ contract/cli-exit-codes/11` (retired, never reused), `~ contract/cli-exit-codes/14`, and `~ contract/viola-send-output/unable`, which now exits 10/12/14. Exit 12 `unverified-cli` (contract/cli-exit-codes/12, picked) stands as the send refusal tied to the stamp's verified set, which the new relied rows change.

@history_item n=25 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola <verb> --help` stays clap's per-verb usage, and the grouped `--help` lists served verbs only. This is the source of contract/viola-verb-help/usage. A new hidden probe arm (like `hook --capture`, or a second-session helper) must stay off the help table, as `viola mcp` does.

@history_item n=26 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb that appears in no help row, leaving the `HELP` literal unchanged. This is the precedent for a hidden verb or arm with no human surface (decision/no-banner names `viola hook` and `viola mcp` as having none). It applies if the second session or the env capture rides a hidden arm.

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (Windows VT enable leaves the cli colour surface) output is uncoloured under `--json`, a pipe, `NO_COLOR` or `TERM=dumb`. Together with verify's plain-ASCII rule (contract/viola-verify-output/plain-ascii, decision/plain-output-modes), any new step-line words must be plain ASCII with no colour.
