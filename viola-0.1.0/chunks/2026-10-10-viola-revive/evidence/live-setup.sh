#!/usr/bin/env bash
# The two session directories of the live readings (plan.md step 12): `a`, where the first session is
# started and so the directory the wrapper records, and `b`, the other directory a revive is typed in and a
# resume is made from by hand. Each holds one project file, a CLAUDE.md that names a marker word of its own,
# so a session can be asked which directory's project file it loaded. Both stand under the repository's
# ignored build directory: inside the tree the founder trusted, outside every commit.
#   live-setup.sh <base, relative to the repository root, under target/>
set -eu
ROOT=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
BASE=${1:?usage: live-setup.sh <base under target/>}
case "$BASE" in
  target/rev-live-*) ;;
  *) echo "live-setup: the base is not target/rev-live-*" >&2; exit 2 ;;
esac
[ -e "$ROOT/$BASE" ] && { echo "live-setup: the base already exists" >&2; exit 2; }
mkdir -p "$ROOT/$BASE/a" "$ROOT/$BASE/b"
printf 'The marker word of this project is ALPHA.\n' > "$ROOT/$BASE/a/CLAUDE.md"
printf 'The marker word of this project is BRAVO.\n' > "$ROOT/$BASE/b/CLAUDE.md"
ls -1 "$ROOT/$BASE"
