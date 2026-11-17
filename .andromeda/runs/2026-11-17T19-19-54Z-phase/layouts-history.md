@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s output: one `[NN/MM]` step line per ledger row, the closing `stamped <ver>  <n> pass  <m> fail`, a failing-row run that exits 1 and still writes the stamp, and `MM` growing as owning chunks land rows. This chunk's 23rd relied row extends that same register (contract/viola-verify-output/step-line, /summary, /failing-row, /exit-0, /exit-1). It also set verify's fixed exit-1 refusal pairs, which the new probe's closed failure words must not add to.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — took the wireframe to `/10` and added the static `viola verify --help` paragraph: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe (contract/viola-verify-output/help-paragraph). It also renamed the fixture refusal to `a recorded fixture is not clean: <file> <code>` (contract/viola-verify-refusals/fixture-not-clean). Why it matters: this chunk's sixth run writes a trust record for its own temp dir, and the help paragraph's trust sentence must stay true and name no path. The probe's recorded fixtures must also pass the clean check.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — earlier precedent for a row-count bump (`/10` to `/14`, the failing example to `12 pass  2 fail`). Its Kept says the `--help` sentence "verify never answers it" is about the CLI-native dialog, and verify still never answers it. This chunk's trust-dialog probe must also send no key into any dialog: it only reads whether the dialog shows.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — a row-count bump to `/17`: every shown counter, the passing and failing summaries and the ledger-order list moved together. This is the precedent for this chunk's step-count sweep over every spelling of the count.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `/18`. `MM` stopped being a written number and became `LedgerRow::ALL.len()`, so the count carries its rule. The 23rd row moves `MM` through that rule (contract/viola-verify-output/step-line).

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count, `LedgerRow::relied()`'s length; there is one step line per relied row, and exit 0 means every relied row passes. This is the rule that makes every older stamp read unverified once a 23rd row is relied, and it is why the stale-stamp section is in this chunk's scope.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22nd relied row (`resume-session`): `[NN/22]`, `22 pass  0 fail`, failing example `20 pass  2 fail`. This is the nearest same-size precedent for adding a 23rd row and shows which wireframe parts and Usage list move.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify at 23 rows, with `agents-join` as `[23/23]`. Trap: the history already records a 23rd and a 24th row (`agents-join`, `local-command-compact`), but this scope says 22 relied rows today. Check the live ledger before naming the new row's index or the counts in its step line and summary.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify at 24 rows. It also says `/compact` and `/clear` are read back only on a verified CLI and are otherwise `[  ] unconfirmable` (contract/viola-send-output/read-back-compact, /unconfirmable). Why it matters: once the 23rd row is relied, every 22-row stamp reads unverified, which moves these send outcomes to `unconfirmable` on a stale stamp. The stale-stamp plan must name this.

@history_item n=10 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — introduced `/clear` read back by its new-session post-condition on a verified CLI version (contract/viola-send-output/read-back-clear). This depends on the verify stamp, so it goes stale when the relied count grows.

@history_item n=11 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/viola-send-output/unconfirmable`, `+ contract/viola-send-output/read-back-mcp`: `/mcp` is read back only on a verified CLI version with a readable transcript, and `/compact`, `/clear` and `/mcp` on an unverified version print `unconfirmable`. These are more stamp-gated outcomes that a stale 22-row stamp moves.

@history_item n=12 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's stopped form: Ctrl-C or TERM gives exit 1, stdout empty, no summary, no stamp, and the fixed `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` pair (contract/viola-verify-output/stop, contract/viola-verify-refusals/stopped). Why it matters: this chunk must remove its own trust record on every exit path, the stop included, without adding any line to this pair.

@history_item n=13 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted pair (and other marks) as worded. The stop pair's wording is therefore fixed: the record-removal-on-stop path must leave it byte-identical.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — verify's home pair: the home, `ledger/`, `bin/` or `plugin/` is refused before the log opens, or the stamps write is refused, with stdout empty, no step line and no summary. The run/revive start's home-tree refusal is a separate row (contract/viola-verify-refusals/home-not-private, contract/run-revive-start/home-tree-refused). Why it matters: the record writer is shared with the later `run` write and must sit after these checks, and the probe's config write must not be confused with a viola-home write.

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS; the .cmd/.bat refusal leaves revive and verify; the per-OS wording of help) — `MM` is the same on every build (Linux only), and the stop is `SIGINT`/`SIGTERM` unconditionally. Verify's exit-1 pairs are now not-found, version, fixture, stop and home. `viola <verb> --help` names the program by its executable (contract/viola-verb-help/usage). Why it matters: the 23rd row has no per-OS relied-set split, and the refusal set the probe must not grow is these five pairs.

@history_item n=16 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (relied: 45 kept, 12 cut), checked against the code. The picked verify rows are in their row form from this pass, which may explain the gap between the 24 rows in the history and the 22 relied rows in the scope. Read the code's `LedgerRow::relied()` rather than the history's counts.

@history_item n=17 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — set the `viola wait` / `viola last` lines, the timed-out line among them (contract/viola-wait-output/timed-out). A bounded wait in this chunk's tests and probes must keep that line's form and stay under the 20 s lint.
