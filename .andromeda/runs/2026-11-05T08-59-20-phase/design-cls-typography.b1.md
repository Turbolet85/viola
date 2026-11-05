@answer n=1 entity=typography#1.1 v=no
why: Display role type values (Bahnschrift stack, 16px, tracking); the chunk renders no page and its code and tests read none of them
@answer n=2 entity=typography#1.2 v=no
why: Heading role type values for the rack separator labels; no code or test the chunk changes reads this style
@answer n=3 entity=typography#1.3 v=no
why: Callsign role type values for the NAME cell; the chunk changes no NAME rendering
@answer n=4 entity=typography#1.4 v=no
why: Body role type values including the DIALOG weight; the chunk changes no strip or field output
@answer n=5 entity=typography#1.5 v=no
why: Label role type values for column captions and the ATIS header; no caption or header text moves on Linux
@answer n=6 entity=typography#1.6 v=no
why: Code role type values for empty-state commands and the tape body; the chunk re-words no command text
@answer n=7 entity=typography#1.7 v=no
why: Data role type values and the RB exception; the chunk changes no readback box, timestamp or percentage output
@answer n=8 entity=typography#2.1 v=no
why: the --font-label variable declaration, a CSS custom property the chunk's code and tests never read
@answer n=9 entity=typography#2.2 v=no
why: the --font-field variable declaration, a CSS custom property the chunk's code and tests never read
@answer n=10 entity=typography#2.3 v=no
why: per-OS fallback stack for --font-label; no CI step renders it and the plan marks typography per-OS fallbacks not needed for this chunk
@answer n=11 entity=typography#2.4 v=no
why: per-OS fallback stack for --font-field; no CI step renders it and the plan marks typography per-OS fallbacks not needed for this chunk
@answer n=12 entity=typography#2.5 v=no
why: semi-condensed font-stretch on Bahnschrift with a no-op on fallbacks; no code or test the chunk changes acts on it
@answer n=13 entity=typography#2.6 v=no
why: ch-measured strip and tape columns and the DejaVu width case; the chunk's CI measures no render width, so the chunk does not need to hold it
