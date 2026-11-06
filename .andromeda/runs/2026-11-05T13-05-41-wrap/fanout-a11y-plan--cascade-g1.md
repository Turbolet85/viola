@proposal n=1 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-k76k2lf7 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: drop `batch-script-child` from revive's `<code>` list (ten codes: strict-modes-failed · already-live · no-session · session-live · cwd-missing · stale-heartbeat · pinned-hash-mismatch · squatted-name · socket-dir-refused · internal-error) and from the «start's refusals under `viola revive --json`» item (`pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`); drop the clause «on macOS no peer pid is read (the recorded gap, a named carve-out no route entry owns), so there the refusal means an euid mismatch alone;»
sidecar: a11y-scope-summary#1.1 — batch-script-child left revive's start refusals (codes 11 → 10) and the macOS no-peer-pid carve-out retired, Linux only (founder ruling 2026-11-04)
rationale: Retired names the .cmd/.bat refusal batch-script-child and the macOS no-peer-pid carve-out; Counts moves revive's closed --json refusal codes 11 → 10; Symbols folds SERVER_PID_REPORTED / PEER_EUID_REPORTED
@proposal n=2 detector=D-retired severity=warning file=body anchor=fct-z2htpz2e basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: ## 1. A11y Scope Summary
change: «`pinned-hash-mismatch` and `squatted-name` among them and a program not found `internal-error`» — `batch-script-child` dropped from the broad line
sidecar: §1 body revive --json line — batch-script-child removed with its refusal
rationale: the broad line that a11y-scope-summary#1.1 details names the retired batch-script-child refusal (Retired; Counts revive codes 11 → 10)
@proposal n=3 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-dssvg3hs basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: «no line is written and the mode is left as found» — «or ACL» and «on all three OSes» dropped (Linux only)
sidecar: a11y-scope-summary#3.1 — the Windows ACL arm and the three-OS clause retired with the Windows code
rationale: Retired names the Windows home owner/DACL strict-modes arm and the OS target «Windows first, macOS and Linux» → Linux only; Counts os-target class marks «three OSes» stale
@proposal n=4 detector=D-cascade severity=warning file=a11y-scope-summary anchor=fct-sjhco52d basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. A11y Scope Summary
change: drop «, on Unix,» after the failing-check list and the clause «; off Unix the check reads nothing» — the socket-directory check always runs (its forms `$XDG_RUNTIME_DIR/viola/` and `/tmp/viola-<uid>/`)
sidecar: a11y-scope-summary#4.1 — the off-Unix no-check carve-out left with check_socket_dir's off-Unix arm
rationale: Symbols removes the off-Unix Ok(()) arm of check_socket_dir (its Unix body unconditional; NotADirectory · Symlink · Unreadable unchanged) and the macOS $TMPDIR/viola/ form
@proposal n=5 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-hj6bq3jt basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: «a home under a regular file is this refusal» — the «on Unix» qualifier and the clause «on Windows (such a path stats `NotFound`) the `cli` catch-site fault line stands» dropped
sidecar: a11y-scope-summary#5.1 — the Windows NotFound branch of the home-under-a-file refusal retired with the Windows strict-modes arm
rationale: Retired names the Windows home owner/DACL strict-modes arm; Symbols removes strict.rs's Windows owner/DACL arm and windows_read_verdict
@proposal n=6 detector=D-cascade severity=warning file=a11y-scope-summary anchor=fct-rts3hgep basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 1. A11y Scope Summary
change: «`viola run` and a `viola revive` start over a socket directory that fails its check: each refuses …» — the «, on Unix» qualifier dropped, the rest unchanged
sidecar: a11y-scope-summary#6.1 — on-Unix qualifier dropped, the socket-directory check unconditional on Linux
rationale: Symbols removes check_socket_dir's off-Unix arm, its Unix body unconditional; the refusal itself is unchanged
@proposal n=7 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-itb2wxsm basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: «the unmodified `claude` TUI hosted in openpty» (was ConPTY/openpty); in the third boundary clause drop «On Windows the platform decides what arrives (F-W3): the ConPTY host swallows focus reports, and under win32-input-mode an injected mouse report reaches viola as typed keys and takes the wheel.» — focus, mouse and resize sequences and terminal replies never count as human editing
sidecar: a11y-scope-summary#7.1 — ConPTY host and the Windows F-W3 fact retired; pty_backend always openpty
rationale: Retired names the ConPTY sideload; Symbols makes pty_backend() always openpty; Counts pty_backend words 3 → 1
@proposal n=8 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-yscgzykm basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: «none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes (`e4 b8 ad`) reach it» — «on all three OSes; on Unix» dropped, a Linux statement
sidecar: a11y-scope-summary#8.1 — the feed-panic boundary stated for Linux; the three-OS and Unix-only clauses retired
rationale: Retired: OS target → Linux only; Counts os-target class («three OSes» stale, re-worded to the Linux statement, the claim kept)
@proposal n=9 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-4etq6dn5 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: retire the Owed item (the Windows outer-terminal reading of `e4 b8 ad` after a feed panic): the Windows reading left with the ConPTY code, recoverable at the tag `multi-os-final` (`ecbe7ab3`); nothing is owed on Linux
sidecar: a11y-scope-summary#9.1 — Owed Windows ConPTY byte reading retired with the Windows code and the Windows-only live measurements entry
rationale: Retired names the ConPTY sideload and records the working-route entry «Windows-only live measurements» retired at the 2026-11-04T23-11-43-wrap (pattern wolm)
@proposal n=10 detector=D-retired severity=warning file=body anchor=fct-rdegquaa basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: ## 1. A11y Scope Summary
change: «the boundary still holds after a contained vt100 feed panic (no viola literal on the outer terminal, the child's own bytes reach it)» — «on every OS», «on Unix» and «its Windows byte reading owed to "Windows-only live measurements"» dropped
sidecar: §1 body tui passthrough line — the feed-panic boundary a Linux statement; the Windows owed reading retired
rationale: the broad line a11y-scope-summary#7.1–#9.1 detail carries the retired Windows reading and the wolm entry (Retired; Counts os-target «on every OS» stale)
@proposal n=11 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-walt4vws basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: «… the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`)» — «, on every OS (on macOS no peer pid is read, the named carve-out of the CLI entity above, so there such a server is one of another euid alone)» dropped
sidecar: a11y-scope-summary#10.1 — the macOS no-peer-pid carve-out and on-every-OS clause retired
rationale: Retired names the macOS no-peer-pid carve-out; Symbols folds SERVER_PID_REPORTED / PEER_EUID_REPORTED (peer_euid_admitted is observed == Some(own)) and deviation 11 makes the hook-fail-open and handlers impostor cases unconditional
@proposal n=12 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-vtox22ts basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: Manual verification keeps «Orca with GNOME Terminal» and Overseer Direction 5 (the NVDA with Windows Terminal/conhost and VoiceOver with Terminal.app lines dropped); Service identity tagging reads «The CLI a11y checks run on Linux, so `os` is `linux`» (was all three OSes, tagged per matrix leg); the last note reads «This must hold on `[ubuntu-latest]`»
sidecar: a11y-scope-summary#14.1 — cli surface Linux only: NVDA/VoiceOver passes retired, os linux alone, matrix [ubuntu-latest]
rationale: Retired names the windows-2025 and macos-latest CI legs and os's windows · macos; Schema: diag-line os reads ["linux"]; Counts os enum 3 → 1 and the CI matrix 3 legs → [ubuntu-latest]
@proposal n=13 detector=D-retired severity=warning file=a11y-scope-summary anchor=fct-twqx5x46 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: «**Manual verification (supplemental):** Orca with GNOME Terminal, confirming the wrapped TUI reads the same as unwrapped» (was NVDA with Windows Terminal, Windows the live target); the portable-pty `=0.8.1` outer-PTY driver line unchanged
sidecar: a11y-scope-summary#16.1 — tui manual pass moved from NVDA/Windows Terminal to Orca/GNOME Terminal, Windows retired
rationale: Retired: OS target «Windows first» → Linux only; the Windows manual target leaves with the platform; Dependencies keep portable-pty =0.8.1
@proposal n=14 detector=D-new-detail severity=warning file=a11y-scope-summary anchor=fct-twqx5x46 fact=F1 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 1. A11y Scope Summary
change: add after the tui surface «- **Platform (every surface):** Linux only — the core on any current distribution with CI on `ubuntu-latest`, the 0.2.0 window features on Hyprland; the Windows and macOS code removed, recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
sidecar: a11y-scope-summary — Platform line added: Linux only, CI on ubuntu-latest, Windows/macOS recoverable at multi-os-final
rationale: Retired states the OS target «Linux only: the core on any current distribution (CI on ubuntu-latest), the window features of 0.2.0 on Hyprland; the Windows and macOS code removed, recoverable at the tag multi-os-final (ecbe7ab3)»
@proposal n=15 detector=D-retired severity=warning file=aria-patterns-roles anchor=fct-dx4kie52 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. ARIA Patterns & Roles
change: tui: «three portable-pty outer-PTY nextest cases on `ubuntu-latest`»; clause (1) «zero viola-originated bytes on the outer-PTY stream, byte for byte against an unwrapped fake-agent run»; clause (3) ends «the wheel probed by an `answer` to no pending dialog (exit 13 driver, 10 human)» with the full assertion — «on all three OS legs», «on Linux/macOS; viola's own literals absent on Windows ConPTY» and «the full assertion on Linux and macOS, the platform fact on `windows-2025` (F-W3 …)» dropped
sidecar: aria-patterns-roles#2.1 — P4 tui cases on ubuntu-latest alone; the Windows ConPTY reading and F-W3 retired
rationale: Retired names the windows-2025 and macos-latest legs and the ConPTY; task item 6: on Linux focus, mouse and resize sequences never move the wheel
@proposal n=16 detector=D-retired severity=warning file=body anchor=fct-jkkyxmll basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: ## 4. ARIA Patterns & Roles
change: «on the tui, outer-PTY nextest cases on `ubuntu-latest` — zero viola-originated bytes, keystrokes delivered unblocked past the current atomic paste, and focus, mouse and resize sequences not moving the wheel» — «all three OS legs», «on Linux and macOS» and the windows-2025 platform fact dropped
sidecar: §4 body P4 line — one ubuntu-latest leg, the windows-2025 F-W3 fact retired
rationale: the broad line aria-patterns-roles#2.1 details names the retired legs and the Windows F-W3 fact (Retired)
@proposal n=17 detector=D-retired severity=warning file=aria-patterns-roles anchor=fct-uok334yy basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 4. ARIA Patterns & Roles
change: «on `ubuntu-latest`» in place of both «on all three OS legs»; the spans line reads «… by `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` (bold NAME, amber only on `DIALOG`) … dim only on stale rows is proven at the unit tier and under the outer PTY (the stale-row witness, below); `viola wait` and `viola --help` carry no ESC byte and no lone CR»; the list-modes clause reads «the no-ESC assertion in each mode» — «on windows-2025 the parsed attribute set», «on ubuntu-latest and macos-latest only, not on windows-2025», «on Unix» and «Unix-only, and on windows-2025 each escaped name read as text under a 200-column terminal, because ConPTY re-renders escapes» dropped
sidecar: aria-patterns-roles#3.1 — P6 asserted on ubuntu-latest alone; windows-2025 / ConPTY / Unix-only carve-outs retired
rationale: Retired names the windows-2025 and macos-latest legs and the ConPTY; deviation 11 makes the stale-row witness unconditional; Outcome (a11y) P6 cases inside coverage on ubuntu-latest, none skipped
@proposal n=18 detector=D-retired severity=warning file=body anchor=fct-ijnl2u6y basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: ## 4. ARIA Patterns & Roles
change: «trycmd / assert_cmd assert the non-TTY modes and the portable-pty outer PTY the TTY-only behaviours, on `ubuntu-latest`» (was on all three OS legs)
sidecar: §4 body P6 line — one ubuntu-latest leg
rationale: the broad line aria-patterns-roles#3.1 details names the retired three OS legs (Retired; Counts CI matrix → [ubuntu-latest])
@proposal n=19 detector=D-retired severity=warning file=aria-patterns-roles anchor=fct-jgpx3odq basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## 4. ARIA Patterns & Roles
change: «**P6 — stale-row dim under the outer PTY** (cli). `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`, ungated, on a booted wrapper): …» and the last sentence «It runs on ubuntu-latest.» — «Unix legs», «`cfg(unix)`» and «on ubuntu-latest and macos-latest and not on windows-2025» dropped
sidecar: aria-patterns-roles#4.1 — the stale-row dim witness ungated, on ubuntu-latest
rationale: deviation 11 makes cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal unconditional; Retired names the windows-2025 and macos-latest legs
@proposal n=20 detector=D-retired severity=warning file=body anchor=fct-lchghj7u basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6 dependent_of=D-retired
section: ## 4. ARIA Patterns & Roles
change: «- **Stale-row dim:** dim only on stale rows is proven at the unit tier and, under the outer PTY, on ubuntu-latest» (was «Unix legs only … on ubuntu-latest and macos-latest; the outer-PTY witness does not run on windows-2025»)
sidecar: §4 body stale-dim line — witness ungated on ubuntu-latest; Unix-legs carve-out retired
rationale: the broad line aria-patterns-roles#4.1 details names the retired legs; deviation 11 ungates the witness
@proposal n=21 detector=D-retired severity=warning file=visual-design-verification anchor=fct-es7le4as basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 6. Visual Design Verification
change: «… picks a truecolor / 256 / 16 depth» (the «on Windows after enabling VT processing on stdout, restored afterwards; a failed enable writes none» clause dropped); «… and `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, the outer-PTY witness of a stale row dim from its first byte to its last» (the «on ubuntu-latest and macos-latest only (`cfg(unix)`, not run on windows-2025)» qualifier dropped); the sentence «Under `viola run` on Windows the outer-PTY stream also carries the ConPTY host's own SGR … on both backends.» dropped; the last sentence «Checked with assert_cmd + portable-pty on `[ubuntu-latest]`.»
sidecar: visual-design-verification#1.1 — Windows VT enable, the ConPTY host-bytes check and the three-leg matrix retired; stale-row witness ungated on ubuntu-latest
rationale: Retired names the Windows VT enable (rung 7) and the Windows depth branch, the ConPTY sideload and the windows-2025 / macos-latest legs; Symbols: style.rs loses the vt fact, rungs 1–6 and the depth rule unchanged; deviation 11 ungates the witness
@proposal n=22 detector=D-retired severity=warning file=visual-design-verification anchor=fct-46bk3wap basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## 6. Visual Design Verification
change: «… as the outer-PTY stale-row witness above asserts it on ubuntu-latest» (was «on the two Unix legs»); the sequence `[DIM, BOLD, RESET, DIM, RESET]` unchanged
sidecar: visual-design-verification#2.1 — stale-row witness on ubuntu-latest, not two Unix legs
rationale: deviation 11 ungates the witness; Retired names the macos-latest leg; Outcome (design) Linux human output byte-identical, the stale-row witness among it
@remark
Body facts carrying retired OS/shim wording that no g1 group details (other passes own them): §1 fct-orpg5jsb (NVDA/VoiceOver SR pass), fct-ktqfqjb4 (.ps1 runner, per-OS test job, all three OS legs), fct-hu3qpzu6 (P4 win32-input-mode), fct-ol6ygl55 (NVDA/VoiceOver/Orca trigger); §4 table fct-yzqlnwsx and fct-4ktfiloi; §7 SR table fct-2kjabyk2 and fct-wawttggc; §8 fct-2jdilj7v (cognitive-accessibility); §9 fct-by4mf7d7, fct-s4vibfo3, fct-pkoabs22; §10 fct-k6kozbbz; §11 fct-csjr2vg7 (ConPTY F-W3) and fct-4sblkbrt (scripts/agent-run.*); and the §3 contract keys keyboard-test-harness, a11y-testing-tool-pick, ci-integration.
