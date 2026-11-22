@row_set n=1 row=contract/cli-exit-codes/13 cell=promise seen=1 fact=F1 detector=D-design-cli-wording
value: `not-delivered`; among its details `paste-cut`, when the claude child does not take the whole text within the compiled paste bound of 5 s, the bracket closed with no Enter and the wheel unmoved.
basis: fct-shrbvdzl (layout-templates `contract/cli-exit-codes/13` + `paste-cut`), fct-z6owtgsx (`NotDelivered::PasteCut`, `PASTE_BOUND` 5 s)
because: The closed not-delivered detail set gained `paste-cut` at exit 13, and the exit-13 row must name the new detail a send can now end with.
@row_add n=1 row=contract/viola-send-output/after-a-paste-cut area=surface-cli fact=F2 detector=none
boundary: viola send output
item: after a paste-cut
promise: After a `paste-cut`, every following send is refused `unable  <name>  not-delivered  paste-cut`, exit 13, with no cursor and nothing issued, until a human key, a prompt-submitted, a session-start or a session-end; a release with no key and a turn-ended leave the refusal in place.
basis: fct-z6owtgsx (`Flight::cut_at`, its set and clears; the mark's rung refusing `paste-cut` with no cursor), fct-p2qhyrcq (a release with no key and a turn-ended leave the mark)
because: The chunk adds a standing refusal state to viola send's output that no surface-cli row states.
@row_add n=2 row=contract/viola-send-output/paste-cut-hint area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola send output
item: paste-cut hint
promise: An unable not-delivered paste-cut outcome, on its own struck mirror line, ends stderr with `hint: <name> did not take the whole text in time; a human must look at the session before the next send`, a provisional wording; the answer hints are unchanged.
basis: fct-z6owtgsx (`send_hint("paste-cut")`, PROVISIONAL wording), fct-rgis5vav (the `paste-cut` hint and struck line pinned by literal)
because: The chunk adds a new send hint wording, and the masters must state it as the input-not-ready hint is stated.
