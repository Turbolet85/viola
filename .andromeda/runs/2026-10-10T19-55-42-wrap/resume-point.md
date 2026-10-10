# Resume point — wrap of 2026-10-10-statusline-pass-through

**Phase 1 is done. Phase 2 is next.** Written 2026-10-10T20:00Z by the session that ran implement, the operator
pass and this wrap's Phase 1. The operator directed the stop (that session's context read past its line) and
resumes the wrap from Phase 2 in a fresh session, with the route directions given there.

## How to resume

- Setup 2a applies: `chunk_dir/report.md` exists. Resume in THIS run dir
  (`.andromeda/runs/2026-10-10T19-55-42-wrap/`), reuse the report as it is, and start Phase 2 at its first
  step, the citation sweep (`cites.py apply --dry-run`, then without). No fan-out was made; no master, sidecar,
  key file, rule or leaf was touched by this wrap.
- Re-fire Phase 1's two reads first, as a resume does: `gate.py scope` (read `clean` here: changed 36, listed
  35, recorded 1) and `cites.py added --out <run dir>/new-text.md` (expect `unchanged`; the listing is the
  report's last section, pasted by `splice.py`, 364 lines, 292 blocks).
- Setup facts read here: branch `build/viola-0.1.0`, 0 ahead of its upstream, HEAD `67ab367`. The basis is
  `4d77eac`, the parent of the oldest pre-CI commit `58f8720`. The pass's commits: `58f8720` (pre-CI),
  `67ab367` (test-only fix). `state.yaml` read `last_wrap` 2026-10-10T15:49:39Z, `session_count` 60.
- The code-graph refresh fired at this Setup finished: rust 4979 nodes / 26188 edges, ts 7 / 1. Phase 4 reads
  `.andromeda/cache/.refresh-done`; `tree_db_refreshed_at` is this run's (about 19:56Z; read the marker file's
  own time).
- The tree is dirty only with bookkeeping and this wrap's products: the friction ledger, the implement run
  dir's evolve and gate trails, `evidence/operator-pass.md` (its sections written after the fix commit), the
  report, this run dir.
- Phase 3's conversation is gone with the clear. Everything that conversation held for curation, the route and
  the handoff is below; nothing else of it needs to be recalled.

## One escalation waiting for Phase 2 (Validate)

**The plan's acceptance 1 is unmet as written in one clause** (report: Spec claims disproved 1, Outcome). The
plan says the `--settings` literal is defined in `viola-agent-claude` "and nowhere else in product code".
`src/cmd/verify/typed.rs:132` and `:173` hold the literal; they predate the chunk and sit under the plan's own
preservation guard. The chunk's diff adds no second product site (`src/run/mod.rs:262`, `:273` are test
oracles; `src/run/mod.rs` uses `SETTINGS_FLAG`). The criterion is linked to no capability, so it is a P2
escalation: the operator decides whether the sentence is corrected in the masters as it lands (the flag has two
product users, `run`'s override and `verify`'s Runs C and D) or a route entry moves `verify`'s two sites onto the
constant. Implement's own report did not carry this; it was found at this report's authoring.

## Findings for the masters and the route (the report carries each; this is the list)

