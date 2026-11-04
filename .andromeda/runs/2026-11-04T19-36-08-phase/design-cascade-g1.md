@carried n=1 label=design-system/brand-identity fact=fct-zdmbmsfz
why: The v1-40 integration-tier witness the chunk takes up must read, under the outer PTY on Unix, a static board where colour is only a second cue: SGR dim on the `stale` row beside its word and amber only on `DIALOG`.
quote: cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners.
@carried n=1 run=2 label=design-system fact=fct-ibtjtsuk
quote: **NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim), and never either without its word.** The quiz confirms colour as a second cue only.
@carried n=1 run=3 label=design-system fact=fct-2ks73rzm
quote: **Coasting track.** `stale` liveness. The strip is dimmed to lamp-off, never deleted, and keeps its slot.
@carried n=1 run=4 label=design-system fact=fct-ve72ntsx
quote: On `dialog_pending` the row keeps its slot and shows the word `DIALOG` plus the dialog kind, the word amber on the CLI.
@carried n=1 run=5 label=design-system fact=fct-ltouy2sa
quote: `stale` is a coasting track (lamp-off), not red, and the bay's only colour state is the cocked strip's amber beside its `DIALOG` word.
@carried n=2 label=design-system/brand-identity fact=fct-e37xe7yc
why: The openpty `viola run` wrapper the chunk builds, and a passed `viola revive` on Unix, must print nothing and emit no colour, glyph or cursor control while the child runs, keeping the child's terminal untouched exactly as the ConPTY wrapper does.
quote: cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
@carried n=2 run=2 label=design-system fact=fct-4m7ag2lh
quote: viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
@carried n=3 label=design-system fact=fct-olwc37rq
why: The openpty wrapper's resize and every Unix test under openpty must take width only from the PTY's own size, truncating only where it is known and never wrapping.
quote: **NEVER hardcode widths or wrap at arbitrary points.** Truncate only where the terminal width is known, and never wrap.
@binding n=1
**brand-identity's cli TTY row ↔ the cli surface's tokens and Per-Surface Bans**: the v1-40 witness under the outer PTY must read the expression row's dim `stale` beside its word (design-system/brand-identity fct-zdmbmsfz) as the ban's only two colours (design-system fct-ibtjtsuk), styled by viola's own SGR module alone (design-system fct-3n5f23up), on a stdout `IsTerminal` reads as a terminal (design-system fct-7yh2z5by).
@binding n=2
**brand-identity's 0.0 passthrough row ↔ the cli surface's Per-Surface Bans and the openpty wrapper**: the Unix `viola run` and passed `viola revive` must print nothing while the child runs (design-system/brand-identity fct-e37xe7yc), the same passthrough ban the ConPTY wrapper keeps (design-system fct-jphpatxv), with no motion at either level (design-system fct-4m7ag2lh).
@binding n=3
**cli Platform list ↔ Platform-Specific Notes**: parity on macOS and Linux terminals (design-system fct-7z5o76y5) leaves `cfg(windows)` only to behaviours one OS owns, the Git Bash warning and the conhost fallback (design-system fct-7u3hdiu2) (design-system fct-jphpatxv), while `hook` and `mcp` keep their contract on every OS (design-system fct-cj5433yq).
