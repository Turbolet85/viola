#!/usr/bin/env bash
# P5 known-positive controls for the plan's four guard entries (12-15), fed the same patterns.
set -u
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "== 12: a recorded TIMEOUT line counts (must exit 0, last line 1)"
printf '        TIMEOUT [45.012s] viola::cli_verify verify_window_without_screens_fails_every_interactive_row\n' > "$tmp/ev.md"
grep -cE 'TIMEOUT \[4[0-9]\.[0-9]+s\]' "$tmp/ev.md"; echo "exit $?"
echo "== 12: the 20 s override's line does not count (must exit 1, last line 0)"
printf '        TIMEOUT [20.003s] x\n' > "$tmp/ev20.md"
grep -cE 'TIMEOUT \[4[0-9]\.[0-9]+s\]' "$tmp/ev20.md"; echo "exit $?"

echo "== 13: a path changed since an older base reads changed (must exit 1)"
git diff --quiet 75198e53b918 -- tests/cli_verify.rs; echo "exit $?"

echo "== 14: a minted non-comment line counts 1 (must exit 0, last line 1 -> the guard reads red)"
printf -- '--- a/x\n+++ b/x\n+    let x = 1;\n+    /// a doc line\n' | grep -E '^[+-][^+-]' | grep -cvE '^[+-][[:space:]]*//'; echo "exit $?"
echo "== 14: comment-only lines count 0 (must exit 1, last line 0)"
printf -- '--- a/x\n+++ b/x\n+    /// a doc line\n-    // an old comment\n' | grep -E '^[+-][^+-]' | grep -cvE '^[+-][[:space:]]*//'; echo "exit $?"

echo "== 15: a minted sleep line trips the guard (must exit 1)"
! (printf '+        std::thread::sleep(std::time::Duration::from_secs(60));\n' | grep -E '^\+.*(#\[ignore|retries *=|test\.skip|std::env::var|thread::sleep)'); echo "exit $?"
echo "== 15: a minted ignore attribute trips the guard (must exit 1)"
! (printf '+#[ignore]\n' | grep -E '^\+.*(#\[ignore|retries *=|test\.skip|std::env::var|thread::sleep)'); echo "exit $?"
