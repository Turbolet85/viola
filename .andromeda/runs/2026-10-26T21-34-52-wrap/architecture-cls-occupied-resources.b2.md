# Classified — architecture · occupied-resources.b2

## Answers
occupied-resources#24.1 · no · the agent-run session record is written by viola-harness under target/agent-run, outside every reached file, and the session's home checks act on the homes under target/e2e-home, not on this record
occupied-resources#25.1 · yes · add-to · tests/support/home.rs is a reached file and seed_conpty now creates the home with create_private_dir, F17: a home made by a plain create before viola is refused Writable on an inherited Users write ACE
occupied-resources#26.1 · no · the profraw census script is outside the reached files, and the session's start changes (the home-tree check, the replayed snapshot read) leave the profile tallies it counts untouched
occupied-resources#27.1 · yes · add-to · F16: the protected DACL a home viola makes outside the profile reads LA and SY on windows-2025, LA being the runner's local Administrator, which is the user + SYSTEM DACL this scratch takes
