# The live work's preconditions (plan.md steps 1 and 2)

Read at 2026-10-09T18:51:41Z, before any edit, before the rehearsal and before any live start. Each line is what
the command printed, not what the plan forecast. Times are UTC, read from the clock.

| # | precondition | command | reading | holds |
|---|---|---|---|---|
| 1 | the 2.1.287 binary answers by path | `"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` | `2.1.287 (Claude Code)`, exit 0 | yes |
| 2 | the stamped home holds one version, 17 rows `pass` | `jq` over `target/e2e-home/viola-live-4043089/home/ledger/stamps.json` | `v` 1, `writer` `verify`, `written_at` `2026-10-08T07:27:51.144Z`; `data.versions` holds one key, `2.1.287`; its `rows` hold 17 values, all `pass`; the file is mode 0600, 857 B | yes |
| 3 | the homes' backing | `test -L target/e2e-home`, then `findmnt -n -o FSTYPE -T target/e2e-home/` | a link, exit 0; `tmpfs` | yes |
| 4 | the build that stands before the change | `sha256sum target/release-check/release/viola` | prefix `62bf6028d95fe34e`; 3 998 456 B | yes |

No precondition failed, so stop rule S1 did not fire.

Read beside them:

- The implementing session is bridge-wrapped: its environment holds `VIOLA_BIN`, `VIOLA_DIR` and `VIOLA_NAME` and
  ten `CLAUDE*` names (names only, thirteen in all). The rig's host declares the session's environment
  (`live-pty.py`: `HOME`, `USER`, `LOGNAME`, `SHELL`, `PATH`, `LANG`, `XDG_RUNTIME_DIR`, `TERM=xterm-256color`), so
  none of the thirteen crosses into a session, and every `live-drive.py` call runs with the three `VIOLA_*` names
  removed and `--home` given.
- `target/harness/debug/viola-fake-agent` is present (the harness build of 2026-10-09).
- The host (`hostwatch.py read --last 15 --for viola`, 18:36:41Z to 18:51:41Z): verdict `QUIET`, 0 s stalled on
  IO; load peak 33.2, mean 27.8; another project's mutation build alive for 899 s of the 900.

## The rig (plan.md step 2)

Copied from `viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup/evidence/` at 18:52:26Z.

| file | `cmp` against the last chunk's | reading |
|---|---|---|
| `live-pty.py` | exit 0 | byte for byte |
| `live-start.sh` | exit 0 | byte for byte |
| `live-drive.py` | exit 0 | byte for byte |
| `live-ledger.py` | exit 0 | byte for byte |
| `live-read.py` | not compared: extended | the copy, plus three fields on a `reading` line (`sent_cr_count`, `sent_lf_count`, `prompt_equals_sent_under_the_rule`), the reader's own oracle `under_the_rule`, and the `selftest` verb |

- `live-read.py selftest`: `selftest: 16 cases, 0 mismatches`, exit 0 (eight rows it must read true, eight it must
  read false, among them a prompt that still holds the CR and a prompt that lost a line break).
- `live-drive.py selftest`: `selftest: 18 cases, 0 mismatches`, exit 0.

**The private directory**, outside the tree, mode 0700: `icr-rig/` under the implementing session's scratchpad,
`/tmp/claude-<uid>/<project-dir>/32a7fb5f-cd74-44f1-a982-8fd59c440595/scratchpad/icr-rig`, where `<project-dir>`
is the repository's absolute path with every `/` written as `-`. It holds the sent texts, the journal, the row
files and each product call's stdout and stderr. It stands on the tmpfs behind `/tmp` and is gone at a reboot.

The six live texts were written there at 18:53Z, each mode 0600, synthetic, with no newline ending:

| id | bytes | CR | LF |
|---|---|---|---|
| `rule-inner-lf` | 69 | 0 | 1 |
| `rule-inner-crlf` | 70 | 1 | 1 |
| `rule-inner-cr` | 71 | 1 | 0 |
| `rule-inner-cr-cr` | 71 | 2 | 0 |
| `rule-inner-lf-cr` | 71 | 1 | 1 |
| `rule-crlf-lines` | 82 | 2 | 2 |

## The product build after the block's first run (plan.md step 8)

The release-check entry (gate entry 9: `cargo clean --release -p viola`, then `scripts/release-check.sh`, both
under `CARGO_TARGET_DIR=target/release-check`) rebuilt `target/release-check/release/viola` on the changed source
at 18:58:31Z: green, last line `release-check: viola only`; sha256 prefix `b4659b98029d0f94`, 4 000 408 B, read
at 19:01:00Z. This is the build the rehearsal and the live start ran on; the live home pinned its own copy of it
(`bin/0.1.0-b4659b98029d0f94/`).

## Before the live start (plan.md step 10)

Read at 2026-10-09T19:04:03Z, after the rehearsal and before the start's ledger row (19:04:12Z). Each line is what
the command printed.

| # | precondition | reading | holds |
|---|---|---|---|
| 1 | the 2.1.287 binary answers by path | `2.1.287 (Claude Code)`, exit 0 | yes |
| 2 | the stamped home holds the one version | one key, `2.1.287`; 17 rows, all `pass` | yes |
| 3 | the homes' backing | a link, exit 0; `tmpfs` | yes |
| 4 | the build equals step 8's | prefix `b4659b98029d0f94` | yes |

Read beside them: no rig process stood (0 rig hosts, 0 processes of the product build, 0 of the harness's fake
agent, 0 of `claude` 2.1.287 by path); the host (`hostwatch.py read --last 5 --for viola`, 18:59:03Z to
19:04:03Z) read `QUIET`, 0 s stalled on IO, load peak 14.0, mean 6.9. Over the live window itself (19:03:00Z to
19:05:30Z) it read `QUIET`, load peak 5.2, no build tool seen.

