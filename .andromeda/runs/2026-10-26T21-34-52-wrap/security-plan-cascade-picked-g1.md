# security-plan · picked accounting · group 1 of 2

## Picked
threat-model-summary#2.1 · proposed · its as-built line still says every client reads the snapshot without strict-modes except link / unlink, a gap the report retires
threat-model-summary#5.1 · proposed · the hook event, dialog and statusline paths now check the instance first and statusline runs nothing over a replayed snapshot
threat-model-summary#6.1 · proposed · the MCP frame tools now refuse on the instance's strict-modes check before liveness, which its trust boundary does not name
threat-model-summary#7.1 · proposed · the CLI trust boundary gains the home-tree and instance strict-modes checks that now gate what `--home` / `VIOLA_DIR` choose
threat-model-summary#8.1 · proposed · the replay's product readers went from two to six and «nothing checks integrity beyond the content hash» no longer holds
authentication-authorization#1.6 · proposed · its dated strict-modes gaps are retired, the collision check reads the caller's one replay-aware read, and F14 belongs beside it
authentication-authorization#1.8 · proposed · its dated interim gaps are retired, `check_instance`'s set widened, the Windows instance case built, and F1, F2, F4, F5, F6, F11, F12, F13, F15, F16, F21 belong beside it
authentication-authorization#3.2 · proposed · the verdict's reference values now come from `read_snapshot_or_replay`, not `read_snapshot`
authentication-authorization#3.3 · proposed · link / unlink's check moved onto `client::refuse_untrusted`, now shared with the six frame verbs, and the eight gaps it set itself apart from are retired
authentication-authorization#3.4 · proposed · every frame verb's strict-modes refusal now comes before its liveness and server checks, F3 belongs beside it
