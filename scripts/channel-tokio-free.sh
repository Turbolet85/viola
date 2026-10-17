#!/usr/bin/env bash
# viola-channel's own normal graph stays tokio-free. Its `tokio` feature (the Tokio client, enabled
# by viola-mcp alone) unifies into every whole-workspace read, so `cargo deny` over viola-channel as
# a sole root reports tokio there; `cargo tree -p` resolves the named package's own features instead.
# Without its feature viola-channel must reach no tokio, and with `--features tokio` (the control)
# it must, so a form that has stopped seeing tokio cannot read green. One verdict line, last.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

# present · absent · error, for viola-channel's normal graph under the given extra flags.
reading() {
  local out rc=0
  out=$(cargo tree -p viola-channel -e normal --target all -i tokio --prefix none "$@" 2>&1) || rc=$?
  out=$(printf '%s\n' "$out" | tr -d '\r')
  if printf '%s\n' "$out" | grep -q '^tokio v'; then
    echo present
  elif [ "$rc" -eq 0 ] || printf '%s\n' "$out" | grep -qF 'did not match any packages'; then
    echo absent
  else
    echo error
  fi
}

own=$(reading)
control=$(reading --features tokio)
echo "viola-channel normal graph: tokio $own; with --features tokio: tokio $control"

if [ "$own" = error ] || [ "$control" = error ]; then
  echo "channel-tokio-free: FAILED — cargo tree did not run"
  exit 1
fi
if [ "$control" != present ]; then
  echo "channel-tokio-free: FAILED — the control reaches no tokio: the form no longer sees it"
  exit 1
fi
if [ "$own" != absent ]; then
  echo "channel-tokio-free: FAILED — viola-channel reaches tokio without its feature"
  exit 1
fi
echo "channel-tokio-free: ok"
