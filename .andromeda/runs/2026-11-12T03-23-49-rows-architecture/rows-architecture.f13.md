@row_add n=1 row=name/crate/viola-e2e area=occupied-resources rec=1,2,3
class: crate
name: viola-e2e

@row_add n=2 row=contract/child-environment/viola-dir area=occupied-resources rec=4,5
boundary: The wrapped child's environment
item: VIOLA_DIR
promise: Set by viola run in the child to the absolute path of the instance's state directory; hook, mcp and every CLI verb run inside the session derive the viola home as its grandparent, so a session started with --home keeps its hooks, MCP server and nested CLI calls in that home.

@row_add n=3 row=decision/no-live-claude-in-ci area=occupied-resources rec=6,7
name: No live claude in CI
rule: When CI is present, the test harness's run --local-live refuses before any build, spawn or suite, and viola itself never reads the harness variables.
why: so the real claude never runs in CI

@row_add n=4 row=contract/config-json/v area=occupied-resources rec=8,9,10,11
boundary: config.json
item: v
promise: config.json carries v, which must be 1; the file is read with a MAX_FRAME cap.

@row_add n=5 row=contract/config-json/five-hour area=occupied-resources rec=8,9,10,11
boundary: config.json
item: five_hour
promise: The five-hour budget threshold, a JSON whole number from 0 to 100, default 90; any other value takes the default with one parse-rejected config-json line, the other key standing, and no value switches the gate off.

@row_add n=6 row=contract/config-json/seven-day area=occupied-resources rec=8,9,10,11
boundary: config.json
item: seven_day
promise: The seven-day budget threshold, a JSON whole number from 0 to 100, default 85; any other value takes the default with one parse-rejected config-json line, the other key standing, and no value switches the gate off.
