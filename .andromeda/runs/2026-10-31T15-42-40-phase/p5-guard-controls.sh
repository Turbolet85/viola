#!/usr/bin/env bash
# The guard entry's known-positive control (validation.md check 4 (9)): fired as the entry over a scratch clone of HEAD.
set -u
root="$(git rev-parse --show-toplevel)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
chunk=viola-0.1.0/chunks/2026-10-31-exit-cause-code-catalogue
git clone -q --shared "$root" "$scratch/clone"
mkdir -p "$scratch/clone/$chunk"
cp "$root/$chunk/plan.md" "$root/$chunk/research.md" "$root/$chunk/scope.md" "$scratch/clone/$chunk/"
fire() {
  echo "== $1 · clone HEAD $(git -C "$scratch/clone" rev-parse --short=12 HEAD)"
  python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan "$scratch/clone/$chunk/plan.md" --entry 13 --project-root "$scratch/clone"
}
fire "unmodified"
# Every path research's two lists hold, so the --allow pattern is read against the whole write set.
for f in \
  Cargo.toml .config/nextest.toml schemas/diag-line.v1.json \
  crates/viola-core/src/lib.rs crates/viola-state/src/liveness.rs \
  src/main.rs src/human/hints.rs \
  src/cmd/mod.rs src/cmd/client.rs src/cmd/run.rs src/cmd/revive.rs src/cmd/verify.rs \
  src/cmd/send.rs src/cmd/wait.rs src/cmd/last.rs src/cmd/answer.rs src/cmd/pause.rs src/cmd/link.rs \
  src/cmd/list.rs src/cmd/plugin.rs \
  crates/viola-mcp/src/result.rs crates/viola-mcp/src/tools.rs \
  crates/viola-mcp/tests/handlers.rs crates/viola-e2e/tests/mcp_tools.rs \
  tests/support/start.rs tests/chaos_wait_vanish.rs tests/cli_send.rs tests/cli_list.rs \
  tests/cli_machine_contract.rs tests/cli_wait_last.rs tests/cli_controls_not_disableable.rs \
  tests/cli_plugin_install.rs tests/cli_links.rs tests/cli_error_bodies.rs tests/cli_instance_state.rs \
  tests/cli_revive.rs tests/cli_verify.rs tests/security_negatives_server_identity.rs \
  tests/security_negatives_strict_modes.rs tests/contract_error_bodies.rs; do
  echo '# control' >> "$scratch/clone/$f"
done
echo '// control' > "$scratch/clone/tests/cross_exit_causes.rs"
fire "the 40 modified paths and the new file of research's lists changed"
echo '// control' >> "$scratch/clone/src/cmd/mcp.rs"
echo '// control' >> "$scratch/clone/tests/support/fake.rs"
echo '# control' >> "$scratch/clone/Cargo.lock"
echo '// control' > "$scratch/clone/crates/viola-channel/src/new_file.rs"
fire "four paths outside the lists: src/cmd/mcp.rs, tests/support/fake.rs, Cargo.lock, an untracked crates/viola-channel/src/new_file.rs"
