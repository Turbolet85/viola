@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=84 class=omission
leaf-text: - `gh run view --json jobs` / `gh run download` — read CI results and artifacts
source: fct-fumv4pch «- **The per-OS JUnit reader:** `scripts/junit-os-sets.py --run <id>` (python 3 standard library and the host's `gh`) is the operator pass's per-OS JUnit reading, never a CI step»
replace-with: - `gh run view --json jobs` / `gh run download` — read CI results and artifacts; `scripts/junit-os-sets.py --run <id>` (python 3 standard library and the host's `gh`) — the operator pass's per-OS JUnit reading, never a CI step: downloads `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest` into a temporary dir removed on exit, prints `{os}: {n} cases · {f} failed · {s} skipped` per OS, then `## {label}: {count}` and the sorted `classname::name` lines of `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`; exit 0, 1 (a report missing or unparsable, named) or 2 (usage); reads no environment variable, writes nothing in the tree
