# viola

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
viola is a hybrid local developer tool that lets one interactive Claude Code session drive another on the user's own subscription. It has a native Linux CLI binary and a Claude Code plugin of hooks and a stdio MCP server that calls that binary; a keyboard-driven terminal fleet console is planned, not built, and there is no web or graphical frontend. It wraps the unmodified `claude` CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment. It is a modular monolith: one binary with internal crates around `viola-core`, and no daemon.

**Stack:** Rust 1.98.1 pinned by `rust-toolchain.toml` (edition 2024, `rust-version` 1.96), one Cargo workspace (`crates/*`, the `fuzz/` workspace excluded) building one native `viola` binary: std threads with Tokio `=1.53.3` only inside `viola-mcp`, clap `=4.6.7`, portable-pty `=0.9.0` behind viola's own PTY seam, vt100 `=0.16.2` for readiness, interprocess `=2.4.4` local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomic JSON snapshots (no database, broker, container or hosting), rmcp 3.4.1 (`>=3.4.1, <3.5`) for the stdio MCP server; CI on GitHub Actions `ubuntu-latest` against the fake agent; deployed by a local `cargo install`, the plugin embedded and loaded with `--plugin-dir`.

**Key directories:**
- `src/` — the `viola` bin: subcommand dispatch (`src/cmd/`), the `run` pump, wheel and budget governor (`src/run/`), human output (`src/human/`)
- `crates/` — the workspace library crates `viola-core`, `-pty`, `-channel`, `-state`, `-agent-claude`, `-mcp` and the test-only `viola-e2e`
- `plugin/` — `hooks/hooks.json`, `.mcp.json`, `.claude-plugin/plugin.json`, embedded in the binary and written out by `viola run`
- `fixtures/` — `claude/<cli-version>/` payloads and screens recorded by `viola verify` and replayed by the fake agent in CI; `fake-scripts/` the committed fake-agent turn scripts
- `scripts/` — `agent-run.sh` (the headless harness), the CI check scripts and the code-graph pipeline
<!-- GENERATED:setup:overview end -->

## Modules
<!-- GENERATED:setup:modules start -->
- **`viola-core`** — depends on no viola crate; the normalised event kinds (a new kind is added only here, never with a Claude-specific name), the refusal reason type, `obs_event!` and the service identity constants
- **`viola-pty`** — the PTY seam over portable-pty, host raw mode through libc termios; depends on no viola crate, knows no agent, takes no thiserror
- **`viola-channel`** — JSON-RPC 2.0 over ndjson on interprocess local sockets; sync client and server, the Tokio client only behind its `tokio` feature
- **`viola-state`** — ndjson append logs and atomic JSON snapshots through one shared persist helper, `.lock` siblings, the strict-modes checks and the board read model (sync, std only, Claude-free); writes the stamps, budget and settings files
- **`viola-agent-claude`** — the only crate that knows Claude: hook JSON and `claude agents --json` shapes, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate, the capability ledger and the Claude flag literals; does no file I/O and takes no `viola-state`
- **`viola-mcp`** — the rmcp stdio MCP server (features exactly `server` + `transport-io`), tools `send · wait · last · answer · list`; the only product crate that enables Tokio, on `viola-channel`'s `tokio` feature, and takes no serde
- **`viola-e2e`** (test-only, no product crate depends on it) — `viola-harness` behind `scripts/agent-run.sh`, `viola-mcp-tap` and the Tokio-based E2E clients
- **`viola`** (root bin) — depends on every member: clap subcommand dispatch, the `run` pump, the wheel and the budget governor, all consuming only normalised events; human output only through its human module; the only crate with anyhow; also builds the test-only `viola-fake-agent`
- **`viola-fuzz`** (`fuzz/`, a crate row but no workspace member: its own cargo-fuzz workspace excluded from the root) — the fuzz targets on a pinned nightly, its own lockfile audited separately, committed synthetic seeds
<!-- GENERATED:setup:modules end -->

