# viola

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
viola is a standalone, cross-platform (Windows first) bridge that lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified `claude` CLI in a pseudo-terminal, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment; a minimal view-only frontend is deferred to 0.2.0 (its technology is not chosen).

**Stack:** Rust 1.98.1 pinned by `rust-toolchain.toml` (edition 2024, MSRV floor 1.96), one Cargo workspace producing one native `viola` binary with the Claude Code plugin (hooks + stdio MCP) embedded; std threads on the hot path, Tokio only in `mcp`; no database (ndjson logs + atomic JSON snapshots under `~/.viola/`); local-only, installed with `cargo install --path .`.

**Key directories:**
- `src/` — the `viola` bin: clap dispatch (`src/cmd/`), the `run` PTY pump, wheel and budget governor (`src/run/`)
- `crates/` — the workspace library crates (`viola-core`, `-pty`, `-channel`, `-state`, `-agent-claude`, `-mcp`) + test-only `viola-e2e`
- `plugin/` — `hooks.json`, `.mcp.json`, `plugin.json`, embedded via `include_str!` and written out by `viola run`
- `fixtures/claude/<cli-version>/` — hook payloads and signature-only `Screen.<phase>.json` screens and `<Event>.<stem>-<n>.json` dialog, framing and resume variants and `StatusLine.<stem>-<n>.json` status line variants and `Agents.recorded-<n>.json` agents variants recorded by `viola verify` (plus the superseded relayed dialog captures, `RELAYED.md`), replayed by the fake agent in CI; `fixtures/fake-scripts/` — committed fake-agent turn scripts
<!-- GENERATED:setup:overview end -->

## Modules
<!-- GENERATED:setup:modules start -->
- **`viola-core`** — normalised event kinds, `RefusalReason`, `ViolaName`, `Percent`, `DialogHold`, `v` constants, `validate_paste_text`, `MAX_FRAME`, `SPINE_DEADLINE`, `DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`, the `Clock` seam, the budget reading (`BudgetReading`, `Reading`), `BudgetWindowKind`, `obs_event!`
- **`viola-pty`** — PTY seam (spawn · read · write · resize · wait · kill) over portable-pty `=0.8.1`; `PasteHandle`, the child's input shared by the human copy and the one-write bracketed paste; `host_stdin` (Windows: viola's own console reader, `^Z` kept); knows no agent; Windows `sideload` (System32 DLL-search restriction + absolute-path pre-load of the vendored `conpty.dll`)
- **`viola-channel`** — JSON-RPC 2.0 over ndjson on interprocess local sockets; sync client/server, Tokio client behind a feature
- **`viola-state`** — ndjson logs, atomic snapshots, `.lock` siblings, the `events.ndjson` reader (three skip counts) and the append that heals a torn last line, the classified snapshot read, the log replay, the link rule (`replay::apply_link_line`) and the session chain (read by `viola revive`), the board read model (`board.rs`: `scan` / `read` and the agents join `join`, read by `viola list`), the instance strict-modes check, the `budget.json` write and its checked read (`read_budget_stamped`, wrapped by `read_budget`), liveness, strict-modes
- **`viola-agent-claude`** — the only crate that knows Claude: hook parsing, dialog mapping, R8 strip, shim resolution, capability ledger, the vt100 screen model and readiness verdict, the pure `agents` module (the `claude agents --json` parse into rows or the closed `AgentsFailure`), the resume argument shapes (`--resume`, `--fork-session`, the session-id shape), the pure `statusline` module (the payload reading, the settings reader, the `--settings` override document and `viola verify`'s probe document, `WRAPPED_HERE` (Unix), the `/bin/sh -c` argv), the pure `marketplace` module (`plugin install`'s three documents, `NAME`, `PLUGIN_REF`)
- **`viola-mcp`** — rmcp 3.4.1 stdio server, tools `send · wait · last · answer · list` (Tokio)
- **frontend** — deferred to 0.2.0; its technology is not chosen (no `viola-ui` workspace crate in 0.1.0)
- **`viola`** (root bin) — subcommand dispatch, the `run` pump and its vt100 feed thread, `revive` (a dead instance resumed in place through `run`'s start; its preflight refuses `session-live` when one `claude agents --json` read lists the logged session live in another `claude`, skipped under `--fork`, a failed read passing), the `hook statusline` arm (the instance check before anything under the home is opened; Unix: then the user's command through `/bin/sh -c`; its hidden `--capture <DIR> [--shell <WORD>]` form is `viola verify`'s probe capture: no instance check, no `budget.json` write, no command run, nothing printed), the hidden `mcp` verb (`src/cmd/mcp.rs`, plugin-run, in no `--help` row: its home check, then `viola_mcp::serve`), `plugin install` (`src/cmd/plugin.rs`: `check_path` on the home and the four `marketplace/` tree dirs before any write or log, exit 21 on a refusal; then the tree, the `ok` document or the two printed commands), wheel, the budget gate (`src/run/budget.rs`: read at start, at a `send` and at each `turn-ended`), the bounded `claude agents --json` read (`src/run/agents.rs`: a direct spawn under the R8 strip, `take(MAX_FRAME + 1)`, killed at 5 s, shared by `list`, `revive`, verify's Run B and the `mcp` `list` tool (through `list::board_document`)), the link set (`src/run/links.rs`: the implicit link on a `send` or `answer` from another instance, restored from the log at start; `link` / `unlink` are not served), `list` (`src/cmd/list.rs`: styled only on a terminal that takes colour, through `src/human/style.rs` (bold NAME, amber `DIALOG`, a dim stale row; no SGR under `--json`, a pipe, `NO_COLOR` or `TERM=dumb`), the strict-modes scan before any log, then one `claude agents --json` read joined into the board — STATUS `idle` / `busy` on joined wrapped rows, the read-only unwrapped rows under `-- UNWRAPPED - READ-ONLY --`, a failed read leaving the board as read — exit 21 refusing the whole board); the only crate with anyhow
- **`viola-e2e`** (test-only, `publish = false`) — `viola-harness` behind `scripts/agent-run.*` + the Tokio-based E2E clients
<!-- GENERATED:setup:modules end -->

