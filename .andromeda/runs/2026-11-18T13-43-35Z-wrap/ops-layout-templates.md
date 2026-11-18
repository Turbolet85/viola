@row_set n=1 row=contract/cli-verbs/run cell=promise seen=1 fact=F1 detector=none
value: Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs.
basis: fct-xro6zomc
because: viola run's start now writes the CLI's trust record for an untrusted session folder on a verified CLI, so the verb's promise must name what the start does beyond passthrough.
@row_set n=2 row=contract/viola-run/start-output cell=promise seen=1 fact=F2 detector=none
value: `viola run` prints nothing of its own, and the start's trust step adds no refusal, exit code, hint, `--json` code or terminal byte; the terminal belongs to the wrapped claude TUI until it exits.
basis: fct-xro6zomc
because: The new start trust step never refuses or alters the start and writes no viola byte, so the passed-start output contract must state that the step leaves it unchanged.
@row_set n=3 row=contract/cli-verbs/revive cell=promise seen=1 fact=F3 detector=none
value: Resumes a dead instance in place (its newest logged session, in the recorded directory, which also keys the trust record its start writes) and is passthrough like viola run.
basis: fct-xro6zomc
because: viola revive now writes the trust record keyed by the session's recorded working directory, the same directory it spawns the child in, so the verb's promise must say so.
