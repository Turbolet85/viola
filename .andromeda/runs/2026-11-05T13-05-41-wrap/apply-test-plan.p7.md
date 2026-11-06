@apply_row n=1 ref_file=fanout-test-plan.md ref_n=54 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan.md ref_n=56 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan.md ref_n=57 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan.md ref_n=58 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan.md ref_n=59 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan.md ref_n=60 v=applied edit=E6
@apply_row n=7 ref_file=fanout-test-plan.md ref_n=61 v=applied edit=E7
@apply_row n=8 ref_file=fanout-test-plan.md ref_n=62 v=applied edit=E8
@apply_row n=9 ref_file=fanout-test-plan.md ref_n=63 v=applied edit=E9
@apply_row n=10 ref_file=fanout-test-plan.md ref_n=64 v=refused
why: a hand removal · fct-uiwmcfut — its one list item «Windows x64, root tests only: `tests/support/home.rs` `seed_conpty(home)` …» is a whole line an in-line change cannot empty, and the batch refuses moving the next item up (`reshaped · the structural prefix changed`); the in-line rest rides with that hand edit: «the §5 second carve-out» → «the §5 carve-out», «; on Windows it is the plain create» dropped, «unit-tested on every OS» → «unit-tested»
@fact id=fct-gvvrytf4 expect_rev=1
old: Linux host only, forwarded by both shims, not
new: Linux host only, forwarded by the shim, not
old: harness tests. The other OSes are CI's alone.
new: harness tests.

@fact id=fct-hgewdmj2 expect_rev=2
old: reads 32 sites in 24 files under
new: reads 31 sites in 23 files under
old: as measured at chunk 2026-11-04-linux-and-macos-parity's wrap on the Linux dev host;
new: as measured at chunk 2026-11-05-linux-only-code-and-ci's wrap on the Linux dev host;
old: rule — on Windows a client connect reads NotFound, on Unix a connect to the path is refused or the path is absent, what `holder_gone` reads there)
new: rule — a connect to the path is refused or the path is absent, what `holder_gone` reads)
old: in place of `wait_endpoint_gone`: on Unix a connect to the path is refused or the path is absent, on Windows the pipe is not found.
new: in place of `wait_endpoint_gone`: a connect to the path is refused or the path is absent.
old: An exit code is not the endpoint gone: on Windows the stopped wrapper's own pipe took a hook's connect and write 25 ms after its `process-exit` line, as measured at ci#36532038635; why the pipe outlives the observed exit is recorded, not established. Each wait
new: An exit code is not the endpoint gone. Each wait
old: beside `--in-diff`. On a Windows host (`HOST_SCRATCH`) the run first resolves the host mutation scratch `<repo parent>/viola-mutants-scratch` from the repository's absolute path and refuses unless its final component is exactly `viola-mutants-scratch` and the repository is neither it nor inside it (`reason:"scratch-refused"`, exit 1, never a fallback to `%TEMP%` or the repository). It then measures the scratch's bytes and removes and recreates it (`reason:"scratch-wipe-failed"` when the removal fails, a leaked process holding a file), the re-creation through `viola_state::fs::create_private_dir`: the scratch takes the protected user + SYSTEM DACL, so every temp copy and test home laid inside it inherits owner-only, as one under `%TEMP%` does (a plain re-create on the runner's workspace drive inherited BUILTIN\Users write, and the root package's baseline failed the instance check there, run 38215104120).
new: beside `--in-diff`.
old: The `#[cfg(windows)]` unit `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` pins it on CI's `test (windows-2025)` leg, with no dispatch: an instance tree inside a wiped scratch passes `check_instance`, the same tree in a plain-created sibling granted the Users group's write explicitly (`icacls <plain> /grant *S-1-5-32-545:(OI)(CI)(W)`, its exit asserted) reads `Writable` whatever the sibling's parent grants (as measured at ci#38375200217, chunk 2026-10-13-epoch-4-cleanup); whether the root package's Windows baseline then reads green is not yet measured (the next `windows-mutants.yml` dispatch). The command runs with `TMP`/`TEMP` = the scratch and `--output <scratch>`: cargo-mutants' temp copy follows `TMP`/`TEMP` through `std::env::temp_dir()` (as measured at chunk 2026-09-27-epoch-2-cleanup: the copy appeared in the scratch mid-run, none in `%TEMP%`), so no copy lands on C: and no `mutants.out/` in the repository. Off Windows nothing changes: the inherited temp dir and `mutants.out/` at the repository root.
new: The run keeps the inherited temp dir and writes `mutants.out/` at the repository root.
old: output dir (on a Windows host `<scratch>/mutants.out/`, the whole scratch wiped first), so only
new: output dir, so only
old: `mutants.out/outcomes.json` (`<scratch>/mutants.out/` on a Windows host) and read
new: `mutants.out/outcomes.json` and read
old: `AGENT_RUN_KEEP_FAILED=0` (the host scratch arm unchanged, inert off Windows). A stale
new: `AGENT_RUN_KEEP_FAILED=0`. A stale

