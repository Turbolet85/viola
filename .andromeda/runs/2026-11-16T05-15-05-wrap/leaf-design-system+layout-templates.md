@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/la
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=7 class=omission
leaf-text: a send stays `open` until the matching `prompt-submitted` reads it back (on
source: ctr-qe3btyko «A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back: the submitted prompt equals the sent text or, provisionally, the sent text with its trailing run»
replace-with: a send stays `open` until the matching `prompt-submitted` reads it back (its prompt equal to the sent text or, provisionally, to it with a trailing run of ASCII space, TAB, CR or LF removed; any leading or inner difference leaves it unconfirmed) (on
