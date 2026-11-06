@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `.andromeda/architecture.md` §Standard Contracts (Wrapper 
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `.andromeda/architecture.md` §Standard Contracts (snapshot
@leaf_edit n=1 leaf=.claude/docs/services/viola-pty.md line=6 class=stale
leaf-text: viola's own PTY seam over portable-pty `=0.8.1`
source: fct-omwrdnlc «portable-pty `=0.9.0` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill).»
replace-with: viola's own PTY seam over portable-pty `=0.9.0`
@leaf_edit n=2 leaf=.claude/docs/services/viola-pty.md line=11 class=stale
leaf-text: portable-pty `=0.8.1`; tracing
source: fct-kxiazeyf «| PTY layer | portable-pty `=0.9.0` (pinned) behind viola's own `pty` seam; libc `=0.2.189` for the host tty's raw mode, behind `HostTerminal`»
replace-with: portable-pty `=0.9.0`; tracing
@leaf_edit n=3 leaf=.claude/docs/services/viola-pty.md line=27 class=retired
leaf-text: - Moving `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441) is owed, owner: "Version done-check"; the swap trigger is portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run`.
source: fct-omwrdnlc «0.9.0's graph holds neither `pin-utils` (RUSTSEC-2026-0441) nor the `serial` crate family, and `viola-pty` builds and passes its tests on it unchanged.»
replace-with: - `portable-pty` is pinned `=0.9.0`: its graph holds neither `pin-utils` (RUSTSEC-2026-0441) nor the `serial` crate family, and `viola-pty` builds and passes its tests on it unchanged.
@leaf_edit n=4 leaf=.claude/docs/services/viola-channel.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md`
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: Primary source: `doc:architecture`
@leaf_edit n=5 leaf=.claude/docs/services/viola-channel.md line=45 class=tree
leaf-text: - `.andromeda/architecture.md` §Standard Contracts (Wrapper channel frames)
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: - `doc:architecture` §Standard Contracts (Wrapper channel frames)
@leaf_edit n=6 leaf=.claude/docs/services/viola-core.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md`
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: Primary source: `doc:architecture`
@leaf_edit n=7 leaf=.claude/docs/services/viola-core.md line=42 class=tree
leaf-text: - `.andromeda/architecture.md` — §Conventions
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: - `doc:architecture` — §Conventions
@leaf_edit n=8 leaf=.claude/docs/services/viola-mcp.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md`
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: Primary source: `doc:architecture`
@leaf_edit n=9 leaf=.claude/docs/services/viola-mcp.md line=39 class=tree
leaf-text: - `.andromeda/architecture.md` [MCP]
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: - `doc:architecture` [MCP]
@leaf_edit n=10 leaf=.claude/docs/services/viola-pty.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md`
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: Primary source: `doc:architecture`
@leaf_edit n=11 leaf=.claude/docs/services/viola-pty.md line=47 class=tree
leaf-text: - `.andromeda/architecture.md` [PTY]
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: - `doc:architecture` [PTY]
@leaf_edit n=12 leaf=.claude/docs/services/viola-state.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md`
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: Primary source: `doc:architecture`
@leaf_edit n=13 leaf=.claude/docs/services/viola-state.md line=49 class=tree
leaf-text: - `.andromeda/architecture.md` §Standard Contracts (snapshot envelope, instance snapshot)
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: - `doc:architecture` §Standard Contracts (snapshot envelope, instance snapshot)
