# arch extract

## Relevance
relevant — the chunk grows the capability ledger, gives `viola verify` its first PTY child, retires the S3/S7/S8 dated exception and lands `send`'s `unconfirmable` outcome, all arch-owned contracts.

## Constraints
- **The ledger is the single gate.** Every behaviour this chunk measures must become a closed-set ledger row in `viola-agent-claude`, each with a `viola verify` probe and a post-condition check. That covers the input-box and modal signatures, quiet period, maximum wait, local commands, S3/S7/S8, dialog concurrency, the long-paste wrapper, tag escaping, harness prefixes and the R8 identity floor. A version is verified only when every landed row reads `pass`, so growing `LedgerRow::ALL` un-verifies every existing stamp until it is re-stamped. The source is arch §Cross-cutting Patterns "Capability ledger as the single gate for CLI-specific behaviour" and §Established Decisions [CLI Version Compatibility]. That same anchor holds the dated exception ("the S3/S7/S8 decision bodies … gated only by the six-row `cli_verified` until `working-route.md:84`"). The exception closes with this chunk, so its text in both places is an amendment owed at wrap.
- **Crate placement.** `viola-agent-claude`'s `ledger` and `screen` modules stay pure: no I/O, no clock read, every instant a parameter. The crate gains no `viola-state` dependency, and it is the only crate that knows Claude shapes. `viola-pty` depends on no viola crate and knows no agent. So the PTY drive, the timing loop, the capture wait and the stamps I/O belong in the root bin (`src/cmd/verify.rs`), which consumes `viola-pty` and the pure ledger. Sources: arch §Infrastructure Patterns → Crate dependency direction, and §Established Decisions [Module Boundaries] / [Screen Model].
- **Tokio containment.** The verify PTY drive must be std threads and blocking I/O. Only `viola-mcp` and `viola-ui` may build a runtime. Sources: arch §Cross-cutting Patterns "Tokio containment" and §Established Decisions [Concurrency / Backend Framework].
- **Child-spawn discipline for the new PTY child.** The program is resolved to an absolute path by viola. A `.cmd`/`.bat` child is refused except the `claude` npm shim, which resolves to `claude.exe`. The child is spawned directly, never through sh/bash/cmd. Exit is detected on the process handle, never on EOF, and the output reader is capped at `MAX_FRAME`. Sources: arch §Established Decisions [PTY] / [CI/CD], and §Cross-cutting Patterns "Cross-platform discipline".
  - Open question for research and the plan: arch §Occupied Resources → Environment variables defines the R8 `CLAUDE*` strip for `viola run`'s child only.
  - Arch §Established Decisions [Plugin Scope] names verify's *print-mode* probe as the one unwrapped exception. An interactive probe widens that wording, which is an arch amendment.
- **Stamps contract.** Only `viola verify` writes `ledger/stamps.json`, and only through `viola_state::stamps::update_stamps` (lock held, `replace_private`).
  - The measured values (quiet period, maximum wait, and later the confirmation window) go in the version's `measured` object.
  - A merge replaces only its own version's entry and keeps unknown versions and fields.
  - A failing row still writes its stamp, and verify then exits 1.
  - Sources: arch §Standard Contracts "Ledger stamps envelope" and §Occupied Resources → Filesystem (`ledger/stamps.json`).
- **Delivery-confirmation semantics.** `unconfirmable` is reachable only for a local command that is on the ledger list and whose post-condition is "none" or not yet measured on this CLI version. It is decided by the ledger, never by a leading slash. `/clear` is confirmed by a SessionStart with source `clear` plus a new `session_id`. Any other unconfirmed send stays `not-delivered` / `no-prompt-submitted`. `unconfirmable` is never a refusal detail. Sources: arch §Established Decisions [Delivery Confirmation], §Cross-cutting Patterns "Fail open toward the human" and §Conventions "Error handling schema".
- **Real CLI is local only.** The real `claude` probe and `--record` run only locally, and CI runs `verify` against the fake agent. Sources: arch §Established Decisions [CI/CD] and [CLI Version Compatibility].
  - The binary reads no env var outside `VIOLA_*` except the two `fake-agent`-gated seams (arch §Conventions "Environment variables").
  - `CI` is read only by `viola-harness` (arch §Occupied Resources → Environment variables).
  - So the scope's "[inferred] live leg refused under `CI`" cannot be a `CI` read inside `viola verify` without a Decisions Log entry. Where that refusal lives is the plan's question.

## Patterns to follow
- **The print-mode probe's skeleton.** Its parts, per arch §Occupied Resources → Filesystem (`ledger/probes/<pid>/…`) / → Claude Code integration names, and §Established Decisions [Hook Contract]:
  - the probe dir `ledger/probes/<pid>/{plugin/,captures/}`, 0700, removed whole by a drop guard on every exit path;
  - the `viola-verify-probe` plugin of exec-form hooks on the pinned path, each calling the hidden `hook <event> --capture <probe>/captures`;
  - one stdout step line per row plus a `stamped` summary.

  The dialog re-probe needs PreToolUse/PermissionRequest capture hooks beyond today's four spine hooks, and the plugin's registered set is an Occupied Resources entry to amend.
