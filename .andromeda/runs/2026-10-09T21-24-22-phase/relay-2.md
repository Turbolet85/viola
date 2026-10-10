P4 fork round, 2026-10-09, two questions asked in one dialog, the recommended option first in each. The answers as they came back, verbatim:

1. Question: "By my sizing the three CARRYs fit one builder window, tightly: 2 new + 18 modified files, about 14 new test cases, 3 in-place splits, no live session, and 4.3 to 5.5 h of mutation machine time run one after another. Which cut do you take? A split is the founder's word."
   Options shown: Keep one chunk (Recommended) · Split by kind · Score leaves alone.
   Answer: "Keep one chunk (Recommended)"
   Notes: "During the long mutation runs the CLI may compact an idle session after about 55 minutes: write each run start, its output dir and the next step into the run dir before waiting, and re-read them from disk when a run returns."

   The chosen option's preview, as shown:
   ONE CHUNK
     files      2 new + 18 modified
     tests      ~14 new cases, 3 splits
     machine    viola-e2e score      up to 2.5 h (78 min measured once)
                witness viola-state  ~15 min (205 mutants)
                witness ledger.rs    ~7 min  (304 mutants)
                witness root files   ~56 min (210 mutants)
                one root re-run      +56 min if a kill misses
                gates                under 5 min
     total      4.3 to 5.5 h, sequential, builder mostly idle
     cycles     1 phase + 1 implement + 1 wrap

2. Question: "Four of the eighteen survivors cannot be killed by a test as the code stands, and the test plan gives an argued survivor no standing (it says never "equivalent"; an argued one stays missed and reads red at the next boundary). What is done with them?"
   Options shown: Restate all four (Recommended) · Restate the two redundant · Change no product line.
   Answer: "Restate all four (Recommended)"
   Notes: "Read at both product sites before answering: each restated form rests on a guarantee in the same function (the take of cap bytes one line above the events guard; a capture is never both a pre and a post). Each restatement names that guarantee in its comment, and the witness run over the file shows no new missed mutant."

   The chosen option's preview, as shown:
   ledger.rs:685:51  clear_start
     drop the `+ 1`: the search already skips the paste itself
   fake-agent :473:46  submit
     drop `framing.is_some() &&`: the equality implies it
     (then :473:57 is killed by a lower-bound hold test)
   ledger.rs:670:33  parallel_both_before_first_post
     ask "no PostToolUse before the second PreToolUse"
     with no ordering operator
   events.rs:209:24  next_line
     state the guard as an equality with the cap
     (len never exceeds it); the oversize case kills its mutant

   result: 14 killed by a test, 4 gone by restatement
