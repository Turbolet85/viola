
## 2026-10-03-mutation-scoring-completion — run --mutants --package; the diff prefix pin; the boundary tier's form
**Section:** §3 → 5-command implementation (Command body; `run` step 4 Diff, Classification, Verdict, Test scope, Package; Output format; Closed enums; Test selection) · §2 Test Strategy (Mutation row) · §10 Quality Gates (Mutation gate)
**Change:**
- New `run --mutants --package <member>` (clap `requires = "mutants"`): one whole member, no Base, no `chunk.diff`, no Classification. The member must be `viola` or a `crates/<member>` whose manifest `[features]` declares `fake-agent`, named in lowercase ASCII, digits, `-`, `_`; anything else is `reason:"package-refused"`, exit 2, before any cargo. `cargo mutants --package <member> --features fake-agent [--file …] --test-tool=nextest --copy-target=true …` follows the root prebuild in `target/mutants`. Document `{"tested":N,"verdict":"package","package":"<member>"}` (+ `files`, + `scratch_bytes` on Windows). `mutants.verdict` adds `package`; `run` `reason` adds `package-refused`.
- Diff: both `git diff` argvs pin `--src-prefix=a/ --dst-prefix=b/` (a host `diff.mnemonicprefix` / `diff.noprefix` broke the header parse).
- Test scope: the viola-e2e scenarios run only against viola-e2e's own mutants (`--package viola-e2e`).
- §10: the boundary tier scores a whole member with `run --mutants --package <member>`. A mutant the host cannot compile or reach is "not measured here; owed to {route entry}" by coordinate, never "equivalent", and `missed == 0` reads over the measurable set. On the Linux dev host every mutation run takes a NOCOW btrfs `TMPDIR` (the `/tmp` quota and the reflink exec-bit loss, as measured at the chunk's `evidence/m3.md`).
**Why:** M3, a tested harness arm rather than a recipe (overseer, founder-delegated, at plan review). The not-measured vocabulary is the overseer's ruling. The `TMPDIR` rule is the overseer's direction.
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/
