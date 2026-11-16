@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once a
@leaf_edit n=1 leaf=.claude/rules/events.md line=20 class=stale
leaf-text: matched exactly on the send's typed text, the sent text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters)
source: ctr-u4sxg2hi «A prompt-submitted whose text is the in-flight send's typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with origin driver, its text kept as submitted»
replace-with: matched on the send's typed text, the sent text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters, or on a non-empty text equal to that typed text less its trailing run of ASCII space, TAB, CR and LF, the text kept as submitted)
