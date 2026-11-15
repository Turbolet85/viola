# Founder forks — 2026-11-15-live-round-on-the-trimmed-product (P4, the away form)

Written while the operator is away (`inputs#I1` item 2: the operator is asleep; these are the founder's own caps and
words, WRITTEN, never answered by the builder or a delegate). Each answer fills ONE line of
`.andromeda/cache/chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/founder-words.md` (plan step 2); no
re-plan. Every live rig refuses a step whose line reads `unanswered`. Counts and versions below were measured this
phase (research.md §Files inspected); none is an estimate.

## Fork 1 — the live-`claude` cap for this chunk (`cap:`)
Measured cost per item, in `claude` SESSIONS started (each one an API-spending start):
| item | plan entry | starts | what it records |
|---|---|---|---|
| `viola verify --record` on a fresh test home (stamps the round's CLI green; records the whole fixture set into a private dir, the box-only screen among it if the live screen is box-only) | 9 | **6** (the print-mode probe + interactive Runs A–E; `src/cmd/verify/typed.rs:1–17`, the 2026-10-14 «one re-stamp costs six live starts»). Also 2 non-session spawns (`--version`, Run B's `claude agents --json`) | the stamp line, the recorded set, `box-only: yes/no` |
| the wrapped start: start readings, name refusals, keys, the ~3600 s hold, killed-wrapper | 10 | **1** (the refusals spawn no child; the hold is one turn in the same session) | v2-03 whole, v2-02's reading, CARRY 3's wrapped side |
| the unwrapped start (CARRY 3's other side) | 11 | **1** | the statusLine command's hits unwrapped |
| the MCP tap (`claude -p` through `viola-mcp-tap`) | 12 | **1** | CARRY 1's re-recorded `Mcp.tools-list-1.json` |
| **total** | | **9** | |

Options:
1. **`cap: 9 starts` (Recommended)** — every item: v2-03 and v2-02 claimable, all three CARRYs read. A red step is not
   re-fired: the cap spends at most 9.
2. `cap: 8 starts` — drops entry 11 (the unwrapped start): CARRY 3 keeps only its wrapped side (hits on one side,
   argv, no settings file) and its live comparison stays a CARRY; or drops entry 12: CARRY 1 stays a CARRY.
3. `cap: 7 starts` — verify + wrapped only: v2-03 and v2-02 still close; CARRY 1 and CARRY 3's unwrapped side carried;
   CARRY 2 closes only if the recorded screen is box-only on `cli: 2.1.302`.
4. `cap: 1 start` — the tap alone (CARRY 1); nothing else live: v2-02, v2-03, CARRY 2 and CARRY 3 carried.
   (`cap: 0` — rehearsals only; every live item carried.)

## Fork 2 — the keys a rig may type into its own pty (`keys: trust … · input-box …`)
Pattern: «Boundary widening» (playbook rules rul-r74hshif, rul-p2zl2kgl): a rig's byte into `viola run`'s stdin or into
the CLI is a crossing of that boundary, and the 2026-10-08 one-start takeover answer and the 2026-10-18 trust-key answer
do not carry over (security history items 4 and 22). Two parts, each yes or no:
- **trust** — entries 10 and 11 need a cwd the CLI trusts. `trust yes`: the cwd is `target/e2e-home/viola-live-*/proj/`
  on the tmpfs, its own git repo, and the rig passes the folder-trust dialog by at most 2 down-arrows and 1 Enter typed
  into its own pty, only while the drained bytes hold the trust literal (the 2026-10-18 form). `trust no`: the cwd is
  an in-tree `target/live-round-cwd-*/`, trusted through the repository root (measured on 2.1.287 only, 2026-11-06; NOT
  measured on 2.1.296, 2.1.302 or 2.1.325); no key; a modal meets the start → killed with no key, the start spent red.
  In-tree also loads this repository's `CLAUDE.md` and project hooks into the live session.
- **input-box** — v2-03's «passes the human's keys unchanged» needs one human key. `input-box yes`: one `x` and one DEL
  typed into the wrapped session's input box (never a dialog), before the send, read back as the echoed `x` and a
  `wheel` line `human`, then `viola release` (a human verb, the rig acting as the human). `input-box no`: that clause
  stays on its fake-agent witness (`tests/tui_passthrough.rs`) and v2-03 cannot be claimed whole — it stays pooled with
  a note.

Options:
1. **`keys: trust yes · input-box yes` (Recommended)** — every reading reachable; the trust keys have a measured
   precedent form; the input-box keys never touch a dialog.
2. `keys: trust no · input-box yes` — no key into a CLI dialog; the in-tree trust is unmeasured on the round's CLI, so
   entries 10/11 may each be spent red on a dialog.
3. `keys: trust yes · input-box no` — v2-03 stays pooled (its keys clause unread live).
4. `keys: trust no · input-box no` — no rig byte reaches the CLI; both risks above.

## Fork 3 — which CLI the round runs (`cli: <version>`)
Measured: this session's PATH `claude` is the builder's shim → `mise` 2.1.296 (`2.1.296 (Claude Code)`); `mise`'s
`latest` is **2.1.325** (it moved past the handoff's 2.1.323); the fixture dir is `fixtures/claude/2.1.302/`. The live
home's stamps (read only): 2.1.296 24/24 (2026-10-19), 2.1.302 19/24 (2026-11-09, four relied rows red under an older
build), 2.1.323 23/24 (2026-11-13, only the retired row red; 3 box-only guards). A fresh test home is stamped by entry
9 whichever CLI is chosen — every choice costs the same 6 starts.
1. **`cli: 2.1.302` (Recommended)** — the only arm that lands files: CARRY 1's re-record replaces the stale
   `fixtures/claude/2.1.302/Mcp.tools-list-1.json` in place, and a box-only screen (if 2.1.302 draws one) replaces its
   2.1.302 namesake. Risk: 2.1.302's last live stamp read 4 relied rows red (under an older build; the 2026-11-13
   readiness fix is after it) — if entry 9 reads red, entries 10's hold and v2-03's `cli_verified:true` go unread, as in
   0.1.0.
