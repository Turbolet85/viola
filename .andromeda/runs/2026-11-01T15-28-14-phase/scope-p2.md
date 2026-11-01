# Scope — Unix endpoint and home hardening

**Chunk:** `2026-11-01-unix-endpoint-and-home-hardening` · viola-0.1.0 · Epoch 7 — Cross-OS completion (its first entry)

**The working entry:** Unix endpoint and home hardening — per-user 0700 socket directory, 0600 socket, peer euid both
sides, Unix strict modes, macOS pid gap recorded.

## What it builds

- The per-user Unix socket directory is verified before a wrapper binds and before a client connects: owner, mode, not a
  symlink. A directory that fails the check is refused, never used and never narrowed in place.
- The socket stays 0600 by chmod after the bind, a secondary control only.
- The peer's euid is checked on both sides of the local socket: the wrapper against a connecting client, every Unix client
  against the server it reached, before its first frame.
- Unix strict modes on the paths this entry reaches, with the board's home check made one rule (the CARRY below).
- The macOS pid gap is recorded where it stands: macOS reports no peer pid, so the server check there is a match with no
  comparison, a named carve-out.
- [inferred] What each refusal reads on each surface: a refused socket directory at a wrapper's start and at a client's
  connect, and a peer euid mismatch on either side, each take a closed code and a hint in the exit-cause catalogue's form,
  a row of `tests/cross_exit_causes.rs`, and P3 reads which exit each one is (the CARRYs below name both exit 21 and a
  `run` refusal).

## The freight, folded

- CARRY (chunk 2026-09-27-wrapper-channel): that chunk landed the per-user 0700 socket directory (`$XDG_RUNTIME_DIR/viola/`,
  `$TMPDIR/viola/`, `/tmp/viola-<uid>/`), the socket's 0600 by chmod after the bind (never `ListenerOptionsExt::mode`:
  `Unsupported` on macOS fails the bind) and the `.lock` start arbiter; the directory verification (owner, mode, not a
  symlink, before bind and before connect) and the peer euid check stay this entry's.
- CARRY (chunk 2026-10-10-viola-revive, this entry on the operator's answer at its wrap's Phase 2 halt, three placements
  shown with their prices, `chunks/2026-10-10-viola-revive/inputs/I8-relay-3.md.txt`, answer 5): a killed wrapper removes
  nothing, so on Unix its socket file stays, and the next bind takes over the leftover file.
- [inferred] That CARRY's measurement, kept with its marker: "measured at that chunk: the root kill path waits on a connect
  probe, `holder_gone` in `tests/support/home.rs`, because a wait on the file being absent never ended". P3 re-derives it at
  HEAD, and whether the takeover of a leftover file holds beside this entry's directory and euid checks.
- [inferred] That CARRY's hypothesis, kept with its marker: "hypothesis: the harness `cleanup` reads `endpoint_gone:false`,
  and so exits 1, after it force-kills a wrapper on Unix, because its Unix rule is the socket path gone
  (`crates/viola-e2e/src/harness/cleanup.rs`); no run measured it". P3 measures it.
- CARRY (chunk 2026-10-26-server-verification-before-any-frame, its P5 review card's row 3, the word
  `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/inputs/I4-relay-2.md.txt`, delegate the Viola overseer,
  founder-delegated): that chunk's server check compares the peer pid and its start time on Linux and Windows only — macOS
  reports no peer pid, so its verdict there is a match with no comparison — and its negatives compile out on macOS; this
  entry adds the client-side peer euid check (every Unix client) and test-plan §6's macOS exit-cause case (the per-user
  socket directory loosened to 0755 → exit 21) beside its own directory and euid freight.
- CARRY (chunk 2026-10-26-home-and-code-bearing-file-integrity, its wrap's P2 halt Q3, the overseer's answer,
  founder-delegated and PROVISIONAL, `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/inputs/I6-relay-1.md.txt`;
  the handoff of the `2026-11-01T02-46-44` 0-pending wrap reads this label as superseded, its `adaptation-record.md` line 25,
  the owner standing): `viola list`'s `board::scan` runs `check_path` (the write rule) on the home and then `check_instance`
  per instance, which holds the home to the read rule, so a 0750 home passes the board while it holds no instance and is
  refused naming the instance (`ScanRefused::Instance`) once it holds one; making the board's home check one rule is this
  entry's (`board.rs` unchanged by that chunk; its report's Count 14).
