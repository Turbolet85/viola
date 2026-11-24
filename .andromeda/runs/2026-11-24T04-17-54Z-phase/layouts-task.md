@task_prose part=chunk
Adds a new human CLI verb `viola up` that re-opens the fleet after a host reboot: it reads the board's instances under the home, selects those that were live before the host went down, and orders them parents before children by the recorded lineage.
It skips every instance whose recorded pid and start time verify live, and re-opens each remaining one resumed in place through the existing revive start path, in the terminal window viola open would give it, in its recorded workspace.
It prints one line per instance naming the instance and a closed outcome word, and under --json one document of codes and closed words only, beside the existing CLI machine contract.
It adds tests proving the selection, the order, the skip, the lines and the argv handed to each start, over the fake agent and the Hyprland fakes under a test-owned user home; no live start, window, dispatch or key in this phase.
@task_file n=1 path=src/cmd/ mark=edits
what: the root bin's clap dispatch of the CLI verbs — the verb table, help rows and exit causes — edits: a new up verb and its dispatch arm
@task_file n=2 path=src/human/ mark=edits
what: the human output module that renders CLI lines and the --json documents — edits: the per-instance up line and its --json document
@task_item n=1 part=needs
The CLI verb table and the viola --help output: a new flat verb up joins the verb group rows and gets a per-verb usage under viola up --help.
@task_item n=2 part=needs
The per-instance output line of viola up: one line per instance naming it and a closed outcome word (re-opened, skipped as still live, refused), words carrying every state, results on stdout and the rest on stderr, no spinner, no progress glyph, no banner, no prompt.
@task_item n=3 part=needs
The --json form of viola up: one document of codes and closed words only, under the shared --json mode and exit-code --json form of the CLI machine contract.
@task_item n=4 part=needs
The CLI exit codes and refusal lines for viola up: which exit code a whole run returns when some instances are refused, the shared refusal line form and word column, a hint for every refusal cause as the last stderr line naming the next verb and never a path.
@task_item n=5 part=needs
The revive start arm it re-opens each instance through: the revive preflight order, the session-live check, the strict-modes checks and the closed refusal codes (already-live, session-live, no-session, cwd-missing, stale-heartbeat, strict-modes-failed) carried into up's per-instance outcome.
@task_item n=6 part=needs
The skip of a still-live instance: liveness read from the board read model (snapshot pid plus start time, heartbeat), matching the already-live refusal of revive and run, and reported as a skip, not a failure.
@task_item n=7 part=needs
The window placement of a re-opened instance: the terminal spawn viola open uses and the wrapper's window record under --origin, against revive's current no-window placement; the Hyprland seam's closed request set is the only route.
@task_item n=8 part=needs
The parent-before-child order: read from the recorded lineage (parent, role) of each instance, as the board tree reads it, with what a missing, never-live or cyclic parent does settled at P3.
@task_item n=9 part=needs
The launch pair (model, effort) and lineage record written at every start reach each instance up re-opens, as revive's start already writes them; up never passes a launch pair of its own.
@task_item n=10 part=needs
The target of each re-open is a wrapped ViolaName from the board, never case-transformed, rendered as the board and revive render it.
@task_item n=11 part=needs
The --home global flag: up reads instances under the given viola home, and its strict-modes check of the home tree refuses as list and revive refuse.
@task_item n=12 part=needs
The CLI catch site and internal-error output in human and --json form apply to the new verb.
@task_item n=13 part=needs
CLI-only: re-opening the fleet is a boundary widening whose conservative arm adds no MCP tool for up; the MCP tool set stays as it is.
@task_item n=14 part=needs
Every clause of v2-12 that needs a real host restart or a real resumed session is claimed GATED on the operator's live reading; the tests reach the verb only over the fake agent, the Hyprland fakes and a test-owned user home.
@task_standing n=1
convention: each new CLI verb owes a verb table row, a help row, a per-verb usage, an exit-cause table entry and a --json form within the shared machine contract
home: surface-cli
@task_standing n=2
convention: each new human output line owes plain-mode and uncoloured-mode rendering, hand-written SGR styling only through the token roles, and a hint for every refusal cause
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page and up adds none.
@task_item n=2 part=not_needed
The send, wait, last and answer output contracts and the readback mirror and its vocabulary: up sends no prompt and reads no turn.
@task_item n=3 part=not_needed
The wheel and handoff verb output (pause, release) and the link and unlink output: up does not take or hand the wheel, nor link instances.
@task_item n=4 part=not_needed
The viola verify output and refusals: up runs no fixture verification against the claude CLI.
@task_item n=5 part=not_needed
The viola plugin install output and refusals: up installs nothing.
@task_item n=6 part=not_needed
The MCP tool results and the MCP tools rows for answer, last, list, send and wait: up adds no MCP tool and changes none.
@task_item n=7 part=not_needed
The viola list board's rendering details (column widths, truncation, LINKS block, stale-row dimming, the BAY line): up prints one line per instance, not a board.
@task_item n=8 part=not_needed
Deep links or URL coupling in the CLI: up names no link.
