group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:49   class: omission
  leaf-text: «(nothing linked, the target std installed as host preparation; neither a CI step nor a `pre-push` stage).»
  source: .andromeda/test-plan-detail/ci-integration.md:11 «Beside it, a check-only macOS-target clippy is a recipe on the Linux dev host (`CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`»
  replace-with: «(nothing linked, the target std installed as host preparation; neither a CI step nor a `pre-push` stage). Beside it, a check-only macOS-target clippy is a recipe on the Linux dev host: `CARGO_TARGET_DIR=target/maccheck cargo clippy --workspace --all-targets --features fake-agent --target aarch64-apple-darwin -- -D warnings` (nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`; it reproduced CI's `lint (macos-latest)` error line for line; neither a CI step, nor a `pre-push` stage, nor a plan gate entry).»
