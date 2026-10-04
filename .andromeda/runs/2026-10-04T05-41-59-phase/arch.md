# arch extract

## Relevance
relevant — the chunk lands the first driving verb (`send`) across the CLI, the wrapper channel, the `run` pump and the capability ledger, all of which architecture.md contracts directly.

## Constraints
- **Wire shape is fixed.** Per architecture §Standard Contracts ("Wrapper channel frames"; "Channel methods: `params` → `ok` payload"), `send` takes `{text, from?}` plus `v` and `sender`. It returns one of three shapes:
  - `{submitted_at, cursor}` when confirmed;
  - `{confirmed:false, detail:"unconfirmable", cursor}`;
  - `result.refusal` + `detail`.

  `cursor` is the `events.ndjson` end offset at acceptance, before the paste. Refusals travel in `result`, never `error`; `error` is reserved for the `-32600…-32603` protocol faults (§Conventions "Channel refusal / success / protocol fault"; §Established Decisions [API Style]). The method name is plain `send`, in the request list (§Conventions "Channel methods"). Only `hook.event` may be id-less, and a frame is at most `MAX_FRAME` (§Standard Contracts, the paragraph under the frames).
- **Refusal vocabulary is a closed set extended only in `viola-core`.** §Conventions "`RefusalReason`" lists `not-delivered` details as exactly `input-not-ready` · `no-prompt-submitted` · `turn-running` · `unknown-dialog`. Neither `control-character` (named by security.md and the scope) nor any one-in-flight detail is in arch's set. Adding them is a `viola-core` extension plus a §Conventions amendment at wrap.
  - §Conventions fixes the `send` precedence: `human-typing` → `budget-paused` → `not-delivered`/`turn-running` → gate (`input-not-ready`) → confirmation (`no-prompt-submitted`).
  - Where the validator and the in-flight guard sit in that order is unspecified and is P4's to pin.
  - `unconfirmable` is never a refusal detail. It appears only in an `ok` payload.
- **Confirmation semantics.** Per §Established Decisions [Delivery Confirmation] and §Design Philosophy "Measured, never assumed":
  - every send is confirmed after the fact, never presumed;
  - `send` blocks for at most `GATE_MAX_WAIT` + the atomic paste + the window, and returns no later than the window's close;
  - an unstamped CLI version uses `CONFIRM_WINDOW_FALLBACK` (10 s, PROVISIONAL, in `viola_agent_claude::screen`), never a `config.json` value;
  - matching compares the sent text with `prompt-submitted.text` as already normalised by `viola-agent-claude`, so the wrapper-side matcher sees no Claude shape;
  - the local-command decision depends on the ledger list, never on a leading slash. A listed command with a measured post-condition (`/clear`: SessionStart `clear` + a new `session_id`) is `ok` and confirmed. A listed command with "none" or no measurement on this version is `unconfirmable`. Every other unconfirmed send is `not-delivered`/`no-prompt-submitted`.
- **The bridge write.** Per §Established Decisions [Human Takeover / Wheel], a bridge send is ONE `ESC[200~…ESC[201~` + CR write through the `pty` seam ([PTY]). Human bytes arriving during that write are held in the pump and passed on straight after, never refused or dropped. The one-in-flight guard and every wait refuse automation only (§Design Philosophy "The human always wins the wheel").
- **The readiness gate.** Per §Established Decisions [Screen Model], the gate types nothing and reports `input-not-ready` when:
  - the screen is not quiet within `GATE_MAX_WAIT`;
  - a signature check fails;
  - the model is poisoned.

  The tee writes the human's bytes first and only then sends a copy, so the passthrough never waits on the model. CARRY 5's bound on the tee → feed queue must keep that property: overflow may degrade the verdict, never block or alter the pump. "On unverified builds the gate falls back to delivery confirmation only" is the sentence this chunk, its first consumer, must give a reading.
