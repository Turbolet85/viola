@answer n=1 entity=color-palette#1.1 v=no
why: Primary buff hex is web ink and the readback box fill; the CLI paints no hex, so the chunk's bytes do not carry it.
@answer n=2 entity=color-palette#1.2 v=no
why: Secondary hex for stale ink is a page colour; the stale row's token is SGR dim (task item 3), not this hex.
@answer n=3 entity=color-palette#1.3 v=yes
why: names the CLI DIALOG word of viola list in accent amber, which the cocked-row bytes the v1-40 witness reads must carry.
@answer n=4 entity=color-palette#1.4 v=no
why: handoff blue marks transfer arrows and link/unlink tape names, web tape lines; the chunk adds no blue CLI token.
@answer n=5 entity=color-palette#2.1 v=no
why: Base is the page and stale-strip background fill; the CLI sets no background at all, so this hex never reaches the bytes.
@answer n=6 entity=color-palette#2.2 v=no
why: Raised-1 is the lit wrapped strip fill, a web surface with no CLI counterpart in the chunk.
@answer n=7 entity=color-palette#2.3 v=no
why: Raised-2 is reserved for popovers and tooltips, none exist in v1, so no CLI line or test acts on it.
@answer n=8 entity=color-palette#2.4 v=no
why: Raised-3 is reserved for a v1.x brake confirmation that v1 does not have.
@answer n=9 entity=color-palette#2.5 v=no
why: Inset is a web cell fill for the DIALOG cell; the CLI sets no background, so only the amber word (entity 1.3) binds.
@answer n=10 entity=color-palette#3.1 v=yes
why: states the treatment of the DIALOG kind word (primary text), which the cocked row's bytes must match against the amber DIALOG word.
@answer n=11 entity=color-palette#3.2 v=no
why: Secondary caption colour for the web rack header and ATIS age; the CLI paints no caption colour.
@answer n=12 entity=color-palette#3.3 v=no
why: Tertiary colour for tape timestamps, activity and harness lines; the chunk has no tape and no such CLI line.
@answer n=13 entity=color-palette#3.4 v=yes
why: every field of a stale strip is muted and no ink is dimmer than the floor, which is the rule the stale row's whole-row SGR dim must keep inside.
@answer n=14 entity=color-palette#3.5 v=no
why: graphite text on a buff fill is the web readback box, which the CLI does not paint.
@answer n=15 entity=color-palette#4.1 v=yes
why: classes the RB read-back mark as a confirmed success, the mark the shared send readback cases read and assert.
@answer n=16 entity=color-palette#4.2 v=yes
why: classes unconfirmable as a warning degraded but not refused, which the send cases' stdout and stderr split and exit codes must honour.
@answer n=17 entity=color-palette#4.3 v=yes
why: classes the unable-plus-reason refusal line as an error and the 1px slash strike as the [/ ] mark, both read by the send and refusal cases.
@answer n=18 entity=color-palette#4.4 v=no
why: Info colours for activity, harness, skipped counts and tape status; the chunk has no tape and none of these CLI lines.
@answer n=19 entity=color-palette#5.1 v=yes
why: the live liveness word the list board prints beside stale, the other value of the liveness column the list cases read.
@answer n=20 entity=color-palette#5.2 v=yes
why: the stale liveness word and whole-strip treatment (all ink lamp-off, no fill change) are the stale row the chunk pins as SGR dim across the row.
@answer n=21 entity=color-palette#5.3 v=yes
why: the idle status word the STATUS column of the viola list board prints, which the Unix list cases read.
@answer n=22 entity=color-palette#5.4 v=yes
why: the busy status word the STATUS column of the viola list board prints, with no animation, which the Unix list cases read.
@answer n=23 entity=color-palette#5.5 v=yes
why: the unknown status word the STATUS column prints when the agents query is missing, which the list cases read.
@answer n=24 entity=color-palette#5.6 v=no
why: the driver holder word and its box colours are web wheel-holder styling; the scope names no holder column in the list.
@answer n=25 entity=color-palette#5.7 v=no
why: the human holder word and its weight 600 are web layout; the scope names no holder column in the list.
