@task_prose part=chunk
A test-only corrective chunk: it makes a green coverage suite and a green viola mutation baseline reachable for the next boundary's audit, and runs neither (no mutation run, no coverage run, no live claude start, no live desktop step).
It gives the hook fail-open test's channel endpoint a socket name that does not grow with the pid and the event label, and proves the path stays inside the 107-byte socket path bound under the mutation scratch with a 7-digit pid.
It widens the nextest mutants profile's slow-case kill (kept under cargo-mutants' 20 s auto-timeout floor) and moves the shared receipt wait bound with it, re-measuring the slow cases' wall times with the host's load beside them.
It kills the ten surviving mutants in the test-only Hyprland recorder tap with tests that fail on each mutated site (each proved by a remove-the-guard control under a kill deadline), or states in the test-plan why the recorder's main is outside the tier.
It makes the rig's test-home helper enforce that a home given to viola verify --record never holds a path component named home.
@task_file n=1 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open integration tests and the test channel's endpoint socket they bind under a temp dir — edits: the socket name build, its join and its bind
@task_file n=2 path=.config/nextest.toml mark=edits
what: the nextest test-runner config, its mutants profile used by mutation testing at the epoch audit — edits: the mutants profile's slow-timeout kill and any per-test override for the slow-case class
@task_file n=3 path=tests/support/watch.rs mark=edits
what: the test support receipt watcher and its shared wait bound for fake-agent receipts — edits: the WITHIN wait constant and its doc comment, the wait_for bound
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only Hyprland recorder tap binary that records fixtures, keeping reply strings by a closed allowlist — edits: its tests over main's argument guard and its record and record-dispatch arms, version_of's version arm and texts' array arm
@task_file n=5 path=tests/support/home.rs mark=edits
what: the test-side home helper every test takes its own viola home from — edits: TestHome::new, the home() fixture and scratch(), as the home-naming rule's enforcement point
@task_file n=6 path=tests/cli_verify/runs.rs mark=re-reads
what: the viola verify --record run tests that take their home from the test-home helper, and the Run E resume-run case red on the receipt wait
@task_file n=7 path=tests/cli_verify/stamps.rs mark=re-reads
what: the viola verify --record stamps tests that take their home from the test-home helper
@task_item n=1 part=needs
The test channel endpoint is a Unix socket bound under the test's own temp directory: the new name must keep it in that per-test directory (never at the temp root), stay a path-based socket (no abstract-namespace name to dodge the length bound) and bind without overwriting an existing socket file.
@task_item n=2 part=needs
The socket path bound itself: the test proves, computed or asserted, that the full endpoint path under the mutation scratch with a 7-digit pid and the longest event label fits the 107-byte socket path limit.
@task_item n=3 part=needs
The hook fail-open tests keep exercising the hook failing open to the human when the channel is unreachable; only the endpoint's name changes, the checks they make stay as they are.
@task_item n=4 part=needs
Mutation testing at the epoch audit: the mutants profile's kill is resized for the audit's baseline run, staying below cargo-mutants' 20 s auto-timeout floor; the chunk runs no mutation itself and adds no mutants.out upload.
@task_item n=5 part=needs
The shared fake-agent receipt wait bound moves with the mutants kill and stays strictly below it; every wait_for caller shares it, so a per-call or scaled bound is weighed for the narrower change.
@task_item n=6 part=needs
The Hyprland recorder tap keeps strings only by its closed allowlist and drops reply text at the parse: the new tests over texts' array arm and version_of's version arm hold that behaviour on the mutated sites.
@task_item n=7 part=needs
The recorder tap is a test-only binary that records fixtures for the e2e tests: its main is touched only if a test proves a recorder line wrong, and the release build still refuses test-only binaries.
@task_item n=8 part=needs
Recorded fixture hygiene: a home given to viola verify --record never holds a path component named home, enforced by the test-home helper so a test cannot name it wrong; it ties to verify's unclean-fixture refusal for a home path.
@task_item n=9 part=needs
Each survivor's proof is a one-off remove-the-guard control run under a kill deadline, its case the test's name alone, holding only when that case ran and failed.
@task_item n=10 part=needs
Test-only boundary: no product crate line changes unless a test proves one wrong; test seams stay under the fake-agent feature and test-support stays a development-only feature.
@task_item n=11 part=needs
No live claude start and no live desktop step: the real claude CLI stays local-only and outside CI, and CI holds no secrets.
@task_item n=12 part=needs
A red or defect found now folds into this chunk as an inferred item with its own acceptance; behaviour of nextest's profile inheritance, the slow-timeout form and tempfile's use of TMPDIR is measured, not assumed.
@task_standing n=1
convention: tests use their own viola home, never the user's real home or the Claude home
home: secret-management
@task_standing n=2
convention: test-side removal touches only directories the test itself created
home: security-anti-patterns
@task_standing n=3
convention: test homes and scratch directories are kept out of git
home: bootstrap-phases
@task_standing n=4
convention: test seams live only under the fake-agent feature and are compiled out of release
home: security-anti-patterns
@task_standing n=5
convention: any fixture a new test records is scrubbed or refused whole, and reviewed before commit
home: data-protection
@task_item n=1 part=not_needed
MCP tool contracts, the MCP stdin line cap and the MCP isError documents: the chunk adds no MCP tool and changes no MCP reply.
@task_item n=2 part=not_needed
The CLI exit codes and the strict-modes refusal contracts on each surface: no product refusal or exit code changes.
@task_item n=3 part=not_needed
Paste and dialog free-text character sets, send and answer refusal words: no typed-text path is touched.
@task_item n=4 part=not_needed
revive --json codes and revive argument contracts, collision-check codes: the chunk changes no revive or start behaviour.
@task_item n=5 part=not_needed
rmcp feature set and banned features, the API security area: no dependency feature changes.
@task_item n=6 part=not_needed
Release, attestation, signed self-update and CVE response-time decisions: no release stage is touched.
@task_item n=7 part=not_needed
Event log rotation, link-line contracts and the wrapper's link and unlink replies: no event log line is added or changed.
@task_item n=8 part=not_needed
Terminal colour facts, human-output escaping and screen-model feed rules: no output rendering is touched.
@task_item n=9 part=not_needed
The absences (hosting, public listener, outbound network, accounts, database, secrets manager): nothing the chunk adds reaches them.
