# Journal — implement, 2026-11-01-unix-endpoint-and-home-hardening

Each long run, named before it is waited on.

- 2026-11-01T16:50Z first targeted nextest over the new and changed cases, profile ci (a per-test kill line): the socket-directory binary, the two state units, the channel units, the board, the catalogue, the error bodies, the controls, the MCP handlers, the harness lifecycle.
  Read: 551 of 562 passed. Nine handlers cases and two socket-directory cases red. Handlers: a bare `tempfile::tempdir()` is left to the umask (0755 here), so the new client check refused it: research's 0700 reading was false, step 16's named trap; narrowed at the four sites, two recorded in the scope record. The other two: `send`'s human line is its mirror's form, pinned.
- 2026-11-01T16:52Z the whole block, `gate.py run`, backgrounded. Its `pre-push` entry declares 7200 s, `run --perf` 3600 s; every entry bounded by the tool.
  Read: red 2 and 15 only: rustfmt on a test line, and two scope-record `serves` fields not in the guard's form. Green: pre-push `linux-tests` with 3446 passed and the coverage gate clean, `run --perf` 7 passed.
- 2026-11-01T16:56Z the remove-the-guard runs, ten neutralisations one after another, each restored and read back by hash, each under the `ci` kill line and a 420 s bound on its nextest call. Script in the session scratchpad; readings in the chunk's evidence.
  Read: all ten red under the neutraliser, each file back by hash, no neutraliser left in the tree.
- 2026-11-01T17:00Z the whole block again on the final tree, `gate.py run`, backgrounded; the macOS check-only clippy after it, in its own target dir.
  Read: block `green 21 · red 0 · recorded 1 · not-run 3`; macOS clippy exit 0.
- 2026-11-01T17:06Z operator pass: `pre-push` re-fired before its push, entry 22, bound 7200 s.
  Read: green. Hygiene clean, land, pre-CI commit `c4bf0a92`, pushed by entry 24 with the three door commits (`1dbff048`→`c4bf0a92`).
- 2026-11-01T17:08Z entry 25, the CI read of `c4bf0a92`, backgrounded: `ci.py conclusion --sha HEAD --wait 2400`.
  Read: `verdict: green · checks 15/15 · wall 863 s · ci#41329861618`; each OS-sensitive witness read by its own `PASS` line in its job's log.
