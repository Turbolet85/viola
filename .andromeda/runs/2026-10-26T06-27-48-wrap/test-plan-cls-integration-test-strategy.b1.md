# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · rstest 0.27.0 still holds on the changed Cargo.lock, which moved only its tokio entry
integration-test-strategy#1.2 · no · tempfile 3.27.0 still holds on the changed Cargo.lock, which moved only its tokio entry
integration-test-strategy#1.3 · no · no HTTP mocking library still applies on the changed Cargo.lock, since the lock delta adds no crate, and the fake-agent harness is unchanged
integration-test-strategy#1.4 · no · interprocess 2.4.4, jsonschema 0.57.0 and windows-sys 0.61.2 still hold on the changed Cargo.lock, which the report names unmoved
integration-test-strategy#1.5 · no · portable-pty =0.8.1 still holds on the changed Cargo.toml and Cargo.lock, whose only moves are the tokio pin and its entry
integration-test-strategy#1.6 · no · assert_cmd 2.2.2 still holds on the changed Cargo.lock, which moved only its tokio entry
integration-test-strategy#2.0 · no · its tempfile and rstest uses hold on the changed Cargo.lock, and the fresh-home and seed contract is unchanged by the tokio-only pin and lock move
integration-test-strategy#3.1 · no · the channel wire, its tests and the rmcp client crate are unchanged by the tokio-only pin and lock move
integration-test-strategy#3.2 · no · it names no dependency or manifest row, and the on-disk suite is unchanged by the tokio-only pin and lock move
integration-test-strategy#4.1 · no · the `[[test]]` and `required-features` rows of the changed root Cargo.toml still hold, its only move being the tokio pin, and assert_cmd and trycmd are unmoved
