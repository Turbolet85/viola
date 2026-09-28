# Guard runs (plan step 6) — 2026-09-28, implement run 2026-09-28T09-07-16

Each guard was neutralised, run, restored and run again. The site was re-read before each edit, and the edit was
confirmed landed by grep before the run.

| guard | neutralised | command | reading | file |
|---|---|---|---|---|
| clap without `color` | `"color"` put back into the workspace clap feature list | `run --integration --filter 'binary(=cli_output_plain)'` | red: exit 1, ok:false, 3 failed / 1 passed (all three witnesses fail; the SGR parser self-check passes); archive 161 | `guard-colour-restored-red.json` |
| clap without `color` | restored (Cargo.toml and Cargo.lock byte-identical to the post-step-1 copies) | same | green: exit 0, ok:true, 4 passed; archive 162 | `guard-colour-removed-green.json` |
| one write for the pair | `write_refusal` as two `writeln!` | `run --unit --filter 'test(/write_refusal/)'` | red: exit 1, ok:false, `human::tests::write_refusal_writes_the_pair_in_one_write` failed, 3 passed; archive 163 | `guard-two-writes-red.json` |
| one write for the pair | restored | same | green: exit 0, ok:true, 4 passed; archive 164 | `guard-one-write-green.json` |

The red terminal case on Windows (ConPTY) read these SGR attributes from `viola --help`, bold `1` and underline `4`
among them: `[0, 1, 4, 22, 24, 1, 22, 1, 4, 22, 24, 1, 22, 1, 22, 1, 4, 22, 24, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22]`.
The pipe cases failed on `ESC on stdout` (`--help`) and `ESC on stderr` (`frobnicate`).
