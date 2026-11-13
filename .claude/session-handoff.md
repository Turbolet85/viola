# Session Handoff

**Last Updated:** 2026-11-13T13:18:46Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** 2026-11-13-epoch-8-cleanup — one shared masking scanner, three files split, survivors closed, the cookie class removed

## Position
- Done: 2026-11-13-epoch-8-cleanup — `tests/support/source_scan.rs` for the three census binaries; the ledger test
  cases, the fake agent and `tests/cli_verify.rs` split under 800 code lines; `check_present` deleted; 25 survivor
  sites holding under `gate.py control`; `private_dir`; the secret scan's `cookie` class gone; CI ci#43415781471 green;
  v2-01 and v2-06 verified.
- Next: «Rows-proposal rule work» (`/andromeda-phase`) — it now carries the `fn digit_runs` CARRY.

## Work done
- Code + the operator pass (pre-CI `c072f354`, pushed); wrap: 14 row operations in 3 masters (TX 946–950), 2 leaf
  edits, curation T2 2 · T3 1 extended, 1 CARRY.

## Drift resolved
- The cookie class left test-plan, security-plan and obs-plan (6 rows set, 3 retired, the obs-plan
  `claude-env-and-cookie-never-logged` row kept under its key with name and rule set); security-plan gains
  `contract/artifact-canary-scan-classes/unreadable`; test-plan gains `decision/os-gated-cases-in-binary-root`.
- Rejected: 4 at Validate, 2 by fit (test-plan `decision/one-shared-test-helper`, `not_master` — its helpers reached
  testing.md at curation; obs-plan `decision/claude-env-never-logged`, `stands`).

## Notes
- The founder's own word (inputs#I5 item 3, «Не считается»): a print-and-exit read (`claude --version`, `--help`) is
  no live start and counts against no live-start cap — written into session-learnings.md's live-start-cap entry; the
  last wrap's curation conflict (`mise ls claude` against `claude --version`) is cleared: both reads stand.
- The unpin of `vb.sh`'s 2.1.296 shim is the operator's (the last chunk's P5 card row; no route entry).
- **Owned by 0.2.0** (carried): the 3600 s hold ceiling, PROVISIONAL; the live round's two reds; matrix#v1-27 deferred.
- **Carried, unchanged:** `chunks/2026-10-26-explicit-session-links`'s card confirm and P2 halt;
  `chunks/2026-10-27-bounded-inputs-at-every-boundary`'s P2 halt Q1 and Path 1's runner-variance reading;
  `obs-scope-summary#20.5` and `slo-invariants-telemetry-budgets#2.2` label `max` readings the report labels `mean`.
- **Deferred learnings:** the eight of the 2026-11-06 wrap (unchanged), and 3 learnings analyzed but not applied
  (max-3 cap):
  - tempfile's `persist_noclobber` is `renameat2(RENAME_NOREPLACE)`: a test separating a present-name guard from another
    error kind pins the parent folder's mtime, since a directory at the name reads EEXIST either way (confidence 0.8)
  - `rec.py find --text` is case-sensitive: a sweep for a word a master capitalises takes `--regex "(?i)…"` (0.8)
  - a library crate's unit test reads a line it emits through a minimal `tracing::Subscriber` scoped by `with_default`
    in its own test module — no `tracing-subscriber` dependency, no global install (0.8)
  Review with `/andromeda-wrap-session --review` if any should be applied.
- **Host:** python `yaml` is absent; read `df -h /tmp` before a stamped-home gate. claude 2.1.323 is `latest` and stamped.
- **Last failed command:** none.
