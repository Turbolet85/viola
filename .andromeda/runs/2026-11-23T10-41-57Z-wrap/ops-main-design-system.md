@row_set n=1 row=contract/cli-exit-codes/2 cell=promise seen=1 detector=none
value: Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`), `UNUSABLE_TEXT` line (unusable `viola send`) or `PAIR_AFTER_DASHES` line (a pair after `--` of `run` or `revive`) on stderr, whatever `--json` says; no document.
basis: fct-ydz7awsz
because: run and revive now refuse a pair after the dashes with the one fixed line PAIR_AFTER_DASHES at exit 2, so the row's named fixed lines are short of it.
