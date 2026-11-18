# layouts · standing conventions

- S1 · each recorded fixture file is written by the recorder through its scrub and holds plain, uncoloured output with no host path, name or token in it · surface-cli
  decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
  decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
