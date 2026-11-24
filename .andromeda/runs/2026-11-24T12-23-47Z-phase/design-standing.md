# design · standing conventions

- S1 · every new verb's output lines, result line and --json document are stated as a CLI verb-output contract beside the existing verbs' rows · surface-cli · surface
- S2 · every new verb's exit codes and closed --json refusal codes join the CLI exit-code table and refusal-code lists, each with its fixed hint · surface-cli · surface
- S3 · every new human output line obeys the anti-patterns: colour carries meaning only as a second cue, confirmations carry context, no internals in errors, controls escaped, plain output modes · anti-patterns-never-do-these
  contract/viola-link-output/result-line · viola link and unlink output · result line — `viola link` and `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0.
  decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
  decision/no-shimmer-or-glow-theme · No skeleton shimmer or glow theme — No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once. · why: Nothing glows in a tower cab unless the lamp is on it.
  decision/no-traffic-light-status-dots · No traffic-light status dots — Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word. · why: A strip prints its status word in a fixed column.
