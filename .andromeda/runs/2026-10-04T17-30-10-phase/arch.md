# arch extract

## Relevance
relevant — the wheel is an Established Decision of its own (§Established Decisions [Human Takeover / Wheel]), and the chunk lands two channel methods, two CLI verbs, the `wheel` event's non-`start` causes, and the wheel step in the `send` and `answer` refusal orders.

## Constraints
- **Workspace placement.** Per architecture §Infrastructure Patterns → Crate dependency direction (root-bin row), the wheel lives in the root `viola` bin beside the `run` pump and "consume[s] only normalised events". Per the same contract's `viola-pty` row and §Established Decisions [Agent Coverage], `viola-pty` "knows no agent" and the PTY/wheel layers "speak only viola's normalised events". So the editing-key byte classifier must be agent-neutral: in `src/run/`, or in `viola-pty` only if it carries no Claude shape. The harness-turn test stays on `viola-agent-claude`'s `HARNESS_PREFIXES` through the already-normalised `prompt-submitted` `origin` (§Established Decisions [Human Takeover / Wheel]; §Standard Contracts → Event `data` per kind). Per §Infrastructure Patterns → Project directory structure, `src/run/` is "PTY pump, wheel, budget governor" and `src/cmd/` holds one module per subcommand, `pause` and `release` among them.
- **Wheel semantics.** §Established Decisions [Human Takeover / Wheel] requires all of the following:
  - The wheel starts as `driver`.
  - Any human editing key since the last turn boundary moves it to `human`, a human interrupt included.
  - Focus, mouse and resize sequences do not move it.
  - Human bytes that arrive during the one `ESC[200~…ESC[201~`+CR write are held and passed on straight after it.
  - An unsent `prompt-submitted` confirms and logs the take; a harness-prefixed prompt is never a take.
  - `viola pause` moves the wheel to `human` (detail `manual-pause`).
  - The wheel returns only through CLI `viola release`, which also clears the running-turn state.
  - While the wheel is `human`: `hook.dialog` is answered `null` at once, `answer` is refused `human-typing`, and a dialog pending when the wheel moves is answered `null` at once with its `dialog_id` cleared.
  - §Design Philosophy "The human always wins the wheel" adds that no human byte is ever blocked, refused or delayed past the current atomic paste. [PTY] (H2) adds: "viola never holds, queues or re-orders a human key behind a resize". The only hold is the paste window.
- **Wire shapes (§Standard Contracts → Channel methods).**
  - `pause` `{}` → `{wheel:"human"}`.
  - `release` `{budget?:bool}` → `{wheel:<holder>, budget_paused:bool}`.
  - Every `params` carries `v` and `sender` (§Conventions Protocol versioning / Sender version).
  - §Conventions Channel methods already lists `pause` and `release` as id-bearing requests.
  - The `from` sentence in §Standard Contracts names only `send · wait · last · answer` as readers of `from`, while §Conventions CLI says the CLI adds `from` from its own `VIOLA_NAME` when set. How a `release` (and a `pause`) treats `from` must be settled against security's `release-from-driver -32602` rule. P4 decides `pause`'s `from` handling, because the plan is silent on it.
- **Refusal order and codes (§Conventions Error handling schema).**
  - `send`: `control-character` → `human-typing` → `budget-paused` → `turn-running` → readiness gate → confirmation.
  - `answer`: `control-character` → `human-typing` → `unverified-cli` → `unknown-dialog`.
  - The `human-typing` details form the closed set `null` · `manual-pause`, extended only in `viola-core`.
  - CLI exit `10` is `human-typing`.
  - A `-32602` protocol fault is exit `20` with the `wrapper-fault` `--json` body (§Standard Contracts → CLI `--json` output).
  - Unreachable is `21`.
- **Event and snapshot records (§Standard Contracts → Event `data` per kind, and the paragraph after it).**
  - `wheel` `{holder, cause}` with `cause` `start` · `human-input` · `manual-pause` · `release`.
  - It is appended `source: wrapper` "once at start and on every change". It is log-only and never wakes `wait`.
  - The instance snapshot is rewritten on every change to `wheel` (§Standard Contracts → Instance snapshot).
  - `pending_dialog` is removed when a dialog "is answered `null` on a wheel move".
  - Replay rebuilds `wheel` from the log (§Standard Contracts, the snapshot-fallback paragraph), so every holder change must reach `events.ndjson`.
  - A `hook.dialog` answered `null` under a human wheel still appends its dialog event with its own `dialog_id` (§Standard Contracts, the `hook.dialog` paragraph).
- **`release --budget` boundary.** Per §Established Decisions [Budget Governor], "`release --budget` leaves the wheel where it is, and a plain `release` returns the wheel to `driver` without lifting a budget pause". If the chunk ships the `--budget` flag's wire shape ahead of `:91`, it must not move the wheel. The flag sits in §Occupied Resources → Binary, subcommands and exit codes, `release` (incl. `--budget`).
- **Sync paths and terminal silence.**
  - Per §Cross-cutting Patterns Tokio containment, new code on the `run`, channel-server and CLI-verb paths uses std threads and blocking I/O, and CLI `pause`/`release` go through `viola-channel`'s sync client.
  - Per §Cross-cutting Patterns Diagnostic output channels, `run` writes nothing to the terminal but the child's output, so a wheel move prints nothing there.
  - `pause` and `release` fall under the `cli` role's `cli-<name>.ndjson` log and its `error: internal error` catch site (§Conventions CLI exit codes, the `role_of` clause). Whether `role_of` already files them under `cli` is a research question.

