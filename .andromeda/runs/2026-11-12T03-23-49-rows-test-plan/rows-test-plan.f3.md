@row_add n=1 row=decision/unit-tests-inline area=2-test-strategy rec=1,2,3,4
name: Inline unit tests
rule: Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.
@row_add n=2 row=decision/integration-suite-per-contract area=2-test-strategy rec=5,6
name: Integration suite for every standard contract
rule: Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
@row_add n=3 row=decision/hook-deadlines-gated-by-hyperfine area=2-test-strategy rec=7,8
name: Hook deadlines gated by hyperfine
rule: hyperfine gates the hook deadlines on max.
@row_add n=4 row=decision/fixtures-schema-checked area=2-test-strategy rec=9,10
name: Recorded fixtures checked against their schema
rule: jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
@row_add n=5 row=decision/complete-lines-reader area=2-test-strategy rec=11,4
name: Live-appended files read by complete lines
rule: Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader.
why: a Linux reader can observe an append part-way
@row_add n=6 row=decision/integration-binary-surface-prefix area=2-test-strategy rec=12
name: Integration binaries prefixed by surface
rule: Rust integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer.
why: so nextest filtersets can select layers
@row_add n=7 row=decision/fixture-named-by-hook-event area=2-test-strategy rec=12
name: Fixtures named by hook event
rule: Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
@row_add n=8 row=decision/test-function-naming area=2-test-strategy rec=13,14
name: Test function naming
rule: Test functions are named <subject>_<condition>_<expected> in snake_case.
except: critical-path scenarios are named path<N>_<slug>
@row_add n=9 row=decision/nextest-test-runner area=4-unit-test-strategy rec=15,16
name: cargo-nextest runs the tests
rule: Tests run on libtest through cargo-nextest, process per test, with JUnit output.
except: Doctests run through cargo test, because nextest cannot run them.
@row_add n=10 row=absence/database area=5-integration-test-strategy rec=17,18
capability: A database
@row_add n=11 row=absence/outbound-http area=5-integration-test-strategy rec=19,20,21,22,23,24
capability: Outbound HTTP
@row_add n=12 row=decision/fake-agent-as-claude area=5-integration-test-strategy rec=19,22
name: Fake agent serves the claude boundary
rule: The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH.
@row_add n=13 row=decision/fresh-home-per-test area=5-integration-test-strategy rec=25,26,27,28
name: Fresh home per test
rule: Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests.
why: so obs-plan's CI gates, the secret scan and the upload cover integration runs
except: The chaos feed panic test alone boots outside the scans.
