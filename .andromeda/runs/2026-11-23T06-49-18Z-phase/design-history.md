@history_item n=1 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap 4.6.7 (derive) was built without its `color` feature, so help and usage stay plain and every styled byte is viola's own SGR module's (decision/clap-without-colour-feature). Any model/effort argument this chunk adds to `viola run` / `viola revive` lands on that clap surface, and its help and usage text must stay plain.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — landed `revive`'s exit-1 refusals in a fixed order that stops at the first (one fixed-message line and one hint per cause, no directory, pid or logged id shown, no hint naming `viola release`). It also set that `revive` prints nothing once the child starts, defined the `--list` rows and put `revive` in the `setup:` help group. The pair replay on resume goes through this verb, so any new refusal or line has to fit this order and this silence (contract/viola-revive-refusals/order, contract/viola-revive-output/after-child-start, decision/start-refusals-show-no-path-or-pid).

@history_item n=3 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — `viola revive`'s strict-modes refusal carries no detail code (it was "reuses `strict-modes-failed` at exit 1"). This is the precedent behind contract/viola-revive-refusals/strict-modes-no-log, which a start that now reads a recorded pair must keep.

@history_item n=4 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added `revive`'s `session-live` pair, read after the logged session and before the recorded directory (skipped under `--fork`), and built the unwrapped rack (NAME `sessionId`, `n/a` in WHEEL/DIALOG/CLI). The revive order a pair replay must slot into, and the fixed shape of the board if the pair were ever shown (contract/viola-revive-refusals/session-live, contract/viola-list-board/unwrapped-row).

@history_item n=5 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive --json` its one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` with no hint and stderr empty, and `--list --json`. Any failure mode this chunk adds to a start (e.g. an unknown recorded pair) meets this closed document form (contract/viola-revive-output/json-refusal).

@history_item n=6 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the board's colour layer and width cut: NAME column width, unwrapped-only NAME truncation, the width from stdout's own TTY (no `COLUMNS`), and `--help` listing served verbs only. These constrain any display of the pair on `viola list` (the fork), and how new `run`/`revive` flags show in help (contract/viola-list-board/name-column-width, contract/human-tables/width-source).

@history_item n=7 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: the row-mode escaping of every control character, the empty board's BAY line and caption, WHEEL padded to 23, and `list`'s whole-board exit-21 refusal in the fixed pair. This is what a list-side display of the pair would have to respect (contract/viola-list-board/row-escaping, decision/list-refuses-board-on-strict-modes-failure).

@history_item n=8 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `verify`'s step counter became one line per *relied* ledger row, `MM` = `LedgerRow::relied()`'s count, the summary's n + m equal to `MM`. If the model/effort CLI channel is undocumented behaviour it becomes a ledger row (scope), and that row's relied status moves `MM` and the summary (contract/viola-verify-output/step-line, contract/viola-verify-output/summary-line).

@history_item n=9 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added the `resume-session` ledger row (`--resume <id>` reopens the session under that id), relied on every OS. This is the resume behaviour this chunk's replay of the recorded pair rides on, and the precedent for a probe-backed row on a `claude` launch flag.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build; the .cmd/.bat refusal and batch-script-child retired) — `relied()` reads all rows on every build (24 then). `revive --json`'s closed codes went back to ten without `batch-script-child`, and `run`'s .cmd/.bat refusal left. A new ledger row is relied on every build, and the revive code list this chunk might touch is the ten-code set (contract/revive-json-refusal-code/*).

@history_item n=11 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added the verify row that writes and removes a trust record in the Claude config file, promising that no output names that file, key or value. This is the precedent for a probe touching `claude` settings. The chunk writes nothing to the user's settings (v2-31 keeps them byte-identical), and a model/effort probe row would follow this names-nothing rule.

@history_item n=12 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line gained the verdict word `absent` (only on `browser-tools-prompt`) and the summary's `n` counts it. These are the current forms of the step-line/summary rows a new ledger row's line must match. Ratified by 2026-11-22T17-29-41Z-wrap.

@history_item n=13 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — changed contract/cli-verification/verified and contract/cli-verification/unverified-cli: a stamp must read *every* relied row `pass`, so a version stamped before the relied set grew reads `unverified-cli` until it is re-stamped. Trap: adding a relied model/effort ledger row turns every existing stamp unverified (transport only) until the next verify.

@history_item n=14 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — added the five automatic-verify start lines of `viola run` / `viola revive` plus their line form: one stderr write, ASCII, no SGR, holding only the parsed version and a closed word, none under `--json`. These are the only lines a start may print before the child. A model/effort line at launch would break decision/run-prints-nothing-while-wrapped unless it fits here, and a new relied row triggers these lines on the next start.

