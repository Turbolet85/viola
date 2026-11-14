@task_prose part=chunk
Builds viola's own Hyprland seam: the one place the binary talks to the compositor, carrying only the relied-on window queries and dispatches the later window entries need, no general Hyprland client.
Each reply is read capped and parsed tolerantly into closed viola types as untrusted upstream output; each seam call ends in a closed outcome code logged codes-only, and outside Hyprland the seam answers a closed unsupported outcome.
Adds a fake compositor that replays replies recorded live on the founder's host, so the seam's tests run green on CI where no compositor runs.
Builds no verb and no user-facing output: open, focus, up, adopt and the fleet console are their own later entries.
@task_item n=1 part=needs
The Hyprland seam component itself: the single owner of every Hyprland request, reply shape and socket; nothing outside it names one.
@task_item n=2 part=needs
The relied-on request set: window queries (clients, active window, workspaces, active workspace, monitors) and dispatches (focus a window, show a workspace, place a window).
@task_item n=3 part=needs
Reply reads capped at the frame limit and refused past it; replies parsed tolerantly into closed viola types.
@task_item n=4 part=needs
Reply text (window titles, classes, every other reply string) is content, never a command, and reaches no log line, error or refusal.
@task_item n=5 part=needs
The seam's closed outcome codes (ok, unsupported, unreachable, malformed and the like), one ending each seam call.
@task_item n=6 part=needs
Codes-only logging through the shared event macro: no reply text, address, pid, path or socket name in any line.
@task_item n=7 part=needs
The unsupported outcome when no compositor is reachable: never a panic and never a guess.
@task_item n=8 part=needs
The fake compositor that replays recorded, scrubbed reply fixtures, one fixture set per measured Hyprland version, so the seam's tests run on the CI host.
@task_item n=9 part=needs
Kill deadlines on every test or control that spawns a compositor client, a wrapper or viola itself.
@task_item n=10 part=needs
Indirect: a compositor program resolved on PATH is spawned directly with no shell and an absolute path; a request socket owes the same owner and private-mode checks the channel socket directory has.
@task_item n=11 part=needs
Indirect: the compositor instance signature is a new environment read the binary does not make today, owed its own decision entry.
@task_item n=12 part=needs
Indirect: the seam is Linux-only and the window features target Hyprland only.
@task_item n=1 part=not_needed
The web surface absence: the chunk renders no page.
@task_item n=2 part=not_needed
Board, list, send, wait, last, answer, revive and run output contracts: the chunk adds no verb and changes no verb's printed output.
@task_item n=3 part=not_needed
CLI token rendering and styling rows (colour roles, SGR styling, spacing, motion, BAY line, banners, spinners): the seam prints nothing to a terminal.
@task_item n=4 part=not_needed
The MCP tool and tool-result contracts: no MCP tool is added or changed.
@task_item n=5 part=not_needed
Help output, verb groups and per-verb usage rows: no verb is added to the help.
@task_item n=6 part=not_needed
The verify output and verify refusal contracts: the Claude capability ledger stays Claude-only and the verify verb is untouched.
@task_item n=7 part=not_needed
Release, pause, plugin install, link and unlink output rows: none of those verbs is touched.
