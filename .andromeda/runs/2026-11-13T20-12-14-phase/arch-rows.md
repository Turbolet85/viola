# architecture · the rows that bind 2026-11-13-rows-proposal-rule-work — picked 262 of 987 (131 by every draw) · rule any · pin 950

## design-philosophy · ## Design Philosophy
decision/send-confirmed-after-the-fact · Every send confirmed after the fact — A send is confirmed only by a matching `prompt-submitted` within the compiled confirmation window (never in `config.json`), or for a ledger-listed local command by its measured post-condition; otherwise `not-delivered`; the ledger decides, never a leading slash. · why: A turn boundary per the hooks does not prove the input box is ready. · except: A ledger-listed local command whose post-condition is none, not yet measured, or whose CLI is unverified returns `ok` with `confirmed: false` and detail `unconfirmable`.

## stack-and-technologies · ## Stack and Technologies
decision/append-one-write-per-line · Log appends one write per line — Log appends use std append mode with one write per line. · why: crash-safe multi-process writes with no C code
decision/atomic-replaced-state-files · Replaced files through one atomic helper — Snapshots and every other replaced file go through one shared helper that sets the mode on a temp file before the write, syncs, and persists it in one attempt. · why: crash-safe multi-process writes with no C code
decision/clippy-print-and-macro-bans · clippy deny warnings with print and dbg bans — clippy runs with warnings denied, the workspace bans on print_stdout, print_stderr and dbg_macro, and disallowed tracing level macros.
decision/error-context-detail-file-only · Error context chains only in the detail file — Error context chains are built at dispatch and recorded only in the owner-only instance detail file.
decision/frame-params-redacted · Frame params never rendered in Debug — A frame's params never reach a Debug render, through veil's derived redaction with its toggle feature banned.
decision/libc-only-os-binding · libc is the one OS-binding crate — libc is the one OS-binding crate of the Linux-only build, a plain dependency; no crate depends on windows-sys.
decision/nutype-domain-newtypes · nutype domain newtypes — Bounded domain values are nutype newtypes: ViolaName (charset and length cap), Percent (0 to 100), DialogHold (whole seconds 60 to 3600).
decision/portable-pty-seam · portable-pty behind viola's own pty seam — The unmodified claude is hosted in openpty through a pinned portable-pty behind viola's own pty seam, with libc for the host tty's raw mode.
decision/run-never-reads-screen-content · run never reads screen content — run never reads screen content; the only row-text exit of the screen is read by verify alone, for its signature-only recorded screen fixtures.
decision/coverage-floors-gate · Coverage floors enforced by the harness gate — Coverage is measured by cargo-llvm-cov and the harness gate enforces the coverage floors.
decision/pre-push-filter-not-verdict · Local pre-push gate is a filter, not the verdict — The local pre-push gate runs the ubuntu test job's suites natively in the working tree under a cleared environment, refuses off Linux and installs nothing; it is a pre-push filter, never the verdict of record.
decision/socket-dir-strict-check · Socket directory checked before bind and connect — The recorded endpoint's directory is checked before a bind and before each client's connect; a symlink, a non-directory, another owner or a group or other bit is refused and never chmodded, an absent path passes, and a refused directory writes no frame.
decision/tracing-process-logs · tracing for one-line JSON process logs — Process logs are one-line JSON written by tracing into the viola home's diagnostics, with the root bin the only product dependent of tracing-subscriber; the line format and levels are owned by the obs plan.
decision/typed-errors-per-crate · thiserror per crate, anyhow at the bin edge — Each crate has typed errors callers can branch on through thiserror; anyhow is used at the bin edge only. · except: viola-pty's error type is hand-written

