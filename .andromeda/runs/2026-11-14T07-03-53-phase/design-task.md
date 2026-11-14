@task_prose part=chunk
Builds viola's own Hyprland seam: the one place the binary talks to the compositor, as the pty and channel seams are the one place their libraries are used; nothing outside it names a Hyprland request, reply shape or socket.
It carries only the relied-on window queries and dispatches the later window verbs and the fleet console will need (open, focus, up, the console), no general Hyprland client, and builds no verb itself.
Each reply is read capped and treated as untrusted upstream output, parsed tolerantly into closed viola types; each call ends in a closed outcome code logged codes-only; outside Hyprland it answers a closed unsupported outcome.
A fake compositor replays replies recorded live on the founder's host and scrubbed, so the seam's tests run green in CI where no compositor runs; the reply shapes and dispatch effects are measured live in named operator steps.
@task_item n=1 part=needs
The Hyprland seam as a component: the single boundary between the binary and the compositor, exposing only the relied-on queries (clients, active window, workspaces, active workspace, monitors) and dispatches (focus a window, show a workspace, place a window) as closed viola types.
@task_item n=2 part=needs
Reply reading: every compositor reply read under a frame cap and refused past it, then parsed tolerantly; a reply that does not parse is a closed malformed outcome, never a panic and never a guess.
@task_item n=3 part=needs
Reply text as untrusted content: window titles, classes and every other text the compositor returns are content, never a command, and reach no log line, error, refusal or hint.
@task_item n=4 part=needs
The seam's closed outcome codes (ok, unsupported, unreachable, malformed, and the rest the plan closes) as a new set of codes, one ending each seam call.
@task_item n=5 part=needs
Codes-only logging of seam calls through the shared observation event: each line carries the outcome code alone, with no reply text, window address, pid, path or socket name.
@task_item n=6 part=needs
The not-under-Hyprland state: with no compositor reachable the seam answers the closed unsupported outcome, never a panic, a stack trace or a guessed answer.
@task_item n=7 part=needs
Errors and failures inside the seam surface no internals: no path, pid, socket name, instance signature or reply text in any error the seam hands back to a caller.
@task_item n=8 part=needs
The fake compositor: a test double that replays recorded replies so the seam's tests run in CI on ubuntu-latest, with fixtures recorded live on the founder's host and scrubbed like every fixture.
@task_item n=9 part=needs
The live-measured record of the compositor: the Hyprland version read with each recording and the replies kept per version, parallel to and separate from the Claude capability ledger.
@task_item n=10 part=needs
The live-desktop steps as named operator steps: read-only compositor queries (L1), a dispatch that focuses or moves a window or shows a workspace (L2) with the lock state read first and a stop on locked, never unlocking, and a window-opening dispatch (L3) only if needed; no typed key and no live claude start.
@task_item n=11 part=needs
The transport to the compositor: either the hyprctl program with JSON output, spawned directly with no shell and an absolute path as viola run resolves its child, or the compositor's request socket under the runtime directory with owner and mode checks like the channel's socket directory.
@task_item n=12 part=needs
A new environment read of the compositor instance signature, which the binary does not make today, as a value the seam may need to reach the compositor.
@task_item n=13 part=needs
Tests and guard controls that spawn a compositor client, viola run, mcp or a wrapper each carry a kill deadline; a guard's one-off control runs under the gate's control step, with no mutation run.
@task_item n=14 part=needs
Platform reach: Linux alone, the window features Hyprland only, CI on ubuntu-latest.
@task_standing n=1
convention: each new log line carries a closed detail code only, never a path, pid, socket name or upstream text
home: surface
@task_standing n=2
convention: each new error or failure the code adds names no internals, prints no stack trace and quotes no upstream text
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
Brand identity rows on strips, the readback box, send readbacks, dialog-pending, budget pause and handoff display: the chunk renders no session strip or readback.
@task_item n=2 part=not_needed
Colour palette, typography, spacing and iconography rows (accent, bold, dim, glyphs, separators, columns): the seam prints nothing to a terminal.
@task_item n=3 part=not_needed
The web-page surface absence and the rendered-page absence: no page of any kind is involved.
@task_item n=4 part=not_needed
The verb output and refusal contracts of existing verbs (list board, revive, run start refusals, send mirror, wait, verify, link and unlink, release budget, last): no existing verb's output changes.
@task_item n=5 part=not_needed
CLI exit codes, exit-21 causes, revive --json refusal codes and the help verb groups: no verb is built, so no exit code or help entry is added.
@task_item n=6 part=not_needed
The command name register: open, focus, up, adopt and the console are their own later entries and no verb name is added here.
@task_item n=7 part=not_needed
The CLI colour decision order and colour depth rows: the seam emits no styled output.
@task_item n=8 part=not_needed
The self-validation protocol's design tests (squint, swap, readback box signature): they judge rendered output this chunk does not produce.
