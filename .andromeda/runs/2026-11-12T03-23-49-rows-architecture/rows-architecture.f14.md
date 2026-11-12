@row_add n=1 row=contract/config-json/diagnostics-level area=occupied-resources rec=1,2,3,4
boundary: config.json
item: diagnostics_level
promise: "info" by default, or "debug"; diagnostics_level is the only source of the process-log level, and RUST_LOG has no effect.
@row_add n=2 row=contract/config-json/claude-env-keep area=occupied-resources rec=1,2,3,4
boundary: config.json
item: claude_env_keep
promise: The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect.
@row_add n=3 row=contract/config-json/dialog-hold-s area=occupied-resources rec=1,2,3,4
boundary: config.json
item: dialog_hold_s
promise: The dialog hold, a JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected config-json line, so no value makes the hold zero or unbounded.
@row_add n=4 row=decision/recorded-fixtures-scrubbed area=occupied-resources rec=5,6
name: Recorded fixtures scrubbed and hygiene-walked
rule: Fixtures recorded by viola verify --record are written only when no row fails, each is scrubbed and hygiene-walked, and an unclean recording is refused whole, naming the file and the check code, never the content.
guard: contract_fixture_hygiene
@row_add n=5 row=decision/config-precedence area=cross-cutting-patterns rec=7,3
name: Configuration precedence
rule: CLI flags override the viola home's config.json, which overrides built-in defaults; config.json is parsed tolerantly and carries v.
@row_add n=6 row=decision/viola-home-resolution area=cross-cutting-patterns rec=7,3
name: Viola home resolution order
rule: The viola home resolves from --home, then the grandparent of VIOLA_DIR, then .viola under the user home; viola run passes the home to its child only through VIOLA_DIR, so every per-home file follows it.
why: The home is the isolation point for tests: each test runs in its own home.
@row_add n=7 row=absence/secrets area=cross-cutting-patterns rec=7,3
capability: Secrets of any kind in v1 configuration
@row_add n=8 row=decision/capability-ledger-gate area=cross-cutting-patterns rec=8,9,10,11
name: Capability ledger as the single gate for CLI behaviour
rule: Any new dependence on an undocumented claude behaviour is added as a capability-ledger row in viola-agent-claude with a viola verify probe that has a post-condition check, never hard-coded as an unconditional assumption.
except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs.
why: A ruled limit sets the exception.
@row_add n=9 row=decision/bounded-readers area=cross-cutting-patterns rec=12,13
name: Bounded readers at every boundary
rule: Every product reader of external bytes is bounded in its own statement or named as an exception; none reads an external byte source to its end unbounded.
except: The own-executable reads, which are no external input.
guard: contract_bounded_readers: product_readers_are_each_bounded_or_named
