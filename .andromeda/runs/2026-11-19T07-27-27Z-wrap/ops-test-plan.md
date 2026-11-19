@row_set n=1 row=decision/tests-never-read-host-sessions cell=except seen=1 fact=F3 detector=none
value: The live rig's local-only vh arm runs verify under an explicit real home and Claude config, reading only names and one needle as a closed word, no credentials, no value kept; else refused as usage.
basis: fct-xiw3vi2f
because: The rig gains the vh arm that reads the host's real Claude config by names only and is refused as usage without all three home flags, an exception the rule must carry.
@row_add n=1 row=contract/verify-output/browser-tools-line area=5-integration-test-strategy fact=F1 detector=none
boundary: viola verify output
item: browser-tools line
promise: When the browser-tools prompt stood at start, verify prints the fixed line "the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again" before the summary, which stays last; absent, stdout is unchanged.
basis: fct-sfqjrkq4
because: Verify's stdout gains a fixed line on a met browser-tools prompt, a promise at the CLI output boundary no row states yet.
@row_none n=1 fact=F2 type=4
relied: the live rig's cx arm reads context-post as one closed word of both, prompt-submitted, tag or none.
