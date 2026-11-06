@apply_row n=1 ref_file=fanout-obs-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-obs-plan.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-obs-plan--cascade-g1.md ref_n=4 v=folded_into to_file=fanout-obs-plan.md to_n=2
@apply_row n=4 ref_file=fanout-obs-plan.md ref_n=3 v=applied edit=E3
@apply_row n=5 ref_file=fanout-obs-plan.md ref_n=4 v=applied edit=E4
@apply_row n=6 ref_file=fanout-obs-plan.md ref_n=5 v=applied edit=E5
@apply_row n=7 ref_file=fanout-obs-plan.md ref_n=6 v=refused
why: its place, fct-v725cwjk, stands in the contract key snapshot-paste-to-ai-integration, outside doc:obs-plan's body (this batch's one target read); the key's writer carries it
@apply_row n=8 ref_file=fanout-obs-plan.md ref_n=7 v=applied edit=E6
@apply_row n=9 ref_file=fanout-obs-plan--cascade-g4.md ref_n=21 v=folded_into to_file=fanout-obs-plan.md to_n=7
@apply_row n=10 ref_file=fanout-obs-plan.md ref_n=8 v=applied edit=E7
@apply_row n=11 ref_file=fanout-obs-plan--cascade-g4.md ref_n=22 v=folded_into to_file=fanout-obs-plan.md to_n=8
@apply_row n=12 ref_file=fanout-obs-plan.md ref_n=9 v=applied edit=E8
@apply_row n=13 ref_file=fanout-obs-plan.md ref_n=10 v=applied edit=E9
@apply_row n=14 ref_file=fanout-obs-plan--cascade-g1.md ref_n=17 v=folded_into to_file=fanout-obs-plan.md to_n=10
@apply_row n=15 ref_file=fanout-obs-plan.md ref_n=11 v=refused
why: its place, fct-i4z6jsd7, stands in the contract key logging-stack, outside doc:obs-plan's body (this batch's one target read); the key's writer carries it
@apply_row n=16 ref_file=fanout-obs-plan.md ref_n=12 v=refused
why: its place, fct-brueynpb, stands in the contract key logging-stack, outside doc:obs-plan's body (this batch's one target read); the key's writer carries it
@apply_row n=17 ref_file=fanout-obs-plan.md ref_n=13 v=applied edit=E10
@apply_row n=18 ref_file=fanout-obs-plan--cascade-g1.md ref_n=13 v=folded_into to_file=fanout-obs-plan.md to_n=13
@apply_row n=19 ref_file=fanout-obs-plan.md ref_n=14 v=applied edit=E11
@apply_row n=20 ref_file=fanout-obs-plan--cascade-g4.md ref_n=23 v=folded_into to_file=fanout-obs-plan.md to_n=14
@apply_row n=21 ref_file=fanout-obs-plan.md ref_n=15 v=refused
why: its place, fct-r43ltybn, stands in the contract key log-file-location, outside doc:obs-plan's body (this batch's one target read); the key's writer carries it
@apply_row n=22 ref_file=fanout-obs-plan.md ref_n=16 v=refused
why: its place, fct-6bbewo2y, stands in the contract key otel-sdk-init, outside doc:obs-plan's body (this batch's one target read); the key's writer carries it
@apply_row n=23 ref_file=fanout-obs-plan.md ref_n=17 v=applied edit=E12
@apply_row n=24 ref_file=fanout-obs-plan--cascade-g3.md ref_n=21 v=folded_into to_file=fanout-obs-plan.md to_n=17
@apply_row n=25 ref_file=fanout-obs-plan.md ref_n=18 v=applied edit=E13
@apply_row n=26 ref_file=fanout-obs-plan--cascade-g3.md ref_n=20 v=folded_into to_file=fanout-obs-plan.md to_n=18
@apply_row n=27 ref_file=fanout-obs-plan.md ref_n=19 v=applied edit=E14
@apply_row n=28 ref_file=fanout-obs-plan--cascade-g4.md ref_n=20 v=folded_into to_file=fanout-obs-plan.md to_n=19
@apply_row n=29 ref_file=fanout-obs-plan.md ref_n=20 v=applied edit=E15
@apply_row n=30 ref_file=fanout-obs-plan.md ref_n=21 v=refused
why: its place, fct-rrjwwqcp, stands in the contract key service-identity, outside doc:obs-plan's body (this batch's one target read); the key's writer carries it
@fact id=fct-t2iihkln expect_rev=1
old: portable-pty `=0.8.1`, plus the windows-sys `TerminateProcess` fallback
new: portable-pty `=0.8.1`

@fact id=fct-m24p7svx expect_rev=1
old: the ConPTY and openpty OS APIs behind it are boundary-only
new: the openpty OS API behind it is boundary-only

