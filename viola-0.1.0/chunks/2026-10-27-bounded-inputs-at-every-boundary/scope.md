# Scope — Bounded inputs at every boundary

**Marker:** `2026-10-27-bounded-inputs-at-every-boundary` · **Version:** viola-0.1.0 · **Epoch:** Epoch 6 — Security hardening
**Working entry (verbatim title + hint):** Bounded inputs at every boundary — size caps, depth limits, closed types,
names validated before path joins, paste rule client and wrapper side

## Intent
Every input that crosses into a `viola` process from outside it is bounded and validated where it enters: a reader of
an external byte stream or file is size-capped before it buffers, a JSON parse runs at serde_json's default depth, a
decision read from outside is a closed type, a name reaches no path join unvalidated, and the paste rule holds on the
client side and authoritatively in the wrapper. The chunk makes that property hold at every boundary and proves it,
so that the boundary set is one a reader can enumerate and check, not a set of separate chunks' local claims.

## What it builds
- **Size caps** — every external reader (stdin of `hook` and `mcp`, channel frames on both sides, the child's PTY
  output into the vt100 feed, `claude agents --json` and `claude --version` output, the `--file` / stdin text of
  `send`, every file read under the home or the user's `~/.claude` — `config.json`, `snapshot.json`,
  `events.ndjson`, `budget.json`, `ledger/stamps.json`, `statusline-source.json`, `settings.json` — and the status
  line command's stdout) reads through `Read::take(MAX_FRAME)` (or the documented `MAX_FRAME + 1` over-size form)
  before it buffers; a reader found without one is bounded. [premise-corrected: the product census (research.md
  §Sweeps) reads 24 of these sites through `take(…)` already; two readers have no bound — `viola mcp`'s stdin,
  read by rmcp 3.4.1's `AsyncRwTransport::receive` with an unbounded `read_until` (`inputs#I2`), and
  `replace_private_shared`'s whole-file compare read of a plugin file or ConPTY companion under the home
  (`crates/viola-state/src/fs.rs:354`) — and the chunk bounds both; the process's own executable
  (`checked_exe`, `pin_exe`) is read whole and is no external input]
- **Depth limits** — every serde_json parse of external bytes runs at the default recursion limit; no
  `unbounded_depth` anywhere in the product graph. [verified: 0 hits for `unbounded_depth` or
  `disable_recursion_limit` in any manifest or product source; `Cargo.lock` records no features, so no gate holds
  the absence today — the chunk adds the `deny.toml` feature ban and its probe]
- **Closed types** — every decision read from an external input (a hook event kind, a dialog decision, a channel
  method and its params, an MCP tool argument, a config value, a `claude agents --json` row's status) is a closed
  enum or a validated newtype, an unknown value refused or skipped and counted as its format's rule says, never
  passed through as a free string into a decision. [verified: the closed shapes stand as landed — `HookEvent`,
  `DialogKind`, the per-kind `response` enums, `dialog_id` `u64`, the channel params checks, the MCP handlers'
  raw-argument re-validation, `Percent` / `DialogHold`, `AgentsFailure` (the security and arch extracts; the
  history files' landings); the chunk names each in its enumeration and changes none]
- **Names before path joins** — every name that reaches a path join (`instances/<name>`, a `VIOLA_NAME`, a CLI
  positional, a channel or MCP `name` / `driver` / `driven`, a revive `--id`, a pinned key) passes
  `ViolaName::try_new` (or its own closed-shape validator) first. [verified: every product join of
  `instances/<name>` takes a `ViolaName` (research.md §Sweeps); the session id passes `is_session_id`, the pinned
  key comes from `pin_key`, the hook's `VIOLA_DIR` from `instance_of`]
- **Paste rule both sides** — `validate_paste_text` runs in the client before any frame (CLI `send`, MCP `send`) and
  authoritatively in the wrapper before the first wheel read; the refusal is `not-delivered` / `control-character`
  (and `empty-text` for an empty typed text), never a strip. [verified: `validate_paste_text` has six product
  callers — CLI `send`, CLI `answer`, MCP `send`, MCP `answer` and the wrapper's `send` and `answer` (code-graph,
  research.md §Graph impact); landed at chunk 2026-10-04-confirmed-send-with-cl-1-records]
- **Terminal-parser panic** — a panic in the vt100 screen model degrades readiness to `input-not-ready` without
  harming passthrough of the child's bytes to the human's terminal. [verified, one half unwitnessed:
  `src/run/gate.rs:133` writes the human's bytes before it offers the copy and `:46` catches the panic;
  `tests/chaos_feed_panic.rs` witnesses the degrade and a key reaching the child, but asserts nothing about the
  child's output reaching the outer terminal after the panic — the chunk adds that half]
- **The proof** — a unit-level witness per property and per boundary, and an enumeration a reader can check: the set
  of external readers named, each with its cap, depth and type rule, so a new reader without a bound is visible.
  [verified as owed: no test enumerates the readers today (research.md §Files inspected: `tests/contract_lints.rs`
  and `scan_patterns.rs` are the source-walking precedents, neither about readers)]

## Boundaries (out of scope)
- The `Last-Event-ID` / resume-cursor clause of the cap's acceptance: carried to `.andromeda/residuals.md` for 0.2.0 by
  the founder's ruling of 2026-10-16 (the matrix notes of `verification-matrix.json#v1-46`); the CLI half stands
  unweakened.
- Error-body sanitisation (codes and fixed messages only) — the next entry, "Sanitised error surfaces".
- The exit-cause code catalogue — "Exit-cause code catalogue".
- Socket, peer-credential and home hardening — "Unix endpoint and home hardening".
- Any frontend surface — deferred to 0.2.0.
- Windows-only live readings — "Windows-only live measurements".

## Capability
- Candidate claim: `verification-matrix.json#v1-46` (Bounded, validated inputs at every boundary), its web clause out
  per its own notes. [inferred: P4 confirms the claim lean against the cap's requirement and the route; no other
  markerless working-route entry names `v1-46` (re-derived: `command grep -n 'v1-46'
  viola-0.1.0/working-route.md`, 0 lines)]

## Folded freight and observations
- watch: Path 1's CI timeout — `path1_run_start_one_home_in_order` killed at the ci profile's 20 s line on
  `test (ubuntu-latest)` at `ci#40425866096`, read as runner variance and not a product red, the operator's technical
  answer, the overseer, founder-delegated, PROVISIONAL
  (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/inputs/I6-wrap-pinned-copy-directive.md.txt` item 3);
  measured profile on the Linux dev host ≈ 6.07 s, the `stamped_home` fixture 4.88 s or 80 %, the four product starts
  0.93 s, the test's own file I/O 0.16 s; CI green at 13.468 s and 8.219 s, red at 20.089 s
  (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md`); the kill line does not move,
  and a recurrence reopens the owner question for the founder (0/3; since
  2026-10-27-pinned-copy-race-and-reproducible-builds; retires: a recurrence, or 3 green runs). Tallied on this
  chunk's CI reads per the directive (inputs#I1 item 6). [verified against its recorded run, not HEAD: the reading
  stands as the prior chunk recorded it (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md`),
  a runner-only observation whose non-reproduction here is its expected state; the runner-variance cause stays
  PROVISIONAL as written, and this chunk changes no part of Path 1's start sequence]
- CI (Setup 5a): `4b0bf776d33f` — verdict: in progress · checks 15/15 · `ci#40447371065` in_progress (oldest running
  `lint (windows-2025)` at 230 s): verdict not yet available, not folded, not read as green; it counts toward the
  WATCH tally only once settled.

## Operator directive
- The operator's requests for this phase (inputs#I1): an ordinary chunk with no forced sample; keep every subagent
  and Workflow transcript and the run dir; stop and report at a hard cascade failure, or agents writing nothing or
  doing the orchestrator's work, with no re-spawn around it; a boundary widening, a live-session cap or a route split
  is the founder's live word, asked as a P4 dialog held in the away form, a delegate answer recorded PROVISIONAL; no
  epoch boundary inside Epoch 6; `df -h /tmp` read before a stamped-home gate. [verified: inputs#I1 holds these
  six items as written]
- The ruling on the cascade's design group 1 and on the MCP stdin (inputs#I2): a one-off second re-spawn of
  design-cascade-g1, which passed; the rmcp 3.4.1 unbounded `read_until` on `viola mcp`'s stdin is this chunk's
  own subject. [verified: inputs#I2]
