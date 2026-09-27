#!/usr/bin/env bash
# P5 baseline for the build-essential entry, read WITHOUT mutating the host before the word:
# the root path's identity, the package state, and apt's simulation of the entry (-s).
cd /d/dev/projects/viola || exit 9
D=.andromeda/runs/2026-09-26T21-29-01-phase/p5
{
  echo "== id -u as root"; MSYS2_ARG_CONV_EXCL='*' wsl.exe -d Ubuntu -u root --exec /usr/bin/id -u; echo "exit=$?"
  echo "== dpkg -s build-essential"; MSYS2_ARG_CONV_EXCL='*' wsl.exe -d Ubuntu -u root --exec /usr/bin/dpkg -s build-essential 2>&1 | head -3; echo "exit=${PIPESTATUS[0]}"
  echo "== apt-get -s install -y build-essential"; MSYS2_ARG_CONV_EXCL='*' wsl.exe -d Ubuntu -u root --exec /usr/bin/apt-get -s install -y build-essential > "$D/06-sim.log" 2>&1; echo "exit=$?"
  grep -E "newly installed|^E:" "$D/06-sim.log"
} > "$D/06.log" 2>&1
cat "$D/06.log"
