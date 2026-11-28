@task_prose part=chunk
Builds the fleet console's scaffold: the console's own surface section written into the masters in place of the 0.1.0 stub deferred to 0.2.0, with the minimal code behind it, the rows of the view's columns and the actions' keys named as owed to later entries.
Fixes the console's quarter-screen layout and smallest whole size, and a frame that stays whole there and degrades by a named rule below it.
Reconciles the redraw bans (spinner, in-place rewrite, cursor-moving status) with an interactive console by an exception named for the console's own surface only, never generalised.
Adds a test-only keystroke driver feeding the console its scaffold keys (quit and what the frame needs) through its own pseudo-terminal under a test-owned home, asserting on the console's state or log lines, never screen bytes; and gives the console process its own diagnostics role reachable through the logs command.
Folds two carries: re-reads the chrono advisory RUSTSEC-2026-0543 ignore at the epoch boundary, and the audit's M1 (one step function for the per-paste framing probes, a look at the send guarded section) and M3 (the relied-row count from one source, the stamped-set table held once).
@task_file n=1 path=deny.toml mark=edits
what: the dependency advisory policy of cargo-deny — edits: the RUSTSEC-2026-0543 (chrono, unmaintained) ignore entry and its reason line saying it is re-read at each epoch boundary
@task_item n=1 part=needs
The console's surface: a surface section of its own for the fleet console beside the CLI's surface, replacing the deferred stub, with the rows owed to the fleet console view and the console actions named as owed, not built.
@task_item n=2 part=needs
The fleet console is no verb's output: the console's frame is its own surface, not a verb's stdout or stderr lines, and the CLI verbs' output rules stay with the verbs.
@task_item n=3 part=needs
The redraw bans and their reach: the rows banning spinners, elapsed-time counters, in-place rewrites, cursor-moving status, motion and animated busy, and the one progress pattern; the console gets a named exception for its own surface only, each row keeping its rule for the CLI verbs.
@task_item n=4 part=needs
No interactive prompt or pager: the console is interactive by nature, so this ban is settled for the console's surface by a named exception, never widened to the verbs.
@task_item n=5 part=needs
The console's width and size: the frame's quarter-screen size and smallest whole size, the named degrade rule below it, and how it reads the terminal's size against the rules that COLUMNS is never read, that widths are never hardcoded, and that lines are never wrapped.
@task_item n=6 part=needs
TTY detection: the console decides it holds a terminal by std IsTerminal, as the CLI does.
@task_item n=7 part=needs
Terminal drawing and styling: any new dependency that draws or reads a terminal is a founder fork; the styling stays with the hand-written SGR module, clap stays without its colour feature, no background colour is set.
@task_item n=8 part=needs
The console's frame text obeys the human-output rules: ASCII-only, text glyphs only and no decorative icons, control characters escaped, colour only amber and dim and only as a second cue.
@task_item n=9 part=needs
The console's start verb or flag and its name: a new entry in the command names, the flat-verbs rule and the help output's verb groups; its name is a founder fork taken provisional.
@task_item n=10 part=needs
The console's exit and refusal behaviour: its exit codes, its stderr lines in the refusal and hint form, no stack traces and no internals in errors, as the CLI's exit codes and stderr contracts state them.
@task_item n=11 part=needs
The colour decision order for the console's process: where the console sits beside run, hook and mcp in the NO_COLOR, TERM=dumb and not-a-terminal order.
@task_item n=12 part=needs
The console's diagnostics role: structured logs under its own role beside run, hook, mcp and ui, reachable through the logs command, with the obs event catalog's rows for it; the roles' list and its count wherever pinned.
@task_item n=13 part=needs
The keystroke test driver: test-only, its own pseudo-terminal under a test-owned home, asserting on a structured state read the console exposes for tests or on its log lines, never on screen bytes; driven over the fake agent and fixtures as the contract, bounded under the 20 s test lint.
@task_item n=14 part=needs
The structured state read the console exposes for tests: what it carries and that nothing outside tests depends on it.
@task_item n=15 part=needs
Boundaries of proof: no live start, no window opened or moved, nothing dispatched to the compositor, no key to a real terminal, no real claude, nothing read of the operator's real home; no crate without a real consumer and a test.
@task_item n=16 part=needs
The M1 carry's restructure invariant: the per-paste framing probes moved into one step function and the send guarded section looked at, each doing what it does today, held by the existing send and framing tests.
@task_item n=17 part=needs
The M3 carry's change: the relied-row count read from one source instead of literals moved by hand at each new row, and the stamped-set table held once.
@task_item n=18 part=needs
The chrono advisory carry: RUSTSEC-2026-0543 re-read; a safe upgrade found removes the ignore with it, none keeps it one more epoch on the founder's word, the reason line kept.
@task_item n=19 part=needs
The console's surface section names which existing CLI rows reach the console and which stay the verbs' alone.
@task_standing n=1
convention: every human-facing line or frame text the console adds is ASCII-only, escapes control characters and uses colour only as a second cue
home: anti-patterns-never-do-these
@task_standing n=2
convention: every new stderr message the console adds takes the CLI's unable and hint line form with no internals
home: surface-cli · surface
@task_standing n=3
convention: every new name the console adds to the command register follows the flat-verbs naming of the existing commands
home: surface-cli
@task_item n=1 part=not_needed
The flight-progress strip's field contracts (NAME, STATUS, WHEEL, LIVE, DIALOG and the field with no reading) and the strip-board decisions over sessions' slots, cocked and stale strips: the fleet rows are the fleet console view's.
@task_item n=2 part=not_needed
The send readback, readback box and handoff contracts: verbs by keyboard and their readback are the console actions' entry.
@task_item n=3 part=not_needed
The existing verbs' own output, refusal and json-code contracts (run, revive, verify, up, link and unlink, wait, focus, last, list board and table, plugin install): the scaffold changes none of them.
@task_item n=4 part=not_needed
The exit 21 causes, wrapper fault details, peer reply screening and wrapper socket rows: the scaffold reaches no instance endpoint.
@task_item n=5 part=not_needed
The rendered-page absences and the web surface: the console is a terminal surface, no page.
@task_item n=6 part=not_needed
The design self-validation tests (squint, swap, readback box signature): they judge the finished console, owed to the fleet a11y verification and the view.
