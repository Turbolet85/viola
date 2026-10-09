# The live work's preconditions (plan.md step 1)

Read at 2026-10-09T15:46:54Z, before the rehearsal and before any live start. Each line is what the command
printed, not what the plan forecast.

| # | precondition | command | reading | holds |
|---|---|---|---|---|
| 1 | the 2.1.287 binary answers by path | `"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` | `2.1.287 (Claude Code)`, exit 0 | yes |
| 2 | the stamped home stands with one version, 17 rows `pass` | `jq` over `target/e2e-home/viola-live-4043089/home/ledger/stamps.json` | `v` 1, `writer` `verify`, `written_at` `2026-10-08T07:27:51.144Z`; `data.versions` holds one key, `2.1.287`; its `rows` hold 17 values, all `pass`; the file is mode 0600, 857 B | yes |
| 3 | the pre-change product build | `sha256sum target/release-check/release/viola` | prefix `2bf1b8ab19e16c8e`; 3 997 632 B, built 2026-10-08 | yes |
| 4 | the homes' backing | `test -L target/e2e-home`, then `findmnt -n -o FSTYPE -T target/e2e-home/` | a link, exit 0; `tmpfs` | yes |

No precondition failed, so stop rule S2 did not fire.

Read beside them:

- The implementing session is bridge-wrapped: its environment holds `VIOLA_BIN`, `VIOLA_DIR` and `VIOLA_NAME` and
  ten `CLAUDE*` names (names only). The rig's host declares the session's environment (`live-pty.py`: `HOME`,
  `USER`, `LOGNAME`, `SHELL`, `PATH`, `LANG`, `XDG_RUNTIME_DIR`, `TERM=xterm-256color`), so none of the thirteen
  crosses into a session, and every `live-drive.py` call ran with the three `VIOLA_*` names removed and `--home`
  given.
- `live-drive.py selftest`: `selftest: 18 cases, 0 mismatches`, exit 0. The file is a byte-for-byte copy of the
  2026-10-08 chunk's (`cmp` exit 0), and so is `live-start.sh`.
- The host before the rehearsal (`hostwatch.py read --last 15`, 15:35:17Z to 15:50:17Z): verdict `QUIET`, 0 s
  stalled on IO; load peak 50.0, mean 35.3; another project's mutation build alive for 863 s of the 900.
- The build's hash was read again right before each of starts 1 and 2: `2bf1b8ab19e16c8e` both times.

## The product build after the gates (plan.md step 12)

The release-check entry (gate entry 14: `cargo clean --release -p viola`, then `scripts/release-check.sh`, both
under `CARGO_TARGET_DIR=target/release-check`) rebuilt `target/release-check/release/viola` on the changed source.

| firing | when | reading |
|---|---|---|
| the first run of entries 1 to 14 | 16:10:44Z | green, last line `release-check: viola only`; sha256 prefix `62bf6028d95fe34e`, 3 998 456 B |
| the second run, after the three remove-the-guard readings were restored (`guard-red-green.md`) | 16:16:35Z | green, the same last line; sha256 prefix `62bf6028d95fe34e`, 3 998 456 B |

The two firings built the same bytes from the same source, so the build is the changed tree's. Start 3 ran on it:
the hash was read again right before the start's ledger row (16:17:25Z) and read `62bf6028d95fe34e`.

Before start 3 the first, second and fourth preconditions were read again (16:17:17Z): the by-path probe printed
`2.1.287 (Claude Code)`, the stamped home's `stamps.json` still holds the one version `2.1.287`, and the backing
read `tmpfs`.

## Before start 4 (plan.md step 16)

Read at 2026-10-09T16:45:50Z, in the re-entry after the plan's revision (`inputs#I7`, `inputs#I8`), before the
start's ledger row. Each line is what the command printed.

| # | precondition | command | reading | holds |
|---|---|---|---|---|
| 1 | the 2.1.287 binary answers by path | `"$HOME/.local/share/mise/installs/claude/2.1.287/claude" --version` | `2.1.287 (Claude Code)`, exit 0 | yes |
| 2 | the stamped home holds the one version | `jq` over `target/e2e-home/viola-live-4043089/home/ledger/stamps.json` | `v` 1, `writer` `verify`, `written_at` `2026-10-08T07:27:51.144Z`; `data.versions` holds one key, `2.1.287`; its `rows` hold 17 values, all `pass`; mode 0600, 857 B | yes |
| 3 | the homes' backing | `test -L target/e2e-home`, then `findmnt -n -o FSTYPE -T target/e2e-home/` | a link, exit 0; `tmpfs` | yes |
| 4 | the build start 3 ran on | `sha256sum target/release-check/release/viola` | prefix `62bf6028d95fe34e`; 3 998 456 B | yes |

No precondition failed, so stop rule S2 did not fire.

Read beside them:

- The re-entering session is bridge-wrapped as the first was: its environment holds `VIOLA_BIN`, `VIOLA_DIR` and
  `VIOLA_NAME` and ten `CLAUDE*` names (names only). The rig's host declares the session's environment, so none
  crosses, and the one `live-drive.py` call runs with the three `VIOLA_*` names removed and `--home` given.
- No process of the rig stood before the start: 0 `live-pty.py` hosts, 0 processes of the product build, 0 of
  `claude` 2.1.287 by path, 0 of the harness's fake agent.
- The host (`hostwatch.py read --last 15`, 16:30:50Z to 16:45:50Z): verdict `QUIET`, 0 s stalled on IO; load peak
  46.6, mean 24.6; another project's mutation build alive for 660 s of the 900.
- The build's hash was read again right before the start's ledger row (16:46:29Z): `62bf6028d95fe34e`.

## The product build after the whole block of the re-entry (plan.md step 17)

Start 4 was over before the block was fired. The block's release-check entry (gate entry 14) cleaned and rebuilt
`target/release-check/release/viola` at 16:50:53Z: green, last line `release-check: viola only`; sha256 prefix
`62bf6028d95fe34e`, 3 998 456 B, read at 16:52:23Z. It is the third firing to build the same bytes from the same
source: no product file changed between start 3, start 4 and this firing.
