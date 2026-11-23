# Journal — implement 2026-11-23-viola-open-2

- 2026-11-23T23:24:30Z · P2 firing 1: `gate.py run --only 1-12` backgrounded (fmt, clippy, the touched crates'
  units, the filtered new-case entries, the integration binaries, `bash -n`, deny, the host read). Layers 13-15 wait
  for the directive's hostwatch read and `df -m /tmp` (read 31978 MiB free at 23:24Z).
- Firing 1 read: green 10 · red 1 (2, clippy `large_enum_variant` on `Recovered`: `Replayed` grew by `window`) ·
  recorded 1 (12). Fix: `Recovered::Replayed`'s `state` boxed (clippy's own suggestion), 9 constructor sites.
- P2 firing 2: `--only 1,2,4,5,6,7,8,9` backgrounded after the fix → green 8 · red 0.
- 2026-11-23T23:27:00Z · host read `hostwatch.py read --last 15`: `verdict QUIET - 5 s stalled on IO`; `df -m /tmp`
  31975 MiB free. P2 firing 3: `--only 13,14,15` (the three whole layers) backgrounded → green 3 · red 0.
- The three remove-the-guard controls (`control-1.toml`: open-allowlist and place-only-seen on entry 7,
  window-launch on entry 6) fired through `gate.py control --timeout 900`, backgrounded; `df -m /home` 550950 MiB
  free.
