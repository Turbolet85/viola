# Classified — security · secret-management.b1

## Answers
secret-management#1.1 · yes · the socket-root resolution reads `XDG_RUNTIME_DIR` and `TMPDIR`, and 1.1 is the closed list of environment reads, so no `VIOLA_*` variable or new environment read may reach the directory, peer or home checks
secret-management#2.1 · no · standing S1 · each new test uses its own `--home`, and the pre-push gate's children carry only `HOME` and `PATH`
