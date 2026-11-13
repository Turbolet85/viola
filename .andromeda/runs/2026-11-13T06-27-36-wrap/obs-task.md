@task_prose part=session
The session moved the readiness reading of the claude screen from footer literals alone to one predicate: the input box is up when no row holds a modal literal and either a footer literal stands or the box's own rows stand (a rule row of at least 40 box-drawing chars directly above a row opening the prompt char).
That predicate now decides verify's typed-run guards, the input-box-signature ledger row's check, and viola run's pre-send readiness gate, so the gate reads a screen ready on the box's rows alone; its quiet period, GATE_MAX_WAIT and poisoned arms are unchanged.
Recorded screen fixtures now keep a box row as its literal alone (the rule run, the bare prompt char); the fixture walk refuses any other form, and a recording refuses where the box's prompt row holds a home path, user word or email.
The ledger stamps envelope's measured.typed_probe gains box_only_guards (an integer or null), the count of verify guards that passed on the box alone; no version bump.
The supply-chain gate went green: cargo deny now ignores three rmcp advisories, each reasoned to a disabled or banned rmcp feature and standing until rmcp leaves its <3.5 range; a lockfile bump of rustix clears a fourth; no crate added.
The newest claude CLI, 2.1.323, was verified live and stamped (23 pass, 1 fail on statusline-rate-limits; 24 relied rows unchanged).
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures and the vt100 readiness gate's verdict over the claude screen (the footer, modal and now input-box literals, the input_box_up predicate)
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: its relied rows' checks (input-box-signature), the stamps envelope's measured fields, and the signature-rows-only recorded screen fixtures with their scrub fault check
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit tests (box-only screens, recorded box rows, the box prompt seam refusal, the box_only_guards stamp)
@task_file n=4 path=src/cmd/verify/typed/turns.rs
what: viola verify's typed run turn guards over the real CLI's screen, now counting guards passed on the box alone
@task_file n=5 path=src/cmd/verify/typed/pty.rs
what: viola verify's PTY probe settle and box-wait over the claude screen (ends on the input box or a modal literal)
@task_file n=6 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs (the trusted run and Run B's ready guard) that write the measured typed probe stamps
@task_file n=7 path=src/run/gate.rs
what: viola run's pre-send readiness gate (wait_ready) on a verified CLI; one test case added, no code changed
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene contract test that walks recorded screen fixtures and refuses any row that is not a signature row
@task_file n=9 path=deny.toml
what: the cargo deny configuration of the supply-chain gate (advisories ignore list, banned features)
@task_file n=10 path=Cargo.lock
what: the dependency lockfile (rustix bumped 1.1.5 to 1.1.6)
@task_file n=11 path=fuzz/fuzz_targets/vt100_feed.rs
what: the vt100 feed fuzz target that constructs the screen signatures
@task_item n=1 part=touches
The readiness gate's span and the send-refused input-not-ready detail: viola run's pre-send readiness gate now reads a screen ready on the input box's rows alone, so a row that says when the gate reads ready or refuses input-not-ready (decision/manual-readiness-gate-span, contract/send-span-attributes/run-readiness-gate, contract/send-refused-detail/input-not-ready) may be stale in what readiness means; the span, its attributes and the detail word themselves are unchanged.
@task_item n=2 part=touches
No telemetry from screen content (decision/no-telemetry-from-screen, decision/no-telemetry-from-screen-content): the screen is now read for two more compiled literals, the box's rule and prompt chars; still compiled signatures only and nothing of a row is logged, a row saying the screen is read only for signature literals stays true but may name the literal set.
@task_item n=3 part=touches
Recorded fixtures are no telemetry source and claude fixtures are scrub-required (decision/fixtures-not-telemetry, decision/fixtures-not-a-telemetry-source, decision/claude-fixtures-scrub-required): the recorded screen form now keeps a box row as its literal alone, and a recording refuses where the box's prompt row holds a home path, user word or email.
@task_item n=4 part=touches
The capability ledger and its stamps (decision/ledger-contents-never-logged, decision/verify-sole-stamps-writer, contract/run-log-parse-rejected/ledger-stamps): the stamps envelope's measured.typed_probe gains box_only_guards, written by verify alone; run reads no field of it, so a parse-rejected ledger-stamps line has no new cause.
@task_item n=5 part=touches
viola verify's lines and stdout (decision/verify-lines-redacted, decision/verify-lines-carry-no-content, contract/verify-stdout/step-line, contract/verify-stdout/summary-line): verify's typed-run guards now read the box predicate and count box-only passes; the step and summary lines and the relied count 24 are unchanged, so any row holding that count stays true.
@task_item n=6 part=touches
cargo deny gates (decision/obs-deps-pass-cargo-deny, decision/cargo-deny-failure-fails, decision/banned-telemetry-dependencies): the advisories ignore list goes from empty to three reasoned rmcp ignores; a row that says obs dependencies pass cargo deny with no ignore, or that the deny configuration ignores nothing, is touched.
@task_item n=7 part=touches
vt100 panic containment and the fuzz target (decision/vt100-panics-contained, decision/vt100-feed-panic-contained): the screen signatures type the fuzz target builds gained two fields; the feed and its containment are unchanged.
@task_item n=8 part=touches
Real-CLI verify never in CI and CI verify against the fake agent only (decision/real-cli-verify-never-in-ci, decision/ci-verify-fake-agent-only): the live verify on claude 2.1.323 ran by hand on the dev host, and no box-only screen fixture replayed by the fake agent was added; both rows stay true.
@task_item n=9 part=touches
Wrapped claude internals are not a contract (decision/claude-internals-not-a-contract, decision/transcripts-not-a-contract): readiness now leans on the input box's drawn rule and prompt chars, another rendering of the claude screen read as a version-stamped signature, not a contract.
@task_item n=10 part=untouched
The diagnostics line schema, its event enum, field names, detail codes and corr rules: no event, field, detail word or exit code was added, renamed or removed.
@task_item n=11 part=untouched
The hook, MCP, channel, dialog, wheel and budget paths with their spans and log fields: none of them changed.
@task_item n=12 part=untouched
Panic capture, error reporting, the NEVER-log floor and the absences of exporters, reporters and network egress: no telemetry dependency or sink was added; the only dependency move is a lockfile bump of a non-telemetry crate.
@task_item n=13 part=untouched
Perf budgets, hyperfine gates, heartbeat liveness, log file locations, rotation and service identity: untouched.
@task_item n=14 part=untouched
Secret scan, artifact upload and retention rows in CI: no CI step, job or upload changed; the supply-chain job reads the existing deny configuration.
