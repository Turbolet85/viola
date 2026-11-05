@carried n=1 label=architecture/occupied-resources fact=fct-vs5zumck
why: The home-tree check must lose the Windows name of the pinned copy with its arm, while its Unix write-rule check over the same entries stays exactly as it is.
quote: with a key, `bin/<key>/`, the pinned `viola` (`viola.exe` on Windows), `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (the write rule). It checks only entries that exist and refuses as `TreeRefused::HomeLevel` or `TreeRefused::InstanceLevel`.
@carried n=1 run=2 label=architecture fact=fct-hnee7xdr
quote: `bin/<version>-<hash>/viola(.exe)`: the pinned copy of the binary, which `viola run` places once when it is absent, with no clobber, and never replaces (0700 on Unix)
@carried n=1 run=3 label=architecture fact=fct-lvi3drxp
quote: Binary: `viola` (`viola.exe`).
@carried n=2 label=architecture/occupied-resources fact=fct-sbmskf35
why: Tests that plant a regular file above a checked path must drop their Windows oracle and keep the Unix strict-modes refusal (verify exit 1, frame verbs exit 21) as their one, ungated oracle.
quote: A path under a regular file stats `NotFound` on Windows, so the strict-modes check skips it as absent and the `cli` catch-site line stands there; on Unix the same home is a strict-modes refusal (`verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair). A test that plants a regular file above a checked path therefore needs a per-OS oracle (as measured at ci#40398601594, `windows-2025`).
@carried n=3 label=architecture/occupied-resources fact=fct-isxdwj6d
why: The read rule's Windows SID half must leave with its arm while a 0750 home still refuses the whole board on Linux.
quote: `viola list` holds the home to the read rule on every OS: `board::scan` reads it through `check_read` with or without an instance, so a 0750 home, or on Windows one granting read to a SID `trusted_sid` rejects, refuses the whole board (`ScanRefused::Home`).
@carried n=3 run=2 label=architecture fact=fct-cicfj3bq
quote: the home, the instance directory and its `diagnostics/` to the read rule (Unix no group or other bit; Windows no read right granted to another SID), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds; a refused check writes no line.
@carried n=3 run=3 label=architecture fact=fct-5jecrew3
quote: a symbolic link refused, the folder to the read rule, the file to the read rule on Unix and the write rule on Windows, an absent path passing; a refusal is a failed open (the role file's writer ends at a sink, a detail line is dropped).
@carried n=4 label=architecture/occupied-resources fact=fct-jznqbn54
why: The version gate's strict stamps read must keep its owner-only Unix check, now ungated, and its `cli_verified:false` refusal record, while the owner + DACL Windows half leaves.
quote: Readers use `read_stamps` (no lock; over the cap is an error), except `run`'s version gate, which reads through `read_stamps_strict` (the `viola_state::strict` check first: owner-only modes on Unix, owner + DACL on Windows); a refusal records `cli_verified:false` and one `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}`.
@carried n=4 run=2 label=architecture fact=fct-vevvxa42
quote: `ledger/` 0700 and both files 0600 on Unix. The ONE writer is `viola_state::stamps::update_stamps`, called only by `viola verify`; `run`'s version gate reads it behind the strict-modes check, a refusal recording `cli_verified:false`.
@carried n=5 label=architecture/occupied-resources fact=fct-i7o6tqmg
why: The Unix-only statusline probe settings and the per-session settings override must lose their Unix gate with no change in what Linux writes.
quote: on Unix, when `probe_document` is `Some`, `typed/statusline.json` (0600) is Run B's probe settings.
@carried n=5 run=2 label=architecture fact=fct-klpb7auf
quote: Per-session settings override: `instances/<ViolaName>/settings.json` holds a `statusLine` command running the pinned path's `hook statusline`, rewritten whole at every start before the spawn; written only on Unix, and only for a pinned path of plain characters, else no override is written and no `--settings` is passed.
@carried n=6 label=architecture/occupied-resources fact=fct-c3nhxk2n
why: The Windows console-control arm of verify's stop handler must leave while the Unix SIGINT / SIGTERM handling and the drop guards' removal stay unchanged.
quote: All are removed when verify passes, fails, or is stopped by Ctrl-C or TERM: Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT` (`cmd::verify::stop`, installed by `viola verify` alone, whose handler only sets one process-global flag that every wait of the run reads).
@carried n=6 run=2 label=architecture fact=fct-dut3tvmp
quote: each a drop guard: removed when verify passes, fails or is stopped by Ctrl-C or TERM, only the dirs that run created, never by a pattern or a listing.
@carried n=7 label=architecture/occupied-resources fact=fct-flit34us
why: Harness cleanup's Unix endpoint reading must lose its gate once its Windows sibling leaves, keeping the same `ok:true` reading after a force-killed wrapper.
quote: Test-only: harness `cleanup`'s `endpoint_gone` on Unix reads through `unconnectable` (`crates/viola-e2e/src/harness/cleanup.rs`): `UnixStream::connect` refused or `NotFound` reads gone, so `cleanup` reads `ok:true` after a force-killed wrapper, removing no leftover file; `holder_gone`, above, holds the same reading for `Wrapper::kill`.
@carried n=8 label=architecture/occupied-resources fact=fct-w6vpbrjt
why: The Frozen guard must keep only its Unix twin, ungated, as the guard's one home, and no Windows-gated file may be left behind for orphans-check to trip on.
quote: one twin per OS under its own gate: on Unix `attach` sends `kill -STOP` and polls `ps -o stat=` until it reads `T`, and the drop sends `kill -CONT`; on Windows `attach` attaches a debugger with kill-on-exit off and waits for the first debug event, and the drop detaches. It is the guard's one home
@carried n=8 run=2 label=architecture fact=fct-eesxehzs
quote: `target/orphans-probes/run-<utc>-<pid>/`: the throwaway crates `scripts/orphans-check.sh --probe` writes per run, each with an empty `[workspace]`: a planted `src/stray.rs`, a file gated on a target no CI triple compiles, and a `cfg(windows)`/`cfg(unix)` file-pair control (verdict `orphans-check probes: 2/2 fired, control clean`), gitignored.
@carried n=9 label=architecture/occupied-resources fact=fct-bh2gc2qq
why: The exit-cause table's four checks must stay green after the removal, as a witness that the typed exit causes come out unchanged on Linux.
quote: Test-only: `tests/cross_exit_causes.rs` is the exit-cause table, a root `cross_` binary, with four checks over the table itself: `no_two_codes_share_a_hint`, `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`.
@carried n=9 run=2 label=architecture fact=fct-qslc37ld
quote: Exit codes `0, 1, 2, 10–14, 20, 21` as defined in Conventions.
@carried n=10 label=architecture/occupied-resources fact=fct-5pbk7sp3
why: The floor guards a tmpfs backing that only the Linux dev host has, so the two backing cases must decide on a figure the test controls and never on the host's real free space.
quote: made again after a reboot by the keepers `prepare_home_base` and `Workspace::ensure_e2e_home`, which refuse a target that is not a real, owner-only directory or whose filesystem reads less than the 9 GiB headroom floor free (Repository, below).
@carried n=10 run=2 label=architecture fact=fct-cny66nid
quote: `viola` never reads or writes it by that name. No CI runner has it.
@carried n=11 label=architecture/occupied-resources fact=fct-xib5fpty
why: The msrv fix must hand the backing cases a free-space figure the test supplies while the floor stays one compiled constant per keeper, equal in both, set by no variable, flag or config key, with its fixed message unchanged.
quote: A linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` (`f_bavail × f_frsize`) is refused too, with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor` (the two counts, no path); the floor is one compiled constant per keeper, equal in both and set by no variable, flag or config key, its basis the tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/headroom.md`). Anything else refuses the start with a fixed message that names no path: the root keeper panics, and the harness keeper's error reads the existing `build-failed` at `boot` and `verify-exit-none` at `run --local-live` (no reason added). On Windows the keepers are the plain create.
@carried n=11 run=2 label=architecture fact=fct-e4sufzw6
quote: the base is prepared by two test-side keepers that hold one contract.
@carried n=12 label=architecture/occupied-resources fact=fct-5v4ir6ih
why: The socket-root variable's macOS arm must leave, `XDG_RUNTIME_DIR` becoming the one variable, with the Linux socket rules unchanged and the Windows pipe rows losing their code.
quote: `SOCKET_ROOT_VAR` being `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere, used by `tests/security_negatives_socket_dir.rs`.
@carried n=12 run=2 label=architecture fact=fct-wular7qm
quote: Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory, never the `/tmp` or `$TMPDIR` root, checked before the bind and before every client's connect and, when refused (`socket-dir-refused`), left as found, never narrowed; the socket is chmod 0600 after the bind; every other process connects to the `endpoint` recorded in the instance snapshot, and both sides admit only a peer of the owning user's euid, the wrapper dropping another before any frame.
@carried n=12 run=3 label=architecture fact=fct-vjccfh2l
quote: **Windows:** named pipe `\\.\pipe\viola-<h12>`, one per `viola run`.
@carried n=12 run=4 label=architecture fact=fct-s2vyyjop
quote: named pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` (Windows), created by `tests/channel_sqos_open.rs`, and endpoints named `viola-test-chan-<pid>-<label>` (a named pipe on Windows, a socket of that base name elsewhere)
@carried n=13 label=architecture/occupied-resources fact=fct-3fjm2cf6
why: The relied ledger set must lose its Windows count with its arm while `contract_ledger_probes` keeps reading 24 pass on Linux.
quote: is stamped at the relied set (`LedgerRow::relied()`: 24 rows on Unix, 21 on Windows) by `contract_ledger_probes` (its `ROW_IDS` the twenty-four ids; `24 pass  0 fail` on Unix)
@carried n=14 label=architecture/occupied-resources fact=fct-fs7kfa27
why: The per-OS harness, JUnit and secret-scan uploads the shrunk CI matrix reaches must keep their exclusions on the Linux job, and the Windows relink rationale loses its subject.
quote: `chunk.diff` (`run --mutants`' diff, repository source text) is the one file `secret-scan` skips, by exact path, and the `harness-<os>` upload excludes it.
@carried n=14 run=2 label=architecture fact=fct-6edq3mfh
quote: `chunk.diff` is the one file `secret-scan` skips, by exact path, and the `harness-<os>` upload excludes it.
@carried n=14 run=3 label=architecture fact=fct-io6ijhtt
quote: CI uploads it as artifact `secret-scan-<os>` (7 days) only when the scan fails.
@carried n=14 run=4 label=architecture fact=fct-5q3bjmyd
quote: `target/harness/`: the harness's own cargo target dir (`CARGO_TARGET_DIR` for its builds and test runs), because a running `target/debug/viola-harness.exe` cannot be relinked on Windows.
@carried n=15 label=architecture/occupied-resources fact=fct-snmldjr6
why: The ConPTY companions, `seed_conpty`, `target/conpty-seed/` and `vendor/conpty/` must leave with the Windows code, so these rows lose their subject and the plan names them as expected amendments.
quote: An inheritable `icacls` grant (`(OI)(CI)`) on a seeded test home's `bin/`, or on the home, reaches through `seed_conpty`'s hard links into the shared `target/conpty-seed/<key>/` companion and widens every seeded home until it is removed;
@carried n=15 run=2 label=architecture fact=fct-qe46hlhe
quote: `target/conpty-seed/<key>/` (Windows x64): the one per-run copy of the ConPTY companions that root `tests/support/home.rs` `seed_conpty` hard-links (copy fallback) into a test home's `bin/<key>/conpty/` before `Wrapper::boot`
@carried n=15 run=3 label=architecture fact=fct-ekftm72e
quote: `bin/<version>-<hash>/conpty/OpenConsole.exe` and `bin/<version>-<hash>/conpty/conpty.dll` (Windows x64 only): Microsoft's ConPTY companions embedded in the binary, written by `viola run` if absent (`pin_companions` through `replace_private_shared`) and never overwritten;
@carried n=15 run=4 label=architecture fact=fct-rryrskwd
quote: `vendor/conpty/<version>/x64/{conpty.dll,OpenConsole.exe}` (today `1.24.260710001`): the committed Microsoft binaries from the nuget package `Microsoft.Windows.Console.ConPTY` (MIT), byte-identical to the pinned nupkg, `.gitattributes` `vendor/conpty/** binary`, embedded by `src/conpty.rs`.
@carried n=15 run=5 label=architecture fact=fct-swegdoif
quote: It is the second named carve-out to the test-data rule, beside `seed_conpty` (the founder's ruling, 2026-10-04).
@carried n=16 label=architecture/occupied-resources fact=fct-jyuckxyp
why: The Windows and macOS JUnit artifacts this operator instrument downloads no longer exist once those jobs leave, so the plan must re-point or retire it.
quote: `--run <id>` downloads the `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest` artifacts with `gh run download` into a `tempfile.mkdtemp()` dir removed on exit,
@carried n=16 run=2 label=architecture fact=fct-ziwerfk3
quote: `target/nextest/ci/junit.xml`: the nextest `ci` profile's JUnit, gitignored. The `junit-<os>` upload (behind the secret scan) carries it alone.
@carried n=17 label=architecture/occupied-resources fact=fct-xtqblyzi
why: The Windows mutation-scratch arm must leave with `windows-mutants.yml`, the harness's off-Windows behaviour (scratch inert, `target/mutants` its target dir) staying as it is on Linux.
quote: On a Windows host (`HOST_SCRATCH = cfg!(windows)`) `run --mutants` puts cargo-mutants' temp copies there (`TMP`/`TEMP` on the `cargo mutants` command) and its `--output`, so `mutants.out/` is `<scratch>/mutants.out/`.
@carried n=17 run=2 label=architecture fact=fct-7wuxnmfc
quote: Off Windows the harness arm is inert.
@carried n=17 run=3 label=architecture fact=fct-4jytw3f7
quote: `target/mutants/`: the `run --mutants` cargo target dir (`MUTANTS_TARGET`).
@carried n=18 label=architecture/occupied-resources fact=fct-ymvbuev2
why: The Windows DACL oracles, `strict::win`'s test and the test-support helpers `canonical_sddl`, `dacl_of` and `user_sid` must leave with the Windows code, so these measured rows lose their subject.
quote: The protected owner-only DACL of a home viola makes outside the profile reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` on `windows-2025`, read back through `icacls /save`: canonical SDDL spells the runner's user, the local Administrator (RID 500), `LA`, so an oracle matching its SID string fails (as measured at ci#40400887151).
@carried n=18 run=2 label=architecture/occupied-resources fact=fct-znlazsvj
quote: so the plain-home case reads the grant by its right (inherited, a `WRITE_RIGHTS` bit, a SID `trusted_sid` does not trust; `strict::win::tests::inherited_untrusted_writes`), with a control that cannot pass vacuously (as measured at ci#40439188053).
@carried n=18 run=3 label=architecture fact=fct-4whhx2hw
quote: Test-only feature: `viola-channel` `test-support = []` gates `pub mod test_support` (the `JsonFields` tracing field visitor; on Windows `canonical_sddl`, `dacl_of` and an independent `user_sid`).
@binding n=1
**res-repository e2e-home keepers ↔ the chunk's msrv acceptance and the body's dev-host backing**: the two backing cases must decide on a free-space figure the test hands the keeper's check, never the host's `statvfs`, while `HEADROOM_FLOOR_MIB` stays one compiled constant set by no variable, flag or config key and the message `e2e-home backing: {free} MiB free, below the {floor} MiB floor` keeps its form (architecture/occupied-resources fct-xib5fpty) (architecture/occupied-resources fct-5pbk7sp3) (architecture fct-cny66nid)
@binding n=2
**res-filesystem per-OS twins ↔ crosscut-os-gated-tests**: the Frozen guard, `SOCKET_ROOT_VAR`, harness `endpoint_gone` and the regular-file per-OS oracle keep their Unix arm ungated, the census `tests/contract_os_gated_tests.rs` follows the gates removed, and orphans-check's `cfg(windows)`/`cfg(unix)` file-pair control must still fire (architecture/occupied-resources fct-w6vpbrjt) (architecture/occupied-resources fct-5v4ir6ih) (architecture fct-fa2klbw5) (architecture fct-eesxehzs)
@binding n=3
**res-filesystem strict-modes rows ↔ conv-exit-codes**: the strict-modes read and write rules lose their Windows SID / DACL halves while every Unix refusal keeps its exit (verify 1 with the home pair, frame verbs and `viola list` 21) and its `strict-modes-failed` record (architecture/occupied-resources fct-isxdwj6d) (architecture/occupied-resources fct-jznqbn54) (architecture fct-jdhp6aqu)
@binding n=4
**res-repository ConPTY rows ↔ dec-pty**: the companions, `seed_conpty`, `target/conpty-seed/` and `vendor/conpty/` leave with the ConPTY sideload, while the portable-pty pin and the child-exit-on-handle rule stay for Linux (architecture/occupied-resources fct-snmldjr6) (architecture fct-omwrdnlc)
@binding n=5
**res-repository per-OS uploads and junit-os-sets ↔ dec-ci-cd and Inherited Defaults' CI row**: the `harness-<os>`, `junit-<os>` and `secret-scan-<os>` uploads and `scripts/junit-os-sets.py` follow the matrix shrunk to ubuntu-latest (architecture/occupied-resources fct-fs7kfa27) (architecture/occupied-resources fct-jyuckxyp) (architecture fct-bvzxxcy7) (architecture fct-f53f3qjo)
@binding n=6
**Windows-only occupied-resources rows ↔ the masters' expected amendments**: every row this group carries that names Windows or macOS code the removal deletes is amended by the wrap, and the plan names each as an expected amendment rather than editing the master (architecture/occupied-resources fct-ymvbuev2) (architecture/occupied-resources fct-xtqblyzi) (architecture fct-igbvtfv6)
