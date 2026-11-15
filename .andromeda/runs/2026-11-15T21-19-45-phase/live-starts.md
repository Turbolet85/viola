# Live starts — chunk 2026-11-15-api-error-turn-end (cap 2, inputs#I2)

Each start is entered here with the clock's time (`date -u`) BEFORE it is fired.

## Start 1 — 2026-11-15T22:03:16Z (entered before firing)
- What: ONE `claude -p "viola probe: reply ok" --model viola-probe-no-such-model --plugin-dir <private plugin>
  --no-session-persistence`, claude 2.1.325, cwd a private 0700 dir under `target/live-stopfailure-20261115/`
  (gitignored), stdin closed, killed at 120 s, every `CLAUDE*` name removed from its environment.
- The capture plugin registers `StopFailure`, `Stop` and `SessionStart` to `/usr/bin/tee <capture>/<Event>.json`
  (exec form, no shell). Nothing of the founder's status line, config or credentials is read or changed by the
  probe; `~/.claude.json` and `~/.claude/settings.json` are compared by stat facts (mtime, size) only, before and
  after. No stamp moves. No ledger row.
- Reads: whether StopFailure fires for an unknown-model end on 2.1.325, its field names and its `error` word.
- Result: exit 1 after 1.82 s. NO hook fired, none of `StopFailure`, `Stop` or `SessionStart` (the capture dir
  empty). stdout: «There's an issue with the selected model (viola-probe-no-such-model). It may not exist or you may
  not have access to it. Run --model to pick a different model.» stderr: an `unrecognized_model` notice. The CLI
  ended the unknown-model start before its session hooks ran, so this forcing cannot reach StopFailure.
- Condition NOT held, reported: the CLI itself rewrote `~/.claude.json` (stat facts: mtime and size changed; the
  content was not read). `~/.claude/settings.json` was unchanged. The probe wrote nothing there; the CLI's own write
  at start is the cause. Stated on the P5 card.

## Start 2 — 2026-11-15T22:04:24Z (entered before firing; start 1 showed no StopFailure)
- The other forcing, which touches no credential and no file of the founder's: the same capture plugin and one
  `claude -p "viola probe: reply ok" --model haiku --plugin-dir <private plugin> --no-session-persistence`, claude
  2.1.325, in a fresh private 0700 root `target/live-stopfailure-20261115-s2/`, every inherited `CLAUDE*` name
  removed, and four names set on the child alone:
  - `CLAUDE_CONFIG_DIR` = a private 0700 dir inside that root, so the CLI's state file and credential lookup stay
    out of `~`;
  - `ANTHROPIC_API_KEY` = the literal `viola-scratch-not-a-key`;
  - `ANTHROPIC_BASE_URL` = `http://127.0.0.1:9`, a closed local port: the connection is refused and nothing is sent;
  - `CLAUDE_CODE_MAX_RETRIES` = `0`.
  `--model haiku` passes the model pre-check that ended start 1.
- Reads: whether a connection-class API error (the class of the measured connection-lost end) fires StopFailure on
  2.1.325, its field names and its `error` word.
- Result: exit 1 after 1.12 s. stdout: «API Error: Connection refused — a firewall or proxy may be blocking it
  (ECONNREFUSED)»; stderr empty. NO hook fired, none of `StopFailure`, `Stop` or `SessionStart` (the capture dir
  empty). The private config dir stayed empty. `~/.claude.json` and `~/.claude/settings.json` were unchanged (stat
  facts).

## Reading — the cap spent, the gap recorded
- Both forcings ended the print-mode start BEFORE its session hooks ran (no `SessionStart` either time), so neither
  reached the query loop where the static read places the StopFailure dispatch. The live payload of StopFailure on
  2.1.325 is NOT read. This is not evidence that StopFailure does not fire: an end inside a started session's turn
  (the measured connection-lost end) was never produced.
- The cap is 2, and both starts are spent. No third start is taken.
- The gap joins the named exception owned by «The 2.1.325 recorded fixture set» (inputs#I2): a StopFailure payload
  recorded live, inside a started session's turn.
- Left on disk for the operator's desk (gitignored, never committed, holding absolute paths in the plugin files and
  the CLI's printed lines): `target/live-stopfailure-20261115/` and `target/live-stopfailure-20261115-s2/`.
