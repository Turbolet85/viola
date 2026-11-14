@task_prose part=session
Test-only session in the viola-hyprland crate: two new inline unit tests pin the Hyprland seam's two MAX_FRAME bounds at the exact frame; no product line, caller, public API, dependency, schema or spec changed.
The read test plants a real hyprctl stub executable on the crate's own stub helper, each in its own tempdir, both planted before either spawns: a reply of exactly MAX_FRAME bytes reads an empty clients reply with process-start and process-exit lines and no parse-rejected; one byte more reads Oversize with a parse-rejected detail oversize.
The dispatch test feeds a reply of exactly MAX_FRAME spaces to the dispatch parser: it reads Refused (past the size check, not the success text); one byte more reads Oversize.
Each bound was proven by a remove-the-guard control that held: guard in place green, guard removed red with the new test the failing case.
The read case runs through the crate's own planted stub rather than the root fake agent, because a root-package test cannot kill a viola-hyprland mutant (mutants are tested by the owning package only).
The crate's own test count moved 82 to 84; no row holds it. CI green 7/7.
@task_file n=1 path=crates/viola-hyprland/src/read.rs
what: the Hyprland seam's bounded external reader: runs hyprctl and drains its reply under the MAX_FRAME frame cap, refusing a reply past it as oversize and logging process-start, process-exit and parse-rejected; holds its inline unit tests
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the Hyprland seam's reply parser: parses hyprctl's clients JSON and its dispatch reply under the MAX_FRAME frame cap, oversize refused before parsing, a non-success dispatch reply refused; holds its inline unit tests
@task_item n=1 part=touches
The rule that every external reader is bounded and bounded readers keep the frame cap: the Hyprland seam's read and dispatch parse now each have an exact-frame witness (MAX_FRAME accepted, MAX_FRAME plus one refused as oversize); the bound value itself is unchanged.
@task_item n=2 part=touches
The bounded remove-the-guard witness convention: both seam bounds were removed by a control and the new tests turned red, guard in place green.
@task_item n=3 part=touches
The MCP stdin line cap and its exact-MAX_FRAME line contract: the precedent this session follows for an exact-frame case; a true claim, unchanged, now with a Hyprland-seam counterpart beside it.
@task_item n=4 part=touches
The statusline shell-out frame cap: another bounded shell-out reader sharing the exact-frame idea; unchanged, a sibling case now exists for hyprctl.
@task_item n=5 part=touches
External processes are real stub executables: the read test plants a real shell-script hyprctl stub, not a mock and not the root fake agent.
@task_item n=6 part=touches
Mutants tested by their own package: the reason the read case lives in the crate's inline tests and not in a root integration binary.
@task_item n=7 part=touches
Mutation gate, verdict and survivors at the epoch boundary: the two bound sites are now killable by the owning package's tests; no survivor row or known-limit row names these sites, and no threshold moved.
@task_item n=8 part=touches
Inline unit tests and test function naming: two inline tests added under the long descriptive takes-and-refuses naming form.
@task_item n=9 part=touches
tempfile for test directories and fake-agent-style copies planted before any child: each stub in its own tempdir, both planted before either spawns.
@task_item n=10 part=touches
The parse-rejected, process-start and process-exit log event lines: the read test asserts the seam's line sequence and the oversize detail word; the line shape itself is unchanged.
@task_item n=11 part=touches
The bounded-readers contract table's hyprctl reply row: its witness is a parse test, not one that drives the read's own drain past its cap; this session adds that drain witness, and the row's witness naming is carried to the route, not changed here.
@task_item n=1 part=untouched
CLI verbs, exit codes, human and --json output: no CLI surface changed.
@task_item n=2 part=untouched
MCP tools, tool results and the wrapper channel JSON-RPC contracts: none changed.
@task_item n=3 part=untouched
The test harness contract, agent-run commands, gate breaches, pre-push stages and run documents: no harness change.
@task_item n=4 part=untouched
CI jobs, artifacts, supply-chain gates, toolchain pins and workflow permissions: no CI or dependency change.
@task_item n=5 part=untouched
Coverage floors, perf budgets and hook deadlines: no threshold or budget moved.
@task_item n=6 part=untouched
Fake agent, recorded fixtures, fixture hygiene and verify stamps: the root fake agent was deliberately not used and no fixture changed.
@task_item n=7 part=untouched
Budget governor, dialogs, send confirmation, wheel and link rules: no product behaviour changed.
@task_item n=8 part=untouched
Absences (browser suite, database, outbound HTTP, Windows and macOS, deploy stage): none gained or lost.
