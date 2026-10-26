# Scope — Home and code-bearing file integrity (2026-10-26-home-and-code-bearing-file-integrity)

**Working entry (viola-0.1.0/working-route.md, Epoch 6 — Security hardening):** Home and code-bearing file integrity —
owner-only home, DACL check before any diagnostics file, pinned-binary re-hash, per-start plugin rewrite, single
stamp/snapshot writers.

**Invocation:** the operator's `/andromeda-phase Proceed.`, with the overseer's directive for this phase (`inputs#I1`):
forced sample at P2 (`detail.py sample --side phase --force`), the two-draw classify's first live run, every subagent
and Workflow transcript and the run dir kept, a hard cascade failure (or agents writing nothing, or doing the
orchestrator's work) stops the phase with no re-spawn around it; the founder's own forks — the readers' move onto the
log replay (fork CARRY 7/8) is his to rule at P4 as a live dialog, and any boundary widening halts for his live word;
standing: no epoch boundary inside Epoch 6, `df -h /tmp` read before a stamped-home gate.

## What it builds
security.md §Local trust boundary's home rules made whole at every entry point, and the end of the dated strict-modes
exceptions the founder ratified "until the Epoch 6 entry 'Home and code-bearing file integrity'".

- **Owner-only home.** Every directory under the home created 0700 and every file 0600 explicitly on Unix; on Windows
  a protected owner + SYSTEM DACL. The working entry states it; what already stands (`viola_state::fs::create_private_dir`,
  `replace_private`, the Windows DACL at creation for a home outside `%USERPROFILE%`) is P3's to read.
- **The strict-modes check before any read or open of the home** at every entry point — the check before the open,
  never after it (a role-file open narrows an existing home to 0700, so a check made after it passes a home at 0770;
  CARRY 14). The entry-point set this chunk widens to: `send`, `wait`, `last`, `answer`, `pause`, `release` (CLI, each
  calling `client::start` first today: send.rs:190, wait.rs:43, last.rs:30, answer.rs:94, pause.rs:32, release.rs:35);
  `viola hook`'s nine events, event and dialog paths (its `viola_obs_init` at hook.rs:104 before any check, its snapshot
  read at hook.rs:204, failing open); `viola mcp`'s frame tools (`frame`, tools.rs:323, before `live_endpoint`);
  `verify` (its `cli` log at verify.rs:65 and `update_stamps` at verify.rs:264); `run` (no check on its path today; its
  log opens first at run.rs:164) over the home, `instances/<name>/`, `bin/`, `plugin/`, `ledger/` and the trusted files.
  [premise-corrected: `ui/` dropped — no `ui/` resource exists since the 2026-10-16 GUI cut (research.md Sweeps: 0 hits);
  the read rule `mode & 0o077` on the home and `instances/<name>/` (security-plan detail authentication-authorization.md
  line 38) is not in code: `unix_verdict` refuses `0o022` only (strict.rs:76–84), so this chunk builds it]
- **DACL check before any diagnostics file** (working entry): the Windows owner + DACL read before
  `instances/<name>/diagnostics/detail-*.ndjson` (and any role file) is opened. Verified at P3: `open_role_file`
  (obs.rs:70) and `write_detail` (obs.rs:310) open with no check on either OS and narrow an existing dir first on Unix;
  the Windows proof of `check_instance` (CARRY 12) and the seeded-home question (CARRY 13) are its witnesses.
- **Pinned-binary re-hash** (working entry): `bin/<version>-<hash>/viola` re-hashed (truncated SHA-256) before reuse;
  verified at P3: `pin_exe` re-hashes an existing copy and refuses on any difference (pin.rs:71–85); the home copy of
  the binary is also this chunk's (CARRY 16, the `pin_exe` race) and so is the measurement of whether a rebuild of one
  tree can read as another binary (CARRY 11).
- **Per-start plugin rewrite** (working entry): `plugin/` rewritten atomically on every start, every exec-form `command`
  the absolute pinned path. Verified at P3: `pin_and_plugin` rewrites `plugin/<key>/` whole at every start through
  `replace_private_shared` (run.rs:415–420). [premise-corrected: the `marketplace/` tree (CARRY 17) is off the integrity
  set by the founder's own word (his Epoch 5 batch, 2026-10-20, security-plan-amendments.md entry
  2026-10-20-t06-58-31-wrap, superseding the 2026-10-17-optional-plugin-install mark); nothing of it is built here]
