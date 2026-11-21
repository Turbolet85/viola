# P5 controls — revision 1 of 2026-11-20-harness-prefix-and-identity-floor-ledger-rows (plan rev 9/10)

validation.md check 4 (9). Clone: `git clone --shared` of the project at HEAD `b3eb184c`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/42cf1a44-d2b7-4a4c-9451-7298e235e01f/scratchpad/ctl` (the
session scratchpad; `$TMPDIR` = `/tmp`). Inputs planted by
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/42cf1a44-d2b7-4a4c-9451-7298e235e01f/scratchpad/p5_rev1_inputs.py {clone} pass|pre|fail|clean`
(the earlier window's script, re-keyed to `live-stamp-HG.sha256` and given a `pre` mode: the round's pre-landing
state, the committed set without the env record). Each entry FIRED AS ITSELF through `gate.py run --chunk {marker}
--docs-root . --entry {n} --project-root {clone}` (plan rev 9 from the store), save the two round legs 19 and 20,
which the tool does not fire by `--entry` (`not run — leg round (the letter drives it)`): those are typed as the tool
types an entry, `bash -o pipefail -c '{run}'` in the clone, the command unchanged. The clone ends `clean` (`git
status --short` empty). The clone is not deleted (no recursive delete).

| Entry | Unmodified clone | Pass input | Fail input |
|---|---|---|---|
| 3 cut guard | green · exit 0 (`cut: whole`; the tree's red is the stopped run's uncommitted edits, which a clone lacks) | = unmodified (the seven files at `b3eb184c`) | red · exit 1 (`hook.rs` given one trailing LF) |
| 4 name census | green · exit 1, 0 B (no cut name at `b3eb184c`: the pre-existing `prompt_origin_files_the_cross_session_tag_as_harness` and the harness literals do not match) | = unmodified | red · exit 0, 42 B (`src/control_probe.rs` holding `CrossSessionRun`) |
| 19 name diff (typed) | not fired: no record dir | exit 1, `< Env.recorded-1.json` and `> Mcp.tools-list-1.json` — the atoms hold | exit 1, both atoms still hold plus `< Stray.extra-1.json`: 19 checks content only; the exactness is entry 20's |
| 20 name diff count (typed) | not fired | exit 1, last line `2` | exit 1, last line `3` — red |
| 22 kept stamp | red · exit 1 (no `floor-stamp-2.json`) | green · `28 ok` | red · last `28 short` (identity-floor `fail`) |
| 24 live stamp | red · exit 1 (no `live-stamp-HG.sha256`) | green · `live-stamp: unmoved` (the file = the live stamp's real sha, its sha alone read) | red · exit 1 (a sha of zeros) |
| 25 `diff -r` | red · exit 2 (no record dir) | green · exit 0, no output (record = committed set + Env, Mcp excluded) | red · exit 1 (a stray file, `Stop.default.json` given a trailing space) |

Gate log dirs: `$TMPDIR/andromeda-gate/2026-11-20-harness-prefix-and-identity-floor-ledger-rows/run-20261121T0803{15..42}Z`.

**Key paths of 22 and 23 over a real kept stamp** (typed, the path operand `floor-stamp-2.json` → `HF`'s
`target/live-rig/floor-stamp.json`; for 23 the `added` print reduced to its count): 22 printed
`browser-tools-prompt: absent` / `29 short`, exit 0; 23 printed `added: 13 names`, `unknown: CLAUDE_CODE_CHILD_NOTES_DIR
CLAUDE_PLUGIN_DATA`, `other_count: 14`, `unshaped: 0`, exit 0. The literal `data.versions.2.1.328.rows` /
`.measured.env_probe` and its keys are the rig's real stamp shape.

**Entry 9's red cause** (`gate.py show --n 9 --firing 1` of this run dir): 62 run, 57 passed, 5 failed — the five
stamped-home cases, each `failing rows ["cross-session-prefix"]`: the 29th row the cut R2 removes. The cut not yet
made on the tree: the red the plan predicts.
