# What `claude` 2.1.287 expects back from the terminal — a static read of its binary (2026-10-08, 07:45Z)

The operator's hypothesis (inputs#I16): the recorded fixtures under `fixtures/claude` hold the queries the CLI
sends at its start. **Falsified.** `grep -r -l -a` for an ESC byte or an escaped ESC (`u001b`, `x1b`) over
`fixtures/`: 0 files. The 2.1.287 set holds 28 files: hook payloads and three `Screen.<phase>.json` files, whose
keys are `cols`, `rows`, `screen_phase` (signature rows, no byte stream).

The next source that needs no live start is the CLI itself. The binary is not snapshotted: `inputs.py snap`
refuses a binary source over 1 MiB. Its identity: `~/.local/share/mise/installs/claude/2.1.287/claude`,
244 317 368 bytes, sha256 `3920489a5109cff5786a1a392c25277408ff22bc796d5edb9c16a60e5a1718f0`. Read with a
regex scan over the file (the scripts are in the session scratchpad; what they found is below, as it stands in
the embedded script).

## The reply parser

One function of the embedded script turns a completed terminal sequence into a typed response. Its regexes,
copied from the binary:

| the CLI's name | the reply it parses |
|---|---|
| `decrpm` | `^\x1b\[\?(\d+);(\d+)\$y$` |
| `da1` | `^\x1b\[\?([\d;]*)c$` |
| `da2` | `^\x1b\[>([\d;]*)c$` |
| `kittyKeyboard` | `^\x1b\[\?(\d+)u$` |
| `cursorPosition` | `^\x1b\[\?(\d+);(\d+)R$` |
| `themeNotify` | `^\x1b\[\?997;([12])n$` |
| `cellSize` | `^\x1b\[6;(\d+);(\d+)t$` |
| `osc` | `^\x1b\](\d+);(.*?)(?:\x07|\x1b\\)$` |
| `xtversion` | `^\x1bP>\|(.*?)(?:\x07|\x1b\\)$` |
| `kittyGraphics` | `^\x1b_G(?:[^;]*,)?i=(\d+)[^;]*;(.*?)\x1b\\$` |

These ten are every response type that function returns; any other sequence returns null there. The same
script holds one more regex that tests whether a string is made of terminal responses only: mouse, focus,
`\??\d+;\d+(?:;\d+)*R`, `[?>]\d+(?:;\d+)*c`, `\?\d+(?:;\d+)*\$y`, `\?997;[12]n`, `\?\d+u`, any DCS, any OSC.

Query literals found beside it: `CSI c`, `CSI 6n`, `CSI > q` (raw), `?6n` (2 hits), `OSC 11;?`, and the kitty
graphics query `_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA`. Most queries are built from numbers, so the absence of a
literal (`?996n`, `16t`: 0 hits each) says nothing; the parser is the evidence of what the CLI asks for.

## Against the wheel's closed list, on this host's terminal

| the reply the CLI parses | on the closed list at HEAD (`src/run/wheel.rs:496-503`) | foot 1.28.0 sends it | read as typing today |
|---|---|---|---|
| `decrpm`, `da1`, `da2`, `kittyKeyboard` | yes | yes | no (measured, `evidence/reply-probe.ndjson`) |
| `osc` (10, 11, 4: 25 to 26 bytes), `xtversion` (18 bytes) | yes, as OSC / DCS strings of at most 64 payload bytes | yes | no (measured) |
| `themeNotify` `CSI ? 997;1 n` | no | yes | **yes** (measured: `theme-996`) |
| `cellSize` `CSI 6;h;w t` | no | yes | **yes** (measured: `winops-16`) |
| `cursorPosition` `CSI ? r;c R` | no (the list's CPR form has no `?`) | **no**: foot gave no answer to `CSI ? 6 n` | no reply arrives (measured in this revision, `replies-r.ndjson`, 07:47Z) |
| `kittyGraphics` (an APC string) | no | no: foot gave no answer | no reply arrives (measured, `kitty-graphics`) |

So on foot, the replies this CLI can draw that are read as typing today are two, `CSI ? 997;1 n` and
`CSI 6;h;w t`, and both are among the seven shapes the operator's word adds. The other five of the seven
(`CSI 0 n`, `CSI 4;…t`, `CSI 8;…t`, `CSI 48;…t`, `CSI > 4;1 m`) have no parser in this CLI.

## What this does not show

- Which queries the CLI sends in its first 237 ms, as bytes. That needs a live start.
- That the parser is complete for what arrives: a terminal may write a reply nobody asked for.
- Other terminals. A terminal that answers `CSI ? 6 n` (`CSI ? r;c R`) or the kitty graphics query (an APC
  string) would write a reply that is typing at HEAD and is not among the seven.

## The revision's probe, 07:47Z (no live start, no key)

`reply-probe-r.sh` with `reply-probe-r-child.py` (this run dir), on the standing own compositor, the product
build as it stood before the fix. Rows: `replies-r.ndjson`.

| query | foot answered | `wheel` record of cause `human-input` |
|---|---|---|
| DA1 `CSI c` (the control) | `CSI ? 62;4;22;28;52 c` | 0 |
| DECXCPR `CSI ? 6 n` | nothing | 0 |
| XTVERSION `CSI > q` | `DCS > | foot(1.28.0) ST` | 0 |

The desktop lock read locked right before it (07:47:00Z: shell `true`, desktop instance `true`); the own
compositor stood (alive at 07:47:08Z, 0 windows).
