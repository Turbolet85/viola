# Classified — architecture · occupied-resources.b1

## Answers
occupied-resources#1.1 · no · the session moves no socket path, endpoint or start-arbiter lock (the untouched IPC-endpoints line)
occupied-resources#2.1 · no · the reached root Cargo.toml's only described change is the tokio pin, so the `[[bin]]` behind `fake-agent` still holds on the changed file (the untouched binaries line)
occupied-resources#3.1 · no · no subcommand, verb argument or hidden verb changed (the untouched binaries line)
occupied-resources#4.1 · no · no hidden flag or capture arm is added or changed (the untouched binaries line)
occupied-resources#5.1 · no · no flag passed to the child and no `SETTINGS_FLAG` or `PROGRAM` constant changed (the untouched Claude-integration line)
occupied-resources#6.1 · no · the probe plugin and verify's child flags are unchanged (the untouched Claude-integration line)
occupied-resources#7.1 · no · the statusline settings override and its path rule are unchanged (the untouched Claude-integration line)
occupied-resources#8.1 · no · `VIOLA_DIR` and its home derivation are unchanged (the untouched env-var line)
occupied-resources#9.1 · no · the R8 strip and the identity-floor names are unchanged (the untouched env-var line)
occupied-resources#10.1 · no · the harness env names and the CI workflows that read them are unchanged, and the session reaches no workflow
occupied-resources#11.1 · no · the reached root Cargo.toml's only described change is the tokio pin, so the `fake-agent` feature gate still holds on the changed file (the untouched env-var line)
occupied-resources#12.1 · no · the `config.json` keys, defaults and reads are unchanged (no config changed)
occupied-resources#13.1 · no · the `diagnostics/` path and the Tokio client's channel methods and wire format are unchanged (the untouched home-layout and channel-methods lines)
occupied-resources#14.1 · no · the `instances/` file set and its modes are unchanged (the untouched home-layout line)
occupied-resources#15.1 · no · the `budget.json` path, mode and checked read are unchanged (the untouched home-layout line)
occupied-resources#16.1 · no · no ledger row, probe or stamp moved, so `ledger/stamps.json` and its writer hold as written
occupied-resources#17.1 · no · the `ledger/probes/<pid>/` layout is unchanged and no probe or capture path moved
occupied-resources#18.1 · no · verify's interactive probe dirs and their cleanup are unchanged, since no source changed
occupied-resources#19.1 · no · the test-only watch dir and the `Watch` bound are unchanged, since no test source changed
occupied-resources#20.1 · no · the `/tmp/viola-e2e-home-<uid>/` home and its headroom floor are unchanged, since no harness source changed
occupied-resources#21.1 · no · the committed fixture sets and their counts are unchanged, and no ledger row, probe or stamp moved
occupied-resources#22.1 · no · the `target/agent-run/` layout and its writer are unchanged, since no harness source changed
