proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: "## 1. A11y Scope Summary"
    change: "In the CLI verbs entity, replace `on macOS no peer pid is read and the refusal cannot occur (the recorded gap)` with `on Linux and macOS the same refusal also reads a peer of another euid, and on macOS, where no peer pid is read (the named carve-out), an euid mismatch is all it means`."
    sidecar: "a11y-plan §1 CLI verbs entity: the frame verbs' server-verify refusal now reaches macOS as an euid mismatch alone; the no-peer-pid clause stays as the named carve-out."
    rationale: "The client euid check is retired from the dated gaps as built (report.md:26), `server-verify-failed` now reaches Linux, Windows and macOS with an euid mismatch alone on macOS (report.md:31), so `the refusal cannot occur` is false, while the statement that macOS compares no peer pid stays true (report.md:33)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 71
    section: "## 1. A11y Scope Summary"
    change: "In the plugin entity against a server that is not the recorded wrapper, replace `off macOS (no peer pid is read there and the check reads as passed: the recorded gap)` with `on Linux, Windows and macOS (on macOS no peer pid is read, the named carve-out, and the check there reads a peer of another euid alone)`."
    sidecar: "a11y-plan §1 plugin entity: the server check no longer reads as passed on macOS; there it refuses a peer of another euid."
    rationale: "`viola hook` and MCP `frame` now refuse a peer of another euid as `server-verify-failed` (report.md:16, report.md:17), so `off macOS` and `reads as passed` no longer stand (report.md:31); the no-peer-pid clause is kept as the named carve-out (report.md:33)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 8
    section: "## 1. A11y Scope Summary"
    change: "In the CLI verbs entity's `revive --json` code list, extend `... · squatted-name · internal-error` with ` · socket-dir-refused`, so `<code>` holds eleven words."
    sidecar: "a11y-plan §1 CLI verbs entity: `revive --json`'s exit-1 codes ten to eleven, adding `socket-dir-refused`."
    rationale: "`revive --json`'s exit-1 codes move ten to eleven and each list holding all ten extends (report.md:29); this list holds all ten."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 31
    section: "## 1. A11y Scope Summary"
    change: "Add a group after this one: **Entity:** `viola send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` over a socket directory that fails its check (mode open to another user, a symlink, not a directory, another owner), on Unix, in the human TTY, plain and `--json` modes: each refuses before any frame with exit 21 and the socket-directory `hint:` line last on stderr (`hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`, a planning form, the founder's wording owed at Epoch 7's boundary batch), plain ASCII, no SGR, naming no path, pid or uid, or under `--json` one document with `error` `instance-unreachable` and `detail` `socket-dir-refused`; the directory is left as found; off Unix the check reads nothing. **Source:** `src/cmd/client.rs` (`connect`), `src/human/hints.rs` (`send_hint`); `tests/security_negatives_socket_dir.rs` (`security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame`), `tests/cross_exit_causes.rs` (`a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused`), `tests/cli_error_bodies.rs` (`cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`). **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal."
    sidecar: "a11y-plan §1: new entity, the eight frame verbs over a refused socket directory, exit 21 with `socket-dir-refused`."
    rationale: "The chunk adds the check at the eight frame verbs with the sixth exit-21 cause, a11y plain ASCII with `hint:` last and no SGR (report.md:15, report.md:28, report.md:71, report.md:76, report.md:118); the check and its verdicts are Unix-only (report.md:11, report.md:128) and the wording is owed (report.md:66, report.md:94)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 39
    section: "## 1. A11y Scope Summary"
    change: "Add a group after this one: **Entity:** `viola run` and a `viola revive` start over a socket directory that fails its check, on Unix: each refuses with exit 1 before any snapshot, bind or child, a failed create reading the same, with the pair `unable: the socket directory for <name> cannot be trusted` and `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`, `hint:` last, plain ASCII, no SGR, naming only the parsed name (a planning form, the founder's wording owed at Epoch 7's boundary batch), and revive under `--json` gives its one `socket-dir-refused` document (the CLI entity above); the directory is left as found. **Source:** `src/cmd/run.rs` (`socket_dir_ready`, `refuse_socket_dir`); `tests/security_negatives_socket_dir.rs` (`security_negative_a_start_refuses_the_socket_directory_before_any_bind`), `tests/cross_exit_causes.rs` (`a_loosened_socket_directory_on_a_start_is_socket_dir_refused`), `src/human/hints.rs` (`send_hint_is_the_design_string`). **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal."
    sidecar: "a11y-plan §1: new entity, `viola run` and revive's start over a refused socket directory, exit 1 with the new pair, its wording a planning form."
    rationale: "The start refuses before any snapshot, child or bind, sharing the check with revive's start (report.md:14, report.md:70), a failed create and a refused check read one code (report.md:95), and the new pair is plain ASCII with `hint:` last, its wording owed (report.md:49, report.md:76)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 50
    section: "## 1. A11y Scope Summary"
    change: "Add a sub-bullet to the TUI passthrough entity's boundary list: `on Unix a client stream of another euid is dropped on the wrapper's accept thread before any frame: it reads end-of-file, viola writes nothing to the outer terminal and the wheel does not move.`"
    sidecar: "a11y-plan §1 TUI passthrough entity: the wrapper's drop of a refused peer holds the boundary, no terminal byte and no wheel move."
    rationale: "The wrapper's `accept_loop` drop is new on the passthrough's own process, and the a11y acceptance reads it as writing nothing to the outer terminal and moving no wheel (report.md:13, report.md:75, report.md:120); the euid is reported on Unix only (report.md:12); the plan names this edit, `the tui entity's wrapper drop` (report.md:67)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:75
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 85
    section: "## 1. A11y Scope Summary"
    change: "Add a group after this one: **Entity:** Claude Code plugin over a socket directory that fails its check, on Unix: the `viola hook` event and dialog calls and the stdio MCP server's frame tools. **Source:** `src/cmd/hook.rs` (`connect`, `NotSent`) and `crates/viola-mcp/src/tools.rs` (`frame`); `tests/security_negatives_socket_dir.rs` (`security_negative_a_hook_fails_open_on_the_socket_directory`) and `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_refuse_a_loosened_socket_directory`). **Assertability:** not-assertable. **Reason:** a hook event or dialog call sends no frame and fails open: exit 0, empty stdout and stderr, one `hook-decision` line at WARN with `detail` `socket-dir-refused`; an MCP frame tool returns `isError` with the code `instance-unreachable` and `detail` `socket-dir-refused`, sends no frame and writes no line. Neither writes a human-facing byte, so the entity stays not-assertable and no human keystroke is touched."
    sidecar: "a11y-plan §1: new not-assertable entity, the plugin over a refused socket directory: the hook fails open, MCP returns `isError`, no human-facing byte."
    rationale: "The chunk adds the check at `viola hook` and MCP `frame`, both a11y n/a with no human-facing byte (report.md:16, report.md:17, report.md:72, report.md:73, report.md:111); the sibling plugin entities in this file state the same boundary."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
note: "Outside this file, the body's §8 Error recovery › Verification (the server-verify legs, `macOS reports no peer pid`) and Plain language › Verification (the literal-pin list, the new socket-directory hint) are report-named a11y-plan edits for the §8 pass; separately, this file's CLI verbs entity line already exceeds the 2 000-char entity shape limit and the two edits there lengthen it."
