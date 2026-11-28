@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=19 class=omission
leaf-text: cli: no interactive prompt, pager or redraw, and input is stdin / `--file` only;
source: dec-3f4o5u3u «decision/cli-no-interactive-input · No interactive CLI input — The CLI has no interactive prompt, pager or redraw, and takes input only from stdin or `--file`. · except: the terminal fleet console (`viola console`), its own surface, which reads keys from stdin in raw mode and redraws on a resize»
replace-with: cli: no interactive prompt, pager or redraw, and input is stdin / `--file` only, except the terminal fleet console (`viola console`), its own surface, which reads keys from stdin in raw mode and redraws on a resize;