@fact id=fct-p4bwzizt expect_rev=1
old: ndjson on a named pipe or Unix socket, one endpoint
new: ndjson on a Unix socket, one endpoint

@fact id=fct-zce67g4u expect_rev=1
old: OS APIs: ConPTY / openpty, named pipe / Unix socket, filesystem locks and atomic rename, sysinfo, `TerminateProcess`
new: OS APIs: openpty, Unix socket, filesystem locks and atomic rename, sysinfo

@fact id=fct-q2zh72zo expect_rev=1
old: (`scripts/agent-run.sh logs` / `.ps1`, via
new: (`scripts/agent-run.sh logs`, via

@fact id=fct-xyexk37w expect_rev=1
old: see §8 item 5, and the dispatch-only `windows-mutants.yml`, the boundary audit's Windows mutation leg, which uploads nothing), native matrix `windows-2025` / `macos-latest` / `ubuntu-latest` (arch
new: see §8 item 5), matrix `[ubuntu-latest]` (arch

@fact id=fct-rzzg2wyr expect_rev=1
old: `shell: bash` on all three OSes, and
new: `shell: bash` on the `ubuntu-latest` leg, and

@fact id=fct-mjdbrsjn expect_rev=1
old: Coverage Triggers, Multi-OS; arch CI/CD (windows-2025 / macos-latest / ubuntu-latest)
new: Coverage Triggers; arch CI/CD (`ubuntu-latest`)

@fact id=fct-dao4nifd expect_rev=1
old: the same file sink and line schema on every OS, with identical fields and no OS-specific
new: one file sink and one line schema on Linux, with no OS-specific

@fact id=fct-wrd7y3z2 expect_rev=1
old: 0700 directories (Windows: home DACL), plus
new: 0700 directories, plus
old: in `run`'s version gate, on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged) and on `verify`'s
new: in `run`'s version gate and on `verify`'s

@fact id=fct-ydzhack5 expect_rev=1
old: not 0600 (or not DACL-protected on Windows).
new: not 0600.

@fact id=fct-itztmsse expect_rev=1
old: env values in a span, and never the sideload companions' paths or hashes; the child's
new: env values in a span; the child's
old: in no line or span; a sideload outcome is never a refusal or an exit change; a failure's
new: in no line or span; a failure's

@fact id=fct-q67chfay expect_rev=1
old: the recorded wrapper (Linux and Windows) or runs under another euid (Linux and macOS) ends
new: the recorded wrapper or runs under another euid ends
old: `server-verify-failed` and no line (on macOS an euid mismatch alone, since macOS compares no peer pid, its named carve-out); before
new: `server-verify-failed` and no line; before

@fact id=fct-pqz5mgd5 expect_rev=1
old: server-verification mismatch (Linux, Windows and macOS, on macOS an euid mismatch alone) — is
new: server-verification mismatch — is

@fact id=fct-26ufyipy expect_rev=1
old: a start refusal (`.cmd`/`.bat` child, live or stale name,
new: a start refusal (live or stale name,
old: two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid)
new: two fixed stderr lines (no path or pid)
old: called by `run` (8 sites), `verify` (6),
new: called by `run` (7 sites), `verify` (5),
old: at chunk 2026-11-01-unix-endpoint-and-home-hardening;
new: at chunk 2026-11-05-linux-only-code-and-ci;
old: adds three refusals to `run`'s six:
new: adds three refusals to `run`'s five:

@pattern id=obs1-01
fixed: plus the windows-sys `TerminateProcess` fallback
@pattern id=obs1-02 ignore_case=true
fixed: the ConPTY and openpty OS APIs
@pattern id=obs1-03 ignore_case=true
fixed: on a named pipe or Unix socket
@pattern id=obs1-04
fixed: ConPTY / openpty, named pipe / Unix socket
@pattern id=obs1-05
fixed: `scripts/agent-run.sh logs` / `.ps1`
@pattern id=obs1-06
fixed: the boundary audit's Windows mutation leg
@pattern id=obs1-07 ignore_case=true
fixed: on all three OSes
@pattern id=obs1-08
fixed: Coverage Triggers, Multi-OS
@pattern id=obs1-09 ignore_case=true
fixed: line schema on every OS, with identical fields
@pattern id=obs1-10
fixed: 0700 directories (Windows: home DACL)
@pattern id=obs1-11
fixed: (or not DACL-protected on Windows)
@pattern id=obs1-12
fixed: a sideload outcome is never a refusal or an exit change
@pattern id=obs1-13
fixed: since macOS compares no peer pid
@pattern id=obs1-14
fixed: (Linux, Windows and macOS, on macOS an euid mismatch alone)
@pattern id=obs1-15
fixed: command is a .cmd or .bat script
