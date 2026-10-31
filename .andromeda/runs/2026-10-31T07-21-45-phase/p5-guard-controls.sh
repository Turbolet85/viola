#!/usr/bin/env bash
# The guard entry's known-positive control (validation.md check 4 (9)): fired as the entry over a scratch clone of HEAD.
set -u
root="$(git rev-parse --show-toplevel)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
chunk=viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces
git clone -q --shared "$root" "$scratch/clone"
mkdir -p "$scratch/clone/$chunk"
cp "$root/$chunk/plan.md" "$root/$chunk/research.md" "$root/$chunk/scope.md" "$scratch/clone/$chunk/"
fire() {
  echo "== $1 · clone HEAD $(git -C "$scratch/clone" rev-parse --short=12 HEAD)"
  python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan "$scratch/clone/$chunk/plan.md" --entry 15 --project-root "$scratch/clone" | grep -E '^ +15 |^entries'
}
fire "unmodified"
echo '// control' >> "$scratch/clone/src/cmd/client.rs"
echo '// control' >> "$scratch/clone/crates/viola-mcp/src/result.rs"
fire "two allowed files changed"
echo '// control' >> "$scratch/clone/src/cmd/mcp.rs"
echo '// control' >> "$scratch/clone/.config/nextest.toml.x"
echo '// control' > "$scratch/clone/crates/viola-mcp/src/new_file.rs"
fire "three paths outside the lists"