@fact id=fct-zxaumenb expect_rev=1
old: viola then creates the home itself: 0700 on Unix, and on Windows the explicit protected user + SYSTEM DACL that security requires for a `--home` outside `%USERPROFILE%` (a hosted runner's workspace is outside it). A home pre-created by the harness would inherit the workspace ACL and fail the Windows strict-modes check. The base
new: viola then creates the home itself, 0700. The base
old: Test data bootstrap); on Windows it is the plain create. A refused
new: Test data bootstrap). A refused
old: (`LedgerRow::relied()`: all twenty-four on Unix, elsewhere twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`)
new: (`LedgerRow::relied()`: all twenty-four)

@fact id=fct-tuv4h7zt expect_rev=1
old: `[&str; 24]` under `cfg(unix)` — the six
new: `[&str; 24]` — the six
old: `local-command-compact` — and the 21-id form (the first eighteen, `resume-session`, `agents-join` and `local-command-compact`) under `cfg(not(unix))`, spelled so, not `cfg(windows)`, so the harness stays outside the Windows mutation scope) names
new: `local-command-compact`) names
old: belongs to "Windows-only live measurements", blocked on an interactive Windows host)
new: belongs to the open residuals of origin `2026-11-04T23-11-43-wrap`, target `another-os`)
old: unit-proven per OS (`harness::run::tests`: `LIVE_PASS` under the same split, and
new: unit-proven (`harness::run::tests`: `LIVE_PASS`, and

@fact id=fct-zujg4opw expect_rev=1
old: (the run's own output dir, `<scratch>/mutants.out/` on a Windows host; `survived`
new: (the run's own output dir; `survived`
old: On a Windows host a counted, scoped or package object also carries `"scratch_bytes"`, the bytes the host scratch held before its wipe (a count, never the path). A counted
new: A counted

@fact id=fct-47aec6c3 expect_rev=1
old: `mutants.out/outcomes.json`, under the host scratch on a Windows host)
new: `mutants.out/outcomes.json`)

@fact id=fct-beydn7zm expect_rev=1
old: must be gone: on Windows a `viola-channel` client connect fails with NotFound, and on Unix a connect to the
new: must be gone: a connect to the
old: (`$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, `/tmp/viola-<uid>/` as the fallback;
new: (`$XDG_RUNTIME_DIR/viola/`, `/tmp/viola-<uid>/` as the fallback;

@fact id=fct-5z3qj22n expect_rev=1
old: "mode_check":"unix"|"skipped-windows","hits"
new: "mode_check":"unix","hits"

@fact id=fct-mihqkrxe expect_rev=1
old: `corpus-empty`; by the Epoch 2 cleanup chunk: `scratch-refused`, `scratch-wipe-failed` (the Windows-host mutation scratch); by the verify
new: `corpus-empty`; by the verify
old: Usage `detail`: `unknown-suite`.
new: Retired with `mutants/scratch.rs` by the linux-only-code-and-ci chunk: `scratch-refused`, `scratch-wipe-failed`. Usage `detail`: `unknown-suite`.
@pattern id=tp7-01
regex: forwarded by both shims|The other OSes are CI's alone
@pattern id=tp7-02
regex: 32 sites in 24 files|on Windows a client connect reads NotFound|on Windows the pipe is not found|stopped wrapper's own pipe took a hook's connect|resolves the host mutation scratch|host scratch arm unchanged, inert off Windows|on a Windows host `<scratch>/mutants\.out/`|`<scratch>/mutants\.out/` on a Windows host
@pattern id=tp7-03
regex: elsewhere twenty-one|on Windows the explicit protected user \+ SYSTEM DACL|fail the Windows strict-modes check|on Windows it is the plain create
@pattern id=tp7-04
regex: the 21-id form|unit-proven per OS|belongs to "Windows-only live measurements"
@pattern id=tp7-05 ignore_case=true
regex: on a Windows host a counted, scoped or package object also carries|`<scratch>/mutants\.out/` on a Windows host
@pattern id=tp7-06
fixed: under the host scratch on a Windows host
@pattern id=tp7-07
regex: client connect fails with NotFound|`\$TMPDIR/viola/` on macOS
@pattern id=tp7-08
fixed: "skipped-windows"
@pattern id=tp7-09
fixed: `scratch-refused`, `scratch-wipe-failed` (the Windows-host mutation scratch)
