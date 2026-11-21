@row_set n=1 row=decision/verify-children-direct-spawn cell=rule seen=1 detector=none
value: viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals; Run B's settings take a compiled accept literal, and the sender's prompt holds compiled literals and the composed uds address alone.
basis: fct-zjeenbpz
because: verify now runs a print-mode sender whose prompt carries the composed uds address and gives Run B a compiled cross-session accept settings literal, which the row's child set leaves out.
@row_set n=2 row=decision/probe-prompts-synthetic cell=rule seen=1 detector=none
value: Probe prompts used to record committed fixtures must be synthetic; the composed uds address is the one non-literal part of the cross-session sender's prompt.
basis: fct-ugegm744
because: the sender's prompt now carries a composed address that is no compiled literal, and the recorded cross-session arrival holds it.
@row_set n=3 row=decision/fixture-recording-scrubbed cell=rule seen=1 detector=none
value: verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a home, drive-letter or user-folder path, the user word or an email-shaped token survives, naming file and closed code, never content.
basis: fct-wn36g3ew
because: the wrap measured that the scrub and the hygiene walk read only home, drive-letter and user-folder paths, so a runtime-dir path passes and the row's any-path refusal is false.
@row_set n=4 row=decision/fixture-scrub-and-refusal cell=rule seen=2 detector=none
value: Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home, drive-letter or user-folder path, the user word or an email-shaped token survives.
basis: fct-wn36g3ew
because: a runtime-dir absolute path passes the recording's check, so the row's refusal of any surviving absolute path is false.
@row_set n=5 row=decision/live-rig-kept-stamp-owner-only cell=rule seen=4 detector=none
value: The live rig keeps a stamp from its CS, WS, PQ, HF, HG, XS or XS-2 scratch home owner-only and no-clobber, and seeds it only into its own CC or R3W scratch home; CC with no kept stamp refuses before its ledger line and before any scratch, R3W before its ledger line.
basis: fct-whtuymhr
because: the rig's XS and XS-2 labels now keep stamps of their own, which the row's closed list of keeping arms leaves out.
@row_set n=6 row=decision/hook-capture-arm cell=except seen=1 fact=F5 detector=D-security-input
value: Under --names-only the arm files only closed words, a tool class and a send result read from the success boolean alone, never a payload byte; an unknown shape reads unread.
basis: fct-zjeenbpz
because: the capture arm gained a names-only mode that files no raw stdin, so the row's raw-write rule now has a named exception.
@row_set n=7 row=decision/shared-validators-in-core cell=except seen=3 detector=D-security-input
value: The Hyprland window Address validator lives in the seam crate; the Claude transcript path shape check, called by hook and wrapper, and verify's uds address check live in the Claude adapter crate.
basis: fct-zjeenbpz
because: the new uds address validator stands in the Claude adapter crate, outside the core crate, and the row's list of such exceptions must name it.
@row_set n=8 row=decision/no-stored-secrets cell=except seen=3 detector=D-security-auth
value: test-only seams, list's terminal colour facts, XDG_RUNTIME_DIR for the socket directory and verify's sender address, and CLAUDE_CONFIG_DIR's value for verify and run and revive's start when kept
basis: fct-zjeenbpz
because: viola verify now reads XDG_RUNTIME_DIR to compose the sender's address, an environment read the row's closed exception list leaves out.
@row_add n=1 row=decision/verify-cross-session-probe-residuals area=data-protection fact=F1 detector=none
name: Verify's cross-session probe residuals accepted
rule: verify's cross-session probe residuals are accepted as measured: the address closes the target save the pid-reuse window, W4 and T5, the closed-word and sender plugin files a SIGKILL leaves, the peer key's unmeasured no-token frame, and the address reaching the model provider.
why: Accepted residuals of the ratified cross-session probe.
basis: fct-ugegm744
because: the cross-session probe adds accepted residuals of its own, and the plan names a new residual row beside the echo probe's.
@row_add n=2 row=decision/cross-session-fixture-keeps-runtime-path area=data-protection fact=F2 detector=none
name: Cross-session fixture keeps the sender's socket address
rule: The committed cross-session fixture keeps, inside the CLI's own prefix, the recording host's runtime-dir path with the uid, the sender pid and a seven-digit from-name number; the runtime dir reaches no log, stamp or fixture verify writes of its own.
why: The record scrub rewrites only the home and the user word, and the hygiene walk reads only home and user-folder paths.
basis: fct-wn36g3ew
because: the wrap measured what the recorded cross-session arrival keeps through the scrub, a data-protection fact no row states.
@row_add n=3 row=decision/verify-runtime-dir-held-in-memory area=secret-management fact=F3 detector=D-security-auth
name: Verify's runtime dir held in memory only
rule: viola verify reads XDG_RUNTIME_DIR once, holds it in memory and uses it only to compose the claude CLI's inbox socket address for the sender's prompt; it enters no log line, stamp, fixture, error or step line.
why: A ratified environment read.
basis: fct-zjeenbpz
because: verify gained a new environment read whose value must stay out of every record, which no row states.
@row_add n=4 row=decision/uds-address-closed-shape area=input-validation fact=F4 detector=D-security-input
name: Composed uds address held to a closed shape
rule: The uds address verify composes is built only from an absolute runtime dir of bytes A-Z a-z 0-9 / . _ - with no dot or dot-dot component and a path of at most 103 bytes, else the row reads not-run; the sender prompt re-checks the shape, and no directory is read to compose it.
basis: fct-fh35e6ia
because: the runtime dir is a new external input that reaches a child's prompt, and its validation is the boundary's rule.
@row_add n=5 row=decision/test-verify-owned-runtime-dir area=secret-management fact=F6 detector=none
name: Test verifies given an owned runtime dir
rule: Every test and harness viola verify is given a short owner-only test-owned runtime dir for the stamp's life.
why: The pre-push gate's empty environment leaves XDG_RUNTIME_DIR unset, and every stamp of the relied rows depends on it.
basis: fct-m4yamtxm
because: verify's new environment read made every test stamp depend on a runtime dir, so tests now supply their own rather than any host value.
