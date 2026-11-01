# Classified — security · secret-management.b1

## Answers
secret-management#1.1 · yes · the production rule that environment variables are not a configuration channel binds the new socket-directory, peer euid and home checks, which read no env, VIOLA_* variable or config key; the R8 child-environment strip does not reach the chunk
secret-management#2.1 · yes · the launcher shape (`env -i HOME PATH`, no uid-0 launch exists) binds the peer euid negative proof, which must stay inside it (Needs: no harness command, gate or pre-push stage launches as uid 0); its per-test `--home` line is standing S1
