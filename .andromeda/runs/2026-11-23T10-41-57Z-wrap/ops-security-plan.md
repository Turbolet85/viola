@row_set n=1 row=decision/cli-input-trust-boundary cell=rule seen=1 detector=D-security-input
value: CLI arguments, flags and environment are trusted as same-user invocation and checked by clap parsing, the ViolaName nutype, the closed model-word and effort-word checks and tolerant config.json parsing.
basis: fct-objkygma
because: viola run and viola revive gain --model and --effort, checked by the model-word and effort-word value parsers, so the row naming the CLI's checks must name them.
@row_set n=2 row=contract/revive-arguments/id cell=promise seen=1 detector=D-security-input
value: --id accepts only the session-id shape, anything else a usage error with exit 2; --list conflicts with --id, --fork, --model, --effort and the trailing arguments.
basis: fct-ydz7awsz
because: ReviveArgs gains --model and --effort, each conflicting with --list, so the promise of what --list conflicts with must follow.
@row_add n=1 row=decision/launch-flags-checked area=input-validation fact=F1 detector=D-security-input
name: Launch pair flags checked before anything is read
rule: viola run's and viola revive's --model must be a model word, 1 to 64 bytes, a lowercase letter first, then lowercase letters, digits, dots and dashes, and --effort one of the five effort words of claude --help; a bad value is a usage error, exit 2, before anything is read.
basis: fct-ydz7awsz; fct-objkygma
because: The chunk adds the --model and --effort flags, a new external-input surface, held to closed shapes by value parsers before anything is read.
@row_add n=2 row=decision/launch-pair-argv-crossing area=input-validation fact=F2 detector=D-security-input
name: Launch pair handed on the claude child's argv
rule: The launch pair crosses onto the claude child's command line only as four separate argv elements, --model and its word then --effort and its word, of a direct spawn, never through a shell, after the plugin-dir flag and before the caller's words.
why: A ratified crossing; revive handing the recorded pair onto the resumed child's argv is a second ratified crossing.
basis: fct-ydz7awsz
because: The child launch now hands the pair as four argv elements between the plugin-dir flag and the caller's words, and revive hands the recorded pair, a crossing of a logged value onto a child's command line.
@row_add n=3 row=decision/launch-flags-refused-after-dashes area=input-validation fact=F3 detector=D-security-input
name: Launch pair flags refused after the dashes
rule: viola run and viola revive refuse a --model or --effort flag, or its equals form, among the program words after the dashes, with one fixed usage line naming the two flags and echoing none of the caller's words.
state: provisional
basis: fct-ydz7awsz; fct-dgfm4vwr
because: The chunk adds a refusal of the pair in the caller's words, exit 2 with one fixed line, so the pair reaches the child only through the checked flags.
@row_add n=4 row=contract/config-json/default-model area=input-validation fact=F4 detector=D-security-input
boundary: config.json
item: default_model
promise: A string of 1 to 64 bytes, a lowercase letter first, then lowercase letters, digits, dots and dashes; any other value takes the model half of the compiled default pair DEFAULT_PAIR with one parse-rejected line; no value disables a control.
basis: fct-xb6c4igy
because: config.json gains the default_model key, a new external input held to the model-word shape with its bad-value fallback.
@row_add n=5 row=contract/config-json/default-effort area=input-validation fact=F4 detector=D-security-input
boundary: config.json
item: default_effort
promise: One of the five effort words of claude --help; any other value takes the effort half of the compiled default pair DEFAULT_PAIR with one parse-rejected line; no value disables a control.
basis: fct-xb6c4igy
because: config.json gains the default_effort key, a new external input held to the closed effort words with its bad-value fallback.
@row_add n=6 row=decision/revive-launch-pair-resolution area=input-validation fact=F5 detector=D-security-input
name: revive's launch pair re-parsed before it is handed
rule: viola revive takes each half of the launch pair from its override, else the recorded half while it still re-parses, else the default half; the replay takes the last launch line whole, a line with a non-string member reading no pair.
state: provisional
basis: fct-ydz7awsz
because: revive now reads a recorded pair from the snapshot or the replay and hands it to the child, so a recorded word is re-checked before it crosses.
@row_add n=7 row=decision/board-launch-pair-re-parsed area=input-validation fact=F6 detector=D-security-input
name: Board model and effort shown only when they re-parse
rule: The board shows a recorded model only when it passes the model-word check and a recorded effort only when it passes the code-word check.
state: provisional
basis: fct-ydz7awsz
because: The board item gains model and effort, read from the recorded pair and kept only when each re-parses.
@row_add n=8 row=decision/launch-pair-kept-to-snapshot-and-event-log area=data-protection fact=F7 detector=none
name: Launch pair kept to the snapshot and the event log
rule: An instance's launch pair is kept only in the snapshot's launch field and the events log's launch line, log-only with no span, process-log line or terminal byte, and leaves only through list --json and the MCP list item.
state: provisional
basis: fct-objkygma; fct-xb6c4igy
because: The chunk records the pair in a new snapshot field and a new log-only event kind and lets it cross into list --json and the MCP list item.
@row_add n=9 row=decision/launch-pair-authenticates-nothing area=authentication-authorization fact=F8 detector=D-security-auth
name: Recorded launch pair authenticates nothing
rule: The recorded model and effort are self-reported words and authenticate nothing.
state: provisional
basis: fct-ydz7awsz
because: The chunk records a model and effort per instance and reads them back at revive and on the board, so they must never be taken as an identity.
@row_add n=10 row=decision/effort-word-check-in-agent-crate area=input-validation fact=F9 detector=D-security-input
name: Effort-word check in the Claude adapter crate
rule: The effort-word check lives in the Claude adapter crate beside the claude flag literals.
why: An exception to shared validators living in the core crate.
state: provisional
basis: fct-ydz7awsz
because: The chunk places the effort-word check beside the claude flag literals, outside the core crate, and the shared-validators row's except would pass its bound to hold it.
