
## 2026-10-05-real-cli-verify-probes — verify's typed-input PTY probe: the widening, its dirs, residual and refusal
**Section:** Threat Model Summary → Child process spawning · Input Validation → Child process output · Constants · Data Protection (Interactive probe dirs · Repository fixtures · two new notes) · Security Anti-Patterns → Universal
**Change:**
- Child process spawning: verify spawns two interactive `claude` PTY children after its print probe, both direct spawns under the R8 strip at 80×24 with output capped at `MAX_FRAME` into a `Screen` under `catch_unwind`. Run A is untrusted in a fresh 0700 OS-temp dir with empty input and is killed. Run B is trusted in `<cwd>/.viola-verify-<pid>/`, gets one compiled `PROBE_PROMPT` paste and ends by Ctrl-C ×2, then kill. Neither run sends a byte into a CLI-native dialog; a modal start is killed with no key. viola writes nothing under `~/.claude`.
- Child output row and Constants: `MAX_FRAME` also caps verify's PTY output, read only against the compiled literals.
- Interactive probe dirs: both outside the viola home, 0700, removed on every exit path.
- Repository fixtures: screens keep only signature rows. The refusal also covers an email-shaped token and a seam-split username. It was the fixed `a recorded payload still holds a path or a username`; now it is `a recorded fixture is not clean: <file> <code>` (`<file> row <n>[ seam] <code>` for a screen), with the closed codes `home-path` · `absolute-path` · `username` · `email`, never the content.
- New accepted risk: Run B leaves the CLI's synthetic-prompt transcript under `~/.claude/projects/` and runs the user's global hooks and status line (dev host: 5 transcripts, 13 → 18 dirs).
- New dev-host prerequisite: run from a trusted folder whose external import is answered (keyed on the git root). The overseer set the repo-root flags to "No" in one atomic `~/.claude.json` edit with a backup.
- Universal: the R2 dated gap was the six-row stamp (`/06`) until `:84`; now the ten-row stamp (`/10`) until "Dialog rows and re-probe".
**Why:** boundary widenings, each the founder's:
- the probe, no-key and dirs: his live rulings R-S2 (~00:00Z), "two runs, never accept" (~07:00Z, re-affirmed ~08:50Z) and the two dirs (08:25Z), all 2026-10-05, each answered after the widening was shown, relayed by the overseer and ratified by the operator at this wrap;
- the named refusal: his live ruling at this wrap, through the overseer's AskUserQuestion, with the closed code set and the no-content rule shown;
- the residual and the flag edit: accepted at this wrap on the overseer's word, relayed by the operator — the founder saw the class at his trust ruling, and the edit was his amended live ruling (~09:05Z).
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/
