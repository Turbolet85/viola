# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the lock delta moved tokio alone and no other crate moved, so the libc backtrace and dladdr calls it names are unchanged, and src/panic_frames.rs is untouched
error-capture-reporting#2.1 · no · the edited Cargo.toml changed only its tokio pin at line 242 and its profile panic = "unwind" lines did not move, and the catch sites are source the session did not touch
error-capture-reporting#3.1 · no · the init window's order (clap, home resolution, the instance checks before any role file opens) is source the session did not touch and no Cargo change moves it
error-capture-reporting#4.1 · no · the vt100 panics are caught by the catch_unwind in src/run/gate.rs, which the session did not touch, and the lock delta moved tokio alone so vt100 is unchanged
