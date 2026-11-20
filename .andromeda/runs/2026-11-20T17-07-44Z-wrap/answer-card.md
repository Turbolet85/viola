# Answer card — wrap of «A new CLI version stamped automatically» (Q1)

## The founder's own answers it rests on (the phase run's `founder-forks.md`, `inputs#I2` and `inputs#I3`)
- «A: verify перед сессией (Recommended)» — form (A); widenings (i)–(iv) over the rows §5 names as reached (inputs#I2 item 1).
- «Выключено, включаете один раз (Recommended)» — key `verify_new_cli`, default `false` (inputs#I2 item 2).
- «Как любой красный (Recommended)» — verify's stamping unchanged; a red entry is an attempt, cleared by a human's
  `viola verify`, a newer relied set or a newer CLI (inputs#I2 item 3).
- «1 старт + 1 запасной (Recommended)» — the live cap (inputs#I2 item 4; spent: ONE start, the spare unspent).
- P4-F1 «Печатать, строки поправить (Recommended)» · P4-F2 «Одно именованное исключение (Recommended)» · P4-F3
  «Исключение в обеих строках (Recommended)» — over the five rows they name (inputs#I3).
- The overseer's technical answers 5 (the lock's loser) and 6 (a stop ends the start) ride under form (A): a delegate's
  word, named as such.

## Where the built change departs from what he was shown (one line each, inputs#I5 item 4)
- The automatic verify runs AFTER the endpoint bind, not «between the version probe and the gate's verdict».
- Under the attempt lock a start takes another start's verdict only when the stamps entry changed after its own gate read it (narrowed from «read the stamps again»).
- The stop handler stands before the start line; a stopped start dies by its signal with no stamp and no session.
- A verify that cannot start writes nothing and is no attempt (L5 `spawn-failed`).

## The operations lying wholly inside those answers (23)
- ops-architecture.md · decision/on-disk-file-naming · conventions · except set: `ledger/verify.lock` by name (empty before) · fit: fits type 2
- ops-architecture.md · contract/instance-snapshot/cli-verified · standard-contracts · the final verdict · fit: fits type 3
- ops-architecture.md · decision/start-verifies-new-cli-version · established-decisions · add: form (A), key off by default, after the bind · fit: fits type 1
- ops-architecture.md · decision/no-automatic-reverify-of-red · established-decisions · add: a red entry never re-verified automatically · fit: fits type 1
- ops-architecture.md · name/other/ledger-verify-lock · occupied-resources · add: the named lock file · fit: fits type 5
- ops-architecture.md · contract/config-json/verify-new-cli · occupied-resources · add: the key, boolean, default false · fit: fits type 3
- ops-main-architecture.md · decision/run-start-order · established-decisions · the order as built, «only then» kept · fit: fits type 1
- ops-main-architecture.md · decision/automatic-verify-attempt-lock · occupied-resources · add: the lock, the loser, the narrowed re-read · fit: fits type 1
- ops-design-system.md · contract/auto-verify-line/verifying · surface-cli · add (L1) · fit: fits type 3
- ops-design-system.md · contract/auto-verify-line/did-not-verify · surface-cli · add (L2) · fit: fits type 3
- ops-design-system.md · contract/auto-verify-line/earlier-red · surface-cli · add (L3) · fit: fits type 3
- ops-design-system.md · contract/auto-verify-line/verifying-elsewhere · surface-cli · add (L4) · fit: fits type 3
- ops-design-system.md · contract/auto-verify-line/no-stamp · surface-cli · add (L5) · fit: fits type 3
- ops-design-system.md · contract/auto-verify-line/line-form · surface-cli · add (one write, ASCII, no SGR, none under --json) · fit: fits type 3
- ops-layout-templates.md · contract/viola-run/start-output · surface-cli · P4-F1 · fit: fits type 3
- ops-test-plan.md · decision/single-writer-state-files · 6-e2e-test-strategy · verify stays the one writer, a keyed start's included · fit: fits type 1
- ops-obs-plan.md · contract/config-json/known-keys · otel-sdk-init · four keys → five (§5's contract/config-json/* class read across masters) · fit: fits type 3
- ops-obs-plan.md · contract/diag-line-fields/process-start · log-coverage · the subject list gains auto-verify · fit: fits type 3
- ops-obs-plan.md · contract/diag-subject/auto-verify · log-coverage · add: the closed subject · fit: fits type 3
- ops-obs-plan.md · contract/run-process-log/auto-verify-stop · span-trace-coverage · add: the stop's exit pair (overseer's answer 6) · fit: fits type 3
- ops-a11y-plan.md · contract/tui-passthrough/human-keystrokes · 1-a11y-scope-summary · P4-F3 · fit: fits type 3
- ops-a11y-plan.md · contract/tui-passthrough/viola-terminal-bytes · 1-a11y-scope-summary · P4-F1 · fit: fits type 3
- ops-main-a11y-plan.md · decision/human-keystrokes-never-blocked · 2-a11y-strategy · except set: the start's verify window (empty before) · fit: fits type 2

## Every reworded row, old beside new (inputs#I5 item 5; no except shortened — both excepts were empty and gain one named member)
- architecture decision/run-start-order · rule
  - old: A start resolves the program, checks collision, pins the copy and plugin, gates the version, on a verified CLI writes the trust record of an untrusted spawn folder, binds the endpoint, writes the first snapshot, heartbeat and start events, and only then spawns the child.
  - new: A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's trust record, writes the first snapshot, heartbeat and start events, and only then spawns the child.
- architecture decision/on-disk-file-naming · except
  - old: (none)
  - new: ledger/verify.lock, the automatic verify's attempt lock, guards no file of its own name and stands apart from stamps.json.lock.
- architecture contract/instance-snapshot/cli-verified · promise
  - old: `cli_verified` comes from `run`'s version gate.
  - new: `cli_verified` is the start's final verdict: run's version gate, its stamps read again after the start's automatic verify where one ran.
- layout-templates contract/viola-run/start-output · promise
  - old: `viola run` prints nothing of its own, and the start's trust step adds no refusal, exit code, hint, `--json` code or terminal byte; the terminal belongs to the wrapped claude TUI until it exits.
  - new: Before the child, only a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`); the trust step adds no refusal, exit code, hint, `--json` code or terminal byte; then the terminal belongs to the wrapped claude TUI until it exits.
- test-plan decision/single-writer-state-files · rule
  - old: Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
  - new: Only the wrapper writes snapshot.json and only verify writes stamps.json, the verify a keyed start runs included; every other writer-capable entry point leaves both byte-identical.
- obs-plan contract/config-json/known-keys · promise
  - old: config.json has four known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s; any other key, …
  - new: config.json has five known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s, verify_new_cli; any other key, … (the rest unchanged)
- obs-plan contract/diag-line-fields/process-start · promise
  - old: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, hyprctl-probe), service_name, …
  - new: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, hyprctl-probe, auto-verify), service_name, … (the rest unchanged)
- a11y-plan contract/tui-passthrough/human-keystrokes · promise
  - old: Human keystrokes are never blocked, refused or delayed past the current atomic paste.
  - new: Human keystrokes are never blocked, refused or delayed past the current atomic paste once the wrapped child holds the terminal; the one exception is a start's automatic verify window, before the child starts.
- a11y-plan contract/tui-passthrough/viola-terminal-bytes · promise
  - old: viola writes zero bytes of its own to the terminal; only the child's output reaches it.
  - new: Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.
- a11y-plan decision/human-keystrokes-never-blocked · except
  - old: (none)
  - new: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.

## Beside the card (not asked)
- not_master (rejected by fit): architecture decision/stop-during-start-verify (type 4) · obs-plan decision/auto-verify-on-run-start-span (type 4) · security-plan decision/start-verify-child-isolated (type 4).
- Outside his answers — to `founder-forks.md` at the most conservative option, row unchanged: F1 architecture decision/viola-home-resolution · F2 security-plan decision/config-cannot-disable-controls · F3 test-plan decision/human-never-blocked · F4 test-plan decision/run-checks-before-child-spawn · F5 test-plan contract/fake-agent-cli/argv-only-options, decision/fake-agent-argv-only · F6 obs-plan contract/run-terminal-output/child-output · F7 a11y-plan own-bytes, zero-viola-bytes-on-the-terminal, wheel-ignores-terminal-events, portable-pty-outer-pty-driver, time-bounds-fall-on-the-driver, time-bounds-never-on-human.

## The answer
Asked by dialog (AskUserQuestion), answered by the overseer: «Confirm all 23 (Recommended)» — notes, verbatim: «Overseer's delegate confirm, read at 2026-11-20T17:46:36Z against answer-card.md, its ten old-beside-new rows and the founder's recorded answers: the 23 operations lie inside form (A) with its widenings, the key's default, the red rule and his three P4 answers; the four built departures are said on the card. F1-F7 stand unchanged for the epoch batch, with the rows that now read short of the built behaviour named there. PROVISIONAL until the founder's own word.»
Authority: delegate the overseer, 2026-11-20 — PROVISIONAL until the founder's own word at the epoch batch.
Correction after the answer: the question's text said «12 rows outside his answers»; F1–F7 above name 13 rows (F5 holds two). The list was whole on the card's file; the figure in the question was miscounted. Nothing the confirm covered changes.
