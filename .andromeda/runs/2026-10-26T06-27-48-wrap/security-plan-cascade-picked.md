# security-plan · picked accounting

## Picked
dependency-security#2.1 · proposed · the group states viola-channel's Tokio client as tokio `=1.53.1`; the report's Dependencies bullet moves the workspace pin to `=1.53.3` (Cargo.toml:242), `io-util` and the feature gating unmoved
dependency-security#8.1 · proposed · the line states an open RUSTSEC-2026-0386 against tokio `=1.53.1` failing `supply-chain`, owned by this chunk; the report's Retired bullet closes it (`cargo deny check` exit 0, advisory absent, ci#40278510123 green)
