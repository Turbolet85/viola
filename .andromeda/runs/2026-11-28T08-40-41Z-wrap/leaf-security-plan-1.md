@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=17 class=omission
leaf-text: `viola up` through the strict board scan before its log opens
source: dec-663jcbro «viola console refuses unless stdin and stdout are both terminals, then holds the home to the strict-modes read rule, an absent home passing, before any log opens; either refusal writes no line, leaves the modes as found and exits 1.»
replace-with: `viola console`, after refusing unless stdin and stdout are both terminals, through the strict-modes read rule on the home (an absent home passing) before any log opens (a refusal exit 1, no line, the modes as found); `viola up` through the strict board scan before its log opens
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=16 class=omission
leaf-text: client- and wrapper-side.
source: dec-dsjweani «viola console reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each read decoded to one closed class: 0x03 anywhere is interrupt, exactly q is quit, anything else is unbound; no key byte reaches any log line or output.»
replace-with: client- and wrapper-side; `viola console` reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each decoded to one closed class (0x03 anywhere interrupt, exactly q quit, anything else unbound), no key byte reaching any log line or output.