- **Single stamp/snapshot writers** (working entry): only the wrapper writes `snapshot.json`, only `viola verify` writes
  `ledger/stamps.json`. Verified at P3 by the graph and a writer grep: `update_stamps` has one product caller
  (verify.rs:264) and `write_snapshot` one product caller module (src/run/snapshot.rs:31, :47); no test pins either
  single-writer fact yet.

## Boundaries
- Unix socket and socket-directory hardening, the client euid check and macOS's server verification are "Unix endpoint
  and home hardening"'s (its own freight); this chunk does not build them.
- Bounded readers (`take(MAX_FRAME)`) are "Bounded inputs at every boundary"'s; error surfaces "Sanitised error surfaces"';
  a new exit-cause code "Exit-cause code catalogue"'s.
- Windows live readings that need an interactive Windows host stay on "Windows-only live measurements" (its BLOCKED-ON);
  this chunk's Windows proof is CI's `windows-2025` leg.
- No boundary is widened without the founder's live word (`inputs#I1` item 6). A widening met on the way — a reader added
  off the trusted-file list, a verb that reads a home file on a new basis, a refusal that now writes a line — halts for it.
- The readers' move onto `read_snapshot_or_replay` (CARRY 19) is the founder's to rule at P4 by live dialog
  (`inputs#I1` item 6; `inputs#I2`, rulings.md:699: option 1 for «Server verification before any frame» is his word, the
  move "stays re-pinned on «Home and code-bearing file integrity»"). Until his answer it is neither planned in nor out.