2. `cli: 2.1.325` — the newest; unstamped anywhere; the recordings land in no dir (another version's set is a whole new
   fixture set, `tests/contract_ledger_probes.rs:41` pins two): CARRY 1 and CARRY 2 read and carried to a new-set
   entry; v2-02/v2-03 proved on the CLI the founder runs today.
3. `cli: 2.1.323` — stamped 23/24 with box-only guards measured: the most likely green verify and box-only screen; its
   files land nowhere (as 2).
4. `cli: 2.1.296` — the builder's shim; 24/24 under the old row set; no fixture dir; nothing lands (as 2).

## Fork 4 — the 3600 s ceiling's PROVISIONAL mark (`hold:`), answered AFTER entry 10's reading
The reading that settles it: entry 10 — `dialog-expired` and `dialog-answered{deadline_hit:true}` with `duration_ms`
≈ 3 600 000 on a verified CLI (the CLI did not end the 3615 s hook first). Options once it exists:
1. **`hold: stands` (Recommended, on a reading at ≈ 3600 s)** — the value stays 3600; the doc comment of
   `DIALOG_HOLD_MAX` loses «PROVISIONAL: measured live only to about 660 s» (plan step 9); the wrap removes the
   provisional mark from (architecture · decision/dialog-hold) and (architecture · decision/dialog-hold-bound) and
   closes residual res-ujtjay7k.
2. `hold: moves-to-read` (on a reading where the CLI ended the hook before 3600 s) — the ceiling moves to the value
   read: a product change across `DIALOG_HOLD_MAX`'s 6 references and `plugin/hooks/hooks.json`'s two 3615 timeouts,
   carried to a new route entry (it adds files this chunk's scope does not name).
3. `hold: stays-provisional` (no reading: a red verify, a red start, or the cap) — the mark stands, carried.

## Fork 5 — the rig's read of the user's `statusLine` setting (`keys:` line gains `· statusline-read yes|no`)
Pattern: «Boundary widening» (rul-r74hshif, rul-p2zl2kgl): viola reads no status-line source by rule; a test rig
reading `~/.claude/settings.json`'s `statusLine.command` (read only, kept only as a hash prefix, the file's mtime and
sha256 compared before and after) is a new crossing, though the directive names it (`inputs#I1` item 3), and a
direction never ratifies a widening.
1. **`statusline-read yes` (Recommended)** — CARRY 3's comparison reads the command's descendant hits on both sides.
2. `statusline-read no` — CARRY 3 reads only the wrapped child's argv and the absent `settings.json` (no comparison):
   CARRY 3 stays a CARRY.
