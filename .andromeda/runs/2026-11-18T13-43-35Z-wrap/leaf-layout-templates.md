@leaf_group group=layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: the `viola run` / `viola revive` passthrough (nothing printed while the child runs;
source: ctr-mwmuxurd «Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs.»
replace-with: the `viola run` / `viola revive` passthrough (on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed, `revive`'s keyed by the recorded directory; the trust step adds no refusal, exit code, hint, `--json` code or terminal byte; nothing printed while the child runs;
