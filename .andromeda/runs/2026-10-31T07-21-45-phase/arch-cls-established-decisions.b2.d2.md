# Classified — arch · established-decisions.b2

## Answers
established-decisions#20.1 · no · the hook's connect and server check are not reached: the hook is none of the three surfaces (Not needed, hook transport tiers) and client.rs edits only fault_of, so connect_by and verify_server stay as they are
established-decisions#21.1 · yes · the rule that content-bearing detail goes only to diagnostics/detail-hook.ndjson is the diagnostic-channel rule the v1-47 detail-file leg reads, and the dialog_hold_s default 60 is a fallback value the config.json cap cases assert
established-decisions#22.1 · no · the dialog-expired wait-feed line is no error body and no detail report, and the chunk adds or changes no event line
established-decisions#22.2 · no · a recorded live measurement of a 671 s dialog hold on claude 2.1.287, which the chunk does not re-measure
established-decisions#22.3 · no · the hook's server-check failure path is none of the three surfaces (Not needed, hook transport tiers) and verify_server is a unit of client.rs the chunk does not edit
established-decisions#22.4 · no · the hook's instance check ahead of any config.json read is not the fallback outcome the config.json cap cases assert, and no cap case drives a refused instance
established-decisions#23.1 · no · a pending live reading of a hold near the 3600 s ceiling, which the chunk does not measure
established-decisions#24.1 · yes · the send and answer refusal codes it states (human-typing, turn-running, not-delivered with unknown-dialog or manual-pause detail) and the release-from-driver -32602 cause are error bodies the MCP and channel enumeration must name; the wheel's byte classifier, Windows leg and HARNESS_PREFIXES list do not
established-decisions#25.1 · yes · a link that cannot be recorded is the channel -32603 on answer, with nothing issued and the answer not taken, and the link and unlink channel methods are error sources the enumeration must name
established-decisions#25.2 · yes · the explicit verbs' untrusted-instance refusal through refuse_untrusted is exit 21, the refusal the CLI --json document (client::refused_document) carries and the enumeration must record, and the check ordering stays as it is
established-decisions#25.3 · yes · the chunk changes the channel-response line for the -32602 causes and the scope names its codes-only rule, so a link or unlink pair must carry method and no driver or driven name
established-decisions#25.4 · no · the Utc::now() stamp of the link line's ts is no error body or detail report, links.rs is not reached, and no cap or witness reads a link timestamp
established-decisions#26.1 · yes · send refused budget-paused (same path as human-typing) is a code on the MCP and channel send surfaces, a release --budget whose records do not land is a channel -32603, and the five_hour 90 and seven_day 85 defaults are fallback values the config.json cap cases assert; the pause and lift behaviour does not bind
