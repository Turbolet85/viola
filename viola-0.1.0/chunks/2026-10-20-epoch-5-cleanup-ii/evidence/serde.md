# The `viola-mcp` → `serde` reading (step 6)

Read 2026-10-20 on the working tree over HEAD 844de97682c8, after item (5)'s moves.

- Edit: `serde.workspace = true` dropped from `crates/viola-mcp/Cargo.toml` `[dependencies]`.
- `cargo check -p viola-mcp --all-targets`: exit 0 (`Checking viola-mcp v0.1.0`, `Finished`), no error, no warning.
- `cargo machete crates/viola-mcp` (cargo-machete 0.9.2): exit 0, "didn't find any unused dependencies".
- `Cargo.lock`: one line removed (`viola-mcp`'s `"serde"` dependency entry); no package added or removed.

Verdict: **green** — the edge goes. No machete `ignored` entry is written in any manifest. The removal departs from
architecture §Infrastructure Patterns → Crate dependency direction, which lists the edge: an expected amendment for the
wrap (plan §Implementation notes).
