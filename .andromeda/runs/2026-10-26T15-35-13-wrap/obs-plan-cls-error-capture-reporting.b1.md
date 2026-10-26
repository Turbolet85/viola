# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the panic hook, its one-line detail record and src/panic_frames.rs are not reached, and the report leaves panic-hook mechanics untouched
error-capture-reporting#2.1 · no · the catch sites, role_of and the revive stderr pair are unchanged (Untouched: catch sites), and link and unlink fall under the any-other-verb cli rule
error-capture-reporting#3.1 · yes · add-to · viola link and viola unlink run check_instance on the driven instance before any role file or log opens, so the window's list of such checks gains them (F5)
error-capture-reporting#4.1 · no · the vt100 feed catch in src/run/gate.rs and tests/chaos_feed_panic.rs are not reached, and the report leaves the catch sites and panic paths untouched
