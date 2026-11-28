@task_prose part=chunk
Builds the fleet console's scaffold: its own surface section in the masters (replacing the stub deferred to 0.2.0) and the minimal console code behind it, a frame fixed to a quarter-screen layout that stays whole at its smallest size and degrades by a named rule below it.
Reconciles the rows banning redraw (spinner, in-place rewrite, cursor-moving status) with an interactive console: an exception named for the console's own surface only, never generalised.
Adds a test-only keystroke driver that feeds the console keys through its own pseudo-terminal under a test-owned home and asserts on the console's state (a structured read or its log lines), never on screen bytes, over the fake agent and fixtures; and gives the console its own diagnostics role beside run, hook, mcp and ui, reachable through the logs command, with its obs event catalog rows.
Folds two carries: re-reads RUSTSEC-2026-0543 (chrono, unmaintained) for a safe upgrade, the ignore leaving or standing one more epoch; and the audit items M1 (one step function for the per-paste probes of the verify turns framing, a look at the send path's guarded section) and M3 (the relied-row count from one source, the stamped-set table held once).
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny dependency policy, its advisory ignores with their reason lines — edits: the RUSTSEC-2026-0543 (chrono, unmaintained) ignore entry and its reason line saying it is re-read at each epoch boundary
@task_file n=2 path=src/cmd/verify/typed/turns.rs mark=edits
what: viola verify's typed probe of turns, the framing turns and their per-paste probes — edits: the framing_turns function, its per-paste probes folded into one step function
@task_file n=3 path=src/run/send.rs mark=edits
what: the wrapper's send path, its post-gate section under the wrapper flight lock — edits: the send_through function's guarded section
@task_item n=1 part=needs
The console's surface: the terminal fleet console as the one view, a keyboard-driven terminal surface (never a web or graphical one), written into the masters beside the CLI surface; the rows owed to «Fleet console view» and «Console actions» are named as owed, not built.
@task_item n=2 part=needs
The console's start verb or flag and its name: a new capability enters as a subcommand or flag, so the name joins the register of commands and the CLI verbs and arguments boundary, in the CLI's flag and subcommand style with the viola prefix; the name is the founder's fork, its conservative arm taken provisional.
@task_item n=3 part=needs
Quarter-screen layout: the frame stays whole at a quarter of the screen and at its smallest size, and degrades by a named rule below it; the scaffold opens or moves no window and the tiled window is the view entry's.
@task_item n=4 part=needs
Redraw-ban reconciliation: the rows that ban redraw on the CLI's surfaces, the rule that run writes only the child's output to the terminal, and the colour rules (clap without colour, colour variables deciding only SGR output) reach the console only through an exception named for its own surface; the rule stays as it is for every other surface — a boundary widening, the founder's fork.
@task_item n=5 part=needs
Human output writing: the console's drawing must go through the one human writer of the root bin, under the print and dbg macro bans and the clippy deny-warnings gate; short-lived CLI verbs' stderr allowance does not cover a long-lived console.
@task_item n=6 part=needs
A new dependency that draws or reads a terminal is the founder's fork; any one taken must pass the cargo-deny policy (licence allowlist, crates.io sources only, no C-building crates, banned features, no telemetry crates), be pinned once in the workspace's dependency table, and keep the sync crates tokio-free with Tokio only inside mcp.
@task_item n=7 part=needs
Where the console code lives: no crate without a real consumer and a test; any new crate follows the crate naming, the flat workspace with the bin at the root, and the dependency direction (the root bin consumes only normalised events, viola-core depends on no viola crate, the pty seam knows no agent), with no orphan module; libc stays the one OS-binding crate and portable-pty stays behind viola's own pty seam.
@task_item n=8 part=needs
The console is a process, never a daemon: no daemon, no listening network port, no polling; the disk stays the shared truth.
@task_item n=9 part=needs
The keystroke test driver: test-only, kept out of release builds and product builds behind the test-support feature, driving only the scaffold's own keys (quit and what the frame needs) through its own pseudo-terminal; it asserts on the console's structured state or its log lines, never on screen bytes.
@task_item n=10 part=needs
The console's state read for tests: a structured read the console exposes for tests is a test seam, never a configuration channel; any seam variable carries the test-harness or fake-agent seam prefix, never the VIOLA_ product namespace, and is never read by the product otherwise.
@task_item n=11 part=needs
Console tests run over the fake agent and fixtures as the contract (headless agent verification first): no live claude, no real terminal key, no compositor dispatch, nothing read of the operator's real home; each under a test-owned viola home and bounded under the 20 s lint and the root tests' one shared wait bound.
@task_item n=12 part=needs
The console's own diagnostics role: a home-level diagnostics role file beside run, hook, mcp and the cli role files, its lines codes-only in the one diagnostics line format, emitted only through obs_event!, levelled only by diagnostics_level, opened only after the strict-modes check and the diagnostics-file mode checks; reachable through the logs command.
@task_item n=13 part=needs
The roles' list and its count: wherever a row, a test or the obs event catalog pins the set of diagnostics roles, the console's role joins it (or ui is shown to be that role), and the catalog gains the console's event rows.
@task_item n=14 part=needs
The chrono advisory carry: RUSTSEC-2026-0543 re-read under the cargo-deny dependency policy and the weekly advisory audit; a safe chrono upgrade, if found, keeps the RFC 3339 UTC millisecond timestamps and removes the ignore with it; none found, the ignore stands one more epoch on the founder's word, its reason line still saying it is re-read at each boundary.
@task_item n=15 part=needs
M1 restructure's invariant: framing_turns' per-paste probes folded into one step function, and send_through's guarded section (the send post-gate section under the wrapper flight lock) looked at; what verify's framing probes record and what the send path does under the flight lock stay as they are, held by the existing verify and send tests.
@task_item n=16 part=needs
M3: the relied-row count read from one source instead of literals moved by hand at each new row, and the stamped-set table held once; the ledger's rule that a CLI version is verified only when every relied row counts, and the stamps' one writer, read that single source.
@task_item n=17 part=needs
Console a11y landmarks: the cli and tui landmarks of the a11y plan carry the console's surface; the SC checks over the finished console are a sibling's.
@task_standing n=1
convention: each new test runs in its own viola home, removed by its owner record only, its channel sockets under the short /tmp test root outside the product namespace, its evidence surviving a killed test
home: occupied-resources
@task_standing n=2
convention: each new test runs against the fake agent in CI on ubuntu, never a live claude
home: ci-cd-approach
@task_standing n=3
convention: each new log line is emitted through obs_event! only, codes only, no tracing level macro
home: build-system
@task_standing n=4
convention: each new test gated on an OS names its reason and names no other OS
home: cross-cutting-patterns
@task_standing n=5
convention: each new test binary, test-support code or fixture stays out of product and release builds, fixtures scrubbed and hygiene-walked
home: occupied-resources
@task_standing n=6
convention: each new insta snapshot runs in check mode only, and end-to-end runs keep zero panic lines
home: project-directory-structure
@task_standing n=7
convention: each new code path stays inside the coverage floors enforced by the harness gate
home: ci-cd-approach
@task_item n=1 part=not_needed
MCP tool rows (the tools list, tool results, tool errors, the stdin line cap, the call deadline): the scaffold adds no MCP tool and changes no MCP behaviour.
@task_item n=2 part=not_needed
Hook rows (hook tiers, the hook-to-kind map, hook exit codes and bodies, the hook capture arm): the scaffold adds no hook.
@task_item n=3 part=not_needed
Wrapper channel method, frame, fault-code and wait rows: the scaffold sends no verb and draws no fleet row; those reach «Console actions» and «Fleet console view».
@task_item n=4 part=not_needed
Wheel, dialog hold, answer and link rows: the scaffold carries no verb or human control over a session.
@task_item n=5 part=not_needed
Hyprland seam errors, hyprctl spawn and window-placement rows: the scaffold opens or moves no window and dispatches nothing to the compositor.
@task_item n=6 part=not_needed
The board and list payload item rows (status, liveness, wheel, links, parent): the fleet rows inside the frame are the view entry's.
@task_item n=7 part=not_needed
Revive, adopt, up, open and plugin-install exit codes, JSON documents and refusal codes: no such verb changes here.
@task_item n=8 part=not_needed
Trust-writer code words and verify refusal and process-exit codes: the verify changes are the M1 and M3 restructures only, no new refusal.
@task_item n=9 part=not_needed
The fuzz workspace, release tooling and crates.io publishing rows: nothing here fuzzes, releases or publishes.
@task_item n=10 part=not_needed
The child's argv and environment rows and the pinned exe copy rows: the console starts no claude child.
