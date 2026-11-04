@carried n=1 label=architecture/occupied-resources fact=fct-gftfjtay
why: The one diagnostics schema must hold this closed enum identically on Linux, macOS and Windows, never an OS-only word, in the reached schemas/diag-line.v1.json.
quote: `channel-response`'s `detail` in `schemas/diag-line.v1.json` is the closed enum `unsupported-version` · `invalid-params` · `release-from-driver`, written at WARN on both the server's and the client's line by `ResponseLine`'s `Drop` (`ProtocolError::cause`)
@carried n=1 run=2 label=architecture fact=fct-gn6wx4l2
quote: `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json`: the obs-owned default-deny line schemas for home-level process-log lines and instance detail lines. Each is self-contained (no cross-file `$ref`), and both are pinned by `tests/contract_diag_schema.rs`.
@carried n=2 label=architecture/occupied-resources fact=fct-if66544y
why: These closed process-exit words must read the same on every OS under the one-schema rule, a code one OS cannot reach staying a word of the same closed set.
quote: the `process-exit` `verify` writes under a parsed `VIOLA_NAME` carries its `Ended::Refused` code (`src/cmd/verify.rs`) in `detail`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture`, beside `interrupted`.
@carried n=3 label=architecture/occupied-resources fact=fct-wdtionxy
why: A parity test over role or detail file opens must read the Unix leg (the read rule on the file) and the Windows leg (the write rule) each on its own runner, its oracle per OS.
quote: a symbolic link is refused, a folder takes the read rule, a file takes the read rule on Unix and the write rule on Windows, and an absent path passes. A refusal is a failed open: the writer ends at `std::io::sink`, and a detail line is dropped.
@carried n=3 run=2 label=architecture fact=fct-5jecrew3
quote: Every open of a role file or a detail file first checks the existing `diagnostics/` folder and the file, before anything is narrowed: a symbolic link refused, the folder to the read rule, the file to the read rule on Unix and the write rule on Windows, an absent path passing;
@carried n=4 label=architecture/occupied-resources fact=fct-hn66ngvt
why: The strict-modes cases left cfg(windows) must carry this per-OS reason where they stand, the Unix leg proven on ubuntu-latest and macos-latest, and no Windows case moved to make a Unix one pass.
quote: The check is proven on Unix (five widened modes). On Windows `cfg(windows)` cases prove it: an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and a plain home made before viola is refused for its inherited write grant to a SID `trusted_sid` does not trust, read by its right whichever SID holds it, while a home `instance_tree()` makes reads none (as measured at ci#40439188053, `windows-2025`).
@carried n=4 run=2 label=architecture fact=fct-klpb7auf
quote: Per-session settings override: `instances/<ViolaName>/settings.json` holds a `statusLine` command running the pinned path's `hook statusline`, rewritten whole at every start before the spawn; written only on Unix, and only for a pinned path of plain characters, else no override is written and no `--settings` is passed.
@carried n=4 run=3 label=architecture fact=fct-cicfj3bq
quote: `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds; a refused check writes no line.
@carried n=5 label=architecture/occupied-resources fact=fct-kekl3p46
why: A parity test of the read rule must read mode bits on Unix and ACEs on Windows, each on its own runner, never one OS's oracle on the other.
quote: The read rule is the write rule plus, on Unix, a refusal of any group or other bit (`mode & 0o077`) and, on Windows, a refusal of any allow ACE, an inherit-only one included, that grants a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects (`unix_read_verdict`, `windows_read_verdict`, `check_read`).
@carried n=6 label=architecture/occupied-resources fact=fct-sbmskf35
why: Any test the chunk ungates or adds that plants a regular file above a checked path must take a per-OS oracle, the Unix refusal on ubuntu-latest and macos-latest and the catch-site line on Windows.
quote: A path under a regular file stats `NotFound` on Windows, so the strict-modes check skips it as absent and the `cli` catch-site line stands there; on Unix the same home is a strict-modes refusal (`verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair). A test that plants a regular file above a checked path therefore needs a per-OS oracle (as measured at ci#40398601594, `windows-2025`).
@carried n=7 label=architecture/occupied-resources fact=fct-isxdwj6d
why: The v1-40 integration witness runs `viola list` on Unix under the outer PTY, so its test home must stand at the read rule or the whole board refuses before any row is painted.
quote: `viola list` holds the home to the read rule on every OS: `board::scan` reads it through `check_read` with or without an instance, so a 0750 home, or on Windows one granting read to a SID `trusted_sid` rejects, refuses the whole board (`ScanRefused::Home`).
@carried n=7 run=2 label=architecture fact=fct-lak6evbp
quote: Read by every `viola` build's human output (the colour decision of `viola list`, `src/human/style.rs` `read`; release builds included), never set by viola: `NO_COLOR` (present, an empty value included, switches colour off), `TERM` (exactly `dumb` switches it off; a value containing `256color` picks 256 colours) and `COLORTERM` (`truecolor` or `24bit` picks truecolor).
@carried n=8 label=architecture/occupied-resources fact=fct-jv5gqhbl
why: The kill-and-revive reading the CARRY keeps on Unix must wait on `holder_gone` inside the `WITHIN` bound, a killed wrapper's socket file staying on Unix, while the standalone pin stays cfg(windows).
quote: One of the waits is `tests/support/home.rs` `wait_endpoint_gone`, which `Wrapper::stop` / `stop_keep` run after the wrapper's exit until the snapshot's recorded `endpoint` is unconnectable; after a kill, `Wrapper::kill` waits on `holder_gone` instead (on Unix a connect to the path is refused or the path is absent, on Windows the pipe is not found), because a killed wrapper removes nothing and its socket file stays on Unix, where the stop path's rule is the file being absent (as measured at chunk 2026-10-10-viola-revive, the kill-and-revive case green on the three CI runners)
@carried n=8 run=2 label=architecture/occupied-resources fact=fct-flit34us
quote: Test-only: harness `cleanup`'s `endpoint_gone` on Unix reads through `unconnectable` (`crates/viola-e2e/src/harness/cleanup.rs`): `UnixStream::connect` refused or `NotFound` reads gone, so `cleanup` reads `ok:true` after a force-killed wrapper, removing no leftover file; `holder_gone`, above, holds the same reading for `Wrapper::kill`.
@carried n=8 run=3 label=architecture fact=fct-w4pd7id3
quote: Test-only, outside the viola home: `<temp dir>/viola-root-watch/<test name>.<label>.report`, the evidence file of the root test support's `Watch` (bound `WITHIN` = 7 s), kept when the test panics or is killed and removed when it passes. `viola` never reads or writes it.
@carried n=9 label=architecture/occupied-resources fact=fct-5v4ir6ih
why: The test-owned socket roots in the reached tests/support/home.rs must keep their per-OS variable, TMPDIR on macOS and XDG_RUNTIME_DIR on Linux, so the Unix socket-dir tests run on both runners.
quote: Test-only, outside the viola home: the test-owned socket roots under `/tmp`. `tests/support/home.rs` `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on` give a test its own socket root, `SOCKET_ROOT_VAR` being `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere, used by `tests/security_negatives_socket_dir.rs`.
@carried n=9 run=2 label=architecture fact=fct-wular7qm
quote: Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory, never the `/tmp` or `$TMPDIR` root, checked before the bind and before every client's connect and, when refused (`socket-dir-refused`), left as found, never narrowed;
@carried n=10 label=architecture/occupied-resources fact=fct-fs7kfa27
why: The perf job re-read green on ubuntu-latest and macos-latest must produce all five perf exports that `gate --require perf` requires.
quote: `artifacts/` holds the per-suite JUnit, `run-summary.json`, `llvm-cov-summary.json` (from `run --coverage`) and the five `perf-<hook>.json` hyperfine exports (from `run --perf`, one per timed row, each required by `gate --require perf`); the perf session's `<session>/` also holds its synthetic `payload-<hook>.json` inputs.
@carried n=10 run=2 label=architecture fact=fct-37dicsq3
quote: Its hyperfine exports are `target/agent-run/artifacts/perf-<hook>.json`, one per row (`session-start`, `user-prompt-submit`, `stop`, `session-end`, `pre-tool-use`), all five required by `gate --require perf` (obs-plan §9); the perf session boots unstamped.
@carried n=11 label=architecture/occupied-resources fact=fct-xib5fpty
why: The obs gates G1 to G4 the chunk re-reads green on Unix read exactly these kept homes, the one chaos home outside them.
quote: every harness and test home but the one `viola-chaos-*` home of `tests/chaos_feed_panic.rs` (Filesystem, above), gitignored and kept in CI for the obs gates (never under `run --mutants`, which keeps no home).
@carried n=11 run=2 label=architecture/occupied-resources fact=fct-x2qzlbqt
quote: `v1-47`'s after every end-to-end test is after each job's end-to-end run: `AGENT_RUN_KEEP_HOMES` `"1"` on the job and `if: always()` in `.github/workflows/ci.yml`, the `viola-chaos-*` home out.
@carried n=11 run=3 label=architecture fact=fct-swegdoif
quote: Its forced vt100 feed panic writes a G2-counted `event:"panic"` line, so the home sits outside `target/e2e-home` and therefore outside G2 (zero panics), G4 (schema conformance) and the secret scan;
@carried n=12 label=architecture/occupied-resources fact=fct-xib5fpty
why: The Linux suite runs on the dev host go through the linked base, so a stamped-home gate there must first meet the headroom floor (the directive's `df -h /tmp` read), while CI runners have no link.
quote: A linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` (`f_bavail × f_frsize`) is refused too, with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor` (the two counts, no path);
@carried n=12 run=2 label=architecture fact=fct-cny66nid
quote: Test-only, outside the repository, on the Linux dev host only: `/tmp/viola-e2e-home-<uid>/` (mode 0700, owned by the user), the tmpfs directory the dev host's `target/e2e-home` link points at, holding every test and session home of that host. `viola` never reads or writes it by that name. No CI runner has it.
@binding n=1
**res-filesystem strict-modes rules ↔ Cross-cutting Patterns (cross-platform discipline)**: each per-OS leg of the strict-modes check (the role file's read rule on Unix, write rule on Windows; mode bits against ACEs; a regular file above a checked path) must be tested on its own CI runner, a Unix case on ubuntu-latest and macos-latest and a Windows case staying cfg(windows) with its reason named where it stands (architecture/occupied-resources fct-wdtionxy) (architecture/occupied-resources fct-kekl3p46) (architecture/occupied-resources fct-sbmskf35) (architecture/occupied-resources fct-hn66ngvt) (architecture fct-fa2klbw5)
@binding n=2
**res-filesystem diag-line closed words ↔ Conventions (data model)**: the one-schema rule holds diag-line.v1.json's closed enums and the process-exit refusal codes to one closed set on every OS, and a field one OS cannot fill is omitted or null, never an OS-only word (architecture/occupied-resources fct-gftfjtay) (architecture/occupied-resources fct-if66544y) (architecture fct-u5xw5irn)
@binding n=3
**res-filesystem test homes ↔ obs gates G1 to G4 and the ci.yml re-read**: the Unix test and perf jobs keep every test home under `target/e2e-home` (`AGENT_RUN_KEEP_HOMES`, `if: always()`) so G1 to G4 read them green beside windows-2025, the chaos home outside G2 and G4, and the perf job yields all five exports `gate --require perf` requires (architecture/occupied-resources fct-xib5fpty) (architecture/occupied-resources fct-x2qzlbqt) (architecture/occupied-resources fct-fs7kfa27) (architecture fct-swegdoif)
@binding n=4
**res-filesystem kill support ↔ Standard Contracts (session liveness)**: the Unix kill-and-revive reading waits on `holder_gone` within `WITHIN` = 7 s and reads the child gone by pid and start time, while `run_child_ends_when_its_wrapper_is_terminated` stays cfg(windows) (architecture/occupied-resources fct-jv5gqhbl) (architecture/occupied-resources fct-flit34us) (architecture fct-arftgre4)
@binding n=5
**res-filesystem `viola list` read rule and res-env-vars colour names ↔ Standard Contracts (session liveness)**: the v1-40 witness must run `viola list` on a home at the read rule, with `NO_COLOR` absent and `TERM` not `dumb` so SGR bytes are written, beside a wrapper whose heartbeat is older than 5 s (architecture/occupied-resources fct-isxdwj6d) (architecture fct-lak6evbp) (architecture fct-arftgre4)
@binding n=6
**res-filesystem socket roots ↔ res-ipc-endpoints (Unix socket)**: the per-OS socket root (`TMPDIR` on macOS, `XDG_RUNTIME_DIR` on Linux) gives each Unix test its own per-user socket directory, never the `/tmp` or `$TMPDIR` root (architecture/occupied-resources fct-5v4ir6ih) (architecture fct-wular7qm)
