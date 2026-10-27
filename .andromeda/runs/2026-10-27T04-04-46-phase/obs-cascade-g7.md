# obs · cascade read · group 7 of 10

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:108) The chunk's closed-type reading of channel params must leave `channel-request` recording `wait`'s `after` / `timeout_ms` only as `u64` values, and no params or result body on either side, whatever a refused or over-size frame carried.
  > `channel-request{corr:<id>, conn, method:"wait"|"last"}` on both sides, plus `after` / `timeout_ms` for method `wait` only and only as `u64` values;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:109)
  > A `last` response carries no `outcome`, and no result body reaches either line;
  (.andromeda/obs-plan.md:333)
  > channel lines never record `params` or result bodies.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:121) The names-before-path-joins witness for the dialog hook and CLI / MCP `answer` must hold that the instance check comes first and a refused or absent instance writes no line, sends no frame and emits no decision.
  > The instance check comes first on both sides: the dialog hook runs `canonical`, then `trusted`, before obs init, and a refused or absent instance exits 0 at once with no line, no frame and no decision (fail open); CLI `answer` runs `client::refuse_untrusted` before any role file opens, and MCP `answer` runs `check_instance` before `live_endpoint`, each refusal with no frame and no line.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:123) The hook's dialog kind must be read as the closed three-value type, and the closed-type `config.json` read must hand the dialog slot a hold inside 60 000–3 600 000 ms, default 60 000, whatever value the file held.
  > `run.dialog_register`: `dialog_kind` (`question|permission|plan`), `pending_conflict` (bool: a second concurrent dialog gets a `null` response);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:124)
  > `run.dialog_await`: `deadline_ms` (the hold the slot was started with: `config.json`'s `dialog_hold_s` in ms, 60 000–3 600 000, default 60 000), `deadline_hit`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:128) The dialog hook's bounded stdin must keep these lines: a `stdin_bytes` count and never the payload, no line at all for a refused or absent instance, the closed `unknown-dialog` refusal on the driver side, and never the question, answer or tool `input`.
  > hook: `hook-invoked{corr:<dialog_id>, hook_event, stdin_bytes, invoked_at}`, emitted once `dialog_id` is known (D-07), then `hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}`;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:131)
  > A refused or absent instance (the instance check, `canonical` then `trusted`, made before obs init) is none of the fail-open paths before `dialog_id`: the hook exits 0 with empty streams, writes no `hook-invoked`, no `hook-decision` and no line, and sends no frame.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:132)
  > driver-side refusals: `channel-response{result_class:"refusal", refusal:"not-delivered", detail:"unknown-dialog"}` (exit 13), or `refusal:"unverified-cli"` (exit 12, answer withheld).
  (.andromeda/obs-plan-detail/span-trace-coverage.md:135)
  > The question, answer and tool `input` never appear.
  (.andromeda/obs-plan.md:369)
  > The question, answer and tool `input` never appear; a dialog hook's `hook-invoked` is written only after the `hook.dialog` response.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:136) An oversize or malformed dialog-hook stdin must end in the fail-open pair before `dialog_id`, with a closed `detail`, and the hook's own read must stay inside its backstop bound.
  > The hook writes `deadline_hit:true` only when its own read times out at its backstop bound, `DIALOG_HOLD_MAX + 5 s` (3605 s). Fail-open before `dialog_id` gives `hook-invoked{corr:null}` + `hook-decision{corr:null, detail}` (Section 6 fail-open codes).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:151) `release`'s `from` and `budget` params are channel param decisions the closed-type witness covers: a `from` of another type or a non-bool `budget` must be a plain `-32602` with no line, and `from` reaches a line only when it is a valid name.
  > one `release-from-driver{corr:<rpc id>, conn, from (only when a valid name), from_trust:"self-reported"}` at INFO (D-01), beside the `-32602` `"invalid params"` reply with `data: {"reason":"release-from-driver"}`; a `from` of another type, or a non-bool `budget`, is a plain `-32602` (`data: null`) with no such line
- (.andromeda/obs-plan-detail/span-trace-coverage.md:165) The bounded `budget.json` read must stay span-free and line-free, an over-cap file read as no reading, and `statusline_command` must reach no span or line.
  > Two operations on this path carry no span and no line of their own, as landed: `viola_state::budget::read_budget` (a refused, unreadable, over-cap or other-`v` `budget.json` reads as no reading)
  (.andromeda/obs-plan-detail/span-trace-coverage.md:166)
  > Never `statusline_command`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:169) The statusline arm's bounded stdin must produce exactly these lines on a rejected stdin, and no line at all without a valid `VIOLA_NAME`.
  > `parse-rejected{parser:"hook-stdin", detail:"oversize"|"malformed", count:1}` for stdin the arm rejects (the hook events' own line, written by `reject_stdin` in `src/cmd/hook.rs`);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:171)
  > `hook-decision{hook_event:"statusline", budget_written, duration_ms}`, plus `detail` ∈ `oversize-stdin|malformed-json|deadline` on a fail-open path, and `deadline_hit` written only when true.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:172)
  > no line at all without a valid `VIOLA_NAME` (as landed, chunk 2026-10-10-statusline-pass-through), and none on an instance whose check refuses:
