# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · yes · the no-SGR rule binds: the chunk's refusal output (`unable`/`hint:` lines, the moved `--json` document, `viola list`'s whole-board refusal, `viola plugin install`, `viola verify`'s refused tree) must carry no SGR under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`, with `unable` and `fail` never coloured; the board-row bold, amber and dim styling is not the chunk's to change
