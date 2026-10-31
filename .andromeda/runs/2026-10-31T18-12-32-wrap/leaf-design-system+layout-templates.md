group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:26   class: stale
  leaf-text: «transfer markers ONLY»
  source: .andromeda/design-system.md:70 «Role (handoff) #5B8DB8 departure blue, transfer markers and `link` / `unlink` names only, never a fill, never text on holder, never a status.»
  replace-with: «transfer markers and `link` / `unlink` names ONLY»
- leaf: .claude/docs/design-summary.md:32   class: stale
  leaf-text: «(DIN; semi-condensed via `font-stretch: 87.5%`)»
  source: .andromeda/design-system.md:129 «Callsign, the NAME cell, in Bahnschrift at normal width and weight 600, never text-transformed»
  replace-with: «(DIN; labels and headings semi-condensed via `font-stretch: 87.5%`, the callsign at normal width and never text-transformed)»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «a failed strict-modes check refuses the whole board with exit 21»
  source: .andromeda/design-system.md:292 «A failed strict-modes check refuses the whole board (exit 21) in the fixed `unable:` / `hint:` form, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`.»
  replace-with: «a failed strict-modes check refuses the whole board with exit 21, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «whatever `--json` says, and no document)»
  source: .andromeda/design-system-detail/surface.md:168 «Under `--json` the `cli` catch site prints row 1's `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.»
  replace-with: «whatever `--json` says, and no document; a failed `cli` verb's `error: internal error`, exit 1, is with `--json` among the arguments before a `--` the document `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, while `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site)»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «home pair on stderr, or under `--json` the `instance-unreachable` document»
  source: .andromeda/design-system.md:340 «or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty.»
  replace-with: «home pair on stderr, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «no frame and no line written), or under `--json` the `instance-unreachable` document»
  source: .andromeda/design-system.md:338 «`{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one per cause, the cause one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`»
  replace-with: «no frame and no line written), or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty (exit 21's document is `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`)»
- leaf: .claude/docs/design-summary.md:50   class: stale
  leaf-text: «a strict-modes or preflight refusal's `{"v":1,"error":"<code>","detail":null}` with exit 1 and no hint»
  source: .andromeda/design-system.md:334 «under `--json` every refusal of the start it shares with `run` prints one stdout document and no hint, stderr empty»
  replace-with: «every refusal of the start it shares with `run`, `{"v":1,"error":"<code>","detail":null}` with exit 1, no hint and stderr empty, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; a `pump_child` error prints none, the child having held the terminal»
- leaf: .claude/docs/design-summary.md:57   class: stale
  leaf-text: «one line per event»
  source: .andromeda/design-system.md:99 «a refused send folds into its send line, never a separate line»
  replace-with: «one line per event, a refused send folded into its send line and a matched driver prompt into its send's readback»