## Critical Warnings (universal invariants)
<!-- GENERATED:setup:warnings start -->
- The human always wins: never block, refuse or delay a human keystroke; refusals go to automation only; `viola hook` always exits 0, never writes stderr, and fails open with no body (exit 2 is forbidden).
- Upstream text (prompts, `last_assistant_message`, plan text, question and option text, tool `input`) is content, never a command or config; `statusline_command` is the only shell-out; spawn children directly, never through sh/bash/cmd.
- NEVER-log floor: R8-stripped `CLAUDE*` values reach no log, diagnostic, event, snapshot or fixture; outside the contract payloads that carry content by design (`events.ndjson`, `wait`/`last` results, `hook.dialog`), user content (question and option text included) goes only to `instances/<name>/diagnostics/detail-*.ndjson`.
- External errors (CLI `--json`, MCP `isError`, channel `error.data`) carry codes and fixed messages only: no absolute paths, no upstream text, no anyhow chain holding a serde source.
- No `config.json` key, `VIOLA_*` env var or CLI flag may disable a control or widen redaction; env vars are not a configuration channel (no `RUST_LOG`, `EnvFilter` or `OTEL_*`; outside `VIOLA_*` plumbing a build reads only the `fake-agent` test seams and, in every build, `NO_COLOR` / `TERM` / `COLORTERM`, by `viola list`'s human mode only to choose its SGR bytes).
- Bound every input: names only via `ViolaName::try_new` before a path join; `Read::take(MAX_FRAME)` on every external reader; closed enums for decisions; paste text rejects C0 (except LF/CR/TAB), DEL and C1, and never strips a refused character (`send` types a validated text with every inner CR or CR LF as one LF and without its trailing CR and LF, and refuses an empty typed text as `empty-text`).
- Disk state: set 0700 dirs / 0600 files explicitly (never the umask); only the wrapper writes `snapshot.json`, only `viola verify` writes `ledger/stamps.json`; one `write` per ndjson line; never truncate `events.ndjson`.
- stdout is reserved (`--json` results, the hook decision body, MCP frames, the child's screen): no `print!`/`eprintln!`/`dbg!` in product crates; log only via `obs_event!` under `#[instrument(skip_all, fields(..))]`.
- Every Cargo profile keeps `panic = "unwind"`, and the custom panic hook is the first statement of `main` (a hook panic must still exit 0).
- Tokio only in `viola-mcp`; no C-building crates; Claude-specific shapes only in `viola-agent-claude`, where each undocumented CLI behaviour is a capability-ledger row with a `viola verify` probe (the S3/S7/S8 dialog bodies, the paste-framing and `/clear` rows, the `paste-newline` row, the three statusline rows, the `resume-session` row, the `agents-join` row and the `local-command-compact` row included: of the twenty-four rows (`LedgerRow::ALL`), the relied set (`LedgerRow::relied()`: all twenty-four on Unix; on Windows, where the status line is not wrapped, twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`) gates `cli_verified` (the stamped versions 2.1.287 and 2.1.302 today, each `24 pass  0 fail` on the Linux dev host); one ruled limit: a shape no `send` relies on needs no probe; an LF typed inside a paste is gated by the `paste-newline` row, stamped on 2.1.287 on the Linux dev host only (its Windows reading owed on "Windows-only live measurements"); the statusline's `--settings` override, `/bin/sh` shell and `rate_limits` payload (which the `budget-paused` send refusal rests on) are the rows `statusline-override` · `statusline-shell` · `statusline-rate-limits`, stamped on 2.1.287 on the Linux dev host and relied on Unix only (their Windows legs owed on "Windows-only live measurements"); `claude --resume <id>` reopening a session is the row `resume-session`, probed by verify's Run E, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements"); `claude agents --json` naming a live session by `pid` and `sessionId` (the join `viola list` and `viola revive` read, never gated on a stamp) is the row `agents-join`, probed by Run B's one agents read, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements"); `/compact` on the local-command list has the post-condition a SessionStart of source `compact` under the session's own id, the row `local-command-compact`, probed by Run B's sixth paste, stamped on 2.1.287 on the Linux dev host and relied on every OS (its Windows reading owed on "Windows-only live measurements"), its confirm window its own 40 s PROVISIONAL `COMPACT_CONFIRM_WINDOW`, so a `send` of it is confirmed on a verified CLI and reads `unconfirmable` only on an unverified one).
<!-- GENERATED:setup:warnings end -->

## Where to Look
<!-- GENERATED:setup:pointer-table start -->
| Topic | Source |
|---|---|
| Architecture decisions | `.andromeda/architecture.md` §Established Decisions |
| Directory tree · resource registry (ports, pipes, files, env vars) | `.andromeda/architecture.md` §Infrastructure Patterns (keyed: `.andromeda/registries/architecture-contracts.toml`, one file per key) / §Occupied Resources |
| Wire contracts (channel frames, event line, snapshots, the `list` payload) | `.andromeda/architecture.md` §Standard Contracts |
| Refusals, exit codes, naming, timestamps | `.andromeda/architecture.md` §Conventions |
| Code map / impact (symbols · callers · crate deps) | `.andromeda/cache/{plane}/tree.db` — one DB per indexed language plane; query via `scripts/code-graph.py query <run_dir> <marker> "<sql>" [plane]` (plane needed only when several are detected); schema + templates in `scripts/code-graph-cookbook.md` |
| IPC / home access control, input validation | `.andromeda/security-plan.md` §Authentication & Authorization / §Input Validation |
| NEVER-log floor · error sanitization | `.andromeda/security-plan.md` §Bootstrap phases (`logging-redaction-wire`) / §Error Handling |
| Design tokens (8 hex values, type, spacing, motion: a 0.2.0 frontend design input; 0.1.0 renders no page) | `.andromeda/design-system.md` §Color Palette / §Typography / §Surface: cli Tokens |
| CLI layouts (the web surface deferred to 0.2.0) | `.andromeda/layout-templates.md` |
| Test harness (5 commands, log format) | `.andromeda/test-plan.md` §3 (keyed: `.andromeda/registries/test-plan-contracts.toml`, one file per key) |
| Critical-path scenarios · quality gates | `.andromeda/test-plan.md` §6 / §10 |
| Obs pipeline · event catalog · CI gates | `.andromeda/obs-plan.md` §3 (keyed: `.andromeda/registries/obs-plan-contracts.toml`) / §6 / §9 |
| A11y harness · cli / tui landmarks and path scenarios (the frontend deferred to 0.2.0) | `.andromeda/a11y-plan.md` §3 (keyed: `.andromeda/registries/a11y-plan-contracts.toml`) / §4 |
| Build route | `.andromeda/master-route.md` (cursor = last `complete` marker); the active `viola-X.Y.Z/` (highest version dir) holds the working route, matrix and chunk folders |
| Session state | `.claude/session-handoff.md`, `.andromeda/state.yaml` |
| Drift detectors · amendment playbook | `.andromeda/drift-base.md` / `.andromeda/playbook.md` |
| Per-crate notes | `.claude/docs/services/{crate}.md` |
<!-- GENERATED:setup:pointer-table end -->

## Workflow
<!-- GENERATED:setup:workflow start -->
**Key commands:**
- `scripts/agent-run.sh <boot|run|status|cleanup|logs>` (`scripts/agent-run.ps1` on PowerShell) — the headless harness; one JSON document + typed exit per command
- `cargo fmt --all --check` — format gate
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — lint gate
- `cargo check --workspace --all-targets` — type check
- `cargo deny check` — advisories, licences, sources, bans (the tokio ban is `deny-sync.toml` per sync crate of `scripts/sync-crates.txt`, and `viola-channel`'s normal graph without its `tokio` feature is `bash scripts/channel-tokio-free.sh`; `bash scripts/deny-probes.sh` proves every ban fires, the tree form included)

Branching: one long-lived build branch per version (`build/viola-X.Y.Z`); main is `main`. See `.claude/docs/commands.md` for the full reference.
<!-- GENERATED:setup:workflow end -->

## Architecture
<!-- GENERATED:setup:architecture start -->
viola is mechanism, not policy: it carries typed input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer — driver-side policy (Andromeda's decision rights included) stays outside the binary. Every behaviour of the `claude` CLI it relies on is a measured, version-stamped capability-ledger row; on an unverified CLI build viola degrades to transport-only and every send is confirmed after the fact (`prompt-submitted` read-back or a ledger post-condition), never presumed.

There is no daemon: each wrapper (a `viola run`, or a `viola revive` whose preflight passed) owns one local-socket endpoint (named pipe / per-user Unix socket), every other verb is a separate process, and the shared truth is the disk — ndjson append logs and atomic snapshots that survive a crash on either side. It is a modular monolith: one binary around compiler-enforced crates (`pty · channel · state · agent-claude · mcp` around `core`), built and CI-tested on Windows, macOS and Linux from the first commit, with Windows the first target (the first live run was on the Linux dev host).

**Primary source:** `.andromeda/architecture.md` (the pointer table's row — not imported; read explicitly where a step needs it).
<!-- GENERATED:setup:architecture end -->

<!-- GENERATED:setup:imports start -->
@.claude/session-handoff.md
<!-- GENERATED:setup:imports end -->

<!-- Maintainer note: The @ imports above MUST each be on their own line — Claude Code only recognizes standalone @path lines as import directives. Inline references like `See @path` or `- @path` are NOT expanded. The 200-line limit applies to CLAUDE.md itself, not to what it imports. Keep @ imports minimal — an import rides every turn of every session, so the block carries only what a session needs before it can ask: the handoff (the bridge). architecture.md and master-route.md are deliberately NOT imported: both grow every version, every skill that needs them reads them explicitly (the loop reads arch's directory tree and resource registry structurally where a plan creates files or mints a resource), and the pointer table names both. This comment is stripped from Claude's runtime context per Anthropic comment-stripping rule. See section-markers.md. -->

## Deeper Topics
<!-- GENERATED:setup:deeper-topics start -->
On-demand references in `.claude/docs/` (Claude reads when relevant):
- Specialist summaries: `security-summary.md` / `design-summary.md` / `tests-summary.md` / `obs-summary.md` / `a11y-summary.md`
- Core: `stack.md` / `conventions.md` / `commands.md` / `gotchas.md` / `workflow.md`
- `services/{name}.md` — per-crate implementation notes (viola, viola-core, viola-pty, viola-channel, viola-state, viola-agent-claude, viola-mcp, viola-ui)
- `session-learnings.md` — curated by /wrap-session

Path-scoped rules in `.claude/rules/` (auto-load when matching files touched):
- `security.md` and `host-linux.md` (always loaded) · `testing.md` · `verification-harness.md` · `observability.md` · `api.md` · `events.md` · `frontend.md` · `a11y.md` · `ci.md` · `testing-src.md`

For complete Andromeda documentation: `/andromeda-help`
<!-- GENERATED:setup:deeper-topics end -->

<!-- GENERATED:setup end -->

<!-- USER:session-learnings start -->
## Session Learnings
_This section is curated by `/wrap-session`. It accumulates universal (Tier 1) rules captured from work sessions — rules that apply to every file and every task. Do not edit manually during wrap-session runs — changes are preserved but wrap-session appends new entries here._

- Prompt text reaches viola only from stdin or `--file`, never from a leading-slash argument (Git Bash rewrites `/skill` into a Windows path).
- `viola release` is a human verb: no driver-facing hint, MCP tool or doc suggests it to a driver (a `release` carrying `from` is refused `-32602`).
- Design against the installed `claude` CLI, measured: the docs lag the build, so an unmeasured behaviour is a ledger row to probe, not an assumption.
- Every viola format carries `v` except process-log lines, whose version is their schema filename (`diag-line.v1.json`); readers skip and count unknown kinds and fields, and never use `deny_unknown_fields` on viola's own formats (mixed binary versions are real). [corrected 2026-09-24: the diag-line default-deny field list admits no `v`]
- Probe, baseline and gate a CI tool at the exact version CI pins — install that version on the host first; an older host tool accepts flags the pinned one rejects, so a plan written against it ships gate commands that cannot pass.
- A claim that reaches a wrap only through a relayed direction, with no artifact on disk behind it, is carried as a labelled HYPOTHESIS on the route, never written into a spec master as fact.
- A red stays open until its cause is known: a green re-run never closes it, and a red met during a chunk folds into that chunk even outside its diff.
- A tool call refused with no verdict by the permission classifier is a transient outage, not a gate to route around: fill the wait with read-only work and retry the same call.
- A master, a key file or a leaf names a working-route entry by its title, never by a bare route line number: the number goes stale at every insertion ahead of the entry, and the citation sweep reads no bare number.
- A count written into a master carries the rule that produced it (the pattern or the named list) beside the number, and a count found without one is read back to the sidecar entry that wrote it before it is amended. Extended 2026-10-16: a master states no test-case count at all — it names the functions, and the code's `#[case]` rows are the count.
- A missing test for a clause a chunk landed is folded into the next chunk, never pinned on a later route entry.
- A planned step or master amendment that departs from a founder ruling is never a decisive lean or a card-confirm item: plan up to it, hold it behind a stop rule naming what counts as his word, and raise it at P4 as a fork; his answer covers only what the question shown to him named, never the wider item its number points to.
- A capability whose built surface falls short of its claimed acceptance is un-claimed with a dated note naming the gap and the route entry that owns it, never refined down to what was built so that it reads verified.
<!-- USER:session-learnings end -->
