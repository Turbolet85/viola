# Session Handoff

**Last Updated:** 2026-10-04T01:25Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the pre-CI commit `80b69cd` was pushed in the operator pass; this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** 2026-10-03-mutation-scoring-completion — the wrap commit of Mutation scoring completion

## Position
- Done: **2026-10-03-mutation-scoring-completion**:
  - C3: the diff-prefix pin;
  - M3: viola-e2e scored by the new `run --mutants --package <member>`, 711 mutants, missed 0 over the measurable set;
  - the 12 `cfg(unix)` mutants graded natively (9 caught, 3 unviable);
  - native Linux `pre-push`, with the WSL scripts deleted;
  - the macOS `channel_frames` close race (`test (macos-latest)` green on ci#37166444247).
- Next: **Windows boundary mutation workflow** (`working-route.md:70`) → `/andromeda-phase`. It now carries:
  - the 34 coordinates owed by this chunk (2 viola-e2e `prepare` mutants and 32 `cfg(windows)` mutations);
  - the mutation-run temp-dir leak as an `[inferred]` item with its own acceptance.

## Work done
- Harness:
  - the `--package` arm;
  - native `pre-push` (`env -i` HOME from the passwd entry + constant PATH, stages `tools → linux-tests`);
  - eight kill tests: six in viola-e2e, two in viola-pty.
- Evidence: `evidence/{c3,m3,cfg-unix,tui,operator-pass}.md`.
- Every mutation run on this host takes `TMPDIR=<repo parent>/viola-mutants-scratch`, a NOCOW btrfs dir: `/tmp`'s quota
  is too small, and a btrfs reflink copy drops the exec bit.

## Drift resolved
- **28 amendments, 0 escalations:**
  - architecture: §Stack, CI/CD, directory tree, Occupied Resources;
  - security-plan: Development, Pinning, Anti-Patterns;
  - test-plan: §2, §3 pre-push / run / closed enums, §9, §10;
  - obs-plan: §8 item 6, §1 note.
- **10 leaves re-derived.**
- **Founder rulings carried:**
  - verbatim upstream copies are kept current: playbook rules :40 and :44 superseded and kept, a new rule appended;
  - the mutation-gate rule is curated into `testing.md`.

## Notes
- **`host-win32.md`** still describes the retired Windows host. It now carries one labelled Linux-host fact (the btrfs
  reflink entry). Its replacement is an `/andromeda-setup-project` re-run, on the founder's timing (directive 5).
- **Operator cleanup left on disk** (the permission layer refused the `rm`):
  - `! rm -r /tmp/cargo-mutants-ws-*.tmp` (21 dirs, 325 MB);
  - `! rm -rf ~/dev/projects/viola-mutants-scratch/{.tmp*,cargo-mutants-ws-*,rustdoctest*}` (~8 GB). Keep the dir
    itself: it is NOCOW, and the next mutation run needs that attribute.
- **Installed `claude` is 2.1.287.** Fixtures exist only for 2.1.283, so it stays unverified until `viola verify` runs.
- **Deferred learnings** (max-3 cap):
  - the "not measured here; owed to {route entry}" vocabulary (already in test-plan §10);
  - PID 1 as the cleanup-deadline target that outlives SIGKILL (kill(2));
  - a PTY master close hangs up a live child only when no reader/writer clone holds the master;
  - carried from before: let a red CI run finish before folding its fix; the doubled-backslash guard recurrence.
- **Last failed command:** none (the two `rm` calls were permission refusals, left to the operator above).
