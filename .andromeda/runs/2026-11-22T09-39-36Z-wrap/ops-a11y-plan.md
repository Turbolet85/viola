@row_set n=1 row=decision/no-screen-content-parsing cell=except seen=7 detector=none
value: the test-only rig reads closed words, never text: trust selector's place, input box empty, holds-text or absent, modals, box-row cell attributes before any send or after a turn end, send or resize
basis: fct-qulhmgcs, fct-ynvxadkv
because: The new KR arm reads its box row's cell attributes after a resize of its own pty, never text, so the rig's named exception must name the resize reading beside the turn-end and send readings.
@row_add n=1 row=decision/no-wait-after-resize area=keyboard-test-harness fact=F1 detector=none
name: No viola-side wait after a resize
rule: A key typed right after a resize of claude's own pty is forwarded with no viola-side wait or fix, because real claude on Linux keeps it.
why: keys typed 0, 20 or 100 ms after a resize were kept on Linux on two claude versions; read as no loss at these delays, never as never loses
basis: fct-rdondmwf, fct-jguuj2z7, fct-yr7v5m4h
because: The chunk measured a key after a resize kept and ruled that no viola-side wait or fix follows, a decision on keyboard delivery future work must respect.
