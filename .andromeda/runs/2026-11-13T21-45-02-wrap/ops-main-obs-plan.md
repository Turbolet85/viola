@row_set n=1 row=decision/payload-types-derive-redact cell=rule seen=1 detector=D-obs-pii
value: Every channel, hook and MCP payload type that carries content derives veil Redact, so Debug never reveals content; a payload type with no content field derives none.
basis: fct-ogmw6p5z
because: veil 0.3.0 refuses the Redact derive on a type with no field to redact, so the three content-free MCP inputs derive none, and the operator's wrap directive brings the rule to that fact.
