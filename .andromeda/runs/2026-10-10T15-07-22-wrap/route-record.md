# Route record — wrap of 2026-10-10-viola-revive (Phase 5)

Ten markerless lines of `viola-0.1.0/working-route.md` changed; no frozen line did. Freight blocks on the tail:
57 before, 66 after (`route.py pins`). No entry was added, removed or reordered.

The placements were asked at the Phase 2 halt, in the same two dialogs as that phase's escalations, because the
master texts name route entries by title. Every option was shown with its price; the answers are `inputs#I8`.

| item | entry | authority | gradient |
|---|---|---|---|
| the `session-live` refusal, with P4 and P12 | "The board: viola list" | the founder, live, 2026-10-10T14:16:38Z, relayed by the operator (`inputs#I5`, point 1) | a recorded direction naming the entry and the disposition |
| the owed `--resume` ledger row, the real resume payload's key set, the unmeasured resume readings | "Paste newline ledger row" | the founder: owed on a route entry (`inputs#I3`, answer 1); the entry: the operator (`inputs#I8`, answer 3) | trajectory, asked |
| `viola send` of `/compact` exiting 13 | "Paste newline ledger row" | the operator (`inputs#I8`, answer 4) | trajectory, asked |
| revive's `--json`; `--list` opening the `cli` role log | "CLI machine contract" | the operator (`inputs#I8`, answers 6 and 2) | trajectory, asked |
| the revive exit causes | "Exit-cause code catalogue" | the founder's ruling of 2026-10-01, on the entry "viola revive" | a recorded direction |
| the killed wrapper's leftover socket file; the harness `cleanup` hypothesis | "Unix endpoint and home hardening" | the operator (`inputs#I8`, answer 5: split by consumer) | trajectory, asked |
| the child-ends-with-its-killed-wrapper reading; the tab-close leg | "Linux and macOS parity" | the operator (`inputs#I8`, answer 5) | trajectory, asked |
| the missing Windows case for `strict::check_instance` | "Home and code-bearing file integrity" | this wrap (the report's one coverage flag) | carry-forward, factual |
| the literal `claude` in `src/cmd/verify.rs` | "Interrupted verify cleanup" | this wrap (an acceptance criterion met with a limit) | carry-forward, factual |

## The reading the operator asked for (answer 5)

This wrap's card said the kill-and-revive case kills a wrapper on both Unix runners "without asserting its child
ended". That was wrong, and it came from reading `tests/chaos_revive.rs` alone. The assertion is in the helper the
case calls, `Wrapper::kill` in `tests/support/home.rs` (the report's listing, row 561-586): it reads the recorded
`child_pid` and that process's start time from the snapshot, kills the wrapper through the PTY seam, waits for the
wrapper's exit, then loops while the child's pid still has that start time, bounded by `WITHIN` (7 s), and fails
with "the child outlived its killed wrapper". The case passed on `macos-latest` and `ubuntu-latest` in
`ci#38061685124` (`evidence/operator-pass.md`: "the wrapper's child was gone, by pid and start time, inside the
case's bound"). So the reading exists on Unix with the fake agent. What the CARRY on "Linux and macOS parity" says
is unmeasured: a real CLI on Unix, and the standalone pin in `tests/run_cli.rs`, which stays `cfg(windows)`.

## Other tail edits

- "CLI output discipline": its CARRY quotes two obs-plan sentences this wrap amended for `revive`. A
  `[premise-corrected: …]` note inside the CARRY gives the sentences as they stand now; both still wait on that
  entry for `verify` and the other verbs.
- Citation digits (the sweep's two `route` rows and one naked continuation, each read at the cited line):
  "Paste newline ledger row" `src/bin/viola-fake-agent.rs:378:16`, `:378:38` → `:404:16`, `:404:38` (line 404 holds
  the two `!`), `:97:17` → `:101:17` (line 101 holds the `"--screens"` arm); "Windows-only live measurements"
  `src/cmd/run.rs:385:5` → `:415:5`. `src/cmd/verify.rs:284:9` on the first entry was not moved: that file is
  unchanged since the base.

## Checks

- `BLOCKED-ON` on "Windows-only live measurements": the premise is an interactive Windows host. `uname -s` read
  `Linux` at 15:47Z. Standing; not this chunk's line and not the head.
- No `gated` record (`route.py cursor`: gated 0). No gate deferral and no `WATCH:` in the report, so no PREREQ or
  watch was pinned or migrated. The pending line carries three CARRYs and no standing pin.
- Epoch growth: Epoch 4 stands at 10 entries (5 complete, 1 pending, 4 markerless; `route.py epoch`). No boundary
  is minted inside it, on the founder's word of 2026-10-09 (carried).
- Not placed on the route, carried in the handoff for the founder: his word on the session id as a child argv
  value; the dev host's bare `claude` at 2.1.289.
