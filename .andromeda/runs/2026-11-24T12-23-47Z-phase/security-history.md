@history_item n=1 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Threat Model Summary, 1 row) — `~ decision/cli-input-trust-boundary` added the closed model-word and effort-word checks to the CLI trust boundary when `run`/`revive` gained `--model`/`--effort`. A switch verb taking a new pair goes through these same checks and rests on this row (PROVISIONAL, delegate overseer).

@history_item n=2 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Input Validation, 5 rows, 1 of 2) — added `decision/launch-flags-checked` (model word 1–64 bytes, lowercase first; effort one of five words; a bad value is exit 2 before anything is read), `decision/launch-pair-argv-crossing` (the pair crosses only as four argv elements of a direct spawn, after `--plugin-dir` and before the caller's words; revive handing on the recorded pair is a second ratified crossing), `decision/launch-flags-refused-after-dashes`, `decision/board-launch-pair-re-parsed`, and `~ contract/revive-arguments/id` (`--list` conflicts with `--model`/`--effort`). These are the core input-validation precedents. A switch that records a new pair for revive/up to replay widens the "recorded pair" crossing, and a restart-based switch rides `launch-pair-argv-crossing`. All landed PROVISIONAL on the delegate word (W2), so the operator's own word is still owed.

@history_item n=3 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Input Validation, 1 row, 2 of 2) — `+ decision/effort-word-check-in-agent-crate`: the effort-word check lives in the Claude adapter crate beside the claude flag literals, an exception to `shared-validators-in-core`. A switch verb must call this same check (`viola_agent_claude::is_effort_level`) and must not duplicate it in core (PROVISIONAL, W3).

@history_item n=4 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Data Protection, 1 row) — `+ decision/launch-pair-kept-to-snapshot-and-event-log`: the pair stands only in the snapshot's `launch` field and the events log's `launch` line (log-only, no span, process-log line or terminal byte) and leaves only through `list --json` and the MCP list item. The switch's new events-log record and process-log lines must fit inside this row or be raised as a widening of it (PROVISIONAL, W2).

@history_item n=5 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Authentication & Authorization, 1 row) — `+ decision/launch-pair-authenticates-nothing`: the recorded model and effort are self-reported words and authenticate nothing. A switch that rewrites the recorded pair must not let it gate anything (PROVISIONAL, delegate fork answer).

@history_item n=6 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Input Validation, 2 rows) — `~ decision/launch-flags-checked` now names `adopt` beside run and revive, which take `--model`/`--effort` through run's own parsers; `~ decision/claude-agents-resolved-from-path` adds adopt's live-elsewhere check. Precedent: each new verb taking the pair joins this row's verb list. The switch verb has to be named there too.

@history_item n=7 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Authentication & Authorization, 3 rows) — `+ decision/adopt-signals-no-foreign-process` (adopt acts on no process viola did not start; PROVISIONAL, delegate), and `~ start-checks-home-tree-before-log-opens` / `~ start-checks-socket-dir-before-bind` gain adopt's start. A restart-based switch that ends and resumes a session must keep to the no-foreign-process bound, and any new start path must join both start-check lists.

@history_item n=8 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Data Protection, 3 rows) — `~ decision/start-writes-trust-record-flag-only`, `~ decision/no-writes-under-claude-home` and `~ decision/viola-writes-nothing-in-claude-dir` now name the start of run, revive and adopt as setting the trust flag (PROVISIONAL, delegate Q2). This bears directly on the scope's "nothing written to `~/.claude*`" requirement: a restart-based switch through revive's start writes the trust record under these named exceptions, which are the only standing writes under the Claude home.

@history_item n=9 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Threat Model Summary, 1 row) — `~ decision/nothing-written-under-claude-home` gains adopt's start in its except (PROVISIONAL). The row the switch's "no side effect on the user's settings" claim is read against.

@history_item n=10 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Secret Management, 3 rows, 1 of 2) — `~ credential-strip-by-name-only`, `~ env-not-config-channel` and `~ no-stored-secrets` now name the start of run, revive and adopt as reading `CLAUDE_CONFIG_DIR`'s value (when kept) to choose the trust record's config (PROVISIONAL). Any new start path must be named in these excepts, or it must read no env value.

