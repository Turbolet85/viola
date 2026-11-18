@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added `+ contract/viola-verify-output/trust-record-names-nothing`: verify's trust-record row writes and removes a trust record in the Claude config file, and no output of verify names that file, its key or its value. This is the nearest precedent. The product write at `run` / `revive` reuses that writer, so the same name-nothing floor carries over. A start may print nothing of the file, and the one closed word goes to the run log only.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — landed `revive`'s exit-1 refusals, one fixed-message line and one hint per cause, read in a fixed order that stops at the first, with none showing the recorded directory, a pid or a logged id. It also made `revive` print nothing once the child starts, and named a passed `viola revive` the same passthrough as `viola run` in the 0.0 expression row and the no-colour ban. The trust write sits in the start `run` and `revive` share, before the spawn. It must add no refusal pair and no line to that start: a refused file form leaves the dialog to the human, silently.

@history_item n=3 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive --json` its exit-1 documents `{"v":1,"error":"<code>","detail":null}` (was no `--json`), while `run` prints no document. The trust-write skips and refusals are no start refusal, so they must not add a code to `revive --json`'s closed list or a document to `run`.

@history_item n=4 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added `revive`'s `session-live` pair, read in the fixed cause order (after the logged session, before the recorded directory). This is the precedent for where a new step goes in the shared start's order: the trust write lands after the version gate and before the spawn, and adds no cause to that order.

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check; verify's home-tree and stamps refusals) — made `run`'s and `revive`'s start check the home tree before any log, with no child, printing the home-state or instance-state pair (`revive --json`: `strict-modes-failed`). It also made a refused `stamps.json` update print the home pair. This is the source of the picked rows `viola-run-start-refusals/home-tree-check` and `viola-revive-refusals/home-tree-check`. The trust write comes later in the start, after the version gate (`read_stamps_strict`), and must keep that check's order and its no-log rule.

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's own code; the pending arch block spent, the log's codes and the two carve-outs; the catch site's `internal-error` document; `revive --json`'s documents and its ten codes) — four changes bear on this chunk:
- It gave the stale holder `stale-heartbeat` and the live holder `already-live` as printed and logged codes.
- It listed obs-plan D-20's closed log detail codes, and kept `revive`'s strict-modes refusal with no log line.
- It made `run`, `revive`'s start, `mcp` and `hook` silent at the catch site, under `--json` too.
- It gave `revive --json` its closed codes and `internal-error`, and left a `pump_child` error with no document.

These are the sources of the picked rows `stale-heartbeat`, `already-live`, `strict-modes-no-log`, `cli-catch-site/silent-verbs` and `viola-revive-json-codes/internal-error`. The chunk's one closed word per start in the run log is a new member beside D-20's codes and must stay a closed word. A writer error must not reach the catch site as a printed line or document under `run`.

@history_item n=7 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive; `socket-dir-refused`, exit 21's sixth cause and a log code; `revive --json`'s eleventh code) — added the socket-directory pair to the shared start, refused before any snapshot, child or bind. It made `socket-dir-refused` a D-20 log code written at `run`'s and `revive`'s start, and added that code to `revive --json`. This is the source of the picked rows `viola-run-start-refusals/socket-dir` and `log-detail-codes/socket-dir-refused`. It is also the latest precedent for adding a step and a log word to the shared start, and the trust write's placement before the spawn must respect that step.

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — retired the OPEN mark on the socket-directory pair's wording, which the founder confirmed. The start's refusal wordings are settled, so this chunk must not reword them.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal and batch-script-child retired; verify's relied count on every build; the Windows VT enable retired from the colour rules; the server check without the macOS no-peer-pid carve-out) — four changes bear on this chunk:
- It cut `run`'s `.cmd`/`.bat` start refusal and `batch-script-child`, leaving `revive --json` with ten codes.
- It made `LedgerRow::relied()` the same on every build.
- It dropped the colour decision order's Windows rung.
- It made the server check Linux-only.

The start refusals and codes the chunk extends are this Linux-only set. "On a verified CLI" rests on the relied count, so a stamp taken at fewer relied rows reads unverified, and `run` is then transport-only and writes no record.

@history_item n=10 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — made verify's step counter one line per relied ledger row, `MM` being `LedgerRow::relied()`'s count. This is where "verified" is defined: a stamp covers the relied set. Since the trust-record row became relied, a home stamped before it reads `unverified-cli` (the picked rows `cli-verification/verified` and `unverified-cli`), and the trust write is skipped there.

@history_item n=11 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — gave `viola verify --help` one static ASCII paragraph naming no path, which covers a trusted folder and the external-import blocker. This is existing user-facing text about folder trust. If the product now writes the trust record at `run`, the paragraph may need a matching amendment, with no path in it.

@history_item n=12 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — made a behaviour (the `/compact` readback) hold only on a verified CLI, staying `unconfirmable` on an unverified one. This is the precedent for gating a product behaviour on the verified state: the trust write likewise happens only on a verified CLI, and an unverified CLI gets transport only.

@history_item n=13 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` share the same hint per reason, and `send`'s `unverified-cli` now prints its hint (the picked row `refusal-hints/unverified-cli`). The chunk's unverified skip is no refusal and must print no such hint at `run`.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines, no `--json` document; the `--json` failures the witness drives) — added the exception to the `--json` rule (an oversize `answer` and an unusable `send` exit 2 with no document). It also made every `--json` failure hold no ESC, path or upstream text. These are the source of the picked row `json-one-document-on-stdout`'s except clause and of the no-internals floor. Nothing of the config file or its errors may reach a document or a line.

@history_item n=15 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the server-verification cause and hint cover eight frame verbs) — made `server-verify-failed` a log code written by the eight frame verbs. This is the source of the picked row `log-detail-codes/server-verify-failed`, and a reference for how a log detail code's writers are listed when the chunk names its own closed word.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows VT enable retired from the colour rules) — the colour decision order lost its Windows rung. The picked row `cli-colour-decision-order/viola-run` (no colour, glyph or SGR under `viola run`, nothing while the child runs) now stands on the Linux-only order. The trust write must keep `run` silent.

@history_item n=17 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0, brand, surface and bans scoped to the CLI) — scoped "no interactive prompt, no TUI mode and no pager" and the spinner / progress-bar ban to a verb's output. This is the source of the picked rows `no-interactive-prompt-or-pager` and `no-spinners-in-verb-output`. The trust write asks the human nothing, has no prompt, and types no key.

@history_item n=18 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity: 3 rows) — `~ decision/fixed-phraseology-vocabulary` dropped `budget-paused` from the fixed words. The vocabulary of labels and refusals is closed and short. The chunk's run-log word should be such a fixed word, and it adds no new printed word.

@history_item n=19 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows (decision, contract, absence, name, overview), each checked against the code. Every picked row comes from this conversion. Any amendment this chunk makes takes the row form (`+` / `~` / `−` handle), as the trust-record ledger row's did.
