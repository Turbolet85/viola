# Classified — obs-plan · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · the cited source is the arch CI/CD excerpt, CI is untouched (§9 unchanged) and the reached .config/nextest.toml carries no claim this entity makes
obs-scope-summary#17.2 · no · CI/CD stays not-instrumentable because the session adds no runtime surface and changes no CI step or gate
obs-scope-summary#17.3 · no · the not-product-runtime and fixture-scrub reason holds, since no recorded fixture or scrub shape is added (§8 untouched)
obs-scope-summary#18.1 · no · the cli dispatch functions keep their #[tracing::instrument] spans, the new cli.link and cli.unlink spans having that same shape, and the logger and panic hook are unchanged (§7 untouched)
obs-scope-summary#18.2 · no · cli output stays terminal stdout/stderr in fixed-column text, since link and unlink add one fixed instance-state refusal pair and --json with no prompt, TUI or pager
obs-scope-summary#18.3 · yes · add-to · viola link and viola unlink write process-start and process-exit to cli-<driven>.ndjson, the driven instance's file with or without VIOLA_NAME, so the caller's-file and none-without-VIOLA_NAME wording covers revive alone
obs-scope-summary#18.4 · no · the instance is still the explicit argument or VIOLA_NAME, since link and unlink take the driven name from their argument, and service.name, process and the exporter carve-outs are unchanged
obs-scope-summary#18.5 · no · the exit set 0, 1, 2, 10–14, 20 and 21 is unchanged and link and unlink add no exit or detail code, reusing instance-unreachable and server-verify-failed, with fixed refusal strings in the reached human.rs and verbs.rs
obs-scope-summary#19.1 · no · run's spans are unchanged for the start sequence, the pty seam, viola-state writes and channel server dispatch, and link and unlink add only the existing channel pair and no span of their own (§6 IPC row re-read holds)
obs-scope-summary#19.2 · no · the terminal still carries only the child's screen, since the reached run.rs gains no terminal write and the new arms log to files only
obs-scope-summary#19.3 · no · the run exporter stays a file sink to diagnostics/run-<name>.ndjson with no terminal output, and the new unlink line is a state-log append to events.ndjson that this entity does not describe
obs-scope-summary#19.4 · no · run's service identity (process="run", instance=<ViolaName>) is unchanged
obs-scope-summary#19.5 · no · the panic hook, the 1 s heartbeat tick and the handle-wait exit detection are unchanged in the reached run.rs (§7 and §10 liveness untouched)
obs-scope-summary#20.1 · no · the hook's stdin-parse, channel-call and decision-emit spans are unchanged, since no hook path changed (§10 hook rows untouched)
obs-scope-summary#20.2 · no · the hook frontend stays N/A, since the session adds no hook output
obs-scope-summary#20.3 · no · the hook exporter (hook-<name>.ndjson, stderr forbidden, stdout decision body only) is unchanged, since no hook path changed
obs-scope-summary#20.4 · no · hook service identity (process="hook", instance null without VIOLA_NAME) is unchanged
obs-scope-summary#20.5 · no · the hook latency budget and the PROVISIONAL 5 s statusline shell-out bound are unchanged (§4 budget and §10 hook rows untouched)
obs-scope-summary#21.1 · no · the client-call spans cli.link and cli.unlink and the existing channel-request / channel-response pair cover link and unlink, with corr still the JSON-RPC id and no trace-context field added
obs-scope-summary#21.2 · no · the channel frontend stays N/A, since the session adds no output surface
obs-scope-summary#21.3 · no · the per-side files (run-<name>, hook-<name>, mcp, none for CLI verbs without an instance) still hold, since link and unlink always carry an instance and so the none clause does not reach them
obs-scope-summary#21.4 · no · each side still logs its own process and params.sender is unchanged
obs-scope-summary#21.5 · no · corr, the -32603 internal error with data null and the rule that from is never logged as an identity hold, since link and unlink read no from and keep the -32602 and -32603 shapes
obs-scope-summary#22.1 · no · the MCP tool-call spans cover send, wait, last, answer and list only, since link and unlink are not MCP tools and viola mcp is unchanged
obs-scope-summary#22.2 · no · the MCP frontend stays N/A, since the session adds no output surface
