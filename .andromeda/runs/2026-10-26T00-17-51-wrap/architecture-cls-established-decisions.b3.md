# Classified — architecture · established-decisions.b3

## Answers
established-decisions#17.1 · no · the report shows the start order and liveness rule unchanged, with the reached src/cmd/run.rs and src/run/budget.rs changing in test code only
established-decisions#18.1 · no · the report shows the anyhow-only-in-the-bin rule and the catch-site reporter unchanged, with src/main.rs gaining only a #[cfg(test)] test_support module
established-decisions#19.1 · no · the report shows the workspace members, the flat crates/ layout and the viola-mcp-only tokio rule unchanged, with Cargo.toml gaining only a test-only dev-dependency table
established-decisions#20.1 · no · the report shows the plugin embedding and cargo install --path . unchanged, with no version bumped and no release build carrying the test-only features
established-decisions#21.1 · no · the report shows the plugin scope and --plugin-dir passing unchanged, with no plugin file or hook registration moved and src/cmd/hook/statusline.rs changing in test code only
established-decisions#22.1 · no · the report shows no output line, exit code or --json shape changed, with src/cmd/run.rs and src/cmd/hook/statusline.rs changing in test code only
