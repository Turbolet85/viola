#!/usr/bin/env bash
# Foreign-load watcher beside coverage run 2: every 20 s, any rustc.exe whose path is not this project's pinned
# toolchain (1.98.1) is a foreign build; plus the CPU reading. Ends when cov.sh writes cov-exit.txt.
R=/d/dev/projects/viola/.andromeda/runs/2026-10-01T09-18-50-code-audit
while [ ! -f "$R/cov-exit.txt" ]; do
  f=$(powershell -NoProfile -Command "@(Get-CimInstance Win32_Process -Filter \"Name='rustc.exe'\" | ? { \$_.ExecutablePath -notmatch '1\.98\.1' }).Count")
  cpu=$(powershell -NoProfile -Command "[math]::Round((Get-Counter '\Processor(_Total)\% Processor Time').CounterSamples.CookedValue,0)")
  echo "$(date -u +%FT%TZ) foreign_rustc=$(echo $f | tr -dc '0-9') cpu=$(echo $cpu | tr -dc '0-9')" >> "$R/cov-watch.log"
  sleep 20
done
