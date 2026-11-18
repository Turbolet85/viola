@row_set n=1 row=decision/unverified-cli-gate cell=rule seen=3 fact=F1 detector=none
value: No non-null answer decision is sent without a viola verify stamp for the child's CLI version on which every relied ledger row counts: it reads pass, or absent on the browser-tools-prompt row alone; absent on any other row leaves the version unverified.
basis: fct-xmezyz7c, fct-tlsodzwd
because: The verified rule moved from every relied row passing to every relied row counting, absent counting on the browser-tools-prompt row alone, so the gate's rule must name what a covering stamp now is.
@row_set n=2 row=decision/dialog-answers-need-verify-stamp cell=rule seen=1 detector=none
value: A hook.dialog is answered with a non-null decision, PermissionRequest allow included, only when ledger/stamps.json holds a viola verify stamp for the child's CLI version on which every relied ledger row counts: pass, or absent on the browser-tools-prompt row alone.
basis: fct-xmezyz7c, fct-tlsodzwd
because: The row says every relied ledger row passes, which the new verified rule contradicts: absent on the browser-tools-prompt row now counts toward verified.
@row_set n=3 row=contract/cli-exit-codes/12 cell=promise seen=1 detector=none
value: answer refused unverified-cli: no verify stamp for the child's CLI version on which every relied ledger row counts (pass, or absent on the browser-tools-prompt row alone).
basis: fct-xmezyz7c, fct-tlsodzwd
because: The exit code's trigger is the verified rule, which now counts absent on the browser-tools-prompt row, so covering every relied row no longer names it exactly.
@row_set n=4 row=decision/live-rig-seed-from-bundle-logic cell=rule seen=2 detector=none
value: The live rig seeds its child's private config with no value read from the user's config: its used-config markers and, on every arm but VP, the browser-tools prompt turned off come from the CLI bundle's own logic; VP leaves the browser-tools key out.
basis: fct-xmezyz7c
because: The new VP arm writes the four kept markers with the browser-tools key absent, so the row's claim that the prompt is always seeded off is no longer whole.
@row_add n=1 row=decision/browser-tools-prompt-never-answered area=threat-model-summary fact=F2 detector=none
name: Browser-tools prompt never answered or seeded
rule: viola never answers or seeds the claude CLI's browser-tools prompt; verify and viola run's pre-send gate read it as a modal and type nothing, so a host with the prompt unanswered stays unverified until a person answers it once.
basis: fct-qlgfl5fh, fct-zatpgkji
because: The chunk made the browser-tools prompt a compiled modal literal that verify and the gate refuse to type into, a standing choice with its price that no row of the area states.
@row_add n=2 row=decision/live-rig-cache-key-name-read area=secret-management fact=F3 detector=D-security-auth
name: Live rig reads the extension cache key name only
rule: The live rig may read whether the key name cachedChromeExtensionInstalled stands in its own private scratch config, printing written or absent, and never reads a value.
why: A boundary widening, answered for the rig alone.
basis: fct-h74wljzv, fct-xmezyz7c
because: The chunk added the rig's cache-a reader over its scratch config, a widening of what the rig reads that no secret-management row states.
