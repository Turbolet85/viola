proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: "Add a sub-bullet after this one: A socket-directory refusal carries no path, pid or uid. Its records are closed codes: `viola run` and `revive`'s start write `process-exit` `detail` `socket-dir-refused` with exit 1, the eight frame verbs the same word with exit 21 and `during` `connect` at WARN and no `channel-request`, `viola hook` one `hook-decision` `detail` `socket-dir-refused` at WARN with empty streams, and MCP's frame tools no line, `isError` the record. The stderr pair names only the parsed `ViolaName`. `tests/security_negatives_socket_dir.rs` reads them with `assert_names_nothing` and `digit_runs`, on `ubuntu-latest` and `macos-latest` (`#![cfg(unix)]`, no case on Windows); MCP's by `handlers_frame_tools_refuse_a_loosened_socket_directory`."
    sidecar: "§8 PII Scrubbing: the socket-directory refusal's no-path, no-pid, no-uid rule and its witnesses recorded (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's Coverage of new surfaces brings the check at start, the frame verbs, the hook and MCP with a PII reading on each (no path, pid or uid), and its Outcome holds it MET by `assert_names_nothing` and `digit_runs`; the file's verification list names the server-verification and strict-modes refusals and not this third one. No raw PII is logged, so D-obs-pii is not violated; this is new detail."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 39
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: "Add a sub-bullet after this one: A peer's euid is read only to be compared and is logged nowhere: `Client::server_euid` and `TokioClient::server_euid` write and log nothing, a refused euid at a client seam reads the closed word `server-verify-failed`, and the wrapper's `accept_loop` drops a refused stream before any frame, writing no line and nothing on the terminal."
    sidecar: "§8 PII Scrubbing: the peer euid recorded as compared and never logged, on the three client seams and the wrapper's accept loop (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's Symbols / APIs adds a uid-valued read on both sides of the channel, stated as writing and logging nothing, and the drop as writing no line; the file holds no rule for this value. The report gives the value no classification, so this is a verification item and not a new row of the classification table."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 39
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: "Keep the sentence and add to its end: This scan's reach stays Linux and Windows. On macOS the same `server-verify-failed` word now reads an euid mismatch alone; no scan reads that refusal there, because a foreign euid is not driven and the decision is held by injected values."
    sidecar: "§8 PII Scrubbing: the server-verification scan's Linux and Windows reach kept, with `server-verify-failed` on macOS named as an euid mismatch no scan reads (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report moves `server-verify-failed`'s reach from off macOS to Linux, Windows and macOS, an euid mismatch alone on macOS, with its pattern's rows each changing; the server-identity negatives stay compiled out on macOS and a foreign euid is not driven. The line's OS clause stays true of the scan but now reads as the refusal's whole reach."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 43
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: "Add a fourth live witness to the list: `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`), beside `cli_error_documents_hold_no_path_or_upstream_text`."
    sidecar: "§8 PII Scrubbing: the socket-directory CLI error-document witness added to the live witnesses of clean error bodies (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's New text, by line lists this new test in `tests/cli_error_bodies.rs`, calling `assert_error_body_clean` at 360-365, and its Changes section names that file."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:235
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 42
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: "«the three live witnesses» becomes «the four live witnesses»; the rest of the sentence stands."
    sidecar: "§8 PII Scrubbing: live witnesses reading the error-body check three → four (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The new case calls `assert_error_body_clean`, which this line says the three live witnesses read, so the count moves with the fourth witness."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:240
    dependent-of: D-new-detail
