@row_add n=1 row=decision/strict-modes-check area=authentication-authorization rec=1,2,3
name: Strict-modes check on trusted state
rule: Every process that reads snapshot.json, ledger/stamps.json or statusline_command refuses when the home, its trusted folders or a trusted file is group- or world-writable, not owned by euid, or a symbolic link, and when the home or instance folders grant any group or other permission.
guard: security_negatives_strict_modes, chaos_snapshot_replay
@row_add n=2 row=decision/marketplace-off-integrity-set area=authentication-authorization rec=1,4
name: marketplace/ off the integrity set
rule: marketplace/ is no trusted file and is off the integrity set: viola never reads it back, and the installed command gets no re-hash.
@row_add n=3 row=contract/instance-unreachable-detail/instance-dead area=authentication-authorization rec=5,6,7,8,9
boundary: instance-unreachable detail
item: instance-dead
promise: A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with the instance directory standing (a stale wrapper included), or a failed connect; the reading opens nothing and writes no line.
@row_add n=4 row=contract/instance-unreachable-detail/unwrapped-name area=authentication-authorization rec=5,6,7,9
boundary: instance-unreachable detail
item: unwrapped-name
promise: A cause of exit 21 and of MCP's instance-unreachable result: no endpoint with anything else at the instance path (absent, a regular file or a symlink); the reading opens nothing and writes no line.
@row_add n=5 row=decision/actions-pinned-by-sha area=dependency-security rec=10
name: CI actions pinned by commit SHA
rule: Every action ci.yml uses is pinned by full commit SHA.
