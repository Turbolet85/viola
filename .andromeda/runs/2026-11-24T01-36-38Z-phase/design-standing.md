# design · standing conventions

- S1 · each new CLI verb is a flat verb named in the command register and listed in the viola --help verb groups · surface-cli · surface
- S2 · each new refusal cause gets its fixed unable line, its exit code and --json cause word, and at most one human-mode hint per cause that shows no internals · surface-cli · surface
  contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run` or `revive` prints a fixed-message `unable:` line and one hint per cause; `revive`'s causes are read in a fixed order that stops at the first.
- S3 · each new line of human output is ASCII-only, escapes control characters, puts data on stdout and messages on stderr, and carries colour only as a second cue · anti-patterns-never-do-these
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
