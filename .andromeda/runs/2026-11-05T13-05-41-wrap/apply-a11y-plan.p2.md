@apply_row n=1 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=3 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=4 v=applied edit=E3
@apply_row n=4 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=5 v=applied edit=E4
@apply_row n=5 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=6 v=applied edit=E5
@apply_row n=6 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=7 v=applied edit=E6
@apply_row n=7 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=8 v=applied edit=E7
@apply_row n=8 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=9 v=refused
why: a hand removal · fct-4etq6dn5
@apply_row n=9 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=11 v=applied edit=E8
@apply_row n=10 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=12 v=applied edit=E9
@apply_row n=11 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=13 v=applied edit=E10
@apply_row n=12 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=14 v=applied edit=E11
@apply_row n=13 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=15 v=applied edit=E12
@apply_row n=14 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=17 v=applied edit=E13
@apply_row n=15 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=19 v=applied edit=E14
@apply_row n=16 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=21 v=applied edit=E15
@apply_row n=17 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=22 v=applied edit=E16
@fact id=fct-k76k2lf7 expect_rev=1
old: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
new: `stale-heartbeat` · `pinned-hash-mismatch`
old: Error recovery); on macOS no peer pid is read (the recorded gap, a named carve-out no route entry owns), so there the refusal means an euid mismatch alone; their `error` word
new: Error recovery); their `error` word
old: `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`
new: `pinned-hash-mismatch`, `squatted-name`