## The product build after the whole block (plan.md step 12): the file's hash is not step 8's

**The plan's sentence "it must equal step 8's" does not hold for the file, and holds for what the file loads.**
The whole block's release-check entry rebuilt the binary at 19:08:24Z: green, last line `release-check: viola
only`; sha256 prefix `4057b91d84010eca`, 4 000 416 B. The live start ran on `b4659b98029d0f94`, 4 000 408 B.

Between the two builds one file of the build graph was written: `crates/viola-agent-claude/src/hook.rs`, at
19:06:50Z, for step 11's sentence in `typed_text`'s doc comment (the ids of the confirmed readings). The edit
keeps the file's line count and the line of every code line; it changes comment text only.

What was measured, each a rebuild through the same entry (`gate.py run … --entry 9`), green every time:

| # | the tree | when read | sha256 prefix | size |
|---|---|---|---|---|
| 1 | step 8: the rule, the comment without the ids | 19:01:00Z | `b4659b98029d0f94` | 4 000 408 B |
| 2 | step 12: the comment with the ids | 19:09:34Z | `4057b91d84010eca` | 4 000 416 B |
| 3 | the same tree, built again | 19:10:43Z | `4057b91d84010eca` | 4 000 416 B |
| 4 | the comment put back to its text at build 1 (the same 24 lines, the same widths) | 19:11:09Z | `b4fc939b7085eb39` | 4 000 416 B |
| 5 | that tree, built again | 19:13:14Z | `b4fc939b7085eb39` | 4 000 416 B |
| 6 | the comment with the ids again (the final tree; `hook.rs` byte-identical to build 2's) | 19:14:28Z | `4057b91d84010eca` | 4 000 416 B |

- The build is repeatable for a given source text (builds 2, 3 and 6; builds 4 and 5), and the file's hash moves
  with comment text alone (2 against 4).
- Build 4 did not give build 1's hash back. **Why build 1's file differs from build 4's is not known.** No text
  I could restore reproduces build 1's bytes, and build 1's source state is not kept anywhere (it was never
  committed).
- Where the files differ, read from build 1's pinned copy in the live home against the final build 6:
  - the strings: three compiler-named local symbols, `anon.<hash>.<n>.llvm.<number>`, whose last number differs
    (19 digits in build 1, 20 in build 6: the 8 bytes of size); no other string differs;
  - with the symbol tables stripped (`strip`), the two files are the same size (2 990 704 B) and differ in 20
    bytes, offsets 813 to 832: the linker's build-id note;
  - `.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame` and `.gcc_except_table` each hash the same in both.

So the code and data the final build loads are byte-identical to the build the live start ran on; the two files
differ in the build-id note and in three local symbol names. Reported as a deviation from step 12 as written.
The final tree is build 6's: the comment carries the ids. No start was made on builds 2 to 6.

## The re-entry's reading of the loaded sections (the revised plan.md step 12)

Read at 2026-10-09T19:32:06Z, after the re-entry's whole firing of the block (19:29:37Z to 19:31:48Z: 23 entries,
20 green, 0 red, 3 not run, the three the operator leg's). The tree was read first as the stopped run's: HEAD
`3c5e012b9a43`, the five files of the plan's touchpoints modified and no other file under `src`, `crates` or
`tests`; it read the same after the block.

The block's release-check entry (gate entry 9) rebuilt `target/release-check/release/viola` at 19:30:45Z: green,
last line `release-check: viola only`, the artifact fresh.

| file | sha256 prefix | size |
|---|---|---|
| the live start's build, the copy the live home pinned (`target/e2e-home/viola-live-4043089/home/bin/0.1.0-b4659b98029d0f94/viola`) | `b4659b98029d0f94` | 4 000 408 B |
| the rebuilt file (`target/release-check/release/viola`) | `4057b91d84010eca` | 4 000 416 B |

The rebuilt file's prefix and size are the ones predicted. The two file hashes differ, as they did at 19:14:28Z
and at 19:19:17Z.

Each section taken from both files with `objcopy -O binary --only-section=<name>` (exit 0 on every one of the
twelve calls) and hashed with `sha256sum`:

| section | the live start's build (sha256 prefix, size) | the rebuilt file (sha256 prefix, size) | equal |
|---|---|---|---|
| `.text` | `74c10f368cdcf6ed`, 2 163 139 B | `74c10f368cdcf6ed`, 2 163 139 B | yes |
| `.rodata` | `bc436857831a4abd`, 175 984 B | `bc436857831a4abd`, 175 984 B | yes |
| `.data` | `a54af246ec14e57a`, 4 760 B | `a54af246ec14e57a`, 4 760 B | yes |
| `.data.rel.ro` | `5fdb009c7b4114cd`, 78 448 B | `5fdb009c7b4114cd`, 78 448 B | yes |
| `.eh_frame` | `84781f80c870906e`, 255 116 B | `84781f80c870906e`, 255 116 B | yes |
| `.gcc_except_table` | `23eabd13dbfb4708`, 119 368 B | `23eabd13dbfb4708`, 119 368 B | yes |

Six of six equal, so stop rule S6 did not fire. This is the third reading of the six pairs, beside the stopped
run's of 19:14:28Z and the revision's of 19:19:17Z, which read all six equal; the pinned copy still stood on the
tmpfs for it.

The reading's two controls, taken at 19:30:15Z through the same script (kept in the session scratchpad, outside
the tree): the pinned copy against itself read 0 of 6 unequal; the pinned copy against another program of the
host read 6 of 6 unequal.

**The file-hash difference stays recorded as not explained.** Nothing in this reading says why the first build's
file differs from a rebuild, and no step of the re-entry looked for a cause. No start was made.
