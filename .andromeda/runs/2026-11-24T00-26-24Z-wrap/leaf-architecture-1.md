@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/rul
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel frame
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `.c
@leaf_edit n=1 leaf=.claude/docs/services/viola-core.md line=20 class=omission
leaf-text: and, last, `launch` (`data` keys `model`, `effort`, both strings; `source: wrapper`, one line at every start after `lineage`, log-only);
source: ctr-zuxcjjbj «`window` is a normalised event kind, appended by the wrapper, log-only, that never wakes `wait`.»
replace-with: `launch` (`data` keys `model`, `effort`, both strings; `source: wrapper`, one line at every start after `lineage`, log-only) and `window` (`data` keys `address`, `workspace`, `placement`, `origin`; `source: wrapper`, one line after `launch` only when started under `--origin`, log-only, never waking `wait`);
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: and `verify` takes no `--json`)
source: ctr-jayaxyks «A refusal: strict-modes-failed before any log, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error»
replace-with: and `verify` takes no `--json`; also `viola open`'s refusals, `strict-modes-failed` before any log, `already-live`, `stale-heartbeat`, `pinned-hash-mismatch`, `terminal-not-found`, `not-live` or `internal-error`, each under `--json` the `{"v":1,"error":"<code>","detail":null}` document)
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=40 class=omission
leaf-text: `viola link|unlink <driver> <driven>`;
source: ctr-3g45airy «`viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude`.»
replace-with: `viola link|unlink <driver> <driven>`; `viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude`;
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=40 class=omission
leaf-text: and with none given each is the `config.json` default, else the compiled default.
source: ctr-ed4qz2og «viola run --origin <WORD> is a hidden flag naming the window's origin, an origin word; any other word is a usage error, exit 2.»
replace-with: and with none given each is the `config.json` default, else the compiled default; the hidden `viola run --origin <WORD>` names the window's origin, an origin word (any other word a usage error, exit 2), and the hidden `viola run --place <N>` the workspace the window is moved to (`--place` without `--origin` a usage error, exit 2).