- Decided at P4 by the founder's own answers (`inputs#I3`, 2026-10-26T18:11Z, relayed by the overseer): the four readers
  move onto `read_snapshot_or_replay` in this chunk (his ratification of that boundary widening); a refused check at
  every new entry point writes no line, as `link` / `unlink`; the six frame verbs print `link`'s instance pair; CARRYs 11
  (build reproducibility) and 16 (the `pin_exe` race) leave this chunk for a new entry «Pinned copy race and
  reproducible builds» right after this one in Epoch 6 (his route-split word; the P5 card's Beyond-the-plan row).

## Folded freight (working-route.md, the entry's own line — nineteen CARRYs; each a hypothesis until P3 re-reads it)
1. CARRY (chunk 2026-09-28-capability-ledger-and-viola-verify; founder ratifications 2026-09-28 12:39:40 and 20:24:32,
   security-plan Decisions Log `2026-09-28`, stamps read): `viola verify`'s `update_stamps` reads `ledger/stamps.json`
   without the home strict-modes check; this entry adds that reader to the strict-modes entry-point set, closing that
   dated exception [premise-corrected upstream: `run`'s version gate left it with chunk 2026-10-04-dialog-answers-by-dialog-id,
   reading through `read_stamps_strict`]. → owner-only home / strict-modes.
2. CARRY (chunk 2026-09-24-three-os-ci-headless-harness-skeleton): `viola run` creates the home and `diagnostics/` with
   Unix 0700/0600 only, no strict-modes check before opening the role file (test-plan §3 boot step 2, security-plan
   §Anti-Patterns Data Protection state the target); this entry keeps the per-entry-point checks of the home,
   `instances/<name>/`, `bin/`, `plugin/`, `ui/` and the trusted files, and test-plan §6's Windows `RUNNER_TEMP` negative
   case [premise-corrected upstream: the Windows protected user + SYSTEM DACL for a home outside `%USERPROFILE%` landed
   with chunk 2026-10-04-dialog-answers-by-dialog-id, `viola_state::fs::create_private_dir`, pulled forward by the
   founder's live ruling of 2026-10-04 ~16:40Z, relayed by the overseer].
3. CARRY (chunk 2026-09-27-hooks-to-normalised-events; operator ruling at its P4 fork 2; the interim gap ratified by the
   founder, live, 2026-09-28, Decisions Log `2026-09-28`): `viola hook` reads `snapshot.json` through `VIOLA_DIR`,
   shape-checked only, no canonicalisation and no home strict-modes; this entry runs the strict-modes check in the hook
   client before that read, failing open on a failure, closing that dated exception.
4. CARRY (chunk 2026-09-28-hook-perf-gate): `tests/cli_controls_not_disableable.rs` stands in its interim shape; this
   entry adds its 0770 `--home` negative (exit 21), then the table's completeness case — every `VIOLA_*` var, global flag
   and `config.json` key present — once the `send`, `answer` and wheel negatives stand (test-plan §5 CLI).
5. CARRY (chunk 2026-09-29-sideloaded-conpty; founder live ratification 2026-09-29 10:41:12, relay the Viola overseer,
   Decisions Log `2026-09-29`): on Windows x64 `viola run` loads `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}`
   under a `FILE_SHARE_READ`-only held handle and a full SHA-256 re-hash, without the owner/DACL check; this entry adds the
   `conpty/` folder and both files to the Windows strict-modes set, closing that dated exception.
6. CARRY (chunk 2026-09-29-sideloaded-conpty, the operator's ruling at its wrap, the overseer agreeing): on Windows x64
   the root tests' `seed_conpty` creates a test home before viola runs (test-plan §5 carve-out); once viola sets its own
   home DACL at creation, a seeded home would skip it, so the seed must then let viola create the home first.
7. CARRY (chunk 2026-10-04-confirmed-send-with-cl-1-records; the founder's ruling, live, 2026-10-04, the fourth dated gap):
   `viola send` reads `snapshot.json` (`endpoint`, `pid`, `started_at`) and the heartbeat without the strict-modes check;
   this entry adds `send` to the strict-modes entry points and removes that gap.
8. CARRY (chunk 2026-10-04-wait-and-last, the fifth dated gap): the same for `viola wait` / `viola last`.
9. CARRY (chunk 2026-10-04-dialog-answers-by-dialog-id, the founder's ruling F1, the sixth dated gap): the same for
   `viola answer`.
10. CARRY (chunk 2026-10-04-the-wheel, the founder's ruling F-W1, the seventh dated gap): the same for `viola pause` /
    `viola release`.
11. CARRY (chunk 2026-10-09-inner-cr-and-crlf-in-a-sent-text; verified at P3 as an owed measurement, its cause not
    re-derived — the measurement is an implement step, harness rule 2026-10-09; owner named by the operator's answer at its wrap,
    `chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/inputs/I8-relay-2.md.txt`): a release build's file hash is not
    explained by its source text. Measured on the Linux dev host (that chunk's `evidence/live-preconditions.md`): the
    build was repeatable for a given source text (three builds at sha256 prefix `4057b91d84010eca`), the hash moved with
    comment text alone, and the first build's file (`b4659b98029d0f94`, 4 000 408 B) was not reproduced when its comment
    text was put back (`b4fc939b7085eb39`, 4 000 416 B); the files differ in the linker's build-id note and three
    compiler-named local symbols, `.text` `.rodata` `.data` `.data.rel.ro` `.eh_frame` `.gcc_except_table` hash equal.
    "The cause is not known, and no record guesses one." For this entry: whether a rebuild of one tree can read as
    another binary is this entry's to measure (two clean release builds of one tree, compared by file and by section)
    and the founder's to act on.
12. CARRY (verified at P3: no `cfg(windows)` `strict_instance_*` case stands in strict.rs; chunk 2026-10-10-viola-revive,
    pinned by its wrap as the entry that owns the DACL check):
    `viola_state::strict::check_instance` is proven on Unix only (five widened modes and two root cases, all
    `cfg(unix)`); no case widens the DACL of an instance tree on Windows. On `windows-2025` the revive cases ran over
    trees `viola run` had just made and none was refused.
13. CARRY (verified at P3 in source, the DACL itself unread: `seed_conpty` makes the home with `create_dir_all`,
    tests/support/home.rs:299, which on `windows-2025` inherits BUILTIN\Users write — measured on the mutation scratch at
    ci#38375200217 — the entry `check_instance` refuses as `Writable`; chunk 2026-10-10-statusline-pass-through; owner
    named by the operator's answer at its wrap,
    `chunks/2026-10-10-statusline-pass-through/inputs/I6-relay-1.md.txt`): on Windows x64 a home the root tests'
    `seed_conpty` made (`create_dir_all`, before viola runs) is refused by `check_instance` — three `hook_statusline`
    cases on `windows-2025`, CI run 38080061631, while the stamped-home case passed. "The refused home's DACL was not
    read, so which entry of it the check refuses is not known." Owed here with the check's own Windows proof: read that
    DACL, and say whether a home a test made first is one the product must refuse.
14. CARRY (the narrowing mechanism re-derived at HEAD: obs.rs:43–45, 76–78; chunk 2026-10-10-budget-governor, pinned by
    its wrap): opening a role file sets an existing home to
    0700 (`open_role_file` → `create_private_dir`), so a strict-modes check after it passes a home at 0770 (measured on
    `revive` and the statusline arm). Every other verb that opens a role file runs no instance check today and narrows an
    existing home the same way: `client::start` for `send`, `wait`, `last`, `answer`, `pause`, `release`, `hook`'s nine
    events, `verify` and `run` ("read in source, not measured on those verbs"). The check this entry adds goes before the
    open. "Not measured: the two fixed orders on Windows (both witnesses are `cfg(unix)`), a home another user owns, a
    home with only an other-writable bit." Open for the founder: a refused check writes no line on either arm (obs-plan
    §10, exemption 7), and the wrapper refusing a driver on `budget.json`, NOT ratified. [premise-corrected: the
    `budget.json` refusal is founder-ratified, his own answer of 2026-10-13 (security-plan detail
    authentication-authorization.md line 38); the existing refused checks' no-line shapes are his, accepted as built in
    his Epoch 5 batch (2026-10-20); a new entry point's no-line refusal is not named by that batch and follows his own
    P4 form for `link` / `unlink` (chunk 2026-10-26-explicit-session-links)]
15. CARRY (chunk 2026-10-17-mcp-server-for-drivers, its P5 card's row 4,
    `chunks/2026-10-17-mcp-server-for-drivers/inputs/I3-relay-3.md.txt`): the eighth dated gap's other half — `viola mcp`'s
    frame tools read `snapshot.json` through `live_endpoint` without the strict-modes check; this entry adds them.
16. CARRY (verified at P3 as owed, the cause still a hypothesis: `replace_private_retrying` renames the temp file over
    the pinned path while its write handle is open, fs.rs:289–297, a source reading research.md names, not a measurement;
    chunk 2026-10-17-mcp-server-for-drivers, the operator's direction at its wrap,
    `chunks/2026-10-17-mcp-server-for-drivers/inputs/I5-relay-1.md.txt` and `…/I6-relay-2.md.txt`): the `pin_exe` race —
    two first `viola run`s of one version on a fresh home both find `bin/<key>/viola` absent and replace it; one agent's
    first hook spawn then fails (receipt `ran:false`) and its `session-start` never lands (measured: 6 of 24 concurrent
    unstamped two-instance harness boots failed, 0 of 24 single-instance, `evidence/unstamped-boot-race.md`; "the kernel
    error is not read — hypothesis: `ETXTBSY`"); the test-side workaround (Path 7 boots one instance) stands; this entry
    owns the home copy of the binary.
17. CARRY (chunk 2026-10-17-optional-plugin-install, its P5 card's row 2, the operator's own word,
    `chunks/2026-10-17-optional-plugin-install/inputs/I3-relay-3.md.txt`): the `<home>/marketplace/` tree `viola plugin
    install` writes against a same-user writer — an unknown entry in it is left as found, and the installed `.mcp.json`
    `command` (the canonical exe that ran the install, not the pinned copy) gets no re-hash.
18. CARRY (chunk 2026-10-26-server-verification-before-any-frame, its P5 card's row 4, the word
    `chunks/2026-10-26-server-verification-before-any-frame/inputs/I4-relay-2.md.txt`, delegate the Viola overseer,
    founder-delegated): the dated gaps of CARRYs 3, 7–10 and 15 now hold their strict-modes half only — on Linux and
    Windows each of those clients verifies the serving process's pid and start time before its first frame; this entry's
    strict-modes check before each snapshot read still closes each gap.
19. CARRY (chunk 2026-10-26-server-verification-before-any-frame, its P5 card's row 1; fork CARRY 7/8 ruled by the
    founder's own word, option 1, by dialog at his wake on 2026-10-26, `inputs#I2`, relayed by the overseer): every
    snapshot reader keeps `read_snapshot`, so an unreadable or newer `snapshot.json` still reads as absent at every
    client; the readers' move onto `read_snapshot_or_replay` (`verification-matrix.json#v1-41`, test-plan §6 E5 steps 1–3 —
    the three product readers `src/cmd/hook.rs` once, `src/cmd/hook/statusline.rs`, `run`'s collision check, and
    `viola_state::liveness::live_endpoint`) is this entry's, the classified read taken only after the `check_instance` this
    entry adds before those same reads; a replayed snapshot carries no `pid` or `started_at`, so no client can write a
    frame on it. → the founder's P4 fork (Boundaries).

## CI read at Setup (Setup 5a)
- `99df45362ac6` (the last wrap's commit, the only sha from the flip through HEAD): at Setup `verdict not yet available`
  (ci#40363908098 in progress, trail `.andromeda/runs/2026-10-26T16-59-13-phase/ci-no-marker.json`); re-read at P3:
  `verdict: green · checks 15/15 · wall 686 s` (trail
  `.andromeda/runs/2026-10-26T16-59-13-phase/ci-2026-10-26-home-and-code-bearing-file-integrity.json`). Nothing to fold.
