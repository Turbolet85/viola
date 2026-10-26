# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · clippy -D warnings and the orphans gate rule over LinkSet::remove, whose dead_code allow the chunk drops, and over the new verbs and handlers that must stay reachable
ci-integration#1.2 · no · the chunk adds no crate, feature, manifest, lockfile, deny config or workflow, and the tokio ban and channel tree form rule over viola-channel's normal graph, which it leaves unchanged
ci-integration#1.3 · yes · the root-bin unit cases the scope names (refusal ordering, the two verbs' exit-code mapping, LinkSet::remove beside its product caller) run in this process-per-test layer
ci-integration#1.4 · yes · the new root integration cases (tests/channel_links.rs, tests/cli_links.rs, the HELP literal in tests/cli_output_plain.rs, the raw-channel name and server-identity negatives) run in this layer
ci-integration#1.5 · yes · Scenario E3 steps 1 and 3, which the chunk builds, run as E2E on all three OSes, where the identity check's Linux and Windows branches are verified
ci-integration#1.6 · no · standing S10 · coverage gates held per OS, a convention the task lists
ci-integration#1.7 · no · the mutation gate runs only at the epoch boundary and on demand, never per chunk
ci-integration#1.8 · yes · the G2 zero-panic check run in this job rules over the new product code and the edited LinkSet::remove (seam.rs is its only exemption), while the hook perf budgets are untouched
ci-integration#1.9 · yes · the new code and its unit cases must compile and pass on the 1.96 floor (cargo check --workspace and run --unit under RUSTUP_TOOLCHAIN=1.96)
ci-integration#1.10 · no · the replay covers existing fuzz corpora only, and the chunk adds no fuzz target, corpus or parser surface (nothing fuzzed changes)
ci-integration#1.11 · no · the chunk adds no binary, test-only feature or artifact, and the release-check judges only artifact records
ci-integration#1.12 · no · standing S10 · the §10 breach gate's zero-flakiness and coverage-per-OS rule, a convention the task lists