@history_item n=11 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Threat Model Summary, 1 row) — `~ decision/state-tree-strict-modes-before-use` adds adopt's start to the verbs that check the home tree before any log. Precedent for how a new verb is added to the strict-modes lists.

@history_item n=12 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Error Handling, 3 rows) — `~ home-tree-strict-modes-before-log`, `~ run-refuses-socket-dir-before-start` and `~ contract/cli-json-error/internal-error` each gain adopt's start. A new verb has to be placed on the internal-error catch-site lists (it either prints the `internal-error` document under `--json` or is silent) and on the start-check lists, as adopt was.

@history_item n=13 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Input Validation, 1 row) — `~ decision/session-id-argv-closed-shape` except: adopt places an operator-typed id on `--resume` in the same closed shape through the one builder (PROVISIONAL, W2). A restart-based switch resuming the session uses this crossing.

@history_item n=14 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Data Protection, 1 row) — `~ decision/exec-commands-absolute-pinned-path` adds adopt to the starts that use the pinned path. A switch that restarts through run's start inherits this.

@history_item n=15 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Input Validation, 3 rows) — `~ open-terminal-direct-spawn` (up spawns the pinned `viola revive <name> --origin` in the recorded cwd), `~ run-window-flags-checked` (revive's hidden `--origin`), and `~ contract/revive-arguments/id` (`--list` conflicts with `--origin`). up replays a session through revive, so this is how a switched pair would reach a later `up`. Each new revive flag also joins the `--list` conflict list.

@history_item n=16 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Threat Model Summary, 3 rows) — `~ contract/cli-exit-codes/21` names up's board-scan refusal; `~ decision/cwd-only-in-snapshot` and `~ decision/spawn-cwd-snapshot-only` add the recorded cwd on up's terminal argv. Trap: exit 21's promise lists each verb's cause, and the cwd's crossings are a closed list that a new verb must not widen silently.

@history_item n=17 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Authentication & Authorization, 1 row) — `+ contract/strict-modes-refusal/up`: every new verb gets its own per-verb strict-modes refusal contract. A switch verb is owed one too (PROVISIONAL, delegate).

@history_item n=18 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Authentication & Authorization, 1 row) — `+ decision/up-checks-before-log-and-spawn`: a verb that spawns pinned starts must state which checks come before its log and its first spawn (PROVISIONAL). Precedent for a restart-based switch.

@history_item n=19 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Threat Model Summary, 2 rows) — `cwd-only-in-snapshot` and `spawn-cwd-snapshot-only` set provisional: a recorded value crossing onto a new argv is a boundary widening of its own, asked as a separate fork. The same treatment applies if the switch hands the recorded pair or cwd onto a new argv.

@history_item n=20 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Threat Model Summary, 2 rows) — the cwd rows' provisional mark was discharged by the operator's relayed W1 answer. This shows how a crossing widening for a resume path gets ratified.

@history_item n=21 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Input Validation, 5 rows) — `+ open-closed-argument-allowlist`, `+ open-terminal-direct-spawn`, `+ run-window-flags-checked`, `+ board-window-and-origin-re-parsed`. Sibling precedent for a new verb's closed argument surface and for recorded metadata that is re-parsed before the board shows it.

@history_item n=22 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Threat Model Summary, 2 rows) — `~ contract/cli-exit-codes/21` adds focus (a non-frame verb exiting 21 on a strict-modes refusal) and `~ state-tree-strict-modes-before-use` (PROVISIONAL). Exit 21 is no longer a frame verb's alone, so a switch verb's exit choice joins this list.

@history_item n=23 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Threat Model Summary, 1 row) — corrected `state-tree-strict-modes-before-use` to drop focus, because focus runs the instance check (`client::refuse_untrusted`, `check_instance`), not `check_home_tree`. Trap: name a verb on the home-tree row only if it really calls `check_home_tree`. A verb that talks to a live instance belongs on `cli-frame-verbs-strict-modes-first`.

