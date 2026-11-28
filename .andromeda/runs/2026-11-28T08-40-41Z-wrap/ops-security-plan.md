@row_add n=1 row=decision/console-key-reader-bounded area=input-validation fact=F1 detector=D-security-input
name: Console key reader bounded and closed
rule: viola console reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each read decoded to one closed class: 0x03 anywhere is interrupt, exactly q is quit, anything else is unbound; no key byte reaches any log line or output.
guard: console_logs_an_unbound_key_by_class_alone
state: provisional
basis: fct-xexxpxch
because: The chunk adds the console's key reader as a new external-input surface, and §Input Validation holds no row for its bound and closed decoding.
@row_add n=2 row=decision/console-terminal-and-home-check area=authentication-authorization fact=F2 detector=none
name: viola console checks its terminal and home first
rule: viola console refuses unless stdin and stdout are both terminals, then holds the home to the strict-modes read rule, an absent home passing, before any log opens; either refusal writes no line, leaves the modes as found and exits 1.
basis: fct-xexxpxch
because: The new console verb gains its own start checks, and no standing row names the console among the checkers of the home's read rule.
@row_add n=3 row=contract/strict-modes-refusal/console area=authentication-authorization fact=F3 detector=none
boundary: strict-modes refusal
item: viola console
promise: Exit 1 with the fixed pair unable: viola console could not use its home, then hint: make the viola home readable by you alone, then run viola console again, on stderr, with no line written and the mode left as found.
basis: fct-xexxpxch
because: The console's strict-modes refusal is a new fixed stderr pair beside the other verbs' strict-modes refusal items.
@row_add n=4 row=decision/console-restores-terminal-before-error area=error-handling fact=F4 detector=none
name: Console restores the terminal before any error
rule: viola console's screen guard restores the terminal, cursor shown and alternate screen left, before any error is printed; a dispatch error or a caught panic then writes error: internal error once and exits 1 with a process-exit internal-error line.
state: provisional
basis: fct-xexxpxch; fct-elardgcb
because: The console adds a raw-mode screen whose error path must leave the human's terminal usable before printing, which no standing error-handling row says.
@row_add n=5 row=decision/console-log-content-free area=data-protection fact=F5 detector=none
name: Console diagnostics hold no key byte or path
rule: viola console writes its own diagnostics file console.ndjson, whose lines carry only process-start without an endpoint kind, console-frame with columns, rows and fits, console-key with a closed class word, and process-exit, and hold no key byte and no path.
guard: console_logs_an_unbound_key_by_class_alone
state: provisional
basis: fct-xexxpxch; fct-u62b25hk; fct-krgdptlj
because: The chunk adds a new diagnostics role file whose content is held to closed fields, and §Data Protection holds no row for it.
@row_add n=6 row=decision/console-local-surface area=threat-model-summary fact=F6 detector=none
name: viola console is a local terminal surface
rule: viola console is a local surface on the human's own terminal: it reads keys only from its own stdin, binds no endpoint, connects to no instance and sends no channel frame.
state: provisional
basis: fct-xexxpxch; fct-krgdptlj
because: The new console verb adds an interactive surface to the threat model, and the summary must say it opens no endpoint or channel.
