
## 2026-10-17-cli-machine-contract — architecture: the CLI verbs take the VIOLA_DIR step and canonicalise the home (not ratified)
**Section:** §Occupied Resources Environment variables (`VIOLA_DIR`); §Cross-cutting Patterns Config management (Viola home, the statusline source rule)
**Change:**
- `VIOLA_DIR` is read by `hook` and every CLI verb, `run` and `revive` included (was: "`hook` is its only reader: the CLI verbs do not read it yet"); the `mcp` verb is not built.
- `resolve_home(flag, name, dir, user_home)` (`src/cmd/mod.rs`) resolves `--home` as given, else the `instance_of` home when `VIOLA_NAME` and `VIOLA_DIR` pass the one shape check, else `<user home>/.viola`, a failing pair falling through, never refusing; `resolved_home` reads the two variables and `std::env::home_dir()` once at every verb arm (was: the CLI verbs take `--home`, else the default; the step owed to "CLI machine contract").
- `canonical_home` canonicalises a home that exists (`std::fs::canonicalize`; Windows `\\?\C:\…` written back as `C:\…`, other verbatim forms kept) and keeps `std::path::absolute`'s spelling for one that does not, creating nothing; `run`'s start canonicalises after its wrapper log created the home, before the collision check, the pin and the bind.
- The statusline source's default-home test is `canonical_home(default_home(user_home))`; a home named by `--home` or reached through `VIOLA_DIR` reads no user settings; the `home_dir()` read is `resolved_home`'s.
**Why:** a boundary widening (a CLI verb's home is chosen by an environment variable) landed as built and NOT ratified: the Viola overseer's answers (P4 `inputs#I2`, this wrap's Q1 `inputs#I3`) are a delegate's, so the founder's own word is owed at his next batch.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/
