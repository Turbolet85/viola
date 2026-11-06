@apply_row n=1 ref_file=fanout-a11y-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-a11y-plan.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-a11y-plan.md ref_n=5 v=applied edit=E5
@apply_row n=6 ref_file=fanout-a11y-plan.md ref_n=6 v=refused
why: its place, the §3 key `ci-integration` (fct-56elcbkj), is a contract key file the body holds only a pointer to (fct-gmsx3s7a); not among this pass's target addresses, so no read of it stands to edit against
@apply_row n=7 ref_file=fanout-a11y-plan.md ref_n=7 v=refused
why: its place, the §3 key `ci-integration` (fct-gcqo3m4v), is a contract key file the body holds only a pointer to (fct-gmsx3s7a); not among this pass's target addresses, so no read of it stands to edit against
@apply_row n=8 ref_file=fanout-a11y-plan.md ref_n=8 v=refused
why: its place, the §3 key `a11y-testing-tool-pick` (fct-dlpkde76), is a contract key file the body holds only a pointer to (fct-gmsx3s7a); not among this pass's target addresses, so no read of it stands to edit against
@apply_row n=9 ref_file=fanout-a11y-plan.md ref_n=9 v=refused
why: its place, the §3 key `bootstrap-phases-derive-for-route-setup-project` (fct-yftct3hq), is a contract key file the body holds only a pointer to (fct-gmsx3s7a); not among this pass's target addresses, so no read of it stands to edit against
@apply_row n=10 ref_file=fanout-a11y-plan.md ref_n=10 v=applied edit=E6
@apply_row n=11 ref_file=fanout-a11y-plan.md ref_n=11 v=applied edit=E7
@apply_row n=12 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=16 v=folded_into to_file=fanout-a11y-plan.md to_n=11
@apply_row n=13 ref_file=fanout-a11y-plan.md ref_n=12 v=applied edit=E8
@apply_row n=14 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=18 v=folded_into to_file=fanout-a11y-plan.md to_n=12
@apply_row n=15 ref_file=fanout-a11y-plan.md ref_n=13 v=applied edit=E9
@apply_row n=16 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=20 v=folded_into to_file=fanout-a11y-plan.md to_n=13
@apply_row n=17 ref_file=fanout-a11y-plan.md ref_n=14 v=applied edit=E10
@apply_row n=18 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=7 v=folded_into to_file=fanout-a11y-plan.md to_n=14
@apply_row n=19 ref_file=fanout-a11y-plan.md ref_n=15 v=applied edit=E11
@apply_row n=20 ref_file=fanout-a11y-plan.md ref_n=16 v=applied edit=E12
@apply_row n=21 ref_file=fanout-a11y-plan.md ref_n=17 v=applied edit=E13
@apply_row n=22 ref_file=fanout-a11y-plan.md ref_n=18 v=applied edit=E14
@apply_row n=23 ref_file=fanout-a11y-plan.md ref_n=19 v=applied edit=E15
@apply_row n=24 ref_file=fanout-a11y-plan.md ref_n=20 v=refused
why: its place, the §3 key `screen-reader-test-pattern` (fct-tocuc2rh), is a contract key file the body holds only a pointer to (fct-gmsx3s7a); not among this pass's target addresses, so no read of it stands to edit against
@apply_row n=25 ref_file=fanout-a11y-plan.md ref_n=21 v=refused
why: its place, the §3 key `bootstrap-phases-derive-for-route-setup-project` (fct-t3p7hz2f), is a contract key file the body holds only a pointer to (fct-gmsx3s7a); not among this pass's target addresses, so no read of it stands to edit against
@apply_row n=26 ref_file=fanout-a11y-plan.md ref_n=22 v=applied edit=E16
@apply_row n=27 ref_file=fanout-a11y-plan.md ref_n=23 v=applied edit=E17
@apply_row n=28 ref_file=fanout-a11y-plan.md ref_n=24 v=refused
why: its place, the §3 key `keyboard-test-harness` (fct-avr5o2ub), is a contract key file the body holds only a pointer to (fct-gmsx3s7a); not among this pass's target addresses, so no read of it stands to edit against
@apply_row n=29 ref_file=fanout-a11y-plan.md ref_n=25 v=applied edit=E18
@apply_row n=30 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=2 v=folded_into to_file=fanout-a11y-plan.md to_n=25
@apply_row n=31 ref_file=fanout-a11y-plan.md ref_n=26 v=applied edit=E19
@apply_row n=32 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=10 v=folded_into to_file=fanout-a11y-plan.md to_n=26

@fact id=fct-by4mf7d7 expect_rev=1
old: matrix `[windows-2025, macos-latest, ubuntu-latest]`
new: matrix `[ubuntu-latest]`

@fact id=fct-s4vibfo3 expect_rev=1
old: | Unit / integration (all three OS legs) |
new: | Unit / integration (`ubuntu-latest`) |
old: | the per-OS `test` job's `gate
new: | the `test` job's `gate

@fact id=fct-pkoabs22 expect_rev=1
old: run inside the per-OS `test` job's single instrumented `coverage` suite on the Windows, macOS and ubuntu legs; each leg's `gate --require coverage,doctest` is its own verdict.
new: run inside the `test` job's single instrumented `coverage` suite on the `ubuntu-latest` leg, whose `gate --require coverage,doctest` is the verdict.

@fact id=fct-k6kozbbz expect_rev=1
old: pass on all three OS legs inside
new: pass on the `ubuntu-latest` CI leg inside

