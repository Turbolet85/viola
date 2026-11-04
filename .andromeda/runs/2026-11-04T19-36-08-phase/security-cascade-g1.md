@carried n=1 label=security-plan/threat-model-summary fact=fct-qcqqc5fi
why: The chunk's one diagnostics schema must give these content-bearing events.ndjson fields and their wait/last documents the same keys on Linux and macOS as on Windows, and they must stay confined to those contract payloads.
quote: `prompt-submitted.data.text` holds every submitted prompt (driver, human and harness), and `turn-ended.data.last_assistant_message` holds each final assistant reply. The wrapper keeps the latest reply in memory for `last`. The same lines go out verbatim over MCP `wait`/`last` and over CLI `wait`/`last`
@carried n=1 run=2 label=security-plan/threat-model-summary fact=fct-zfobg22w
quote: The `permission` event carries `{tool, input}`, where `input` is the tool call's full arguments as JSON. For Bash and Write-type tools that means shell commands and file bodies. `plan` carries the plan text and `question` carries questions, options and each option's description and preview text (`option_details`), which also go out over CLI `wait --json` and MCP `wait`.
@carried n=2 label=security-plan/threat-model-summary fact=fct-33o4e7zi
why: The snapshot fields that decide what code runs and where clients connect must carry the same keys on every OS under the one schema, and the Unix suite must exercise the Unix-only settings override with the pinned path only.
quote: `instances/<ViolaName>/settings.json` is the per-session statusline override. It holds `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`, never the user's command.
  - `snapshot.json` stores `statusline_command`, the user-written shell string that `hook statusline` reads through `VIOLA_DIR` and runs through a shell. This is the only shell-out in viola.
@carried n=3 label=security-plan/threat-model-summary fact=fct-jy2pv6x7
why: The openpty spawn on Linux and macOS must strip the same CLAUDE* names from the child's environment as the Windows spawn, and no OS's output may carry their values.
quote: The `viola run` wrapper process's environment holds `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and other `CLAUDE*` parent-identity variables, which are stripped from the child's environment (R8 strip list, Occupied Resources: Environment variables).
@carried n=4 label=security-plan/threat-model-summary fact=fct-qgluq6ji
why: The snapshot schema the chunk aligns across OSes must keep `cwd` optional and confined to the 0600 snapshot on Linux and macOS as on Windows.
quote: the optional `cwd` (the start's spawn directory: an absolute host path that can carry the OS username; it lives in the 0600 snapshot only, on no log line, error body or stdout, and is omitted when absent).
@carried n=4 run=2 label=security-plan fact=fct-l2iemwjy
quote: The snapshot's recorded `cwd`, an absolute host path that can carry the OS username, stands in the 0600 snapshot only, on no log line, error body or stdout.
@carried n=5 label=security-plan/threat-model-summary fact=fct-t7qeqkiu
why: The Unix wrapper's endpoint, now exercised by the suite on both Unix OSes, must stand in the verified per-user 0700 directory, a refused directory giving no bind and no frame.
quote: On Unix the socket sits in the per-user 0700 directory, not in world-writable `/tmp`; a directory another user owns, a loosened one or a symlink is refused by the socket-directory check, with no bind on `run` and no frame on a client.
@carried n=5 run=2 label=security-plan fact=fct-2jp5ovui
quote: Per-instance named pipes (Windows) or Unix domain sockets (a verified per-user 0700 directory, never the `/tmp` or `$TMPDIR` root); no Linux abstract namespace.
@carried n=5 run=3 label=security-plan fact=fct-qtqta3hw
quote: v1 intends to trust every process of the same OS user and no other; there is no per-connection authentication, the self-reported `from` authenticates nothing, and the endpoint name is predictable.
@carried n=6 label=security-plan/threat-model-summary fact=fct-t7qeqkiu
why: Every frame-writing client run by the Unix suite must make the socket-directory and euid checks on Linux and macOS, and no parity test may read macOS's carve-out as a pid or start-time comparison.
quote: ahead of that comparison, on Linux and macOS, each runs the socket-directory check on the recorded endpoint before connecting (a refusal `socket-dir-refused`, no frame) and the peer euid decision past the connect (a refusal `server-verify-failed`, no frame); macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry.
@carried n=7 label=security-plan/threat-model-summary fact=fct-t7qeqkiu
why: The one schema requires every OS to write only these six closed causes in CLI --json and MCP results, never an OS-only word, path or pid.
quote: An `instance-unreachable` on CLI `--json` or an MCP result names its cause in `detail`, one of six closed words from `viola_core::UnreachableCause` (no serde path, every word written by the two clients, no path, pid or upstream text)
@carried n=8 label=security-plan/threat-model-summary fact=fct-ql7nqspi
why: The scope names the pty seam's write, so a bridge send through openpty must be the same single bracketed-paste write as through ConPTY.
quote: A bridge send is one `ESC[200~…ESC[201~` + CR write of the typed text
@carried n=9 label=security-plan/threat-model-summary fact=fct-zyxi3lxw
why: The chunk's freight carries the cursor-position and kitty reply question, and no reply shape may join the closed list under the fake agent without a measurement on the founder's word.
quote: The stdin classifier's closed non-editing list (the founder's ruling F-W2): only a sequence on it never counts as typing, and every other byte does.
@carried n=9 run=2 label=security-plan fact=fct-hscavt6w
quote: only a sequence on the stdin classifier's closed non-editing list never counts as typing, and a reply no terminal has been measured to write is never added on a guess.
@carried n=10 label=security-plan/threat-model-summary fact=fct-6fvyxh3k
why: The hook must fail open on every OS, and the Unix-only statusline shell-out the Unix suite now runs must take its command only from a snapshot whose home holds its permissions.
quote: Any failure fails open to the human: exit 0 with no body.
  - `hook statusline` runs `statusline_command` from `snapshot.json` through a shell. That snapshot is found through `VIOLA_DIR`, so its integrity is set by filesystem permissions on the viola home.
