# The `vt100_feed` fuzz target's panic-hook override — remove-the-guard pair

Measured 2026-10-04 at /implement on the Linux dev host, toolchain `nightly-2026-09-20` (`fuzz/rust-toolchain.toml`),
cargo-fuzz 0.13.2, libfuzzer-sys 0.4.13.

## The premise, now read from source
The plan held "libfuzzer-sys aborts on any panic through its own panic hook" as a premise not measured (its source was
not on the host at P4). At implement it is in the cargo registry (`libfuzzer-sys-0.4.13/src/lib.rs`):
- `:83-94` — `initialize` takes the default hook and installs one that calls `std::process::abort()` after it, on
  every panic, a caught one included;
- `:60-70` — the target body runs inside the crate's own `catch_unwind`, and an `Err` there calls
  `std::process::abort()`.
So the target's silent hook (`fuzz/fuzz_targets/vt100_feed.rs`, `SILENT_HOOK.call_once`) removes the abort for
vt100's caught panic only; a panic that escapes the target's own `catch_unwind` still aborts through `:60-70`.

## The pair
| reading | target source | command | exit | verdict |
|---|---|---|---|---|
| with the guard | as committed | `cargo +nightly-2026-09-20 fuzz run --fuzz-dir fuzz vt100_feed fuzz/corpus/vt100_feed -- -runs=0` | 0 | `seed corpus: files: 6` · `Done 7 runs` |
| with the guard | as committed | `… vt100_feed fuzz/corpus/vt100_feed/wide-one-col` | 0 | the 24×1 wide-char seed replays green |
| guard removed | the `SILENT_HOOK.call_once` line commented out | `… vt100_feed fuzz/corpus/vt100_feed/wide-one-col` | 1 | `panicked at …/vt100-0.16.2/src/screen.rs:730:22` · `ERROR: libFuzzer: deadly signal` |
| guard removed, control seed | same | `… vt100_feed fuzz/corpus/vt100_feed/wide-line-24x80` | 0 | a seed vt100 does not panic on stays green without the guard |

The line was restored and re-read (`grep -n call_once` → `:24`, uncommented) before any gate ran. No crash artifact
was written under `fuzz/artifacts/vt100_feed/` (a single-file replay writes none).

## The seed corpus (`fuzz/corpus/vt100_feed/`, synthetic)
First byte → rows `1 + b0 % 64`, second → cols `1 + b1 % 200`.
| file | size | bytes after the two size bytes |
|---|---|---|
| `wide-one-col` | 24×1 | U+4E2D (`e4 b8 ad`) — research.md M2, panics |
| `csi-u-one-by-one` | 1×1 | `?u` — M2, panics |
| `abc-one-by-two` | 1×2 | `abc` — M2, panics |
| `sgr-cup-24x80` | 24×80 | SGR bold red, CUP 5;10, the tests' canary, SGR reset, CRLF, `> ` |
| `osc-title-bel-24x80` | 24×80 | OSC 0 title (the canary) ended by BEL, then the canary |
| `wide-line-24x80` | 24×80 | U+4E2D U+6587, a space, the canary |
