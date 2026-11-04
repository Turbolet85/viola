@carried n=1 label=obs-plan/obs-scope-summary fact=fct-tgmj25nt
why: the four process roles must be the same closed `process` words on every line on all three OSes, as the chunk re-asserts one diagnostics schema.
quote: - 5 telemetry surface entries across 4 process roles (`run` / `hook` / `mcp` / `cli`) and 2 product parts;
@carried n=1 run=2 label=obs-plan fact=fct-uwfffcsu
quote: Per line, role and instance are carried by the tests-required fields `process` (`run|hook|mcp|cli`) and `instance` (`ViolaName` or null)
@carried n=2 label=obs-plan/obs-scope-summary fact=fct-drmiyyaa
why: the multi-OS compat trigger is the obligation the chunk re-asserts: the same sink and line schema must hold on ubuntu-latest and macos-latest beside windows-2025.
quote: - 18 telemetry triggers, including cross-surface propagation through `corr`, perf budgets, chaos fault telemetry, multi-OS compat, and binding Founder Directions.
@carried n=2 run=2 label=obs-plan fact=fct-mjdbrsjn
quote: - **Source:** tests excerpt, Coverage Triggers, Multi-OS; arch CI/CD (windows-2025 / macos-latest / ubuntu-latest)
@carried n=2 run=3 label=obs-plan fact=fct-dao4nifd
quote: - **Required telemetry signal:** the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
@carried n=3 label=obs-plan/obs-scope-summary fact=fct-4jptauao
why: on both Unix OSes the role files must sit at the reconciled diagnostics root at 0600 in 0700 directories, where the gates and secret scan the chunk re-reads find them.
quote: - the `diagnostics/` directory root (tests `<home>/diagnostics/` vs arch and security `instances/<name>/diagnostics/`);
@carried n=3 run=2 label=obs-plan fact=fct-igz2gfrn
quote: - **Log file location:** obs aligns to tests on the basenames `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson` and the `logs` wrapper; the directory root is reconciled with a Decisions Log entry, and a home-level `mcp.ndjson` never carries user content; every file is 0600 inside a 0700 directory; there is no rotation in v1.
@carried n=3 run=3 label=obs-plan fact=fct-23yn4mby
quote: Until a wrap brings a difference current, §3, §6 and §12 win where they differ: diagnostics root (D-08, D-22)
@carried n=3 run=4 label=obs-plan fact=fct-wrd7y3z2
quote: `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL)
@carried n=4 label=obs-plan/obs-scope-summary fact=fct-lt3l45qm
why: `mcp` must write `mcp.ndjson` on every OS, stderr JSON only when that file cannot be opened, and never telemetry on stdout, so its sink is identical on Unix and Windows.
quote: - `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
@carried n=4 run=2 label=obs-plan/obs-scope-summary fact=fct-drz74jk7
quote: - **Reason if not fully instrumentable:** N/A. stdout is reserved for MCP frames (arch Stack and Obs-Relevant Conventions), so no telemetry may go to stdout.
@carried n=4 run=3 label=obs-plan/obs-scope-summary fact=fct-yqxfnq5h
quote: - **Instrumentability:** instrumentable
@carried n=4 run=4 label=obs-plan fact=fct-mawfkjfn
quote: - **Exporter:** always the file sink `<home>/diagnostics/mcp.ndjson`, stderr JSON only when that file cannot be opened; stdout is forbidden because it carries MCP frames, so the writer is set explicitly; `isError` results and the log lines for rejected calls carry codes only.
@carried n=5 label=obs-plan/obs-scope-summary fact=fct-iyfle2l2
why: the `run` wrapper on Linux and macOS must keep the root bin's error-capture boundary: fixed messages on stderr, the full chain only in `diagnostics/`, and no default panic hook.
quote: - **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
@carried n=5 run=2 label=obs-plan/obs-scope-summary fact=fct-d4mbb5n4
quote: - **Instrumentability:** instrumentable
@carried n=5 run=3 label=obs-plan fact=fct-cjx2g7cf
quote: - **Instrumentability:** instrumentable — the only crate that uses anyhow, so the error-capture boundary: a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
@carried n=6 label=obs-plan/obs-scope-summary fact=fct-y6hfs4wy
why: the exit-21 cause words, `socket-dir-refused` and macOS's `server-verify-failed` included, must be the same closed words in every `--json` and `isError` document on every OS, with no path or pid in a detail code.
quote: `viola_core::UnreachableCause` is the closed set of exit 21's six cause words (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`), with no serde path. The two clients write each word into their two documents (the CLI's `--json` exit-21 document, MCP's `isError` body); the type writes no log line of its own, and a refused strict-modes check writes no line.
@carried n=6 run=2 label=obs-plan/obs-scope-summary fact=fct-5xfc4cig
quote: - **Instrumentability:** not-instrumentable
@carried n=6 run=3 label=obs-plan fact=fct-y6qmqcu2
quote: each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
@carried n=6 run=4 label=obs-plan fact=fct-wrd7y3z2
quote: a refused socket directory, read by the strict-modes read verdict, is no strict-modes failure: its line, where it has one, carries its own `detail`, `socket-dir-refused`.
@carried n=6 run=5 label=obs-plan fact=fct-5wgriooq
quote: the absolute paths in `snapshot.json` and config values are scrubbed from external output.
@carried n=7 label=obs-plan/obs-scope-summary fact=fct-jzzdh6ip
why: the openpty backend the chunk runs behind the PTY seam must be instrumented at the seam only, and the PTY byte stream must never be logged on any OS.
quote: - **Reason if not fully instrumentable:** N/A for the seam. The ConPTY and openpty OS APIs behind it are boundary-only (see the OS PTY entry). The PTY byte stream is user content and is never logged (security Data Classifications, user content; brief §3.2 "Nobody parses the rendered screen for content").
@carried n=7 run=2 label=obs-plan/obs-scope-summary fact=fct-cfbdfwxc
quote: - **Instrumentability:** instrumentable
@carried n=7 run=3 label=obs-plan fact=fct-t2iihkln
quote: - **Entity:** `viola-pty`: PTY seam (spawn, read, write, resize, wait, kill) over portable-pty `=0.8.1`, plus the windows-sys `TerminateProcess` fallback
@carried n=7 run=4 label=obs-plan fact=fct-m24p7svx
quote: - **Instrumentability:** instrumentable at the seam; the ConPTY and openpty OS APIs behind it are boundary-only, and the PTY byte stream is user content, never logged.
@carried n=8 label=obs-plan/obs-scope-summary fact=fct-aowjxl3a
why: the sync crates must stay Tokio-free in their Unix dependency graphs, as the lint job's cargo deny the chunk re-reads green on ubuntu-latest and macos-latest asserts.
quote: - **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
@carried n=8 run=2 label=obs-plan/obs-scope-summary fact=fct-mr2vh66r
quote: - **Instrumentability:** instrumentable
@carried n=8 run=3 label=obs-plan/obs-scope-summary fact=fct-pssgrgft
quote: - **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.
@carried n=8 run=4 label=obs-plan/obs-scope-summary fact=fct-qrw2kvvm
quote: - **Instrumentability:** instrumentable
@carried n=8 run=5 label=obs-plan fact=fct-hezc5s2v
quote: - **Instrumentability:** instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
@carried n=8 run=6 label=obs-plan fact=fct-ewk35na4
quote: - **Instrumentability:** instrumentable, and the instrumentation must be sync: Stack bans Tokio here.
@carried n=9 label=obs-plan/obs-scope-summary fact=fct-add24jhw
why: no R8-stripped `CLAUDE*` value may reach any diagnostics file on any OS, which the secret scan the chunk re-reads green on both Unix OSes must witness.
quote: Drift reports go to `diagnostics/` only (security Vector 4).
  - R8-stripped `CLAUDE*` values are never logged anywhere (security Vector 6).
