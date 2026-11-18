@task_prose part=session
The session read the relied trust-record ledger row live on claude 2.1.325 in the live rig's private home: start S0B read 23/23 relied rows passing, trust-record pass, keys none, so the row is now verified on 2.1.325 under the rig only; nothing is claimed for 2.1.326 or 2.1.327.
It named the first screen of each interactive verify run: a classifier over the compiled screen signatures, modal before box, gives each of the six runs a closed word (trust, imports, box, none, exited), and the ledger stamp's measured block gains first_screen, first_settle_ms and first_facts (row count, box_any, literals by box literal id, rule_above_prompt, first_row family rule/banner/text, last_col).
It added a hidden verify option, --record-first-screens, passed only by the rig: on a failed --record verify it writes one FirstScreen.<x>.json per run not reaching the box (Run A's trust excepted), whole rows passed through the fixture scrub, an unclean set refused whole; a plain --record and a passing verify are unchanged, and the files are kept local and gitignored.
It measured the cause that stopped runs B to F short of the input box: 2.1.325's browser-tools prompt (ChromeAutoEnableDialog); viola verify and viola run were not changed for it, and a compiled modal literal for it is a new route entry, not built here.
The live rig gained a VC arm, a git-root and used-config seed (five bundle keys, including the browser-tools key set false), first-screen summary lines and the S0B labels; the seed retired two markers; the verify-window test kill moved 60 s to 80 s; the version probe now runs in a given cwd after an instrumented child left coverage profiles in the test cwd.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the claude screen signatures as compiled literals and the screen classifier verify reads rows with; now the first-screen classifier, its closed words and the first-screen row facts
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger's typed-run record and the stamp merge that writes the measured block of the ledger stamps
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit cases
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamp merge's unit cases, now including the six first screens, their settles and facts
@task_file n=5 path=src/cmd/verify.rs
what: the viola verify command: its arguments, its fixture recording under --record with the scrub and unclean-fixture refusal, now the hidden rig-only first-screen record
@task_file n=6 path=src/cmd/verify/typed.rs
what: verify's typed interactive runs A to F against the claude CLI
@task_file n=7 path=src/cmd/verify/typed/run.rs
what: one interactive verify run's screen reading, settle and end (kill, ctrl-c, finish), now keeping its first screen reading and rows
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: the dialog, plan and resume verify runs, each now returning its first screen to the typed-run record
@task_file n=9 path=tests/cli_verify.rs
what: the viola verify integration tests over the fake agent, including the --record fixture cases
@task_file n=10 path=tests/cli_verify/runs.rs
what: the verify integration tests of the typed runs' screens over the fake agent
@task_file n=11 path=tests/support/verify.rs
what: the test-support helpers that start a verify against the fake agent in an isolated test home
@task_file n=12 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated scratch and private config home: its git-root layout, its used-config seed and the local keep of first screens
@task_file n=13 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's arms, start labels, summary lines and the version probe, driving the real claude CLI locally
@task_file n=14 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's vt100 screen model
@task_file n=15 path=crates/viola-e2e/src/rig/ledger.rs
what: the live rig's start ledger and its start caps and spares
@task_file n=16 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig test-only binary and its live verb
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent, including its no-credential and no-host-path record check
@task_file n=18 path=.config/nextest.toml
what: the nextest profiles and per-class test kill times for ci and mutants
@task_file n=19 path=.gitignore
what: the repository ignore list, now including local first-screen files under the recorded claude fixtures
@task_item n=1 part=touches
contract/ledger-stamps/measured: the measured block gains first_screen (closed words trust, imports, box, none, exited), first_settle_ms (whole numbers) and first_facts (row count, box_any, literals keyed by box literal id, rule_above_prompt, first_row, last_col) for each of the six runs; no screen text enters it.
@task_item n=2 part=touches
decision/verify-record-only-clean: a failed --record verify with the hidden rig-only option now writes first-screen files; an exception to fixtures recorded only from a clean verify (a plain --record is unchanged).
@task_item n=3 part=touches
decision/screen-fixtures-signature-rows-only and decision/recorded-fixtures-scrubbed: the first-screen record keeps whole rows, not signature rows only, each passed through the fixture scrub and the screen-fault check, kept local and never committed; an exception to state beside them.
@task_item n=4 part=touches
contract/unclean-fixture-refusal items and contract/viola-verify-refusal-codes/unclean-fixture with contract/verify-exit-detail/unclean-fixture: an unclean first-screen set is refused whole through the same refusal.
@task_item n=5 part=touches
contract/viola-cli-arguments/verify and decision/cli-flag-and-subcommand-style: verify takes a hidden option --record-first-screens, absent from --help, passed only by the live rig.
@task_item n=6 part=touches
decision/screen-rows-read-only-by-verify: verify now reads each interactive run's first screen rows and classifies them; still verify alone reads row text, and run is unchanged.
@task_item n=7 part=touches
decision/screen-signatures-compiled and decision/verify-screen-read-against-literals-like rules: the first-screen classifier works over the compiled signatures, modal before box; 2.1.325's browser-tools prompt has no compiled literal yet (a route entry, not built).
@task_item n=8 part=touches
decision/trust-record-ledger-row and decision/verify-own-trust-record: the trust-record row is now verified on claude 2.1.325 under the live rig's private config (23/23); nothing is claimed for 2.1.326 or 2.1.327.
@task_item n=9 part=touches
decision/version-verified-all-relied-rows: the relied row count stays 23; a version is verified only when all 23 pass, as S0B read.
@task_item n=10 part=touches
decision/verify-never-types-into-dialog, decision/verify-types-nothing-into-dialogs and decision/no-typing-into-trust-dialogs: confirmed, not changed; verify read the browser-tools prompt as a stop and typed nothing.
@task_item n=11 part=touches
decision/stamps-single-writer and decision/verify-sole-stamps-writer: verify's stamp merge writes the new measured keys; still the one writer.
@task_item n=12 part=touches
decision/agent-claude-pure and decision/claude-shapes-in-one-crate: the first-screen classifier, its closed words and row facts are added in the claude agent crate as pure functions over rows.
@task_item n=13 part=touches
decision/wire-enum-values-kebab-case: the first-screen words, the box literal ids and the row families are kebab-case closed words.
@task_item n=14 part=touches
decision/on-disk-file-naming and contract/probe-capture/payload-file: a new local file name FirstScreen.<x>.json with v, run, screen, cols 80 and rows.
@task_item n=15 part=touches
name/binary/viola-live-rig, decision/real-cli-probe-local-only and decision/real-claude-local-only: the live rig, local only, gained a VC arm, labels S0R-3, S0C, S0B and S0B-2, a git-root and five-key used-config seed in its private home, and new summary lines; CI steps unchanged.
@task_item n=16 part=touches
decision/e2e-test-only and decision/test-binaries-never-released: the rig's new arm and keep of first screens live in the test-only e2e crate.
@task_item n=17 part=touches
decision/profraw-census-dev-host-only and decision/test-evidence-survives-kill: an instrumented fake-agent child spawned by the rig's version probe in the test cwd wrote coverage profiles there; the probe now runs in the scratch root; the verify-window test class kill moved 60 s to 80 s in ci and mutants.
@task_item n=18 part=touches
decision/bounded-readers: the rig's config reader is bounded at MAX_FRAME.
@task_fact n=1 slug=standard-contracts
fact: the ledger stamp's measured block holds, for each of the six interactive verify runs, first_screen from the closed words trust, imports, box, none, exited, first_settle_ms as a whole number, and first_facts with the row count, box_any, literals keyed by box literal id with each its first row, rule_above_prompt, first_row from rule, banner, text, and last_col; no screen text.
@task_fact n=2 slug=occupied-resources
fact: viola verify has one hidden option, --record-first-screens, absent from --help and passed only by the live rig; on a failed --record verify it writes one local FirstScreen.<x>.json of whole scrubbed rows per run not reaching the input box, Run A's trust screen excepted, refuses an unclean set whole, and is the one exception to fixtures recorded only from a clean verify and to signature-rows-only fixtures; a passing verify or a plain --record writes none.
@task_fact n=3 slug=established-decisions
fact: verify classifies each interactive run's first settled screen against the compiled signatures, a modal literal before a box literal, into one closed word, and reads an ended child as exited.
@task_item n=1 part=untouched
The wrapper channel, its methods, frames, fault codes and JSON-RPC contracts: nothing on the channel changed.
@task_item n=2 part=untouched
viola run, send and the pre-send readiness gate, the wheel and dialog hold: unchanged; the browser-tools prompt for run is a later route entry.
@task_item n=3 part=untouched
Hooks, the hook-to-kind map, the plugin and its install: unchanged.
@task_item n=4 part=untouched
The MCP server, its tools and results: unchanged.
@task_item n=5 part=untouched
CLI exit codes and refusal codes of verbs other than verify, and verify's own exit codes: no code added or moved.
@task_item n=6 part=untouched
State, events log, snapshots, liveness and revive: unchanged.
@task_item n=7 part=untouched
Dependencies, toolchain pin, MSRV, cargo-deny policy and the GitHub Actions workflow steps: none changed.
@task_item n=8 part=untouched
The Hyprland seam, the fleet console and window features: unreached.
