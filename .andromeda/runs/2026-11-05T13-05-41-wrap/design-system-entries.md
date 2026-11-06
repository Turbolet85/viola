@amendment marker=2026-11-05-linux-only-code-and-ci
title: Typography on the Linux-only target
section: §Typography (body: the Bahnschrift item, the Windows-only faces item, the role-stack sentence; typography detail: the per-OS table head)
change:
  - The Bahnschrift item reads «It ships with Windows 10 1709+.»; was «…, and Windows is the first target».
  - The faces item reads «Both faces are Windows-only and viola targets Linux only», the Linux fallback «the target's own rendering, not a degraded one»; was «D3 makes macOS and Linux supported», the Linux fallback «a first-class rendering».
  - The role-stack sentence reads «Each role token's stack ends at `sans-serif` / `monospace`»; was «names a face per OS (Windows, macOS, Linux) and ends at …».
  - The per-OS table head reads `Windows` · `macOS` · `Linux (the target)`; was `Windows (live target)` · `macOS (CI-tested)` · `Linux (CI-tested)`.
why: the OS target «Windows first, macOS and Linux» is retired for Linux only, the Windows and macOS code removed and recoverable at the tag `multi-os-final` — the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6).
kept: each role keeps its per-OS fallback stack and the table its Windows and macOS columns; the stacks still end at the generic `sans-serif` / `monospace`, never `system-ui`, and the Linux render (DejaVu Sans Mono) stays the widest case the strip and tape columns hold.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the cli Platform narrowed to Linux terminals
section: §Surface: cli › Platform (body)
change:
  - The Platform line reads «Linux terminals (the core on any current distribution, CI on `ubuntu-latest`)»; was «Windows 10/11 (Windows Terminal, conhost), macOS and Linux terminals».
why: the OS target is Linux only, the core on any current distribution with CI on `ubuntu-latest` — the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the Windows VT enable retired from the colour rules
section: §Surface: cli › Toolkit / Framework, Tokens (platform-specific), Platform-Specific Notes (body)
change:
  - The Toolkit bullet on Windows VT enabling (windows-sys 0.61.2 `SetConsoleMode(ENABLE_VIRTUAL_TERMINAL_PROCESSING)`, no colour if it fails) is removed.
  - The colour decision order loses rung 7 «Windows VT enable failed».
  - The depth rule's 256-colour line reads «256-colour if `TERM` contains `256color`;»; was «…, or on Windows once VT enabling has succeeded;».
  - The Platform-Specific Notes lose the «conhost without VT falls back to no colour» note.
why: `src/human/style.rs` lost its VT enable (the `vt` fact, the `vt_enable_failed` row, the Windows depth rows) and the `windows-sys` dependency is removed; the Windows VT enable, rung 7 and the Windows depth branch are retired with the Windows target — the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the .cmd/.bat refusal and batch-script-child retired
section: §Surface: cli › Component Patterns (body: the revive line; surface detail: the refusal hints, the `revive --json` documents item, exit-code row 1)
change:
  - `revive`'s closed `--json` refusal codes read ten, in the body's revive line and the `revive --json` item: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`; was eleven, with `batch-script-child`.
  - `viola run`'s exit-1 start refusals lose `unable: <name>'s command is a .cmd or .bat script` (Windows) and its hint `pass the real executable, not a .cmd or .bat shim`.
  - The restated obs-plan D-20 log detail codes lose `batch-script-child`.
  - Exit-code row 1 loses `run`'s `<name>'s command is a .cmd or .bat script` and `verify`'s `the claude CLI is a .cmd or .bat script`.
why: `AgentError::BatchScriptChild` and `run`'s and `verify`'s `BatchScriptChild` refusals are removed and the `.cmd`/`.bat` refusal is retired with the Windows target — the founder's own rulings of 2026-11-04 (inputs#I1, inputs#I5 item 6); the closed word `batch-script-child` left the schema with its code by the overseer's technical answer of 2026-11-05 (inputs#I2). revive's codes ten and verify's five are pinned in `tests/cross_exit_causes.rs`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the cli width read by TIOCGWINSZ alone
section: §Surface: cli › Tokens (platform-specific) (surface detail: the width item)
change:
  - The width is read by `viola_pty::host_size()` (a `TIOCGWINSZ` ioctl); was «Windows `GetConsoleScreenBufferInfo`, a `TIOCGWINSZ` ioctl elsewhere».
why: `host_size()`'s `GetConsoleScreenBufferInfo` arm is removed, its `TIOCGWINSZ` arm unconditional.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: strict-modes readings without an OS arm
section: §Surface: cli › Component Patterns (surface detail: the board's strict-modes scan, the home under a regular file)
change:
  - The board's strict-modes scan «holds the home to the read rule»; was «… on every OS».
  - A home under a regular file is a strict-modes refusal (`verify` the home-state pair, exit 1; the frame verbs the instance-state pair, exit 21) with no OS lead; was «On Unix …», with the Windows `NotFound` reading (passed as absent, the catch-site line kept) removed.
why: `board.rs`'s OS arms are removed (`board_scan`'s read rule stays) and `strict.rs`'s Windows owner/DACL arm is removed, retired with the Windows target; an «on every OS» clause is stale wording on a Linux-only product.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the stale-row colour witness and where it now stands
section: §Surface: cli › Component Patterns (surface detail: the stale row's colour layer)
change:
  - The witness `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`, ungated) runs on `ubuntu-latest`, the one CI runner, passing as measured at ci#42030025050; was on the Unix runners `ubuntu-latest` and `macos-latest`, passing on both at ci#41918694243.
why: the CI matrix is `[ubuntu-latest]` alone, the `macos-latest` leg retired; the test's gate was made unconditional, its only sibling gone; ci#42030025050 on `d3690534` is green for `test`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the server check without the macOS no-peer-pid carve-out
section: §Surface: cli › Component Patterns (surface detail: the refusal hints, the macOS server-check item, exit 21's server-verification cause)
change:
  - The refusal hints lose «On macOS `server-verify-failed` means an euid mismatch alone: no peer pid is compared, a named gap no route entry owns.»
  - The item on macOS's server check (no peer pid compared, the euid alone) is removed.
  - Exit 21's server-verification cause reads «also a peer whose euid is not this user's or cannot be read»; was «on Linux and macOS also …, on macOS the euid alone».
why: `server_verdict` / `verify_server` lose the macOS `Matches` arm and `SERVER_PID_REPORTED` / `PEER_EUID_REPORTED` are removed (`peer_euid_admitted` is `observed == Some(own)`, an unreadable euid still refused); the macOS no-peer-pid carve-out is retired with the macOS target.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: verify's relied count on every build
section: §Surface: cli › Component Patterns (surface detail: the `verify` step counter)
change:
  - `LedgerRow::relied()` reads 24 on every build; was 24 on Unix, 21 on Windows, the three statusline rows not relied there.
  - The statusline rows and the resume, agents and compact rows are relied with no OS qualifier; the summary sample `stamped 2.1.287  24 pass  0 fail` drops «on Unix».
  - New: the `shim-resolution` row passes when `claude --version` answers.
why: `WRAPPED_HERE` is folded (`relied()` all twenty-four rows on every build) and the Windows relied set of twenty-one is retired; `ProbeRun.program_is_script` left with `is_script`, so the row reads `run.version_answered` alone, its master wording left to this wrap.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
