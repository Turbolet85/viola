# Session Handoff

**Last Updated:** 2026-11-21T23:11:01Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** 2026-11-21-mcp-calls-bounded — every MCP frame tool bounded by a compiled call deadline, its expiry `call-deadline` (session 152)

## Position
- Done: 2026-11-21-mcp-calls-bounded — `MCP_CALL_DEADLINE` (55 s) races every frame tool (`send · wait · last ·
  answer`; `list` is not under it), its expiry `instance-unreachable` / `call-deadline`, a seventh member of
  `UnreachableCause` written by the MCP client alone (the CLI's exit-21 set stays six); the deadline an injected seam
  in tests, Tokio's `time` feature in viola-mcp alone. Live, the rig's new `MB` arm over the fake agent: `MB` 2.1.330
  and `MB8` 2.1.328 — a 50 s `wait` returned whole; `MC` 2.1.330 — a 5000 ms server `timeout` cut the call at 5 s
  (`tool-timeout`). The DEFAULT bound's value stays a READING of the bundles' strings per version, never measured; that
  `viola mcp` outlived the cut call is a reading from ONE start. v2-24 verified at this wrap. CI ci#44956459404 on
  `261e48d5` green 7/7.
- **The paste's missing time bound** (one blocking PTY write in `viola-pty`'s pump) is a FINDING, not folded: pinned
  as a CARRY (`cry-7mgt7ybs`) on «Key after a resize»; its own route entry «Paste write bounded» is the operator's —
  proposed, NOT minted (`founder-forks.md` item 1 of `.andromeda/runs/2026-11-21T22-34-30Z-wrap/`).
- **The pinned product holds none of six mends** (the earlier list stands) until the product re-pin step (no route
  entry; CARRY `cry-fqwjpnqt` on «Version done-check»). The driver's rules «set no scheduled wake» and «never print a
  compiled screen literal in a chat message» stay binding.
- Next: `/andromeda-phase` on «Key after a resize»; it carries `cry-234cbfoa` (the rehearsal red, re-pinned whole: not
  met here) and the paste CARRY.

## Work done
- Wrap `2026-11-21T22-34-30Z-wrap`: report closed (tx 3769–3770); 23 row operations in 5 masters (tx 3773–3781), 4 of
  them on one answer card; 13 leaf findings landed by the tool and 6 leaves hand-written to name bounds by constant
  (tx 3792–3797); curation T3 1; route tx 3803.

## Drift resolved
- Q1 answer card (4 operations: decision/mcp-call-deadline, cause-set, time-faked-through-seams, the `MB` arm's row):
  inside the operator's answers «Вшитый дедлайн 55 с» and «3 базовых + 1 запасной», confirmed by the overseer as
  delegate, PROVISIONAL.
- Rejected: 8 duplicates of one fact offered in a second master; 5 proposer operations as wrong (one of them also a
  duplicate) — a dropped qualifier, `call-deadline` listed under the CLI document, the directive's reading left out,
  the ratified shape missed, a one-start reading stated as measured.
- Fit `not_master` ×3, each followed by main's shorter set: the wait cap's why (readings to the sidecar), the send
  bound (the paste gap to the route), the identical-payloads except (no member counts).
- Re-based off the 2.1.287 reading: architecture decision/mcp-wait-cap's why; leaves `api.md`, `viola-mcp.md`.

## Notes
- **Provisional, owed the founder's word (the epoch batch):** this wrap's Q1 card (`answer-card.md`) and
  `founder-forks.md` of `.andromeda/runs/2026-11-21T22-34-30Z-wrap/` (item 1 the paste entry; item 2 the four
  PROVISIONAL rows). The earlier wraps' lists stand as the last handoffs named them.
- **Route:** Epoch 4 has 20 taken-up entries and 1 markerless; unsplit on the founder's standing word.
- **Unmeasured, named:** the MCP client's default bound's value on any version; `viola mcp` outliving a cut call beyond
  one start; the cross-session row on any version but 2.1.328; the earlier list stands.
- The operator's desk list (no route entry) stands as the last handoff named it.
- **Disk:** read `df -h /home` and `target/` before a full rebuild.
- **Deferred learnings:** the earlier list stands — review with `/andromeda-wrap-session --review`.
- **Host:** python `yaml` absent; PATH's claude 2.1.330 (mise latest), 2.1.328 by path; `bwrap` 0.13.1; Hyprland
  0.56.2. The host's own Claude sessions rewrite `~/.claude.json` about once a minute.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-11-22 09:49:15