@carried n=10 run=2 label=security-plan fact=fct-3ajwacjp
quote: `hook statusline` runs `statusline_command` through a shell, its integrity set by filesystem permissions on the viola home.
@carried n=11 label=security-plan/threat-model-summary fact=fct-bur5z45t
why: MCP results are documents the one schema aligns, and their socket-directory and euid refusals must read the same closed words with no frame and no line on both Unix OSes.
quote: Past the liveness pre-check `frame` runs the socket-directory check on the recorded endpoint, a refusal `instance-unreachable` with `detail` `socket-dir-refused`, and past the connect the peer euid decision beside the server check, a refusal with `detail` `server-verify-failed`, each with no frame and no line.
@carried n=12 label=security-plan/threat-model-summary fact=fct-qxr7dgpn
why: The Unix suite must read these strict-modes refusals with the same typed exit codes on Linux and macOS, and the v1-40 witness's viola list must keep the home read rule on every OS.
quote: Before its first read or open under the home, `run`, `revive`'s start and `verify` hold the home tree to the strict-modes check (`check_home_tree`; a refusal exit 1, no log opened), and the frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` hold the instance to it (`check_instance`; a refusal exit 21), so a state tree chosen by `--home` or `VIOLA_DIR` that another user can write, or whose home, instance directory or `diagnostics/` another user can read, is refused.
  - `viola list`'s board scan holds the home to the read rule on every OS (`check_read`) and `instances/` to `check_path`