## established-decisions · ## Established Decisions
decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`, which stays pure; the other layers speak only normalised events. · why: A trait designed before a second agent exists would encode guesses; isolating the agent in one crate keeps the later extraction mechanical.
decision/cli-human-text-json-exit-codes · Human text by default, --json for agents — CLI verbs print human text by default and a machine document under --json, with a typed exit code per refusal; without --json the human output is unchanged. · why: Agents need machine-parseable refusals.
decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need machine-parseable refusals.
decision/every-send-confirmed · Every send is confirmed — A send not matched by a prompt-submitted within the confirmation window is reported not-delivered with detail no-prompt-submitted. · why: A turn boundary per the hooks does not prove the input box is ready; confirmation is the backstop behind the screen gate.
decision/fake-agent-after-double-dash · Fake agent named after -- — The fake agent is substituted by naming its executable after `--` (`viola run <name> -- <fake agent> <args>`).
decision/hook-never-writes-stderr · Hook logs only to diagnostics — A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
decision/hook-without-trusted-instance-exits-silently · Hook with no trusted instance exits 0 untouched — A hook whose instance check refuses or finds no wrapped instance exits 0 before reading config or stdin, and writes no line, frame or file. · why: The fail-open hook contract and the hook strict-modes tests rely on it, and the check must come before the log open that narrows the home.
decision/identity-by-instance · Identity by instance, never by location — Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys. · why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
decision/instance-check-before-any-read · Instance check before any read — The client verbs check the home and target instance directories before any role file, log or snapshot read. · why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
decision/link-recording · How a driver-to-driven link is recorded — A link `driver → driven` is recorded when a `send` or `answer` is issued from inside a wrapped session to another instance, or by an explicit `viola link <driver> <driven>`; `viola unlink` removes it.
decision/links-are-log-events · Links are events in the log — Links are events in the log, and the snapshot's link set is derived by replaying them. · why: The log stays the single source of truth for the link set.
decision/links-recorded-as-log-events · Links recorded as log events — A link `driver → driven` is recorded when a `send` or `answer` comes from inside a wrapped session to another instance, or by `viola link`, and removed by `viola unlink`; the snapshot's link set is derived by replaying those events. · why: The log stays the single source of truth for the link set as for everything else.
decision/linux-only · Linux only — viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree. · why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.
decision/mcp-frame-tools-strict-modes-check · MCP frame tools refuse a strict-modes failure — Every MCP frame tool refuses a home or target instance directory that fails the strict-modes check with the instance-unreachable result and detail strict-modes-failed, sending no frame. · why: The handlers test expects the refusal before any snapshot read or frame, so an untrusted home never steers a frame to a wrapper.
decision/no-key-held-behind-resize · No human key held behind a resize — viola never holds, queues or re-orders a human key behind a resize; a test that writes a key after a resize waits for the child to observe the new size first.
decision/nutype-newtypes · nutype newtypes for validation — Validated values are nutype newtypes (`ViolaName`, `Percent`, `DialogHold`); no garde or validator.
decision/partial-gate-on-unverified-cli · Partial readiness gate on an unverified CLI — On a CLI version with no passing stamp the gate still refuses a poisoned model and a screen not quiet within the maximum wait, and a quiet screen is ready with no row read, leaving the rest to delivery confirmation.
decision/poisoned-screen-model · Poisoned screen model reads not ready — A caught vt100 panic or a dropped feed message poisons the screen model until the host size changes, and a poisoned model reads input-not-ready. · why: A dropped byte desynchronises the screen.
decision/private-mode-before-write · File mode set before any byte is written — A replaced private file's temp file has its mode set before any byte is written, then is written, synced and persisted.
decision/real-cli-probe-local-only · Real CLI probes run only locally — The real claude probe and --record run only locally; CI runs verify against the fake agent's print mode and its interactive modes.
decision/screen-signatures-compiled · Screen signatures as compiled literals — Modal and input-box signatures are compiled literals validated by each CLI version's stamp; a screen is ready when no row holds a modal literal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char. · why: The input-box literal (shift+tab to cycle) is mode-independent and held by no other CLI text read.
decision/send-never-queued · A send mid-turn or mid-send is refused, never queued — A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
decision/send-waits-out-paste-hint · Send waits out the paste hint — On a verified CLI a send issued while the paste hint stands waits in the readiness gate for the input box, bounded at 8.5 s; a hint that outlasts the bound ends not-delivered / input-not-ready with nothing typed. · why: Ratified ruling.
decision/snapshots-read-through-replay · Snapshots read through the replay — Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no statusline command, and no frame is written and nothing runs.
decision/tolerant-external-payloads · External payloads parsed tolerantly — Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only. · why: The CLI changes between builds, and parsing must never break when the ledger decides behaviour.
decision/unverified-cli-transport-only · Transport-only degrade on an unverified CLI — On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial. · why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.
decision/verify-record-only-clean · Fixtures recorded only from a clean verify — viola verify --record writes its scrubbed fixtures only when 0 rows fail, and an unclean recording is refused whole, naming the file and the check code, never the content.
decision/verify-refuses-untrusted-home-or-stamps · Verify refuses an untrusted home tree or stamps — A strict-modes refusal of the home tree, before any probe, or of the stamps, again at the write, makes viola verify exit 1 with the home pair and leaves the stamps neither read nor written. · why: Run's version gate trusts the stamps verify writes, and the verify window test expects the widened stamps byte-equal with strict-modes-failed on the line.
contract/channel-answer/unknown-dialog · channel answer · unknown-dialog — An answer for a dialog that was not held pending is refused not-delivered with detail unknown-dialog.
contract/human-hint-lines/input-not-ready · Human hint lines · input-not-ready — Reads "<name> was not ready for input; send again, and if it repeats a human must look at the session" for a modal, a never-quiet screen, a poisoned screen or a hint past the bound; no hint line is printed for the paste-hint cause.
contract/not-delivered-details/empty-text · not-delivered refusal details · empty-text — A send whose typed text is left empty is refused `not-delivered` with this detail, nothing issued.
contract/not-delivered-details/input-not-ready · not-delivered refusal details · input-not-ready — A screen not ready at the readiness gate's bound is refused `not-delivered` with this detail, nothing typed.
contract/not-delivered-details/no-prompt-submitted · not-delivered refusal details · no-prompt-submitted — A send with no matching `prompt-submitted` within the compiled confirmation window is refused `not-delivered` with this detail.
contract/send-refusals/empty-text · send refusals · empty-text — A send whose typed text is empty is refused not-delivered / empty-text (exit 13) and never issued: nothing reserved, typed or logged as a send, and no wheel read.
contract/send-results/input-not-ready · send results · input-not-ready — A screen not quiet within the maximum wait, a poisoned model, a quiet screen still without an input-box row at the bound, or a modal on a verified CLI makes send type nothing and report not-delivered with detail input-not-ready.
contract/viola-revive-json-error-codes/strict-modes-failed · viola revive --json error codes · strict-modes-failed — A strict-modes check refused the start.
contract/viola-revive-json/error-document · viola revive --json · error document — Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.
contract/viola-verify-refusal-codes/strict-modes-failed · viola verify process-exit refusal codes · strict-modes-failed — Written on the process-exit verify logs under a parsed VIOLA_NAME when a strict-modes check refuses; a refused home tree writes no line.
contract/viola-verify-refusal-codes/unclean-fixture · viola verify process-exit refusal codes · unclean-fixture — Written on the process-exit verify logs under a parsed VIOLA_NAME when a recording is unclean; the fixture's file enters no code.
decision/ci-against-fake-agent · CI on ubuntu-latest against the fake agent — GitHub Actions on `ubuntu-latest` runs the tests against the fake agent; the real CLI runs only in local live tests. · why: No tokens spent, no flakes in CI, and no cross-compilers (cross 0.2.5 is stale).
decision/error-chain-to-diagnostics-only · Error chains reach only the post-mortem detail file — An anyhow error chain is written as chain to the instance's diagnostics detail file for the owner's post-mortem, never to a role line, stdout or stderr. · why: The owner's post-mortem is the one place a person reads the chain.
decision/hooks-fail-open · Hooks fail open — `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr. · why: Exit 2 blocks Claude Code, and a viola bug must never block the session. · except: The hidden `--capture` arms, called only by `viola verify`'s probes, read no `VIOLA_*` and open no channel, still exiting 0 with empty stderr.
decision/human-key-never-held-behind-resize · Human key never held behind a resize — viola never holds, queues or re-orders a human key behind a resize.
decision/human-wheel-renders-dialogs · Human-held wheel renders dialogs at once — While the wheel is `human`, `hook.dialog` is answered `null` at once and `answer` is refused `human-typing`; a dialog pending when the wheel moves to the human is answered `null` at once. · why: A dialog held by the hook is never hidden from the human for the rest of the hold.
decision/mcp-unreachable-result-for-every-cause · MCP frame tools report every unreachable cause one way — Every unreachable cause an MCP frame tool meets surfaces as the instance-unreachable result with the cause in detail and no line of its own; every cause but endpoint-lost is found before any frame is sent. · why: MCP drivers branch on the one instance-unreachable error and its closed detail; in the MCP process the result is the only record of the cause.
decision/no-live-endpoint-dead-or-unwrapped · No live endpoint reads instance-dead or unwrapped-name — Without a live endpoint, whether the wrapper is stale, gone or replayed, a client reads an existing instance directory as instance-dead and anything else as an unwrapped name. · why: The CLI frame verbs and the MCP frame tools take their exit 21 detail and hint from it, and the cross exit-cause tests pin both words.
decision/no-start-over-live-instance · No start over a live or stale instance — A `viola run` or `viola revive` whose `ViolaName` is `live` or `stale` refuses to start.
decision/pinned-copy-rehash · Pinned copy re-hashed before reuse — An existing pinned copy is re-hashed before reuse; a copy that no longer matches refuses the start with exit 1, pinned-hash-mismatch, and is left as found.
decision/run-refuses-held-name · A start refuses a held name — viola run and viola revive refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
decision/screen-rows-read-only-by-verify · Screen row text read only by verify — The screen model has one row-text exit, and only viola verify reads it.
decision/socket-dir-refused-covers-create-and-check · Socket-dir create failure and refusal read the same — A failed socket-directory create and a refused check both refuse the start as socket-dir-refused with exit 1. A failed bind does not: it reads squatted-name when another process holds the endpoint and internal-error otherwise. · why: Machine readers branch on the socket-dir-refused start document and exit line, and the cross exit-cause test pins it at exit 1 for a start.
decision/spine-hook-loses-event-not-turn · Spine hook loses the event, never delays the turn — A spine hook that cannot reach or write the endpoint within the deadline records the failure in `diagnostics/` and exits 0, so the event is lost. · why: The event is lost rather than the turn delayed; a driver recovers through `wait`'s `timeout_ms`.
decision/start-pins-the-bytes-it-checked · Start pins the very bytes it checked — A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked. · why: The strict-modes check of the pinned copy's bin and plugin folders and the pinned-hash-mismatch refusal both assume no second read of the exe.
decision/thiserror-one-enum-per-crate · thiserror, one error enum per workspace crate — Each workspace library crate except viola-core has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin. · why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError. · except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
decision/thiserror-per-crate-anyhow-in-bin · thiserror per crate, anyhow only in the bin — Each workspace crate except `viola-core` has one thiserror error enum; anyhow is used only in the `viola` bin. · why: Callers branch on refusal variants across three surfaces, and anyhow's chains serve only the owner's post-mortem, never a role line, stdout or stderr. · except: Ratified: `viola-pty`'s hand-written `PtyError`; `viola-mcp`'s `ServeError`; `viola-state`'s `StampsError`, `PinError`, `StampsReadError` and `Refused`.
decision/viola-name-prefix · viola prefix for names — The product, the binary and identifiers use the `viola` prefix, environment variables included (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`, newtype `ViolaName`). · why: One name everywhere removes the need to translate between prototype and product. · except: Ratified: test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC` (fake-agent builds only); `NO_COLOR`, `TERM`, `COLORTERM`, read never set, deciding only SGR bytes.
decision/vt100-readiness-gate · vt100 as a pre-send readiness gate only — vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no known modal signature and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model. · why: CLI-native modals bypass every hook, and the screen lags the hooks.
decision/wheel-byte-source-claim · Byte-source claim of the wheel — Any human editing key since the last turn boundary moves the wheel to the human and `send` is refused `human-typing`; only a closed non-editing list of keys never counts. · why: `viola run` owns the real stdin, so it knows byte provenance.
contract/dialog-hold-end/null-reply · Dialog hold end · null reply — At the hold's end the wrapper replies `null`, the hook exits 0 with no body, the dialog renders for the human, and a later `answer` for that dialog is refused `not-delivered` with detail `unknown-dialog`.
contract/event-line/link · The event line · link — The driven wrapper appends a `link` line, `source: wrapper`, only when the pair is not already in its link set; a refused `send` or `answer` and a `from` equal to the wrapper's own name link nothing.
contract/hook-diagnostics/server-verify-failed · Hook diagnostics · server-verify-failed — A hook whose serving process has another euid, is not the snapshot's pid and start time, or cannot be checked sends no frame, exits 0 with no body and records `server-verify-failed`.
contract/hook-diagnostics/socket-dir-refused · Hook diagnostics · socket-dir-refused — A hook meeting a refused socket directory sends no frame, exits 0 with empty streams and records one `hook-decision` at WARN with `detail` `socket-dir-refused`.
contract/mcp-stdin-oversize/parse-rejected · MCP stdin oversize process log · parse-rejected — A session ended by an oversize line writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line at WARN, holding no byte of the line.
contract/process-log/link-channel-lines · Wrapper process log · link and unlink channel lines — The `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name.
contract/send-refusal/manual-pause · Send refusal details · manual-pause — After `viola pause`, automated `send` is refused `human-typing` with detail `manual-pause`.

## conventions · ## Conventions
decision/nutype-errors-not-folded · nutype validation errors stay out of crate enums — A nutype validation error is never folded into a crate error enum: the caller drops it, refusing the input (a usage error, invalid params, a closed field word) or treating it as absent (a config default with a rejection, no instance).
decision/refusal-details-closed-set · Refusal details from a closed set — Each refusal reason's `detail` comes from a closed set per reason, extended only in `viola-core`.
decision/viola-name-instance-key · ViolaName as the instance key — The instance key is `ViolaName`: ASCII `[a-z0-9-]`, 1 to 32 characters, starting with a letter, enforced by nutype. · why: No database, so no UUID or serial keys.
contract/cli-exit-21/strict-modes-failed · CLI exit 21 · strict-modes-failed — A refused strict-modes check of send, wait, last, answer, pause or release exits 21: human mode prints the unable: / hint: pair, --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} with stderr empty; nothing under the home is opened and no line is written.
contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
contract/cli-exit-codes/21 · CLI exit codes · 21 — Instance unreachable: no live endpoint, a server that is not the recorded wrapper, a refused strict-modes check of the board, the plugin install tree or the target or driven instance, or a refused socket directory; no frame is written.
contract/cli-internal-error/output · CLI internal error output · internal error line — A dispatch error or caught panic in a cli-role verb prints exactly error: internal error on stderr once, with no hint; under --json before a -- the internal-error document is printed on stdout instead.
contract/exit-21-human-hints/placement · Exit 21 human hints · placement — Each exit 21 human hint (unwrapped name, endpoint lost, socket directory) is the last line on stderr and is plain ASCII.
contract/mcp-frame-tools/strict-modes-failed · MCP frame tools · strict-modes-failed — A send, wait, last or answer first checks strict modes on the home and the target instance; a refusal returns the instance-unreachable isError result with detail strict-modes-failed, text refused: instance-unreachable, no frame and no line.
contract/refusal-reason/budget-paused · RefusalReason · budget-paused — Serialised kebab-case; its detail is five-hour or seven-day.
contract/refusal-reason/detail · RefusalReason · detail — A kebab-case string from a closed set per reason, extended only in viola-core; readers treat an unrecognised code-word detail as opaque text, never as an error.
contract/refusal-reason/human-typing · RefusalReason · human-typing — Serialised kebab-case; its detail is null or manual-pause.
contract/refusal-reason/not-delivered · RefusalReason · not-delivered — Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
contract/refusal-reason/unknown · RefusalReason · `unknown` refusal reason — The catch-all reason (`#[serde(other)]`), its detail a code word or null; an unrecognised code word is opaque text to a reader.
contract/refusal-reason/unknown-catch-all · RefusalReason · unknown catch-all — The serde catch-all, so an older client can still read a refusal from a newer wrapper; its detail is a code word or null.
contract/refusal-reason/unverified-cli · RefusalReason · unverified-cli — Serialised kebab-case; its detail is null.
contract/run-start-refusals/home-tree · viola run and revive start refusals · home tree — A refused home tree exits 1 with the home-state or instance-state pair, opens no log, writes no line and starts no child; under revive --json it is the strict-modes-failed document.
contract/run-start-refusals/socket-dir-refused · viola run and revive start refusals · socket-dir-refused — A refused or failed socket directory exits 1 with an unable and hint pair, or the socket-dir-refused document under revive --json, and one process-exit line; no path, pid or uid; a bind failure is not this code.
contract/verify-process-exit/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — The code verify's process-exit carries on a strict-modes refusal.
contract/verify-process-exit/unclean-fixture · viola verify process-exit detail · unclean-fixture — The code verify's process-exit carries for an unclean recorded fixture; the fixture's file enters no code.
contract/verify-refusals/home-tree · viola verify refusals · home tree — A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines, and with an instance set its process-exit carries strict-modes-failed.
decision/first-refusal-only · Only the first refusal returned — Of several refusals only the first is returned, in a fixed order per verb, `control-character` first.
decision/one-error-enum-per-crate · One error enum per crate — Each crate has one error enum named `<Crate>Error`, each a `thiserror` enum except `PtyError`, whose `Display` and `Error` are hand-written with fixed messages. · except: Ratified exceptions: `ServeError` beside `McpError`, `StampsError` beside `StateError`, `PinError`, `StampsReadError` and `Refused`.
decision/silent-roles-at-internal-error-catch · Run, revive, mcp and hook silent at the catch site — Run, revive's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there. · why: MCP stdout carries only protocol frames, a hook must stay silent to fail open, and a wrapper's terminal belongs to its child.
contract/cli-exit-codes/10 · CLI exit codes · 10 — The `human-typing` refusal.
contract/cli-exit-codes/11 · CLI exit codes · 11 — The `budget-paused` refusal.
contract/cli-exit-codes/12 · CLI exit codes · 12 — The `unverified-cli` refusal.
contract/cli-exit-codes/13 · CLI exit codes · 13 — The `not-delivered` refusal.
contract/cli-exit-codes/14 · CLI exit codes · 14 — The `unknown` refusal.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Usage error, and viola send text over 16 MiB or not UTF-8; never produced by viola hook.
contract/cli-json-error-document/shape · CLI --json error document · shape — One stdout document of v 1, error code and detail, with stderr empty; codes include strict-modes-failed, already-live, no-session, session-live, cwd-missing, stale-heartbeat, pinned-hash-mismatch, squatted-name, socket-dir-refused, internal-error.
contract/client-server-verify/check-order · Client server verification · check order — The client's own refusals, the strict-modes instance check and the socket-directory check come before any connect; the cause has no exit code or RefusalReason of its own.
contract/client-server-verify/server-verify-failed · Client server verification · server-verify-failed — A connected server that is not the recorded wrapper gets no frame; the verb exits 21 with the instance-unreachable document of detail server-verify-failed under --json, or the unable line and server-verify hint on stderr.
contract/mcp-tool-results/wrapper-fault · MCP tool results · wrapper-fault — A wrapper protocol fault is an isError true result {"error":"wrapper-fault","detail":…} carrying the sanitised JSON-RPC fault {code,message,data}: a fixed message, data a closed object or null.
contract/normalised-event-kinds/link · normalised event kinds · link — `link` is a normalised event kind.
contract/paste-text/control-characters · Paste text · control characters — LF, CR and TAB pass; every other C0, DEL and C1 is refused as not-delivered control-character, never stripped; the CLI checks before any frame and the wrapper checks again.
contract/paste-text/empty · Paste text · empty text — A text empty after CR LF and lone CR become LF and trailing CR and LF are dropped is refused as not-delivered empty-text, by the CLI and again by the wrapper; one also holding a refused character is control-character.
contract/peer-reply/refusal · A peer's reply · refusal — A refusal that is no code word is read as unknown.
contract/refusal-order/answer · Refusal order · answer — control-character on the response's free text, then human-typing, unverified-cli, then unknown-dialog.
contract/refusal-order/send · Refusal order · send — control-character, empty-text, human-typing, budget-paused, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, then no-prompt-submitted.
contract/send/paste-failure-after-issue · send refusals · paste failing after send-issued — A paste that fails after send-issued is refused as input-not-ready with that send's cursor.

## standard-contracts · ## Standard Contracts
decision/client-strict-modes-before-frame · Strict-modes check before any client frame — Each channel client runs the instance's strict-modes check before the server comparison, a refusal writing no frame and no line, and only a real snapshot, not a replayed one, goes on to the comparison.
decision/link-set-changes-only-on-link-calls · Link set changes only on implicit links and link calls — The wrapper's link set changes only on an implicit link from a send or an answer and on a served link or unlink call, with each change appended to the instance log first and then mirrored to the snapshot. · why: Restoring the set at start replays only link and unlink lines, so the snapshot and the log agree only while no other path changes the set.
decision/link-set-follows-log · Link set follows the log — The wrapper restores its link set from the log at every start without appending or logging; a failed append leaves the set as it was, and a landed line is in the set even when the snapshot write then fails.
decision/malformed-stamps-unverified · Malformed or untrusted stamps read unverified — A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.
decision/replay-writes-no-file · Snapshot replay writes no file — Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.
decision/snapshot-replay-log-fields-only · Snapshot replay recovers only log-derived fields — A snapshot rebuilt by replay recovers only links, agent_session_id, wheel and the budget gate from the log; a field no line gave is absent, never a default.
decision/stamps-writer-checks-before-and-under-lock · Stamps writer checks strict modes before and under its lock — The stamps writer runs the strict-modes check of the ledger folder and the stamps file before creating the folder and again under its lock, and a refusal reads no stamps, never runs the update and writes none. · why: Verify stamps the home through it and run trusts those stamps; a path another user could write must not be read or rewritten.
contract/channel-methods/hook-dialog-unverified-cli · channel methods · hook.dialog unverified CLI — On an unverified CLI the wrapper replies `null` to `hook.dialog` at once.
contract/channel-methods/send · channel methods · send — A request: `send` `{text, from?}` answers `{submitted_at, cursor}` when confirmed, or `{confirmed:false, detail:"unconfirmable", cursor}`; `cursor` is the end offset of `events.ndjson` when the send was accepted, before the paste.
contract/channel-methods/send-budget-paused · channel methods · send budget-paused — A closed budget gate refuses `send` `budget-paused` before `send-issued`, with nothing typed.
contract/channel-send/budget-paused · channel send method · budget-paused — A closed budget gate read at arrival refuses with budget-paused, detail five-hour or seven-day, before send-issued, with no cursor, nothing typed and no flight reserved.
contract/channel-send/result · channel send method · result — {text, from?} answers {submitted_at, cursor} when confirmed, or {confirmed:false, detail:"unconfirmable", cursor}; cursor is events.ndjson's end offset when the send was accepted, before the paste.
contract/cli-exit-21-document/strict-modes-failed · exit-21 CLI document · strict-modes-failed — A refused strict-modes check of the home and target instance prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout under --json (exit 21), stderr empty, no log line.
contract/cli-json-output/result · CLI `--json` output · channel result — Mirrors the channel `result`: `{"v":1,"ok":{…}}` or `{"v":1,"refusal":"human-typing","detail":null}`, with the matching exit code.
contract/instance-snapshot/links · instance snapshot · links — `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and a failed append leaves it as it was.
contract/revive-json/exit-1-document · viola revive --json · exit-1 document — Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
contract/revive-json/refusal-document · viola revive --json · start refusal document — The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`; the replay writes no file.
decision/frame-client-check-order · Frame-writing clients run their checks in one order — A CLI or MCP frame call runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order; the hook keeps the same order without the liveness pre-check. · why: Each refusal maps to its own exit-21 cause, and no check may open the home or write a frame before the ones ahead of it pass.
decision/hook-output-one-crate · Hook output produced by one crate — The Claude Code hook contract is produced only by the `viola-agent-claude` crate.
decision/option-text-never-in-human-line · Upstream option text kept out of human lines — Question option text is upstream content: carried into the event log, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line.
decision/statusline-check-before-log · Statusline instance check before the log — The statusline arm runs the instance check before it opens its log or reads anything under the home. · why: Opening the log sets an existing home to 0700, so a check made after it passes a home at 0770.
contract/answer-refusals/unknown-dialog · answer refusals · `unknown-dialog` — A late `answer` to a `question` first raised by PermissionRequest with no matching armed dialog is refused `unknown-dialog`.
contract/channel-link-unlink/answer-order · channel link and unlink methods · answer order — Each answers only after its event line and the snapshot write.
contract/channel-link/already-linked · channel link method · already_linked — A link of a pair the set holds appends nothing and answers that pair's since with already_linked true; a new pair answers its new link line's ts as since with already_linked false.
contract/channel-methods/answer · channel methods · answer — A request: `answer` `{dialog_id, from?, response}` answers `{}`, with `response` normalised per dialog kind.
contract/channel-methods/from · channel methods · from — `from` is the caller's `VIOLA_NAME` when set, else omitted; `send`, `wait`, `last`, `answer` and `pause` take it absent, `null` or a valid `ViolaName` and answer anything else `-32602` with `data: null`, never `-32600`.
contract/cli-exit-21-document/endpoint-lost · exit-21 CLI document · endpoint-lost — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/instance-dead · exit-21 CLI document · instance-dead — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/server-verify-failed · exit-21 CLI document · server-verify-failed — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-exit-21-document/socket-dir-refused · exit-21 CLI document · socket-dir-refused — A refused socket directory met at connect prints the instance-unreachable document with this detail (exit 21) and one process-exit line at WARN.
contract/cli-exit-21-document/unwrapped-name · exit-21 CLI document · unwrapped-name — One of six closed detail causes; v 1, error instance-unreachable.
contract/cli-json-output/instance-unreachable · CLI `--json` output · `instance-unreachable` — `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with exit 21, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.
contract/cli-json-output/internal-error · CLI `--json` output · `internal-error` — `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent.
contract/cli-json-output/plugin-install-strict-modes-refused · CLI `--json` output · `viola plugin install --json` strict-modes refusal — A refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` and exit 21.
contract/cli-json-output/revive-refusal · CLI `--json` output · `viola revive --json` refusal — A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.
contract/event-data/link · event data · `link` — `{driver, driven}`.
contract/event-data/send-confirmed · event data · `send-confirmed` — `{cursor, confirmed?}`: a confirmed send writes `{cursor}`, an unconfirmable send writes `{cursor, confirmed:false}`, the one record of that outcome (no `send-refused`); the field is additive, no `v` bump, and no product code reads the record.
contract/event-data/send-refused · event data · `send-refused` — `{refusal, detail, cursor?}`; `cursor` is absent for a refusal before `send-issued`.
contract/hook-event/notification · channel hook.event · notification — Sent id-less with no reply only after the instance, socket-directory and server checks pass; otherwise nothing is sent and the hook fails open. Shape {ts, event:{kind, data}}.
contract/hook-statusline/exit · viola hook statusline · exit status — Always exits 0 with empty stderr.
contract/hook-statusline/refused-check · viola hook statusline · refused instance check — A refused check ends the arm at once with exit 0: nothing under the home read or opened, no stdin read, no command run, nothing printed, `budget.json` untouched and the home's mode left as found.
contract/instance-snapshot/endpoint · instance snapshot · endpoint — `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
contract/instance-snapshot/pending-dialog · instance snapshot · pending_dialog — `{dialog_id, kind}` while a `hook.dialog` is open, written before its event line and reset when that append fails, so a reader that sees the line sees it; removed on answer, expiry or a `null` answer on a wheel move; never rebuilt from the log.
contract/mcp-list/refusal · MCP list · refusal — The board's refusal, its only one, is returned as instance-unreachable with detail strict-modes-failed.
contract/snapshot-envelope/v · snapshot envelope · v — Read first: above 1 is unsupported whatever else the envelope holds, 1 must parse whole, anything else is unreadable; an unsupported or unreadable snapshot is ignored and the state rebuilt by replaying events.ndjson.
contract/wrapper-channel-frames/refusal-result · wrapper channel frames · refusal result — A refusal is answered as `result` holding `{"refusal": <reason>, "detail": <detail>}`.

## occupied-resources · ## Occupied Resources
decision/board-refused-whole · Whole board refused on one failed check — viola list checks the home, instances/ and every instance before opening any file, and the first failure refuses the whole board with exit 21, no log and no line. · why: Ratified as the founder's own answer.
decision/diagnostics-open-checks-modes · Diagnostics files checked before open — Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open, ending the role writer at a sink or dropping the detail line.
decision/fixture-keeps-own-repository-remote · Fixture keeps this repository's own remote — A workspace.repo naming this repository's own remote is kept in a fixture rather than scrubbed. · why: ratified
decision/list-home-read-rule · viola list holds the home to the read rule — viola list holds the home to the strict-modes read rule with or without an instance, so a 0750 home refuses the whole board. · except: The budget read, mcp's start and plugin install keep the write rule on the home.
decision/no-live-claude-in-ci · No live claude in CI — When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables. · why: so the real claude never runs in CI
decision/path-under-a-file-is-a-strict-modes-refusal · A path under a regular file is a strict-modes refusal — A trusted path whose stat fails for any reason other than absence, as under a regular file, is refused as unreadable: verify exits 1 with the home's pair and the frame verbs exit 21 with the instance's pair. · why: Only an absent path may be skipped by the check; verify's regular-file test pins the exit code and the pair.
decision/recorded-fixtures-scrubbed · Recorded fixtures scrubbed and hygiene-walked — Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content. · guard: contract_fixture_hygiene
decision/refused-check-writes-no-line · Refused check writes no line — A refused strict-modes or instance check opens no role file, writes no line and leaves the home's mode as found.
decision/screen-fixtures-signature-rows-only · Screen fixtures keep only signature rows — A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording.
decision/stamps-single-writer · Ledger stamps have one writer — ledger/stamps.json has one writer, called only by viola verify, which checks strict modes before and under the lock and replaces the file whole; a refusal reads and writes nothing.
decision/strict-check-before-log-open · Strict-modes check before any log opens — Every checked entry point runs its strict-modes or instance check before any log opens or any file under the home is read. · why: Opening a log sets an existing home to 0700, so a check made after it would pass a home at 0770.
decision/strict-modes-check-before-log · Strict-modes check before any log — Every command that uses the viola home or an instance's files checks their modes, a folder before what it holds, before opening any log; a refused check exits 1 with the home-state or instance-state unable and hint pair, writes no line and starts no child. · why: opening a log narrows an existing home
decision/strict-read-rule · Strict-modes read rule — The strict-modes read rule is the write rule plus a refusal of any group or other permission bit.
decision/version-gate-strict-stamps-read · Version gate reads stamps strictly — run's version gate reads the stamps only after the owner-only strict-modes check; a refusal records cli_verified false and one parse-rejected ledger-stamps strict-modes-failed line.
contract/diag-line/format · diagnostics line · format — Owned by obs and fixed by schemas/diag-line.v1.json; one line per write; folder 0700, files 0600.
contract/frame-verb-exit-codes/21 · send, wait, last, answer, pause and release exit codes · 21 — The instance check refused, the socket folder was refused, or the server runs under another euid; human form puts the unable/hint pair on stderr, --json the instance-unreachable document on stdout.
contract/instance-unreachable-detail/strict-modes-failed · the instance-unreachable --json document · strict-modes-failed — The detail when the instance check refused the instance.
contract/unclean-fixture-refusal/absolute-path · unclean fixture refusal · absolute-path — The code given when a recording holds an absolute path.
contract/unclean-fixture-refusal/email · unclean fixture refusal · email — The code given when a recording holds an email-shaped token.
contract/unclean-fixture-refusal/home-path · unclean fixture refusal · home-path — The code given when a recording holds a home path.
contract/unclean-fixture-refusal/message · unclean fixture refusal · message — An unclean recording is refused whole as `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen; the line never names the content.
contract/unclean-fixture-refusal/username · unclean fixture refusal · username — The code given when a recording holds the username, including one split across a screen row seam.
contract/verify-exit-detail/strict-modes-failed · viola verify process-exit detail · strict-modes-failed — A refusal code verify writes under a parsed VIOLA_NAME; a refused home tree writes no line.
contract/verify-exit-detail/unclean-fixture · viola verify process-exit detail · unclean-fixture — A refusal code verify writes under a parsed VIOLA_NAME; the fixture's file name enters no code.
contract/viola-mcp-exit-codes/1 · viola mcp exit codes · 1 — The session ended before initialize, or the home or diagnostics folder failed the strict-modes check, with nothing on stdout or stderr and no role file.
contract/viola-verify-exit-codes/1 · viola verify exit codes · 1 — The home-tree check refused the home, ledger, bin or plugin folder, before any log or step line, or the stamps update was refused; the home-state unable/hint pair is printed.
decision/codes-only-role-logs · Codes-only role logs — Home-level role logs hold codes only, with no row text, prompt, id, path or endpoint; content-bearing detail goes only to the instance's owner-only detail files.
decision/colour-env-decides-only-sgr · Colour variables decide only SGR output — Every viola build's human output reads NO_COLOR, TERM and COLORTERM, never sets them, only compares them, never echoes them, and they decide only whether and how deep SGR bytes are written; COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read. · why: ratified: the colour variables are to be read
decision/dev-host-e2e-home-on-tmpfs · Dev-host test homes on a tmpfs link — On the Linux dev host the e2e-home base is a link to a tmpfs, an environment fact named by no variable, flag or config key; the home path strings are unchanged and CI homes stay on the runner's disk. · why: so a start's pinned copy does not wait on the shared volume's dirty-page budget
decision/linked-e2e-home-base-strict · Linked e2e-home base must be a private real directory — A linked e2e-home base must have an absolute target, created with mode 0700 when gone and never chmodded after, and is accepted only when it reads as a real directory with no group or other bit; anything else refuses the start with a fixed message naming no path.
decision/process-logs-hold-codes-only · Home-level process logs hold codes only — Home-level per-role process logs carry codes only, one line per write; content-bearing detail stays in the instance detail files.
decision/refused-statusline-check-no-trace · Refused statusline check leaves no trace — A refused instance check on viola hook statusline exits 0 at once with no config read, no role file, no line, no stdin read, no command run, nothing printed and budget.json untouched. · why: Ratified as the founder's own answer.
decision/root-waits-share-one-bound · Root test waits share one bound — Root test waits on a child share the one 7 s WITHIN bound of the watch support.
decision/test-evidence-survives-kill · Test evidence survives a killed test — Real-process test rigs stream step evidence to report files outside the viola home, kept when the test panics or is killed and removed when it passes; viola never reads or writes them. · why: so a test the runner kills keeps its evidence
decision/test-home-owner-sweep · Test homes removed by owner record only — A test home sweep removes only dirs whose recorded owner process is gone, never by age or name and never a dir without a record, and every removal deletes the owner record last. · why: a removal that stops part-way keeps the record for a later sweep
decision/test-homes-cleaned-by-owner · Test homes cleaned by owner process — Harness and test homes are kept in CI for the obs gates, and a new test home removes only the dirs whose recorded owner process is gone, never by age or name.
decision/verify-sole-stamps-writer · verify is the one stamps writer — viola verify is the one writer of the ledger stamps; run's version gate reads them behind the strict-modes check, and a refusal records cli_verified false.
decision/wait-report-kept-on-failure · Wait report kept on failure — A test wait writes a report created empty at its start, with a line only when the polled state changes, codes, counts and booleans only; the report is kept when the test panics or is killed and removed when it passes. · why: A test the runner kills keeps its evidence.
contract/child-environment/viola-name · The wrapped child's environment · VIOLA_NAME — Set by viola run to the instance's ViolaName; when it is absent, names no existing instance directory, or the instance fails its strict-modes check, every hook event and dialog path exits 0 silently, reading no config, no stdin and writing nothing.
contract/diag-line/state-recovered · diagnostics line · state-recovered — A snapshot the replay rebuilds from the log writes one state-recovered line into the role file of the process that read it, one line per replay.
contract/diagnostics-lines/detail-line · Diagnostics lines · instance detail line — Format owned by obs and fixed by the default-deny, self-contained schema diag-detail.v1.
contract/diagnostics-lines/process-log-line · Diagnostics lines · home-level process-log line — Format owned by obs and fixed by the default-deny, self-contained schema diag-line.v1.
contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and the caller's VIOLA_NAME for list, verify, revive --list and plugin install.
contract/diagnostics-role-files/hook-name · home-level diagnostics role files · hook-<name>.ndjson — The hook's hook-invoked, hook-decision and parse-rejected lines.
contract/diagnostics-role-files/mcp · home-level diagnostics role files · mcp.ndjson — viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes.
contract/diagnostics-role-files/run-name · home-level diagnostics role files · run-<name>.ndjson — The wrapper's codes-only process lines, viola revive's start arm included.
contract/frame-verb-exit-detail/endpoint-lost · CLI frame verb process-exit detail · endpoint-lost — Exit 21 with during call, for a wrapper lost with the call open.
contract/frame-verb-exit-detail/instance-dead · CLI frame verb process-exit detail · instance-dead — Exit 21 with during connect, at WARN.
contract/frame-verb-exit-detail/server-verify-failed · CLI frame verb process-exit detail · server-verify-failed — Exit 21 with during connect when the serving process is not the recorded wrapper or runs under another euid; no channel-request line.
contract/frame-verb-exit-detail/socket-dir-refused · CLI frame verb process-exit detail · socket-dir-refused — Exit 21 with during connect, at WARN, no channel-request line; run and revive's start write it with exit_code 1.
contract/frame-verb-exit-detail/unwrapped-name · CLI frame verb process-exit detail · unwrapped-name — Exit 21 with during connect.
contract/instance-unreachable-detail/server-verify-failed · the instance-unreachable --json document · server-verify-failed — The detail when the server runs under another euid.
contract/instance-unreachable-detail/socket-dir-refused · the instance-unreachable --json document · socket-dir-refused — The detail when the recorded socket folder was refused, past the instance check.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — The detail of a stopped run; a failing ledger row is no refusal and carries no code.
contract/viola-link-exit-codes/21 · viola link and unlink exit codes · 21 — The instance check refused, no live endpoint is recorded, the recorded endpoint's folder is refused, or the serving process is not the recorded wrapper or runs under another euid.
contract/viola-list-exit-codes/21 · viola list exit codes · 21 — The strict-modes check refused the home, instances/ or any one instance, and the whole board is refused.
contract/viola-plugin-install-exit-codes/21 · viola plugin install exit codes · 21 — The strict-modes check refused the home or an existing marketplace tree folder; nothing created, no log opened.
contract/viola-revive-exit-codes/1 · viola revive exit codes · 1 — A refused strict-modes check, a preflight refusal, or a refusal of the start after a passed preflight.
contract/viola-run-start-exit-codes/1 · viola run and revive start exit codes · 1 — The home-tree check refused, before any log, or the socket folder was refused or could not be created, before any snapshot, child or socket bind.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/cli-verbs-human-stderr · Short-lived CLI verbs may use stderr — Short-lived CLI verbs may use stderr for their human output.
decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp or cli ndjson).
decision/no-other-os-test-gates · No test gate naming another OS — A test gate whose predicate names windows or macOS is a fault even when the gates table names it. · guard: census_reads_a_gate_naming_another_os_as_a_fault_even_when_named
decision/os-gated-tests-name-reason · OS-gated tests name their reason — Every OS-gated test names its reason in the census, and a gate whose predicate names windows or macos reads as a fault even when a row names it. · guard: contract_os_gated_tests
decision/sanitised-error-surfaces · Sanitised error surfaces — CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value. · guard: contract_error_bodies: product_error_writers_are_each_named · except: A code word outside the closed list stays visible by design.
decision/unconfirmed-delivery-not-delivered · Unconfirmed delivery reported as not-delivered — Any failure to confirm delivery is reported as not-delivered and never retried blindly. · except: A ledger local command whose post-condition is none or not yet measured on this CLI version is reported as ok with confirmed false and detail unconfirmable.
contract/exit-21-human-hints/socket-dir · Exit 21 human hints · socket-dir — The unable: / hint: pair for a socket directory names only the parsed instance name, no path, pid or uid, is plain ASCII with hint: last and no SGR.
decision/agents-join-read-ungated · Agents join read not gated on a stamp — viola list and viola revive do not gate their claude agents --json read on a ledger stamp. · why: The read only enriches and its failure is never an error.
decision/bounded-readers · Bounded readers at every boundary — Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded. · guard: contract_bounded_readers: product_readers_are_each_bounded_or_named · except: The own-executable reads, which are no external input.
decision/ci-linux-fake-agent · CI on Linux against the fake agent — CI runs on GitHub Actions ubuntu-latest against the fake agent.
decision/content-detail-per-instance · Content-bearing detail only in the instance — Content-bearing diagnostic detail belongs only in the instance's per-role detail ndjson file.
decision/crash-safe-disk-writes · Crash-safe disk writes — Append-only logs use one write per line, mutable files are replaced atomically, every guarded file has its own .lock sibling, and readers always tolerate a torn last line.
decision/hook-exits-on-untrusted-instance · Hook exits at once on an untrusted instance — viola hook takes its instance only when the VIOLA_DIR instance directory exists and passes the strict-modes check; otherwise it exits 0 at once with no config read, no stdin read, no line, no frame and no SessionEnd append. · except: The two --capture arms, which read no VIOLA_* variable, run ahead of the check.
decision/hook-fails-open · Hook fails open toward the human — Any internal failure in viola hook produces exit 0 with no body, so the dialog renders for the human; viola hook always exits 0.
decision/os-gated-tests-named · OS-gated tests named with their reason — Every test held off a runner by an OS gate is named in the gates table with a non-empty reason, and the table names no gate that no longer stands. · guard: contract_os_gated_tests: os_gated_tests_are_each_named_with_their_reason
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — A thiserror error message interpolates no value. · guard: thiserror_messages_interpolate_nothing
decision/tokio-containment · Tokio containment — Code on the run, hook, send and channel-server paths uses std threads and blocking I/O; only viola-mcp may build a Tokio runtime, and CLI wait is a blocking read on the channel's sync client.
decision/untrusted-upstream-text · Upstream text is untrusted — A driven session's Stop text and dialog payloads are content, never commands to viola; viola forwards them as escaped strings and never interprets them.
contract/hook-output/stdout · Hook output · stdout and stderr — viola hook writes only its decision body to stdout and nothing to stderr.
contract/mcp-output/stdout · MCP server output · stdout and stderr — viola mcp writes only MCP frames to stdout; when its role file cannot be opened it exits 1 and prints nothing.

## inherited-defaults · ## Inherited Defaults
decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude.
contract/channel-result/refusal · Channel result · refusal — A refusal is a typed RefusalReason in result.refusal.
decision/ndjson-logs-and-json-snapshots · State in ndjson logs and JSON snapshots — State is kept in ndjson append logs and atomic JSON snapshots guarded by std file locks, with serde_json.

## build-system · key of ## Infrastructure Patterns
decision/no-orphan-modules · No orphan modules — No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans. · guard: scripts/orphans-check.sh
decision/no-print-macros · No print macros — The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them. · guard: tests/contract_lints.rs · except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
decision/single-human-writer · One human writer in the root bin — The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
contract/human-stderr/refusal · Human stderr · refusal — A refusal writes the two fixed `unable:` / `hint:` lines as one write.
decision/banned-features · Banned dependency features — The features veil/toggle, rmcp auth and transport-streamable-http-server, and tracing-subscriber/env-filter are banned. · guard: cargo deny check
decision/workspace-dependency-pins · Third-party versions pinned in one place — Every third-party dependency version is pinned once in the workspace dependency table and shared by the members that take it.

## ci-cd-approach · key of ## Infrastructure Patterns
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
decision/real-claude-local-only · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally. · guard: tests/cli_verify.rs
decision/tests-against-fake-agent · Tests run against the fake agent — The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.

## crate-dependency-direction · key of ## Infrastructure Patterns
decision/agent-claude-pure · viola-agent-claude does no file I/O — viola-agent-claude takes no viola-state dependency and writes no file; stamps, budget and settings files are written by viola-state and the root bin. · why: So the crate stays pure.
decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.

## project-directory-structure · key of ## Infrastructure Patterns
decision/pty-seam-portable-pty · portable-pty is the PTY seam — The PTY seam is built over portable-pty, with host raw mode through libc termios.

# architecture · its absences, names and overviews, whole

## stack-and-technologies · ## Stack and Technologies
absence/model-api-calls — Calling any model API; viola drives the unmodified claude CLI on the user's subscription
overview/stack-and-technologies — viola is one native Rust binary: a std-thread core with Tokio built only inside `mcp`, clap for the CLI, portable-pty behind viola's own `pty` seam, vt100 for readiness, interprocess local sockets carrying JSON-RPC 2.0 over ndjson, ndjson append logs and atomically replaced JSON snapshots with no database, broker, container or hosting, rmcp for the stdio MCP server, and GitHub Actions on `ubuntu-latest` against the fake agent.

## established-decisions · ## Established Decisions
absence/daemon — A daemon
absence/graphical-frontend — A web or graphical frontend, page or graphical control surface
absence/mobile-view — A mobile or phone view
absence/windows-and-macos-support — Support for Windows or macOS

## conventions · ## Conventions
absence/database — A database

## standard-contracts · ## Standard Contracts
absence/graphical-interface — A web or graphical surface
absence/polling-endpoints — Polling endpoints (v1)
absence/websocket — WebSocket endpoints (v1)

## occupied-resources · ## Occupied Resources
absence/listening-network-port — A listening network port of any kind: no database port, no broker port, no listening TCP port
name/binary/viola — viola
name/binary/viola-fake-agent — viola-fake-agent
name/binary/viola-harness — viola-harness
name/binary/viola-mcp-tap — viola-mcp-tap
name/command/answer — answer
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/verify — verify
name/command/wait — wait
name/crate/viola — viola
name/crate/viola-agent-claude — viola-agent-claude
name/crate/viola-channel — viola-channel
name/crate/viola-core — viola-core
name/crate/viola-e2e — viola-e2e
name/crate/viola-fuzz — viola-fuzz
name/crate/viola-mcp — viola-mcp
name/crate/viola-pty — viola-pty
name/crate/viola-state — viola-state
name/other/agent-run-chunk-base — AGENT_RUN_CHUNK_BASE
name/other/agent-run-keep-failed — AGENT_RUN_KEEP_FAILED
name/other/agent-run-keep-homes — AGENT_RUN_KEEP_HOMES
name/other/fake-agent-hook-panic — FAKE_AGENT_HOOK_PANIC
name/other/fake-agent-pump-delay-ms — FAKE_AGENT_PUMP_DELAY_MS
name/other/hook-notification — hook notification
name/other/hook-permission-request — hook permission-request
name/other/hook-post-tool-use — hook post-tool-use
name/other/hook-post-tool-use-failure — hook post-tool-use-failure
name/other/hook-pre-tool-use — hook pre-tool-use
name/other/hook-session-end — hook session-end
name/other/hook-session-start — hook session-start
name/other/hook-statusline — hook statusline
name/other/hook-stop — hook stop
name/other/hook-user-prompt-submit — hook user-prompt-submit
name/other/mcp-serve-test-child — MCP_SERVE_TEST_CHILD
name/other/mcp-tool-answer — answer
name/other/mcp-tool-last — last
name/other/mcp-tool-list — list
name/other/mcp-tool-send — send
name/other/mcp-tool-wait — wait
name/other/viola — viola
name/other/viola-at-viola — viola@viola
name/other/viola-bin — VIOLA_BIN
name/other/viola-dir — VIOLA_DIR
name/other/viola-h12-sock — viola-<h12>.sock
name/other/viola-name — VIOLA_NAME
name/other/viola-test-chan-pid-label — viola-test-chan-<pid>-<label>
name/other/viola-verify-probe — viola-verify-probe
name/server/viola — viola

## cross-cutting-patterns · ## Cross-cutting Patterns
absence/secrets — Secrets of any kind in v1 configuration

## project-intent · ## Project Intent
absence/web-or-graphical-frontend — A web or graphical frontend
overview/project-intent — viola is a hybrid local developer tool that lets one interactive Claude Code session drive another on the user's own subscription. It has a native Linux CLI binary and a Claude Code plugin of hooks and a stdio MCP server that calls that binary; a keyboard-driven terminal fleet console is planned, not built. It wraps the unmodified claude CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment. It is a modular monolith: one binary with internal crates around core, and no daemon.

## inherited-defaults · ## Inherited Defaults
absence/containers-and-hosting — Containers or hosting of any kind

## ci-cd-approach · key of ## Infrastructure Patterns
absence/deploy-stage — A deploy stage

## deployment-model · key of ## Infrastructure Patterns
absence/container-or-serverless-deployment — Container, orchestration or serverless deployment (Docker, Compose, Kubernetes, serverless)
