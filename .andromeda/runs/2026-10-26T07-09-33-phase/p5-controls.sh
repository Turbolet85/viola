#!/usr/bin/env bash
# P5 controls for the plan's preservation guard (entry 13), fired AS THE ENTRY through the gate tool over a scratch
# clone of HEAD holding the chunk folder (validation.md check 4 (9)).
set -u
root=$(git rev-parse --show-toplevel)
tools=~/.claude/skills/andromeda-tools/scripts
chunk=viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT

fire() { # $1 label · $2 clone · $3 entry
  echo "## $1 · clone HEAD $(git -C "$2" rev-parse --short=12 HEAD)"
  python -X utf8 "$tools/gate.py" run --plan "$root/$chunk/plan.md" --entry "$3" --project-root "$2" 2>&1
}

git clone -q --shared "$root" "$scratch/guard"
g="$scratch/guard"
mkdir -p "$g/$chunk"
cp -r "$root/$chunk/." "$g/$chunk/"

# unmodified (must read clean), an allowed file changed and the new test file minted (must pass), then a file
# outside the lists edited and an untracked file minted (must fail).
fire "guard unmodified" "$g" 13
echo '// control' >> "$g/src/cmd/hook.rs"
echo '// control' > "$g/tests/security_negatives_server_identity.rs"
fire "guard with src/cmd/hook.rs changed and the new test file minted (must pass)" "$g" 13
echo '// control' >> "$g/src/cmd/run.rs"
echo '// control' > "$g/src/new_file.rs"
fire "guard with src/cmd/run.rs edited and an untracked src/new_file.rs (must fail)" "$g" 13
