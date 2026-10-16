
## 2026-10-14-resume-and-compact-ledger-rows — the resume row, Run E and /compact listed
**Section:** §Stack and Technologies (Screen model); §Established Decisions → [CLI Version Compatibility], [Plugin Scope], [Delivery Confirmation]; §Standard Contracts → Ledger stamps envelope; §Cross-cutting Patterns → Capability ledger; registries `ci-cd-approach`, `crate-dependency-direction`, `project-directory-structure`
**Change:**
- The ledger holds twenty-two rows. The 22nd, `resume-session` (`--resume <id> reopens the session under that id`), is relied on every OS.
- The relied set is 22 on Unix and 19 on Windows (the first eighteen and `resume-session`), so `MM` is 22 / 19.
- Older stamps:
  - Was: an all-`pass` stamp of the eighteen ids verified on Windows.
  - Now: one of the 14, 17, 18 or 21 older ids reads unverified on every OS, and eighteen plus `resume-session` verifies on Windows only.
  - Every home stamped before reads `cli_verified:false` until it is re-stamped.
- verify drives five interactive runs (was four). Run E runs last:
  - it takes Run B's first SessionStart id through `ledger::resume_target`, of the session-id shape only; with none it spawns nothing and the row fails;
  - it runs in a 0700, guarded `<cwd>/.viola-verify-<pid>-resume/`;
  - its argv is the run args, then `--plugin-dir` with the `resume/` capture plugin, then `resume_args(id, false)`;
  - input is empty and there is no `--settings`;
  - it waits for one SessionStart, settles, and is killed.
- A verify is six `claude` starts. `Probes` gains `resumed`, and `--record` writes `SessionStart.resume-1.json` (`resume_variants`).
- `LOCAL_COMMANDS` holds `/clear` (a new session), `/remote-control` and `/compact` (none). A `/compact` send is `unconfirmable` at once.
- The Capability ledger's "one relied-on dependence has no row" (`--resume`) is retired.
- `/compact`'s post-condition row is owed to "Compact post-condition ledger row" at the end of Epoch 5.
- As measured live on 2.1.287 on the Linux dev host:
  - `stamped 2.1.287  22 pass  0 fail`;
  - `--resume <id>` found Run B's session from Run E's own `-resume` project dir;
  - Run E added no transcript.
**Why:**
- Run E is a boundary widening of the verify child set. It is ratified by the founder's own answer to P4 fork 2, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer, `inputs#I6`), superseded by his own word at the Epoch 5 boundary.
- Listing `/compact` and owing its row is his answer to fork 1 (`inputs#I1`).
- The row's placement is his own word at this wrap, relayed verbatim by the overseer (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/
