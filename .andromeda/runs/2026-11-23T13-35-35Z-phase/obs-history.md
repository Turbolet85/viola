@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. Log Coverage: 5 rows) — `~ contract/diag-line-fields/process-start` gained subject `hyprctl-probe`, `~ contract/diag-line-fields/parse-rejected` gained parser `hyprctl-json`, `~ contract/parse-rejected-detail/oversize` gained «hyprctl-json past MAX_FRAME», `+ contract/diag-subject/hyprctl-probe`, `+ contract/parse-rejected/hyprctl-json`. Why it matters: this is the seam's whole log surface the chunk extends — a `monitors` query and a move-to-workspace dispatch should ride the existing `hyprctl-probe` pair and the `hyprctl-json` parse-rejected line (oversize / malformed, codes only); a new subject or parser word would need its own row and a Decisions Log entry, and is an operator fork under the scope's boundary.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary: 3 rows) — `~ decision/per-parser-parse-failure-counts` added «the hyprctl replies», `~ decision/instrument-call-sites-not-os-internals` added «the hyprctl spawn», `~ decision/parse-failures-surface-as-unknown` got the except «a hyprctl reply failing its parse reads oversize or malformed with one parse-rejected line, never unknown». Why it matters: the new `monitors` parse and its keep-list must keep the closed error word (never `unknown`) and its own per-parser count, and the new dispatch's spawn is instrumented at viola's call site in the seam, not inside hyprctl.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary: 2 rows) — `~ decision/os-apis-at-call-sites` added «and the hyprctl spawn … never in their internals»; `~ decision/parse-failures-unknown-counter` got the same hyprctl except. Why it matters: the twin rows of the picked scope summary; both bind the new request and dispatch exactly as they bind the existing six.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (2. Telemetry Strategy: 1 row) — `~ decision/span-naming` added area `hyprland`, for the manual `skip_all` span `hyprland.read`. Why it matters: the read whose deadline the chunk settles and times, and whose stdout drain the chunk's new witness drives past MAX_FRAME, is that span; any new span stays in the `hyprland.<operation>` form, `skip_all`.

@history_item n=5 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (1. Obs Scope Summary, two entries) — `~ decision/parse-failures-unknown-counter` and `~ decision/parse-failures-surface-as-unknown` widened the except to its current text: the hyprctl oversize / malformed half plus a StopFailure word outside the CLI's set reading cause api-error. Why it matters: the picked rows' except clause holds two halves; a hyprctl edit that touches the except must keep both whole.

@history_item n=6 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (6. Log Coverage: 2 rows) — `+ contract/parse-rejected-detail/unknown-value` (hook-stdin only) and `+ contract/parse-rejected/hook-stdin-unknown-value`. Why it matters: the picked `unknown-value` row is scoped to hook-stdin; a hyprctl reply holding an unexpected value must still read `malformed` (or the keep-list drops it), not this word, unless the operator forks it.

@history_item n=7 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap) — `CappedLines` at MAX_FRAME: a line of at most MAX_FRAME bytes, its newline counted, passes whole; a longer one is refused whole, never truncated or skipped (the source of the picked `contract/mcp-stdin/line-within-max-frame`). Why it matters: the precedent for the drain-cap witness — the bound sits in the reader viola hands over, tested at exactly MAX_FRAME and MAX_FRAME+1; the `tests/contract_bounded_readers.rs` hyprctl row should get a witness that drives the read's own drain, not only the parse.

@history_item n=8 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (D-05 `parse-rejected` gains the `mcp-stdin` parser) — the parser list moved 9→10, counted by the schema enum's members; «a new closed parser value takes a Decisions Log record», kept in the amendments file. Why it matters: the procedure if the chunk adds any closed word (a parser, detail or subject) for `monitors` or the dispatch; the count rule is the enum's members.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `oversize` gained `claude-agents-json` (an answer past MAX_FRAME); the `agents-probe` spawn pair was reused across three callers with no new subject. Why it matters: precedent for a child-output probe with an oversize cap and a reused subject — the new hyprctl request reuses `hyprctl-probe` and `hyprctl-json` the same way.

@history_item n=10 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — `verify-probe` joined the closed `subject` set by D-35: «a new closed value takes a Decisions Log entry»; spawns are logged at the call site and `run_bounded` stays unlogged. Why it matters: the rule for the `contract/child-spawn-log/subject` and `contract/diag-subject/*` rows if a dispatch spawn is thought to need its own subject.

@history_item n=11 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. Log Coverage: 1 row) — `~ contract/child-spawn-log/subject` changed from «child and shell spawns» to «child spawns»: viola runs no shell. Why it matters: the hyprctl spawn and the new dispatch must stay a direct argv spawn (the scope's «not an `exec`»), never a shell, or this row breaks.

@history_item n=12 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. Log Coverage: 6 rows) — `~ contract/diag-line-fields/process-exit` lost `shell_exit_status`; `~ contract/diag-line-fields/process-start` lost subject `statusline-shell`; `− contract/diag-subject/statusline-shell`. Why it matters: these set the current field sets of the picked process-start / process-exit rows that the hyprctl pair writes; a probe exit carries `child_exit_status` and `duration_ms`, no shell field.