@history_item n=24 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Error Handling, 1 row) — `~ decision/cli-frame-verbs-strict-modes-first` adds focus (instance check before any frame or compositor read, exit 21, no line). A switch verb that addresses a running instance joins this row's verb list.

@history_item n=25 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Data Protection, 4 rows) — `+ decision/start-trust-skips-trusted-folders`, `~ decision/trust-record-writer-guarded` (one guarded writer for verify and start), and the run/revive start's exception added to `no-writes-under-claude-home` and `viola-writes-nothing-in-claude-dir`. This is why a revive-based switch writes into the Claude config. Any new write there must go through this one guarded writer.

@history_item n=26 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Data Protection, 1 row) — `+ decision/start-writes-trust-record-flag-only`: on a verified CLI the start sets only `hasTrustDialogAccepted`. It was ratified with its price: it overrides a human's earlier "no", and a write lost in the lock window is accepted. This is the one standing settings-side effect a restart-based switch would carry.

@history_item n=27 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Input Validation, 2 rows) — `~ revive-recorded-cwd-spawn-dir` and `~ revive-cwd-spawn-only`: the recorded cwd is used only as the spawn dir and as the trust record's key. A switch resuming through revive inherits this. Any further use of the recorded cwd is a widening.

@history_item n=28 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Data Protection: start-trust-skips-trusted-folders · trust-record-writer-guarded · no-writes-under-claude-home · viola-writes-nothing-in-claude-dir) — the founder ratified the run/revive start's trust-record write. It now stands on his word, while adopt's extension is still PROVISIONAL. A switch that adds a new start path to these excepts needs its own word.

@history_item n=29 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Data Protection, 3 rows) — first bounded exception to `no-writes-under-claude-home` / `viola-writes-nothing-in-claude-dir` (verify's own trust record), plus `+ decision/trust-record-writer-guarded`. This sets the "nothing written under the Claude home" invariant the switch must keep: every exception is named and ruled.

@history_item n=30 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Authentication & Authorization, 1 row) — `~ decision/explicit-mode-before-write` except: the trust record writer keeps Claude's config file's own mode. This is the only carve-out from "mode set before any byte" for a file outside the viola home.

@history_item n=31 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Data Protection, 5 rows) — retired the per-session `settings.json` override and the child's `--settings` (`− settings-override-plain-path`, `− statusline-override-plain-path-only`, `~ code-bearing-files-rewritten-each-start`). A settings write is one of the scope's candidate switch channels, so this is the precedent: viola no longer writes a settings file or passes `--settings`, and bringing it back would reverse a removal.

@history_item n=32 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive spawns in the recorded cwd (ratified) and puts the logged session id on `--resume` as one argv element of a direct spawn, through `viola_agent_claude::resume_args` only. This is the original crossing a restart-based switch rides.

@history_item n=33 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (revive: the session id ratified) — the logged session id on a revived child's `--resume` was ratified by the founder as a crossing. Precedent: a logged value placed on a child's argv needs its own ratification, and the recorded launch pair took that path again.

@history_item n=34 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's preflight reads `claude agents --json` and refuses `session-live` when the id is live elsewhere. A restart-based switch that resumes the session meets this check, which adopt now shares.

@history_item n=35 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive's `--json` closed-code documents (never a path or pid), and the CLI home resolution (`--home`, else the `VIOLA_DIR` pair, else default, canonicalised; later ratified). A new verb's `--json` document and home resolution follow this shape.

@history_item n=36 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21 names its cause in detail) — exit 21's `detail` is a closed cause set, and the CLI and MCP each write every word. A switch verb that refuses an unreachable instance reads these causes and must not invent new ones without a ruling.

@history_item n=37 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start) — revive's closed exit-1 codes cover every start refusal it shares with run. A revive-based switch reports through these codes.

@history_item n=38 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's document under --json) — the `internal-error` document replaces the human line under `--json`, and the start roles stay silent there. A new verb must be classed on one side.

@history_item n=39 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's code at the collision check) — `stale-heartbeat` vs `already-live` at the start's collision check. A restart-based switch's start meets it.

