@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=47
last: - 2026-11-24: A new verb that starts through `run::start` in
@leaf_edit n=1 leaf=.claude/rules/security.md line=14 class=omission
leaf-text: and at the start of `viola mcp` on its home (exit 1, nothing on stdout or stderr, no role file)
source: dec-663jcbro «decision/console-terminal-and-home-check · viola console checks its terminal and home first — viola console refuses unless stdin and stdout are both terminals, then holds the home to the strict-modes read rule»
replace-with: and at the start of `viola mcp` on its home (exit 1, nothing on stdout or stderr, no role file), and `viola console`, after refusing unless stdin and stdout are both terminals, on its home before any log opens (an absent home passes; either refusal exit 1, no line, modes left as found, the strict-modes refusal the fixed `unable: viola console could not use its home` / `hint: make the viola home readable by you alone, then run viola console again` pair on stderr)
@leaf_edit n=2 leaf=.claude/rules/security.md line=25 class=omission
leaf-text: on every external reader, and
source: dec-dsjweani «decision/console-key-reader-bounded · Console key reader bounded and closed — viola console reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each read decoded to one closed class»
replace-with: on every external reader, `viola console`'s keys read from its own terminal in raw mode in reads of at most 64 bytes, each decoded to one closed class (0x03 anywhere interrupt, exactly `q` quit, anything else unbound; no key byte reaches any log line or output), and
