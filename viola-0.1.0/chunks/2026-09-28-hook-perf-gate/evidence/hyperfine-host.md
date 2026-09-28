# hyperfine on the host — plan step 0

Installed at the CI pin before any perf gate ran: `cargo install --locked hyperfine@1.20.0` (Finished `release` in
27.89 s, `Installed package hyperfine v1.20.0`), measured 2026-09-28T05:45Z.

- `hyperfine --version` → `hyperfine 1.20.0`
- `hyperfine --help` lists every flag the perf arm uses:
  - `-N` (help line 97, "An alias for '--shell=none'"), and `--shell <SHELL>` whose help names `none` (line 94);
  - `--input <WHERE>` (line 169);
  - `-w, --warmup <NUM>` (line 14);
  - `-r, --runs <NUM>` (line 22);
  - `--export-json <FILE>` (line 131).

No flag was missing, so no substitute was needed.
