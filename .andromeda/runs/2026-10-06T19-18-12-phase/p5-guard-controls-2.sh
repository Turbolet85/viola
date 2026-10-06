#!/usr/bin/env bash
# P5 controls for the revised send-strand guard (the revision of 2026-10-06, inputs#I4). Read-only: each line
# prints a label and the exit or the reading it names. Run from the repository root.
set -u
base=2fbc9545bee9
keep='^[0-9]+[[:space:]]+0[[:space:]]'

# 1. The entry itself on the untouched source tree: expected exit 0.
git diff --quiet "$base" -- src/run/send.rs src/cmd/send.rs src/human.rs \
  && test -z "$(git diff --numstat "$base" -- tests/cli_send.rs | grep -vE "$keep")"
echo "entry on the untouched tree: exit $?"

# 2. The source half against a base before src/run/send.rs last changed: expected exit 1.
git diff --quiet '012fc50^' -- src/run/send.rs src/cmd/send.rs src/human.rs
echo "source half against 012fc50^: exit $?"

# 3. The numstat filter on minted lines: additions only must read empty, any deletion must print.
printf '37\t0\ttests/cli_send.rs\n' | grep -vE "$keep"
echo "minted additions-only line (37 added, 0 deleted): grep -v exit $? (1 = filtered out, the guard reads empty)"
printf '37\t1\ttests/cli_send.rs\n' | grep -vE "$keep" >/dev/null
echo "minted line with one deletion (37 added, 1 deleted): grep -v exit $? (0 = kept, the guard reads non-empty)"
printf '0\t10\ttests/cli_send.rs\n' | grep -vE "$keep" >/dev/null
echo "minted deletions-only line (0 added, 10 deleted): grep -v exit $? (0 = kept)"

# 4. The test half against a real base where tests/cli_send.rs lost lines: expected a non-empty reading.
first=$(git log --reverse --format=%H -- tests/cli_send.rs | head -n 1)
n=$(git diff --numstat "$first" -- tests/cli_send.rs | grep -vE "$keep" | wc -l)
echo "test half against the file's first commit: $n kept numstat line(s) (>= 1 = the guard would read red)"
git diff --numstat "$first" -- tests/cli_send.rs
