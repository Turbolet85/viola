@answer n=1 entity=visual-design-verification#1.1 v=yes
why: task item 5 names the CLI SGR discipline under the outer PTY that the portable-pty bump reaches, and this entity's amber, bold, dim and no-SGR rules are what those outer-PTY and unit witnesses assert
@answer n=2 entity=visual-design-verification#2.1 v=yes
why: the stale-row escape sequence [DIM, BOLD, RESET, DIM, RESET] is the outer-PTY witness the chunk must keep green on ubuntu-latest after the bump, which task item 5 names as dim on stale rows under the outer PTY