@fact id=fct-ktqfqjb4 expect_rev=1
old: `scripts/agent-run.sh run` (or `.ps1`).
new: `scripts/agent-run.sh run`.
old: **Gate:** the per-OS `test` job's `gate --require coverage,doctest` never allows skips.
new: **Gate:** the `test` job's `gate --require coverage,doctest` on `ubuntu-latest` never allows skips.
old: in CI inside the per-OS `test` job's single instrumented `coverage` suite, on all three OS legs.
new: in CI inside the `test` job's single instrumented `coverage` suite, on the `ubuntu-latest` leg.

@fact id=fct-yzqlnwsx expect_rev=1
old: outer-PTY SGR checks; all three OS legs | NVDA + Windows Terminal/conhost; VoiceOver + Terminal.app; Orca + GNOME Terminal |
new: outer-PTY SGR checks; the `ubuntu-latest` CI leg | Orca + GNOME Terminal |

@fact id=fct-jkkyxmll expect_rev=1
old: outer-PTY nextest cases on all three OS legs —
new: outer-PTY nextest cases on `ubuntu-latest` —
old: not moving the wheel on Linux and macOS, on `windows-2025` the platform fact that an injected mouse report takes the wheel.
new: not moving the wheel.

@fact id=fct-ijnl2u6y expect_rev=1
old: TTY-only behaviours, on all three OS legs —
new: TTY-only behaviours, on `ubuntu-latest` —

@fact id=fct-lchghj7u expect_rev=2
old: **Unix legs only:** dim only
new: **Stale-row dim:** dim only
old: on ubuntu-latest and macos-latest; the outer-PTY witness does not run on windows-2025.
new: on ubuntu-latest.

@fact id=fct-2jdilj7v expect_rev=1
old: on all three OS legs (the frame verbs' server-verify peer pid witness on windows-2025 and ubuntu-latest only: macOS reports no peer pid, the word there an euid mismatch alone; the socket-directory refusal on ubuntu-latest and macos-latest).
new: on ubuntu-latest (the frame verbs' server-verify peer pid witness and the socket-directory refusal among them).

@fact id=fct-orpg5jsb expect_rev=1
old: NVDA (Windows Terminal), VoiceOver (macOS) and Orca (Linux) on Section 4
new: Orca (Linux) on Section 4

@fact id=fct-ol6ygl55 expect_rev=1
old: a manual NVDA / VoiceOver / Orca pass
new: a manual Orca pass

@fact id=fct-4ktfiloi expect_rev=1
old: keystrokes unblocked | NVDA + Windows Terminal |
new: keystrokes unblocked | Orca + foot on Hyprland |

@fact id=fct-2kjabyk2 expect_rev=1
old: | cli (Windows / macOS / Linux) | NVDA + Windows Terminal/conhost; VoiceOver + Terminal.app; Orca + GNOME Terminal |
new: | cli (Linux) | Orca + GNOME Terminal |

@fact id=fct-wawttggc expect_rev=1
old: | tui (Windows) | NVDA + Windows Terminal |
new: | tui (Linux) | Orca + foot on Hyprland |

@fact id=fct-hu3qpzu6 expect_rev=1
old: ignored for wheel purposes (on Windows an injected mouse report under win32-input-mode arrives as typed keys and takes the wheel, F-W3).
new: ignored for wheel purposes.

@fact id=fct-csjr2vg7 expect_rev=1
old: as editing (P4) — a mouse report the Windows ConPTY already turned into win32 key-down records is typing (F-W3).
new: as editing (P4).

@fact id=fct-z2htpz2e expect_rev=1
old: `batch-script-child`, `pinned-hash-mismatch`
new: `pinned-hash-mismatch`

@fact id=fct-rdegquaa expect_rev=1
old: (no viola literal on the outer terminal on every OS, the child's own bytes on Unix), its Windows byte reading owed to "Windows-only live measurements".
new: (no viola literal on the outer terminal, the child's own bytes reaching it).

@pattern id=a11y1-01
fixed: matrix `[windows-2025, macos-latest, ubuntu-latest]`
@pattern id=a11y1-02 ignore_case=true
fixed: on all three OS legs
@pattern id=a11y1-03
fixed: the per-OS `test` job
@pattern id=a11y1-04
fixed: on the Windows, macOS and ubuntu legs
@pattern id=a11y1-05
fixed: `scripts/agent-run.sh run` (or `.ps1`)
@pattern id=a11y1-06
fixed: NVDA + Windows Terminal
@pattern id=a11y1-07
fixed: VoiceOver + Terminal.app
@pattern id=a11y1-08
fixed: on `windows-2025` the platform fact that an injected mouse report takes the wheel
@pattern id=a11y1-09
fixed: the outer-PTY witness does not run on windows-2025
@pattern id=a11y1-10
fixed: macOS reports no peer pid
@pattern id=a11y1-11
fixed: a manual NVDA / VoiceOver / Orca pass
@pattern id=a11y1-12
fixed: NVDA (Windows Terminal), VoiceOver (macOS)
@pattern id=a11y1-13
fixed: under win32-input-mode arrives as typed keys and takes the wheel
@pattern id=a11y1-14
fixed: already turned into win32 key-down records is typing
@pattern id=a11y1-15
fixed: `batch-script-child`, `pinned-hash-mismatch`
@pattern id=a11y1-16
fixed: byte reading owed to "Windows-only live measurements"
