# Apply — a11y-plan · part 2 of 2

## Verdicts
- fanout-a11y-plan--cascade.md 8 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:19#1
- fanout-a11y-plan--cascade.md 10 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:25#1
- fanout-a11y-plan--cascade.md 11 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:39+#1
- fanout-a11y-plan--cascade.md 12 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:51+#1
- fanout-a11y-plan--cascade.md 13 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:128#1
- fanout-a11y-plan--cascade.md 14 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:129#1
- fanout-a11y-plan--cascade.md 15 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:129+#1
- fanout-a11y-plan--cascade.md 16 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:143+#1
- fanout-a11y-plan--cascade.md 17 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:182+#1

## Edits
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  old: «Without `--json`, the exit-1 refusals of `viola revive` take the fixed-message form of design-system cli pattern 2 (one `unable: <text>` line, then one `hint:` line last; a clap usage error exits 2), pinned by the root cases in `tests/cli_revive.rs`; under `--json` every refusal of the start it shares with `run` gives»
  new: «Without `--json`, the exit-1 refusals of `viola revive`, and `viola run`'s socket-directory refusal (`refuse_socket_dir`, on the start both verbs share), take the fixed-message form of design-system cli pattern 2 (one `unable: <text>` line, then one `hint:` line last; a clap usage error exits 2), pinned by the root cases in `tests/cli_revive.rs` and, for the socket-directory refusal, by `a_loosened_socket_directory_on_a_start_is_socket_dir_refused` in `tests/cross_exit_causes.rs`; under `--json` every refusal of the start `revive` shares with `run` gives»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:25
  old: «`unwrapped-name`, `endpoint-lost` and `server-verify-failed`.»
  new: «`unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`.»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:39
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, the eight frame verbs, a refused socket directory:** `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` are refused with exit 21, `detail` `socket-dir-refused`, before any frame, the hint through `send_hint`'s cause `socket-dir` as the last stderr line and `send`'s first line in its mirror's form; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` on stdout with stderr empty and no hint.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:51
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, `viola run` / `viola revive` start, a refused socket directory:** exit 1 with the fixed `unable:` / `hint:` pair, `hint:` last on stderr, before any snapshot, child or bind, the directory left as found; under `viola revive --json` one document, `socket-dir-refused` its code, holding no path, pid or uid, with stderr empty and no hint.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:128
  old: «which runs on windows-2025 and ubuntu-latest only.»
  new: «which runs on windows-2025 and ubuntu-latest only, a limit of its peer pid witness and not of the word: `server-verify-failed` also reaches macOS (below).»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:129
  old: «the server-verify refusal cannot occur: no peer pid is read and the check reads as passed (the recorded gap), so its cases compile out there; its macOS witness is owed to "Unix endpoint and home hardening".»
  new: «no peer pid is read (the recorded gap, a named carve-out no route entry owns), so the peer pid cases compile out there; the server-verify refusal there means an euid mismatch alone, its decision and its real read held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`; the socket-directory refusal's macOS witness is `tests/security_negatives_socket_dir.rs`, on `macos-latest`.»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:129
  anchor: «  - **cli, macOS:** the server-verify refusal cannot occur: no peer pid is read and the check reads as passed (the recorded gap), so its cases compile out there; its macOS witness is owed to "Unix endpoint and home hardening".»
  add: «  - **cli, the client euid refusal:** no foreign peer is driven: its decision is tested over injected euids and its real read on Linux and macOS; each client seam's wiring is read only by forced refusals in remove-the-guard runs, which no committed test repeats.»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:143
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-errors -->»
  add: «»
  add: «- **Verification, the socket-directory refusal:** it has no Windows witness: `tests/security_negatives_socket_dir.rs` is `#![cfg(unix)]`, covering `ubuntu-latest` and `macos-latest`, and off Unix the check reads nothing and passes (`check_socket_dir_off_unix_reads_nothing`).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:182
  anchor: «  - The `unwrapped-name` and `endpoint-lost` hints (`src/human/hints.rs`) are pinned by the unit `send_hint_is_the_design_string` and by the exit-cause table (`tests/cross_exit_causes.rs`).»
  add: «  - The socket-directory hint (`send_hint`'s cause `socket-dir`, `src/human/hints.rs`) is pinned by the unit `send_hint_is_the_design_string` and by the literals `SOCKET_DIR` (`tests/cross_exit_causes.rs`) and `HINT` (`tests/security_negatives_socket_dir.rs`), with `no_hint_names_release`, `no_two_codes_share_a_hint` and `every_code_is_a_code_word` holding the catalogue.»

## Patterns
[[pattern]]
id = "a11y2-01"
fixed = '`endpoint-lost` and `server-verify-failed`'

[[pattern]]
id = "a11y2-02"
fixed = 'server-verify refusal cannot occur'

[[pattern]]
id = "a11y2-03"
fixed = 'macOS witness is owed'
