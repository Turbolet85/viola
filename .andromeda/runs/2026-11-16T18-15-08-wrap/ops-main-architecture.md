@row_set n=1 row=contract/hook-to-kind-map/stopfailure cell=promise seen=1 detector=none
value: StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null and `cause` one of the nine TurnEndCause words, classified from the CLI's error word inside the Claude adapter.
basis: fct-xw6ru4by
because: The StopFailure arm now writes the classified cause word, one of nine, where the row promises the one word `api-error`.
