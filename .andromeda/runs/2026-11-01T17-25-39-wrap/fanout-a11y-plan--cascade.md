proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'In `revive --json`''s list of `<code>` values, add `socket-dir-refused` as the eleventh: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`.'
    sidecar: 'a11y-scope-summary: `revive --json`''s exit-1 codes ten to eleven, `socket-dir-refused` added.'
    rationale: 'report.md:29 moves the codes from ten to eleven and says each list holding all ten extends; report.md:14 has revive''s start share the refusal.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: '## 1. A11y Scope Summary'
    change: 'Replace `on macOS no peer pid is read and the refusal cannot occur (the recorded gap)` with `on macOS no peer pid is read (the recorded gap, a named carve-out no route entry owns), so there the refusal means an euid mismatch alone`.'
    sidecar: 'a11y-scope-summary: the frame verbs'' `server-verify-failed` now reaches macOS, as an euid mismatch alone; the peer pid gap stays named, with no owner.'
    rationale: 'report.md:31 moves the word''s reach to Linux, Windows and macOS, on macOS an euid mismatch alone; report.md:15 reads a refused euid as `server-verify-failed`; report.md:33 keeps the pid gap named and owned by no route entry.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 12
    section: '## 1. A11y Scope Summary'
    change: '`- **The start''s refusals under `viola revive --json`:** `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` each print one exit-1 document in place of the human pair, before the child spawns (`socket-dir-refused` before any snapshot, child or bind too); without `--json` each prints its pair.`'
    sidecar: 'a11y-scope-summary: `socket-dir-refused` joins the start''s refusals under `viola revive --json`.'
    rationale: 'report.md:14 refuses at `start` through `refused_with` before any snapshot, child or `Server::bind`, and `revive`''s start shares it.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 50
    section: '## 1. A11y Scope Summary'
    change: 'Add a bullet under the boundary clauses: `    - a peer the wrapper refuses is dropped in `accept_loop`, before its worker thread and any frame: nothing on the outer terminal and no wheel movement, witnessed by `serve_drops_a_refused_peer_before_any_frame`, with `channel_endpoint_answers_protocol_faults` as control.`'
    sidecar: 'a11y-scope-summary: the tui boundary covers the wrapper''s drop of a refused peer.'
    rationale: 'report.md:13 and report.md:75 state the drop, its witness and its control; report.md:120 records the a11y acceptance met.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:75
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 71
    section: '## 1. A11y Scope Summary'
    change: 'Replace `, off macOS (no peer pid is read there and the check reads as passed: the recorded gap)` with `, on every OS (on macOS no peer pid is read, the recorded gap and a named carve-out no route entry owns, so there such a server is one of another euid alone)`.'
    sidecar: 'a11y-scope-summary: the plugin entity against an unverified server is no longer limited to off macOS.'
    rationale: 'report.md:16 and report.md:17 put the euid refusal on the hook and MCP seams, and report.md:31 and report.md:33 give the word''s macOS reach and the ownerless pid gap; the entity stays not-assertable, neither seam writing a human-facing byte.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 83
    section: '## 1. A11y Scope Summary'
    change: 'Add an entity in the group''s shape: `- **Entity:** the `viola hook` event and dialog calls against a refused socket directory`, `**Source:** `src/cmd/hook.rs` (`connect`); `tests/security_negatives_socket_dir.rs` (`security_negative_a_hook_fails_open_on_the_socket_directory`)`, `**Assertability:** not-assertable`, `**Reason:** exit 0 with empty streams and no frame, its one record a `hook-decision` at WARN, so it adds no a11y surface and touches no human keystroke.`'
    sidecar: 'a11y-scope-summary: `viola hook` on a refused socket directory recorded as adding no a11y surface.'
    rationale: 'report.md:16 and report.md:72 state the hook''s exit, streams and one WARN record, and a11y n/a.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 83
    section: '## 1. A11y Scope Summary'
    change: 'Add an entity in the group''s shape: `- **Entity:** the stdio MCP server''s `frame` against a refused socket directory`, `**Source:** `crates/viola-mcp/src/tools.rs` (`frame`); `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_refuse_a_loosened_socket_directory`)`, `**Assertability:** not-assertable`, `**Reason:** it sends no frame and writes no line, its record the `isError` result, so it adds no a11y surface.`'
    sidecar: 'a11y-scope-summary: MCP''s `frame` on a refused socket directory recorded as adding no a11y surface.'
    rationale: 'report.md:17 and report.md:73 state no frame, no line and `isError` the record, and a11y n/a.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:73
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace `Without `--json`, the exit-1 refusals of `viola revive` take the fixed-message form` with `Without `--json`, the exit-1 refusals of `viola revive`, and `viola run`''s socket-directory refusal (`refuse_socket_dir`, on the start both verbs share), take the fixed-message form`.'
    sidecar: 'cognitive-accessibility: `viola run`''s socket-directory refusal joins the fixed-message form.'
    rationale: 'report.md:14 adds `refuse_socket_dir` on `start`, and report.md:34 moves `run`''s `human::refuse(` count from 7 to 8.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add `- **CLI, the socket-directory pair''s wording:** `unable: the socket directory for <name> cannot be trusted` and `hint: viola''s socket directory must be a directory you own, readable only by you; viola does not change its permissions` is a planning form, its wording owed to the founder at Epoch 7''s boundary batch.`'
    sidecar: 'cognitive-accessibility: the socket-directory pair recorded as a planning form, its wording owed to the founder.'
    rationale: 'report.md:49 and report.md:94 hold the wording as a planning form owed at Epoch 7''s boundary batch; report.md:76 gives the pair.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:49
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 25
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace the closing list with `the closed words `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused``.'
    sidecar: 'cognitive-accessibility: exit 21''s closed causes five to six, `socket-dir-refused` added.'
    rationale: 'report.md:28 moves exit 21''s closed causes from five to six and gives the list.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 37
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add `- **CLI, the eight frame verbs, a refused socket directory:** `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` exit 21 with `socket-dir-refused` and no frame, reaching the hint through `send_hint` cause `socket-dir`, `hint:` last and `send`''s first line printing its mirror''s form; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` on stdout with stderr empty and no hint.`'
    sidecar: 'cognitive-accessibility: the eight frame verbs'' refusal of a refused socket directory recorded.'
    rationale: 'report.md:15 lists the eight callers and the `socket-dir` hint cause; report.md:71 gives the exit, detail and no frame; report.md:98 the mirror form; report.md:28 the closed word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 49
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add `- **CLI, `viola run` / `viola revive` start, a refused socket directory:** exit 1 with the fixed `unable:` / `hint:` pair, `hint:` last on stderr, before any snapshot, child or bind, the directory left as found; under `--json` one document, `socket-dir-refused` its code, holding no path, pid or uid, with stderr empty and no hint.`'
    sidecar: 'cognitive-accessibility: the start''s refusal of a refused socket directory recorded.'
    rationale: 'report.md:14 places the refusal before any snapshot, child or bind; report.md:11 has the check never chmod; report.md:70 and report.md:118 give the exit and the clean document.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 128
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace the closing clause with `These run on all three OS legs, except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only: that limit is the peer pid witness''s, not the word''s, since `server-verify-failed` also reaches macOS, as an euid mismatch alone.`'
    sidecar: 'cognitive-accessibility: the two-leg limit narrowed to the peer pid witness, the word reaching macOS.'
    rationale: 'report.md:110 keeps the server-identity negatives compiled out on macOS, and report.md:31 widens the word''s reach, so the leg limit no longer covers the word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:110
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 129
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: '`  - **cli, macOS:** no peer pid is read (the recorded gap, a named carve-out no route entry owns), so the peer pid cases compile out there; the server-verify refusal means an euid mismatch alone, its decision and its real read held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`; the case owed to "Unix endpoint and home hardening" landed in `tests/security_negatives_socket_dir.rs`, on `macos-latest`.`'
    sidecar: 'cognitive-accessibility: the macOS clause owed to "Unix endpoint and home hardening" discharged; the peer pid gap stays named, with no owner.'
    rationale: 'report.md:26 retires the owed clauses; report.md:33 keeps the pid gap named and ownerless; report.md:64 and report.md:113 land the macOS case; report.md:109 and report.md:110 give the decision, the read and the compiled-out negatives.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 129
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add `  - **cli, the client euid refusal:** no foreign peer is driven: its decision is tested over injected euids, the real read on Linux and macOS, and each seam''s wiring was read only by the implementing run''s forced refusals, which no committed test repeats.`'
    sidecar: 'cognitive-accessibility: the client euid refusal''s limit of proof recorded.'
    rationale: 'report.md:54, report.md:74 and report.md:130 state the injected decision, the real read, the forced refusals and that no foreign euid is driven.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 141
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Add `- **Verification, the socket-directory refusal:** it has no Windows witness: `tests/security_negatives_socket_dir.rs` is `#![cfg(unix)]`, covering `ubuntu-latest` and `macos-latest`, and the check off Unix reads nothing and passes.`'
    sidecar: 'cognitive-accessibility: the socket-directory refusal''s two-leg witness recorded.'
    rationale: 'report.md:128 reads no case of that binary on Windows, `#![cfg(unix)]`; report.md:107 names the two legs; report.md:11 has the check off Unix return `Ok(())`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:128
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 182
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Add `  - The socket-directory hint (`send_hint`''s cause `socket-dir`, `src/human/hints.rs`) is pinned by literal through `send_hint_is_the_design_string`, with `no_hint_names_release`, `no_two_codes_share_a_hint` and `every_code_is_a_code_word` over the catalogue and the hint constants in `tests/cross_exit_causes.rs` and `tests/security_negatives_socket_dir.rs`.`'
    sidecar: 'cognitive-accessibility: the socket-directory hint''s literal pin named.'
    rationale: 'report.md:76 names the four cases over the new pair; report.md:19 the hint cause; the New text section lists the `SOCKET_DIR` and `HINT` constants in the two test files.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 257
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace `(`viola revive`''s exit-1 refusals and `viola verify`''s interrupted pair: one `unable: <text>` line)` with `(`viola revive`''s exit-1 refusals, `viola run`''s socket-directory refusal and `viola verify`''s interrupted pair: one `unable: <text>` line)`.'
    sidecar: 'a11y-plan §8: `viola run`''s socket-directory refusal joins the broad line''s one-line form.'
    rationale: 'report.md:14 adds `refuse_socket_dir` on `run`''s start; this is the broad line cognitive-accessibility#2.1 details.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 261
    section: '## 8. Cognitive Accessibility › ### Error recovery'
    change: 'Replace the closing parenthesis with `(the frame verbs'' server-verify peer pid witness on windows-2025 and ubuntu-latest only: macOS reports no peer pid, the word there an euid mismatch alone; the socket-directory refusal on ubuntu-latest and macos-latest)`.'
    sidecar: 'a11y-plan §8: the verification''s leg limits restated, the peer pid witness off macOS and the socket-directory refusal on two legs.'
    rationale: 'report.md:31 and report.md:110 split the word''s reach from the compiled-out witness; report.md:107 and report.md:128 give the socket-directory legs.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 273
    section: '## 8. Cognitive Accessibility › ### Plain language'
    change: 'Replace `the server-verify, `unwrapped-name` and `endpoint-lost` hints` with `the server-verify, socket-directory, `unwrapped-name` and `endpoint-lost` hints`.'
    sidecar: 'a11y-plan §8: the socket-directory hint joins the broad line''s strings pinned by literal.'
    rationale: 'report.md:76 pins the new hint through `send_hint_is_the_design_string`; report.md:67 carries the new literal pin to a11y-plan §8.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
note: 'D-a11y-surface: the report adds no interactive element, only a non-interactive refusal pair. D-a11y-obs-schema: the report moves two `detail` enums in `schemas/diag-line.v1.json`, and a11y-plan states neither those enums nor a violation schema of its own, so nothing diverges. The detail''s server-verify rows read as the word''s, and obs-plan''s enum counts belong to obs-plan.'
