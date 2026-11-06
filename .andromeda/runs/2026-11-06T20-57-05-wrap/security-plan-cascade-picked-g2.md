@account n=1 entity=error-handling#1.1 v=not_touched
why: hook.rs moved by one line with no log field, event kind or exit path added, so the fail-open detail list still holds
@account n=2 entity=error-handling#1.2 v=not_touched
why: the server-verify-failed path is unchanged; the report adds no hook-decision detail and removes none
@account n=3 entity=error-handling#1.3 v=not_touched
why: the socket-dir-refused hook path is unchanged; the live round re-read the socket directory 0700 unmoved
@account n=4 entity=error-handling#1.4 v=not_touched
why: the replayed-snapshot channel-unreachable path is untouched; no snapshot or log surface moved
@account n=5 entity=error-handling#1.5 v=not_touched
why: hook statusline over a replayed snapshot is unchanged; the statusline shell-out rules were left alone
@account n=6 entity=error-handling#1.7 v=not_touched
why: no viola-mcp file is in the change and no MCP result word moved
@account n=7 entity=error-handling#1.9 v=not_touched
why: client.rs moved by one line with no channel method or fault reader change; sanitised_fault still holds
@account n=8 entity=error-handling#1.12 v=not_touched
why: the is_code_word admission is untouched; no refusal or detail word was added
@account n=9 entity=error-handling#1.13 v=not_touched
why: the code-word exception is not one of the five error-enum divergences T1 ratifies, and its exit is unchanged
@account n=10 entity=error-handling#1.16 v=not_touched
why: the -32603 absence and its unit witnesses are unchanged; no failpoint or panic seam was added
@account n=11 entity=error-handling#1.17 v=not_touched
why: refuse_untrusted and exit 21 are unchanged; send lost only the Git Bash warning, not its strict-modes refusal
@account n=12 entity=error-handling#1.18 v=not_touched
why: exit 21's six causes are unchanged; the report adds no cause word
@account n=13 entity=error-handling#1.19 v=not_touched
why: the peer euid refusal words in client.rs and hook.rs are unchanged; those files moved by one line each
@account n=14 entity=error-handling#1.20 v=not_touched
why: list.rs moved by one line with no change to refused_document or the exit-21 writer
@account n=15 entity=error-handling#1.21 v=not_touched
why: the SocketDir mapping on CLI frame verbs is unchanged; the report adds no hint or process-exit field
@account n=16 entity=error-handling#1.22 v=not_touched
why: the unwrapped-name and endpoint-lost hints are unchanged; the HELP literal and the help tests are byte-identical
@account n=17 entity=error-handling#1.23 v=not_touched
why: the socket-directory refusal text is unchanged; no path, pid or uid line was added
@account n=18 entity=error-handling#1.24 v=not_touched
why: the exit-2 UNUSABLE_TEXT line stands; only the separate Git Bash rewritten-path stderr warning left send.rs
@account n=19 entity=error-handling#1.25 v=not_touched
why: the run, revive and verify strict-modes refusals are unchanged; strict.rs and stamps.rs moved by comment lines only
@account n=20 entity=error-handling#1.29 v=not_touched
why: StampsError's fixed Display messages are unchanged; stamps.rs moved by one line with no surface added
@account n=21 entity=error-handling#1.31 v=proposed
why: the thiserror census rule is re-worded to the product sources (grep reads 10, the tenth test-only) and the count stays 9
@account n=22 entity=error-handling#1.32 v=proposed
why: the five divergences are ratified as named exceptions on the founder's word (T1), so the open divergences and their Version done-check CARRY owner are wrong
@account n=23 entity=error-handling#1.38 v=not_touched
why: check_diagnostics' failed-open rule is unchanged; strict.rs moved by comment lines only