@history_item n=15 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (the delegate's confirms ratified by the founder for 2026-11-18-browser-tools-prompt-modal, 2026-11-20-a-new-cli-version-stamped-automatically and 2026-11-22-paste-write-bounded) — the auto-verify-line, step-line, summary and exit-13 rows this chunk rests on stand on the founder's own word. No PROVISIONAL mark is left on them.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check; verify's home-tree and stamps refusals; the six frame verbs' strict-modes refusal) — `run`/`revive` check the home tree before any log and with no child (home-state / instance-state pairs, exit 1, revive `--json` `strict-modes-failed`). A `stamps.json` update refused by its strict-modes check prints the home pair. A per-instance pair record or a `config.json` default read at start sits behind this check (contract/viola-run-start-refusals/home-tree-check, contract/viola-verify-refusals/stamps-update).

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's own code; the pending arch block spent; the catch site's `internal-error` document; `revive --json`'s documents and its codes) — gave the stale holder its own code, `stale-heartbeat`. It set the closed log detail codes, with `revive`'s strict-modes refusal writing no log line. `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, with `--json` counted only before `--`. It gave `revive --json` one document for every refusal of the start it shares with `run`, and no document after `pump_child` (the child held the terminal). These are the closed sets and silences a launch-time pair change must not widen without a ruling.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive; `socket-dir-refused`; `revive --json`'s eleventh code) — added the socket-directory refusal to the shared start, before any snapshot, child or bind, plus the `socket-dir-refused` log code and revive `--json` code. These are the latest extension of the start's refusal set and closed codes that a pair-at-launch start passes through. The same chunk also made the board hold the home to the read rule and `instances/` to the write rule, refusing the whole board with the home pair. That applies if the board reads a per-instance pair record.

@history_item n=19 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — retired the OPEN on the socket-directory pair's wording, so the start-refusal literals in contract/viola-run-start-refusals/socket-dir stand on the founder's word.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines, no `--json` document; the `--json` failures the witness drives; the code word and the screen; `unknown`'s detail) — exit 2 is clap usage or a fixed stderr line with no document whatever `--json` says. Every `--json` failure is one document with stderr empty, no ESC, path or upstream text (guard `tests/cli_error_bodies.rs`). A code word is 1 to 64 bytes `[a-z0-9-]` starting with a letter (`viola_core::is_code_word`). Relevant to a bad model/effort argument (a clap exit 2) and to recording the pair as a closed set of values.

@history_item n=21 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — fixed verify's interrupted `unable:` / `hint:` pair at exit 1, with no step line and no summary (contract/viola-verify-refusals/interrupted). An automatic verify at start that runs a new ledger row inherits it.

@history_item n=22 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — `viola verify --help` gained its static ASCII paragraph naming no path, and the unclean-fixture refusal became `a recorded fixture is not clean: <file> <code>` with closed codes, never the content (contract/viola-verify-refusals/unclean-fixture). A recorded fixture for a model/effort probe row must pass that screen.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity; Surface: cli, 4 and 10 rows) — removed `budget-paused` from decision/fixed-phraseology-vocabulary, retired exit 11 for good (contract/cli-exit-codes/10-14, contract/cli-exit-codes/11) and narrowed decision/no-hint-names-viola-release's why to the human-typing refusal. It is the latest edit of those picked rows and shows that a closed word or exit code, once retired, is never reused.

@history_item n=24 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — contract/refusal-hints/refusal-line now says that `send` and `answer` share hints for the same reason or detail, and that an opaque or absent detail prints no hint. Any new refusal this chunk adds is keyed under this rule.

@history_item n=25 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0: brand, surface and bans scoped to the CLI; the page's design tokens retired) — viola has no web or graphical surface. The design direction is the CLI's, and the terminal fleet console inherits it, so "no interactive prompt, no TUI mode, no pager" is scoped to a verb and the spinner ban to a verb's output. A model/effort choice at launch therefore has no interactive picker (decision/no-interactive-prompt-or-pager).

@history_item n=26 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Iconography 2 rows; Surface: cli name-column-width; row-order) — the chunk this one rests on: the pair is recorded beside its lineage record. It kept `viola list` at six fixed columns, captions and slot order (no link column, tree indent inside NAME, links in the `-- LINKS --` block) and made NAME's width include the tree indent. Showing the pair on the board is a fork that cannot add a column (decision/list-six-fixed-columns, contract/viola-list/columns, contract/viola-list-board/name-column-width).

@history_item n=27 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal takes the fixed instance-state pair) — extended the closed list of fixed `unable: <text>` users (contract/cli-refusal-line/fixed-unable-form, contract/cli-stderr/fixed-message-refusal). Any new fixed-form refusal this chunk adds must be named in those lists.

@history_item n=28 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — spelled out message-mode escaping (every control character but `\n`/`\t` as `\xHH`, `--json` serde-escaped). Those are the escaping rules (contract/cli-message-escaping/*) for any value an operator gives, a model name included, that reaches human output.
