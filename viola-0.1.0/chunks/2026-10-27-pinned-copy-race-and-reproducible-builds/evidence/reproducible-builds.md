# Measurement B: clean release builds of one tree, compared by file and by section (plan step 9)

Read 2026-10-27 on the Linux dev host, run dir `.andromeda/runs/2026-10-27T02-08-46-implement/`. Script:
`viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-builds.sh`, run from the root
under a 3600 s `timeout`. Each build is `CARGO_TARGET_DIR=<scratch>/target bash scripts/release-check.sh` into a
`<scratch>/target` the script makes by `mkdir` before the build and removes after copying `release/viola` out, so
both builds of a firing use one same, empty target path; `<scratch>` is a `mktemp -d` under `/tmp`, removed by the
script's trap. No `RUSTC_WRAPPER`, `RUSTFLAGS`, `CARGO_BUILD_*` or cargo config file is set on the host (read before
the reading); 32 cores, so a cold release build takes 16–18 s.

Toolchain: `rustc 1.98.1 (48a229cea 2026-09-01)` · `cargo 1.98.1 (797e8a9bc 2026-08-05)` · `GNU objcopy (GNU
Binutils) 2.47` · `GNU readelf (GNU Binutils) 2.47`. Tree: HEAD `99977116182b` plus this chunk's uncommitted edits
(12 rows at the first firing; 13 at the third, the extra row `tests/path1_run_start.rs`'s temporary timing record,
a test file outside `--bin viola`).

## The reading (third firing, the script as it stands)

Builds: 1 at 02:20:25Z..02:20:43Z, 2 at 02:20:43Z..02:21:01Z, each `release-check: viola only`.

| | build 1 | build 2 |
|---|---|---|
| file sha256 (16 hex) | `57bce92d92ca5a2a` | `ff63f38e9700f432` |
| size | 8 179 264 B | 8 179 272 B |
| build-id note | `ce0631a6794b6f97502f3cac1bb919d246a9fe8c` | `bc18839736028f6e6cd1ac24cb0e236bb0b39af8` |

Sections (every section `readelf -S -W` names, each hashed from the file at its offset and size; NOBITS read as its
size): **34 named, 3 unequal** — `.note.gnu.build-id` (`6a892d51…` / `f8ebd7a6…`), `.symtab` (`93644dda…` /
`52a3073e…`), `.strtab` (`3784ad1f…` / `fb529271…`). Equal, among the 31: `.text` `60c6ea8b…`, `.rodata`
`05f4b5fc…`, `.data` `79665d0a…`, `.data.rel.ro` `fc414e05…`, `.eh_frame` `94b2d544…`, `.gcc_except_table`
`1b286c82…` (the CARRY's six loaded sections), and every other loaded section, `.comment` and `.shstrtab`.

`strings -a`: 6 differing lines — three local symbols whose `.llvm.<number>` suffix differs:
`anon.15867591b078eac10c8c2c5140586b7a.{9,56,57}.llvm.2173237286001873254` in build 1 against
`….llvm.13381032314865616282` in build 2 (each line cut at 120 bytes by the script).

Controls, through the same comparison:
- build 1 against itself: 34 named, **0 unequal**, 0 differing strings lines.
- build 1 against `/usr/bin/true` (another host program, 43 320 B): 38 named, **35 unequal**, 76 473 differing strings
  lines. Three sections hash equal there: `.interp`, `.note.ABI-tag` and `.fini` — both are dynamically linked
  glibc x86-64 executables (one interpreter path, one ABI note, one `.fini` stub); the plan's "all unequal" reads 35
  of 38, recorded as measured.

## The three firings

| firing | builds | build 1 | build 2 | the comparison's defects |
|---|---|---|---|---|
| 1 (02:17:16Z..02:17:49Z) | cold, one target path | `57bce92d…` | `ff63f38e…` | the `true` control resolved to the shell builtin (no file compared); `-O binary --only-section` writes loaded sections only, so `.comment` / `.symtab` / `.shstrtab` / `.strtab` hashed as empty |
| 2 (02:18:51Z..02:19:24Z) | cold, one target path | `ff63f38e…` | `ff63f38e…` | stopped at `.symtab` (`objcopy --dump-section` refuses it), exit 1; the file pair equal |
| 3 (02:20:25Z..02:21:01Z) | cold, one target path | `57bce92d…` | `ff63f38e…` | none — the reading above |

Six cold builds of one tree gave two files, `57bce92d…` (3 times) and `ff63f38e…` (3 times), in no fixed order
(A/B, B/B, A/B). Wherever the two met (firings 1 and 3) they differed in exactly the build-id note and 8 bytes of
symbol data (`.symtab` / `.strtab`); every loaded section was equal.

## What it means for the pin, and what is not explained

The pin key is a truncated SHA-256 of the whole file (`viola_state::pin::pin_key`), so two clean builds of one tree
can key two `bin/<key>/` and `plugin/<key>/` folders while loading the same code. **Not explained:** why the
`.llvm.<number>` suffix of those three local symbols differs from one cold build to the next (and, with it, the
build-id the linker hashes over the file). No cause was measured, and none is recorded. No product or build change
rests on this reading: acting on it (a profile key, a linker flag, a key over loaded sections) is the founder's.
