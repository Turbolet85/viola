#!/usr/bin/env bash
# The launcher of one statusline session on the headless pty host (plan.md step 12). It holds the prompt, so no
# quoted text crosses a command line, and it passes no --settings of its own: under `wrapped` the only --settings
# the CLI gets is the override viola writes.
#   live-sl-start.sh <private dir> <label> unwrapped <dir>
#   live-sl-start.sh <private dir> <label> wrapped   <dir> <home> <instance>
#   live-sl-start.sh <private dir> <label> rehearsal <dir> <home> <instance> [stdin]
# <dir> and <home> are relative to the repository root. `unwrapped` and `wrapped` start claude 2.1.287, named by
# path, with its install directory first on PATH, on the model alias haiku, with one short prompt as its
# argument. `rehearsal` starts the harness build's fake agent under the product build instead (no live start);
# with `stdin` it hands the fake agent the rig's synthetic statusline payload.
set -eu
ROOT=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
HERE=$(cd "$(dirname "$0")" && pwd)
PRIV=${1:?usage: live-sl-start.sh <private dir> <label> <mode> <dir> ...}
LABEL=${2:?label}
MODE=${3:?mode}
DIR=${4:?dir}
case "$DIR" in
  target/sl-live-*/a | target/sl-live-*/b) ;;
  *) echo "live-sl-start: the directory is not target/sl-live-*/a or /b" >&2; exit 2 ;;
esac
[ -d "$ROOT/$DIR" ] || { echo "live-sl-start: no such directory" >&2; exit 2; }
CLI_DIR=$HOME/.local/share/mise/installs/claude/2.1.287
CLI=$CLI_DIR/claude
VIOLA=$ROOT/target/release-check/release/viola
PROMPT='Reply with the single word READY and nothing else.'
home_of() {
  case "$1" in
    target/e2e-home/viola-live-sl-*/home) ;;
    *) echo "live-sl-start: the home is not a viola-live-sl home under target/e2e-home/" >&2; exit 2 ;;
  esac
  printf '%s' "$ROOT/$1"
}
cd "$ROOT/$DIR"
export PATH="$CLI_DIR:$PATH"
case "$MODE" in
  unwrapped)
    exec python3 "$HERE/live-pty.py" "$PRIV" "$LABEL" -- "$CLI" --model haiku "$PROMPT"
    ;;
  wrapped)
    H=$(home_of "${5:?home}")
    exec python3 "$HERE/live-pty.py" "$PRIV" "$LABEL" -- "$VIOLA" --home "$H" run "${6:?instance}" -- \
      "$CLI" --model haiku "$PROMPT"
    ;;
  rehearsal)
    H=$(home_of "${5:?home}")
    FAKE=$ROOT/target/harness/debug/viola-fake-agent
    set -- "$VIOLA" --home "$H" run "${6:?instance}" -- "$FAKE" --receipt "$PRIV/$LABEL.receipt.ndjson" ${7:+--statusline-stdin "$PRIV/rehearsal.stdin"}
    exec python3 "$HERE/live-pty.py" "$PRIV" "$LABEL" -- "$@"
    ;;
  *)
    echo "live-sl-start: unknown mode" >&2
    exit 2
    ;;
esac
