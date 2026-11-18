@row_set n=1 row=decision/live-rig-seed-from-bundle-logic cell=rule seen=1 detector=none
value: The live rig seeds its child's private config with no value read from the user's config: its used-config markers and the browser-tools prompt turned off come from the CLI bundle's own logic.
basis: fct-xro6zomc
because: The standing rule that the rig seeds only bundle-logic keys is false now that its start arm also seeds a synthetic sentinel entry, so the rule keeps only what still holds: nothing seeded is read from the user's config.