@carried n=13 label=security-plan/threat-model-summary fact=fct-feql22h5
why: A Unix wrapper start under openpty must hold the home tree to the owner-and-mode check before its log opens, the Windows owner-and-DACL check left as it is.
quote: Beyond the content hash in the folder name, `run` and `revive`'s start hold the home tree to the strict-modes check (Unix owner and mode, Windows owner and DACL) before the wrapper log opens
@carried n=14 label=security-plan/threat-model-summary fact=fct-uemrwfo5
why: The openpty spawn must take the child the wrapper resolved itself and the PATH it built, as the ConPTY spawn does.
quote: `run` resolves the child executable and runs it once with `--version`.
@carried n=14 run=2 label=security-plan fact=fct-m3uflr5q
quote: the trust boundary is whatever the calling process's PATH resolves to
@carried n=15 label=security-plan/threat-model-summary fact=fct-uemrwfo5
why: The Unix child's PATH must put the pinned copy's folder first under openpty as under ConPTY.
quote: `run` puts the pinned copy's folder first on the child's PATH.
@carried n=16 label=security-plan/threat-model-summary fact=fct-v2lh3e2i
why: The chunk re-measures the CI test, perf and lint jobs over this three-OS matrix, which must read green on ubuntu-latest and macos-latest beside windows-2025.
quote: GitHub Actions `ci.yml`, triggered on push and PR, on a native matrix of `windows-2025`, `macos-latest` and `ubuntu-latest`
@carried n=16 run=2 label=security-plan fact=fct-ju2d5wlo
quote: GitHub Actions `ci.yml` on push and PR over a native `windows-2025`, `macos-latest` and `ubuntu-latest` matrix
@carried n=17 label=security-plan/threat-model-summary fact=fct-v2lh3e2i
why: The re-measured test and perf jobs run the fake-agent feature on the Unix legs, and the per-OS release build must keep refusing any test-only binary or feature.
quote: tests against the fake agent with recorded fixtures (per OS under coverage), the per-OS `perf` job (`agent-run run --perf`), MSRV 1.96, fuzz corpus replay, and the per-OS release build (`cargo build --release --locked --bin viola` through `scripts/release-check.sh`, which refuses any test-only binary).
@carried n=18 label=security-plan/threat-model-summary fact=fct-v2lh3e2i
why: The chunk has no live session, so its Unix legs must run verify against the fake agent only.
quote: The real `claude` CLI runs only locally; `viola verify` runs in CI only against the fake agent, and its real-CLI probe and `--record` only locally.
@binding n=1
**boundary-ipc ↔ authentication-authorization**: the Unix client checks the threat model states (security-plan/threat-model-summary fct-t7qeqkiu) must be the IPC access control (Unix) and client-side server verification rows (security-plan fct-dthgjfbs) (security-plan fct-phspeqis) on both Unix OSes, the server-side euid drop (security-plan fct-unwy7exn) included; on macOS `server-verify-failed` means an euid mismatch alone and no parity test reads it as a pid comparison.
@binding n=2
**threat-data ↔ bootstrap logging-redaction-wire and error-handling**: the one diagnostics schema across OSes (security-plan/threat-model-summary fct-qcqqc5fi) (security-plan/threat-model-summary fct-qgluq6ji) must keep the NEVER-log floor (security-plan fct-auxwpgtg) and the closed MCP and CLI --json codes (security-plan fct-kulo4ws7) on every OS, a value one OS cannot fill reading its documented null or closed word.
@binding n=3
**boundary-hook-stdin ↔ input-validation and error-handling**: the hook's fail-open and the statusline shell-out (security-plan/threat-model-summary fct-6fvyxh3k) must hold as the Unix-only statusline row (security-plan fct-2fgozo5r) and the hook contract (security-plan fct-r24nqlwe) state, exit 0 with no body and no stderr on every OS.
@binding n=4
**boundary-child-spawn and threat-data credential ↔ data-protection and input-validation**: the openpty spawn (security-plan/threat-model-summary fct-uemrwfo5) (security-plan/threat-model-summary fct-jy2pv6x7) must keep handle non-inheritance (security-plan fct-p7rzex2i), hand portable-pty the absolute path the wrapper resolved (security-plan fct-sgmmo3ki), and apply the R8 strip on Linux and macOS.
@binding n=5
**threat-infra ↔ secret-management, dependency-security**: the re-measured three-OS CI jobs (security-plan/threat-model-summary fct-v2lh3e2i) must keep the Unix-only non-0600 diagnostics canary class (security-plan fct-umh22lbq), the single G2 exemption (security-plan fct-wsxv34pw) and the release check refusing test-only features (security-plan fct-h3byb6pq) on the Unix legs.
@binding n=6
**boundary-terminal ↔ the scope's freight**: the closed non-editing list (security-plan/threat-model-summary fct-zyxi3lxw) admits no cursor-position or kitty reply shape under the fake agent, the list widened only by measurement on the founder's word (security-plan fct-hscavt6w).