## Critical Warnings (universal invariants)
<!-- GENERATED:setup:warnings start -->
- The human always wins: security refusals go only to automation (`send` / `answer`), never to the human; `viola hook` never writes stderr and always exits 0, failing open with no body; the panic hook is the first statement of `main` and every Cargo profile keeps `panic = "unwind"`.
- Upstream text (prompts, `last_assistant_message`, plan text, question and option text, tool `input`) is content, never a viola command or config; screen signatures, harness prefixes and local-command lists come only from compiled `viola-agent-claude` ledger rows.
- NEVER-log floor: `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and every R8-stripped `CLAUDE*` value reach no log, stderr, `events.ndjson`, snapshot, `diagnostics/` or fixture; content-bearing detail goes only to the instance's `detail-*.ndjson`.
- External errors (CLI `--json`, MCP error results, channel `error.data`) carry codes and fixed messages only — no path, pid or upstream text, no anyhow chain holding a serde source; a peer's reply reaches them only as a closed value, and a `thiserror` message interpolates nothing.
- No `config.json` key, `VIOLA_*` env var or CLI flag switches off a control or widens redaction; env vars are no configuration channel (only `NO_COLOR` / `TERM` / `COLORTERM` for `list`'s colour, `XDG_RUNTIME_DIR`, `PATH` and two `fake-agent` test seams are read).
- Bound and validate every input: an instance path only from a `ViolaName`-validated string; every external read capped at `MAX_FRAME`; C0 (but LF/CR/TAB), DEL and C1 refused, never stripped; the wrapper re-validates what a client or schema checked.
- Disk state: the home's dirs 0700 and files 0600 set explicitly (the pinned binary 0700), the strict-modes check before any create, mode set or log open; only the wrapper writes `snapshot.json`, only `viola verify` writes `ledger/stamps.json`; one write per ndjson line, atomic replace for the rest.
- Local trust boundary: no channel frame before the server's euid, pid and start time are verified against a strict-checked snapshot; same OS user only; the socket lives in a verified 0700 per-user directory, never at the temp root or in the abstract namespace.
- stdout is reserved per role (the hook decision body, MCP frames, `run`'s child screen, the CLI's `--json`): no `print!`/`println!`/`eprint!`/`eprintln!`/`dbg!` in the `hook`, `run` or `mcp` paths; events only through `obs_event!` under `#[instrument(skip_all, fields(..))]`.
- Every dependence on an undocumented `claude` behaviour is a capability-ledger row in `viola-agent-claude` with a `viola verify` probe, never an unconditional assumption; Claude shapes only in `viola-agent-claude`; Tokio only in `viola-mcp`.
<!-- GENERATED:setup:warnings end -->

## Where to Look
<!-- GENERATED:setup:pointer-table start -->
The masters live in the store, no file: fire `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py {verb} --root .` with `tree doc:{name}` · `render doc:{name} --section "{heading}"` · `find --doc {name} --text T` · `get {id}`, the id the command prints · an old `get {path}[:{n}]`, which names the fact it stood on (an agent reads a whole master by `read doc:{name}`, in pages).

