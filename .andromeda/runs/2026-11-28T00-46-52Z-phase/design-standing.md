# design · standing conventions

- S1 · every human-facing line or frame text the console adds is ASCII-only, escapes control characters and uses colour only as a second cue · anti-patterns-never-do-these
- S2 · every new stderr message the console adds takes the CLI's unable and hint line form with no internals · surface-cli · surface
  contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` use `unable: <text>`.
- S3 · every new name the console adds to the command register follows the flat-verbs naming of the existing commands · surface-cli
