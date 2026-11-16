@row_set n=1 row=contract/hook-to-kind-map/stopfailure cell=promise seen=1 fact=F1 detector=none
value: StopFailure maps to `turn-ended`, not a dialog: message and `background_tasks` null, `cause` the CLI error word's class (rate_limit rate-limited, five auth words auth-failed, billing_error billing, max_output_tokens output-limit, other known words by name; absent, unknown or other api-error).
basis: fct-xw6ru4by
because: StopFailure's `cause` is no longer always `api-error` but the class of the CLI error word, so the map row's fixed-word promise is false.
@row_set n=2 row=contract/turn-end-cause/cause-set cell=promise seen=1 detector=none
value: `turn-ended` has a closed cause set, TurnEndCause, in viola-core beside UnreachableCause, of nine words (rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error) with no Claude-specific name; a `turn-ended` cause is one of its words.
basis: fct-ulztjw6q
because: TurnEndCause grew from the one member `api-error` to nine, so the row naming one member is false.
@row_set n=3 row=contract/event-data/turn-ended-cause cell=promise seen=1 detector=none
value: `cause` is present only on an API-error turn end, one of the nine TurnEndCause words, and absent on a Stop end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`.
basis: fct-rexe2a5m
because: The cause is now one of nine words rather than `api-error` alone, and the nine words stand ratified, no longer applied provisionally.
@row_set n=4 row=decision/unparseable-external-values-unknown cell=except seen=3 detector=none
value: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.
basis: fct-xw6ru4by
because: An unrecognised StopFailure error string now becomes the cause word `api-error`, not the string `unknown`, an exception the convention row must name.
@row_set n=5 row=decision/claude-shapes-in-one-crate cell=rule seen=1 fact=F3 detector=none
value: viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json; the API-error classification stays in it, and only the agent-neutral turn-end cause word crosses out, never the CLI error word or text.
basis: fct-vldmp3ur
because: The StopFailure error-word classification lives only in the adapter and only the TurnEndCause word leaves it, which the crate-boundary row says less of.
@row_add n=1 row=decision/unknown-stopfailure-error-word-logged area=conventions fact=F2 detector=none
name: Unknown StopFailure error word logged by code only
rule: A StopFailure error string outside the CLI's known words makes the hook write one codes-only parse-rejected line (parser hook-stdin, detail unknown-value, count 1) and still deliver the event; the outside value enters no line.
state: provisional
basis: fct-rexe2a5m
because: The hook now logs an outside StopFailure error word as a codes-only parse-rejected unknown-value line, applied ahead of ratification on the overseer's technical answer.
