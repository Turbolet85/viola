# Founder forks — wrap of «A new CLI version stamped automatically» (2026-11-20-a-new-cli-version-stamped-automatically)

Run dir `.andromeda/runs/2026-11-20T17-07-44Z-wrap/` · written at P2 Validate. The operator is away (inputs#I5): each
row below is a master row the built change reaches that stands OUTSIDE the rows the phase block's §5 names as reached
and the five rows of the founder's three P4 answers. Per inputs#I5 item 3 none is folded: each takes its most
conservative option — **the row unchanged** — and waits for the founder's own word. Each is named on the wrap's answer
card. A row below that now reads short of or against the built behaviour is said so; the proposer's words are in
`ops-{doc}.md` of this run dir, its disposition line in `fanout-results.md`.

## F1 — architecture `decision/viola-home-resolution`
- Stands: «… viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it.»
- Built: the start's verify child takes the home by `--home`, with `VIOLA_DIR` removed; the claude child still takes it
  through `VIOLA_DIR` alone.
- Options: (1, taken now) unchanged — «its child» read as the claude child. (2) name the verify child's `--home`.

## F2 — security-plan `decision/config-cannot-disable-controls`
- Stands: «No configuration value makes the dialog hold zero or unbounded, and an identity-floor name in claude_env_keep
  has no effect.» — still true.
- Proposed: add «or lifts the unverified CLI gate» (the key never lifts it alone; a green stamp does).
- Options: (1, taken now) unchanged. (2) the longer rule.

## F3 — test-plan `decision/human-never-blocked`
- Stands: «The human is never blocked: human typing still reaches the agent while automation is refused.» (no except)
- Built: during a keyed start's verify no agent exists; a key waits in the terminal's buffer (the founder's P4-F3 named
  the two a11y-plan rows alone).
- Options: (1, taken now) unchanged. (2) one except naming the start's verify window, as a11y-plan's.

## F4 — test-plan `decision/run-checks-before-child-spawn`
- Stands: «… binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then
  spawns the child.» — it now reads SHORT of the built order: the automatic verify (key on) runs after the bind, and
  the trust record is written after it on the final verdict.
- Options: (1, taken now) unchanged. (2) the order as built (architecture `decision/run-start-order` carries it).

## F5 — test-plan `contract/fake-agent-cli/argv-only-options` and `decision/fake-agent-argv-only` (test-only)
- Stand: «… taken from argv only, never from the environment …»; «… from argv alone, no env var …», its except the
  hyprctl files.
- Built: the fake agent also reads a `fake-args` file beside its executable, one argument per line within MAX_FRAME,
  put ahead of argv (argv's own value wins) — the P5 lean the overseer passed for the green fake-agent proof. Both rows
  now read against the built test-only fake.
- Options: (1, taken now) unchanged. (2) the `fake-args` file named in both (the except grows by adding it).

## F6 — obs-plan `contract/run-terminal-output/child-output`
- Stands: «run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a
  revived start writes anything of its own.» — its first clause now reads against a keyed start (its one line and
  verify's output precede the child); its second holds. P4 read obs's run-silent rows as not reached.
- Options: (1, taken now) unchanged. (2) name the keyed start's line and verify's output before the child.

## F7 — a11y-plan, six rows (P4-F1 / F3 named two other a11y rows alone)
- `contract/tui-passthrough/own-bytes` «viola writes zero bytes to the terminal except the child's output …» — reads
  against a keyed start's lines before the child.
- `decision/zero-viola-bytes-on-the-terminal` «The outer-PTY stream holds no viola-originated bytes …» — reads against
  a keyed start's stream.
- `decision/wheel-ignores-terminal-events` «… and viola originates zero bytes on the human's terminal.» — the same.
- `decision/portable-pty-outer-pty-driver` «… checking that viola emits zero terminal bytes of its own …» — a test
  rule; true of the cases it drives (key off).
- `decision/time-bounds-fall-on-the-driver` and `decision/time-bounds-never-on-human` — their closed lists lack the
  start's verify stop grace (30 s, falling on viola's own child); the proposer's sets also dropped «blocking, refusing
  or» / «refused or», and are rejected on that ground too.
- Options: (1, taken now) all six unchanged. (2) each amended to the built behaviour, the time-bound lists growing by
  adding the stop grace by name.

## Not forks — said for the record
- Three operations fit read `not_master` (the code holds them): architecture `decision/stop-during-start-verify`,
  obs-plan `decision/auto-verify-on-run-start-span`, security-plan `decision/start-verify-child-isolated`. Rejected
  before the card; the stop's outward promise stands in obs-plan `contract/run-process-log/auto-verify-stop`.
- §5 rows the built change does not reach are left unchanged and listed in `fanout-results.md` (its last section).
