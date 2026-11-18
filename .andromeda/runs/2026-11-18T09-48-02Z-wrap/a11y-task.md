@task_prose part=session
The session read the relied trust-record ledger row live on claude 2.1.325 in the live rig's private home: start S0B read 23/23 with trust-record pass, so the row is verified on 2.1.325 only, with nothing claimed for 2.1.326 or 2.1.327.
It named the first screen of each of verify's six interactive runs with closed words (trust, imports, box, none, exited), classified over the compiled signatures, modal before box, and added the words, settle walls and closed row facts (literal ids, rule above prompt, first-row family, last column) to the stamp's measured block.
It added a hidden rig-only verify option, absent from --help, that on a failed recording verify writes each non-box run's first screen as whole rows passed through the fixture scrub and refused whole when any row is unclean; a plain record and a passing verify write no such file.
The cause of runs B to F stopping short of the input box was named as claude's browser-tools prompt; only the rig was mended, its seed now carrying five bundle keys including the browser-tools key, and viola verify and viola run are unchanged for that prompt, which becomes its own route entry.
The verify-window test-class kill moved from 60 s to 80 s; a red under coverage, stray coverage profiles from the version probe run in the test cwd, was fixed by running the probe in the scratch root.
The relied ledger row count stays 23; no capability is claimed and no CLI output line, refusal, exit code or help line of a served verb changed.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the screen signature reader over compiled literals, now also the first-screen classifier and its closed row facts
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger and its stamp writer, the measured block gaining first-screen words, settles and facts
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit cases
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the stamp writer's unit cases, a new case for the six first screens
@task_file n=5 path=src/cmd/verify.rs
what: the viola verify command, its record option and the new hidden rig-only first-screen record with the scrub and whole-set refusal
@task_file n=6 path=src/cmd/verify/typed.rs
what: verify's typed interactive runs A to F
@task_file n=7 path=src/cmd/verify/typed/run.rs
what: one interactive verify run's reading of the child screen, now keeping its first screen and settle
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: verify's dialog, plan and resume runs, each returning its first screen
@task_file n=9 path=tests/cli_verify.rs
what: the viola verify integration suite, new cases for the failed-verify first-screen record and its absence from help
@task_file n=10 path=tests/cli_verify/runs.rs
what: verify's per-run integration cases over the fake agent
@task_file n=11 path=tests/support/verify.rs
what: the verify test support, the started-verify helper and its place
@task_file n=12 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated child scratch, its private config seed of five bundle keys and its git-root layout
@task_file n=13 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig's arms, labels and summary lines, the V arm and the new VC arm
@task_file n=14 path=crates/viola-e2e/src/rig/screen.rs
what: the rig's own screen model at a given size
@task_file n=15 path=crates/viola-e2e/src/rig/ledger.rs
what: the rig's live-start ledger and its spare labels
@task_file n=16 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary and its live verb
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent
@task_file n=18 path=.config/nextest.toml
what: the nextest profiles, the verify-window test-class kill moved to 80 s in ci and mutants
@task_file n=19 path=.gitignore
what: the ignore list, gaining the local first-screen record files under the fixtures
@task_item n=1 part=touches
No screen-content parsing beyond signatures: the first-screen classifier and its row facts read only compiled literal ids and closed row families, but the rig-only failed-verify record now writes whole scrubbed rows of the child screen to a local file, an exception the founder answered himself.
@task_item n=2 part=touches
Boundary check never inspects claude content and the single vendor-content boundary: the hidden failed-verify first-screen record copies vendor screen rows out, rig-only and local, through the scrub and a whole-set refusal; a picker should judge whether those rows need the exception beside them.
@task_item n=3 part=touches
Paths not carried: the first-screen record rewrites the home path in its rows and keeps a temp path, through the fixture scrub.
@task_item n=4 part=touches
viola --help served verbs only and the usage line: the new verify option is hidden and absent from --help, pinned by a case that help names no first-screens option.
@task_item n=5 part=touches
Critical path verify output and the verify step line: viola verify's printed output is unchanged; only the stamp's measured block and the hidden record file gained content.
@task_item n=6 part=touches
viola verify interrupted, its exit and stderr pair: a stopped verify with record writes no first-screen file, a new case beside those contracts.
@task_item n=7 part=touches
Live readings never gate: the S0B reading of 23/23 on 2.1.325 is a live rig reading, recorded and not a gate.
@task_item n=8 part=touches
Time bounds fall on the driver only and no harsh timeouts: the verify-window test-class kill moved from 60 s to 80 s, a test-harness bound and not one on the human; a row stating sixty seconds for it would be stale.
@task_item n=9 part=touches
A11y cases run inside the coverage suite and gate requires coverage: a red read only under coverage, profiles written into a test cwd by an instrumented child, was fixed; the coverage gate itself is unchanged.
@task_item n=10 part=touches
Control characters escaped: the first-screen rows go out as JSON after the scrub and the screen-fault check, not printed to the terminal.
@task_item n=11 part=touches
Wheel and human-keystroke rows: the browser-tools prompt is now named as a modal claude raises; viola run's pre-send gate is unchanged for it and the gap is a route entry, not built here.
@task_fact n=1 slug=11-a11y-anti-patterns-never-do-these
fact: The only exception to no screen-content parsing beyond signatures is the rig-only hidden failed-verify first-screen record: whole rows, scrubbed, refused whole when any row is unclean, kept local and never written by a plain record or a passing verify.
@task_fact n=2 slug=aria-patterns-roles
fact: viola verify carries one hidden option, the rig-only first-screen record, which --help never names; help lists served verbs and visible options only.
@task_item n=1 part=untouched
CLI refusal format, unable and hint lines, reason codes and exit codes: no refusal, code word or exit changed.
@task_item n=2 part=untouched
The viola list board, its captions, colour, SGR and truncation rows: no board or colour output changed.
@task_item n=3 part=untouched
Screen-reader targets and manual Orca and keyboard passes: nothing of them moved.
@task_item n=4 part=untouched
Revive, link, plugin install, send, wait and MCP frame-tool contracts: none of those verbs changed.
@task_item n=5 part=untouched
Absences of a web, graphical or touch frontend, i18n and a rendered page: still absent.
@task_item n=6 part=untouched
TUI passthrough contracts for viola run, its entry, keystrokes, terminal bytes and feed panic: viola run is unchanged.
@task_item n=7 part=untouched
Structured violation JSON rows and the a11y CI workflow steps: no CI step or status shape changed.
