@task_prose part=session
The readiness read of the claude screen gained a second arm: the input box reads up when no row holds a modal literal and either a footer literal stands or the box's own rows stand (a rule row of the compiled rule char, at least 40 of them, directly above a row opening the compiled prompt char).
One predicate now decides readiness for verify's typed-run guards and settle and box waits, for the input-box-signature ledger row's check, and for viola run's pre-send readiness gate on a verified CLI, which now reads a screen ready on the box's rows alone; quiet period, GATE_MAX_WAIT and the poisoned arm are unchanged.
Verify's settle and box waits end on the box up OR a modal literal on a row, keeping their modal arm; verify now counts guards passed on the box alone and stamps that count in the ledger stamps' measured typed probe.
A recorded screen fixture keeps a box rule row as its rule run alone and the prompt row below it as the prompt char alone; the fixture walk accepts those two forms and refuses any other text on them, and a recording refuses where the live prompt row holds a home path, the user word or an email.
The dependency policy now ignores three rmcp advisories, each reasoned as sitting in a banned or disabled rmcp feature and standing until rmcp leaves its ratified range; a lockfile bump of rustix cleared a fourth advisory; rmcp's version, range and features are unchanged.
The founder ratified both forks this wrap: the three ignores, and the box signature beside the footer literals for verify and run's gate, taken as a boundary widening.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures and the vt100 screen verdict, the pre-send readiness gate's read of the input box and modal literals
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger, its input-box-signature row check, the recorded screen fixture rows it keeps and its scrub fault check, and the ledger stamps' measured typed probe
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit cases for the signature row check, the fixture signature rows, the fixture fault check and the stamps merge
@task_file n=4 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed-run turn guards and the input-box readiness read verify uses between added turns
@task_file n=5 path=src/cmd/verify/typed/pty.rs
what: viola verify's PTY settle and box wait, waiting for a quiet screen holding a compiled literal
@task_file n=6 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed probe runs, Run B's ready guard and the trusted run's framing turns
@task_file n=7 path=src/run/gate.rs
what: viola run's pre-send readiness gate, the wait for the input box before a send
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene walk, refusing a screen fixture row that is no signature row
@task_file n=9 path=deny.toml
what: the cargo-deny dependency policy, its advisories ignore list, bans, banned features, licences and sources
@task_file n=10 path=Cargo.lock
what: the workspace lockfile of exact resolved dependency versions
@task_file n=11 path=fuzz/fuzz_targets/vt100_feed.rs
what: the fuzz workspace's vt100 feed target, building its own screen signatures
@task_item n=1 part=touches
decision/screen-signatures-compiled: ready no longer only when a row holds a footer literal; the box's rule and prompt chars are now compiled input-box literals beside the two footer literals, read by one predicate with no modal on any row.
@task_item n=2 part=touches
decision/verify-box-wait-for-literal: verify's box wait and settle after added turns now end on the box's own rows or a footer literal or a modal literal, a quiet screen holding a compiled literal still.
@task_item n=3 part=touches
decision/vt100-readiness-gate and the pre-send gate rows: viola run's gate on a verified CLI now reads a screen ready on the box's rows alone, with no footer literal; a modal on any row still reads not ready.
@task_item n=4 part=touches
decision/run-never-reads-screen-content and decision/screen-rows-read-only-by-verify: indirect, run's gate now reads the box's rule and prompt rows for compiled chars through the screen verdict; check each still holds as written.
@task_item n=5 part=touches
decision/cli-capability-ledger and decision/capability-ledger-gate: the input-box-signature row's check now reads the box-or-footer predicate over the ready, turn and long-turn screens; its id and the relied count of 24 are unchanged.
@task_item n=6 part=touches
decision/screen-fixtures-signature-rows-only: a recorded fixture now also keeps a box rule row as its rule run alone and the prompt row under it as the prompt char alone, never the text after it.
@task_item n=7 part=touches
decision/recorded-fixtures-scrubbed: the hygiene walk now passes a row made only of the rule char or exactly the prompt char, and refuses a prompt row with text after the prompt char or a rule row with other text.
@task_item n=8 part=touches
contract/unclean-fixture-refusal/home-path, contract/unclean-fixture-refusal/username, contract/unclean-fixture-refusal/email and contract/verify-exit-detail/unclean-fixture: a recording now also refuses where the input box's live prompt row holds a home path, the user word or an email, the kept rule row's seam.
@task_item n=9 part=touches
contract/ledger-stamps/measured: the measured typed probe gains box_only_guards, an integer or null, beside the five timings; no envelope version bump and run reads no field of it.
@task_item n=10 part=touches
decision/verify-sole-stamps-writer and decision/stamps-merge-own-version-only: indirect, verify's stamps merge now writes the box-only guard count.
@task_item n=11 part=touches
decision/cargo-deny-dependency-policy, decision/cargo-deny-bans and decision/weekly-advisories: the advisories ignore list moves from empty to three reasoned rmcp ignores, each standing until rmcp leaves its ratified range.
@task_item n=12 part=touches
decision/banned-features, decision/rmcp-release-features, decision/rmcp-features-exact and decision/rmcp-exact-features: indirect, the three ignores rest on rmcp's banned auth and Streamable HTTP server features and its off Streamable HTTP clients; the features themselves are unchanged.
@task_item n=13 part=touches
decision/workspace-dependency-pins: indirect, a lockfile-only bump of a transitive crate clearing an advisory; no workspace pin and no crate added.
@task_item n=14 part=touches
decision/claude-shapes-isolated, decision/claude-shapes-in-one-crate and decision/agent-claude-pure: indirect, the box's two chars and the new predicate live in the Claude agent crate as pure reads with no file I/O.
@task_item n=15 part=touches
decision/fuzz-own-workspace, decision/fuzz-separate-workspace and decision/fuzz-nightly-toolchain: indirect, the fuzz target builds a screen signatures value and had to take the two new chars; no gate entry builds the fuzz workspace.
@task_item n=16 part=touches
decision/verify-never-types-into-dialog and decision/no-typing-into-trust-dialogs: indirect, verify's settle keeps its modal arm so a trust modal still settles and is never typed into.
@task_item n=17 part=touches
decision/gate-timings-compiled and decision/poisoned-screen-model: indirect, the gate's quiet period, GATE_MAX_WAIT and poisoned arm stand unchanged around the new box arm.
@task_fact n=1 slug=established-decisions
fact: The input box's rule row reads as the box only when it is the compiled rule char alone, at least 40 of them, trailing spaces aside; a shorter rule leaves the gate waiting.
@task_fact n=2 slug=established-decisions
fact: Verify counts each typed-run guard that passes on the input box's own rows with no footer literal on any row, and stamps that count as the box-only guards measure.
@task_fact n=3 slug=occupied-resources
fact: A kept box rule row is box-drawing alone, so its row seam is the neighbouring live prompt row's whole text; a recording whose prompt row holds a home path, the user word or an email is refused, never kept.
@task_fact n=4 slug=build-system
fact: A dependency advisory is ignored only where it sits in a feature the dependency policy bans or keeps off the release graph, each ignore with a reason naming that feature and standing until the dependency leaves its ratified range.
@task_item n=1 part=untouched
The wheel, its claim, take and release, and the human-always-wins rules: no wheel behaviour changed.
@task_item n=2 part=untouched
The wrapper channel protocol, its methods, frames, fault codes, refusal reasons and protocol version: no frame or method changed.
@task_item n=3 part=untouched
The MCP server, its tools, results, stdin caps and exit codes: unchanged.
@task_item n=4 part=untouched
Hooks, hook tiers, the hook-to-kind map, dialog hold and dialog answering: unchanged.
@task_item n=5 part=untouched
The budget governor, budget.json and the statusline wrapper: unchanged.
@task_item n=6 part=untouched
Revive, list and the board, session liveness, instance snapshots and the events log: unchanged.
@task_item n=7 part=untouched
CLI verbs, exit codes, --json documents and human output lines: no verb, code or output shape changed; the send refusal input-not-ready keeps its meaning.
@task_item n=8 part=untouched
Occupied names: no binary, command, crate, environment variable or endpoint name was taken or freed.
@task_item n=9 part=untouched
Local sockets, socket directories, peer admission, strict-modes checks and pinned copies: unchanged.
@task_item n=10 part=untouched
The CI workflow, its jobs, action pins, coverage floors and pre-push gate: no step changed; the existing supply-chain job reads the dependency policy.
@task_item n=11 part=untouched
The harness prompt prefixes, a different set of compiled literals from the screen signatures.
