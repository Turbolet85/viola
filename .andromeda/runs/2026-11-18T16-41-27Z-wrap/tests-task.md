@task_prose part=session
The session made the CLI's browser-tools prompt (literal Yes, use my browser) a third compiled modal literal, a sixth first-screen closed word browser-tools, and a 24th relied ledger row browser-tools-prompt, last, with its verify probe; verify and the pre-send gate never type into the prompt.
The row reads pass, absent (no run met the prompt) or fail; the verified rule moved from every relied row pass to every relied row counting, where absent counts on browser-tools-prompt alone; a stamp of the 23 pre-chunk ids reads unverified.
viola verify's step line now prints the verdict word pass, absent or fail; the summary n pass counts the rows that count (an absent row in n); relied rows 23 to 24, step counter [NN/24], all-pass summary stamped <ver>  24 pass  0 fail.
The fake agent gained the argv option --start-screen <phase> (under --screens only): a trusted start keeps that screen standing, each stdin byte is one key receipt line, Ctrl-C ends with no key line, no byte reaches the input box, no env var.
The live rig gained arms VP and VK (labels BP, BP-2, BK) stopping with no key, a scratch seed that leaves the browser-tools key out for VP, summary lines cache-a (written, absent or unread, reading only the key NAME cachedChromeExtensionInstalled in its own private config, by the founder's own recorded answer) and browser-tools-prompt; the harness local-live row list went to 24 with absent counting on the 24th.
Measured live on 2.1.325: BP showed the prompt (pass-count 8/24, verify-exit 1, cache-a written), BK read 24/24 with the row absent (cache-a absent); a remove-the-guard control on a fail-fast nextest run read INDETERMINATE; CI green 7/7; nothing retired.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures: the modal literals, the untrusted-start modal set, and the first-screen classification with its closed words
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the CLI capability ledger: its relied rows, each row's verify probe and check, the row word, and the verified-version rule over the stamps
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: inline unit cases of the ledger rows' readings (pass, absent, fail)
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: inline unit cases of the ledger checks (the modal-signature and trust-record checks)
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: inline unit cases of the ledger stamps: merging a stamp, the verified rule, the row counting
@task_file n=6 path=src/cmd/verify.rs
what: viola verify: its step line per ledger row and its summary line
@task_file n=7 path=src/cmd/verify/typed/pty.rs
what: verify's typed runs over the PTY: settling on a first screen
@task_file n=8 path=src/cmd/verify/typed/turns.rs
what: verify's typed runs: the turns typed into each run
@task_file n=9 path=src/run/gate.rs
what: viola run's readiness gate before a send, reading the compiled screen signatures
@task_file n=10 path=src/run/version_gate.rs
what: the unverified CLI version gate reading the stamps
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the fake agent (test-only binary): its argv options, its screens and its receipt lines
@task_file n=12 path=tests/cli_send.rs
what: the viola send integration suite over the fake agent
@task_file n=13 path=tests/cli_verify.rs
what: the viola verify integration suite: step lines, summary line, stamps
@task_file n=14 path=tests/cli_verify/runs.rs
what: viola verify integration cases over the typed runs and their first screens
@task_file n=15 path=tests/cli_verify/stamps.rs
what: viola verify integration cases over the stamps written
@task_file n=16 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite: every ledger row has a verify probe
@task_file n=17 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command and its local-live suite row list
@task_file n=18 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its arms, labels, stops and summary lines
@task_file n=19 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's private scratch home: the seeded used markers and the cache-a reader
@task_file n=20 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent
@task_item n=1 part=touches
decision/version-gate-needs-all-relied-rows: the verified rule now reads every relied row counting, pass, or absent on browser-tools-prompt alone; absent on any other row stays unverified.
@task_item n=2 part=touches
contract/verify-output/step-line: the step line's verdict word is now pass, absent or fail, the counter [NN/24].
@task_item n=3 part=touches
contract/verify-output/summary-line, contract/viola-verify/stamped-line, contract/cli-human-lines/stamped and contract/verify-output/last-line: n pass counts the rows that count (an absent row in n); the all-pass line is stamped <ver>  24 pass  0 fail.
@task_item n=4 part=touches
decision/ledger-row-verify-probe and decision/ledger-probe-contract-suite: a 24th relied row browser-tools-prompt with its verify probe, its check true unless some first screen reads none.
@task_item n=5 part=touches
decision/stamps-refuse-widened-ledger and decision/single-writers: the stamp gains row key browser-tools-prompt (pass, absent, fail) and the first_screen closed word browser-tools; a stamp of the 23 pre-chunk ids reads unverified.
@task_item n=6 part=touches
decision/screen-signatures-only, decision/no-assertion-beyond-compiled-literals and decision/never-parse-rendered-child-screen: a third compiled modal literal Yes, use my browser, and a sixth first-screen word browser-tools; the modal-signature check reads the untrusted-start literals only.
@task_item n=7 part=touches
decision/readiness-gate-tested-by-refusals, contract/cli-send/input-not-ready and contract/cli-exit-codes/13: the pre-send gate refuses at once on the browser-tools prompt; send on a verified CLI exits 13 with no key typed.
@task_item n=8 part=touches
contract/fake-agent-cli/argv-only-options and decision/fake-agent-argv-only: the new argv option --start-screen <phase>, drawn only under --screens, no env var.
@task_item n=9 part=touches
contract/fake-agent-receipt/key, contract/fake-agent-input/keystroke and contract/fake-agent-screens/trust-dialog: under --start-screen the standing screen takes every stdin byte as one key receipt line, Ctrl-C ends with no key line, no byte reaches the input box.
@task_item n=10 part=touches
decision/wrapper-boots-pass-trusted-screens and decision/fake-agent-matches-recordings: a trusted start under --screens may now stand on a screen other than its ready screen.
@task_item n=11 part=touches
decision/standing-live-rig: arms VP and VK (labels BP, BP-2, BK), their no-key stop, VP's seed leaving the browser-tools key out, summary lines cache-a and browser-tools-prompt; pass-count and failing read through the counting rule; the rig floor of 21 rows unchanged.
@task_item n=12 part=touches
decision/tests-never-read-host-sessions and decision/real-claude-local-only: the rig reads whether the key NAME cachedChromeExtensionInstalled stands in its own private scratch config, never a value, by the founder's own recorded answer.
@task_item n=13 part=touches
contract/local-live-suite/pass, contract/run-suite/local-live and contract/local-live-failure/row-missing: the local-live row list is 24 rows and accepts absent on browser-tools-prompt alone.
@task_item n=14 part=touches
decision/harness-boot-creates-stamps-and-starts, decision/fake-agent-replay-for-stamps and contract/agent-run-status-polled-fields/cli-verified: the harness boot now stamps 24 rows, cli_verified true.
@task_item n=15 part=touches
decision/live-failing-reading-not-verification and decision/verify-failing-row-no-refusal: on a host where the browser-tools prompt stands unanswered verify settles on B to F, types nothing and leaves the version unverified (BP 8/24, verify exit 1).
@task_item n=16 part=touches
decision/unverified-cli-gate and decision/unverified-cli-gates-answer-only: a CLI whose browser-tools prompt is unanswered stays unverified until a person answers it once.
@task_item n=17 part=touches
decision/record-only-at-zero-failing-rows and decision/stamps-from-fake-agent-verify: an absent row now counts toward a stamping verify that fails nothing.
@task_item n=18 part=touches
decision/bounded-remove-the-guard-witness: a remove-the-guard control fired on a fail-fast nextest run can stop before the named case and read INDETERMINATE.
@task_fact n=1 slug=test-strategy
fact: A remove-the-guard control fired on a fail-fast nextest run can read INDETERMINATE: the guard's neighbours fail first and the runner stops before the named case; the control's verdict is then neither holds nor fails.
@task_fact n=2 slug=6-e2e-test-strategy
fact: The live rig reads the extension cache key only by its NAME, cachedChromeExtensionInstalled, in its own private scratch config, after verify exits and before teardown, printing cache-a written, absent or unread; it never reads the value.
@task_fact n=3 slug=6-e2e-test-strategy
fact: The browser-tools prompt literal is read live on 2.1.325 only, on a host where the extension is detected and the account flag is on; nothing is claimed for later CLI versions or a reworded prompt.
@task_item n=1 part=untouched
MCP tool surfaces, tools/list, MCP invalid-params fields and MCP reply screens: no MCP method or tool changed.
@task_item n=2 part=untouched
The wrapper channel, JSON-RPC errors, link and unlink methods, socket directory and strict-modes refusals: no IPC method, endpoint or socket changed.
@task_item n=3 part=untouched
Hook exec, dialog decision bodies, dialog hold and expiry, hook deadlines and fail-open rows: no hook or dialog behaviour moved.
@task_item n=4 part=untouched
viola revive, viola list, viola wait, viola last, viola release and the wheel rows: none of those verbs changed.
@task_item n=5 part=untouched
CI workflows, artifacts, supply-chain, cargo-deny, MSRV, zizmor and toolchain pins: no workflow, dependency or tool version changed.
@task_item n=6 part=untouched
Coverage floors, mutation gates, perf budgets and the quality gate's breach codes: no threshold or budget moved.
@task_item n=7 part=untouched
Process log format, log event corr, secret scan, pid-file and status-endpoint shape rows: no log field, scan class or status field changed.
@task_item n=8 part=untouched
Fuzz and property test rows: no parser surface or corpus changed.
