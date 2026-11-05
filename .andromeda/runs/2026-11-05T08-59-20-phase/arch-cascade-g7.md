@carried n=1 label=architecture/occupied-resources fact=fct-nm6raycf
why: The socket dir's `$TMPDIR/viola/` macOS arm leaves with the removal, while every Linux socket rule (per-user 0700 dir, the dir check and `socket-dir-refused`, the 0600 socket, the lock arbiter) must come out of it unchanged.
quote: Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (security-plan arch amendment 2): `$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, and `/tmp/viola-<uid>/` only as a fallback; never the `/tmp` or `$TMPDIR` root. `run` and `revive`'s start check that directory before any snapshot, child or `Server::bind`, creating it only when absent and checking it again, and the CLI frame verbs, `hook` and `mcp`'s frame tools check the recorded `endpoint`'s directory before they connect; a symlink, a non-directory, a foreign owner or any group or other bit is refused (`socket-dir-refused`) and left as found, the check never changing its mode. The socket is chmod 0600 after the bind.
@carried n=1 run=2 label=architecture fact=fct-wular7qm
quote: both sides admit only a peer of the owning user's euid, the wrapper dropping another before any frame. Linux abstract-namespace names are never used, because they carry no filesystem permissions and any local user could connect.
@carried n=1 run=3 label=architecture fact=fct-vjccfh2l
quote: **Windows:** named pipe `\\.\pipe\viola-<h12>`, one per `viola run`.
@carried n=1 run=4 label=architecture fact=fct-s2vyyjop
quote: named pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` (Windows), created by `tests/channel_sqos_open.rs`, and endpoints named `viola-test-chan-<pid>-<label>` (a named pipe on Windows, a socket of that base name elsewhere)
@carried n=2 label=architecture/occupied-resources fact=fct-uqyfijk4
why: The server check's `on Unix` euid step becomes unconditional and the macOS peer-pid carve-out loses its code, while the Linux pid and start-time comparison and the no-frame refusal (exit 21, MCP `instance-unreachable`, hook failing open) stay as they are.
quote: and then, on Unix, the connected peer's euid (`viola_channel::peer_admitted` over `Client::server_euid` / `TokioClient::server_euid`), a foreign euid or one it cannot read reading `server-verify-failed`. A mismatch or a check that cannot be made sends no frame: CLI exit 21 `instance-unreachable`, MCP `instance-unreachable`, the hook failing open. macOS reports no peer pid and stays on the recorded gap, owned by no route entry: there the pid and start-time comparison is absent and `server-verify-failed` means an euid mismatch alone.
@carried n=2 run=2 label=architecture/occupied-resources fact=fct-scvtx5bh
quote: or the serving process is not the recorded wrapper or, on Unix, runs under another euid, 20 for a wrapper error
@carried n=2 run=3 label=architecture/occupied-resources fact=fct-l6ysfimx
quote: They exit 21 the same way, past the instance check, on a refused socket directory (`--json`: the `instance-unreachable` document with `detail` `socket-dir-refused`; human: `send_hint` cause `socket-dir`, `hint:` last), and with `detail` `server-verify-failed` on a server of another euid.
@carried n=2 run=4 label=architecture/occupied-resources fact=fct-2hatag6l
quote: A test seam, not a configuration channel: it disables no control and widens no redaction.
@carried n=3 label=architecture/occupied-resources fact=fct-bzckmn3n
why: The harness's `{sh,ps1}` pair is named here, so the plan's decision on `scripts/agent-run.ps1` must leave this row, and the harness's per-OS upload names and Windows relink rationale beside it, naming only what still exists.
quote: `viola-harness` (in `viola-e2e`, behind `scripts/agent-run.{sh,ps1}`; its command contract is test-plan §3)
@carried n=3 run=2 label=architecture fact=fct-6edq3mfh
quote: `target/agent-run/<session>/`, `target/agent-run/chunk.diff` and `target/agent-run/artifacts/`: written by `viola-harness`, gitignored. `chunk.diff` is the one file `secret-scan` skips, by exact path, and the `harness-<os>` upload excludes it.
@carried n=3 run=3 label=architecture fact=fct-ziwerfk3
quote: The `junit-<os>` upload (behind the secret scan) carries it alone.
@carried n=3 run=4 label=architecture fact=fct-io6ijhtt
quote: CI uploads it as artifact `secret-scan-<os>` (7 days) only when the scan fails.
@carried n=3 run=5 label=architecture fact=fct-5q3bjmyd
quote: `target/harness/`: the harness's own cargo target dir (`CARGO_TARGET_DIR` for its builds and test runs), because a running `target/debug/viola-harness.exe` cannot be relinked on Windows.
@carried n=3 run=6 label=architecture fact=fct-eesxehzs
quote: `target/orphans-probes/run-<utc>-<pid>/`: the throwaway crates `scripts/orphans-check.sh --probe` writes per run, each with an empty `[workspace]`: a planted `src/stray.rs`, a file gated on a target no CI triple compiles, and a `cfg(windows)`/`cfg(unix)` file-pair control (verdict `orphans-check probes: 2/2 fired, control clean`), gitignored.
@carried n=4 label=architecture/occupied-resources fact=fct-mnor3ks3
why: Each verb's exit codes lie in the typed set the chunk re-asserts, so every one must come out of the removal unchanged on Linux.
quote: it exits 0 with a board (one stdout write), 21 when its strict-modes check refuses the home, `instances/` or any one instance (the whole board refused), 1 for an internal error through the `cli` catch site and 2 for a clap usage error
@carried n=4 run=2 label=architecture fact=fct-qslc37ld
quote: Exit codes `0, 1, 2, 10–14, 20, 21` as defined in Conventions.
@carried n=5 label=architecture/occupied-resources fact=fct-yoyjutua
why: `hook`'s always-exit-0, empty-stderr contract is re-asserted by the scope, so its capture arms and its missing-instance path must keep it after the Unix gates go, and the Windows-drive rationale of the `--answers` unit case loses its subject.
quote: no `VIOLA_*` read, no obs init, no channel; always exit 0 with empty stderr.
@carried n=5 run=2 label=architecture/occupied-resources fact=fct-2oafusjr
quote: a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no session, so `viola hook` exits 0 and writes nothing.
@carried n=5 run=3 label=architecture fact=fct-4u2vf5lk
quote: It sits under the current dir, not the OS temp dir, because a temp dir on another Windows drive (`windows-2025`: the checkout on `D:`, the temp dir on `C:`) has no relative path from the current dir.
@carried n=6 label=architecture/occupied-resources fact=fct-ehpssyzz
why: The settings override and the `--settings` flags of `run` and verify's Run B are Unix-only today, so their gates become unconditional with no change in Linux behaviour, and the Windows override's owner, the retired entry, is re-pointed by the expected amendments.
quote: It is written only on Unix, and only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :` (`viola_agent_claude::statusline::override_document`); for any other path, and on Windows, no override is written and no `--settings` is passed. That the child takes a `statusLine` given through `--settings` is the ledger row `statusline-override` (probed by Run B's probe `--settings`, relied on Unix only, since chunk 2026-10-13-owed-ledger-rows; §Cross-cutting Patterns, Capability ledger). A code-bearing file written at every start and handed to the CLI is a boundary widening the founder ratified (live, 2026-10-10, relayed verbatim by the operator). The Windows override is owed to the working-route entry "Windows-only live measurements".
@carried n=6 run=2 label=architecture/occupied-resources fact=fct-p6lni4fy
quote: then `--settings <instance dir>/settings.json` when the start wrote a settings override (none on Windows, and none for a pinned path the override rule refuses; see the next bullet); then the caller's arguments.
@carried n=6 run=3 label=architecture/occupied-resources fact=fct-nbf2wdvs
quote: Run B with cwd `<cwd>/.viola-verify-<pid>/` and, on Unix for shell-plain paths, `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair (no flag on Windows);
@carried n=6 run=4 label=architecture fact=fct-klpb7auf
quote: written only on Unix, and only for a pinned path of plain characters, else no override is written and no `--settings` is passed.
@carried n=7 label=architecture/occupied-resources fact=fct-lb4pv4em
why: The Windows registry reader and the case-insensitive match are Windows arms that leave, while the Unix `claude_env_keep` pass-list, the identity floor and the names-only rule stay.
quote: every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) except a name the user defines persistently — on Windows a value NAME under `HKCU\Environment` or `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment` (read through windows-sys Registry, names only), on Unix a name in `config.json` `claude_env_keep`.
@carried n=7 run=2 label=architecture/occupied-resources fact=fct-3jytrhfq
quote: `claude_env_keep`: the Unix R8 pass-list, an array of at most 32 names each matching `^CLAUDE[A-Z0-9_]*$`, default empty — a bad value is rejected whole, and an identity-floor name in it has no effect;
@carried n=7 run=3 label=architecture fact=fct-i4wacyrp
quote: `claude_env_keep` (the Unix R8 pass-list)
@carried n=7 run=4 label=architecture fact=fct-jujxlicd
quote: Names only: the strip, the registry reader and the logs never read a value.
@carried n=8 label=architecture/occupied-resources fact=fct-5lqu6sil
why: `windows-mutants.yml` leaves whole, so this row and the Windows mutation scratch it serves lose their subject, and the dev host's tmpfs e2e home stays the one the backing cases' free-space floor guards.
quote: No CI workflow sets it: `ci.yml` and `nightly.yml` run no mutation job, and the dispatch-only `windows-mutants.yml` runs the `--package <member> --file …` arm, which reads no base.
@carried n=8 run=2 label=architecture fact=fct-7wuxnmfc
quote: `<repo parent>/viola-mutants-scratch/` (outside the repository, a sibling of it): the host mutation scratch, where on a Windows host `run --mutants` puts cargo-mutants' temp copies and its `--output`; a guard refuses any other final component or a repository inside it, never a fallback to `%TEMP%` or the repo, and its path is never printed. Off Windows the harness arm is inert.
@carried n=8 run=3 label=architecture fact=fct-cny66nid
quote: Test-only, outside the repository, on the Linux dev host only: `/tmp/viola-e2e-home-<uid>/` (mode 0700, owned by the user), the tmpfs directory the dev host's `target/e2e-home` link points at, holding every test and session home of that host. `viola` never reads or writes it by that name. No CI runner has it.
@carried n=9 label=architecture/occupied-resources fact=fct-6u55z4cp
why: The pinned copy's `.exe` naming and its Unix-gated 0700 mode are touched by the removal, and its `bin/<version>-<hash>/` tree's ConPTY companions, the `vendor/conpty/` binaries, the `conpty-seed` copy and the `seed_conpty` carve-out leave with the Windows code.
quote: `bin/<version>-<hash>/viola(.exe)`, the pinned copy (0700 on Unix):
@carried n=9 run=2 label=architecture fact=fct-lvi3drxp
quote: Binary: `viola` (`viola.exe`).
@carried n=9 run=3 label=architecture fact=fct-hnee7xdr
quote: `bin/<version>-<hash>/viola(.exe)`: the pinned copy of the binary, which `viola run` places once when it is absent, with no clobber, and never replaces (0700 on Unix)
@carried n=9 run=4 label=architecture fact=fct-ekftm72e
quote: `bin/<version>-<hash>/conpty/OpenConsole.exe` and `bin/<version>-<hash>/conpty/conpty.dll` (Windows x64 only): Microsoft's ConPTY companions embedded in the binary, written by `viola run` if absent (`pin_companions` through `replace_private_shared`) and never overwritten; at every start `bin/<version>-<hash>/conpty/` and each companion that exists first pass the strict-modes check (owner and DACL), then each is held with `FILE_SHARE_READ` only and fully re-hashed against its compiled SHA-256 pin until the spawn. A refused check or a mismatch writes nothing, leaves the file as found, never loads it, never refuses the start, and runs the child on the inbox ConPTY (`sideload_fallback` `strict-modes-failed` or `hash-mismatch`). v1 never deletes them.
@carried n=9 run=5 label=architecture fact=fct-rryrskwd
quote: `vendor/conpty/<version>/x64/{conpty.dll,OpenConsole.exe}` (today `1.24.260710001`): the committed Microsoft binaries from the nuget package `Microsoft.Windows.Console.ConPTY` (MIT), byte-identical to the pinned nupkg, `.gitattributes` `vendor/conpty/** binary`, embedded by `src/conpty.rs`. `scripts/conpty-vendor.sh` is their one writer (the default re-vendor) and their verifier (`--verify`: a fetch outside the tree, the nupkg SHA-256 before extraction, a byte compare and the Authenticode signer; `--probe`: its four refusals and a control). Its pins are parsed from `src/conpty.rs`'s text.
@carried n=9 run=6 label=architecture fact=fct-qe46hlhe
quote: `target/conpty-seed/<key>/` (Windows x64): the one per-run copy of the ConPTY companions that root `tests/support/home.rs` `seed_conpty` hard-links (copy fallback) into a test home's `bin/<key>/conpty/` before `Wrapper::boot` and the piped and outer-PTY `viola run` starts (never in `run_viola_unseeded` starts or `tests/conpty_sideload.rs`, which keep the product's write path). Regenerated on demand, gitignored; `viola` never reads it.
@carried n=9 run=7 label=architecture fact=fct-swegdoif
quote: It is the second named carve-out to the test-data rule, beside `seed_conpty` (the founder's ruling, 2026-10-04).
@carried n=10 label=architecture/occupied-resources fact=fct-kekl3p46
why: Every strict-modes rule has a Unix half and a Windows half: the Windows halves, their `cfg(windows)` proving cases and the test-support Windows helpers leave, and the Unix halves lose their gate with no change in what Linux refuses.
quote: The read rule is the write rule plus, on Unix, a refusal of any group or other bit (`mode & 0o077`) and, on Windows, a refusal of any allow ACE, an inherit-only one included, that grants a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects (`unix_read_verdict`, `windows_read_verdict`, `check_read`).
@carried n=10 run=2 label=architecture/occupied-resources fact=fct-hn66ngvt
quote: The check is proven on Unix (five widened modes). On Windows `cfg(windows)` cases prove it: an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and a plain home made before viola is refused for its inherited write grant to a SID `trusted_sid` does not trust, read by its right whichever SID holds it, while a home `instance_tree()` makes reads none (as measured at ci#40439188053, `windows-2025`).
@carried n=10 run=3 label=architecture/occupied-resources fact=fct-wdtionxy
quote: a symbolic link is refused, a folder takes the read rule, a file takes the read rule on Unix and the write rule on Windows, and an absent path passes.
@carried n=10 run=4 label=architecture fact=fct-cicfj3bq
quote: the home, the instance directory and its `diagnostics/` to the read rule (Unix no group or other bit; Windows no read right granted to another SID), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds; a refused check writes no line.
@carried n=10 run=5 label=architecture fact=fct-5jecrew3
quote: the folder to the read rule, the file to the read rule on Unix and the write rule on Windows, an absent path passing;
@carried n=10 run=6 label=architecture fact=fct-4whhx2hw
quote: `viola-channel` `test-support = []` gates `pub mod test_support` (the `JsonFields` tracing field visitor; on Windows `canonical_sddl`, `dacl_of` and an independent `user_sid`).
@carried n=11 label=architecture/occupied-resources fact=fct-6oxorhgk
why: The diagnostics dir and file modes are Unix-gated, so the gate goes with their Linux modes kept, and the closed `socket-dir-refused` and `server-verify-failed` words of the line schema must stay unchanged on Linux.
quote: One line per `write`; dir 0700, files 0600 on Unix; line format owned by obs and fixed by `schemas/diag-line.v1.json`.
@carried n=12 label=architecture/occupied-resources fact=fct-if66544y
why: `batch-script-child` names the Windows `.cmd`/`.bat` refusal, so the plan must list it as a closed word the removal may leave naming no code, while every closed enum stays unchanged on Linux.
quote: `verify`'s refusal codes: the `process-exit` `verify` writes under a parsed `VIOLA_NAME` carries its `Ended::Refused` code (`src/cmd/verify.rs`) in `detail`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` or `unclean-fixture`, beside `interrupted`.
@carried n=12 run=2 label=architecture/occupied-resources fact=fct-gftfjtay
quote: `channel-response`'s `detail` in `schemas/diag-line.v1.json` is the closed enum `unsupported-version` · `invalid-params` · `release-from-driver`,
@binding n=1
**IPC endpoints ↔ Local endpoint trust boundary**: the Windows named pipe and the test-only SQOS pipe leave with the Windows code, while the Linux socket file, its 0700 dir check and the euid admission stay and become ungated, so no other OS user can connect and no env, flag or feature seam forces a refusing peer-euid decision (architecture/occupied-resources fct-nm6raycf) (architecture/occupied-resources fct-uqyfijk4) (architecture/occupied-resources fct-2hatag6l) (architecture fct-ao5mmzh3) (architecture fct-w2igrcse)
@binding n=2
**Strict-modes rules ↔ Cross-platform discipline's OS-gated census**: the Windows halves of the read, write and diagnostics rules leave with their `cfg(windows)` proving cases and the test-support Windows helpers, and the census binary `tests/contract_os_gated_tests.rs` must be re-pointed or retired to count only the gates that remain (architecture/occupied-resources fct-kekl3p46) (architecture/occupied-resources fct-hn66ngvt) (architecture fct-fa2klbw5)
@binding n=3
**Settings override ↔ the masters' OS-target amendments**: the override's Unix-only gate and its Windows leg owed to the retired entry "Windows-only live measurements" are named by the plan as expected amendments, and the override stays as written on Linux (architecture/occupied-resources fct-ehpssyzz) (architecture fct-4nt6qtyp) (architecture fct-igbvtfv6)
@binding n=4
**ConPTY companions, vendor tree and seed ↔ the PTY decision**: the companions, `vendor/conpty/`, `scripts/conpty-vendor.sh`, `target/conpty-seed` and `seed_conpty` leave with the sideload, while the portable-pty pin and child-exit-on-handle rule stay for Linux (architecture fct-ekftm72e) (architecture fct-rryrskwd) (architecture fct-qe46hlhe) (architecture fct-omwrdnlc)
@binding n=5
**R8 strip ↔ Agent Coverage decision**: the registry reader leaves the agent's strip, while `claude_env_keep`, the identity floor and the names-only rule stay inside `viola-agent-claude`'s pure boundary (architecture/occupied-resources fct-lb4pv4em) (architecture/occupied-resources fct-3jytrhfq) (architecture fct-7tdo3mcl)
@binding n=6
**Harness uploads, mutation scratch and orphans probe ↔ the CI/CD decision**: the shrunk Linux-only CI keeps the harness's upload names and the orphans-check probe consistent with what runs, the Windows mutation arm and `windows-mutants.yml` leave together, and the probe's `cfg(windows)`/`cfg(unix)` control must still fire with any Windows-gated file left behind tripping it (architecture/occupied-resources fct-bzckmn3n) (architecture/occupied-resources fct-5lqu6sil) (architecture fct-7wuxnmfc) (architecture fct-eesxehzs) (architecture fct-bvzxxcy7)
@binding n=7
**Per-verb exits and hook's capture arms ↔ the Conventions exit codes**: every verb's exits stay inside `0, 1, 2, 10–14, 20, 21` and `viola hook` always exits 0 with no body and no stderr after the removal (architecture/occupied-resources fct-mnor3ks3) (architecture/occupied-resources fct-yoyjutua) (architecture fct-jdhp6aqu) (architecture fct-nppkdemz)
@binding n=8
**Dev-host tmpfs e2e home ↔ the e2e-home backing cases**: the backing cases' free-space floor guards the dev host's `/tmp/viola-e2e-home-<uid>/`, which no CI runner has, so the cases must decide on a free-space figure the test controls, never the host's (architecture fct-cny66nid) (architecture fct-e4sufzw6)
