# Design validation — route draft

## Insert
- Between `Design token bundle and bay layout` and `Strip-bay live page`: **"Strip and readback primitives — light-DOM viola-session-row strip in lit, stale, cocked and unwrapped states; viola-readback in four data-rb states, one drawing"** (epoch: `Epoch 7 — Web UI`)
  Reason: design-system §Surface: web-spa → Component Patterns 1–2 mark `<viola-session-row>` and `<viola-readback>` as "bootstrap first". The composite live page, both readback placements (tape and marker) and the Signature Test all depend on these two primitives.

## Reorder
- Move `CLI output discipline` before `Capability ledger and viola verify`
  Reason: design-system §Surface: cli (Tokens, Colour decision order, Streams) is the CLI's token layer. Its first consumers are `verify`'s step-counter lines (cli pattern 5), then the `send` mirror (Epoch 3) and the `viola list` amber, dim and bold styling (Epoch 4), and all of these currently come before it.

## Rewrite
- `CLI output discipline`: "linear static output on three OSes, colour only beside its word" → "SGR attention/stale/callsign tokens with depth fallback, colour decision order, stdout/stderr split, ASCII-only static output, grouped --help, colour only beside its word"
  Reason: the current scope leaves out the cli token table, the stream split and the board/traffic/wheel/handoff/setup help grouping (design-system §Surface: cli Tokens + Navigation Pattern; layout-templates cli §Primary navigation).
- `Confirmed send with CL-1 records`: "typed not-delivered details" → "typed not-delivered details, [RB]/[  ]/[/ ] readback mirror with per-reason hint line"
  Reason: the CLI half of the signature has no chunk in the draft, and the Signature Test requires it (design-system §Brand Identity Signature; cli pattern 2; layout-templates cli §Hero / signature output line).
- `The board: viola list`: "six-column board" → "BAY context header over six-column board, bold NAME, amber DIALOG word, dim stale row"
  Reason: the BAY header and the three SGR cues are the board's design, and the draft names neither (design-system cli pattern 1; layout-templates cli §Header / banner).
- `viola ui loopback server`: "OS-assigned port and launch file" → "OS-assigned port, launch file and unstyled two-line stderr launch line"
  Reason: design-system cli pattern 5 fixes the launch line as unstyled, with the URL alone on its own line and no OSC 8 hyperlink.
- `Design token bundle and bay layout`: "light-DOM viola-* elements" → "token test, per-OS font stacks resolved on the Linux CI render"
  Reason: the Linux DejaVu render is the one CI asserts, and the eight-hex / tokened-length check is required before any UI output (design-system §Typography "Assertions hold on the Linux fallback"; Self-Validation §4 Token Test). The light-DOM elements move to the inserted primitives chunk.
- `Strip-bay live page`: "silent tape with readback box" → "silent tape with readback box, expandable lines, 2000-line cap and follow rule"
  Reason: the expanded tape line is a listed primary screen, and the line cap and follow rule shape the tape region (layout-templates web-spa §Wireframe — Tape line, expanded; §Primary content block 3).
- `401 access strip`: "401 access strip — a stale cookie after a ui restart shows the access strip" → "Access and error strips — 401 access strip after ui restart, 503 state-unreadable rack strip, TAPE stopped strip"
  Reason: the degraded bay screen (TAPE stopped, 503 strip) has no chunk anywhere in the draft, and all three strips share one anatomy (design-system web-spa component 6; layout-templates web-spa §Wireframe — Bay, degraded).
