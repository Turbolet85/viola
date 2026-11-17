@task_prose part=chunk
First step: the shared stamped test home is valid, or rebuilt, on any host where its home is absent, whatever a cache restored under target/; a test fails with a stamp marker present and its home absent, and passes after the fix.
Then: a `viola run` wrapper a test boots, and the fake agent under it, end when that test ends, whether it passed, failed, timed out or was SIGKILLed; after a test run no booted wrapper or fake agent is left running (requirement v2-37).
A proof of killed: a harness the test owns SIGKILLs a test process mid-run on the fake agent, then a census at its end shows nothing of that test's own process tree still running; no test kills a process it does not own.
P3 re-measures from the code every place a test boots a wrapper, what ends it today on each exit path, and why a killed test's wrapper spun; it re-reads the CI red ci#44048204364 (shared stamped home absent) and tests the cache-restored stamp-marker hypothesis, closed only by a green CI run on an exact cache hit.
Fixtures and the fake agent only; a change to `viola run` so it ends once its PTY or parent is gone is in scope only as P3 measures it; no live claude start, no mutation run, the standing live rig not reworked.
@task_file n=1 path=tests/support/home.rs mark=edits
what: the shared stamped test home of the integration tests, the fixture home every booted test reads — edits: stamp_dir, stamped_home
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the E2E harness boot of a `viola run` wrapper over the shared stamped home, its fixture lifecycle and the end of the wrapper it boots
@task_file n=3 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent the booted wrapper drives in tests and CI in place of the real claude CLI
@task_item n=1 part=needs
The shared stamped fixture home: a stamp marker under target/ is trusted only when its home (under the runner's /tmp through the target/e2e-home link) is present; otherwise the home is rebuilt and re-stamped, and a test proves marker-present home-absent fails before and passes after.
@task_item n=2 part=needs
Integration-test homes stay under the one e2e-home root, under the root the secret scan reads, and are kept until the gate steps run; the rebuilt home must keep that placement.
@task_item n=3 part=needs
Every booted wrapper and its fake agent end when the test that booted it ends, on pass, failure, timeout and SIGKILL, a kill which runs no destructor; the end cannot rest on a drop alone.
@task_item n=4 part=needs
The killed-test proof: a test-owned harness SIGKILLs a test process mid-run on the fake agent, and an end-of-run census over that test's own process tree finds nothing still running; no kill is sent to a process the test does not own.
@task_item n=5 part=needs
If P3 measures a `viola run` change so the wrapper ends once its PTY or parent is gone (it spun at near a full core once its test was gone): its process-exit line carries a closed detail code distinct per exit cause, no path or pid, and its claude child's exit is read from the process handle.
@task_item n=6 part=needs
If the wrapper's end path touches the PTY seam's kill-fallback or the claude-child process-start and process-exit lines, those lines keep their fields and events; no new free-form log line, and no info line inside the loop the wrapper spins in.
@task_item n=7 part=needs
The heartbeat file stays the liveness signal and is touched, not logged; a wrapper that ends stops ticking it, and its liveness transitions write no line.
@task_item n=8 part=needs
CI verify runs only against the fake agent; the real-CLI verify never runs in CI and no test here starts a live claude.
@task_item n=9 part=needs
A change to the fake agent's screens puts cli_verify in the gate block; recorded fake-agent fixtures are no telemetry source.
@task_item n=10 part=needs
Test waits on a channel, a thread or a process end take a bound under the project's 20 s lint and cannot hang past a failure; no sleep-based waits for a log line or a process end; a test socket goes under the short /tmp root.
@task_item n=11 part=needs
The CI red is closed by its cause and by a green run on an exact cache hit; a green re-run is no substitute, and no retry-once or retry policy is added to the test or the fixture.
@task_item n=12 part=needs
Each control names its timeout and fires filtered to its own cases; each acceptance predicate can fail, the plan says what change turns it red, and each filter selects at least one test.
@task_item n=13 part=needs
The OS-gated test census holds for any new test the chunk adds (the SIGKILL and process-tree census proof is Linux-only).
@task_item n=14 part=needs
The zero-panics and schema-conformance gates still read the test homes after the tests, failed or killed ones included, and every line a booted wrapper writes stays schema-conformant.
@task_standing n=1
convention: each new integration or E2E test's home lives under the one e2e-home root, under the secret scan's root, kept until the gates run
home: 9-ci-integration · ci-integration
@task_standing n=2
convention: each line a new test's booted process emits is schema-checked, and the panic and schema gates run even after failed tests
home: ci-integration
@task_standing n=3
convention: each new test waits by bound and exit code, never by sleep, and adds no retry; budgets and verdicts asserted by exit code
home: 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
@task_standing n=4
convention: any product line the chunk adds goes through the obs event macro with closed event values, no print macros and no raw tracing macros
home: 11-obs-anti-patterns-never-do-these
@task_item n=1 part=not_needed
MCP stdio server tools, frames, isError results and stdin MAX_FRAME rows: the chunk adds or changes no MCP surface.
@task_item n=2 part=not_needed
Hook stdout, hook decision lines, hook fail-open and hook latency budgets: no hook path is touched.
@task_item n=3 part=not_needed
Send, wait, dialog, wheel and link span chains, their span attributes and their log field rows: the chunk changes no driver verb.
@task_item n=4 part=not_needed
Exit-1 and exit-21 detail codes of CLI frame verbs, revive preflight and the cli --json refusal documents: no CLI verb's refusal changes.
@task_item n=5 part=not_needed
Future error reporter, Sentry, OTel SDK, exporters, traceparent and network egress rows: nothing here exports telemetry.
@task_item n=6 part=not_needed
Detail-file line schema, secret-scan hit report fields and content-redaction rows (veil, allow-list): the chunk adds no content-bearing record.
@task_item n=7 part=not_needed
Derived metrics, quantiles, histograms and the hook perf jobs: no metric or perf budget is added or changed.
@task_item n=8 part=not_needed
State-recovered, snapshot replay, events.ndjson event shapes and config.json keys: no state or config format changes.
