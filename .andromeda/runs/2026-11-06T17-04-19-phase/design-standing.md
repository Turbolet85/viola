# design · standing conventions

- S1 · any new or re-worded assertion over a CLI human string (a trycmd / assert_cmd case or a text pin) pins it exactly as the cli Component Patterns give it: ASCII only, two spaces between columns, results on stdout and refusals with their `hint:` lines on stderr · surface
  (design-system/iconography fct-g2p3izdb) icons
- S2 · any new test reading human output from a pipe, under `NO_COLOR`, `TERM=dumb` or `--json` expects no colour, no glyph and no SGR, the colour decision order's first match winning · surface
