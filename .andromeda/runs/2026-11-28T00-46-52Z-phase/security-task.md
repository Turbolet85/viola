@task_prose part=chunk
Builds the fleet console's scaffold: its surface section in the masters, a frame that stays whole at a quarter of the screen and degrades by a named rule below it, and the minimal code behind them.
Reconciles the redraw-ban rows with the console, as an exception named for the console's own surface only, never a generalised rule.
Adds a test-only keystroke driver that feeds the console's own keys through its own pseudo-terminal under a test-owned home and asserts on the console's state or log lines, never on screen bytes, over the fake agent and fixtures.
Gives the console process its own diagnostics role beside run, hook, mcp and ui, reachable through the logs command, with event catalog rows for it.
Folds two carries: re-reads the chrono unmaintained advisory (an upgrade lifts the ignore, else it stands one more epoch), and restructures the framing per-paste probes into one step function and the relied-row count and stamped-set table into one source each.
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny gate's configuration on the root graph, holding the closed advisory ignore list — edits: the RUSTSEC-2026-0543 chrono ignore entry and its reason line
@task_item n=1 part=needs
A new interactive console process and surface: the console is a new CLI surface with its own start verb or flag, so the CLI input trust boundary and the checks a CLI verb owes before it reads the home (home resolution order through --home and VIOLA_DIR, owner-only home, strict-modes check before any read) bind its start.
@task_item n=2 part=needs
The console's start checks the home tree before its diagnostics log opens; a refused diagnostics open writes nowhere, and a diagnostics file failing strict modes fails open.
@task_item n=3 part=needs
Redraw-ban reconciliation: any redraw allowance is an exception named for the console's own surface, never a widened or switchable control; no config, env var or flag turns a control off, and no width or colour override env is introduced.
@task_item n=4 part=needs
The console's frame is human-mode terminal output: C0/C1 control characters in any text it draws are escaped, terminal colour facts choose SGR only and colour environment variables are only compared, and error messages it shows are fixed, interpolating no paths, ids or upstream text.
@task_item n=5 part=needs
The console reads keystrokes from its terminal: every product reader stays bounded or is named in the bounded-readers census, and a key read is input checked at the seam, never treated as a command from upstream text.
@task_item n=6 part=needs
The console's own diagnostics role: its log files are owner-only and keep their modes over rotation, user content goes only to diagnostics, full error detail only to diagnostics, and CLAUDE values and messaging secrets are never logged.
@task_item n=7 part=needs
The console's process is a new product error writer and a new source of panics: every error writer is named, the zero-panics gate's single exemption is not widened, and stderr honours the never-log floor.
@task_item n=8 part=needs
A state read the console exposes for tests is a test seam: it lives only under the fake-agent feature, is compiled out of release, the release build refuses test-only binaries and features, and test-support stays a development-only dependency.
@task_item n=9 part=needs
The keystroke test driver keys only into its own pseudo-terminal under a test-owned home, runs no real claude, reads nothing of the operator's real Claude home or viola home, removes only directories it created, and stays bounded under the 20 s test lint.
@task_item n=10 part=needs
Any new dependency that draws or reads a terminal: crates.io only, exact pin, licence on the closed allow list, no C-building crate, no tokio in the sync crates, within the minimum supported Rust version, and passing the cargo-deny gate on the root graph.
@task_item n=11 part=needs
The chrono advisory carry: the RUSTSEC-2026-0543 ignore is admitted only on the advisory-ignore grounds and stays on the closed ignore list only while no safe upgrade exists; an upgrade removes the ignore with it, and the reason line keeps saying it is re-read at each epoch boundary.
@task_item n=12 part=needs
Restructure invariant of the two code-audit carries: the framing per-paste probes folded into one step function, the send path's guarded section reviewed, and the relied-row count and stamped-set table each held in one source; send stays one bracketed paste with its paste-text checks on both sides and stamps still checked before and under lock, held by the existing tests over framing, send and stamps.
@task_item n=13 part=needs
A new log role and any new environment read or seam the console adds owes its log entry, and its log lines carry no paths, pids or ids in refusals.
@task_item n=14 part=needs
The console writes nothing under the Claude home and sends no key to a real terminal, opens or moves no window and dispatches nothing to the compositor in this chunk.
@task_item n=15 part=needs
No crate without a real consumer and a test: a console crate, if added, is reached by the console and exercised headless over the fake agent and fixtures.
@task_standing n=1
convention: every test runs under its own home, never the user's home or real Claude home
home: secret-management
@task_standing n=2
convention: test homes are kept out of git
home: bootstrap-phases
@task_standing n=3
convention: every new fixture is recorded clean under the fixture hygiene rules and reviewed before commit
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: every new third-party crate is pinned exactly and passes the dependency audit tools
home: dependency-security
@task_standing n=5
convention: every new log line holds no secret, CLAUDE value or user content outside diagnostics
home: bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
MCP tools and their inputs, results and structuredContent contracts: the scaffold adds no MCP tool and changes no MCP reply.
@task_item n=2 part=not_needed
The wrapper channel's methods, frames, error codes and response-log details: the scaffold drives no verb and sends no frame; the console's actions belong to a later entry.
@task_item n=3 part=not_needed
Hook stdin, hook decisions and hook refusal contracts: the console is not a hook and touches no hook path.
@task_item n=4 part=not_needed
viola revive and its --json codes, and the run and revive start refusals: the scaffold changes neither verb.
@task_item n=5 part=not_needed
viola verify's probes, refusal codes, live rigs and real-CLI residuals: no live round, no real claude and no verify change in this chunk.
@task_item n=6 part=not_needed
Pinned binary and plugin install, marketplace and trust record rows: the console places, hashes and installs nothing.
@task_item n=7 part=not_needed
Hyprland window recording, focus and window placement rows: the tiled window and fleet rows are a later entry's, and this chunk moves no window.
@task_item n=8 part=not_needed
Link and unlink events, lineage and launch-pair handling: the scaffold shows no fleet rows and reads no session record.
@task_item n=9 part=not_needed
Release, signing and deploy rows and release workflow rows: the chunk adds no release stage.
