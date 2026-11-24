@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=62
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=48
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=46
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=44
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: also `viola focus`'s refusals, each an `unable:` / `hint:` pair on stderr with no path, or under `--json` one refusal document)
source: ctr-tfxx4gxd «Internal error, and the start, revive-preflight, open, focus, up and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path»
replace-with: also `viola focus`'s and `viola up`'s refusals, each an `unable:` / `hint:` pair on stderr with no path, or under `--json` one refusal document (`viola up`'s `boot-time-unread`, `strict-modes-failed`, `pinned-hash-mismatch` or `terminal-not-found`); also a `viola up` whose lines are not all successes)
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: also `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board, no log opened;
source: ctr-kwqcbsw3 «contract/viola-up-exit-codes/21 · viola up exit codes · 21 — The board's strict refusal (instance-unreachable, strict-modes-failed), with no line printed.»
replace-with: also `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board, no log opened, and `viola up`'s same board refusal (`instance-unreachable`, `strict-modes-failed`), with no line printed;
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=40 class=stale
leaf-text: (`--list` conflicts with `--id`, `--fork`, `--model`, `--effort` and the trailing arguments)
source: ctr-zecfbixa «`--list` conflicts with `--id`, `--fork`, `--model`, `--effort`, the hidden `--origin` and the trailing arguments.»
replace-with: (`--list` conflicts with `--id`, `--fork`, `--model`, `--effort`, the hidden `--origin` and the trailing arguments; under the hidden `--origin <cli|agent>`, which takes no `--place`, the revive places its window on the workspace a real snapshot recorded)
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=40 class=omission
leaf-text: `viola focus <name> [--json]` takes one ViolaName and `--json` only;
source: ctr-uv2g64ih «contract/cli-verbs/up · CLI verbs · up — `viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; names given narrow»
replace-with: `viola focus <name> [--json]` takes one ViolaName and `--json` only; `viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through `viola revive` in a new terminal (names given narrow the fleet, a name not scanned reads `not-taken`; CLI-only, no MCP tool and no channel method);
