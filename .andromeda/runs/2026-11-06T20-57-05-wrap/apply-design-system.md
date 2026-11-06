@apply_row n=1 ref_file=fanout-design-system.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-design-system--cascade.md ref_n=1 v=refused
why: a hand removal · fct-bmfbeece
@fact id=fct-7u3hdiu2 expect_rev=1
old: **Git Bash on Windows** rewrites leading-slash arguments. Prompt text therefore comes
new: **Prompt text** comes
old: `--file`. The warning for a rewritten-path argument is one plain stderr line, `warning: argument looks like a Git Bash rewritten path`.
new: `--file`.
@pattern id=ds-01
fixed: looks like a Git Bash rewritten path
@pattern id=ds-02 ignore_case=true
fixed: Git Bash on Windows rewrites