1. **`viola revive` opens its log before its instance check** (report: Spec claims disproved 5). `revive` →
   `open_wrapper_log` → `viola_obs_init` → `open_role_file` → `create_private_dir(home)` sets an existing home
   to 0700; `preflight` → `check_instance` runs after. So a home the user owns at a group- or other-writable
   mode is narrowed before the check reads its mode, and the check cannot refuse it for that. Read in the
   source; **measured only on the new arm** (`evidence/red-green.md` §3: with the check after the log, the 0770
   case ran the user's command). Not measured on `revive` itself: `tests/cli_revive.rs:285` widens the instance
   directory, which the log's opening does not narrow. The same order question stands for every verb that
   opens a role file before a strict-modes check. Natural owner: the route entry "Home and code-bearing file
   integrity" (it owns the check at every entry point). The operator's route directions decide the pin.
2. **The arm's order is check, then log** (deviation 1). Expected amendments name "the arm's order": it lands as
   canonicalise → `check_instance` → log opens → `hook-invoked` → stdin → write → snapshot read → shell-out →
   `hook-decision`.
3. **Windows: a fixture-booted home is the test's, not viola's** (Spec claims disproved 3). On Windows x64
   `Wrapper::boot` seeds the ConPTY companions before the start, so the home is made by `create_dir_all` with an
   inherited DACL, and `check_instance` refuses it. A test that needs the check to pass on Windows starts from
   a stamped home. The refused home's DACL was not read. test-plan §7 Seed strategies is where the plan sent
   this.
4. **`rate_limits` on the second payload** of live start 2, 0.35 s after the first, against the fetched
   documentation's "only after the first API response". Cause not known. No master states the sentence; a
   candidate hypothesis line for "Budget governor", whose `send` refusal is the shape's first consumer.
5. **A `--home` user loses their own status line** until they plant `statusline-source.json` (live start 3;
   the W3 option's named cost). It is the founder's choice as recorded, not a defect; the masters should say
   it where the source rule lands.
6. **Step 12 read a project-scope status line**, not a user-scope one (deviation 2). What a user-scope one
   runs through and receives is not measured.
7. **`live-start.sh` of the earlier chunks passes a `--settings` of its own**: under a build that writes the
   override, the CLI gets two `--settings` flags and which one wins is not measured. Any later live rig that
   reuses that launcher over a home with the override meets this. A candidate note for "Paste newline ledger
   row" and "Windows-only live measurements", which both plan live rigs.
8. **Not built here, by the plan's cuts, each to its named owner:** `boot --statusline-echo` and its
   `statusline-source-unresolved` refusal → "Budget governor"; the `rate_limits` row → "Budget governor"; the
   settings-override row and the Linux and macOS shell row → "Paste newline ledger row" (only on the founder's
   word R, which chose no row); the Windows override and shell-out and the Windows legs of both rows →
   "Windows-only live measurements" (the founder confirmed the cut: S3 is final).
9. **No mutation run of the new code** (`inputs#I1`): the chunk added mutable code to `viola-core`,
   `viola-agent-claude`, `viola-state` and the root package. The handoff's standing note on
   `windows-mutants.yml` and the epoch boundary's audit covers it; say so again.

## For the handoff's founder and operator sections

- **The founder's words on this chunk are recorded** (`inputs#I4`, relayed verbatim by the operator): W1
  confirmed, W2 confirmed, W3 the file in the viola home, R option B (a manual reading, three live starts, no
  row, no fixture), the Windows cut confirmed. security-plan's rows for W1, W2 and W3 are written with that
  word; none is "Not ratified".
- **Still owed from the last wrap, untouched here:** his word on the logged session id on the child's command
  line (`viola revive`).
- **Three live starts were made**, the cap of three, on `claude` 2.1.287, model alias `haiku`, three short
  model turns on his subscription. Record: the chunk's `evidence/live-statusline.md`, ledger
  `live-sessions.ndjson`.
- **His own `~/.claude/settings.json` holds a `statusLine` of type `command`** (presence and type read, nothing
  else, never written). On the default home a wrapped session would run it through viola's hook; under
  `--home` it is not read.
- **Not measured:** macOS and Windows live; a user-scope status line; the flag the CLI hands its shell; how
  the CLI cancels a running statusline script and whether the user's command then outlives the hook; the 5 s
  bound on a real session; the status line row's own appearance with an empty output; an account without
  `rate_limits`; the `spend_limit` window.
- **Operator desk, added by this chunk (none committed, none removed):**
  - `target/e2e-home/viola-live-sl-20261010/` on the tmpfs: the unstamped rig home, three instance directories
    (`slreh`, `slsrc`, `slbare`), its `budget.json`;
  - `target/sl-live-20261010/` with `a` (one `.claude/settings.json` naming the rig's recorder) and `b`
    (empty), ignored by git;
  - the rig's private directory `sl-rig/` under the implementing session's scratchpad, named in
    `evidence/live-statusline.md`;
  - the CLI's own files for the three sessions under its project directories for `a` and `b`;
  - `target/wincheck/` was used again (the Windows-target lint).
- **The session-end hook did not touch `.claude/session-handoff.md` during the three live sessions** (its one
  changed line predates them).

## Curation candidates (Phase 3; the conversation that held them is gone)

Each is a fact met on live work in this session. Tiering is Phase 3's; the home named is a suggestion.

1. `testing.md` — **A root test that needs `check_instance` to pass on every OS starts from a stamped home**:
   on Windows x64 `Wrapper::boot` seeds the ConPTY companions first, so an unstamped home is made by the
   fixture with an inherited DACL and the check refuses it. Met as three red cases on `windows-2025`
   (run `38080061631`).
2. `testing.md`, an extension of the 2026-10-03 entry on `cfg`-only imports — **a helper function called only
   by `cfg(unix)` cases is dead code on Windows** and fails the Windows lint job; gate it like its callers.
   And the instrument: **the dev host can lint the Windows target** — `cargo clippy --workspace --all-targets
   --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings` under
   `CARGO_TARGET_DIR=target/wincheck`, check only. A control reproduced the runner's exact error. It runs no
   test. (Possibly `ci.md` or `host-linux.md` for the command.)
3. `testing.md` or `security.md` — **a strict-modes check on the home must run before anything opens a role
   file there**: opening the log sets an existing home to 0700, so a later check cannot see a widened mode. A
   guard test that widens the home (not a directory under it) is what shows it.
4. `testing.md` — **a property is run red on a stub before it is trusted**: the first strategy for the
   `resets_at` property stayed green on a stub that returned a wrong value, because arbitrary 64-bit seconds
   almost never land in the range a date holds. The plan's red-once step is what showed it; the strategy
   needs a leaf inside the accepted range.
5. `verification-harness.md` — **a live rig's launcher passes no `--settings` of its own under a build that
   writes the override** (finding 7), and **a rig that must not touch the user's settings names its statusline
   through the rig directory's project settings**.
6. `verification-harness.md` — **the statusline recorder's three quoted words** (`"$0"`,
   `"${BASH_VERSION:+bash}"`, `"${ZSH_VERSION:+zsh}"`) read which shell ran a command string even when that
   shell replaces itself with the command, where the parent's name reads only `claude`.
7. `host-linux.md` — **the shell variable `TMPDIR` is unset on this host**: a redirect written against it
   lands at the filesystem root and the command never runs; use the session scratchpad's path. (The gate
   contract's `$TMPDIR` is a printed name, not the variable.)
8. Recurrences despite a standing learning, for the handoff's list, each once in this session:
   - `host-linux.md 2026-09-28/29`: a `cat` heredoc with a file target, refused by the guard (an append to a
     record; it went through the Edit tool);
   - `host-linux.md` Exit codes / the gate contract: one gate call's output read through a line filter;
   - `ci.md 2026-10-09`: `gh run list --commit` given a wrong sha (mistyped, not short), an empty answer read
     once before the full sha;
   - `testing.md 2026-10-05`: nextest's padded duration broke a fixed-column split of `PASS` lines (93 cases
     read as 72); the count was redone before it was committed anywhere;
   - `testing.md` Test data: three new tests read files after the `TestHome` that held them was dropped (red
     at once on the host; a `stop_keep` fixed each).
9. For the playbook, if Phase 2's escalation confirms a pattern: **an acceptance sentence of the form "X
   nowhere else in product code" is asserted by a grep over the tree, not over the chunk's diff** (the
   `--settings` clause passed implement's report and was caught at the wrap's authoring).

## Counts the masters may state, with their rules (report: Counts / qualifiers moved)

- fake agent argv options +2 and one mode word (architecture's own list is the rule);
- `hook_stdin` fuzz seeds 12 (`ls fuzz/corpus/hook_stdin | wc -l`);
- `InstanceSnapshot` struct literals 12 (`grep -rn -E 'InstanceSnapshot \{' src crates tests --include=*.rs`:
  20 lines, less the type's definition and 7 signature lines);
- nextest verify-driven binary list 13 (the `binary(…)` names on that filter line);
- unit suite 1765 locally; CI totals 2169 / 2159 / 2163 (each run's Summary line, run `38080855246`).

## What Phase 7's light gate should know

- The three `leg = 'operator'` entries are re-verified from `evidence/operator-pass.md`, never re-run: hygiene
  `clean`; the push green twice; the CI read red on `58f872077352` (run `38080061631`) then green on
  `67ab3677cacf` (run `38080855246`, 15/15, attempt 1).
- No `defer`, no `leg = 'live'` or `'round'` entry, no `watch:`. No capability is claimed: the coverage gate is
  a no-op (`claimed 0`).
- `pre-push` takes about 62 s on this host; the whole block about 2 minutes.
