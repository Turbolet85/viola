#!/usr/bin/env bash
# The launcher of a live session (plan.md step 6 and step 9): the product build wraps claude 2.1.287, named by
# path, in the home the round stamped. It holds the --settings object so that no JSON crosses a command line's
# quoting: a session `ask` rule for the one command `true` (the permission dialog of the run), and the CLI's
# plan file in a 0700 plans/ beside the home (nothing of it under ~/.claude).
#   live-start.sh <home, relative to the repository root> <instance name>
set -eu
ROOT=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
H=${1:?usage: live-start.sh <home> <instance name>}
NAME=${2:?usage: live-start.sh <home> <instance name>}
case "$H" in
  target/e2e-home/viola-live-*/home) ;;
  *) echo "live-start: the home is not a viola-live home under target/e2e-home/" >&2; exit 2 ;;
esac
[ -d "$ROOT/$H" ] || { echo "live-start: no such home" >&2; exit 2; }
PLANS=$ROOT/${H%/home}/plans
mkdir -p "$PLANS"
chmod 700 "$PLANS"
SETTINGS=$(jq -cn --arg p "$PLANS" '{permissions: {ask: ["Bash(true)"]}, plansDirectory: $p}')
cd "$ROOT"
exec "$ROOT/target/release-check/release/viola" --home "$ROOT/$H" run "$NAME" -- \
  "$HOME/.local/share/mise/installs/claude/2.1.287/claude" --settings "$SETTINGS"
