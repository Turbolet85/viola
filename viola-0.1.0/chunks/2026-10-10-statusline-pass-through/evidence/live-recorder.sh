#!/bin/sh
# The rig's statusline recorder (plan.md step 12). A copy of it stands in the rig's private directory, outside
# the tree, and is named as a session's statusline command:
#   <private dir>/live-recorder.sh <label> "$0" "${BASH_VERSION:+bash}" "${ZSH_VERSION:+zsh}"
# The three quoted words are expanded by whatever shell runs that string, so they say which shell it was: its
# own name, and whether it is bash or zsh. A launcher that runs no shell hands them over unexpanded.
# Per invocation it files its stdin whole in `rec-<label>-<n>.stdin` and one `rec-<label>-<n>.meta` of
# `key=value` lines: the label, the time, the three words, the names and executables of its three nearest
# ancestor processes, the stdin's size and whether COLUMNS and LINES are set. Both files stay in the private
# directory. It prints the fixed text `SLMARK<label>` with no newline and exits 0.
dir=$(dirname "$0")
label=${1:-none}
n=$(date +%s%N)-$$
cat > "$dir/rec-$label-$n.stdin"
parent_of() { sed -n 's/^PPid:[[:space:]]*//p' "/proc/$1/status" 2>/dev/null; }
name_of() { cat "/proc/$1/comm" 2>/dev/null; }
exe_of() { basename "$(readlink "/proc/$1/exe" 2>/dev/null)" 2>/dev/null; }
p1=$PPID
p2=$(parent_of "$p1")
p3=$(parent_of "$p2")
{
  echo "label=$label"
  echo "at=$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)"
  echo "launcher_arg0=${2-}"
  echo "launcher_bash=${3-}"
  echo "launcher_zsh=${4-}"
  echo "argc=$#"
  echo "p1_comm=$(name_of "$p1")"
  echo "p1_exe=$(exe_of "$p1")"
  echo "p2_comm=$(name_of "$p2")"
  echo "p2_exe=$(exe_of "$p2")"
  echo "p3_comm=$(name_of "$p3")"
  echo "p3_exe=$(exe_of "$p3")"
  echo "stdin_bytes=$(wc -c < "$dir/rec-$label-$n.stdin" | tr -d ' ')"
  echo "columns_set=${COLUMNS:+yes}"
  echo "lines_set=${LINES:+yes}"
  echo "viola_name_set=${VIOLA_NAME:+yes}"
} > "$dir/rec-$label-$n.meta"
printf 'SLMARK%s' "$label"
exit 0
