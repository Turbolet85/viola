@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=72
last: - 2026-10-20: `target/debug/viola` is a hard link cargo re-p
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=22 class=stale
leaf-text: `scripts/agent-run.sh` and `scripts/agent-run.ps1` are identical thin shims:
source: fct-yqn2bvnu «**Harness implementation:** `scripts/agent-run.sh` (POSIX) is the one thin shim.»
replace-with: `scripts/agent-run.sh` (POSIX) is the one thin shim:
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=23 class=stale
leaf-text: forwarded unchanged by both shims
source: fct-gvvrytf4 «- **`pre-push`** (the operator pass's local Linux gate; Linux host only, forwarded by the shim, not an agent command»
replace-with: forwarded unchanged by the shim
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=24 class=stale
leaf-text: Its stages are `tools → linux-tests`
source: fct-gvvrytf4 «Stages, fail-fast in order, the document naming the one it reached: `host` (off Linux → exit 2 `pre-push-linux-only`) · `tools`»
replace-with: Its stages are `host → tools → linux-tests`
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=28 class=retired
leaf-text: copy the fake agent to `target/agent-run/<session>/bin/claude[.exe]`
source: fct-zxaumenb «Copy `<bin dir>/viola-fake-agent` (see step 5) to `target/agent-run/<session>/bin/claude`»
replace-with: copy the fake agent to `target/agent-run/<session>/bin/claude`
@leaf_edit n=5 leaf=.claude/rules/verification-harness.md line=28 class=stale
leaf-text: twenty-four on Unix, elsewhere twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`
source: fct-zxaumenb «stamp every relied ledger row (`LedgerRow::relied()`: all twenty-four)»
replace-with: all twenty-four
@leaf_edit n=6 leaf=.claude/rules/verification-harness.md line=28 class=retired
leaf-text: viola-fake-agent[.exe] statusline-echo
source: fct-zxaumenb «It configures the user statusline command as `<bin dir>/viola-fake-agent statusline-echo <home>/fake/statusline.marker`»
replace-with: viola-fake-agent statusline-echo
@leaf_edit n=7 leaf=.claude/rules/verification-harness.md line=28 class=retired
leaf-text: beside `bin/claude[.exe]`
source: fct-zxaumenb «writes the fake agent's `agents-mode` file beside the session's `bin/claude` copy»
replace-with: beside `bin/claude`
@leaf_edit n=8 leaf=.claude/rules/verification-harness.md line=34 class=retired
leaf-text: on Unix a connect to the socket path refused or the path absent, on Windows a client connect reading NotFound
source: fct-beydn7zm «The endpoint recorded in the snapshot must be gone: a connect to the `<per-user 0700 dir>/viola-<h12>.sock` path is refused or the path is absent»
replace-with: a connect to the socket path refused or the path absent
@leaf_edit n=9 leaf=.claude/rules/verification-harness.md line=41 class=retired
leaf-text: , which `macos-latest`'s `ps` cannot print.
source: fct-nwgyqyjs «(`ignores_sighup`), a failed read carrying `ps`'s exit and both streams; it runs on CI's one leg, `ubuntu-latest` (§7 Fake agent).»
replace-with: ; it runs on CI's one leg, `ubuntu-latest`.
@leaf_edit n=10 leaf=.claude/rules/verification-harness.md line=42 class=retired
leaf-text: on macOS `/usr/bin/sample <pid> 1`'s stack lines, on Linux `/proc/<pid>/wchan`
source: fct-qmrx4mno «That state is the wrapper's last `run-<name>.ndjson` event, its `/proc/<pid>/wchan` and the `stat` state field (`note_stall`;»
replace-with: its `/proc/<pid>/wchan`
@leaf_edit n=11 leaf=.claude/rules/verification-harness.md line=43 class=retired
leaf-text: On a Windows host (`HOST_SCRATCH`) the run first resolves the host mutation scratch `<repo parent>/viola-mutants-scratch`, refuses unless its final component is exactly that name and the repo is neither it nor inside it (`scratch-refused`, no fallback), measures, removes and recreates it (`scratch-wipe-failed`) through `viola_state::fs::create_private_dir`, so the scratch takes the protected user + SYSTEM DACL and every temp copy and test home inside it inherits owner-only (a plain re-create inherited the workspace drive's Users write grant and the root package's baseline failed the instance check there; the `#[cfg(windows)]` unit `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` pins it), and runs `cargo mutants` with `TMP`/`TEMP` and `--output` set to it; the document gains `scratch_bytes`.
source: fct-mihqkrxe «Retired with `mutants/scratch.rs` by the linux-only-code-and-ci chunk: `scratch-refused`, `scratch-wipe-failed`.»
replace-with: The run sets no scratch dir of its own and passes no `--output` (`mutants.out/` at the repository root); on the Linux dev host the operator sets `TMPDIR` to a NOCOW btrfs dir.
@leaf_edit n=12 leaf=.claude/rules/verification-harness.md line=43 class=retired
leaf-text: , whose Windows leg is the dispatch-only, report-only `windows-mutants.yml` (`run --mutants --package <member> --file …` per matrix item on `windows-2025`, ten `mutants (<label>)` jobs with the root package split by file into five, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`; `tests/contract_windows_mutation_scope.rs` keeps its file lists equal to the Windows-gated sources and its labels distinct)
source: fct-ocdr3xdn «The boundary audit has no Windows leg: `windows-mutants.yml` and the `cfg(windows)` code it measured on `windows-2025` are out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: ; the boundary audit has no Windows leg (`windows-mutants.yml` is out of the tree)
@leaf_edit n=13 leaf=.claude/rules/verification-harness.md line=43 class=stale
leaf-text: Its two real-cargo-mutants self-tests are compiled out on macOS.
source: fct-ocdr3xdn «The two real-cargo-mutants harness tests (`run_mutants_passes_when_the_change_is_tested`, `run_mutants_reports_survivors_of_an_untested_change`) run ungated, never `#[ignore]`;»
replace-with: Its two real-cargo-mutants self-tests run ungated, never `#[ignore]`.
