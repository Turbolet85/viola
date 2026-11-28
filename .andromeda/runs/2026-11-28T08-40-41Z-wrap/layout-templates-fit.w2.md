@fit n=1 row=decision/console-own-surface sha=df46d49ed808 v=fits type=1
@fit n=2 row=decision/console-frame-floor sha=be44d2360623 v=fits type=1
@fit n=3 row=decision/console-redraw-on-resize sha=1e0739715a5a v=not_master type=4
why: How the console's draw loop re-reads the size and redraws is one mechanism, and the basis gives no reason for it, so the code holds it.
