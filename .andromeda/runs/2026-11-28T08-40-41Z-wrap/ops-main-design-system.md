@row_add n=1 row=contract/fleet-console-keys/q area=surface-console new_area=Surface:_console detector=none
boundary: Fleet console keys
item: q
promise: `q` quits the fleet console with exit 0.
basis: fct-xexxpxch
because: The console's key set is a new closed contract with the human, under the new Surface: console area the plan names.
@row_add n=2 row=contract/fleet-console-keys/ctrl-c area=surface-console new_area=Surface:_console detector=none
boundary: Fleet console keys
item: Ctrl-C
promise: Ctrl-C ends the fleet console with exit 0.
basis: fct-xexxpxch
because: The console's key set is a new closed contract with the human, under the new Surface: console area the plan names.
@row_add n=3 row=contract/fleet-console-keys/unbound area=surface-console new_area=Surface:_console detector=none
boundary: Fleet console keys
item: any other key
promise: Every key other than `q` and Ctrl-C is unbound and ignored; the end of the console's input also ends it with exit 0.
basis: fct-xexxpxch
because: The console's key set is a new closed contract with the human, under the new Surface: console area the plan names.
@row_add n=4 row=decision/fleet-console-plain-ascii area=surface-console new_area=Surface:_console detector=D-design-tokens
name: Fleet console in plain ASCII without colour
rule: The fleet console uses no colour and no SGR: its text is plain ASCII only.
state: provisional
basis: fct-pns455gq
because: The console's coverage reads tokens n/a with no colour, a choice its later views must keep, under the new Surface: console area the plan names.
