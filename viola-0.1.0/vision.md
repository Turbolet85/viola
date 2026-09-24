# Viola 0.1.0 — vision

_Derived from `viola-0.1.0/intent.md` (overseer-authored, founder-delegated, 2026-09-24). That file is the
authoritative version intent; this is its framing, not a second source._

## The problem this version advances

The founder runs two Claude Code sessions per managed project — an **overseer** that verifies the build and writes
relays, and a **builder** that runs the Andromeda pipeline. Today the founder is the transport between them: relays
are pasted by hand, reviews and forks are answered by hand, skills and `/clear` are typed by hand. With a third
project beside Pulse and Conductor, that hand transport is the bottleneck.

User authority in a Claude Code session comes only from that session's own terminal input, and the platform declined
to supply a way to deliver input into a running session. So one session can drive another only through a bridge that
owns the driven session's terminal input, types at turn boundaries, answers its dialogs through hooks, reports
turn-synchronised events, and hands the wheel to the human the moment they type — on the user's own subscription,
through the unmodified `claude` binary, never the API. A throwaway prototype proved every mechanism on Windows; the
viola repo itself holds no product code yet.

## Who this is for, and why now

The founder, and the overseer sessions that act for them. The prototype already drove this project's own
architecture run, so the mechanisms are known; what is missing is a product that is built test-first, verified
headlessly by an agent, and trustworthy enough to take over the overseer's driving from the prototype.

## What is in (ordered, intent (a)–(g))

- **(a) Infrastructure first** — three-OS CI from the first commit, the headless five-command harness, the fake agent
  with recorded fixtures as the test contract, structured JSON logs and their gates, mutation testing per chunk, the
  quality gates, and a seeded code-graph — all before any feature code.
- **(b) Architecture amendments** the other masters depend on, each folded into `architecture.md` by the wrap
  reconcile of the chunk that implements it.
- **(c) The thin Windows slice** — `run` / `send` / `wait` / `answer` with delivery confirmation and the wheel,
  proven by a live test and by viola driving its own build.
- **(d) The remaining v1 capabilities** — capability ledger and `verify`, budget governor, links, the board, the MCP
  server, the CLI contract and output discipline, crash-safe state, the optional plugin install.
- **(e) Security hardening after the slice** — the IPC endpoint, home integrity, bounded inputs, sanitised errors; two
  items (the SQOS spike, the SHA-256 choice) block specific chunks.
- **(f) The web UI** — the loopback, view-only, authenticated strip-bay page per design, layouts and a11y.
- **(g) Cross-OS completion** — Linux and macOS at parity with Windows against the fake agent.

## What is out

API-key / Agent SDK hosting; the GUI brake routes (v1 asserts only 405); phone/remote view and layouts below 760 px;
public distribution, signing and self-update; a second driven agent and the adapter trait; the OTel SDK, OTLP
export and browser telemetry; real screen-reader runs as a gate; log retention, diagnostics rotation and pinned-copy
cleanup (re-carried as residuals); driver-side decision policy; unmeasured dialog paths (O7, plan approve-with-feedback,
`multiSelect`); cross-session messaging (O3) and `initialUserMessage` seeding (O4); the O2/O1 watch items.

## What "0.1.0 done" means

1. On the founder's Windows host, the overseer drives a builder **through viola itself**: it sends a skill, waits for
   the turn to end, reads the dashboard, answers a dialog, clears between skills — and the founder typing into the
   builder window takes the wheel and pauses automation.
2. Every capability is delivered and verified headlessly by an agent; the harness, fake agent and recorded fixtures
   are the contract, and the real CLI is exercised only by local `viola verify`.
3. CI is green on Windows, macOS and Linux on every commit, with the mutation, coverage, perf, observability,
   accessibility and supply-chain gates holding.
4. A local view-only page shows active sessions, their links and a live event feed, and meets its accessibility label.
5. The masters agree with each other: every cross-plan amendment is reflected in `architecture.md` by the wrap
   reconcile of the chunk that implements it.
