group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:27   class: stale
  leaf-text: «restored from the log at start; `link` / `unlink` are not served)»
  source: .andromeda/architecture-detail/established-decisions.md:150 «The explicit `viola link` / `viola unlink` and their two channel methods are built, with no dated gap: the wrapper serves `link` and `unlink`»
  replace-with: «restored from the log at start, and the served `link` / `unlink` channel methods, sharing `driver_of`)»

- leaf: CLAUDE.md:27   class: omission
  leaf-text: «the `ok` document or the two printed commands), wheel,»
  source: .andromeda/architecture-detail/established-decisions.md:151 «The explicit verbs run `viola_state::strict::check_instance` on the home and `instances/<driven>` before `client::start`, so ahead of any role file, log or snapshot read»
  replace-with: «the `ok` document or the two printed commands), `link` / `unlink` (`src/cmd/link.rs`'s `link_call`, shared by `src/cmd/unlink.rs`: `<driver> <driven> [--json]` acting on the driven instance; `check_instance` on the home and `instances/<driven>` before any role file or log, exit 21 on a refusal; then the liveness pre-check, the server check and one frame), wheel,»
