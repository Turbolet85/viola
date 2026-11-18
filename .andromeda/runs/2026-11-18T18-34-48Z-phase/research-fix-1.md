old: - derived `fixtures/claude/2.1.325/*.json` by `viola verify --record` under the live rig, then `viola-mcp-tap fixture` — the recorded 2.1.325 set
new: - derived `fixtures/claude/2.1.325/*.json` by `viola-live-rig live R4 --cli 2.1.325` — the recorded 2.1.325 set: verify's record kept by the rig, and the Mcp exchange by viola-mcp-tap
