@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision · contract · absence · name), each verified against the code; every row this chunk is bound by, and any it moves in its P3 block, is a row of that conversion, amended since only by `+` / `~` / `−` entries naming a handle.

@history_item n=2 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the human width source became stdout's own size by `viola_pty::host_size()` (`TIOCGWINSZ`), `COLUMNS` never read, a piped stdout no width; the resize path this chunk reads lives in the same `crates/viola-pty` seam, and any change to how sizes are read or forwarded must keep rows decision/columns-never-read and contract/human-tables/width-source true.

@history_item n=3 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the `host_size()` width source (was the overseer's PROVISIONAL answer); the width rule this chunk's resize path sits beside is founder-ruled, not open.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli width read by TIOCGWINSZ alone; the cli Platform narrowed to Linux terminals) — `host_size()` lost its Windows arm and the platform is Linux terminals with CI on `ubuntu-latest` (row decision/linux-terminal-platform); the claim «real `claude` on Linux loses a key after a resize» is measured on the one platform viola targets, with no other-OS arm to price.

@history_item n=5 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive` prints nothing once the child starts and a passed `revive` is the same 0.0 passthrough as `viola run`; any fix on the resize path (a settle wait, a re-type) must print nothing while the child holds the terminal (rows decision/run-prints-nothing-while-wrapped, contract/viola-run-output/child-started, contract/viola-revive-output/after-child-start).

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json`'s documents; the catch site's `internal-error` document) — a `pump_child` error prints no document since the child has held the terminal, and `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site; a fix or failure inside the pump's resize handling falls under this silence (rows decision/no-json-after-child-held-terminal, decision/silent-catch-sites, contract/cli-catch-site/silent-verbs).

@history_item n=7 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the spinner / progress-bar / live-redraw ban reads «in a verb's output», and the readback box's one place is the CLI mirror; a settle wait added after a resize gets no progress indicator and no new box state (rows decision/no-spinners-in-verb-output, decision/readback-box).

@history_item n=8 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the readback box's one place is the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send` and the cocked strip shows `DIALOG` plus kind in amber on the CLI; a read-back-and-re-type fix must keep the box's three states and words as they are (rows decision/readback-box, decision/dialog-pending-cocked-strip, decision/readback-box-signature-check).

@history_item n=9 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on a measured post-condition too (`/clear`'s new session), no new word added; precedent that a new confirmer of a send lands without a new printed word, as a re-typed key after a resize would have to.

@history_item n=10 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — contract/send-readback/prompt's «matching» became the sent text or its trailing ASCII-whitespace-trimmed form, a leading trim not matching; a key lost after a resize yields a submitted prompt that does not match, so the send stays open / reads `unable` under this row, and a re-type option must still meet exactly this match.

@history_item n=11 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the send trailing-only tolerance ratified) — the founder ratified trailing-only matching, a leading trim waiting «for the live reading carried on Standing live rig with a screen model»; precedent that a CLI input behaviour is widened only after a live measurement, as this chunk's «measured, fixed if it does» requires.

@history_item n=12 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `not-delivered · input-not-ready` hint became `<name> was not ready for input; send again, and if it repeats a human must look at the session` (founder's wording); a refusal option for a lost key would reuse or sit beside this hint (row contract/refusal-hints/not-delivered-input-not-ready).

@history_item n=13 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `empty-text` joined exit 13's `not-delivered` detail list with its own founder-worded hint; the precedent for adding a new `not-delivered` detail and hint should the refusal option be chosen (row contract/cli-exit-codes/13).

@history_item n=14 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — contract/refusal-hints/refusal-line: `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint; a new refusal detail must be keyed for both verbs.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the code word and the screen on a peer's reply; the `--json` failures the witness drives; exit 2's fixed stderr lines) — a peer's `refusal` / `detail` must be a code word or it reads `unknown` / no detail, every `--json` failure is one clean document, and exit 2's two paths print no document; any new refusal detail from the wrapper must be a code word (rows contract/peer-reply-screening/code-word, decision/json-failure-documents-clean, decision/json-one-document-on-stdout).

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `budget-paused` left the fixed vocabulary (decision/fixed-phraseology-vocabulary), exit 11 is retired and never reused (contract/cli-exit-codes/10-14), the no-hint-names-release reason now names only `human-typing`, and WHEEL 23 kept as width alone (decision/wheel-column-width-23); a new refusal word or exit must not reuse 11 and must enter the fixed vocabulary.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the eight frame verbs) — added the `socket-dir` refusal before any frame (row contract/frame-verb-refusals/socket-dir); a send-side fix must keep the frame verbs' refusal order before any frame.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' strict-modes refusal) — `send` runs its strict-modes check after reading its text, before any frame, printing the instance-state pair not the mirror (row contract/frame-verb-refusals/strict-modes); a re-type or read-back path inside `send` comes after this gate.

@history_item n=19 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `verify` prints one step line per relied ledger row, `MM` = `LedgerRow::relied()`'s count, the summary's n + m = `MM`; if the answer becomes a relied row (`crates/viola-agent-claude`), the step counter and summary grow by rule, no hand-edited count (rows contract/viola-verify-output/step-line, contract/viola-verify-output/summary).

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — `relied()` is the same set on every build; a new resize row would be relied with no OS qualifier.

@history_item n=21 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — the step line gained the verdict word `absent`, written only on `browser-tools-prompt` and counting as a pass there; precedent and constraint for a resize row whose live reading may be mixed: a new verdict word is a row change of contract/viola-verify-output/step-line and summary, not free.

@history_item n=22 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — contract/cli-verification/verified now needs a stamp reading every relied row `pass` and unverified-cli covers a stamp predating the relied set; adding a relied row turns every existing stamp (2.1.328's included) `unverified-cli` until re-stamped, and moves `/compact`, `/mcp`, `/context` sends to `unconfirmable` meanwhile.

@history_item n=23 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — added the five automatic-verify start lines (PROVISIONAL, delegate's); a relied-row addition reaches starts through these lines (earlier-red, did-not-verify name the parsed version), the picked rows contract/auto-verify-line/did-not-verify and earlier-red.

@history_item n=24 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — unwrapped sessions are read-only bare strips on the board, never a `target`, and `verify` grew a row by the same rule; only wrapped instances (whose PTY viola resizes) are in this chunk's reach (row decision/bare-strips-never-target).

@history_item n=25 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — decision/api-error-turn-end-shown-as-turn-end: an API-error turn end shows as any turn end, its trace one closed cause word of nine in `--json` and MCP `wait`, ratified 2026-11-17; a lost-key outcome may not be surfaced through the turn-end cause or a distinct status.
