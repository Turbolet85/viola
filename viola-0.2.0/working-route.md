# Working Route — viola-0.2.0

### Epoch 1 — Foundation

Epoch 8 cleanup — shared source-masking scanner, over-800-line files split, dead presence check, named mutant survivors, home and guard test helpers, the cookie class
Hyprland seam — relied-on dispatch and window queries measured live; replies read capped as untrusted output, logged codes-only; replayed by a fake in CI to green

### Epoch 2 — The budget gate and the status line leave

Budget gate removed — no rate-limits reading, `budget.json`, send gate, `budget-paused`, budget board or log fields, Path 6 suite; driver `release` guard kept; exit 11 retired
Status-line interception removed — user's command untouched; no `hook statusline` arm, shell-out, settings override, snapshot command, fake-agent modes, fixtures or log subjects; 21 relied rows
Live round on the trimmed product — v1-27's wrapped start proved; a dialog hold near its 3600 s ceiling read, the PROVISIONAL mark settled

### Epoch 3 — Driver reliability: turns and sends

API-error turn end — logged as a turn end with a closed cause code, never the error text; the instance idle again, the next send accepted
`wait` wake condition — documented: a turn-end question held by a background task wakes it; a task-free end's notification gives no false wake
Send confirmation — the driver's send matched whatever whitespace the CLI trims, validated as received, never stripped; the readiness-gate turn race closed
Product-owned settings and local-command delivery — viola's owner-only settings for its sessions, rewritten each start, in no log line; a local command's echo confirms its send
Survey key keeps the wheel — the session-feedback survey's grammar measured live, joining the closed non-editing list

### Epoch 4 — Driver reliability: ledger rows and bounds

PermissionRequest question answered — `allow` with `updatedInput` carrying the answers; a new ledger row, its verify probe, CLI versions re-stamped
Harness-prefix and identity-floor ledger rows — a second session sending into a verify child; env names, never values, captured; stamped live
MCP calls bounded — the client's tool-call timeout measured; every call, `send`'s longest block included, completes below it
Key after a resize — whether real `claude` on Linux loses a key typed right after a resize, measured; fixed if it does

### Epoch 5 — The fleet: lineage, launch and windows

Lineage registry — `lineage` event at launch; fleet trees on `list` in its fixed columns, plain when piped; parent kept by `revive`; links drawn apart, name-checked
`viola open` — a terminal window per session beside its parent; closed flag allowlist, no shell; window identity and origin in its own state, logged codes-only
`viola focus` — the session's window raised and its workspace shown through Hyprland, by recorded identity or ancestry; a named refusal otherwise
`viola up` — the fleet re-opened after a reboot, each resumed in place in its workspace, parents first, pid-and-start-verified live ones skipped, logged per instance
`viola adopt` — a plain `claude` session, by session-shaped id, closed and resumed under an operator-given name and parent; the live-elsewhere refusal guards it

### Epoch 6 — The fleet console

Console scaffold — its surface section, quarter-screen layout and redraw-ban reconciliation; a keystroke test driver reading state, never the screen; its own diagnostics role
Fleet console view — a tiled Hyprland window whole at quarter-screen: lineage trees, each session's state in the board's words, wheel holder, last activity; rows control-escaped
Console actions — every verb by keyboard alone, no key trap, through the CLI verbs; a send shows its readback; refusals as `unable` lines

### Epoch 7 — Polish & ship

The fleet on the founder's host — started, restored after a reboot and driven from one console by viola; `vstart.sh`, `up.sh`, `pwait.sh` retired
Fleet a11y verification — console keyboard-only reach, no trap, states by word, no spinner; fleet starts keep the TUI boundary; SC 2.1.1, 2.1.2, 1.4.1, 2.4.7, 3.2.4
Version done-check — every gate green on Linux, zero unlogged panics across the fleet verbs and console; relied rows stamped; capabilities verified or deferred
