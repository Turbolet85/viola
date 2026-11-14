# Working Route — viola-0.2.0

### Epoch 1 — Foundation

[2026-11-13-verify-readiness-on-the-input-box] Verify readiness on the input box — verify's typed run pastes once the input box is up, never keyed on a footer literal; the newest CLI reads every row pass (v2-29)
[2026-11-13-epoch-8-cleanup] Epoch 8 cleanup — shared source-masking scanner, over-800-line files split, dead presence check, named mutant survivors, home and guard test helpers, the cookie class
[2026-11-13-rows-proposal-rule-work] Rows-proposal rule work — the rows landing's route-owed rule lines and the snapshot-replay guard, each closed in code or tests
[2026-11-14-hyprland-seam] Hyprland seam — relied-on dispatch and window queries measured live; replies read capped as untrusted output, logged codes-only; replayed by a fake in CI to green

### Epoch 2 — The budget gate and the status line leave

Hyprland exact-frame bound pinned — exactly `MAX_FRAME` bytes accepted, one more `oversize`, on the bounded read and the dispatch parse; the next audit catches both survivors  CARRY: the Epoch 1 audit's two survivors (run 2026-11-14T11-39-19-code-audit at 25348549, `viola-hyprland` 96.00 %, 48 caught, 2 missed), each `>` → `>=` on the `MAX_FRAME` bound: `crates/viola-hyprland/src/read.rs:77` in `read_within` and `crates/viola-hyprland/src/reply.rs:151` in `parse_dispatch`; one missing case, pinned by tests on both sites (the read through the fake `hyprctl`, `parse_dispatch`), `clients` already pinned by `parse_clients_takes_max_frame_bytes_and_refuses_one_more_as_oversize`; no mutation run inside the chunk, test-only, no requirement added, no capability claimed; the declined `viola-e2e` and `viola` mutation units stay out (they wait on a pipeline mend); placed first on the founder's word «Корректирующий чанк первым (Recommended)», relayed verbatim by overseer1 via the Viola overseer, 2026-11-14, PROVISIONAL — relay: .andromeda/runs/2026-11-14T12-51-41-wrap/inputs/I1-e1-0.2.0-route-adaptation.md.txt
Budget gate removed — no rate-limits reading, `budget.json`, send gate, `budget-paused`, budget board or log fields, Path 6 suite; driver `release` guard kept; exit 11 retired
Status-line interception removed — user's command untouched; no `hook statusline` arm, shell-out, settings override, snapshot command, fake-agent modes, fixtures or log subjects; 21 relied rows  CARRY: architecture · decision/no-tmux-or-shell-dependency — the status-line shell-out leaves (crates/viola-agent-claude/src/statusline.rs:124), census F-09 (placement provisional — the founder away; rows landing 2026-11-13T01-15-10's route-owed.md)  CARRY: a box-only screen fixture (the input box's rule and prompt rows with no footer literal, recorded by a clean `viola verify --record`) for the fake agent to replay in CI — none can be recorded while `statusline-rate-limits` fails on this account (`--record` writes only at 0 fail), and none is hand-written; once this entry retires the rate-limits row a clean `--record` can carry one (from chunk 2026-11-13-verify-readiness-on-the-input-box; its P5 card's «Beyond the plan» row, `.andromeda/runs/2026-11-13T01-42-03-phase/p5-card-1.md`, approved by the operator's own word «yes», `chunks/2026-11-13-verify-readiness-on-the-input-box/inputs/I6-relay-1.md.txt`)
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
Model and effort at launch — every session starts with model and effort named, viola's default otherwise; the pair recorded per instance, replayed on resume (v2-30)
`viola open` — a terminal window per session beside its parent; closed flag allowlist, no shell; window identity and origin in its own state, logged codes-only  CARRY: the Hyprland seam's `exec` window-placement dispatch and the `monitors` query, left out of the relied set by 2026-11-14-hyprland-seam (its P5 card's Beyond-the-plan row, approved «yes» by the operator: chunks/2026-11-14-hyprland-seam/inputs/I4-relay-2.md.txt); each a new `viola_hyprland::Request`, its argv measured live (0.56.2 runs `dispatch` as Lua `hl.dsp.*`), recorded by viola-hypr-tap and replayed by the fake.  CARRY: settle the PROVISIONAL `HYPRCTL_DEADLINE` (5 s, security-plan decision/hyprctl-read-deadline) against the need «viola open», the seam's first product caller, measures (2026-11-14-hyprland-seam's P5 card Beyond-the-plan row, approved: chunks/2026-11-14-hyprland-seam/inputs/I4-relay-2.md.txt).
`viola focus` — the session's window raised and its workspace shown through Hyprland, by recorded identity or ancestry; a named refusal otherwise
`viola up` — the fleet re-opened after a reboot, each resumed in place in its workspace, parents first, pid-and-start-verified live ones skipped, logged per instance
`viola adopt` — a plain `claude` session, by session-shaped id, closed and resumed under an operator-given name and parent; the live-elsewhere refusal guards it
Switching a session's model and effort — a verb with no side effect on the user's settings; the console reaches it through «Console actions» (v2-31)

### Epoch 6 — The fleet console

Console scaffold — its surface section, quarter-screen layout and redraw-ban reconciliation; a keystroke test driver reading state, never the screen; its own diagnostics role
Fleet console view — a tiled Hyprland window whole at quarter-screen: lineage trees, each session's state in the board's words, wheel holder, last activity; rows control-escaped  CARRY: layout-templates · decision/shared-captions-fixed — the planned fleet console (0.2.0, Epoch 6) reuses the six board captions, their words and slot order (src/human/board/rows.rs:42) (placement provisional — the founder away; rows landing 2026-11-13T01-15-10's route-owed.md)
Console actions — every verb by keyboard alone, no key trap, through the CLI verbs; a send shows its readback; refusals as `unable` lines

### Epoch 7 — Polish & ship

The fleet on the founder's host — started, restored after a reboot and driven from one console by viola; `vstart.sh`, `up.sh`, `pwait.sh` retired
Fleet a11y verification — console keyboard-only reach, no trap, states by word, no spinner; fleet starts keep the TUI boundary; SC 2.1.1, 2.1.2, 1.4.1, 2.4.7, 3.2.4
Version done-check — every gate green on Linux, zero unlogged panics across the fleet verbs and console; relied rows stamped; capabilities verified or deferred
