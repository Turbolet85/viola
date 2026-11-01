proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 129
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "Replace the bullet with: - **cli, macOS:** no peer pid is read, so the pid comparison reads as passed (the recorded gap, the named carve-out, owned by no route entry) and the cases of `tests/security_negatives_server_identity.rs` stay compiled out there; the server-verify refusal on macOS means an euid mismatch alone, its decision held by injected euids (`peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`) and the real read by `client_server_euid_is_this_user`, no foreign peer driven; the macOS witness landed in `tests/security_negatives_socket_dir.rs`, green on macos-latest."
    sidecar: "§8 Error recovery, cli macOS verification: the owed macOS witness landed and the owner clause leaves; server-verify-failed now reaches macOS as an euid mismatch alone; the no-peer-pid gap stays the named carve-out, owned by no route entry."
    rationale: "report.md:26 retires the gaps owed to the route entry; report.md:31 moves server-verify-failed's reach to macOS; report.md:33 keeps the no-peer-pid claim as the named carve-out; report.md:64, :67 and :113 land the macOS witness; report.md:110 keeps the server-identity negatives compiled out on macOS."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 25
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "Extend the closed list to six: `<cause>` being one of the closed words `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`."
    sidecar: "§8 Error recovery, exit 21 under --json: the closed causes five to six, the sixth socket-dir-refused."
    rationale: "report.md:28 moves exit 21's closed causes five to six and a list of the five extends; the file's list names the five."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 21
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "New record in a `values · cognitive-errors` group: - **CLI, a server of another euid:** on Linux and macOS a frame verb whose peer's euid is not this user's, or cannot be read, gets the same `server-verify-failed` refusal as a server that is not the recorded wrapper: exit 21, the hint last, or under `--json` its document, no frame sent. On Windows no euid is reported, and the DACL and the pid comparison stay the controls."
    sidecar: "§8 Error recovery: new record, a frame verb meeting a server of another euid reads server-verify-failed on Linux and macOS."
    rationale: "report.md:12 and :15 give the decision and its word on the CLI seam; report.md:93 records the founder's word on Linux and macOS (inputs#I2 Q2)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 27
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "New record in a `values · cognitive-errors` group: - **CLI, a frame verb, the socket directory:** on Unix, a recorded endpoint whose directory is loosened, a symlink or not a directory is refused by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` with exit 21 and no frame, the directory left as found: plain ASCII, no SGR and no path, pid or uid, with `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` as the last stderr line; under `--json` the `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"socket-dir-refused\"}` document with stderr empty and no hint. Off Unix the check reads nothing. The wording is a planning form, the founder's owed."
    sidecar: "§8 Error recovery: new record, a frame verb's socket-directory refusal, exit 21 socket-dir-refused, its hint a planning form."
    rationale: "report.md:11 and :15 give the check and the cause; report.md:71, :76 and :107 give the output discipline and the refusal; report.md:49 and :66 record the wording as a planning form, the founder's owed."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 51
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "New record in a `values · cognitive-errors` group: - **CLI, `viola run` / `viola revive` start, the socket directory:** on Unix, a socket directory that fails the check or cannot be created is refused with exit 1 before any snapshot, child or bind, the directory left as found: the fixed-message pair `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`, `hint:` last on stderr, naming only the parsed `ViolaName`; under `--json`, `revive` gives its `socket-dir-refused` document (above). The wording is a planning form, the founder's owed."
    sidecar: "§8 Error recovery: new record, the start's socket-directory refusal, exit 1 and the new fixed-message pair, a planning form."
    rationale: "report.md:14 gives the start's order and shared revive path; report.md:29 adds the revive --json code; report.md:76 gives the pair; report.md:95 makes a failed create and a refused check one code; report.md:49 and :66 record the wording as a planning form."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 143
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "New record in a `procedure · cognitive-errors` group: - **Verification, socket-directory refusals:** `tests/security_negatives_socket_dir.rs` (`cfg(unix)`, so on ubuntu-latest and macos-latest only) reads a directory at 0755 and a symlink to 0700: `security_negative_a_start_refuses_the_socket_directory_before_any_bind` the start's exit 1 with no bind and no child, and `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` a frame verb's exit 21 with no frame, each left as found. In `tests/cross_exit_causes.rs`, `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused` and `a_loosened_socket_directory_on_a_start_is_socket_dir_refused` read both modes, and `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) holds their documents clean."
    sidecar: "§8 Error recovery: new verification record, the socket-directory refusals' witnesses, new binary security_negatives_socket_dir on ubuntu-latest and macos-latest."
    rationale: "report.md:39 adds the binary; report.md:107 and :128 give its acceptance and OS legs; the report's New text, by line names the cases (report.md:235, :253, :255, :292, :295)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 182
    section: "## 8. Cognitive Accessibility › ### Plain language"
    change: "New bullet: - The socket-directory hint (`send_hint`'s cause `socket-dir`, `src/human/hints.rs`) is pinned by the unit `send_hint_is_the_design_string`, by the `SOCKET_DIR` literal of the exit-cause table (`tests/cross_exit_causes.rs`) and by the `HINT` literal in `tests/security_negatives_socket_dir.rs`."
    sidecar: "§8 Plain language: the socket-directory hint's literal pins named."
    rationale: "report.md:19 adds the hint cause; report.md:76 names the unit; report.md:252 and :264 list the two literals; report.md:67 expects the new literal pin named."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
note: "D-a11y-surface and D-a11y-obs-schema found no drift here: the report adds no interactive UI element and leaves the a11y violation schema alone. Outside this file, the body's .andromeda/a11y-plan.md:261 still gives macOS's lack of a peer pid as the sole reason for its OS limit, and :273 does not name the socket-directory hint among its literal pins."
