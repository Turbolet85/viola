# design extract

## Relevance
partial: no web-spa surface, tokens, motion or iconography apply. Only `viola verify`'s human CLI output on the new Ctrl-C / TERM stop path is in reach. The two literal moves (`claude`, `--settings`) are outside the design domain.

## Constraints
- The plan defines `verify`'s human output as a step counter: static stdout lines appended one per ledger row, then one summary line, `stamped <ver>  <n> pass  <m> fail`. A failing-row run prints no stderr word (per design-system §Surface: cli → Component Patterns 5). The plan has no form for an interrupted run. Any line the stop path prints is new wording the plan does not hold. That wording is decided at P4 (the operator's word or a fork), never invented at implement.
- Streams are split. Results and data go to stdout. Refusals, their `hint:` lines and `error: …` lines go to stderr (per design-system §Surface: cli → Tokens, Streams). Any interrupt notice is a message, not a result: if one is printed, it goes to stderr.
- The exit-code table holds no row for an interrupted `verify`. For `verify` it names only the four exit-1 `unable:` refusals and the no-stderr-word failing-row case (per design-system §Surface: cli → Exit-code phraseology). The interrupted run's exit code comes from architecture §Conventions at P2. Design fixes none.
- `verify`'s output is never styled. The word `fail` is uncoloured. The cli bans allow colour only on `DIALOG` and `stale` rows, and allow no cursor control (per design-system §Surface: cli → Per-Surface Bans, cli). The stop path adds no SGR, does not erase the line and does not redraw.
- Human output is ASCII only and is never wrapped (per design-system §Surface: cli → Tokens, Width).
- Errors and hints never print upstream text, paths, pids or anyhow chains (per design-system §Surface: cli → Per-Surface Bans, cli). This covers `<cwd>/.viola-verify-<pid>/` and its siblings, `<home>/ledger/probes/<pid>/` and Run A's OS-temp dir name, which no stop-path line may print.
- Stack traces never print. Full detail goes only to `diagnostics/` (per design-system §Surface: cli → Platform-Specific Notes). A signal arriving mid-probe prints no panic text and no backtrace.

## Patterns to follow
- `verify`'s exit-1 refusals use the fixed-message pair: one `unable: <text>` line, then that cause's own `hint:` line, neither naming a path (per design-system §Surface: cli → Exit-code phraseology). If the stop path prints anything, it reuses this form, not a new shape.
- A static appended line is the project's one progress pattern. Lines already printed for completed rows stay as printed (per design-system §Surface: cli → Component Patterns 5).
- `viola verify --help` is one static ASCII paragraph that names no path (per design-system §Surface: cli → Component Patterns 5). Any help wording about interrupt behaviour keeps that form. Whether help changes at all is P4's call.

## Anti-patterns to avoid
- No spinner, no progress bar and no live redraw on the stop path. This includes no "cleaning up…" animation (per design-system §Surface: cli → Per-Surface Bans, cli).
- No bare "done", "success", "aborted ✓" or "cleaned" (per design-system §Surface: cli → Per-Surface Bans, cli). The cli bans forbid "done" or "success" without context and forbid green `✓`, red `✗` and emoji.
- No `stamped …` summary line that implies a measurement on a run that wrote no stamp (per design-system §Surface: cli → Component Patterns 5, where the summary reports the stamp).

## Contract bindings
- CLI exit phraseology ↔ architecture §Conventions: the exit-code table is the source for the interrupted run's code, and design only words the stderr first word.
- CLI error and hint text ↔ the security-plan NEVER-log floor: no path, pid or upstream text in any stop-path line.
- Human stdout/stderr ↔ the obs-plan event catalog: any `obs_event!` line for the interrupt is a log line, never human output. Which event, if any, is obs's to read at P2.

## Acceptance criteria contributions
- On a run that is not interrupted, `verify`'s stdout step-counter lines and its `stamped <ver>  <n> pass  <m> fail` summary are byte-identical to before the chunk (per design-system §Surface: cli → Component Patterns 5).
- Every byte the stop path prints is ASCII with no SGR or cursor control. Any message goes to stderr, and no line holds a path, pid or probe-dir name (per design-system §Surface: cli → Per-Surface Bans, cli).
- An interrupted run prints no `stamped …` summary line (per design-system §Surface: cli → Component Patterns 5).
- An interrupted run prints no stack trace or panic text (per design-system §Surface: cli → Platform-Specific Notes).
