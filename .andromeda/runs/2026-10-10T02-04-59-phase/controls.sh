#!/usr/bin/env bash
# P5 known-verdict controls for the plan's new inline guards. Inputs are minted beside this file; nothing in the
# tree is read except the workflow file.
d="$(cd "$(dirname "$0")" && pwd)/controls"
mkdir -p "$d"
F='.mutants.verdict == "package" and .mutants.package == "viola-pty" and (.mutants.tested | type) == "number" and .mutants.tested > 0 and (.mutants.host_excluded | type) == "array" and (.mutants.host_excluded | length) > 0 and all(.mutants.host_excluded[]; (.name | type) == "string" and (.cfg | type) == "string")'

printf '%s\n' '    timeout-minutes: 180' > "$d/ceiling-moved.yml"
echo "ceiling control (a file whose ceiling moved): $(grep -c 'timeout-minutes: 120' "$d/ceiling-moved.yml"; echo "exit $?")"

printf '%s\n' '          - package: viola' '          - package: viola' '          - package: viola' '          - package: viola' '          - package: viola-e2e' > "$d/split.yml"
echo "split control (four viola items and one viola-e2e): $(grep -c '^          - package: viola$' "$d/split.yml"; echo "exit $?")"

pass='{"ok":true,"mutants":{"tested":89,"verdict":"package","package":"viola-pty","host_excluded":[{"name":"crates/viola-pty/src/lib.rs:434:9: replace console::is_console -> bool with true","cfg":"windows"}]}}'
echo "$pass" > "$d/w-pass.json"
echo '{"ok":true,"mutants":{"tested":89,"verdict":"package","package":"viola-pty","host_excluded":[]}}' > "$d/w-empty.json"
echo '{"ok":true,"mutants":{"tested":89,"verdict":"package","package":"viola-pty"}}' > "$d/w-nofield.json"
echo '{"ok":true,"mutants":{"tested":89,"verdict":"scoped","package":"viola-pty","host_excluded":[{"name":"a","cfg":"windows"}]}}' > "$d/w-scoped.json"
echo '{"ok":true,"mutants":{"tested":89,"verdict":"package","package":"viola-state","host_excluded":[{"name":"a","cfg":"unix"}]}}' > "$d/w-member.json"
echo '{"ok":true,"mutants":{"tested":0,"verdict":"package","package":"viola-pty","host_excluded":[{"name":"a","cfg":"windows"}]}}' > "$d/w-untested.json"
echo '{"ok":true,"mutants":{"tested":89,"verdict":"package","package":"viola-pty","host_excluded":[{"name":"a"}]}}' > "$d/w-nocfg.json"
for f in w-pass w-empty w-nofield w-scoped w-member w-untested w-nocfg; do
  jq -e "$F" "$d/$f.json" > /dev/null 2>&1
  echo "witness control $f: exit $?"
done
