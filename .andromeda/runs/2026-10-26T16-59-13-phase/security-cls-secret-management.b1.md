# Classified — security · secret-management.b1

## Answers
secret-management#1.1 · yes · environment variables are no configuration channel, so no VIOLA_* var may switch off strict-modes, the DACL check or the server verification (Needs #53), which the edited `tests/cli_controls_not_disableable.rs` table proves
secret-management#2.1 · no · standing S1 · each new test or harness case uses its own `--home`, never a committed one, and a test-side removal takes only a dir the test itself created
