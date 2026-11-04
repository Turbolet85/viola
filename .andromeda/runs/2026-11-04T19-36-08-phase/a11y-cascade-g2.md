@carried n=1 label=a11y-plan/cognitive-accessibility fact=fct-6edxk4iz
why: Every time bound the Unix openpty wrapper holds must fall on the driver, the user's command or viola's own child and never block, refuse or delay a human key, read on ubuntu-latest and macos-latest as on windows-2025.
quote: the hold bounds the driver, never the human: any human key still hands the held dialog back at once, whatever the hold. A `viola send` on a verified CLI waits in the readiness gate for the input box for at most `GATE_MAX_WAIT` (8.5 s, architecture [Screen Model]) and is then refused `not-delivered` / `input-not-ready` with nothing typed. That bound falls on the driver only as well: a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is then refused `human-typing`.
@carried n=1 run=2 label=a11y-plan fact=fct-mkekg7f5
quote: every time bound viola holds — the `viola wait` timeout the caller passes, the MCP `wait` cap, the dialog hold, the `viola send` readiness gate, the confirmation windows, the status-line deadline, the `claude agents --json` read, the budget pause — falls on the driver, the user's own command or viola's own child, never on the human: no key is blocked, refused or delayed by it.
@carried n=1 run=3 label=a11y-plan fact=fct-ickgtugn
quote: NEVER set harsh timeouts without warning (no warn-extend pattern per SC 2.2.1). v1 has none.
@carried n=2 label=a11y-plan/cognitive-accessibility fact=fct-n7d5inyr
why: The refusal layout, typed exit codes, hint-free --json and the Unix-only socket-directory and revive --json refusals must read the same on all three OS legs.
quote: **CLI:** `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code. The one refusal with no hint is `unknown` (exit 14): its detail is a code word or nothing, opaque to a reader, so design prints no hint rather than a guess. Faults (`error: wrapper fault`, `error: internal error`) are not refusals and carry no hint. `--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (below). Without `--json`, the exit-1 refusals of `viola revive`, and `viola run`'s socket-directory refusal (`refuse_socket_dir`, on the start both verbs share), take the fixed-message form of design-system cli pattern 2 (one `unable: <text>` line, then one `hint:` line last; a clap usage error exits 2), pinned by the root cases in `tests/cli_revive.rs` and, for the socket-directory refusal, by `a_loosened_socket_directory_on_a_start_is_socket_dir_refused` in `tests/cross_exit_causes.rs`; under `--json` every refusal of the start `revive` shares with `run` gives one `{"v":1,"error":"<code>","detail":null}` document with no hint and empty stderr (`revive_json_*` in `tests/cli_revive.rs`, the start rows in `tests/cross_exit_causes.rs`; chunk 2026-10-17-cli-machine-contract).
@carried n=2 run=2 label=a11y-plan fact=fct-2jdilj7v
quote: trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on all three OS legs (the frame verbs' server-verify peer pid witness on windows-2025 and ubuntu-latest only: macOS reports no peer pid, the word there an euid mismatch alone; the socket-directory refusal on ubuntu-latest and macos-latest).
@carried n=2 run=3 label=a11y-plan fact=fct-c6pzu5nt
quote: NEVER show error without recovery suggestion (per SC 3.3.3). Every human-mode CLI refusal except `unknown` (exit 14, its detail a code word or nothing, design cli) ends with a `hint:` line.
@carried n=2 run=4 label=a11y-plan fact=fct-kgelfpj7
quote: Every kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
@carried n=3 label=a11y-plan/cognitive-accessibility fact=fct-bhnorrcm
why: The exit-21 --json document and its closed cause words must be the same on every OS, a cause one OS cannot reach never replaced by an OS-only word.
quote: **CLI, exit 21 under `--json`:** one document per cause, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` on stdout with stderr empty, no hint and no ESC, `<cause>` being one of the closed words `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`.
@carried n=4 label=a11y-plan/cognitive-accessibility fact=fct-zlmstg36
why: The link/unlink strict-modes refusal must be read on ubuntu-latest and macos-latest with the same pair, exit code and --json document.
quote: **CLI, `viola link` / `viola unlink`, strict-modes:** a driven instance that fails the strict-modes check is refused with exit 21 before any log opens, no log line written: the fixed instance-state `unable` / `hint:` pair on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout with stderr empty and no hint.
@carried n=5 label=a11y-plan/cognitive-accessibility fact=fct-mxmekxqi
why: The frame verbs' strict-modes refusal must be read on both Unix legs with the same pair, exit 21 and --json document.
quote: **CLI, `send` / `wait` / `last` / `answer` / `pause` / `release`, strict-modes:** an instance that fails the strict-modes check is refused with exit 21 before any frame or log (`send` after its text read), no log line written: the same fixed instance-state `unable` / `hint:` pair as `viola link` / `viola unlink` on stderr, `hint:` last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout with stderr empty and no hint.
@carried n=6 label=a11y-plan/cognitive-accessibility fact=fct-gh3uczd3
why: The frame verbs' socket-directory refusal must run on ubuntu-latest and macos-latest with its hint last and its exact --json document.
quote: **CLI, the eight frame verbs, a refused socket directory:** `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` are refused with exit 21, `detail` `socket-dir-refused`, before any frame, the hint through `send_hint`'s cause `socket-dir` as the last stderr line and `send`'s first line in its mirror's form; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` on stdout with stderr empty and no hint.
@carried n=7 label=a11y-plan/cognitive-accessibility fact=fct-tssjslea
why: A Unix wrapper killed with a call open must read endpoint-lost and a stale one instance-dead, as on Windows.
quote: **CLI, a lost or stale wrapper:** a wrapper lost with the call open is `endpoint-lost`; a frame verb against a stale instance reads `instance-dead`, stale heartbeat being exit 1's cause alone.
@carried n=8 label=a11y-plan/cognitive-accessibility fact=fct-zwbgfalb
why: The openpty start on Linux and macOS must refuse a home tree that fails strict-modes before any log, line or child, with the fixed pair and the revive --json document.
quote: **CLI, `viola run` / `viola revive` start, strict-modes:** a home tree that fails the strict-modes check (the home, the instance's files, `ledger/`, `bin/`, `plugin/`, the pinned copy and its plugin folders) is refused with exit 1 before any log opens, no line written and no child started: the fixed home-state `unable` / `hint:` pair for a home-level refusal, the instance-state pair for an instance-level one, `hint:` last on stderr, stdout empty; under `--json`, `revive` gives its `strict-modes-failed` document (above).
@carried n=9 label=a11y-plan/cognitive-accessibility fact=fct-cgrssu3w
why: The Unix start's socket-directory refusal must read exit 1 with the fixed pair, and under revive --json one socket-dir-refused document, on ubuntu-latest and macos-latest.
quote: **CLI, `viola run` / `viola revive` start, a refused socket directory:** exit 1 with the fixed `unable:` / `hint:` pair, `hint:` last on stderr, before any snapshot, child or bind, the directory left as found; under `viola revive --json` one document, `socket-dir-refused` its code, holding no path, pid or uid, with stderr empty and no hint.
@carried n=10 label=a11y-plan/cognitive-accessibility fact=fct-45izkccw
why: A start against a stale or killed Unix wrapper must tell stale-heartbeat from already-live, as the Windows wrapper's does.
quote: **CLI, a start against a holder:** exit 1 tells a stale holder from a live one: `stale-heartbeat` for a `stale` holder, `already-live` for a live holder and a fresh replayed beat.
@carried n=11 label=a11y-plan/cognitive-accessibility fact=fct-bjgpzcmx
why: verify's strict-modes refusal must be read on both Unix legs with no step line and the fixed home-state pair.
quote: **CLI, `viola verify`, strict-modes:** a home whose tree (the home, `ledger/`, `bin/`, `plugin/`) fails the strict-modes check is refused with exit 1 before its log opens, with no step line, no summary and no line written: the fixed home-state `unable` / `hint:` pair on stderr, `hint:` last; a `ledger/` that `update_stamps`'s own check refuses gives the same pair.
@carried n=12 label=a11y-plan/cognitive-accessibility fact=fct-ms6n7636
why: This per-OS split must hold as stated: the Unix strict-modes refusal with its hint read on both Unix legs, the Windows fault line left unmoved.
quote: **CLI, an unstattable home:** a home path under a regular file is a strict-modes refusal with its hint on Unix (`verify`: the home-state pair, exit 1; the six frame verbs: the instance-state pair, exit 21); on Windows such a path stats `NotFound`, the check skips it as absent, and the `cli` catch site's fault line stands, with no hint; under `--json` a frame verb there prints the `internal-error` document on stdout in that line's place, stderr empty (`verify` takes no `--json`).
@carried n=13 label=a11y-plan/cognitive-accessibility fact=fct-wqxezfp7
why: The stopped-instance refusal and the unwrapped-name refusal of link/unlink must end with their own hint on every OS leg.
quote: **CLI, `viola link` / `viola unlink`, stopped instance:** a driven instance with no live endpoint whose instance directory stands is refused with exit 21, the not-running `hint:` as the last stderr line; a driven name with no live endpoint and no such directory reads `unwrapped-name`, its own `hint:` last.
@carried n=14 label=a11y-plan/cognitive-accessibility fact=fct-xzkz7422
why: A name with no endpoint must read instance-dead or unwrapped-name by the same directory rule on every OS, strict-modes first.
quote: **CLI, a name with no endpoint:** the refusal reads the instance directory: a directory is `instance-dead`, anything else (absent, a regular file, a symlink) is `unwrapped-name`, and the strict-modes check stays first.
@carried n=15 label=a11y-plan/cognitive-accessibility fact=fct-5kq5fzne
why: The unwrapped-name and endpoint-lost human forms must end with their own plain-ASCII hint, identical on every OS and pinned by literal.
quote: **CLI, the `unwrapped-name` and `endpoint-lost` human forms:** each ends with its own `hint:` as the last stderr line, in plain ASCII, the name in a hint being the parsed `ViolaName`.
@carried n=15 run=2 label=a11y-plan fact=fct-fhiyvzbh
quote: every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install`, the server-verify, socket-directory, `unwrapped-name` and `endpoint-lost` hints and the `viola link` / `viola unlink` result lines, by literal, so any copy change fails CI until the pinned string is updated on purpose.
@carried n=16 label=a11y-plan/cognitive-accessibility fact=fct-uyauvo6g
why: A wrapper's invalid-params refusal must reach link/unlink as a fault, exit 20 with no hint, from the Unix wrapper as from the Windows one.
quote: **CLI, `viola link` / `viola unlink`, wrapper refusal:** a pair the wrapper refuses as invalid params (`-32602`, e.g. `driver` equal to `driven`) reaches the verb as a fault, exit 20, no hint; the verbs never refuse it client-side.
@carried n=17 label=a11y-plan/cognitive-accessibility fact=fct-yq4bu6pq
why: A peer's refusal word that is no code word must read unknown, and a non-code-word detail nothing, so every --json document keeps closed words on every OS.
quote: **CLI, a peer's refusal:** `reply_of` and `of_reply` screen a peer's reply with `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a `refusal` that is no code word reads `unknown`, and a string `detail` that is no code word reads nothing (null under `--json` and on MCP).
@binding n=1
**cognitive-timeouts ↔ path-p4-wheel (keyboard)**: every bound the Unix wrapper holds falls on the driver, and a human key pressed under any of them reaches the child and takes the wheel at once; the openpty wrapper must keep both on Linux and macOS as ConPTY does on Windows (a11y-plan/cognitive-accessibility fct-6edxk4iz) (a11y-plan fct-csjr2vg7).
@binding n=2
**cognitive-errors ↔ the one diagnostics schema**: the closed refusal and cause words (exit-21 causes, the code-word screen, instance-dead versus unwrapped-name) must be the same on every OS; a cause one OS cannot reach stays out of its tests and never becomes an OS-only word (a11y-plan/cognitive-accessibility fct-bhnorrcm) (a11y-plan/cognitive-accessibility fct-yq4bu6pq).
@binding n=3
**cognitive-errors ↔ per-OS split**: the error-recovery verification runs on all three OS legs except the named splits (server-verify peer pid on windows-2025 and ubuntu-latest only, macOS an euid mismatch alone; socket-directory refusal on ubuntu-latest and macos-latest; an unstattable home a strict-modes refusal on Unix and the fault line on Windows), and each split carries its reason where it stands (a11y-plan fct-2jdilj7v) (a11y-plan/cognitive-accessibility fct-ms6n7636).
@binding n=4
**cognitive-errors ↔ cognitive-plain-language**: the hints and fixed strings the Unix legs print must match the trycmd and literal pins byte for byte, so any OS-only wording fails CI (a11y-plan/cognitive-accessibility fct-5kq5fzne) (a11y-plan fct-fhiyvzbh).
@binding n=5
**cognitive-errors ↔ visual-sgr (dim only on stale rows)**: the stale wrapper the outer-PTY witness of v1-40 boots must read stale-heartbeat on a start and instance-dead on a frame verb, the same stale state the board paints dim (a11y-plan/cognitive-accessibility fct-45izkccw) (a11y-plan fct-lchghj7u).
