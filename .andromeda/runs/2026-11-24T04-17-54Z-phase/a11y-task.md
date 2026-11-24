@task_prose part=chunk
Adds a new human CLI verb `viola up` that re-opens the fleet after a host reboot: it reads the board's instances under the home, selects those that were live before the host went down, and orders them parents before children by the recorded lineage.
It skips every instance whose recorded pid and start time verify live, and re-opens each remaining one resumed in place through the existing `revive` start arm, in the terminal window `viola open` would give it, in its recorded workspace.
It prints one line per instance naming the instance and a closed outcome word, and under `--json` one document of codes and closed words only, beside the existing CLI machine contract.
It adds tests proving the selection, order, skip, lines and the argv handed to each start over the fake agent and the Hyprland fakes under a test-owned home; every live reading of `up` is left to the operator and its clause GATED.
@task_file n=1 path=src/cmd/mod.rs mark=edits
what: the root bin's clap dispatch, the CLI verb table and its help rows — edits: the verb table entry and help row for `viola up`
@task_file n=2 path=src/cmd/up.rs
what: the new `viola up` verb — the fleet re-open over the board read model, the lineage order, the live skip and the per-instance revive start in an open window
@task_file n=3 path=src/human/verbs.rs mark=edits
what: the human module's fixed verb result lines written in one write — edits: the per-instance `viola up` line naming the instance and its outcome word
@task_file n=4 path=src/obs.rs mark=edits
what: the obs event catalog and the process-log lines — edits: the per-instance `up` event line
@task_file n=5 path=tests/cli_up.rs
what: the new CLI test suite for `viola up` over the fake agent and the Hyprland fakes under a test-owned home — selection, order, skip, lines, `--json` document and the argv handed to each start
@task_item n=1 part=needs
The new verb joins the CLI verb table and `viola --help`: its help row, its group label, the usage line, and only served verbs listed; it is a human CLI verb only, with no MCP tool added (the driver re-opening the fleet stays CLI-only).
@task_item n=2 part=needs
Each per-instance line is linear, static, plain output: one line per instance written whole, no spinner or progress redraw while a fleet of N starts, a closed outcome word carrying the state in print rather than by colour.
@task_item n=3 part=needs
Colour on the up lines: no SGR when stdout is not a colour-capable terminal, any SGR only beside its word, refusal and failure words never coloured, and the colour decision taken from the same inputs as the other verbs.
@task_item n=4 part=needs
Instance names, workspaces or any other recorded value printed on an up line or a refusal pass through C0/C1 control-character escaping; printed names are the instance names, never paths (paths not carried).
@task_item n=5 part=needs
Under `--json`, `up` emits one clean document of codes and closed words only, no hint, no SGR, no free text; failure documents and fault bodies under `--json` keep the CLI machine contract's shape.
@task_item n=6 part=needs
An instance the `revive` start arm refuses yields its existing closed refusal code (already-live, session-live, stale-heartbeat, cwd-missing, no-session, squatted-name, pinned-hash-mismatch, strict-modes-failed, socket-dir-refused, internal-error) on that instance's line or in the `--json` document; unlisted code words stay visible, never swallowed or renamed.
@task_item n=7 part=needs
Human refusals of `up` itself keep the CLI refusal form: an unable line with its kebab-case reason code beside a plain word, then exactly one hint line per code, faults and the `unknown` refusal carrying no hint, no hint naming a release, no stack trace or bare status as the error text, and thiserror messages interpolating nothing.
@task_item n=8 part=needs
Exit codes of `up` come from the existing CLI exit-code table (usage error 2, refusal 1, the existing higher codes for unreachable or failed instances); a fleet with some instances refused or skipped maps to a stated code, never a new undeclared one.
@task_item n=9 part=needs
Home strict-modes and socket-directory checks run before any write or any frame: an unstattable or loose home is a strict-modes refusal, and a socket-directory failure refuses before the first start.
@task_item n=10 part=needs
The skip of a still-live instance reads the board's liveness (snapshot pid plus start time, heartbeat) and is reported as its own closed outcome word on that instance's line.
@task_item n=11 part=needs
Each re-open goes through `revive`'s preflight, including the `claude agents --json` read, which fails open as it does for a single revive; `up` adds no stricter gate of its own on that read.
@task_item n=12 part=needs
Time bounds of each start (start verification, its stop grace, the wait for a parent before its children) fall on the driver, never on the human: no time limit is put on the operator and no timeout ends silently.
@task_item n=13 part=needs
`up` asks nothing interactively: no prompt, no confirmation, no keyboard input read from the operator while it runs.
@task_item n=14 part=needs
Each re-opened session runs inside the `viola run` TUI passthrough in its own terminal window: zero viola bytes on that terminal, the terminal belongs to the child, human keystrokes are never blocked, and run diagnostics never reach the terminal; `up` writes nothing into a child's terminal.
@task_item n=15 part=needs
The per-instance process-log line keeps the NEVER-log floor (no window address, pid or workspace in a process-log line) and carries the log identity tags (process, service_name).
@task_item n=16 part=needs
The fixed strings of `up` (outcome words, line layout, refusal and hint copy) come from design and are pinned by literal in tests, the refusal layout held by literal pins.
@task_item n=17 part=needs
Live readings of `up` (a real host restart, a real resumed session, a real window) never gate the build; the fakes carry what they can and the rest is GATED on the operator's reading, with manual keyboard or screen-reader passes supplemental only.
@task_standing n=1
convention: every new CLI a11y check is a literal output-discipline test that runs inside the existing nextest suites under the coverage job, with no separate a11y driver, suite or artifact
home: a11y-testing-tool-pick · bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: an a11y failure in a new test is a nextest failure that fails the GitHub Actions gate, with coverage and doctest required, no skips
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: each new test is deterministic with zero flakiness and no retries; a breach fails the build with no soft SLO
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: boundary tests that drive starts and windows are sequenced on records, never on screen content, and never inspect claude content
home: keyboard-test-harness
@task_item n=1 part=not_needed
The `viola list` board rows (caption order, empty board, unwrapped separator, stale-row dim, NAME bold and truncation): `up` prints its own per-instance lines, not the board.
@task_item n=2 part=not_needed
Dialog, send, answer and wheel rows (dialog amber, dialog hold, dialog-expired event, send refusals while the human types, human key takes the wheel, MCP wait tool timeout): `up` neither answers dialogs nor sends prompts.
@task_item n=3 part=not_needed
MCP server frame-tool and hook event or dialog-call rows: `up` adds no MCP tool and no hook call.
@task_item n=4 part=not_needed
`viola link` / `viola unlink`, `viola plugin install`, `viola verify` and `viola wait` / `last` output contracts: other verbs whose lines `up` does not write.
@task_item n=5 part=not_needed
Absences and graphical-frontend rows (web or graphical frontend, touch surface, rendered page, internationalization) and the empty WCAG-criteria, contrast and focus-management harness areas: `up` is a terminal CLI verb.
@task_item n=6 part=not_needed
Tier, compliance-regime and conformance-claim decisions (a11y tier Standard, no WCAG conformance claim, no compliance regime, cognitive accessibility not triggered): plan-level settings the chunk does not change.
