# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · test-only, no product behaviour moved (the obs tests moved only a capture call and src/main.rs gained only a test helper), and the new dev-dependency features reach no release build under resolver 3, so the hook's line and raw backtrace capture still hold
error-capture-reporting#2.1 · no · test-only, no product behaviour moved (src/main.rs gained only a test helper and witness, and the channel-response shape is unchanged), and the Cargo.toml edit is only a dev-dependency table, so main's catch_unwind, the role and worker catch sites and panic = "unwind" still hold
error-capture-reporting#3.1 · no · test-only, no product behaviour moved (the run-start and statusline modules moved only their test capture calls), so the clap and home window and the instance check ahead of any role file still hold
error-capture-reporting#4.1 · no · the session's touch of src/run/gate.rs is the unit test's capture, whose red under plain cargo test is recorded with no owner, while the feed catch and the one-parse-rejected-per-episode product claim are unmoved and G2 is green with no gate command changed
