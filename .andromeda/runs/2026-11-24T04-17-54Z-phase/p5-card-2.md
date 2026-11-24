# P5 review card 2 — 2026-11-24-viola-up («`viola up`») · plan rev 7 · research rev 3

**Read the plan:** `.andromeda/cache/docs/2026-11-24-viola-up/plan.md` (rendered from `doc:2026-11-24-viola-up/plan` rev
7). No plan stands in a chunk's folder: writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-viola-up`.

**What folded since card 1** — the overseer's review, `inputs#I2` (a delegate's word, the operator away):
- **Technical answers:** T1, T2 and T4–T7 on their leans; T3's lean for the line's form, its «only existing words»
  sentence corrected (founder-forks.md, revised).
- **Change 1, the live root:** R4's live scratch root is `~/.cache/viola-r4/<stamp>/` — `/home` btrfs (`findmnt`), no
  repository (`git -C ~/.cache rev-parse` refuses) — every real `claude`'s cwd a folder under it; a precondition reads
  it outside every repository. Only the rehearsal stays under `target/r4/`.
- **Change 2, R4b:** what a real session needs under a private config is now read from the committed rig
  (`crates/viola-e2e/src/rig/isolate.rs`, `keys.rs`) and written as his fork **U6** (a credentials copy and its rule,
  2 prompts, 4 real starts, no key). Arm taken, PROVISIONAL: `plant` and `after` **refuse** —
  `r4: red precondition=r4b-not-granted`, nothing started, nothing copied — and the script names neither
  `.credentials.json` nor `claudeAiOauth` anywhere. Two new entries prove it (entries 11 and 12; entry 12 controlled by
  hand both ways, `control-r4b-grep.md`). The gated entry (now 16) still needs BOTH green lines, so v2-12 cannot verify
  until a revision built on his U6 answer runs R4b.
- **Change 3, R4a's agent:** the stand-in built by a new operator entry, `CARGO_TARGET_DIR=target/r4-agent cargo
  build --features fake-agent --bin viola-fake-agent` (its own target dir: the release-shaped `target/debug/viola` is
  never swapped); the script copies it to `<root>/bin/claude`, reads it present and answering `--version`, and proves
  no real `claude` is reachable on the children's `PATH=<root>/bin:/usr/bin:/bin` before the first start.
- **Change 4:** the new words `would-open`, `not-taken`, `boot-time-unread`, the boot-time pair and the `not-live`
  hint are his fork **U7**, the plan's forms taken PROVISIONAL.
- **Change 5:** R4a's two workspaces are arguments of the grant (`R4_WS_PARENT`, `R4_WS_CHILD` on the operator
  entry); 7 and 8 stay only inside the rehearsal's fakes. Three windows titled `r4p/r4c/r4h<stamp>`, about 20 s, closed
  by the script.
- **Change 6, the first `up` after a Claude Code update:** read from the code and written as his fork **U8** —
  with `verify_new_cli` on, the first instance reads `not-live` while its verify runs past the 30 s wait (its window
  stays and goes live when the verify ends); every later start finds the verify lock held and comes up **unverified**,
  reading `live`; `up` exits 1. With the key off (its default) every start is unverified and live. Arm taken: leave it
  and say so in the hint (no bound, no new act). It joins the report's «unproven» lines.

## Goal (unchanged in substance)
`viola up [<name>…] [--dry-run] [--json]` re-opens the instances whose heartbeat is older than this boot and whose log
holds no `session-end` after the newest `session-start` (or named), each in a new `foot` running the pinned
`viola revive <name> --origin <cli|agent>`, its window moved by revive's own window step to its recorded workspace;
parents first, live and stale skipped, one line each, sequential under `OPEN_LIVE_BOUND`, CLI-only.

## Top acceptance criteria
- Selection, order and skip held by the four remove-the-guard controls (unchanged).
- Each re-opened instance resumes its logged id in place through revive, its window on its recorded workspace or
  `unplaced`.
- One line per instance in U7's words; one `hint:` line when any reads `not-live`; one `--json` document; no snapshot
  or stamp written.
- R4b refuses as built; the script holds no credential path (entries 11–12).
- **matrix#v2-12, GATED on R4** (R4a now grantable; R4b after his U6 answer and a revision).

## Touchpoints (unchanged)
New: `src/cmd/up.rs`, `tests/cli_up.rs`, `scripts/r4-up.sh`. Modified (16), `src/cmd/run.rs` and `src/run/` untouched.

## Test commands — 25 entries (was 22)
Added: 11 (`plant` refuses, red on the absent script: its must-pass reading implement's first firing), 12 (the
script names no credential file: red, exit 2 on the absent script; controlled by hand), 22 (the stand-in agent's
operator build). Re-fired: 10 (the rehearsal, its new workspace operands): red, exit 127 on the absent script. The
rest's baselines stand as card 1 read them. R4a's operator entry now runs under `~/.cache/viola-r4/` with the two
workspace handles; R4b's two entries carry a note that they refuse until the U6 revision.

## Concretization preview — matrix#v2-12 (unchanged since card 1, its gate restated)
- **Now:** After a host restart, one command re-opens every session that was live before it, each resumed in place
  in its recorded workspace and placement, parents before the sessions under them; it skips a session still live and
  prints one line per session.
- **Proposed (claimed GATED):** After a real host restart, one `viola up` re-opens the instances that were live before
  it — heartbeat older than this boot and no `session-end` after the newest `session-start` (U1 arm 1, PROVISIONAL),
  or named — each resumed in place on its logged id through `viola revive`, in a new window moved to its recorded
  workspace (none recorded: `unplaced`), parents first; it skips a live or stale instance and prints one line per
  instance. Proven over the fakes for selection, order, skip, lines and argv; «after a real host restart» and «in its
  recorded workspace» on a real compositor only by R4a's and R4b's green lines in `evidence/r4.md`.
- **Narrower readings, his to rule:** U1 (a graceful shutdown's `session-end`, R4b reads it) and U8 (after a CLI
  update with `verify_new_cli` on, the first instance is `not-live` at the line and the rest come up unverified).
- **Gated criterion** (verbatim, the defer key and the route stamp): `gated: R4, viola up once on a real compositor
  over a planted fleet (R4a) and once after a real reboot over a real parent and child (R4b), at the operator host,
  unlocked, its moments his to name (founder-forks U1-U5)` — R4b additionally waits on U6's answer and a revision.

## Checks re-run after the fold
`gate.py run --dry-run` parsed 25 entries (entry 23's two handles unset, declared: the grant's); `planlint` 0 hits
(12 (d)'s five «unchanged» claims re-read and true); `gate.py scope --lists-only` parsed 19 items; `inputs.py verify`
2 entries unchanged (I1, I2), none uncited; `gate.py baseline` 11 new, each baselined (10, 11, 12 re-fired now).
`target/r4-agent/` is ignored (`git check-ignore`).

## Beyond the plan
- none

Apply? (yes / review / cancel)
