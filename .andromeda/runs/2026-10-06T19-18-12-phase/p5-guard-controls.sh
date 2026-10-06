#!/usr/bin/env bash
# P5 baselines and known-positive controls for the plan's inline guards (the fixture, send-strand, census, no-ignore and session-ledger guards).
# Run from the repo root; prints one line per reading, never a content line.
set -u
base=2fbc9545bee9
m=viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows

echo "== guard: fixture preservation"
test -z "$(git diff --name-only --diff-filter=MD $base -- fixtures/claude)"; echo "baseline exit $?"
test -z "$(git diff --name-only --diff-filter=MD 75198e53b918 -- fixtures/claude)"; echo "control (base 75198e53b918, fixtures modified since) exit $?"

echo "== guard: send strand untouched"
git diff --quiet $base -- src/run/send.rs src/cmd/send.rs src/human.rs tests/cli_send.rs; echo "baseline exit $?"
old=$(git log -1 --format=%h -- src/run/send.rs)
git diff --quiet "${old}^" -- src/run/send.rs src/cmd/send.rs src/human.rs tests/cli_send.rs; echo "control (base ${old}^, before send.rs last changed) exit $?"

echo "== guard: local-command literal census"
out=$(grep -rnE '/(clear|remote-control)' src/run src/cmd/verify.rs src/cmd/verify src/bin); echo "baseline exit $? lines $(printf '%s' "$out" | grep -c . || true)"
grep -qE '/(clear|remote-control)' tests/cli_send.rs; echo "control (the pattern over tests/cli_send.rs) exit $?"

echo "== guard: no ignore, sleep or env read"
( ! (git diff $base -- tests crates/viola-e2e | grep -E '^\+.*(#\[ignore|thread::sleep)') && ! (git diff $base -- src crates | grep -E '^\+.*env::var(_os)?\(') ); echo "baseline exit $?"
printf '+#[ignore]\n' | grep -qE '^\+.*(#\[ignore|thread::sleep)'; echo "control (a minted +#[ignore] line trips the first grep) exit $?"
printf '+        std::thread::sleep(Duration::from_secs(1));\n' | grep -qE '^\+.*(#\[ignore|thread::sleep)'; echo "control (a minted sleep line trips the first grep) exit $?"
printf '+    let v = std::env::var("X");\n' | grep -qE '^\+.*env::var(_os)?\('; echo "control (a minted env read trips the second grep) exit $?"
printf '+    let v = 1;\n' | grep -qE '^\+.*env::var(_os)?\('; echo "control (a plain added line passes the second grep) exit $?"

echo "== guard: session ledger rows"
( n=$(grep -cE '^\| [0-9]+ \|' $m/evidence/live-sessions.md) && test "$n" -ge 6 && test "$n" -le 8 ) 2>/dev/null; echo "baseline exit $?"
row() { printf '| %s | 2026-10-06T00:00:00Z | 2.1.287 (Claude Code) | x | y |\n' "$1"; }
count() { n=$(grep -cE '^\| [0-9]+ \|') && test "$n" -ge 6 && test "$n" -le 8; }
for k in 1 2 3 4 5 6; do row $k; done | count; echo "control (6 minted rows) exit $?"
for k in 1 2 3 4 5; do row $k; done | count; echo "control (5 minted rows) exit $?"
for k in 1 2 3 4 5 6 7 8 9; do row $k; done | count; echo "control (9 minted rows) exit $?"
