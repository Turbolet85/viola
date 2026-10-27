#!/usr/bin/env bash
# Measurement B (plan step 9): two clean release builds of one tree, compared by file and by section.
# Run from the repository root:  bash viola-0.1.0/chunks/<marker>/evidence/repro-builds.sh
# Both builds go through scripts/release-check.sh into one same target path the script makes empty
# (by mkdir) before each build; the one path it removes is that one, which it made. Then the two copies
# are compared, and two controls run through the same comparison: a copy against itself (0 unequal)
# and a copy against another host program (all unequal).
set -euo pipefail

scratch=$(mktemp -d "${TMPDIR:-/tmp}/viola-repro-XXXXXX")
trap 'rm -rf -- "$scratch"' EXIT
target="$scratch/target"

echo "toolchain: $(rustc --version) · $(cargo --version) · $(objcopy --version | head -n 1) · $(readelf --version | head -n 1)"
echo "tree: HEAD $(git rev-parse HEAD) · uncommitted $(git status --porcelain | wc -l)"

for i in 1 2; do
  mkdir "$target"
  start=$(date -u +%FT%TZ)
  verdict=$(CARGO_TARGET_DIR="$target" bash scripts/release-check.sh 2>"$scratch/build-$i.err" | tail -n 1)
  echo "build $i: $start .. $(date -u +%FT%TZ) · $verdict"
  if [ "$verdict" != "release-check: viola only" ]; then
    echo "build $i: FAILED — stderr tail:"
    tail -n 20 "$scratch/build-$i.err"
    exit 1
  fi
  cp "$target/release/viola" "$scratch/build-$i"
  rm -rf -- "$target"
done

# One line per section: its name and type (the NULL entry left out).
sections() {
  readelf -S -W "$1" | awk '/^ *\[ *[0-9]+\]/ { sub(/^ *\[ *[0-9]+\] */, ""); split($0, f, " "); if (f[1] != "NULL") print f[1], f[2] }'
}

# A section's bytes hashed whole, loaded or not: `--only-section` with `-O binary` writes loaded
# sections only and `--dump-section` refuses `.symtab` / `.strtab`, so every section is read from the
# file at its offset and size as `readelf -S` gives them; a NOBITS section holds no bytes and reads as
# its size. A name the file lacks reads `absent`.
section_sha() {
  local row type off size
  row=$(readelf -S -W "$1" | awk -v n="$2" '/^ *\[ *[0-9]+\]/ { sub(/^ *\[ *[0-9]+\] */, ""); if ($1 == n) { print $2, $4, $5; exit } }')
  if [ -z "$row" ]; then
    echo "absent"
    return
  fi
  read -r type off size <<< "$row"
  if [ "$type" = "NOBITS" ]; then
    echo "nobits-size-$size"
  else
    dd if="$1" bs=65536 iflag=skip_bytes,count_bytes skip=$((16#$off)) count=$((16#$size)) status=none | sha256sum | cut -c1-16
  fi
}

compare() {
  local label=$1 a=$2 b=$3 unequal=0 total=0 name ha hb
  echo "== $label"
  echo "file: a sha256 $(sha256sum "$a" | cut -c1-16) size $(stat -c %s "$a") · b sha256 $(sha256sum "$b" | cut -c1-16) size $(stat -c %s "$b")"
  echo "build-id: a $(readelf -n "$a" | awk '/Build ID/ {print $3}') · b $(readelf -n "$b" | awk '/Build ID/ {print $3}')"
  while IFS= read -r name; do
    total=$((total + 1))
    ha=$(section_sha "$a" "$name")
    hb=$(section_sha "$b" "$name")
    if [ "$ha" = "$hb" ] && [ "$ha" != "absent" ]; then
      echo "section $name: equal $ha"
    else
      unequal=$((unequal + 1))
      echo "section $name: UNEQUAL a $ha b $hb"
    fi
  done < <( { sections "$a"; sections "$b"; } | awk '!seen[$1]++ { print $1 }')
  echo "sections: $total named · $unequal unequal"
  echo "strings: $(diff <(strings -a "$a") <(strings -a "$b") | grep -c '^[<>]' || true) differing lines"
}

compare "build-1 vs build-2" "$scratch/build-1" "$scratch/build-2"
echo "the differing strings lines, build-1 (<) against build-2 (>), each cut at 120 bytes:"
diff <(strings -a "$scratch/build-1") <(strings -a "$scratch/build-2") | grep '^[<>]' | cut -c1-120 || true
compare "control: build-1 vs itself" "$scratch/build-1" "$scratch/build-1"
other=/usr/bin/true
[ -f "$other" ] || other=/bin/true
compare "control: build-1 vs $other" "$scratch/build-1" "$other"
