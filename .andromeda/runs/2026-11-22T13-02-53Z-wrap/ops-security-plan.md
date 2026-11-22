@row_set n=1 row=decision/bridge-send-one-bracketed-paste cell=except seen=1 detector=none
value: A paste not finished within its compiled bound is cut: the bracket is closed with no CR and the send is refused not-delivered paste-cut.
basis: fct-z6owtgsx, fct-shrbvdzl
because: The paste write is now bounded and a cut paste closes the bracket without the CR, so the one-paste-plus-CR rule no longer holds for a cut send.
@row_set n=2 row=decision/send-text-is-one-bracketed-paste cell=except seen=1 detector=none
value: A paste not finished within its compiled bound is cut: the bracket is closed with no Enter and the send is refused not-delivered paste-cut.
basis: fct-z6owtgsx, fct-shrbvdzl
because: A cut paste now ends the bracket with no Enter, an exception the row's paste-then-Enter rule must name.
@row_add n=1 row=decision/paste-write-bounded area=threat-model-summary fact=F1 detector=none
name: Paste write into the PTY bounded
rule: The wrapper's paste write into the child's PTY ends within a compiled 5 s bound that no config key, environment variable or flag sets; a paste not finished, or still queued, at its deadline is cut, the bracket closed with no Enter, and the send refused not-delivered paste-cut.
basis: fct-z6owtgsx, fct-wxrpelpd, fct-nn6stoyu
because: The chunk bounds the paste write with the compiled PASTE_BOUND and cuts a paste that cannot finish, a new decision no standing row states.
@row_add n=2 row=decision/paste-cut-mark area=threat-model-summary fact=F2 detector=none
name: Cut mark refuses driver sends until a human acts
rule: After a cut paste the wrapper holds a cut mark in memory only, and every next driver send is refused not-delivered paste-cut with nothing typed until a human key moves the wheel's key count or a prompt-submitted, session-start or session-end clears it.
except: A release with no key or a turn-ended leaves the mark standing.
basis: fct-z6owtgsx, fct-p2qhyrcq
because: The chunk adds the in-memory cut mark and its clears, a new refusal rule on driver sends that no standing row states.
@row_add n=3 row=contract/channel-refusal/paste-cut area=error-handling fact=F5 detector=none
boundary: channel result.refusal
item: not-delivered/paste-cut
promise: A send whose paste was cut at its compiled bound, or refused while the cut mark stands, is answered with result.refusal not-delivered/paste-cut; the CLI exits 13 with it.
basis: fct-z6owtgsx, fct-vl24ioge, fct-shrbvdzl
because: The closed not-delivered detail set gains paste-cut, appended last, and the channel refusal list holds one row an item.
@row_none n=1 fact=F3 type=4
relied: the human's keys are never dropped and wait behind at most one in-flight 4096-byte chunk and the closing marker
@row_none n=2 fact=F4 type=4
relied: a PTY writer panic ends the run with the child killed
@row_none n=3 fact=F6 type=4
