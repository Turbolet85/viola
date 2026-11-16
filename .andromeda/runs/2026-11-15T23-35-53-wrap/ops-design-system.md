@row_add n=1 row=decision/api-error-turn-end-shown-as-turn-end area=surface-cli fact=F1 detector=none
name: API-error turn end shown as any turn end
rule: A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is the closed cause word `api-error` in `--json` event data, and the CLI's error text is shown nowhere.
guard: list_status_reads_idle_after_an_api_error_turn_end
state: provisional
basis: fct-xbqeduib, fct-vkofdjrk, fct-gyihyjgp, fct-klhffiim
because: The chunk ends an API-error turn as a plain `turn-ended` with a closed `cause` word, never the error text, and adds no new output wording, so the surface's choice to mark it no other way must stand as a provisional decision (F2).
