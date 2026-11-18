@row_set n=1 row=decision/verify-own-trust-record cell=rule seen=1 detector=D-arch-decisions
value: viola verify writes a trust record for its own temp folder only into the Claude config file its child reads, never creating an absent file, and removes that one record after.
basis: fct-vwyqtjra
because: The rule «run and revive write no trust record» is retired by this chunk, since a wrapped start now writes the CLI's trust record, so the row must stop saying it.
@row_set n=2 row=decision/run-start-order cell=rule seen=2 detector=none
value: A start resolves the program, checks collision, pins the copy and plugin, gates the version, on a verified CLI writes the trust record of an untrusted spawn folder, binds the endpoint, writes the first snapshot, heartbeat and start events, and only then spawns the child.
basis: fct-xro6zomc
because: The start gained a trust step after the version gate and before the endpoint bind, so the row's start order is incomplete without it.
@row_set n=3 row=contract/instance-snapshot/cwd cell=promise seen=1 detector=none
value: `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its one reader is `viola revive`, which uses it as the revived child's spawn directory and as the folder its start's trust record is keyed by.
basis: fct-rntilpcs
because: The revive's start now also keys its trust record by the recorded cwd, so «uses it only as the spawn directory» no longer holds.
@row_set n=4 row=decision/spawn-directory-confined-to-snapshot cell=rule seen=1 detector=none
value: The spawn directory host path lives, among viola's own files, only in the owner-only (0600) instance snapshot, on no event, log line, error body or stdout; the start's trust record keys the child's Claude config entry by it.
basis: fct-rntilpcs
because: A wrapped start now writes the spawn folder as the key of its trust record in the child's Claude config, so «lives only in the snapshot» must name that second place.
@row_set n=5 row=contract/trust-writer-words/key-present cell=promise seen=1 detector=none
value: The config already holds a record under the key; it is left as found and nothing is written. Only viola verify's writer gives it; a start reads it as lost.
basis: fct-krdmyvxv
because: The start maps verify's key-present to lost, so the word is verify's only and the row must say so.
@row_set n=6 row=contract/trust-writer-words/removed cell=promise seen=1 detector=none
value: The written trust record was removed again, every other key of the config left as found. Only viola verify gives it; a start reads it as lost.
basis: fct-krdmyvxv
because: The start maps verify's removed to lost, so the word is verify's only and the row must say so.
@row_set n=7 row=decision/env-strip-names-only cell=except seen=2 fact=F5 detector=D-arch-resources
value: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
basis: fct-xro6zomc
because: CLAUDE_CONFIG_DIR's value is now read by the run and revive start too, through the same resolution verify uses, so «verify reads» is incomplete.
@row_set n=8 row=decision/env-strip-reads-names-only cell=except seen=2 detector=D-arch-resources
value: viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.
basis: fct-xro6zomc
because: CLAUDE_CONFIG_DIR's value is now read by the run and revive start too, so the exception naming verify alone is incomplete.
@row_set n=9 row=decision/no-environment-configuration cell=except seen=2 detector=D-arch-resources
value: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.
basis: fct-xro6zomc
because: The exception said CLAUDE_CONFIG_DIR's value is read by verify only, which the run and revive start's new read makes false.
@row_set n=10 row=decision/env-var-viola-prefix cell=except seen=2 detector=D-arch-resources
value: Test seams `FAKE_AGENT_PUMP_DELAY_MS`, `FAKE_AGENT_HOOK_PANIC`; `NO_COLOR`, `TERM`, `COLORTERM` for SGR; `PATH`, read and set for the child; `XDG_RUNTIME_DIR`; a kept `CLAUDE_CONFIG_DIR`
basis: fct-xro6zomc
because: A kept CLAUDE_CONFIG_DIR is now read by the run and revive start as well as by verify, so «verify's kept» is no longer the whole exception.
@row_set n=11 row=contract/config-json/claude-env-keep cell=promise seen=2 detector=D-arch-resources
value: The R8 pass-list of CLAUDE* names, an array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$, default empty; a bad value is rejected whole, and an identity-floor name in it has no effect. Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.
basis: fct-xro6zomc
because: Keeping CLAUDE_CONFIG_DIR now makes the run and revive start read its value too, so the promise naming verify alone is incomplete.
@row_add n=1 row=decision/start-writes-trust-record area=established-decisions fact=F1 detector=D-arch-decisions
name: A wrapped start writes the trust record
rule: On a verified CLI, a run or revive start sets only hasTrustDialogAccepted in an untrusted spawn folder's entry of the child's Claude config, every other key kept, inserting a one-key entry where none stands; the outcome never refuses the start.
why: So no trust dialog stands at the child's start and no key is typed into one.
guard: run_in_an_untrusted_folder_writes_the_record_and_types_no_key
except: Nothing is written for the user's home folder, a folder whose own key is trusted, or a folder under a trusted parent.
basis: fct-xro6zomc
because: The chunk lands a second, flag-only write under the Claude config at every verified wrapped start, a decision future work must respect.
@row_add n=2 row=decision/start-trust-walk area=established-decisions fact=F2 detector=none
name: Start trust walk follows the CLI's
rule: The start reads a folder as trusted the way the CLI does, by its exact key first and then each parent folder up to the git root; the walk, the flag set and the exact read-back are Claude-shape and pure, and the write goes through the guarded writer with three read-back attempts.
guard: trust_walk_reads_the_exact_key_then_each_parent
basis: fct-xro6zomc
because: The start's trust step chose to follow the CLI's own walk and to keep its Claude-shape logic pure behind the one guarded writer, which binds any later change to the trust step.
@row_add n=3 row=contract/start-trust-words/word-set area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Start trust words
item: word set
promise: A closed set of 16 code words, the detail of the trust-record line; viola verify's key-present and removed read as lost at the start.
basis: fct-vrquvti6
because: The chunk lands a new closed word set of 16 start words that the trust-record line's readers rely on.
@row_add n=4 row=contract/diag-line/trust-record area=occupied-resources fact=F4 detector=D-arch-resources
boundary: diagnostics line
item: trust-record
promise: One trust-record line at INFO per run or revive start, its detail one of the 16 start trust words; it carries no path, key or value, and no span instruments the start's trust step.
basis: fct-xro6zomc
because: The closed obs event set gains the trust-record event, an occupied event that must stand in the registry.
@row_add n=5 row=decision/test-owned-user-home area=occupied-resources fact=F6 detector=none
name: Product-starting tests own their user home
rule: A test that starts the product never runs under the runner's real HOME: every test run and revive and every supervised harness wrapper gets a test-owned user home.
why: The start writes into the user's Claude config.
basis: fct-o4xnvz66
because: Once a start writes into the user's Claude config, a test under the real HOME would lock, read and write the operator's own config, so the tests chose to own their user home.
