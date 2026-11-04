@answer n=1 entity=stack-and-technologies#1.1 v=no
why: Cargo.toml, Cargo.lock and rust-toolchain.toml are unchanged, so the Rust pin and the rust-version floor stand.
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: no runtime or async crate changed, and the session adds only std-based test code.
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: clap features and the verb list are untouched, and no product source under src/ moved.
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: the product PTY seam and its windows-sys and libc uses are unchanged, and the Frozen guard's ps and debugger calls are test-side.
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: the witness reads raw viola list bytes rather than a vt100 screen, and the run and verify screen readers are unchanged.
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: no endpoint, socket or peer-credential reader changed, and the session adds no IPC.
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: no ndjson log, snapshot or audit store changed.
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: no record version or schema file moved, and diag-line.v1.json and diag-detail.v1.json are unchanged.
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: no product state-file helper or private-directory code changed.
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the sha2 uses in viola-state and the root known-answer test are unchanged.
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: no frame, log or snapshot format changed, and the new python script parses JUnit with the stdlib, not serde.
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: no tracing setup or log line changed.
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: no product newtype was touched.
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: no chrono use changed.
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: no file under src/ or crates/*/src/ changed, so the error types stand.
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: the session adds no broker, and the test-side freeze guard does not touch local sockets.
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: sysinfo and the heartbeat liveness rule are unchanged, and the witness only back-dates a heartbeat inside a test.
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the rmcp features, the MCP tool set and the viola-e2e client features are unchanged.
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: viola calls no model API, and the session adds none.
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: no mobile surface is in scope, and the session does not touch one.
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: no packaging or install path changed, and the plugin stays compiled into the binary.
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: .github/ is unchanged, the matrix and toolchain installs stand, and the session adds no CI step.
@answer n=23 entity=stack-and-technologies#1.23 v=no
why: the lint and quality tool list is unchanged, and the operator script is not a CI or lint stage.
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: the release tooling (dist, cargo-auditable, self_update) is unaffected by a test-side change.
@answer n=25 entity=stack-and-technologies#2.1 v=no
why: the peer-credential reads and the euid check are product code, and no product source changed.
