# design · standing conventions

- S1 · each new error, refusal or hint line a test, seam or wrapper path adds is a fixed message that prints no upstream text, path, pid, anyhow chain or stack trace; full detail goes to diagnostics only · Anti-Patterns › Per-Surface Bans (cli)
  (design-system/color-palette fct-ylp3uujf) status-states
- S2 · each new test that reads human CLI output keeps results on stdout and waiting, refusal, hint and error lines on stderr, and expects no colour, glyph or cursor control under --json, non-TTY, NO_COLOR, TERM=dumb or viola run · Anti-Patterns › Per-Surface Bans (cli)
  (design-system/surface fct-pi5kme4c) cli-output
