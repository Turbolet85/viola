group: architecture+design-system+layout-templates
read: .claude/rules/frontend.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/frontend.md:3   class: tree
  leaf-text: «"src/cmd/**"»
  source: .andromeda/registries/contracts/architecture/build-system.md:4 «The root bin carries no print allow: its only human writer is the `human` module, `src/human.rs`»
  replace-with: «"src/{cmd/**,human/**,human.rs}"»
- leaf: .claude/rules/frontend.md:23   class: tree
  leaf-text: «## CLI surface (`src/cmd/**`)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:23 «style.rs: the colour layer (`Depth`, `Tokens`, `Facts`, `decide`, `read`, the Windows VT enable and its restoring guard)»
  replace-with: «## CLI surface (`src/cmd/**`, `src/human.rs`, `src/human/**`)»
- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «stderr = `waiting:`, the send issue line, refusals»
  source: .andromeda/design-system.md:314 «`last` prints `last  builder  turn-ended 19:44:10.221Z` on stderr (TTY only), then the message text on stdout with C0/C1 escaped, or `no message` on stderr for `null`.»
  replace-with: «stderr = the TTY-only context lines (`waiting:`, the send issue line, `last`'s header), `last`'s `no message` for `null`, refusals»
- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «(none under `--json`;»
  source: .andromeda/design-system.md:338 «`{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one per cause, the cause one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`»
  replace-with: «(none under `--json`; there exit 21's document `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` names its cause, one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the strict-modes refusals above reading `strict-modes-failed`;»
- leaf: .claude/rules/frontend.md:25   class: stale
  leaf-text: «there `viola revive`'s strict-modes and preflight refusals are one stdout document `{"v":1,"error":"<code>","detail":null}` with stderr empty,»
  source: .andromeda/design-system.md:334 «under `--json` every refusal of the start it shares with `run` prints one stdout document and no hint, stderr empty»
  replace-with: «every refusal of the start `viola revive` shares with `run` is one stdout document `{"v":1,"error":"<code>","detail":null}` with stderr empty, a `pump_child` error printing none, the child having held the terminal,»
- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «and no document), errors.»
  source: .andromeda/design-system-detail/surface.md:168 «Under `--json` the `cli` catch site prints row 1's `internal-error` document on stdout in place of `error: internal error`, stderr then empty»
  replace-with: «and no document), errors (under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty; `run`, `revive`'s start, `mcp` and `hook` stay silent at that site).»
- leaf: .claude/rules/frontend.md:26   class: omission
  leaf-text: «nothing is stripped.»
  source: .andromeda/layout-templates.md:168 «and a value that is no RFC 3339 instant prints escaped»
  replace-with: «a `release --budget` line's end that is no RFC 3339 instant prints escaped; nothing is stripped.»
