@row_set n=1 row=decision/payload-types-derive-redact cell=except seen=1 fact=F1 detector=D-obs-pii
value: A payload type with no content field derives neither veil Redact nor Debug, because veil 0.3.0 refuses a Redact derive with no redact field.
basis: fct-ogmw6p5z
because: veil 0.3.0 refuses the derive on the three content-free MCP input types, so the convention that every payload type derives Redact holds only with this exception, as the operator's wrap directive rules the row.
@row_set n=2 row=decision/veil-redact-payload-types cell=except seen=1 detector=D-obs-pii
value: A payload type with no content field derives neither veil Redact nor Debug, because veil 0.3.0 refuses a Redact derive with no redact field.
basis: fct-irqnu3k2
because: The row says every payload type derives Redact, which the measured veil 0.3.0 refusal on content-free types makes false without its exception.
@row_add n=1 row=decision/strict-modes-refuses-symlink area=otel-sdk-init fact=F3 detector=none
name: Strict-modes check refuses a symbolic link
rule: The strict-modes check refuses a symbolic link at the home, instances, an instance directory, the snapshot or the stamps file before its mode verdict, and leaves the link and its target as found.
guard: strict_instance_refuses_a_symlink_and_leaves_it_as_found
basis: fct-yeqzuggx
because: The check of trusted state now reads the path itself and refuses a link before the mode verdict, a rule no row of the area states.
@row_none n=1 fact=F2 type=4
relied: A link at a role or detail file path is a failed open, which decision/private-diagnostics-files already states as opening without following symlinks; setting 0600 through the handle is how the open is built.
