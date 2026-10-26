# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · yes · the verbs' process-start (subject `self`, to `cli-<name>.ndjson` when an instance resolves) takes only these fields and subject values
log-coverage#1.2 · yes · the verbs' process-exit on the connect and internal-fault paths (`exit_code` 21 or 1, `during:"connect"`, `detail`, `duration_ms`) takes only these fields
log-coverage#1.3 · yes · the wrapper's channel-request for `link` / `unlink` carries `method` and `conn`, and the self-reported driver name is never logged as `from` or `from_trust`
log-coverage#1.4 · yes · the channel-response for `link` / `unlink` takes these `result_class` and `error_code` values, -32602 at WARN for the two refusals and -32603 for a fault
log-coverage#1.5 · no · the chunk adds no send line and reaches no send path
log-coverage#1.6 · no · the dialog and answer path is untouched, its implicit link on a taken answer standing as built (Not needed: the Dialog → answer scenario)
log-coverage#1.7 · no · no hook command or hook role file changes (Not needed: edge flow E1 and the hook perf budgets)
log-coverage#1.8 · no · the panic hook and its line are generic and untouched by the chunk, a re-check every change owes
log-coverage#1.9 · no · the chunk adds no release line, so `release` and its `from` are untouched
log-coverage#1.10 · yes · the unlink's append to `events.ndjson` is the call that heals a torn last line and writes its `state-recovered` line (`torn-line-healed`, basename `file`, `offset`)
log-coverage#1.11 · yes · a logged link name that fails `ViolaName::try_new` is a closed skip, and its `parse-rejected` line takes a value of this `parser` list
log-coverage#2.1 · yes · the verbs' exit-1 causes (`internal-error` and each new closed code) join this detail list and the schema's 13-value enum
log-coverage#2.2 · yes · link and unlink join the six CLI frame verbs, so the connect-path exit-21 `server-verify-failed` line (no pid, start time, path or endpoint) reaches them
log-coverage#2.3 · no · the verbs' exits are 21 and 1 (Needs), and no verb path the chunk adds writes exit 20 `wrapper-fault`
log-coverage#2.4 · no · the hook fail-open codes belong to the hook path, which the chunk does not change
log-coverage#2.5 · yes · a logged link name that fails `ViolaName::try_new` is a closed skip whose `parse-rejected` detail is `malformed`, allowed for any parser
log-coverage#2.6 · no · the link refusals are channel -32602 errors, not RefusalReason details, and the send refusal set is untouched
log-coverage#2.7 · yes · a P4 strict-modes check ahead of the snapshot read falls under this Vector 7 rule, under which no exit-21 arm of this build writes the word
log-coverage#3.1 · no · the chunk spawns no child or shell and adds no agents read (Not needed: verify, the agents boundary, statusline shell-out)
