P4 fork answers, received 2026-10-07T07:27Z through this session's AskUserQuestion (two questions, the answers and their notes verbatim).

Question 1: "Which backing puts the dev host's test homes on tmpfs? (path stays <workspace>/target/e2e-home in all three; ci.yml is untouched in all three)"
Answer: "Link + keeper"
Note: The founder's own live answer at 2026-10-07T07:25Z, given through the overseer's AskUserQuestion after this widening was shown to him in these words: the test harness will create and delete files outside the project's working directory through the link, which Viola's rules class as a boundary widening and which needs his word. He chose the link with the keeper. Relayed by the overseer; record it as his.

Question 2: "What stands as the red side of the contended reading? (the green side is always: real verify-driven tests on the new backing inside a contention window)"
Answer: "One natural window"
Note: overseer (a technical fork, the overseer's): one natural window, the 60 min cap as you set it; if it runs out, stop and ask me before any synthetic writer.
