#!/usr/bin/env bash
# P5 known-positive controls for the plan's guard entries 8, 16, 17, 18, 19: each must FAIL on an input it exists
# to catch. Prints one line per control: the entry, the input, the exit it read.
set -u
cd "$(git rev-parse --show-toplevel)"

# 8 — the preservation guard: a 2.1.287 screen file differs from a revision before it existed.
git diff --quiet ed2edde -- fixtures/claude/2.1.287/Screen.modal.json; echo "8  vs ed2edde (before the screen fixtures): exit $?"

# 16 — no schema / seam change: the diag schema differs from a revision before verify-pty-probe joined it.
git diff --quiet ed2edde -- schemas/diag-line.v1.json; echo "16 schemas/diag-line.v1.json vs ed2edde: exit $?"

# 17 — no ignore / retry / skip / env read: a minted env-read line passes the inner grep, so the guard exits 1.
! (printf '%s\n' '+    let h = std::env::var("HOME");' | grep -E '^\+.*(#\[ignore|retries *=|test\.skip|std::env::var)' | grep -v 'VIOLA_NAME') >/dev/null; echo "17 minted std::env::var line: exit $?"

# 18 — the capture arm adds no VIOLA_ read, obs init, channel client or stderr: a minted VIOLA_ read counts 1.
n=$(printf '%s\n' '+    let dir = std::env::var_os("VIOLA_DIR");' | grep -cE '^\+.*(VIOLA_|viola_obs_init|Client::|eprint|stderr)'); echo "18 minted VIOLA_DIR read: count $n"

# 19 — no deny allow / skip / ignore / exception: a minted skip line counts 1.
n=$(printf '%s\n' '+skip = [{ name = "x" }]' | grep -cE '^\+.*(skip|ignore|exceptions|allow *=)'); echo "19 minted skip line: count $n"