@fact id=fct-dssvg3hs expect_rev=1
old: the mode or ACL is left as found, on all three OSes (the refusal's text
new: the mode is left as found (the refusal's text

@fact id=fct-sjhco52d expect_rev=1
old: another owner), on Unix, in the human TTY
new: another owner), in the human TTY
old: the directory is left as found; off Unix the check reads nothing (the refusal's text
new: the directory is left as found (the refusal's text

@fact id=fct-hj6bq3jt expect_rev=1
old: home-state pair; on Unix a home under a regular file is this refusal, on Windows (such a path stats `NotFound`) the `cli` catch-site fault line stands (the refusals' text
new: home-state pair; a home under a regular file is this refusal (the refusals' text

@fact id=fct-rts3hgep expect_rev=1
old: fails its check, on Unix: each refuses
new: fails its check: each refuses

@fact id=fct-itb2wxsm expect_rev=1
old: TUI hosted in ConPTY/openpty, entered by
new: TUI hosted in openpty, entered by
old: every other byte does. On Windows the platform decides what arrives (F-W3): the ConPTY host swallows focus reports, and under win32-input-mode an injected mouse report reaches viola as typed keys and takes the wheel.
new: every other byte does.

@fact id=fct-yscgzykm expect_rev=1
old: reaches the outer terminal, on all three OSes; on Unix the child's own bytes
new: reaches the outer terminal, and the child's own bytes

@fact id=fct-walt4vws expect_rev=1
old: (`send`, `wait`, `last`, `answer`), on every OS (on macOS no peer pid is read, the named carve-out of the CLI entity above, so there such a server is one of another euid alone)
new: (`send`, `wait`, `last`, `answer`)

@fact id=fct-vtox22ts expect_rev=1
old: - NVDA with Windows Terminal/conhost (live target).
new: - A founder-owned pass; nothing is gated on it.
old: - VoiceOver with Terminal.app.
new: - On Linux alone (the Platform line, below).
old: The CLI a11y checks run on all three OSes, so `os` is tagged per matrix leg.
new: The CLI a11y checks run on Linux, so `os` is `linux`.
old: This must hold on `[windows-2025, macos-latest, ubuntu-latest]` (tests excerpt
new: This must hold on `[ubuntu-latest]` (tests excerpt

@fact id=fct-twqx5x46 expect_rev=1
old: **Manual verification (supplemental):** NVDA with Windows Terminal (Windows is the live target), confirming
new: **Manual verification (supplemental):** Orca with foot on Hyprland (the founder's host's one terminal, the `xdg-terminal-exec` default), a founder-owned pass that gates nothing, confirming

@fact after=fct-twqx5x46 as=n1
- **Platform (every surface):** Linux only — the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland; the Windows and macOS code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).

@fact id=fct-dx4kie52 expect_rev=1
old: three portable-pty outer-PTY nextest cases on all three OS legs, one per
new: three portable-pty outer-PTY nextest cases on `ubuntu-latest`, one per
old: zero viola-originated bytes on the outer-PTY stream (byte for byte against an unwrapped fake-agent run on Linux/macOS; viola's own literals absent on Windows ConPTY);
new: zero viola-originated bytes on the outer-PTY stream, byte for byte against an unwrapped fake-agent run;
old: (exit 13 driver, 10 human); the full assertion on Linux and macOS, the platform fact on `windows-2025` (F-W3: focus reports swallowed, the injected mouse report takes the wheel), Section 3
new: (exit 13 driver, 10 human), Section 3

@fact id=fct-uok334yy expect_rev=2
old: asserts the TTY-only behaviours, on all three OS legs:
new: asserts the TTY-only behaviours, on `ubuntu-latest`:
old: (bold NAME, amber only on `DIALOG`; on windows-2025 the parsed attribute set)
new: (bold NAME, amber only on `DIALOG`)
old: and under the outer PTY on ubuntu-latest and macos-latest only, not on windows-2025 (the stale-row witness, below)
new: and under the outer PTY (the stale-row witness, below)
old: no ESC byte and no lone CR on Unix (`tests/support/sgr.rs`
new: no ESC byte and no lone CR (`tests/support/sgr.rs`
old: checked with assert_cmd on all three OS legs:
new: checked with assert_cmd on `ubuntu-latest`:
old: the no-ESC assertion Unix-only, and on windows-2025 each escaped name read as text under a 200-column terminal, because ConPTY re-renders escapes)
new: the no-ESC assertion in each mode)

@fact id=fct-jgpx3odq expect_rev=1
old: (cli, Unix legs)
new: (cli)
old: (`tests/cross_list_rows.rs`, `cfg(unix)`, on a booted wrapper)
new: (`tests/cross_list_rows.rs`, ungated, on a booted wrapper)
old: It runs on ubuntu-latest and macos-latest and not on windows-2025.
new: It runs on ubuntu-latest.

@fact id=fct-es7le4as expect_rev=2
old: picks a truecolor / 256 / 16 depth (on Windows after enabling VT processing on stdout, restored afterwards; a failed enable writes none).
new: picks a truecolor / 256 / 16 depth.
old: too_narrow_for_it` and, on ubuntu-latest and macos-latest only (`cfg(unix)`, not run on windows-2025), `cross_list_rows_dim_a_stale_row
new: too_narrow_for_it` and `cross_list_rows_dim_a_stale_row
old: carry no ESC byte. Under `viola run` on Windows the outer-PTY stream also carries the ConPTY host's own SGR, cursor and query bytes (not viola's: the inbox host's as measured at chunk 2026-09-25-pty-wrapper-on-windows research fact 4, the sideloaded `OpenConsole.exe`'s `ESC[1t ESC[c ESC[?1004h ESC[?9001h` as measured at chunk 2026-09-29-sideloaded-conpty), so the windows-2025 `viola run` check is that viola's literals are absent, on both backends. `unable`
new: carry no ESC byte. `unable`
old: Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`, the stale-row dim witness on the two Unix legs only.
new: Checked with assert_cmd + portable-pty on `[ubuntu-latest]`.

@fact id=fct-46bk3wap expect_rev=1
old: asserts it on the two Unix legs.
new: asserts it on ubuntu-latest.

@pattern id=a11y2-01
fixed: `stale-heartbeat` · `batch-script-child`
@pattern id=a11y2-02 ignore_case=true
fixed: no peer pid is read
@pattern id=a11y2-03
fixed: the mode or ACL is left as found
@pattern id=a11y2-04
fixed: off Unix the check reads nothing
@pattern id=a11y2-05
fixed: such a path stats `NotFound`
@pattern id=a11y2-06
fixed: hosted in ConPTY/openpty
@pattern id=a11y2-07
fixed: the ConPTY host swallows focus reports
@pattern id=a11y2-08
regex: on all three OS(es| legs)
@pattern id=a11y2-09
fixed: The CLI a11y checks run on all three OSes
@pattern id=a11y2-10
fixed: NVDA with Windows Terminal
@pattern id=a11y2-11
fixed: viola's own literals absent on Windows ConPTY
@pattern id=a11y2-12
fixed: the platform fact on `windows-2025`
@pattern id=a11y2-13
fixed: because ConPTY re-renders escapes
@pattern id=a11y2-14
fixed: and macos-latest and not on windows-2025
@pattern id=a11y2-15
fixed: after enabling VT processing on stdout
@pattern id=a11y2-16
fixed: the ConPTY host's own SGR, cursor and query bytes
@pattern id=a11y2-17
fixed: on the two Unix legs