| Topic | Source |
|---|---|
| Architecture decisions | `doc:architecture` §Established Decisions |
| Directory tree · resource registry (crates, binaries, commands, env vars, sockets) | `doc:architecture` §Infrastructure Patterns (keyed: `rec.py contracts architecture`, one address per key) / §Occupied Resources; the directory tree is the tree's own (`git ls-files`) — the directory-structure key holds rows, no tree |
| Wire contracts (channel frames, event line, snapshots, the `list` payload) | `doc:architecture` §Standard Contracts |
| Refusals, exit codes, naming, timestamps | `doc:architecture` §Conventions |
| Code map / impact (symbols · callers · crate deps) | `.andromeda/cache/{plane}/tree.db` — one DB per indexed language plane; query via `scripts/code-graph.py query <run_dir> <marker> "<sql>" [plane]` (plane needed only when several are detected); schema + templates in `scripts/code-graph-cookbook.md` |
| IPC / home access control, input validation | `doc:security-plan` §Authentication & Authorization / §Input Validation |
| NEVER-log floor · error sanitization | `doc:security-plan` §Secret Management / §Data Protection / §Error Handling |
| CLI colour and words (viola renders no page: no type, spacing or motion tokens, the cli's are the terminal's) | `doc:design-system` §Color Palette / §Surface: cli |
| CLI layouts (no web surface: viola renders no page; the fleet console's surface is owed by «Console scaffold») | `doc:layout-templates` §Surface: cli |
| Test harness (5 commands, log format) | `doc:test-plan` §3 (keyed: `rec.py contracts test-plan`, one address per key) |
| Critical-path scenarios · quality gates | `doc:test-plan` §6 / §10 |
| Obs pipeline · event catalog · CI gates | `doc:obs-plan` §3 (keyed: `rec.py contracts obs-plan`) / §6 / §9 |
| A11y harness · cli / tui landmarks and path scenarios (no web or graphical frontend; the fleet console's a11y scope is owed by «Fleet a11y verification») | `doc:a11y-plan` §3 (keyed: `rec.py contracts a11y-plan`) / §4 |
| Build route | `.andromeda/master-route.md` (cursor = last `complete` marker); the active `viola-X.Y.Z/` (highest version dir) holds the working route and coverage renders; a chunk's plan and report are `doc:<marker>/plan` · `doc:<marker>/report`, read by `rec.py render` |
| Session state | `.claude/session-handoff.md`; the lean state is the `state` view (`rec.py query --root . "select * from state"`) |
| Drift detectors · amendment playbook | the `detector` and `rule` rows: `rec.py find --root . --kind detector` / `--kind rule` |
| Per-crate notes | `.claude/docs/services/{crate}.md` |
<!-- GENERATED:setup:pointer-table end -->

## Workflow
<!-- GENERATED:setup:workflow start -->
**Key commands:**
- `scripts/agent-run.sh <boot|run|status|cleanup|logs>` — the headless harness; one JSON document + typed exit per command
- `cargo fmt --all --check` — format gate
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — lint gate
- `cargo check --workspace --all-targets` — type check
- `cargo deny check` — advisories, licences, sources, bans (the tokio ban is `deny-sync.toml` per sync crate of `scripts/sync-crates.txt`, and `viola-channel`'s normal graph without its `tokio` feature is `bash scripts/channel-tokio-free.sh`; `bash scripts/deny-probes.sh` proves every ban fires, the tree form included)

Branching: one long-lived build branch per version (`build/viola-X.Y.Z`); main is `main`. See `.claude/docs/commands.md` for the full reference.
<!-- GENERATED:setup:workflow end -->

## Architecture
<!-- GENERATED:setup:architecture start -->
viola is mechanism, not policy: it carries typed input, answers dialogs when told to, logs everything and holds the wheel, but never decides what to answer — driver-side policy (Andromeda's decision rights included) stays outside the binary. Every behaviour of the `claude` CLI it relies on is a measured, version-stamped capability-ledger row; on an unverified CLI build viola degrades to transport-only and every send is confirmed after the fact (`prompt-submitted` read-back or a ledger post-condition), never presumed, the one exception a ledger-listed local command whose post-condition is none or not yet measured, or whose CLI is unverified: `ok` with `confirmed: false`, detail `unconfirmable`.

There is no daemon: each wrapper (a `viola run`, or a `viola revive` whose preflight passed) owns one local-socket endpoint (a per-user Unix socket), every other verb is a separate process, and the shared truth is the disk — ndjson append logs and atomic snapshots that survive a crash on either side. It is a modular monolith: one binary around compiler-enforced crates (`pty · channel · state · agent-claude · mcp` around `core`), built and CI-tested on Linux only (`ubuntu-latest`), the Windows and macOS code out of the tree, recoverable at the tag `multi-os-final` (the first live run was on the Linux dev host).

**Primary source:** `doc:architecture` (the pointer table's row — not imported; read by the verbs above that table where a step needs it).
<!-- GENERATED:setup:architecture end -->

<!-- GENERATED:setup:imports start -->
@.claude/session-handoff.md
<!-- GENERATED:setup:imports end -->

<!-- Maintainer note: The @ imports above MUST each be on their own line — Claude Code only recognizes standalone @path lines as import directives. Inline references like `See @path` or `- @path` are NOT expanded. The 200-line limit applies to CLAUDE.md itself, not to what it imports. Keep @ imports minimal — an import rides every turn of every session, so the block carries only what a session needs before it can ask: the handoff (the bridge). architecture.md and master-route.md are deliberately NOT imported: both grow every version, every skill that needs them reads them explicitly (the loop reads arch's directory tree and resource registry structurally where a plan creates files or mints a resource), and the pointer table names both. This comment is stripped from Claude's runtime context per Anthropic comment-stripping rule. See section-markers.md. -->
<!-- This file is the records store's render, written whole by `rec.py harness land` alone (`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py {verb} --root .`): a human's edit inside a region the template marks as the human's is landed by `harness accept CLAUDE.md`, any other edit is shown and discarded by `render --family harness --reconcile`, and until one runs `rec.py check` names the file `DIFFERS`. -->

## Deeper Topics
<!-- GENERATED:setup:deeper-topics start -->
On-demand references in `.claude/docs/` (Claude reads when relevant):
- Specialist summaries: `security-summary.md` / `design-summary.md` / `tests-summary.md` / `obs-summary.md` / `a11y-summary.md`
- Core: `stack.md` / `conventions.md` / `commands.md` / `gotchas.md` / `workflow.md`
- `services/{name}.md` — per-crate implementation notes (viola, viola-core, viola-pty, viola-channel, viola-state, viola-agent-claude, viola-mcp)
- `session-learnings.md` — curated by /wrap-session

Path-scoped rules in `.claude/rules/` (auto-load when matching files touched):
- `security.md` and `host-linux.md` (always loaded) · `testing.md` · `verification-harness.md` · `observability.md` · `api.md` · `events.md` · `frontend.md` · `a11y.md` · `ci.md` · `testing-src.md`

For complete Andromeda documentation: `/andromeda-help`
<!-- GENERATED:setup:deeper-topics end -->

<!-- GENERATED:setup end -->

<!-- USER:session-learnings start -->
## Session Learnings
_This section is curated by `/wrap-session`. It accumulates universal (Tier 1) rules captured from work sessions — rules that apply to every file and every task. Do not edit manually during wrap-session runs — changes are preserved but wrap-session appends new entries here._

- Prompt text reaches viola only from stdin or `--file`, never from a leading-slash argument. [corrected 2026-11-06: the Git Bash reason left with the Windows code; the rule stands on architecture [CLI Conventions]]
- `viola release` is a human verb: no driver-facing hint, MCP tool or doc suggests it to a driver (a `release` carrying `from` is refused `-32602`).
- Design against the installed `claude` CLI, measured: the docs lag the build, so an unmeasured behaviour is a ledger row to probe, not an assumption.
- Every viola format carries `v` except process-log lines, whose version is their schema filename (`diag-line.v1.json`); readers skip and count unknown kinds and fields, and never use `deny_unknown_fields` on viola's own formats (mixed binary versions are real). [corrected 2026-09-24: the diag-line default-deny field list admits no `v`]
- Probe, baseline and gate a CI tool at the exact version CI pins — install that version on the host first; an older host tool accepts flags the pinned one rejects, so a plan written against it ships gate commands that cannot pass.
- A claim that reaches a wrap only through a relayed direction, with no artifact on disk behind it, is carried as a labelled HYPOTHESIS on the route, never written into a spec master as fact.
- A red stays open until its cause is known: a green re-run never closes it, and a red met during a chunk folds into that chunk even outside its diff.
- A tool call refused with no verdict by the permission classifier is a transient outage, not a gate to route around: fill the wait with read-only work and retry the same call.
- A master, a key file or a leaf names a working-route entry by its title, never by a bare route line number: the number goes stale at every insertion ahead of the entry, and the citation sweep reads no bare number.
- A count written into a master carries the rule that produced it (the pattern or the named list) beside the number, and a count found without one is read back to the sidecar entry that wrote it before it is amended. Extended 2026-10-16: a master states no test-case count at all — it names the functions, and the code's `#[case]` rows are the count. Extended 2026-10-26: a count a plan or research states is taken by its master's rule too, never by a wider listing.
- A missing test for a clause a chunk landed is folded into the next chunk, never pinned on a later route entry.
- A planned step or master amendment that departs from a founder ruling is never a decisive lean or a card-confirm item: plan up to it, hold it behind a stop rule naming what counts as his word, and raise it at P4 as a fork; his answer covers only what the question shown to him named, never the wider item its number points to.
- A capability whose built surface falls short of its claimed acceptance is un-claimed with a dated note naming the gap and the route entry that owns it, never refined down to what was built so that it reads verified.
- A timestamp written into any record (a report's date, an evidence or journal stamp) is read from `date -u` at the moment it is written, never estimated or carried forward: the stamp-ahead hook refuses an estimate, and a script that fires no hook lands it unchecked.
<!-- USER:session-learnings end -->
