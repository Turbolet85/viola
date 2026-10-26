# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · add-to · `cli.link` and `cli.unlink` join the per-verb CLIENT spans as the new verbs' dispatch spans (F1)
span-trace-coverage#1.2 · no · link and unlink reach the wrapper through the existing channel.dispatch and open no span of their own (F3)
span-trace-coverage#1.3 · no · link and unlink open no span of their own (F3), so no run.* or state.* span is added
span-trace-coverage#2.1 · yes · add-to · `cli.link` and `cli.unlink` join the per-verb dispatch spans (F1), and the verbs' instance refusal, which opens no log and writes no line, joins the verbs listed as refusing before any log
span-trace-coverage#2.2 · no · the viola run span set is unchanged, link and unlink opening no span of their own (F3)
span-trace-coverage#2.3 · no · no hook path changed and link and unlink are not hook events
span-trace-coverage#2.4 · no · the channel pair keeps its fields and never-params rule, and link and unlink log through it with no driver or driven name (F3)
span-trace-coverage#2.5 · no · link and unlink are not MCP tools, so the MCP surface and its spans are unchanged
span-trace-coverage#3.1 · yes · stale · the frame-verb count moves from six to eight, link and unlink joining the server check inside their cli.link and cli.unlink spans (F1)
span-trace-coverage#3.2 · yes · add-to · link and unlink join the one-per-CLI-frame-verb captured-span cases, each refusal logged inside its cli.link or cli.unlink span (F1)
span-trace-coverage#3.3 · no · no hook path changed, so the hook's server check and its refusal place are unchanged
span-trace-coverage#3.4 · no · the MCP frame tools' server check is unchanged, link and unlink not being MCP tools
span-trace-coverage#4.1 · no · the run start sequence and its link-set restore keep their spans and lines, and the change adds no start step
span-trace-coverage#4.2 · no · the run.start span attributes are unchanged, as the change adds no start span or attribute
span-trace-coverage#4.3 · no · the run start's required log fields and product order are unchanged, as the change adds no start line
span-trace-coverage#4.4 · no · the run start's cleanup and handle-wait exit lines are unchanged
span-trace-coverage#5.1 · no · the send path and its implicit link record keep their no-span, no-own-line rule, and the explicit verbs are not on the send path (F3)
span-trace-coverage#5.2 · no · the send's required span attributes are unchanged, the explicit verbs opening no span (F3)
span-trace-coverage#5.3 · no · the send's required log fields are unchanged, the explicit verbs writing only their channel pair (F3)
span-trace-coverage#5.4 · no · send.client's exit classes and cleanup are unchanged
span-trace-coverage#6.1 · no · the wait and last spans are unchanged, link and unlink being neither wait nor last
span-trace-coverage#6.2 · no · the wait.block and run.wait_dispatch attributes are unchanged
span-trace-coverage#6.3 · no · the wait and last log fields hold, and their strict-modes claim is limited to wait and last, which run no instance check
span-trace-coverage#6.4 · no · the wait.block cleanup is unchanged
span-trace-coverage#7.1 · no · the dialog and answer spans and the answer's implicit link record are unchanged (F3)
