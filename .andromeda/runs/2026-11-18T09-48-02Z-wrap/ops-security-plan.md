@row_set n=1 row=decision/screen-fixture-signature-rows-only cell=except seen=2 fact=F1 detector=D-security-input
value: On a failed recording verify, a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored; an unclean set is refused whole.
basis: fct-xaokr7kk
because: The hidden rig-only --record-first-screens record now keeps whole scrubbed rows of each run that did not reach the box, so the signature-rows-only rule must name it as its exception.
@row_set n=2 row=decision/verify-signature-only cell=except seen=1 detector=D-security-input
value: On a failed recording verify, a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored; an unclean set is refused whole.
basis: fct-ic6u3x4d
because: Row text now also leaves the process as the rig-only first-screen record of whole scrubbed rows, so the signature-only rule is incomplete without that exception.
@row_set n=3 row=decision/verify-screen-read-against-literals cell=except seen=1 detector=D-security-input
value: On a failed recording verify, a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored; an unclean set is refused whole.
basis: fct-ic6u3x4d
because: Row text now also leaves the process as the rig-only first-screen record of whole scrubbed rows, so the rule that it leaves only as signature-only fixtures must carry that exception.
@row_add n=1 row=decision/verify-stamp-first-screen-closed-words area=input-validation fact=F3 detector=D-security-input
name: Verify stamp's first screen in closed words
rule: The verify stamp records each run's first screen only as closed words, whole-number settle walls and closed fact rows of literal ids and row families, never screen text.
guard: merge_stamp_writes_the_six_first_screens_their_settles_and_facts
basis: fct-vynq47jv
because: The stamp's measured part gains first_screen, first_settle_ms and first_facts held to closed words, whole numbers and literal ids, a new stamped surface no row yet bounds.
@row_add n=2 row=decision/live-rig-seed-from-bundle-logic area=secret-management fact=F2 detector=D-security-auth
name: Live rig's child config seeded from bundle logic only
rule: The live rig seeds its child's private config only with keys whose values come from the CLI bundle's own logic, none read from the user's config: used-config markers and the browser-tools prompt turned off.
basis: fct-ic6u3x4d
because: The rig now writes a seed into its child's private config, and the row of its isolation says nothing of where the seeded values may come from.
