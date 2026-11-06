@answer n=1 entity=color-palette#1.1 v=no
why: web surface ink and fills (readback box, strips); no chunk code or test renders or pins this hex, and plain-mode output carries no colour.
@answer n=2 entity=color-palette#1.2 v=no
why: web surface secondary ink on anthracite; the chunk touches no palette value and renders no strip.
@answer n=3 entity=color-palette#1.3 v=no
why: the CLI DIALOG colour is the SGR module's own styled byte, which item 7 keeps unchanged; plain-mode output carries no colour.
@answer n=4 entity=color-palette#1.4 v=no
why: handoff blue for names in web tape lines and link/unlink tape; no chunk code or test reads this colour.
@answer n=5 entity=color-palette#2.1 v=no
why: web page background and surface scale; the chunk builds no surface.
@answer n=6 entity=color-palette#2.2 v=no
why: web raised-surface fill for a lit strip; the chunk builds no surface.
@answer n=7 entity=color-palette#2.3 v=no
why: reserved popover surface with no v1 use; the chunk adds none.
@answer n=8 entity=color-palette#2.4 v=no
why: reserved modal surface with no v1 use; the chunk adds none.
@answer n=9 entity=color-palette#2.5 v=no
why: web inset cell fill; the chunk builds no surface.
@answer n=10 entity=color-palette#3.1 v=no
why: web text hierarchy for strips and tape; no chunk code or test pins this ink.
@answer n=11 entity=color-palette#3.2 v=no
why: web column-caption ink; the chunk changes no caption and pins no colour.
@answer n=12 entity=color-palette#3.3 v=no
why: web tape timestamp ink and its typography; no chunk test reads it.
@answer n=13 entity=color-palette#3.4 v=no
why: web stale-strip ink; the chunk renders no strip.
@answer n=14 entity=color-palette#3.5 v=no
why: graphite ink on buff for a web read-back box; no chunk code uses it.
@answer n=15 entity=color-palette#4.1 v=no
why: web read-back state colours; the chunk adds no state rendering and no colour test.
@answer n=16 entity=color-palette#4.2 v=no
why: web warning state colours; the chunk adds no warning rendering.
@answer n=17 entity=color-palette#4.3 v=no
why: web error state colours; CLI refusal text is unchanged by the chunk and carries no colour in plain modes.
@answer n=18 entity=color-palette#4.4 v=no
why: web info state colours for activity and tape; the chunk changes no such line.
@answer n=19 entity=color-palette#5.1 v=no
why: web holder-strip colours for Liveness::live; the chunk's status rule is the cli board's (item 11), not this palette's colour.
@answer n=20 entity=color-palette#5.2 v=no
why: web stale-strip treatment and its /api/sessions omission; outside the chunk's scope.
@answer n=21 entity=color-palette#5.3 v=no
why: web SessionStatus::idle colours; the board's status-word rule (item 11) sits on the cli surface, not in this palette.
@answer n=22 entity=color-palette#5.4 v=no
why: web SessionStatus::busy colours with no animation; no chunk code or test renders it.
@answer n=23 entity=color-palette#5.5 v=no
why: web box treatment for unknown; the board's unknown rule (item 11) sits on the cli surface, not in a colour.
@answer n=24 entity=color-palette#5.6 v=no
why: web wheel-holder driver colours and weight; the chunk adds no wheel rendering.
@answer n=25 entity=color-palette#5.7 v=no
why: web wheel-holder human colours and weight; the chunk adds no wheel rendering.
