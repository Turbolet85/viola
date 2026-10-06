#!/usr/bin/env bash
# P5 of the second revision (2026-10-06): baselines and known-verdict controls of the two inline guards the
# revision adds or changes. Run from the repository root. Prints one line per reading; writes only under its mktemp dir.
set -u
ledger='viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/evidence/live-sessions.md'
count_guard() { n=$(grep -cE '^\| [0-9]+ \|' "$1") && test "$n" -eq 12; }

count_guard "$ledger"; echo "ledger-count baseline: exit $? (rows $(grep -cE '^\| [0-9]+ \|' "$ledger"))"
tmp=$(mktemp -d)
for rows in 11 12 13; do
  f="$tmp/rows-$rows.md"
  for i in $(seq 1 "$rows"); do echo "| $i | minted | minted | minted | minted |"; done > "$f"
  count_guard "$f"; echo "ledger-count control, $rows minted rows: exit $?"
done
rm -r "$tmp"

git diff --quiet 2fbc9545bee9 -- crates/viola-agent-claude/src/screen.rs
echo "screen guard baseline: exit $?"
older=$(git log --format=%h -n 1 2fbc9545bee9 -- crates/viola-agent-claude/src/screen.rs)
git diff --quiet "${older}^" -- crates/viola-agent-claude/src/screen.rs
echo "screen guard control, against the parent of ${older} (the last commit that changed the file): exit $?"
