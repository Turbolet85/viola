## 2026-10-26-explicit-session-links — the served `link` / `unlink` methods behind the IPC controls
**Section:** Authentication & Authorization (authentication-authorization.md: RBAC / permissions); Input Validation (input-validation.md: Channel frames); Threat Model Summary (threat-model-summary.md: the Local IPC vector)
**Change:**
- RBAC / permissions: the full surface a caller passing the OS check gets is named — the nine served methods (`send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`) and the `hook.event` notification, each behind the IPC rows and its own params check (was "the full surface", unnamed).
- Channel frames: the wrapper's `link` / `unlink` params rule — `driver` and `driven` are strings that pass `ViolaName::try_new` (one shared check); a missing, non-string or invalid name, a `driven` other than the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` `"invalid params"` with `data: null`, no line written; neither method reads `from`, so a `from` of any shape changes nothing.
- Local IPC vector: whoever can connect can also add or remove a driver link, whose effect is a `link` / `unlink` line in `events.ndjson` and the snapshot's links only; neither method reads the self-reported `from`.
**Why:** the chunk's wrapper serves `link` / `unlink` (both answered `-32601` before), routed to its `LinkSet`. Serving them is a boundary widening, a new crossing of the hardened IPC boundary, that the founder answered himself at this chunk's P4 fork (inputs#I3, live by dialog in the overseer session, 2026-10-26T14:41Z, relayed verbatim by the operator); the P2 halt's confirm (inputs#I6) is the overseer's founder-delegated word, PROVISIONAL until the founder's own word at the epoch boundary, and the ratification stays his. This entry is the Security Decisions Log record the Universal ban asks for a new channel method: `link` / `unlink` sit under the listener, peer and frame-cap controls, their names under `ViolaName::try_new` with the own-name and self-pair `-32602`; their verbs' instance check, liveness and server check are the entries below.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the collision check's code citation re-pointed
**Section:** Authentication & Authorization (authentication-authorization.md: IPC client-side server verification (anti-impersonation), the collision check)
**Change:**
- `run`'s collision check cites `src/cmd/run.rs:383` (was `src/cmd/run.rs:379`).
**Why:** the chunk's additions to `src/cmd/run.rs` (the `Methods` link set and its `link` / `unlink` routing) moved the check; the wrap's citation sweep re-pointed the one citation, digits only.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `viola link` / `viola unlink`'s instance check before any log
**Section:** Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the strict-modes entry points); Input Validation (input-validation.md: CLI arguments / stdin)
**Change:**
- The strict-modes entry-point list gains `link` / `unlink`: `viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`, so before any role file or log opens and before any snapshot value is read. A refusal exits 21 — the human form the fixed instance-state `unable:` / `hint:` pair on stderr through `human::refuse`, `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty — opens no log, writes no line and leaves the directory's mode as found; it is none of the dated interim gaps; proven by `cfg(unix)` cases only.
- CLI arguments: `<driver>` and `<driven>` through `ViolaName::try_new` (the verbs' shared name parser) and `--json`; the verbs' instance is the driven one, `instances/<driven>` joined only from that parsed name and checked by `check_instance` before any snapshot read, so neither verb is among the row's interim gaps. A `driver` equal to `driven` is not refused client-side: the wrapper refuses it (`-32602`).
**Why:** the form is the founder's own answer at this chunk's P4 fork (inputs#I3, live by dialog, relayed verbatim by the operator; the P2 halt's confirm inputs#I6 the overseer's founder-delegated word, PROVISIONAL): the instance check before any log or snapshot read, no dated gap. Exit 21, as `list` / `plugin install` and not `revive`'s exit 1, is the founder's option text (card lean 5). The self-pair refused by the wrapper only is the operator's answer of chunk 2026-10-10-session-links (`I2-relay-2`). A trap for later verbs: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home; `link_verbs_refuse_a_widened_instance_before_any_log` reads the mode after the refusal.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `viola link` / `viola unlink` take no dated server-verification gap
**Section:** Authentication & Authorization (security-plan.md: IPC client-side server verification; authentication-authorization.md: the new row IPC client-side server verification, `viola link` / `viola unlink`, and the CLI refusal row); Bootstrap phases (bootstrap-phases.md: auth-scaffolding-baseline); Security Anti-Patterns › Authentication (security-anti-patterns.md: the frame ban); Threat Model Summary (threat-model-summary.md: the Local IPC vector)
**Change:**
- The snapshot read without the strict-modes check, the dated gap owed to "Home and code-bearing file integrity", now excepts CLI `link` / `unlink`, which read it only after the strict-modes check and take no gap (the master, threat-model-summary; bootstrap-phases: owed "for every client but CLI `link` / `unlink`").
- New row: both verbs run `check_instance`, then the liveness-only pre-check (`live_endpoint`; none is exit 21 with the not-running hint), then on Linux and Windows the server check through `cmd::client::connect` before their one frame (macOS the recorded gap); their reference values come from a snapshot whose instance passed the check in the same process, so they borrow none of the eight dated gaps. Stack reference `src/cmd/link.rs` (`link_call`, shared by `src/cmd/unlink.rs`).
- The CLI frame verbs are eight, `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink` (was six), in the CLI refusal row, its stack reference `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` and threat-model-summary's frame-writing clients; `link` / `unlink`'s strict-modes refusal (exit 21) comes before their liveness and server checks.
- Frame ban: `link`/`unlink` join its frame list and meet it in full; only every client's euid check and macOS's pid and start-time check stay owed to "Unix endpoint and home hardening"; they borrow no dated exception.
**Why:** `link_call` runs `check_instance` before `client::start`, then `live_endpoint`, then one request through `cmd::client::connect` (its callers 6 → 8). The no-gap form is the founder's own answer at this chunk's P4 fork (inputs#I3, live by dialog, relayed verbatim by the operator), under the P2 halt's founder-delegated PROVISIONAL confirm (inputs#I6).
**Kept:** the eight dated gaps stand for every other frame-writing client, each still owed to "Home and code-bearing file integrity".
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the tests holding the link verbs' controls
**Section:** Authentication & Authorization (authentication-authorization.md: the new row `viola link` / `viola unlink`, the tests holding their controls)
**Change:**
- New row: `channel_link_names_are_validated_by_the_wrapper` sends raw bad-name, foreign-`driven` and self-pair `link` / `unlink` frames past every CLI check and reads `-32602` with no line written; `link_verbs_refuse_a_widened_instance_before_any_log` (Unix only) reads a group-writable driven instance refused before any log, its mode as found; the server-identity table's `link` / `unlink` rows read no frame and exit 21 in both modes against an impostor server on Linux and Windows, compiled out on macOS with the file. Stack reference `tests/channel_links.rs`, `tests/cli_links.rs`, `tests/security_negatives_server_identity.rs`.
**Why:** the report's security outcomes, each met: the wrapper's name negatives (14 passed), the impostor-server tables (25 passed, CI ubuntu and windows witnesses), the widened instance on ubuntu and macOS, its remove-the-guard control 1 of 1 red when the check is moved after `client::start`.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link lines the served methods write, and the names they keep out
**Section:** Data Protection (data-protection.md: two new Logs groups); Input Validation (input-validation.md: Own state files on read)
**Change:**
- New group, `events.ndjson`'s link lines: the served `link` appends one `link` line through `LinkSet::record`, as an implicit link does, and the served `unlink` one `unlink` line through `LinkSet::remove`, each with `data: {driver, driven}` — the two instance names only, no user content; a held pair's second `link` and an absent pair's `unlink` append nothing; the replies carry only `{since, already_linked}` (`link`) and `{}` (`unlink`).
- New group, the wrapper's process log and the verbs' output: the methods open no span and write no process-log line of their own; the `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no name; `viola link` / `viola unlink` print only their own parsed `ViolaName` arguments, never a name read back from the reply or the log.
- Own state files: a link line's driver comes from a `from` or from a served method's `params.driver`, each having passed `ViolaName::try_new` (was from a `from` only); the `LinkSet` writes at its restore, its record of an implicit link or of a served `link`, and its removal for a served `unlink` (was its restore and its record of an implicit link).
**Why:** `LinkSet::remove` has its first product caller, the `unlink` method, so a product path now appends an `unlink` event line, the first; `link_verbs_log_the_channel_pair_and_no_names` asserts no driver name in the wrapper's process log.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link methods' and verbs' error answers
**Section:** Error Handling (error-handling.md)
**Change:**
- New item: the wrapper's `link` / `unlink` answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink` a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.
**Why:** a `record` / `remove` error is `ProtocolError::Internal`; the verb reads its reply through `client::answer_of`, a wrapper error through `client::fault`.
**Kept:** the verbs' exit-21 outputs (the strict-modes refusal and the server mismatch) stay recorded with their checks in Authentication & Authorization, not repeated here.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/