- **Placement.** Per §Infrastructure Patterns → Project directory structure and → Crate dependency direction:
  - the CLI verb goes in `src/cmd/send.rs` (one module per subcommand);
  - human output goes through `src/human.rs`;
  - the wrapper side goes in `src/run/` (the pump, gate.rs);
  - `RefusalReason` and `validate_paste_text` go in `viola-core`, which depends on no viola crate (§Established Decisions [Error Handling]: "`RefusalReason` lives in `viola-core`");
  - local-command ledger rows go only in `viola-agent-claude`'s pure `ledger` module;
  - the channel method rides `viola-channel`'s sync client/server.

  Per §Cross-cutting Patterns "Tokio containment", the `send` and channel-server paths use std threads and blocking I/O.
- **Ledger rows.** Per §Established Decisions [CLI Version Compatibility] and §Cross-cutting Patterns "Capability ledger as the single gate", the local-command list is a ledger row (each command → post-condition or "none") with a `viola verify` probe carrying a post-condition check. "A version is verified only when its stamp holds every landed row `pass`", so adding a row to the closed `ledger` set changes the verified status of every existing stamp. On an unverified version viola still types and confirms (transport-only).

## Patterns to follow
- CLI verb conventions (§Established Decisions [CLI Conventions]; §Conventions "CLI" and "CLI exit codes"):
  - `viola send <target>` reads the text from stdin or `--file`, never from a leading-slash argument, and warns on a Git Bash rewritten-path prefix;
  - the CLI adds `from` from its own `VIOLA_NAME`;
  - the typed exits are 13 `not-delivered`, 10/11/12/14 for the other reasons, 20 for a wrapper protocol fault and 21 for an unreachable instance;
  - `--json` mirrors the channel `result` as `{"v":1,"ok":{…}}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":"instance-unreachable"|"wrapper-fault",…}` (§Standard Contracts "CLI `--json` output").
- Output channels (§Cross-cutting Patterns "Diagnostic output channels"):
  - short-lived CLI verbs may use stderr for human output;
  - `run` writes nothing to the terminal but the child's output while the child runs, so no send-side status reaches the wrapper's terminal;
  - codes-only process lines go to `diagnostics/cli-<name>.ndjson` / `run-<name>.ndjson`;
  - content-bearing detail (the prompt) goes only to `instances/<name>/diagnostics/detail-<role>.ndjson`.
- Crash-safe disk writes (§Cross-cutting Patterns "Crash-safe disk writes"; §Conventions "ndjson line discipline"):
  - any `events.ndjson` line is one `write`, with multi-line text as one escaped JSON string;
  - `events.ndjson` is never truncated, because `send`'s `cursor` is a byte offset into it (§Occupied Resources → Filesystem, `instances/<ViolaName>/`).
- Fuzz and timing placement:
  - the new fuzz target sits in `fuzz/fuzz_targets/` with a seeded `fuzz/corpus/<target>/`, inside the separate excluded `fuzz/` workspace, never a root member (§Established Decisions [Module Boundaries]; §Infrastructure Patterns → Project directory structure);
  - wall-clock waits go through the `viola-core` `Clock` seam named last chunk (§Infrastructure Patterns → Project directory structure, `viola-core` row).
- Mixed-version tolerance (§Conventions "Protocol versioning" and "Sender version"; §Cross-cutting Patterns "Mixed-version tolerance"): a `params.v` above the supported version is answered `-32602` with `data.supported` / `data.wrapper`, and every reader skips unknown fields.

## Anti-patterns to avoid
- **Presuming delivery or retrying blindly.** Any failure to confirm is `not-delivered`, never retried, and nothing outside a ledger-listed local command is `unconfirmable` (§Cross-cutting Patterns "Fail open toward the human"; §Established Decisions [Delivery Confirmation]). Hard-coding a local-command list or `/clear`'s post-condition outside the ledger is also banned (§Cross-cutting Patterns "Capability ledger as the single gate").
- **Queueing or delaying input.** Never queue a refused `send`: a turn-running send "is not queued" ([Human Takeover / Wheel]), and the same holds for the in-flight second send. Never let the gate, the bounded feed queue or the confirmation wait hold a human byte past the current atomic paste (§Design Philosophy "The human always wins the wheel").
- **Shape leaks.** No Claude-specific shape in `viola-core` or the wrapper-side matcher (§Conventions "Normalised event kinds": "never with Claude-specific names"). No Tokio on the `send` path (§Established Decisions [Concurrency / Backend Framework]). No `deny_unknown_fields` on the new frames (§Established Decisions [Validation]).

