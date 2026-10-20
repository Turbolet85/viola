#!/usr/bin/env bash
# P5 controls for chunk 2026-10-20-epoch-5-cleanup-ii (validation.md check 4 (9)): each entry fired AS THE ENTRY over
# a scratch clone of HEAD, first unmodified, then with the control's input. Runs from the project root.
set -uo pipefail
root=$(git rev-parse --show-toplevel)
plan="$root/viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/plan.md"
gate=(python -X utf8 "$HOME/.claude/skills/andromeda-tools/scripts/gate.py")
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

fresh() {
  local c="$work/$1"
  git clone -q --shared "$root" "$c"
  printf '%s' "$c"
}

fire() {
  local n=$1 clone=$2 label=$3
  echo "== entry $n · $label · clone HEAD $(git -C "$clone" rev-parse --short=12 HEAD)"
  "${gate[@]}" run --plan "$plan" --entry "$n" --project-root "$clone" | grep -E "^ *$n |^entries"
}

# 11 — the split files under 800: unmodified reads 2, both files cut under 800 reads 0.
c=$(fresh e11)
fire 11 "$c" "unmodified"
head -100 "$c/src/cmd/verify/typed.rs" > "$work/typed.rs" && cp "$work/typed.rs" "$c/src/cmd/verify/typed.rs"
head -100 "$c/src/human/board.rs" > "$work/board.rs" && cp "$work/board.rs" "$c/src/human/board.rs"
fire 11 "$c" "must-pass: typed.rs and board.rs cut to 100 lines"

# 12 — no inline test module in ledger.rs: unmodified red, the module line replaced green.
c=$(fresh e12)
fire 12 "$c" "unmodified"
sed -i 's/^mod tests {$/mod tests;/' "$c/crates/viola-agent-claude/src/ledger.rs"
fire 12 "$c" "must-pass: the module line replaced by mod tests;"

# 13 — the refuse census: unmodified green, one call added red.
c=$(fresh e13)
fire 13 "$c" "unmodified"
printf '%s\n' '// human::refuse(' >> "$c/src/main.rs"
fire 13 "$c" "known-positive: one more human::refuse( in src/main.rs"

# 14 — relied_here gone: unmodified red, renamed green.
c=$(fresh e14)
fire 14 "$c" "unmodified"
sed -i 's/relied_here/relied_x/g' "$c/crates/viola-agent-claude/src/ledger.rs"
fire 14 "$c" "must-pass: relied_here renamed in ledger.rs"

# 19 — machete over viola-mcp: unmodified red, the serde edge dropped green.
c=$(fresh e19)
fire 19 "$c" "unmodified"
sed -i '/^serde.workspace = true$/d' "$c/crates/viola-mcp/Cargo.toml"
fire 19 "$c" "must-pass: serde.workspace dropped from crates/viola-mcp/Cargo.toml"

# 20 — the guard: unmodified clean, an allowed file changed clean, an outside file and an untracked file red.
c=$(fresh e20a)
mkdir -p "$c/viola-0.1.0/chunks"
cp -R "$root/viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii" "$c/viola-0.1.0/chunks/"
fire 20 "$c" "unmodified"
printf '%s\n' '// control' >> "$c/crates/viola-agent-claude/src/ledger.rs"
fire 20 "$c" "an allowed file changed (ledger.rs)"
c=$(fresh e20b)
mkdir -p "$c/viola-0.1.0/chunks"
cp -R "$root/viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii" "$c/viola-0.1.0/chunks/"
printf '%s\n' '// control' >> "$c/src/main.rs"
printf '%s\n' '// control' > "$c/src/new_file.rs"
fire 20 "$c" "must-fail: src/main.rs edited and an untracked src/new_file.rs"
