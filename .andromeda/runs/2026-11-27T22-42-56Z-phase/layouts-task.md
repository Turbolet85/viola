@task_prose part=chunk
Three pure moves with no behaviour change, ranked sixth by the Epoch 5 code audit.
One: the fleet re-open loop of viola up is split into its plan (refusal, order, skip) and its act (spawn, wait, line), so no function stays over cognitive 15.
Two: the switch's respawn (restart code) moves out of the PTY pump into its own module of the pty crate, and the pump's inline test module moves out too, so no file is over 800 code lines.
Three: the window step and the trust step of the shared start of run, revive and adopt (and through them open and up) move beside the window module and the trust record, its test module leaving too.
It re-measures each moved and each remaining file by the audit's own measure (tokei code lines, rust-code-analysis cognitive sum), re-counts every moved test module's cases, and re-runs the whole test layers green.
@task_file n=1 path=src/cmd/up.rs mark=edits
what: the viola up verb, the fleet re-open loop over named instances (refusal, order, skip, spawn, wait, result line) — edits: the function upped, split into its plan and its act
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump of the wrapper, its main loop and the switch's respawn of claude for viola switch — edits: the restart code (the respawn arm of Pumping::run) and the inline test module, both moved out
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the shared start of viola run, viola revive and viola adopt (and through them viola open and viola up), with its window placement step and its trust step — edits: run::start's window step call (open_window), its trust step, and the inline test module, moved out
@task_item n=1 part=needs
The restructure's invariant: upped (the viola up fleet re-open loop: refusal, order, skip, spawn, wait, result line), the switch's respawn of the pump (viola switch's restart of claude), and the window step and trust step of the shared start (viola run, revive, adopt, open, up) each do exactly what they did; the existing test layers, green with no test body edited and with equal case counts before and after, hold it.
@task_item n=2 part=needs
The measure the chunk is read by: tokei code lines per Rust file, an inline test module counted in its file, at most 800 for every moved file, every file left behind and every file created; rust-code-analysis cognitive sum at most 15 for every function of the viola up file and of any file its parts land in.
@task_item n=3 part=needs
The split of upped is the only place the chunk may add a function; no other new function, and no refactor the audit did not name (not the start prologue, not the refusal helper, not verify, not the window code's logic, not the harness scripts).
@task_item n=4 part=needs
Where moved units may live: the respawn in its own module of the pty crate; the window and trust steps beside the window module and the trust record; each move exposes only what its existing callers already reach.
@task_item n=5 part=needs
How tests reach a moved unit: only through a re-pointed path, module or name, each re-point named in the plan; no test body edited.
@task_item n=6 part=needs
A moved test module's cases are counted before and after and the counts are equal; a large inline test module leaves its file into at least two files of at most 800 code lines each.
@task_item n=7 part=needs
No test is deleted, merged or weakened to fit a size bound; every moved function keeps its tests with it, since the move changes the sites the next audit's diff mutation run reads (root crate baseline 326 caught, 0 missed).
@task_item n=8 part=needs
Paths, module names and texts a move changes are re-pointed wherever a test, a contract census, a lint list or a mod tree reads them; a plan row or leaf naming a moved path is an expected wrap amendment, not a change of the rule.
@task_item n=9 part=needs
Every refusal, hint, exit code and output line of viola up, viola switch, and the run, revive and adopt starts (start output, preflight order, window placement, trust refusal) stays byte for byte as it is; the moved code emits them from its new home unchanged.
@task_item n=10 part=needs
The boot path of the shared start changes place, so the harness smoke trio is held as operator entries, not run by the chunk.
@task_item n=11 part=needs
No live start, no window opened, no compositor dispatch, no real claude, no live arm of any harness script, no mutation run, nothing read of the operator's real claude or viola home.
@task_item n=12 part=needs
A plan step that asserts what existing code does cites the symbol and the line it read.
@task_item n=13 part=needs
The next audit's complexity over-ceiling list names no function of the viola up file, and its over-800 sizes list names neither the pump file, the shared start file, nor any file the moves create.
@task_item n=1 part=not_needed
The web surface: viola renders no page, and these moves touch no rendering.
@task_item n=2 part=not_needed
The viola list board and its BAY line, column widths, styling, tree and links block: no moved unit renders the board.
@task_item n=3 part=not_needed
viola send output, the readback mirror and readback vocabulary, and the send refusals before any request: no moved unit sends a prompt or reads it back.
@task_item n=4 part=not_needed
The viola mcp tools and their tool results, and the frame verbs' exit-21 hints and details: the moves reach no MCP server and no frame verb.
@task_item n=5 part=not_needed
viola verify output and its refusals: the scope bars verify from the chunk.
@task_item n=6 part=not_needed
viola link and unlink output, viola wait output, viola last output, viola answer output, the wheel verbs pause and release, and viola plugin install: none of their code moves.
@task_item n=7 part=not_needed
CLI token rendering (colour, spacing, radius, motion, roles) and the help and per-verb usage output: the moves change no styling and no clap verb definition.
