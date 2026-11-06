@verdict n=1 ref_file=fanout-architecture--cascade-g1.md ref_n=4 v=contradiction fact=fct-q5vugilx
quote: on Windows the client opens the pipe itself with windows-sys 0.61.2
why: the standing Wrapper IPC row (like the PTY layer and State-file primitives rows) still has windows-sys in use and libc Unix-only, where the proposal says no crate depends on windows-sys and libc is a plain, not target-gated dependency
