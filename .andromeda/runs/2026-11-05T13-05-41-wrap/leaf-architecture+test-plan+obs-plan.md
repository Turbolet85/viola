@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=5 class=omission
leaf-text: has left 0.1.0.
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: has left 0.1.0; the Windows and macOS code, its scripts and its CI jobs are out of the tree, recoverable at the tag `multi-os-final` (`ecbe7ab3`).
@leaf_edit n=2 leaf=.claude/docs/commands.md line=19 class=stale
leaf-text: (Windows the first target; first live on the Linux dev host)
source: fct-igbvtfv6 «**Linux only.** viola targets Linux: the core on any current distribution, built and tested on every commit in CI on `ubuntu-latest` against the fake agent»
replace-with: (Linux only; first live on the Linux dev host)
@leaf_edit n=3 leaf=.claude/docs/commands.md line=21 class=stale
leaf-text: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
source: fct-54jxzo44 «`<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name`»
replace-with: `stale-heartbeat` · `pinned-hash-mismatch`
@leaf_edit n=4 leaf=.claude/docs/commands.md line=22 class=retired
leaf-text: on the Linux dev host; its Windows leg owed to "Windows-only live measurements")
source: fct-23yj6ah3 «the session reaches `idle` and the CLI logs that server's connection as failed (as measured on claude 2.1.287 on the Linux dev host at»
replace-with: on the Linux dev host)
@leaf_edit n=5 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: (on Unix, for shell-plain paths, also one probe `--settings <probe dir>/typed/statusline.json` whose status line the hidden `hook statusline --capture` arm files; none on Windows)
source: fct-qwpklckh «Only for shell-plain paths, it also gets one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair»
replace-with: (for shell-plain paths, also one probe `--settings <probe dir>/typed/statusline.json` whose status line the hidden `hook statusline --capture` arm files)
@leaf_edit n=6 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: check the relied ledger rows (`LedgerRow::relied()`: all twenty-four on Unix; twenty-one on Windows, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`)
source: fct-62qgonvf «A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-four, on every build)»
replace-with: check the relied ledger rows (`LedgerRow::relied()`: all twenty-four, on every build)
@leaf_edit n=7 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: (`MM` the relied count; the last `[24/24] local-command-compact …` on Unix, `[21/21] local-command-compact …` on Windows)
source: fct-qwpklckh «`MM` being `LedgerRow::relied().len()`, 24, the last `[24/24] local-command-compact …`»
replace-with: (`MM` the relied count, 24; the last `[24/24] local-command-compact …`)
@leaf_edit n=8 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: (SIGKILL, a crash or a Windows close leaves the dirs to remove by hand)
source: fct-c3nhxk2n «SIGKILL, a crash and a failed handler install (ignored, fail-open) keep the OS default and leave the dirs to be removed by hand»
replace-with: (SIGKILL, a crash or a failed handler install leaves the dirs to remove by hand)
@leaf_edit n=9 leaf=.claude/docs/commands.md line=24 class=stale
leaf-text: holds every relied row (`LedgerRow::relied()`: all twenty-four on Unix; twenty-one on Windows, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`) `pass`
source: fct-62qgonvf «A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-four, on every build)»
replace-with: holds every relied row (`LedgerRow::relied()`: all twenty-four, on every build) `pass`
@leaf_edit n=10 leaf=.claude/docs/commands.md line=24 class=retired
leaf-text: or a twenty-three-row stamp written before chunk 2026-10-18-compact-post-condition-ledger-row (and so, on Windows, do a stamp of the eighteen and `resume-session` and one of the eighteen, `resume-session` and `agents-join`; one of the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only)
source: fct-62qgonvf «of the twenty-two pre-2026-10-16-unwrapped-sessions-on-the-board ids or of the twenty-three pre-2026-10-18-compact-post-condition-ledger-row ids reads unverified»
replace-with: or a twenty-three-row stamp written before chunk 2026-10-18-compact-post-condition-ledger-row
@leaf_edit n=11 leaf=.claude/docs/commands.md line=26 class=retired
leaf-text: `scripts/agent-run.sh` / `scripts/agent-run.ps1`
source: fct-yqn2bvnu «**Harness implementation:** `scripts/agent-run.sh` (POSIX) is the one thin shim.»
replace-with: `scripts/agent-run.sh`
@leaf_edit n=12 leaf=.claude/docs/commands.md line=27 class=stale
leaf-text: `<bin dir>/viola-fake-agent[.exe] statusline-echo <home>/fake/statusline.marker`
source: fct-zxaumenb «configures the user statusline command as `<bin dir>/viola-fake-agent statusline-echo <home>/fake/statusline.marker`»
replace-with: `<bin dir>/viola-fake-agent statusline-echo <home>/fake/statusline.marker`
@leaf_edit n=13 leaf=.claude/docs/commands.md line=27 class=stale
leaf-text: beside the session's `bin/claude[.exe]` copy
source: fct-zxaumenb «writes the fake agent's `agents-mode` file beside the session's `bin/claude` copy»
replace-with: beside the session's `bin/claude` copy
@leaf_edit n=14 leaf=.claude/docs/commands.md line=32 class=stale
leaf-text: Internal (forwarded by the shims, not agent-facing)
source: fct-gvvrytf4 «the operator pass's local Linux gate; Linux host only, forwarded by the shim, not an agent command»
replace-with: Internal (forwarded by the shim, not agent-facing)
@leaf_edit n=15 leaf=.claude/docs/commands.md line=41 class=retired
leaf-text: On a Windows host the command runs with `TMP`/`TEMP` and `--output` set to the host mutation scratch `<repo parent>/viola-mutants-scratch` (guarded: `scratch-refused`; wiped first: `scratch-wipe-failed`), so `mutants.out/` lives there and the document carries `scratch_bytes`
source: fct-mihqkrxe «Retired with `mutants/scratch.rs` by the linux-only-code-and-ci chunk: `scratch-refused`, `scratch-wipe-failed`.»
replace-with: The run keeps the inherited temp dir and writes `mutants.out/` at the repository root; it has no scratch arm
@leaf_edit n=16 leaf=.claude/docs/commands.md line=44 class=retired
leaf-text: `gh workflow run windows-mutants.yml --ref build/viola-0.1.0` — the boundary audit's Windows leg (founder ruling C2): the dispatch-only, non-blocking `windows-2025` matrix of ten `mutants (<label>)` jobs, one per matrix item (the root package `viola` is five items, split by file, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`), each running `scripts/agent-run.ps1 run --mutants --package <member> --file …` over that item's Windows-gated files (`tests/contract_windows_mutation_scope.rs` keeps the lists complete and the labels distinct). The harness document judges a job: a missed mutant the Windows build never compiled is left out and named in `mutants.host_excluded`, while cargo-mutants' own lines still say MISSED for it. Dispatched only during the epoch-boundary audit, never a gate. Read it back: the run of the pushed sha (`gh run list --workflow windows-mutants.yml --json databaseId,headSha`), `python -X utf8 <andromeda-tools>/scripts/ci.py conclusion --sha <sha> --name mutants` for the per-job verdicts, then each job's harness document and outcome lines via `gh run view --job <id> --log`. No artifact is uploaded; its survivors become corrective-chunk items like any boundary survivor
source: fct-vf6cfvel «The boundary audit has no Windows leg: `windows-mutants.yml` left with the Windows code, recoverable at the tag `multi-os-final`.»
replace-with: no Windows leg in the boundary audit: `windows-mutants.yml` left with the Windows code, recoverable at the tag `multi-os-final`
@leaf_edit n=17 leaf=.claude/docs/commands.md line=49 class=retired
leaf-text: Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints: `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings` (nothing linked, the target std installed as host preparation; neither a CI step nor a `pre-push` stage). Beside it, a check-only macOS-target clippy is a recipe on the Linux dev host: `CARGO_TARGET_DIR=target/maccheck cargo clippy --workspace --all-targets --features fake-agent --target aarch64-apple-darwin -- -D warnings` (nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`; it reproduced CI's `lint (macos-latest)` error line for line; neither a CI step, nor a `pre-push` stage, nor a plan gate entry).
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: No Windows- or macOS-target clippy stands: the tree holds no Windows or macOS code.
@leaf_edit n=18 leaf=.claude/docs/commands.md line=53 class=stale
leaf-text: CI job `lint`, all 3 OS
source: fct-shyrfs6v «`ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs), the one leg of `strategy.matrix.os` in `test`, `perf`, `lint` and `release`»
replace-with: CI job `lint` on `ubuntu-latest`
@leaf_edit n=19 leaf=.claude/docs/commands.md line=55 class=stale
leaf-text: every OS, after `Lint probes`
source: fct-shyrfs6v «`ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs), the one leg of `strategy.matrix.os` in `test`, `perf`, `lint` and `release`»
replace-with: on `ubuntu-latest`, after `Lint probes`
@leaf_edit n=20 leaf=.claude/docs/commands.md line=61 class=retired
leaf-text: `bash scripts/conpty-vendor.sh [--verify|--probe]` — the vendored ConPTY binaries: the default re-vendors `vendor/conpty/<version>/x64/` from the pinned nupkg; `--verify` fetches outside the tree and checks the nupkg SHA-256, the byte-identical files and the Authenticode signer (last line `conpty-vendor: verified <version>`); `--probe` shows its four refusals (last line `conpty-vendor probe: 4/4 refused, control clean`). Pins parsed from `src/conpty.rs`; needs the network; CI `windows-2025` step `ConPTY vendor verification`
source: fct-irhgihut «# removed with the Windows code; recoverable at the tag multi-os-final»
replace-with: no ConPTY vendoring: `scripts/conpty-vendor.sh` and `vendor/conpty/` were removed with the Windows code, recoverable at the tag `multi-os-final`
@leaf_edit n=21 leaf=.claude/docs/commands.md line=74 class=stale
leaf-text: CI job `release`, all 3 OS
source: fct-shyrfs6v «`ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs), the one leg of `strategy.matrix.os` in `test`, `perf`, `lint` and `release`»
replace-with: CI job `release` on `ubuntu-latest`
@leaf_edit n=22 leaf=.claude/docs/commands.md line=84 class=retired
leaf-text: ; `scripts/junit-os-sets.py --run <id>` (python 3 standard library and the host's `gh`) — the operator pass's per-OS JUnit reading, never a CI step: downloads `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest` into a temporary dir removed on exit, prints `{os}: {n} cases · {f} failed · {s} skipped` per OS, then `## {label}: {count}` and the sorted `classname::name` lines of `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`; exit 0, 1 (a report missing or unparsable, named) or 2 (usage); reads no environment variable, writes nothing in the tree
source: fct-shyrfs6v «`ubuntu-latest` (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs), the one leg of `strategy.matrix.os` in `test`, `perf`, `lint` and `release`»
replace-with: ; `junit-ubuntu-latest` is the one JUnit artifact (matrix `[ubuntu-latest]`)
