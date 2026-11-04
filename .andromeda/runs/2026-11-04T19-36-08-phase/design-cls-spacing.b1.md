@answer n=1 entity=spacing#1.1 v=no
why: space-micro 4px is the web readback box's pixel gap to its word cell; the cli has no pixel geometry and its column spacing is two ASCII spaces, so the chunk's code and tests do not reach it.

@answer n=2 entity=spacing#1.2 v=no
why: space-xs 8px is field-cell inline padding and rack row gap in the frontend strips, a 0.2.0 frontend geometry the chunk does not touch.

@answer n=3 entity=spacing#1.3 v=no
why: space-sm 12px is the ATIS header cell gap and the --cock-offset of the web rack, page geometry for the deferred frontend, not a cli rule.

@answer n=4 entity=spacing#1.4 v=no
why: space-md 16px is the page inline margin and rack label gap, web page geometry that the cli's text output does not carry.

@answer n=5 entity=spacing#1.5 v=no
why: space-lg 24px separates racks and the bay from the tape in the web layout, a frontend geometry with no cli counterpart.

@answer n=6 entity=spacing#1.6 v=no
why: space-xl 32px is the top page margin under the sticky ATIS header and the web strip height, frontend geometry the chunk does not reach.

@answer n=7 entity=spacing#2.1 v=no
why: --strip-h 32px is a CSS strip height for the web frontend, which renders no page in 0.1.0 and has no cli input.

@answer n=8 entity=spacing#2.2 v=no
why: --line-h 20px is a CSS tape-line and transfer-marker height for the web frontend, not a cli or pty rule the chunk's code or tests act on.

@answer n=9 entity=spacing#2.3 v=no
why: --band-w 4px is a CSS strip band width for the web frontend, with no cli, pty or test rule behind it.

@answer n=10 entity=spacing#2.4 v=no
why: --rb-size 16px is the readback square's CSS size in a 20px line, a frontend drawing value the cli's ASCII [RB] token does not carry.