@history_item n=13 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — `~ contract/diag-line-fields/process-start`: the subject became «one word of the diag-line subject enum» instead of a list, the enum held one row per word; `~ contract/process-log/child-and-probe-lifecycle` added verify's print-mode sender. Why it matters: a new subject would be a new `contract/diag-subject/*` row plus the enum word, not an edit to the process-start row; the lifecycle row lists the claude child and probes but not the hyprctl probe, so watch for drift if the seam gains a product caller.

@history_item n=14 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (6. Log Coverage: 2 rows) — `+ contract/diag-subject/auto-verify` and the process-start subject list extended, ratified at 2026-11-22T17-29-41Z-wrap. Why it matters: the latest precedent for adding a subject word, one row per word, under the founder's ratification.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary: 1 row) — `~ decision/diag-schema-additive-only` was narrowed to «must not rename or remove a field the product still emits», on the operator's Q2 «Сузить правило». Why it matters: the picked row's current wording; a hyprctl change to `diag-line.v1.json` may only add, through a Decisions Log entry.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS target, Linux only) — the tier values gained the Linux-only OS target: the core on any current distribution, CI on `ubuntu-latest` (the source of the picked `decision/linux-only-os-target`). Why it matters: the hyprctl argv, the fixtures and the replay are Linux and Hyprland only, with no per-OS arm and no CI leg beyond `ubuntu-latest`.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the ConPTY sideload and the Windows kill fallback retired) — the OS-API entity reads «openpty, Unix socket, filesystem locks and atomic rename, sysinfo», the base text the hyprland seam later extended in `decision/os-apis-at-call-sites`. Why it matters: the lineage of the picked OS-API row.

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left `diag-line.v1.json` with their emitter) — §6 gained the rule that a closed schema word no build can emit leaves the schema with its emitter. Why it matters: the other side of additive-only — any word this chunk adds must have an emitter in the build (the seam has no product caller, so an added word must still be emitted by the seam itself).

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the OS call sites the new checks add) — the OS-APIs entity's call-site list was extended when new OS reads were added. Why it matters: precedent that each new OS-level call site is named in `decision/instrument-call-sites-not-os-internals` / `decision/os-apis-at-call-sites`; a new hyprctl dispatch spawn is covered by «the hyprctl spawn» and needs no new entry unless a new OS call appears.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the nine thiserror derives, CoreError retired) — the census of product `thiserror::Error` derives counted by grep. Also the error-body census entry: `tests/contract_error_bodies.rs` holds every product error writer to a name and a witness, and `thiserror_messages_interpolate_nothing` guards fixed messages (the guards of the picked `decision/error-bodies-clean` and `decision/error-messages-interpolate-nothing`). Why it matters: if `HyprlandError` gains a word for the dispatch or `monitors`, its Display stays fixed with no interpolation, and the census rule decides whether a count moves.

@history_item n=21 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the thiserror derive census on the product sources) — the census rule became a grep over `src` and `crates` outside `crates/viola-e2e`, 9 derives. Why it matters: a new error variant adds no derive, but a new error type in `viola-hyprland` would move the counted census and its row.

@history_item n=22 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`Refused::NotADirectory`'s fixed message) — a new variant of an existing typed error adds nothing to the derive count, and its fixed message names no path. Why it matters: the precedent for adding a closed word to `HyprlandError` (5 words today).

@history_item n=23 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (3. Observability Harness Contract: 1 row) — `~ decision/veil-redact-payload-types` got the except: a payload type with no content field derives neither veil Redact nor Debug, since veil 0.3.0 refuses a Redact derive with no redact field. Why it matters: a new `Request` variant or a keep-list reply type that holds reply content (monitor names, a window address) derives Redact; one with no content field derives neither.

@history_item n=24 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (3. Observability Harness Contract: 1 row) — `~ decision/content-only-in-instance-detail-file` dropped «cookie» from what home-level files never carry. Why it matters: the picked row's current text; any hyprctl reply text kept beyond the keep-list may reach only the instance detail file, never a home-level role line, and the seam's lines stay codes only.

@history_item n=25 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — the schema is default-deny through a top-level `unevaluatedProperties: false`; `diag-detail.v1.json` is self-contained with the event enum inlined. Why it matters: the G4 schema-conformance gate (picked `decision/g4-schema-conformance-gate`, `decision/schema-conformance-gate`) fails any field outside the catalog, so a new field on the hyprctl lines must be admitted in both schemas.

@history_item n=26 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (fake agent's print-ban exemption) — `src/bin/viola-fake-agent.rs` is a root-package bin exempted from the print-macro ban by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`. Why it matters: the fake agent standing in as hyprctl replays the new `monitors` and dispatch cases; its stdout writes rely on that exemption, and no product module gains a print allow.

@history_item n=27 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (tracing-subscriber's dev-dependents named) — library crates (`viola-channel`, `viola-state`) take the workspace tracing-subscriber pin under dev-dependencies only, for their unit tests' line capture. Why it matters: if the timed-deadline or drain-cap witness in `crates/viola-hyprland` captures its `parse-rejected` or probe lines, it follows this form, never a product dependency on tracing-subscriber.
