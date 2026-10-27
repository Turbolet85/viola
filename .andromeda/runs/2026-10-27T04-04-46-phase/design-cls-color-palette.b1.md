# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · no · a frontend border and fill colour (core, 0.2.0), its `/` strike is bound by color-palette#4.3
color-palette#1.2 · no · a frontend secondary ink (core, 0.2.0), and the CLI paints no secondary ink
color-palette#1.3 · no · standing S1 · human output is ASCII only and colour-free under --json, a non-TTY stdout, NO_COLOR or TERM=dumb
color-palette#1.4 · no · a frontend transfer-marker blue (core, 0.2.0), and the CLI paints no blue
color-palette#2.1 · no · the frontend page and bare or stale strip surface (surface scale, 0.2.0), and the CLI sets no background
color-palette#2.2 · no · a frontend lit wrapped-strip surface (surface scale, 0.2.0), and 0.1.0 renders no page
color-palette#2.3 · no · a reserved popover surface that v1 does not have and the chunk does not add
color-palette#2.4 · no · a reserved modal surface that v1 does not have
color-palette#2.5 · no · a frontend holder inset cell (surface scale, 0.2.0), and the CLI sets no cell background
color-palette#3.1 · no · frontend Primary ink for callsigns and phraseology words (text hierarchy, 0.2.0), and the CLI prints `unable` and the DIALOG kind plain
color-palette#3.2 · no · frontend Secondary ink for captions, since times and ages (text hierarchy, 0.2.0), and the CLI paints none
color-palette#3.3 · no · the frontend tape's Tertiary timestamp and activity ink (text hierarchy, 0.2.0), and the chunk's wait time prints plain
color-palette#3.4 · no · the Muted ink of zero-count skipped figures (text hierarchy, 0.2.0), and the BAY line prints its zero plain
color-palette#3.5 · no · graphite RB text on a buff readback fill (frontend, 0.2.0), and the chunk prints no RB
color-palette#4.1 · no · the success readback fill and its `read back` word (semantic colour, 0.2.0), and the chunk adds refusal lines only
color-palette#4.2 · yes · the `expired` budget-window state the BAY line prints for an old budget.json reading, degraded and not refused, its colours are frontend
color-palette#4.3 · yes · the `/` strike of an error readback box, which the `[/ ] unable  <name>  not-delivered  control-character` refusal line prints, its colours are frontend
color-palette#4.4 · no · the info state's nonzero skipped border and TAPE states (0.2.0 presentation), a border the ASCII BAY line does not draw and the count visibility is the chunk's own rule
color-palette#5.1 · no · the lit live holder strip (domain status, 0.2.0), and the chunk changes no liveness word
color-palette#5.2 · no · the lamp-off stale strip (domain status, 0.2.0), and the CLI's dim stale row is not a palette value the chunk changes
color-palette#5.3 · no · the idle status's colours and box (domain status, 0.2.0), and the palette states no condition the chunk changes
color-palette#5.4 · no · the busy status's colours (domain status, 0.2.0), with no spinner or animation, and the chunk adds no animated output
color-palette#5.5 · yes · the `unknown` word the board prints when the agents read fails, which the chunk's STATUS rule needs, its box and colours are frontend
color-palette#5.6 · no · the driver holder's weight and colour (domain status, 0.2.0), and the wheel wording is not changed by this chunk
color-palette#5.7 · no · the human holder's weight and colour (domain status, 0.2.0), and the wheel wording is not changed by this chunk
