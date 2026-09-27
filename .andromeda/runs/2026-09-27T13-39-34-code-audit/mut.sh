#!/usr/bin/env bash
# One unit per call: stop rust-analyzer by exact ExecutablePath, run cargo-mutants, then list %TEMP% cargo-mutants-* dirs.
U="$1"
cd /d/dev/projects/viola || exit 9
R=.andromeda/runs/2026-09-27T13-39-34-code-audit
L=$R/mut-$U.log
powershell.exe -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='rust-analyzer.exe'\" | ForEach-Object { if (\$_.ExecutablePath) { Stop-Process -Id \$_.ProcessId -Force; 'stopped ' + \$_.ProcessId + ' ' + \$_.ExecutablePath } }" > $L 2>&1
date -u +start=%FT%TZ >> $L
NEXTEST_PROFILE=mutants timeout 900 cargo mutants -p "$U" --test-tool=nextest -j 4 --output $R/mutants-$U >> $L 2>&1
echo "exit=$?" >> $L
date -u +end=%FT%TZ >> $L
powershell.exe -NoProfile -Command "Get-ChildItem -Directory -Path \$env:TEMP -Filter 'cargo-mutants-*' | ForEach-Object { \$s=(Get-ChildItem -Recurse -File -Force \$_.FullName -ErrorAction SilentlyContinue | Measure-Object Length -Sum).Sum; 'tempdir {0} {1:N2} GB {2:o}' -f \$_.Name, (\$s/1GB), \$_.LastWriteTime }" >> $L 2>&1
echo done >> $L