@carried n=9 run=2 label=obs-plan/obs-scope-summary fact=fct-vaolg4ug
quote: - **Instrumentability:** instrumentable
@carried n=9 run=3 label=obs-plan fact=fct-gira4fst
quote: - **Instrumentability:** instrumentable, its outputs the most sensitive in the product: drift reports go to `diagnostics/` only, and R8-stripped `CLAUDE*` values are never logged anywhere.
@carried n=10 label=obs-plan/obs-scope-summary fact=fct-t3jwc6wb
why: the openpty wrapper must log the child's spawn and its exit detected on the process handle, never on reader EOF, exactly as the ConPTY wrapper does.
quote: - **Reason if not fully instrumentable:** It is a third-party binary run unmodified. Instrument only the boundary:
  - spawn and exit (`process-start` / `process-exit`), with exit detected on the process handle, never on EOF (Stack; brief §4.1);
@carried n=10 run=2 label=obs-plan/obs-scope-summary fact=fct-2drawtvt
quote: - **Instrumentability:** boundary-only
@carried n=10 run=3 label=obs-plan fact=fct-avrouaxx
quote: - **Instrumentability:** boundary-only — a third-party binary run unmodified: its spawn and exit (exit detected on the process handle, never on EOF), paste writes and the vt100 readiness-gate outcome; its internal state and transcripts are never read as a contract.
@carried n=10 run=4 label=obs-plan fact=fct-ctz7espz
quote: - **Exporter:** a file sink only, to `diagnostics/run-<name>.ndjson`; `run` writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns; instrumentation is sync, with no Tokio; the child's `process-exit` comes from the handle wait, never from reader EOF.
@carried n=11 label=obs-plan fact=fct-26ufyipy
why: `run`'s and a revived start's pre-child refusals on Unix (a refused socket directory, a home tree another user could write or read) must write exactly the fixed two-line stderr pair, and nothing of their own while the child runs.
quote: - `run` writes nothing to the terminal except the child's own output, with one exception before any child is spawned: a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint, a socket directory `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`), a home tree another user could write or read: the home pair for a home-level refusal, the instance pair for an instance-level one, before any log opens) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed — the root bin's human-stderr refusal writer, called by `run` (8 sites), `verify` (6), `list` (2), `revive` (1), `plugin install` (1) and `client` (1, `client::refuse_untrusted`, the one site `viola link` / `viola unlink` and the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` share), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-11-01-unix-endpoint-and-home-hardening; `list`'s colour layer adds no stderr writer and its `unable` / `hint:` pair carries no SGR. `viola revive` (its start arm logs as process `run` and makes `run`'s home-tree check before that log) adds three refusals to `run`'s six: state files another user could write, no logged session (or an `--id` the log does not hold) and a missing recorded directory; on a live name it prints `run`'s own collision pair. `viola revive --list` writes the pair for its two refusals and nothing else on stderr. While the child runs, neither `run` nor a revived start writes anything of its own.
@carried n=12 label=obs-plan fact=fct-k3pbv5ix
why: on every OS each role's codes-only log must land in its home-level role file and content-bearing detail only in the instance detail file, the split G2, G4 and the secret scan read.
quote: - Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.
@carried n=13 label=obs-plan fact=fct-bemloa4f
why: under the Unix fake-agent suite the hook must keep its contract: stdout only the decision body or the status-line output, stderr empty.
quote: - `hook` writes only its decision body to stdout and nothing to stderr. The one other thing a `hook` stdout carries is `viola hook statusline`'s: the user's status-line command's output byte for byte (nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden `--capture` arm, which prints nothing). No telemetry reaches stdout on either arm.
@binding n=1
**obs-scope-summary multi-OS compat ↔ log-coverage required fields**: the same file sink and line schema on every OS (obs-plan fct-dao4nifd) must carry the required fields with null encoded by key absence (obs-plan fct-qaeihrqm), so a field one Unix OS cannot fill reads by absence, never by an OS-only word, and the four `process` roles stay the same closed words (obs-plan/obs-scope-summary fct-tgmj25nt).
@binding n=2
**obs-scope-summary viola-pty and wrapped child ↔ log-coverage PTY seam wrapper**: the openpty seam is instrumented only at the seam (obs-plan/obs-scope-summary fct-jzzdh6ip), and its spawn and exit must log as the seam wrapper requires: `process-start{subject:"claude-child"}`, exit from the handle-wait thread, `exit_source:"kill-fallback"`, paste as `text_bytes` (obs-plan fct-3epowryi), never on EOF (obs-plan/obs-scope-summary fct-t3jwc6wb).
@binding n=3
**obs-scope-summary viola-core cause words ↔ span-trace-coverage wait/last scenario**: the exit-21 words `server-verify-failed` and `socket-dir-refused` (obs-plan/obs-scope-summary fct-y6hfs4wy) must surface on macOS as the euid-only carve-out and on both Unix OSes as the refused-socket-dir exit the scenario fixes, with no `channel-request` line and MCP writing no line (obs-plan fct-q67chfay).
@binding n=4
**obs-scope-summary diagnostics root ↔ ci-integration homes and gates**: the role files at the reconciled root (obs-plan/obs-scope-summary fct-4jptauao) are only covered on Unix when every integration-test home lives under `target/e2e-home/` (obs-plan fct-xtfnzpyu), where G2 (obs-plan fct-eodqccd6) and G4 (obs-plan fct-htkftgtl) read them.
@binding n=5
**obs-scope-summary sync crates ↔ slo-invariants lint failure conditions**: the Tokio-free sync crates (obs-plan/obs-scope-summary fct-aowjxl3a) are held by the `cargo deny` failure condition (obs-plan fct-5kn2jpbt), which must read green on ubuntu-latest and macos-latest over their Unix dependency graphs.
@binding n=6
**obs-scope-summary agent-crate never-log floor ↔ pii-scrubbing secret scan**: R8-stripped `CLAUDE*` values never logged (obs-plan/obs-scope-summary fct-add24jhw) are what the secret-scan test over every `diagnostics/` file asserts before upload (obs-plan fct-tcjwyf2p), and it must pass on both Unix OSes.
@binding n=7
**body heartbeat ban ↔ slo-invariants liveness signal**: the `v1-40` witness under the outer PTY must read staleness through the 1 s heartbeat and the >5 s flip with no transition logged (obs-plan fct-gqc4mh6b), never through a per-tick line (obs-plan fct-ie2ur45l).