@history_item n=40 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused`, the sixth cause) — CLI frame verbs run `check_endpoint_dir` after `live_endpoint` and refuse exit 21 `socket-dir-refused` with no frame. This binds a switch verb that talks to the wrapper.

@history_item n=41 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client's peer euid check and the order at a seam) — sets the fixed client seam order: socket-dir check, connect, peer euid, server verification. A switch verb that sends a frame must keep this order (rows `client-seam-check-order`, `client-peer-euid-check`).

@history_item n=42 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six CLI frame verbs share `link` / `unlink`'s instance check) — `client::refuse_untrusted` runs `check_instance` before `client::start`, so before any role file or log. A new frame verb takes the same order.

@history_item n=43 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home tree checked at `run` and `revive`'s start) — `check_home_tree` runs before the wrapper log, and a refusal is exit 1 with no log, line or child. This binds any start path a switch reuses.

@history_item n=44 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the served `link` / `unlink` methods behind the IPC controls) — serving a new channel method is a boundary widening and needs the Security Decisions Log record the Universal ban asks for (`new-listener-needs-log-entry`). If the switch adds a wrapper channel method, this is the precedent.

@history_item n=45 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`viola link` / `viola unlink`'s instance check before any log) — trap for later verbs: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a widened home. The switch's checks must come before its log opens.

@history_item n=46 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — added the human's terminal on `viola run`'s stdin as an attack-surface vector, with its trust boundary the closed non-editing list (widened once by measured replies). The scope's open question of whose wheel a mid-turn switch needs rests on `human-typing-takes-wheel` and the stdin classifier.

@history_item n=47 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — pause/release frames, with driver-originated `release` refused (`release-from-driver`). This is the wheel's security shape that a switch acting on a held or driver-held session must respect.

@history_item n=48 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Security Anti-Patterns, 1 row) — `~ decision/compiled-classification-sources`: a typed text is a local command only by its compiled name, alone or with arguments. If the switch types a command into the live session, that command must be a compiled entry. The scope also notes the CLI's own model command writes the user-scope model key, so it is not the channel.

@history_item n=49 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Threat Model Summary, 1 row) — `~ contract/wrapper-channel-methods/send`: a listed local command sent before its transcript exists, or on an unverified CLI, answers `confirmed:false` and never a confirmation. This binds a switch delivered as a typed command.

@history_item n=50 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (Threat Model Summary, 2 rows) — the excepts of `bridge-send-one-bracketed-paste` and `send-text-is-one-bracketed-paste`: a paste not finished within its bound is cut and refused `paste-cut`. Any typed-command channel for the switch goes through this paste path.

@history_item n=51 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — the typed text normalises CR/CRLF to LF and drops trailing CR/LF, but only after validation. Standing rule: typing any other character not in the received text needs its own ruling. This binds a switch that composes text to type.

@history_item n=52 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — `empty-text` is refused on both sides before any frame. Part of the send refusal order a typed-command switch inherits.

@history_item n=53 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply) — `sanitised_fault` and `is_code_word`: a client takes a refusal or detail only as a code word, with a standing exception that an unknown code word stays visible. A switch verb reading a wrapper reply must use these readers (`client-reads-closed-values-only`).

@history_item n=54 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP tool input validation and error bodies) — MCP handlers validate raw arguments into closed types, and a failure is `invalid-params` with a closed field word. This applies only if a fork brings in an MCP switch tool (out of scope by default; the tool list is an affordance, not an enforcement).

@history_item n=55 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Input Validation, 2 rows) — `+ run-lineage-flags-checked` and `+ board-lineage-and-links-name-checked`: sibling precedent for recorded per-instance metadata that is flag-checked on input and re-parsed before the board shows it (PROVISIONAL).

@history_item n=56 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Data Protection, 1 row) — `+ decision/lineage-kept-to-snapshot-and-event-log`: precedent for confining a new recorded pair to the snapshot and event log, the same shape as the launch pair's row.

@history_item n=57 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Input Validation, 3 rows) — `~ decision/config-cannot-disable-controls` now adds that no config value lifts the unverified-CLI gate. If the switch adds a config key, it must not switch off a control.
