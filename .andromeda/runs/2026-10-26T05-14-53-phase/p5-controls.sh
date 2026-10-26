#!/usr/bin/env bash
# P5 controls (validation.md check 4 (9)) for chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump.
# Each entry is fired AS THE ENTRY (gate.py run --entry n --project-root <clone>) over scratch clones of HEAD
# that hold the chunk folder:
#   A — unmodified: must read what the real tree read (6, 7, 8 red; 17 clean)
#   B — the plan's two steps applied (pin edit + cargo update -p tokio --precise 1.53.3): must-pass for 6, 7, 8, 9, 17
#   C — B plus out-of-scope changes (one more Cargo.lock line moved, a tracked source line edited, an untracked
#       source file): must-fail for 8 (count 4 broken) and 17 (guard: outside)
set -u
root=$(git rev-parse --show-toplevel)
marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
plan="$root/viola-0.1.0/chunks/$marker/plan.md"
T=~/.claude/skills/andromeda-tools/scripts
d=$(mktemp -d)
trap 'rm -rf "$d"' EXIT

mk() {
  git clone -q --shared "$root" "$d/$1"
  mkdir -p "$d/$1/viola-0.1.0/chunks"
  cp -r "$root/viola-0.1.0/chunks/$marker" "$d/$1/viola-0.1.0/chunks/"
}
bump() {
  sed -i 's/^tokio = { version = "=1.53.1", default-features = false }$/tokio = { version = "=1.53.3", default-features = false }/' "$d/$1/Cargo.toml"
  ( cd "$d/$1" && cargo update -q -p tokio --precise 1.53.3 )
}
fire() {
  echo "=== clone $1 · entry $2 · HEAD $(git -C "$d/$1" rev-parse --short=12 HEAD)"
  python -X utf8 $T/gate.py run --plan "$plan" --entry "$2" --project-root "$d/$1"
}

mk a
for n in 6 7 8 17; do fire a $n; done

mk b
bump b
for n in 6 7 8 9 17; do fire b $n; done

mk c
bump c
sed -i '0,/^checksum = /s/^checksum = "\([0-9a-f]\)/checksum = "0/' "$d/c/Cargo.lock"
echo '// p5 control edit' >> "$d/c/src/main.rs"
echo '// p5 control file' > "$d/c/src/p5_control.rs"
for n in 8 17; do fire c $n; done
