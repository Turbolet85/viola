# Proposal for Andromeda's `plan-template.md` — relay to overseer1

Operator ruling at P5 (2026-09-26): a project chunk never edits Andromeda; this proposal is recorded here for the
overseer to relay.

## Proposed wording (plan-template.md §Test Commands, the `leg` key's operator-pass paragraph)
> On a project whose harness exposes a pre-push gate, the operator pass lists it as an `operator` entry BEFORE the
> operator's pre-CI commit and the push entry. It runs on the tree about to be committed; a red there stops the pass
> before any commit, and each `fix(...)` commit takes the gate again before its push.

## Worked example (this chunk's plan, entries 15–16)
```toml
[[gate]]
run = 'bash scripts/agent-run.sh pre-push'
role = 'integration'
leg = 'operator'
timeout = 21600
expect = ['exit 0', 'contains "cmd":"pre-push","ok":true', 'contains "stage":"union"']
[[gate]]
run = 'git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0'
role = 'probe'
leg = 'operator'
```

## Why the order matters (measured at this chunk)
Before the pre-CI commit HEAD is the last wrap flip, and the derived mutation base stepped back to HEAD^ (acd08c7)
while CI derives the flip (a69c5ef); this chunk's base rule now keeps the flip while a promoted chunk is uncommitted,
so a gate run before the commit prints CI's base.
