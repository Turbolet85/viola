@row_add n=1 row=decision/fired-wake-keeps-driver-wheel area=surface-cli fact=F1 detector=none
name: A fired scheduled wake keeps the driver's wheel
rule: A keyless prompt fired by a CLI-scheduled wake on a verified CLI, with no key typed since the last human-filed prompt and at least one wake counted at the last turn end, reads origin `harness` and leaves the wheel with the driver; the typed mark clears only at the next human-filed prompt.
why: so the driver keeps its sends and dialogs across a wake the CLI itself scheduled
except: A prompt submitted by the mouse alone is the one named false positive, unmeasured.
basis: fct-yqdgc3b5, fct-wncy4gmx, fct-zctmkx5l
because: The wrapper now tells a fired wake's prompt from a typed one and keeps the wheel with the driver, a wheel rule the CLI surface shows in WHEEL and in the human-typing refusal that no design-system row states yet.
@row_none n=1 fact=F2 type=5
what: The relied ledger count (26, the step counter's [26/26] and the summary's 26 pass 0 fail) is a count the code holds; the step-line and summary rows state it as MM, the relied count, and stay true.