## Patterns to follow
- **One-slot wrapper modules.** Follow `src/run/send.rs` (one-in-flight slot) and `src/run/dialog.rs` (`DialogSlot`, one pending dialog, Condvar await). A wheel module in `src/run/` holds one holder state that `send`, `answer` and `hook.dialog` all consult (§Infrastructure Patterns → Project directory structure). Whether `run.rs:392`/`:403` already writes `wheel{driver,start}` as described is research's question.
- **Shared child input.** `PasteHandle` is the one input writer, shared by the human copy and the one-write bracketed paste (§Infrastructure Patterns → Project directory structure, `viola-pty`). The hold-during-paste behaviour belongs at that shared seam. Whether it is already a hold, or only a mutex serialisation, is research's question.
- **CLI verb shape.** Follow `src/cmd/send.rs`, `answer.rs` and `client.rs`:
  - human text by default, `--json` mirroring the channel `result` (`{"v":1,"ok":{"wheel":"human"}}`);
  - a typed exit code per outcome;
  - human-facing lines in `src/human.rs`;
  - argument shape `viola pause <target>` and `viola release <target> [--budget]` (§Conventions CLI; §Established Decisions [CLI Conventions]).
- **Additive fields.** A new field such as `send-refused`'s `wheel` is additive and needs no `v` bump. Readers skip unknown fields (§Conventions Protocol versioning; §Established Decisions [Validation]).

## Anti-patterns to avoid
- No MCP `pause` or `release` tool, and no driver-facing hint toward `release`. Both are CLI-only human controls over the wheel (§Established Decisions [MCP]).
- No Claude-specific byte or prompt shape in the wheel or in `viola-pty`. Harness classification stays in `viola-agent-claude` (§Established Decisions [Agent Coverage]; §Infrastructure Patterns → Crate dependency direction). A wheel decision must never become policy: byte provenance plus an explicit verb only (§Design Philosophy "Mechanism, not policy").
- Never hold, queue or swallow a focus, mouse or resize byte, or a key behind a resize. "Never move the wheel" is not "not passed through" (§Established Decisions [PTY] H2 clause; [Human Takeover / Wheel]).

## Contract bindings
- **arch ↔ obs.**
  - The `send-refused` `wheel` field (obs-plan §4 CL-1) is absent from arch §Standard Contracts → Event `data` per kind (`send-refused`: `{refusal, detail, cursor?}`). It is an additive amendment owed at wrap.
  - The `wheel` event causes and the `source: wrapper` append are owned here. Their line format is owned by obs.
  - `pause` and `release` log under the `cli` role.
- **arch ↔ security.**
  - `release` carrying `from` must be refused `-32602 release-from-driver` (security.md). This interacts with §Conventions CLI's "the CLI adds `from` … when set".
  - The `pause` and `release` frames' pre-connect check is outside every dated gap in security.md. P4 must surface it, never borrow one.
- **arch ↔ tests and a11y.**
  - The wheel's exit `10` joins the `cli_controls_not_disableable` table (§Cross-cutting Patterns Config management: no flag, key or env var disables a control).
  - The outer-PTY focus, mouse and resize cases (a11y-plan §3) witness [Human Takeover / Wheel]'s "do not count" clause.
- **arch ↔ budget (`:91`).** The `release` `budget_paused` field and the `budget` flag bind to [Budget Governor]. Here they are wire shape only.

## Acceptance criteria contributions
- A human editing key under a `driver` wheel appends exactly one `wheel{holder:"human",cause:"human-input"}` line (`source: wrapper`), rewrites the snapshot's `wheel`, and wakes no parked `wait`. A second key appends nothing. Focus, mouse and resize bytes append nothing and still reach the child (per architecture §Established Decisions [Human Takeover / Wheel]; §Standard Contracts → Event `data` per kind).
- `viola pause <name> --json` prints `{"v":1,"ok":{"wheel":"human"}}` and exits 0. After it, `viola send` exits 10 with `{"refusal":"human-typing","detail":"manual-pause"}`. After `viola release <name>` the wheel reads `driver` with cause `release`, and the running-turn state is cleared (per architecture §Standard Contracts → Channel methods; §Conventions Error handling schema).
- Under a human wheel, `send` reports `human-typing` ahead of `turn-running`/`budget-paused`, and `answer` reports `human-typing` ahead of `unverified-cli`/`unknown-dialog`. A `hook.dialog` gets `{dialog_id, response:null}` at once, with its dialog event still appended. A dialog pending at the wheel move is answered `null` and `pending_dialog` leaves the snapshot (per architecture §Conventions Error handling schema refusal order; §Standard Contracts → Instance snapshot / `hook.dialog`).
- No MCP tool, `config.json` key, `VIOLA_*` variable or flag adds or bypasses `pause` or `release`. The wheel code lives in the root bin's `src/run/` and `src/cmd/{pause,release}.rs`, with no new dependency edge into `viola-pty` or `viola-agent-claude` (per architecture §Established Decisions [MCP]; §Infrastructure Patterns → Crate dependency direction).
