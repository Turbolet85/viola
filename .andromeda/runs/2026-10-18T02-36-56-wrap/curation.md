# Curation — 2026-10-17-optional-plugin-install (wrap P3)

CLAUDE.md ecosystem: no new learnings this session.
  Filters: 1 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 1 recurrence (→ handoff)
  CLAUDE.md size: read at P7 (`health.py check`, check 1)

## Decisions & corrections — one line per entry
- `target/debug/viola` is no gate's output; a live rig must rebuild it first → rejected, Filter 4 (verified by measurement +0.4, specific detail +0.2 = 0.6, at the bar). Its durable home is the P5 CARRY on "Compact post-condition ledger row", which names the rebuild for the round that entry owes.
- `--scope local` resolves from the git work tree root → not a learning: amended into a master this wrap (security-plan §Data Protection, the accepted residual).
- A git-root rig is untrusted; its start meets the folder-trust dialog, whose confirm row is not the selected one → not a learning: carried at an annotation position by this wrap's P5 CARRY on "Compact post-condition ledger row".
- The CLI's uninstall leaves an orphaned plugin cache → not a learning: amended into security-plan this wrap.
- `cfg!(windows)` in a product file makes it Windows-gated, so it must join a `windows-mutants.yml` job → rejected, Filter 1 (duplicate): the generated body of `.claude/rules/verification-harness.md:43` already says `tests/contract_windows_mutation_scope.rs` keeps the workflow's file lists equal to the Windows-gated sources (0.6 besides).
- The write guard refuses a Write under `target/`, and the Bash guard refuses a cwd-moving `cd` → not a learning: each is a hook that already refuses the act, at one re-issue (`guard-refused`).
- `AskUserQuestion` in this driven session is answered by the driver through viola's dialog hook, and one ran to its 58-minute bound → rejected, Filter 4 (measured +0.4, specific +0.2, a plausible alternative reading −0.2 = 0.4): the second dialog of this window, the answer card, was answered at once, so the timeout reads as the driver's absence, not a rule.
- `hyprctl locked` read `true` before start 2, and the start went ahead on `host-linux.md`'s `wtype` scope → → handoff: P6 rewrites the handoff's standing rule to the host leaf's own wording (focus a window or type a key with `wtype`), with no curation entry.
- The remove-the-guard control → not a learning: a standing rule followed (testing.md 2026-09-25).
- `concat!` literals and the colour facts removed per child → not a learning: standing rules followed (testing.md 2026-10-17).
- The founder's and the overseer's words snapped as inputs → not a learning: the inputs ledger is their home.
- The run dir and the tree re-read after the idle windows → not a learning: the handoff's own re-verify rule followed.
- (this window) a `cd X 2>/dev/null; …` prefix moved the session cwd for one call → → handoff, `recurrence-despite-learning: host-linux.md Paths & the working directory ("anchor every cross-call path absolutely, or cd only inside a subshell")`.
