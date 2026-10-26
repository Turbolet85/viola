# P5 controls — 2026-10-26-tokio-rustsec-2026-0386-lock-bump

Fired by `.andromeda/runs/2026-10-26T05-14-53-phase/p5-controls.sh` (2026-10-26T06:00Z), each entry AS THE ENTRY through
`gate.py run --plan … --entry n --project-root <clone>`, over `git clone --shared` scratch clones of HEAD `b1249fb1a87b`
holding the chunk folder, in the script's own `$(mktemp -d)` (removed on exit).

| clone | input | entry | reading |
|---|---|---|---|
| A | unmodified | 6 | red · exit 1, `0` (as on the real tree) |
| A | unmodified | 7 | red · `contains 'version = "1.53.3"'` ✗, the lock reads `version = "1.53.1"` (as on the real tree) |
| A | unmodified | 8 | red · exit 1, `0` (as on the real tree) |
| A | unmodified | 17 | green · `guard: clean` (as on the real tree) |
| B | the plan's steps 1–2: pin `=1.53.3` + `cargo update -q -p tokio --precise 1.53.3` | 6 | green · `1` |
| B | same | 7 | green · `version = "1.53.3"` |
| B | same | 8 | green · last line `4` |
| B | same | 9 | green · `cargo deny check` exit 0 |
| B | same | 17 | green · `guard: clean` (Cargo.toml and Cargo.lock allowed) |
| C | B + one more `Cargo.lock` checksum line edited, a line appended to `src/main.rs`, an untracked `src/p5_control.rs` | 8 | red · `last line '4' ✗ (last: '6')` |
| C | same | 17 | red · `last line 'guard: clean' ✗ (last: 'guard: outside 2 files')` |

Entry 9's must-fail side is the real tree's own baseline: `advisories FAILED, bans ok, licenses ok, sources ok`, one
`error[unsound]` (RUSTSEC-2026-0386, `AsyncFd::new` and related constructors), four `warning[duplicate]`.
