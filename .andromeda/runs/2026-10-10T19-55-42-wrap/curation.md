# Curation log — wrap of 2026-10-10-statusline-pass-through

Session-local, with one named exception: the wrap ran in two sessions, so the candidates of the session that ran
implement and this wrap's Phase 1 were read from `resume-point.md` (its nine items), as its conversation is gone.
This session added the candidates of Phase 2 on.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a helper called only by cfg(unix) cases is dead code on Windows; the dev host can lint the Windows target" (extension, confidence 0.8)
                                              + host-linux.md: "TMPDIR is unset on this host" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "An acceptance sentence of the form 'X nowhere else in product code' is checked over the tree" (confidence 0.8)
  Filters: 1 dup · 1 task-specific · 0 conflict · 0 deferred · 4 rejected at exactly 0.6 (each with a home)
  No-other-home: "a cfg(unix)-only helper is dead code on Windows" · "TMPDIR is unset on this host" · "X nowhere else in product code"
  Extended: T2/testing.md: "2026-10-03: Import an item used only inside #[cfg(windows)] …" + "a helper function called only by cfg(unix) cases; the Windows-target lint on the dev host"
```

## Applied

- **T2 `testing.md`, extension of the 2026-10-03 entry on `cfg`-only imports** (0.8: measured by a real gate
  failure 0.4, a specific technical detail 0.2, no other durable home 0.2).
  Proof: CI run `38080061631` on `58f872077352` ended `failure` in `lint` on `windows-2025` for two helpers
  called only by `cfg(unix)` cases; a check-only `cargo clippy --target x86_64-pc-windows-msvc` under
  `target/wincheck` reproduced the runner's error in a control on the dev host (report, Decisions & corrections;
  `evidence/operator-pass.md`). The fix commit is `67ab367`.
- **T2 `host-linux.md`** (0.8: measured by a real failure 0.4, a specific technical detail 0.2, no other durable
  home 0.2). The file has no `paths:`, so the entry was judged at Tier 1's bar: one sentence, 229 B.
  Proof: report, Decisions & corrections: "the shell variable `TMPDIR` is unset on this host: a redirect written
  against it lands at the filesystem root" (the implementing session's command never ran).
- **T3 `session-learnings.md`** (0.8: the measurement falsified a plan claim 0.4, a specific technical detail
  0.2, no other durable home 0.2).
  Proof: report, Spec claims disproved 1: `grep -rn -F -e '--settings' src crates --include=*.rs` reads two sites
  in `src/cmd/verify/typed.rs` that predate the chunk and sit under its preservation guard; the plan's
  acceptance 1 said "nowhere else in product code". The operator's direction (`inputs#I5` item 4) corrected the
  sentence in the masters.

## Not applied

- dup · "a strict-modes check on the home runs before anything opens a role file there": this wrap's cascade
  wrote it into the generated body of `rules/security.md` and `rules/observability.md` from the amended masters.
- task-specific · "a sidecar payload's `Supersedes` names a heading the sidecar holds": a wrap mechanic the
  sidecar contract already states.
- 0.6 exactly, rejected, each with its home:
  - "a root test that needs `check_instance` to pass on every OS starts from a stamped home": test-plan §7 Seed
    strategies and §5 carry it (this wrap's amendment), and `tests-summary.md` after the cascade.
  - "a property is run red on a stub before it is trusted": test-plan §6 Property suite carries the fact (the
    strategy's whole seconds, and why).
  - "a live rig's launcher passes no `--settings` of its own under a build that writes the override": the CARRY
    on "Paste newline ledger row" carries it (the operator's answer, `inputs#I6`).
  - "the statusline recorder's three quoted words read which shell ran": the same CARRY names the recorder in
    the chunk's evidence.
- homed in a judgment base, not a learning: "a widening the founder already answered halts at the wrap as one
  card" is the playbook rule appended at Phase 2.

## For the handoff's deferred list

Recurrences despite a standing learning, each once (read from `resume-point.md` for the implementing session;
the last from this session's own start):
- `host-linux.md 2026-09-28/29`: a `cat` heredoc with a file target, refused by the guard;
- `host-linux.md` Exit codes: one gate call's output read through a line filter;
- `ci.md 2026-10-09`: `gh run list --commit` given a wrong sha (mistyped), an empty answer read once;
- `testing.md 2026-10-05`: nextest's padded duration broke a fixed-column split of `PASS` lines (93 read as 72);
- `testing.md` Test data: three new tests read files after the `TestHome` that held them was dropped;
- `host-linux.md` Paths: two Bash calls opened with a bare `cd` (this session's orientation and the light gate's
  launch; nothing broke).