## Contract bindings
- **arch ↔ obs (event kinds).** The scope's CL-1 `send-issued` / `send-refused` are not in arch's normalised kind list (§Conventions "Normalised event kinds"). Two placements are possible:
  - as `events.ndjson` lines, they are new `viola-core` kinds (`source: wrapper`) and need a §Conventions + §Standard Contracts "Event `data` per kind" amendment;
  - as obs process-log events, they sit under obs-plan's catalog instead.

  Which one obs-plan :629 means is the obs extractor's and research's question. Either way the prompt text stays out (NEVER-log floor). The one arch-sanctioned home of prompt text in `events.ndjson` is `prompt-submitted.data.text` (§Standard Contracts "Event `data` per kind").
- **arch ↔ agent-claude/hook path (origin).** §Standard Contracts `hook.event` re-validation admits `prompt-submitted` `origin` only as `harness` · `human`. The event-data contract lists `driver` · `human` · `harness`, and [Human Takeover / Wheel] says "an unsent `prompt-submitted` confirms and logs the take". So the `driver` origin implies a wrapper-side classification against the in-flight send before the append. Whether the code already assigns `driver` anywhere is research's question.
- **arch ↔ security.** Two ties:
  - `validate_paste_text` (security-plan §Input Validation) lands in `viola-core`, the home §Established Decisions [Error Handling] / CLAUDE.md §Modules give it; the wrapper re-runs it.
  - The `send` client's first frame is subject to the security-plan's server verification. Only `hook.event` holds the dated exemption (§Cross-cutting Patterns "Local endpoint trust boundary" delegates enforcement to security).
- **arch ↔ tests/design.** Five ties:
  - the RB readback lines (layout-templates :405) render through `src/human.rs`;
  - the exit-13 row extends `tests/cli_controls_not_disableable.rs`;
  - the fifth fuzz target joins CI `fuzz-replay` (§Infrastructure Patterns → CI/CD approach: `agent-run run --fuzz-replay` + `gate --require fuzz-replay`);
  - the forced feed panic meets G2's `scripts/g2-zero-panics.sh`, whose one exemption is `src/cmd/hook/seam.rs:<digits>` (§Infrastructure Patterns → Project directory structure);
  - any new fake-agent mode is a flag on the test-only `viola-fake-agent` bin, never a new env var (§Established Decisions [Naming]; §Cross-cutting Patterns "Config management": only `FAKE_AGENT_PUMP_DELAY_MS` / `FAKE_AGENT_HOOK_PANIC` exist).

## Acceptance criteria contributions
- (arch) **Wire shape.** A confirmed `send` frame returns exactly `{"ok":{"submitted_at","cursor"}}`, where `cursor` is the `events.ndjson` end offset read before the paste. An unconfirmed non-local send returns `{"refusal":"not-delivered","detail":"no-prompt-submitted"}` in `result`, not `error`, no later than gate max wait + paste + `CONFIRM_WINDOW_FALLBACK` (per architecture §Standard Contracts "Channel methods" and §Established Decisions [Delivery Confirmation]).
- (arch) **Closed refusal set.** Every `not-delivered` detail `send` can emit is a `viola-core` `RefusalReason` detail. Any detail outside §Conventions' four (`control-character`, the in-flight detail) is recorded as a §Conventions amendment at wrap, and `send`'s refusal precedence matches §Conventions' stated order (per architecture §Conventions "`RefusalReason`").
- (arch) **Placement.** `viola send` lives in `src/cmd/send.rs` and writes human output only through `src/human.rs`. `validate_paste_text` / `RefusalReason` live in `viola-core`, and local-command rows only in `viola-agent-claude::ledger`. No `tokio` enters a sync crate (`cargo deny` per `scripts/sync-crates.txt` stays green), and no env var beyond `VIOLA_*` and the two `fake-agent` seams is read (per architecture §Infrastructure Patterns → Project directory structure / → Crate dependency direction and §Cross-cutting Patterns "Tokio containment" / "Config management").
- (arch) **Passthrough under load.** With the bounded tee → feed queue saturated, the passthrough to the human terminal is byte-identical and undelayed, and the gate reads `input-not-ready` (per architecture §Established Decisions [Screen Model] and §Design Philosophy "The human always wins the wheel").
