@row_set n=1 row=contract/event-data/turn-ended cell=promise seen=2 fact=F1 detector=none
value: data holds the keys last_assistant_message, background_tasks and cause; cause is present on an API-error turn end only, absent on a Stop, and is one of nine closed words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
basis: fct-xw6ru4by, fct-ulztjw6q, fct-pqfgw3kz
because: The turn-end cause set grew from the one word api-error to nine closed words, and the report's expected amendment names this row to carry the nine-word set.
@row_none n=1 fact=F2 type=9
