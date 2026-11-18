@row_add n=1 row=decision/verify-hidden-first-screen-record area=surface-cli fact=F1 detector=none
name: Rig-only hidden first-screen record of verify
rule: viola verify carries one hidden option, never listed in --help and passed only by the live rig, that on a failed record verify writes each run's first screen not reaching the input box as whole scrubbed rows to a local file; a plain record verify and a passing verify write no such file.
guard: verify_help_names_no_first_screens_option
basis: fct-xaokr7kk
because: The chunk adds the hidden `--record-first-screens` option of `viola verify`, absent from --help and passed only by the rig's V arm, and no row of the cli surface states a hidden verify option or its failed-verify-only local record.
