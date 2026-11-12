@row_add n=1 row=decision/thiserror-one-enum-per-crate area=established-decisions rec=2,1
name: thiserror, one error enum per workspace crate
rule: Each workspace crate has one thiserror error enum, the refusal reason type lives in the core crate, and anyhow is used only in the root viola bin.
why: Callers branch on refusal variants across three surfaces: channel, CLI exit codes, MCP isError.
except: Ratified exceptions: viola-pty's hand-written PtyError; viola-mcp holds ServeError beside McpError; viola-state holds StampsError beside StateError; PinError, StampsReadError and Refused.
@row_add n=2 row=contract/channel-link/already-linked area=standard-contracts rec=4,3
boundary: channel link method
item: already_linked
promise: A link of a pair the set holds appends nothing and answers that pair's since with already_linked true; a new pair answers its new link line's ts as since with already_linked false.
@row_add n=3 row=decision/verify-child-set area=established-decisions rec=5
name: Verify's bounded child set
rule: viola verify makes one print-mode probe, five interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.
why: Each widening of the child set was ratified.
