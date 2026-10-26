#!/usr/bin/env bash
# P5 controls for the plan's census (entry 9) and preservation guard (entry 13), each fired AS THE ENTRY through the
# gate tool over a scratch clone of HEAD holding the chunk folder (validation.md check 4 (9)).
set -u
root=$(git rev-parse --show-toplevel)
tools=~/.claude/skills/andromeda-tools/scripts
chunk=viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT

fire() { # $1 label · $2 clone · $3 entry
  echo "## $1 · clone HEAD $(git -C "$2" rev-parse --short=12 HEAD)"
  python -X utf8 "$tools/gate.py" run --plan "$root/$chunk/plan.md" --entry "$3" --project-root "$2" 2>&1 | grep -E "^ +$3 "
}

clone() { # $1 name → a fresh clone of HEAD with the chunk folder copied in
  git clone -q --shared "$root" "$scratch/$1"
  mkdir -p "$scratch/$1/$chunk"
  cp -r "$root/$chunk/." "$scratch/$1/$chunk/"
  echo "$scratch/$1"
}

# Entry 9, the census: unmodified (must read as on the tree: red, 9), then the input step 4 builds (must pass: 1).
c=$(clone census)
fire "census unmodified" "$c" 9
for f in src/cmd/hook/statusline.rs src/cmd/run.rs src/run/gate.rs src/run/budget.rs src/run/send/test_support.rs src/run/dialog.rs src/obs.rs; do
  sed -i 's/tracing::subscriber::with_default(/crate::test_support::with_scoped_default(/' "$c/$f"
done
printf '%s\n' '// control: the helper body' 'fn helper() { tracing::subscriber::with_default(s, f) }' >> "$c/src/main.rs"
fire "census with the nine sites through the helper and one helper body (must pass)" "$c" 9

# Entry 13, the guard: unmodified (must read clean), an allowed file changed (must pass), then a file outside the
# lists edited and an untracked file minted (must fail).
g=$(clone guard)
fire "guard unmodified" "$g" 13
echo '// control' >> "$g/src/obs.rs"
fire "guard with an allowed file changed (must pass)" "$g" 13
echo '// control' >> "$g/src/human.rs"
echo '// control' > "$g/src/new_file.rs"
fire "guard with src/human.rs edited and an untracked src/new_file.rs (must fail)" "$g" 13
