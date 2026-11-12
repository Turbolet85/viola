@row_add n=1 row=decision/mcp-server-rmcp area=stack-and-technologies rec=1,2
name: rmcp is the MCP server
rule: The MCP server is an rmcp stdio server with schemars tool input schemas, rmcp minor pinned, a thin adapter over viola-channel.

@row_add n=2 row=decision/plugin-embedded-in-binary area=stack-and-technologies rec=3,2
name: Claude Code plugin compiled into the binary
rule: The Claude Code plugin is compiled into the binary (include_str!) and written out by viola run.

@row_add n=3 row=decision/malformed-stamps-unverified area=standard-contracts rec=4
name: Malformed or untrusted stamps read unverified
rule: A non-`v:1` or wrong-shaped stamps envelope, or a strict-modes refusal, makes run's gate record cli_verified:false with one parse-rejected line.

@row_add n=4 row=contract/mcp-list/refusal area=standard-contracts rec=7,8
boundary: MCP list
item: refusal
promise: The board's refusal, its only one, is returned as instance-unreachable with detail strict-modes-failed.
