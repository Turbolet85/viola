proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 55
    section: "## Threat Model Summary"
    change: "As built, on Linux and Windows every frame-writing client (CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`; `viola hook`'s event and dialog paths; `viola mcp`'s `send` / `wait` / `last` / `answer` tools) compares the connected peer's pid and that process's start time with the snapshot's `pid` and `started_at` before its first frame, and sends no frame on a mismatch or a check it cannot make. The snapshot is read without the strict-modes check (owed to \"Home and code-bearing file integrity\"), except by CLI `link` / `unlink`, which run `check_instance` on the driven instance before any role file, log or snapshot read (the founder's own word at P4, `inputs#I3`, 2026-10-26: no dated gap); macOS, where the peer reports no pid, stays a recorded gap (owed to \"Unix endpoint and home hardening\")."
    sidecar: "threat-model-summary IPC vector: the as-built server check names eight CLI frame verbs (+ `link`, `unlink`), the two reading the snapshot after `check_instance` (chunk 2026-10-26-explicit-session-links)"
    rationale: "Report Counts: pattern `frameverbs` stale `security-plan-detail/threat-model-summary.md:55` (six → eight); Symbols / APIs: `link_call` step 1 (`check_instance` before `client::start`) and step 5 (the server check through `cmd::client::connect`); Decisions: the founder's word `inputs#I3`, no dated gap."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:55
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 45
    section: "## Threat Model Summary"
    change: "Insert after this line, at its indent: `- They can also create or remove a driver link through the served `link` / `unlink` methods (chunk 2026-10-26-explicit-session-links; before, both answered `-32601`): any same-OS-user process that passes the endpoint controls can, the effect being a `link` / `unlink` line in `events.ndjson` and the snapshot's links only; the self-reported `from` plays no part, neither method reading it.`"
    sidecar: "threat-model-summary IPC vector: the served `link` / `unlink` let a connecting same-user process add or remove a driver link, a record only (F10)"
    rationale: "Report Symbols / APIs: channel methods `link` and `unlink` served (before `-32601`), `link` calls `record(driver)`, `unlink` calls `remove(driver)`, neither reads `from`; the effect a line and the snapshot write."
    basis: src/run/links.rs:70-76
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 12
    section: "## Authentication & Authorization"
    change: "**Implementation:** Capability-based, all or nothing. Any caller that passes the OS check (IPC) gets the full surface: the nine served methods (`send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`) and the `hook.event` notification. No roles, because the only principal is the local OS user (Threat Model: Auth model). `link` and `unlink` are served since chunk 2026-10-26-explicit-session-links (before, `-32601`): a boundary widening the founder's own word at P4 (`inputs#I3`, 2026-10-26) ratifies as mapped to the IPC controls, the listener and peer checks unchanged and the CLI verbs going through the instance check, the liveness pre-check and the server check."
    sidecar: "RBAC row: the full surface named as nine served methods plus `hook.event`, `link` / `unlink` served under the founder's word `inputs#I3` (F2)"
    rationale: "Report Symbols / APIs: the `Methods` doc names the served set, nine methods and the `hook.event` notification (`src/cmd/run.rs:68-70`), `dispatch_call` routes `link` / `unlink` (`src/cmd/run.rs:144-145`); Decisions: `inputs#I3` ratifies the widening, no dated gap."
    basis: src/cmd/run.rs:68-70
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: "Insert before **On mismatch**: **No gap (the founder's own word at P4, `inputs#I3`, 2026-10-26, chunk 2026-10-26-explicit-session-links):** CLI `viola link` / `viola unlink` run `viola_state::strict::check_instance(<absolute home>, <home>/instances/<driven>)` before `client::start`, so before any role file or log opens and before any snapshot value is read; then the liveness-only pre-check (`live_endpoint`; none is exit 21 through `client::unreachable`); then, on Linux and Windows, the server identity check through `cmd::client::connect` (the peer pid and its start time against the snapshot's `pid` / `started_at`; macOS the recorded gap) before their one `link` / `unlink` frame. They take their reference values from a snapshot whose instance passed the strict-modes check in the same process, and borrow none of the eight dated gaps."
    sidecar: "server verification row: `viola link` / `viola unlink` recorded as the frame verbs with no dated gap — `check_instance`, liveness, then the server check (F1)"
    rationale: "Report Symbols / APIs: `link_call` (`src/cmd/link.rs:85-136`) steps 1 to 5 in order, `check_instance` at `src/cmd/link.rs:98-106`, `live_endpoint` at `src/cmd/link.rs:113-115`; Decisions: `inputs#I3`, the boundary widening ratified with no dated gap."
    basis: src/cmd/link.rs:98-106
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 34
    section: "## Authentication & Authorization"
    change: "In the strict-modes entry-point list, insert after the `list` (…) item and before `mcp`: `link` / `unlink` (chunk 2026-10-26-explicit-session-links: `viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`, so before any role file or log opens and before any snapshot value is read; a refusal is exit 21, the fixed instance-state `unable:` / `hint:` pair through `human::refuse` in human mode, under `--json` exactly `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}` on stdout with stderr empty, no line in any log and the directory's mode left as found; the founder's own word at P4, `inputs#I3`, 2026-10-26; it is none of the dated interim gaps; proven by `link_verbs_refuse_a_widened_instance_before_any_log`, Unix only)"
    sidecar: "`~/.viola/` access control: the strict-modes entry-point list gains `link` / `unlink` (`check_instance` before any log, exit 21, no dated gap)"
    rationale: "Report Counts: pattern `checkinst` stale `security-plan-detail/authentication-authorization.md:34` (the list names `run`, `revive`, `list`, `hook statusline`, not the two verbs); Symbols / APIs: `link_call` step 1; Decisions: exit 21 as `list` / `plugin install`, not `revive`'s exit 1 (card lean 5)."
    basis: .andromeda/security-plan-detail/authentication-authorization.md:34
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 66
    section: "## Authentication & Authorization"
    change: "**Implementation:** A CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting a mismatch, or a check that cannot be made, writes no frame and exits 21: under `--json` the document `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}` on stdout with stderr empty; in human mode the `unable  <name>  instance-unreachable` line with the hint `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` last on stderr. The client's own exit-13 refusals come first, and `link` / `unlink`'s strict-modes refusal (exit 21) comes before their liveness and server checks."
    sidecar: "CLI refusal row: the frame verbs are eight (+ `link`, `unlink`), the two verbs' strict-modes refusal ordered first"
    rationale: "Report Counts: CLI frame verbs six → eight (pattern `frameverbs`); Symbols / APIs: `link_call` step 5 (exit 21 `instance-unreachable` with the server-verify hint) after step 1's `check_instance` refusal (exit 21)."
    basis: src/cmd/link.rs:85-136
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 67
    section: "## Authentication & Authorization"
    change: "**Stack reference:** root bin `src/cmd/client.rs` (`connect`, `Unreachable::ServerUnverified`); `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs`; `src/human/hints.rs` (the `server-verify` cause)"
    sidecar: "CLI refusal row stack reference: `src/cmd/link.rs`, `src/cmd/unlink.rs` added"
    rationale: "Report Counts: `cmd::client::connect` callers 6 → 8 (+ `src/cmd/link.rs:140`, `src/cmd/unlink.rs:21`)."
    basis: src/cmd/unlink.rs:21
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 73
    section: "## Authentication & Authorization"
    change: "Add after this record a record of the same shape — Requirement: `viola link` / `viola unlink`, the tests holding their controls; Implementation: `channel_link_names_are_validated_by_the_wrapper` sends raw bad-name, foreign-`driven` and self-pair `link` / `unlink` frames and reads `-32602` with no line written; `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) reads a group-writable driven instance refused before any log with its mode as found, its remove-the-guard control red when the check is moved after `client::start`; the identity table's `link` / `unlink` rows read no frame and exit 21 in both modes against an impostor server on Linux and Windows (compiled out on macOS with the file); Stack reference: `tests/channel_links.rs`, `tests/cli_links.rs`, `tests/security_negatives_server_identity.rs`."
    sidecar: "new record: the tests holding the `link` / `unlink` controls (wrapper param negatives, the widened-instance refusal, the identity rows) (F11)"
    rationale: "Report Outcome (security): raw bad-name / foreign-driven / self-pair frames met by `channel_link_names_are_validated_by_the_wrapper`; group-writable instance met by `link_verbs_refuse_a_widened_instance_before_any_log` (remove-the-guard control A 1 of 1 red); impostor server met by the two tables (25 passed); macOS compiles the identity rows out."
    basis: tests/channel_links.rs:39-88
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 17
    section: "## Input Validation"
    change: "Insert after the **`pause` / `release` params (wrapper):** clause: **`link` / `unlink` params (wrapper, chunk 2026-10-26-explicit-session-links):** `driver` and `driven` are read as strings through `ViolaName::try_new` (`LinkSet`'s shared `driver_of`); a missing, non-string or invalid name, a `driven` other than the serving wrapper's own name, and a `driver` equal to `driven` are each `-32602` `\"invalid params\"` with `data: null`, no line written; neither method reads `from` (a `from` of any shape changes nothing)."
    sidecar: "channel-frames row: the wrapper's `link` / `unlink` params rule and its three `-32602` causes (F3)"
    rationale: "Report Symbols / APIs: `driver_of` (`src/run/links.rs:85-99`) reads both names through `ViolaName::try_new`; the bad name, foreign `driven` and self-pair are `ProtocolError::InvalidParams` (`-32602`, `data: null`), no line; neither method reads `from`."
    basis: src/run/links.rs:85-99
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 31
    section: "## Input Validation"
    change: "Insert after **`list` arguments:** `[--json]` only, no name.: **`link` / `unlink` arguments (chunk 2026-10-26-explicit-session-links):** `<driver>` and `<driven>` through `ViolaName::try_new` (the verbs' shared name parser), `--json` besides; the verbs' instance is the driven one, its `instances/<driven>` joined only from that parsed name and checked by `check_instance` before any snapshot read, so neither verb is among this row's interim gaps; a `driver` equal to `driven` is not refused client-side: the wrapper refuses it (`-32602`), which the CLI reports as a wrapper fault, exit 20."
    sidecar: "CLI-arguments row: the `link` / `unlink` arguments, their check taking no interim gap, the self-pair refused by the wrapper only (F4)"
    rationale: "Report Symbols / APIs: `LinkArgs` (`src/cmd/link.rs:21-32`) shared by both verbs, `link_call` step 1 and step 6 (a wrapper error, `-32602` included, is exit 20); Decisions: the self-pair refused by the wrapper only, never client-side (card lean 2)."
    basis: src/cmd/link.rs:21-32
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 50
    section: "## Input Validation"
    change: "In the link-rule sentence, the parenthesis `the wrapper wrote them: …` now reads: the wrapper wrote them, the driver from a `from` or, since chunk 2026-10-26-explicit-session-links, from a served `link` / `unlink` method's `params.driver`, each having passed `ViolaName::try_new`, and the driven its own name."
    sidecar: "own-state-files row: a link / unlink line's driver may now come from the served methods' `params.driver`, validated the same way"
    rationale: "Report Symbols / APIs: `link` calls `record(driver)` and `unlink` calls `remove(driver)` with the driver from `driver_of`, which reads `params.driver` through `ViolaName::try_new`; a product path now appends an `unlink` line."
    basis: src/run/links.rs:70-76
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 51
    section: "## Input Validation"
    change: "In the sentence on the link rule's second product caller, the parenthesis now reads: the wrapper's `LinkSet` (its restore at every start, and its `record` of an implicit link or, since chunk 2026-10-26-explicit-session-links, of a served `link` method; the served `unlink` method is `LinkSet::remove`'s first product caller)."
    sidecar: "own-state-files row: `LinkSet`'s record takes the served `link` method too, and `remove` has its first product caller"
    rationale: "Report Symbols / APIs: `link` calls `record(driver)`; `LinkSet::remove` has its first product caller, the `unlink` method; Retired: `LinkSet::remove`'s `dead_code` allow."
    basis: src/run/links.rs:78-83
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 8
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "Add `link`/`unlink` to the ban's frame list (`send`, `answer`, `hook.dialog`, `wait`/`last`, `pause`/`release`, `link`/`unlink`, and `hook.event` notifications …), and append after the dated exceptions: CLI `viola link` / `viola unlink`'s frames meet the ban in full, taking `endpoint`, `pid` and `started_at` from a `snapshot.json` read after `check_instance` passed in the same process and verifying the server through `cmd::client::connect` (the founder's own word at P4, `inputs#I3`, 2026-10-26, chunk 2026-10-26-explicit-session-links); only every client's euid check and macOS's pid and start-time check stay owed to \"Unix endpoint and home hardening\", and they borrow no dated exception."
    sidecar: "frame ban: `link` / `unlink` join the frame list as frames meeting the ban in full, no dated exception"
    rationale: "Report Symbols / APIs: `link_call` steps 1 (`check_instance` before `client::start`) and 5 (the server check, Linux and Windows; macOS the recorded gap); Decisions: `inputs#I3`, no dated gap taken."
    basis: src/cmd/link.rs:98-106
note: "F7 and F8 (data-protection: the product `unlink` event line's names-only content; the verbs printing only their own parsed names, no driver name in the process log) fit no picked entity — data-protection#1.1 holds the file protections, not log content — so they are the new-detail pass's; the Universal ban's Security Decisions Log entry for the two new channel methods (the founder's word `inputs#I3`) is owed in the amendments log, not a detail file; F5, F6 and F9 belong to error-handling and bootstrap-phases, which no picked entity holds."
