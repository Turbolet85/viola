@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — created the `viola revive` output structure: a start that passes its preflight prints nothing of its own while the child holds the terminal, and the `viola run` / `viola revive` passthrough sits in the 0.0 expression list. This is the source of contract/viola-revive/start-output, contract/viola-run/start-output and the passthrough part of decision/plain-output-modes. The trust write lands inside this shared start, so it must print nothing on the terminal. Its one closed word goes to the run log only.

@history_item n=2 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (revive's fifth pair) fixed the revive start's preflight order: the holder, the logged session, `session-live` (skipped under `--fork`, a failed read passes), then the recorded directory. This is the source of contract/viola-revive/preflight-order. The trust write comes after the version gate and before the spawn, so it has to sit after this preflight without reordering it or adding a preflight refusal.

@history_item n=3 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `viola revive` its `--json` refusal documents (exit 1, stderr empty) and left a post-preflight `run` start refusal with no document. It also filed `revive --list` at the catch site while a revive start stays outside it. The trust write's skip and refusal outcomes are not start refusals: no new `--json` code may appear and no document should print for them.

@history_item n=4 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree refusals of verify and the run and revive starts) added the home-tree check to the shared `run` / `revive` start, before the wrapper log opens: a refusal writes no log line and starts no child. It is a precedent for ordering a check inside the shared start and for when the run log exists. The trust write's closed word needs the run log to be open, so it must come after that point.

@history_item n=5 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `viola revive --json` post-preflight gap closed) set the start's post-preflight refusals as exit-1 documents, with `internal-error` for a program not found or an error of the start itself, and a `pump_child` error printing no document. This is the source of contract/run-revive-start-json/internal-error. Any error of the trust write must not surface as a new start error or as `internal-error`: the scope says a refused file form or a lost record writes nothing and leaves the dialog to the human.

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `cli` catch site's `internal-error` document under `--json`) made the catch site print `{"v":1,"error":"internal-error","detail":null}` under `--json` and kept a `revive` start outside the catch site's verb list. This is the source of contract/cli-catch-site/internal-error. The trust write lives in the start, not at the catch site, so neither the catch site's line nor its document changes.

@history_item n=7 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a stale holder's code under `viola revive --json`) split the holder refusal into `already-live` and `stale-heartbeat` in revive's preflight. It belongs to the preflight the trust write must follow untouched. Without `--json` the pairs stay byte-identical.

@history_item n=8 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory refusal of the `run` and `revive` starts) added a refusal to the shared start before any snapshot, child or bind, plus the `socket-dir-refused` code, and the check never changes a mode. It is a precedent for a check inside the shared start that leaves the file system as found. The trust write is the first step in that start that writes a file outside viola's home, which is the boundary widening the scope names.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the .cmd/.bat refusal leaves revive and verify) cut revive's start codes to `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused` and `internal-error`. This is the current closed set the trust write must not grow, because its outcomes are a log word and not a refusal.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows VT enable leaves the cli colour surface) set the plain-mode conditions to `--json`, a piped stdout, `NO_COLOR` or `TERM=dumb`. This is the source of decision/plain-output-modes. The trust write adds no output in any of these modes.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop on one OS) made `MM` `LedgerRow::relied()`'s length on every build, growing as chunks land rows. A stamp taken at an older count reads unverified to a newer binary, and that is the trap behind the scope's "on a verified CLI after the 23rd row" carry: `run` on a home stamped before the trust-record row writes no record.

@history_item n=12 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added the `viola verify --help` paragraph: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe. It is the standing user-facing wording that folder trust belongs to the human. Planning should check whether the product trust write at `run` / `revive` leaves that verify text true; verify is not in this chunk's scope.

@history_item n=13 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — kept the rule that verify never answers the CLI-native dialog, its own dialogs being answered by its hook and never by verify typing. It is a precedent for the scope's bound that viola types no key into the trust dialog: viola writes the record or leaves the dialog to the human.

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — tied a product behaviour to "on a verified CLI" (`/clear` and `/compact` confirmed only on a verified version, else unconfirmable). It is a precedent for gating a product action on the stamp, which the trust write repeats: write only on a verified CLI, else skip silently.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — restated the no-hint list: `unknown`, `wrapper fault` and `internal error`. This is the except clause of decision/hint-for-every-cause. If the trust write ever surfaced a refusal it would need its own hint; the scope keeps it a log word with no terminal line, so no hint is added.

@history_item n=16 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — set absence/web-surface (no page) and kept the rule of no interactive prompt and no pager in a verb. The trust write must not prompt: it either writes the record or leaves the CLI's own dialog to the human.

@history_item n=17 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master into rows, the source of every picked row (the cli-verbs, start-output, preflight-order, internal-error, plain-output-modes, hint and exit-12 rows), each checked against the code. A change to any of them goes through a row amendment (`+` / `~` / `−` handle) at wrap.
