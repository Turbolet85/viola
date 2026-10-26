# New groups — test-plan · e2e-test-strategy

## Verdicts
- fanout-test-plan--cascade-g1.md 15 · contradiction · lines 226–226 · «`viola link` and `viola unlink` are not built (the wrapper answers both methods `-32601`)» · the standing E3 entry says the explicit verbs are not built, are held, and no product path writes an `unlink` record, while the proposal has `link` / `unlink` landed with `already_linked` and a written `unlink` record
- fanout-test-plan--cascade-g1.md 16 · contradiction · lines 226–226 · «No case reads an `unlink` record a product path wrote, because none writes one.» · the standing E3 entry says no product path writes an `unlink` record, while the proposal has a case reading an `unlink` that stays in force across a wrapper restart
- fanout-test-plan--cascade-g1.md 19 · distinct · no standing group names the link verbs' widened-instance strict-modes refusal or `link_verbs_refuse_a_widened_instance_before_any_log`; the exit-21 strict-modes cause at line 268 states only the general Unix 0770 home case
- fanout-test-plan--cascade-g1.md 22 · distinct · line 298 (its own landing place) states the planned raw-channel negative only; no standing group names `channel_link_names_are_validated_by_the_wrapper`, the foreign-`driven` and self-pair frames, `data: null` or `binary(channel_links)` on the kill list