- CARRY (chunk 2026-10-31-exit-cause-code-catalogue, its P5 review card's Beyond-the-plan row,
  `.andromeda/runs/2026-10-31T15-42-40-phase/p5-card-1.md`, `chunks/2026-10-31-exit-cause-code-catalogue/inputs/I3-relay-2.md.txt`;
  the same record's line 31 makes that word the founder's, the mark gone, the owner standing): the socket-directory
  refusal's code, a leg of `v1-18`, is this entry's. `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the
  catch site today (measured at that chunk's research), so the code holds no such refusal: the verified 0700 directory's
  refusal takes its closed code and its hint, a row of `tests/cross_exit_causes.rs`. macOS's server check stays here too, a
  named carve-out of the founder's own word (`chunks/2026-10-31-exit-cause-code-catalogue/inputs/I2-relay-1.md.txt` Q4).

## Boundaries

- [inferred] No Windows behaviour moves: the pipe listener's SDDL, the client's SQOS and the Windows strict-modes reading stay
  as they stand. "Linux and macOS parity" keeps the same end-to-end suite on both OSes and its live Linux run; a leg of it this
  chunk cannot prove stays there.
- [inferred] No new exit code: the typed set stays 0, 1, 2, 10–14, 20, 21.
- [inferred] `hook` keeps its contract: exit 0, no body, no stderr; a refusal of this chunk reaches it only as a fail-open.
- [inferred] macOS gets no substitute for the peer pid: the gap is recorded, never closed by a weaker comparison reading as a
  match, and no test reads it as a comparison.
- [inferred] A boundary widening, a live `claude` start or a route split this chunk finds is a P4 fork, the founder's own word
  (the directive's item 5).
- The frontend stays deferred to 0.2.0.

## Capabilities

- [inferred] `v1-44` (only the owning user can connect; every client verifies; the macOS gap recorded) reads as this entry's
  by its acceptance and by the route's hint; P4 reads its `requirement` and every working-route line naming it before any lean.
- [inferred] `v1-53` (the same end-to-end suite on Linux and macOS, one live Linux run) reads as "Linux and macOS parity"'s, and
  `v1-18`'s socket-directory leg stands on a verified cap: the code lands here against its carve-out, P4 naming how the
  cap's note records it.

## The CI read (Setup 5a)

Every commit from the last wrap's flip (`aa9e01af`) through HEAD, `ci.py conclusion`:

- `aa9e01af`: green · checks 15/15 · wall 730 s · ci#41234685380.
- `211403ff`, `66876f56`: untested — pushed under a later tip (`1dbff048`).
- `1dbff048`: green · checks 15/15 · wall 758 s · ci#41267399751.
- `a25b7302`, `f8b0e884`, `6ba49f53`: not on the remote, noted — the records door's three local commits, which go out with this
  chunk's normal pushes (the directive's item 9).

No red and no `not green`: nothing to disposition.

## The operator's directive (`inputs#I1`)

- [inferred] The sampler is forced on this phase and on this chunk's wrap (item 1).
- [inferred] Classifiers at effort high with three draws, read from the batch's check line and `pick`'s trail and said on the P5
  card; anything reading `max` or fewer draws stops the phase and is reported (item 2).
- [inferred] Every subagent and Workflow transcript is kept and the run dir left intact (item 3).
- [inferred] A dead draw takes the contract's one re-spawn; a second death on the same agent, any other hard cascade failure,
  or agents doing the orchestrator's work (F352) stops the phase, reported with the run dir (item 4).
- [inferred] A boundary widening, a live-session cap or a route split is the founder's live word, asked as a P4 dialog; the
  founder may be away, and then it holds (item 5).
- [inferred] `df -h /tmp` is read before a stamped-home gate; Epoch 6's diagnosis and audit are no part of this run, and
  new-session's nudge stays unanswered (item 6).
- [inferred] A chunk's documents and files stand in the store: `{work_dir}` is `.andromeda/cache/chunks/{marker}/`, a document
  lands by `rec.py put --doc`, the files by `rec.py land`, the plan is read at `.andromeda/cache/docs/{marker}/plan.md`,
  `rec.py query` over `position` and `coverage_line` reads what `requirements.md` and `verification-matrix.json` held, and
  `route.py` is `promote`, `flip` and `cancel` only; the letters are followed over any habit or older chunk's example (item 7).
- [inferred] What the store changes under the skill is reported at once and listed on the P5 card: every refusal and every
  reach for an old path, naming the letter line followed; any `rec.py check` complaint; any row of `gate`, `planlint` or
  `inputs` reading wrong on the store; a re-put's facts changed, if the trail says; whether the render read at P5 serves (item 8).
- [inferred] HEAD is three local commits ahead of origin (`a25b7302`, `f8b0e884`, `6ba49f53`), going out with this chunk's normal
  pushes (item 9).