- (.andromeda/obs-plan-detail/span-trace-coverage.md:174) The status line command's stdout must pass through unchanged under its `take(MAX_FRAME)` and reach no line, and an oversize stdin must print nothing and run no shell.
  > The user's stdout passes through unchanged and is never logged. Oversize or malformed stdin prints nothing and runs no shell.
  (.andromeda/obs-plan.md:385)
  > Never `statusline_command`; the user's stdout passes through unchanged and is never logged; the shell-out is bounded by `STATUSLINE_DEADLINE` (5 s, PROVISIONAL).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:193) The channel `driver` / `driven` names the scope names must pass `ViolaName::try_new` before use, an invalid name a `-32602` with no line, and the trace pair must hold neither name.
  > **E3**, the explicit methods: the driven wrapper serves `link` / `unlink`, reading `driver` and `driven` as strings through `ViolaName::try_new`; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` and no line written, a `record` / `remove` error is `-32603`, and neither method reads `from`.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:194)
  > its trace is the driven wrapper's `channel-request` / `channel-response` pair with `method:"link"|"unlink"`, which holds neither the `driver` nor the `driven` name
- (.andromeda/obs-plan-detail/span-trace-coverage.md:201) The bounded `snapshot.json` read must keep one `state-recovered` line per call at WARN, an over-cap snapshot read as unreadable and so replayed, and no product reader of `read_snapshot`.
  > As landed (chunk 2026-10-10-self-healing-state) the line is written by `viola_state::replay::read_snapshot_or_replay`, at WARN with no `corr`: `snapshot-replayed` for an unreadable snapshot, `snapshot-unsupported-v` with `v_seen` for a newer one, one line per call; none for a present or an absent snapshot, and none when the replay itself fails.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:202)
  > no product reader calls `read_snapshot` (the rule: `read_snapshot(` under `src/`, outside test modules, as measured at viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md §Changes).
  (.andromeda/obs-plan.md:394)
  > `read_snapshot` has no product caller under `src/`; over a replayed or absent read `live_endpoint` and the hook `connect` give no endpoint, so no frame, and the statusline runs no command.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:208) The bounded `events.ndjson` reader must keep its three skip counts with no process-log carrier, neither healing nor logging.
  > The reader neither heals nor logs: its per-read counts (`unknown_kinds`, `unknown_fields`, `torn_lines`) have no process-log carrier.
  (.andromeda/obs-plan.md:396)
  > the reader neither heals nor logs.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:216) The bounded `ledger/stamps.json` read must land an over-cap ledger in a closed `ledger-stamps` outcome read as `cli_verified:false`, with no path or stamp contents in any line.
  > An unreadable or unparsable ledger, or one the strict-modes check refuses (`run`'s version gate reads it through `viola_state::stamps::read_stamps_strict`), gives `parse-rejected{parser:"ledger-stamps", detail:"unreadable"|"malformed"|"strict-modes-failed", count}` in `run-<name>.ndjson` and is treated as unverified (`cli_verified:false`).
  (.andromeda/obs-plan-detail/span-trace-coverage.md:217)
  > The path and stamp contents never appear.
  (.andromeda/obs-plan.md:398)
  > **Capability ledger** (`ledger/stamps.json`): read inside `run.version_gate`; a missing stamp, or an unreadable, malformed or strict-modes-refused ledger, counts as unverified (`cli_verified:false`); the path and stamp contents never appear.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:229) The hidden capture arms are `hook` stdin readers the enumeration must list with their stated caps, still uninstrumented, and verify's one `claude agents --json` read must keep row text, session ids, `cwd` and paths out of every line.
  > It writes the raw stdin payload (capped at `MAX_FRAME`) 0600 to an exclusively claimed `<probe>/captures/<Event>.<k>.json` under the 0700 `ledger/probes/<pid>/`
  (.andromeda/obs-plan-detail/span-trace-coverage.md:230)
  > It files the raw payload (stdin through `take(MAX_FRAME + 1)`, refused past `MAX_FRAME`) 0600 at the first free `StatusLine.<k>.json`
  (.andromeda/obs-plan-detail/span-trace-coverage.md:225)
  > then one `agents-probe` pair for Run B's single `claude agents --json` read (made once its first turn has settled, before the framing pastes; the capture held in memory; no row text, session id, `cwd` or path in a line; chunk 2026-10-16-unwrapped-sessions-on-the-board)

## Bindings
- **span-trace-coverage ↔ log-coverage**: every outcome a bounded reader adds or keeps on these paths (`hook-stdin` oversize / malformed, the fail-open `hook-decision` `detail`, `ledger-stamps`, `claude-agents-json`) must be a closed §6 `parser` / `detail` value at WARN; an over-cap outcome either maps into an existing code or joins the catalog and the schema's default-deny, (.andromeda/obs-plan-detail/span-trace-coverage.md:169), (.andromeda/obs-plan-detail/span-trace-coverage.md:216), (.andromeda/obs-plan.md:460), (.andromeda/obs-plan.md:482)
- **span-trace-coverage ↔ metric-coverage**: the `-32602` refusals of an invalid `link` / `unlink` name and a mistyped `release` param add to `viola.compat.v_rejected`'s stated over-count, so any new name or param check answered `-32602` falls in that over-count too, (.andromeda/obs-plan-detail/span-trace-coverage.md:193), (.andromeda/obs-plan-detail/span-trace-coverage.md:151), (.andromeda/obs-plan.md:418)
- **span-trace-coverage ↔ pii-scrubbing-compliance**: the bounded readers must keep content out of every home-level line: no params or result body, no question, answer or tool `input`, no row text, no status line stdout, no stamp contents or path, (.andromeda/obs-plan-detail/span-trace-coverage.md:108), (.andromeda/obs-plan-detail/span-trace-coverage.md:174), (.andromeda/obs-plan.md:557)
