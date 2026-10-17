# P4 forks of chunk 2026-10-17-cli-output-discipline — the dialog and its answers, verbatim

Asked at P4 in one AskUserQuestion round (four questions); the recommended option first, marked `(Recommended)`.

## Q1 — Env reads
Boundary widening (playbook pattern "Boundary widening", verdict escalate): may the colour layer read NO_COLOR, TERM
and COLORTERM to decide whether and how deep to colour `viola list`?
1. Read them (Recommended) — the seven-rung decision as design-system §Tokens and a11y-plan P6 specify; recorded as a
   widening of architecture Config management's env sentence and security-plan Secret Management, the reads gating
   only SGR bytes, never escaping, redaction or any control; lets v1-40 be claimed.
2. No env reads — colour whenever stdout is a terminal (and the Windows VT enable succeeds); contradicts design-system,
   layout-templates and a11y-plan P6; v1-40's no-colour clause fails.
3. Defer the colour layer — the board stays uncoloured; tokens, decision and spans go to a new route entry; v1-40
   cannot be claimed here.

Answer: "HOLD — the founder's own word is owed on this widening; he is away overnight and a widening is never settled
by a delegate. Do not build the env reads or pick another branch for it."
Notes: "Overseer at 2026-10-17T04:47:12Z: hold Q1 for the founder."

## Q2 — Width
Where should the unwrapped-NAME truncation on a terminal read the width from?
1. viola_pty::host_size() (Recommended) — the existing reader (a stdout TIOCGWINSZ ioctl on Unix,
   GetConsoleScreenBufferInfo on Windows); the real width, no env read; design-system §Surface: cli Width amended as
   built at the wrap.
2. COLUMNS env var — as design-system words it; a fourth env read added to Q1's widening.
3. Carry truncation — unwrapped NAMEs stay whole; the clipping moves to a new route entry.

Answer: "viola_pty::host_size() (Recommended)"
Notes: "Overseer, technical (founder-delegated): the real width with no env read; design-system's Width line amended as
built at the wrap."

## Q3 — Help rows
The layout-templates wireframe groups `link, unlink` and `plugin install`, which the binary does not serve. What should
the grouped `viola --help` list?
1. Served verbs only (Recommended) — only verbs clap accepts; the `handoff:` row dropped until link/unlink land,
   `plugin install` waits for "Optional plugin install", `Options:` kept for the completeness case; design-system and
   layout-templates amended as built at the wrap.
2. The wireframe as written — all five groups, naming verbs clap refuses with a usage exit 2.

Answer: "Served verbs only (Recommended)"
Notes: "Overseer, technical: help never offers a verb that exits 2; masters amended as built at the wrap."

## Q4 — Authority
Whose word are these answers? The env-read widening counts as ratified only on the founder's own answer, given by him
or relayed verbatim.
1. The founder's own.
2. A delegate's — from the operator or an overseer answering in his stead; provisional, the widening NOT ratified.

Answer: "A delegate's"
Notes: "Q2 and Q3 are the overseer's technical answers, PROVISIONAL; Q1 is HELD for the founder's own word — stop at
the end of P4 with the plan unsynthesised on the colour layer and say what waits."
