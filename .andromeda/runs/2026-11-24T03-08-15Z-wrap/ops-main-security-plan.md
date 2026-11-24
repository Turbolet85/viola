@row_set n=1 row=decision/cli-frame-verbs-strict-modes-first cell=rule seen=1 detector=none
value: CLI send, wait, last, answer, pause, release, link, unlink and focus run the instance strict-modes check before any frame or compositor read; a refusal exits 21, opens nothing under the home and writes no line.
basis: fct-yz4mggmo
because: viola focus runs the same instance check through client::refuse_untrusted (src/cmd/focus.rs, the report's Symbols / APIs) and refuses exit 21 strict-modes-failed before any log, so the row naming the verbs that check the instance first must name it.
