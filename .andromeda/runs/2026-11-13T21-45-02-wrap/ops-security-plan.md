@row_set n=1 row=decision/logs-owner-only cell=rule seen=1 fact=F1 detector=none
value: The event log, heartbeat and diagnostics files are 0600 inside 0700 directories, and their owner-only append open never follows a symbolic link: a link at the path is a failed open, and the 0600 mode is set through the open handle.
basis: fct-yeqzuggx (F6 append: the private append open takes O_NOFOLLOW and sets 0600 through the handle; a link at the path is a failed open)
because: The owner-only append open of the event, heartbeat and diagnostics files now refuses a symbolic link and narrows through the handle, which this row's owner-only promise for those files states less.
@row_add n=1 row=decision/content-payload-types-redact-debug area=data-protection fact=F2 detector=none
name: Content-bearing payload types redact Debug
rule: Every hook, dialog and MCP input payload type that carries content derives veil Redact, so Debug never shows that content; a payload type with no content field derives none.
why: veil refuses the Redact derive on a type with no field to redact.
guard: normalised_debug_holds_no_prompt_text; input_debug_holds_no_answer_or_text
basis: fct-yeqzuggx (F9 Redact), fct-irqnu3k2, fct-kvnor7kc
because: The hook, dialog and MCP input payload types that carry content now derive veil Redact while the content-free MCP inputs derive none, and no row of Data Protection says it.
