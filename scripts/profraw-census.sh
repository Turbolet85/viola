#!/usr/bin/env bash
# The profile census (test-plan §10 Zero-flakiness budget): one test of the instrumented root-bin test binary,
# `cmd::run::tests::start_opens_the_scenario_one_spans_under_run_start`, run <runs> times over <workers> concurrent
# loops, each run's raw coverage profiles counted and sized. That test starts a PTY child and kills it: an
# instrumented child that can exit by itself leaves a profile of its own when it wins the race, and a short one when
# the kill lands inside its exit-time write, which `llvm-profdata merge` refuses. A clean run leaves one profile, the
# test process's own, because neither the wrapper's child nor the version probe's child is instrumented. The census
# passes only when every run passed and left exactly that one whole profile. It reads the binary the last coverage
# run built, so it stands after `agent-run.sh pre-push`; the profile path is set here, for this script's children
# only. An optional third word names the census directory, `target/profraw-census/<name>/`, which may already exist.
set -euo pipefail

test_name="cmd::run::tests::start_opens_the_scenario_one_spans_under_run_start"

usage() {
  echo "usage: profraw-census.sh <runs> <workers> [<name>]"
  exit 2
}
if [ "$#" -lt 2 ] || [ "$#" -gt 3 ] || ! [[ $1 =~ ^[1-9][0-9]*$ ]] || ! [[ $2 =~ ^[1-9][0-9]*$ ]]; then
  usage
fi
runs=$1
workers=$2
name="$(date -u +%Y%m%dT%H%M%SZ)-$$"
if [ "$#" -eq 3 ]; then
  if ! [[ $3 =~ ^[A-Za-z0-9._-]+$ ]] || [ "$3" = . ] || [ "$3" = .. ]; then
    usage
  fi
  name=$3
fi

root=$(cd "$(dirname "$0")/.." && pwd)
out="$root/target/profraw-census/$name"
mkdir -p "$out"

# The newest instrumented binary that holds the test. Its `--list` writes a profile too: into the census dir. The
# listing is read whole from a file: a reader that stops at the first match breaks the lister's pipe.
bin=""
while IFS= read -r candidate; do
  if LLVM_PROFILE_FILE="$out/list-%p.profraw" "$candidate" --list >"$out/list" 2>/dev/null \
    && grep -qxF "$test_name: test" "$out/list"; then
    bin=$candidate
    break
  fi
done < <(find "$root/target/llvm-cov-target/debug/deps" -maxdepth 1 -type f -name 'viola-*' ! -name '*.*' \
  -perm -u+x -printf '%T@ %p\n' 2>/dev/null | sort -rn | cut -d' ' -f2-)
rm -f "$out"/list "$out"/list-*.profraw
if [ -z "$bin" ]; then
  echo "profraw-census: no instrumented binary"
  exit 2
fi

echo "profraw-census: runs $runs · workers $workers"

# One loop: its share of the runs, one line per run in its tally: `<passed 0|1> <profiles> <size>…`.
worker() {
  local w=$1 n=$2 i passed sizes
  local dir="$out/w$w"
  mkdir -p "$dir"
  for ((i = 0; i < n; i++)); do
    passed=1
    LLVM_PROFILE_FILE="$dir/r$i-%p.profraw" "$bin" --exact "$test_name" >"$dir/r$i.log" 2>&1 || passed=0
    mapfile -t sizes < <(find "$dir" -maxdepth 1 -name "r$i-*.profraw" -printf '%s\n')
    echo "$passed ${#sizes[@]} ${sizes[*]}" >>"$dir/tally"
    rm -f "$dir"/r"$i"-*.profraw
    [ "$passed" -eq 1 ] && rm -f "$dir/r$i.log"
  done
  return 0
}

pids=()
for ((w = 0; w < workers; w++)); do
  share=$((runs / workers))
  [ "$w" -lt $((runs % workers)) ] && share=$((share + 1))
  worker "$w" "$share" &
  pids+=("$!")
done
for pid in "${pids[@]}"; do
  wait "$pid"
done

# A profile is short when it is smaller than the largest one the census saw; a run has a third when it left more
# than the one a clean run leaves.
summary=$(find "$out" -name tally -exec cat {} + | awk '
  { n++; passed += $1; profiles += $2; if ($2 > 1) third++
    for (i = 3; i <= NF; i++) { size[++k] = $i; if ($i > max) max = $i } }
  END { for (i = 1; i <= k; i++) if (size[i] < max) short++
        printf "%d %d %d %d %d", n, passed, profiles, third, short }')
read -r n passed profiles third short <<<"$summary"

echo "profraw-census: runs $n · passed $passed · profiles $profiles · third $third · short $short"
if [ "$n" -eq "$runs" ] && [ "$passed" -eq "$n" ] && [ "$profiles" -eq "$n" ] && [ "$third" -eq 0 ] \
  && [ "$short" -eq 0 ]; then
  exit 0
fi
exit 1
