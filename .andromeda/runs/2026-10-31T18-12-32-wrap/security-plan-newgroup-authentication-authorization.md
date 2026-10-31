# New groups — security-plan · authentication-authorization

## Verdicts
- fanout-security-plan--cascade-g1.md 12 · distinct · no group names `bind_endpoint` or a `create_private_dir` failure at bind; line 18's exit 1 is the directory check's failure, another failure than this one
- fanout-security-plan--cascade-g1.md 13 · contradiction · lines 27–27 · «`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":null}`» · the file gives MCP's no-endpoint case one answer with a null detail, the proposal has `frame` call a reading that tells `instance-dead` from `unwrapped-name` there
- fanout-security-plan--cascade-g1.md 14 · contradiction · lines 27–27 · «MCP returns `instance-unreachable` with `detail: null` and writes no line» · the file holds MCP's detail null on a mismatch, on a strict-modes refusal (line 36 too) and with no endpoint, the proposal puts each one's cause in `detail`