- **The pure screen model.** Use `viola_agent_claude::screen::Screen` and its `verdict` with every instant passed in. The bin owns the clock and the feed, as `run`'s feed thread does in `src/run/gate.rs`. The landed rows replace the compiled `Signatures { input_box, modals }` literal lists and the PROVISIONAL `QUIET_PERIOD` / `GATE_MAX_WAIT` per stamped version. `CONFIRM_WINDOW_FALLBACK` stays the unverified-build fallback (per arch §Established Decisions [Screen Model] / [Delivery Confirmation]).
- **Normalisation order is fixed.** `viola-agent-claude` classifies `harness` on the raw prefix first, then unwraps only the exact same-id `<pasted_content id="X">` pair, then reverses `<\` escaping. The new paste-framing and harness-prefix rows measure the shapes this order assumes, and the order itself does not change (per arch §Established Decisions [CLI Version Compatibility], the long-paste wrapper and tag-escaping rows, and §Standard Contracts "Event `data` per kind" `prompt-submitted`).
- **The `unconfirmable` outcome surfaces on every interface as one shape.**
  - Channel: `send` → `{"ok":{"confirmed":false,"detail":"unconfirmable","cursor":…}}`.
  - CLI: `--json` `{"v":1,"ok":{…}}`, exit 0.
  - MCP: `isError:false` with the `ok` payload as `structuredContent`.

  Sources: arch §Standard Contracts "Wrapper channel frames" / "Channel methods" / "CLI `--json` output", and §Conventions "MCP".
- **Fixture recording.** `--record` writes only when 0 rows fail, into `fixtures/claude/<ver>/`, scrubbed (home → `~`, user → `<user>`) and hygiene-walked. The new re-probe's captures supersede the relayed 2.1.287 dialog fixtures (`RELAYED.md`). Source: arch §Occupied Resources → Repository (`fixtures/claude/<cli-version>/`).

## Anti-patterns to avoid
- Do not hard-code a CLI behaviour as an unconditional assumption: a signature literal, the local-command list, a prefix or a timing value compiled outside a ledger row. Do not put a Claude shape in `viola-pty` or the root bin (arch §Cross-cutting Patterns "Capability ledger as the single gate"; arch §Inherited Defaults "Agent isolation").
- Do not decide `unconfirmable` by a leading slash, and do not presume or blindly retry delivery (arch §Established Decisions [Delivery Confirmation]; §Cross-cutting Patterns "Fail open toward the human").
- Do not give `viola verify` a second stamps writer or a path to stdout outside its step lines and `--json`. Do not run the real `claude` or place a credential in CI (arch §Occupied Resources → Filesystem `ledger/stamps.json`; §Cross-cutting Patterns "Diagnostic output channels"; §Established Decisions [CI/CD]).

## Contract bindings
- **arch ↔ tests.**
  - Harness homes are stamped at `boot` by `viola verify` against the fake agent (arch §Occupied Resources → Repository, `target/e2e-home/…`).
  - Once `LedgerRow::ALL` grows, the fake agent must answer every new row's probe, including a typed-input/PTY mode beyond its print mode. Otherwise CI's stamped homes read `cli_verified:false`.
  - This also binds the matrix claims v1-29 / v1-15 / v1-30 named in scope.
- **arch ↔ obs.** Any new probe subject, or a redefinition of `verify-probe`, changes the obs-owned `schemas/diag-line.v1.json` (arch §Occupied Resources → Repository). Recorded screen text is content-bearing and so goes only to `detail-*` (arch §Cross-cutting Patterns "Diagnostic output channels").
- **arch ↔ security.**
  - Screen-text fixtures are a new fixture class. Today `schemas/claude-fixture.v1.json` requires `hook_event_name`, so the class needs its own registry entry and the scrub-or-refuse rule.
  - Two inherited questions: does the R8 strip apply to verify's PTY child, and do the probe plugin's added hooks stay exec-form on the pinned path (arch §Occupied Resources → Repository / → Environment variables / → Claude Code integration names).
- **arch ↔ events.** The closed event-kind list has only `send-issued` / `send-confirmed` / `send-refused` for a send outcome (arch §Conventions "Normalised event kinds"; §Standard Contracts "Event `data` per kind"). Arch does not say which line records an `unconfirmable` settle. Any new kind lands only in `viola-core` with no Claude-specific name. Whether the code already logs it is research's question.

## Acceptance criteria contributions
- (arch) Every row this chunk adds is in `LedgerRow::ALL` with its own `viola verify` probe and post-condition. A non-`null` dialog decision then requires a stamp that holds the S3/S7/S8 rows `pass`. At wrap, the "one dated exception" text is retired from arch §Cross-cutting Patterns and [CLI Version Compatibility] (per arch §Cross-cutting Patterns "Capability ledger as the single gate for CLI-specific behaviour").
- (arch) `viola-agent-claude` gains no `viola-pty`, `viola-state` or `tokio` dependency. `viola-pty` gains no viola-crate dependency. The PTY probe drive lives in the root bin. Check: `cargo tree -p viola-agent-claude` / `-p viola-pty` (per arch §Infrastructure Patterns → Crate dependency direction).
- (arch) The `viola` binary reads no new env var outside `VIOLA_*` and the two `fake-agent` seams. In particular there is no `CI` read in `viola verify`; the live-in-CI refusal stays in `viola-harness` unless a Decisions Log entry says otherwise (per arch §Conventions "Environment variables").
- (arch) Three `send` cases:
  - `/clear` returns `ok` confirmed on SessionStart `clear` plus a new `session_id`.
  - A ledger-listed command whose post-condition is "none" or unmeasured returns `{"confirmed":false,"detail":"unconfirmable","cursor":…}`, CLI `--json` exit 0.
  - An unlisted slash text that gets no `prompt-submitted` returns `not-delivered` / `no-prompt-submitted`.

  Per arch §Established Decisions [Delivery Confirmation] and §Standard Contracts "Channel methods".
