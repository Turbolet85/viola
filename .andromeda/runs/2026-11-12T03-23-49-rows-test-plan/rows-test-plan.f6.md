@row_add n=1 row=decision/single-writer-state-files area=6-e2e-test-strategy rec=1,2
name: Single writers of snapshot and stamps
rule: Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
@row_add n=2 row=decision/frame-verbs-reach-only-the-recorded-wrapper area=6-e2e-test-strategy rec=1
name: Frame verbs reach only the recorded wrapper
rule: A CLI frame verb writes no frame to a server that is not the recorded wrapper.
guard: security_negatives_server_identity
@row_add n=3 row=contract/list-json/name area=6-e2e-test-strategy rec=4,5
boundary: list --json
item: name
promise: list --json carries a session's original name string, escaped as JSON, never the escaped form the table prints.
@row_add n=4 row=decision/property-tests-on-parsers area=6-e2e-test-strategy rec=6,7
name: Property tests on every parser surface
rule: proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline and agents parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
@row_add n=5 row=decision/fixtures-match-tolerant-schema area=6-e2e-test-strategy rec=8,9
name: Fixtures checked against the tolerant schema
rule: jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
@row_add n=6 row=decision/bounded-readers area=6-e2e-test-strategy rec=8,9
name: Every external reader is bounded
rule: Every product reader of external input is bounded in-statement or named in the enumeration with a witness test.
except: Reads of viola's own executable take no external input and are listed as exceptions.
guard: product_readers_are_each_bounded_or_named
@row_add n=7 row=decision/test-homes-made-private area=7-test-data-fixtures rec=10,11,12
name: Test-made homes are private directories
rule: A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation.
why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
@row_add n=8 row=decision/mcp-exchange-recorded-by-tap area=7-test-data-fixtures rec=13,14
name: MCP exchange recorded by a test-only tap
rule: The recorded MCP exchange is taken by the test-only viola-mcp-tap through the same scrub and refusal, its raw capture 0600 and never committed, and the kept fixture is schema-validated and replayed.
@row_add n=9 row=contract/fake-agent-receipt/env area=7-test-data-fixtures rec=15,16,17
boundary: viola-fake-agent receipt
item: env
promise: An env receipt holds variable names, sorted, names only, never a value.
@row_add n=10 row=decision/mockall-on-seam-traits-only area=8-mocking-stubbing-discipline rec=18,19,20
name: mockall doubles on seam traits only
rule: Test doubles are mockall automock, under test only, on traits viola already defines at its seams, with dependency injection preferred over runtime patching of production code; external processes are real stub executables.
@row_add n=11 row=decision/no-orphan-modules area=9-ci-integration rec=21,22,23
name: No orphan modules
rule: Any cargo-modules orphan in any library or binary target fails the build.
guard: orphans-check.sh with its probe of two planted orphans
@row_add n=12 row=decision/mutation-gate-thresholds area=10-quality-gates-coverage-targets rec=24,25
name: Mutation gate thresholds
rule: A mutation run passes only with missed == 0, timeout == 0 and unviable <= caught, missed read after host-excluded records are left out.
why: A run whose mutant builds mostly failed is no evidence of test strength; the unviable threshold was picked by measurement.
@row_add n=13 row=decision/every-surface-asserted area=11-test-anti-patterns-never-do-these rec=26
name: Every surface of a critical path asserted
rule: A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
