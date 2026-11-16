@row_set n=1 row=decision/api-error-classes-adapter-only cell=rule seen=1 detector=none
value: The CLI's StopFailure error word is classified into one of the nine closed turn-end cause words inside the Claude adapter crate only; only that cause word crosses out of the adapter, never the error word or any error text.
basis: fct-vldmp3ur
because: The classification now crosses as one of the nine TurnEndCause words while the classifier stays in the adapter, so the rule that no class is written out is false.
