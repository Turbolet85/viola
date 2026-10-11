Phase P4 fork round, chunk 2026-10-10-session-links. Three questions asked in the dialog, each with a marked recommendation; the operator's answers as returned.

Question 1: "Should a send or an answer that the wrapper refuses still record the driver-to-driven link?"
Options: "Only issued or taken (Recommended)" · "Any call carrying from"
Answer: "Only issued or taken (Recommended)"
The chosen option's text: The link line is appended when the wrapper issues the send (just ahead of its send-issued line) or takes the answer; a refused call adds none.

Question 2: "Which keys should the link result carry so the human line can be built from it (architecture still reads the result as an empty object)?"
Options: "since + already_linked (Recommended)" · "The link item + created"
Answer: "since + already_linked (Recommended)"
The chosen option's text: Two flat keys, exactly the two facts the design's link line prints: the first-linked time and whether the pair already existed. unlink stays an empty object. Additive, no v bump.

Question 3: "What should an unlink of a pair that is not in the link set do?"
Options: "No line, same result (Recommended)" · "Append a line every time" · "Refuse it"
Answer: "No line, same result (Recommended)"
The chosen option's text: Nothing is appended; the reply is the empty object, exit 0, and the verb prints the worded unlink line.
