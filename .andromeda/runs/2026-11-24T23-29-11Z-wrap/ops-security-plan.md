@row_add n=1 row=contract/adopt-refusal/not-resumed area=error-handling fact=F1 detector=none
boundary: viola adopt refusal
item: not-resumed
promise: Exit 1 when the resumed child ended before its session started, written only after the child released the terminal: the unable:/hint: pair naming the instance only, or under --json exactly {"v":1,"error":"not-resumed","detail":null} on stdout; no output holds the session id.
basis: fct-x4brbvla, fct-blcs6o5x, fct-isor5lni
because: viola adopt gains a third own refusal, not-resumed, whose pair and document disclose the instance name only and never the session id, a refusal no row of the section yet states.
@row_add n=2 row=decision/adopt-resumed-from-own-snapshot area=input-validation fact=F2 detector=none
name: Adopt judges resumed from its own snapshot only
rule: viola adopt decides whether its child resumed only from its own instance snapshot: a real snapshot with no agent session id is not resumed, while a replayed, absent or unreadable snapshot fails open to resumed and exit 0, never a refusal.
basis: fct-x4brbvla, fct-isor5lni
because: adopt now reads its own instance snapshot to decide the not-resumed end, reading no child output, and fails open where the snapshot cannot say, a rule no row of the section holds.
