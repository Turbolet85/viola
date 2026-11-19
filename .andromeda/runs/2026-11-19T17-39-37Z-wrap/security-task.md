@task_prose part=session
The session recorded one contained live StopFailure payload on claude 2.1.328 (start SF, 2026-11-19, error word server_error, cause server-error, green on predicates 1-7) through a new SF arm of the test-only live rig; the spare SF-2 start was not spent.
The StopFailure exception is settled as outcome B on the founder's word on that green reading: kept, no longer provisional, its rows to cite the live reading (2.1.328, 2026-11-19, SF, server_error); outcome A (a ledger row with a verify probe) is not opened.
The SF arm runs the real CLI inside a network-closed bwrap sandbox, refuses with refused: sandbox (exit 3, before any credentials read or ledger line) when the sandbox is unavailable, and refuses no-runtime-dir unless XDG_RUNTIME_DIR is absolute and outside the temp root.
The rig's StopFailure capture widens, the one boundary widening of the chunk: key names, the error word only from the CLI's closed 13 (else other) and the byte lengths of two text fields, never a text; the rig also reads the input box row's cell attributes, never text, after the turn end, keying one x and one DEL into its own box.
The fake agent gains two test-only argv options (--turn-fail and --settings-hooks, argv only, no shell, no env read); the rig's kept record dir is keyed by CLI version first and refuses a standing version, writing nothing.
No product crate, product schema, config key, env var, dependency, channel method or MCP tool changed; all of it is test-only.
@task_file n=1 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's isolation of its child: the scratch home, the kept record (now keyed by CLI version, refusing a standing version), the network-closed sandbox argv, the runtime-dir precondition, the program lookup on an absolute PATH dir, the seeded StopFailure capture hook config
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the live rig's arms and plans: the new SF arm (contained, trusted, marker-seeded), the sandbox probe and its refusals (absent, no-runtime-dir, failed, shared), the network-namespace read, the turn-end and cause reading, the box cell-attribute read after the turn end
@task_file n=3 path=crates/viola-e2e/src/rig/capture.rs
what: edited — the live rig's capture hook: its closed per-line record of a hook payload, now with a StopFailure line holding key names, the error word of the CLI's 13 else other, and two text lengths; read through MAX_FRAME + 1 at default JSON depth, owner-only append
@task_file n=4 path=crates/viola-e2e/src/rig/screen.rs
what: edited — the live rig's screen model: now reads a cell's attributes as closed words, never its text
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the live rig's command line: rehearse sf with --contain, live SF and SF-2, every live SF contained
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the rig's rehearsals over the fake agent: four sf rehearsals, needle absence from stdout, ledger and out dir, the sandbox refusal before the ledger line
@task_file n=7 path=src/bin/viola-fake-agent.rs
what: edited — the test-only fake agent binary: new argv options --turn-fail and --settings-hooks (a settings.json's hooks run beside the plugin's, command split on spaces, no shell)
@task_item n=1 part=touches
decision/unverified-cli-gate: its StopFailure exception, owned by the entry A live StopFailure payload, is now spent and settled as outcome B on the founder's word, citing the live reading (2.1.328, 2026-11-19, SF, server_error); no longer provisional; outcome A not opened.
@task_item n=2 part=touches
decision/live-rig-prompt-capture-closed: the rig's capture gains the StopFailure line — key names, the error word only from the CLI's closed 13 (else other), the byte lengths of two text fields, never a text — read through MAX_FRAME + 1 at default JSON depth, owner-only append.
@task_item n=3 part=touches
decision/live-rig-isolated-child: the SF arm's child runs inside a network-closed bwrap sandbox (its own network namespace), with refused: sandbox before the ledger line and the no-runtime-dir refusal.
@task_item n=4 part=touches
decision/live-rig-credentials-copy: the sandbox refusal comes before any credentials read, so a refused SF start copies no credentials; the live SF teardown read credentials removed.
@task_item n=5 part=touches
decision/api-error-classes-adapter-only: the rig's capture holds a test-only copy of the 13 error words taken from the Claude adapter's cause map, and maps the captured word to the cause through the adapter; a test-only exception to keeping the classes inside the adapter.
@task_item n=6 part=touches
decision/api-error-turn-end-text-dropped: the live reading confirmed a StopFailure turn end carrying cause server-error; the capture keeps only lengths of the error-details and last-assistant-message texts, never the text.
@task_item n=7 part=touches
decision/live-round-rig-keys-own-pty: the SF arm keys one x and one DEL into its own box after the turn end to read cell attributes.
@task_item n=8 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: two new fake-agent argv options (--turn-fail, --settings-hooks) are new test seams, built only with the fake-agent feature.
@task_item n=9 part=touches
decision/release-build-no-test-binaries and decision/release-build-without-test-features: the test-only fake agent and the viola-e2e rig grew; still test-only, no product crate reaches them.
@task_item n=10 part=touches
decision/capture-arms-isolated and decision/hook-capture-arm: a new hidden capture path, the rig's StopFailure capture hook in the rig's private settings.json, beside the existing capture arms.
@task_item n=11 part=touches
decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude: the live SF start ran the real claude locally only; CI runs the unwrapped sf rehearsals over the fake agent; whether CI skipped the contained rehearsal is unread.
@task_item n=12 part=touches
decision/nothing-written-under-claude-home, decision/no-writes-under-claude-home and decision/viola-writes-nothing-in-claude-dir: the live SF witness read user-config changed-by-others (the host's own Claude sessions rewrite the user config), no rig needle in it; the CLI's own writes elsewhere under the home stay the named unwitnessed residual.
@task_item n=13 part=touches
decision/pre-push-empty-environment-launch and decision/hermetic-child-environment: under the empty pre-push environment XDG_RUNTIME_DIR is unset, so the contained rehearsal now skips by its printed probe (no-runtime-dir) instead of failing.
@task_item n=14 part=touches
decision/no-socket-in-tmp-root and decision/socket-in-per-user-dir: with XDG_RUNTIME_DIR unset viola placed its socket dir under the temp root, which the sandbox's private temp root hid; the rig now requires an absolute runtime dir outside the temp root before a contained start.
@task_item n=15 part=touches
decision/child-path-trust: the rig resolves bwrap only from an absolute directory of its own PATH as an executable, used by path, never installed.
@task_item n=16 part=touches
decision/max-frame-cap and decision/reads-capped-at-max-frame: the new StopFailure capture read is bounded by MAX_FRAME + 1, oversize or non-object read as unread.
@task_item n=17 part=touches
decision/artifact-canary-scan and decision/fixture-hygiene: the sf rehearsals plant needles in copied 2.1.328 StopFailure fixtures and assert none reaches stdout, the ledger or the out dir.
@task_item n=18 part=touches
decision/input-box-readiness-from-literals: the rig's screen model now reads cell attributes as closed words, never a cell's text, beside the literal-based readiness read.
@task_fact n=1 slug=secret-management
fact: The live rig's SF arm runs the real claude inside a network-closed bwrap sandbox with its own network namespace; when bwrap is absent, fails, or shares the rig's network namespace the arm refuses with refused: sandbox, exit 3, uncounted, before any credentials read or ledger line; bwrap is a test-only host dependency of the rig alone, used by path from the rig's PATH, never installed.
@task_fact n=2 slug=secret-management
fact: A contained start is refused as no-runtime-dir unless XDG_RUNTIME_DIR is absolute and outside the temp root, so viola's socket directory stays reachable outside the sandbox's private temp root; the contained rehearsal skips there by its printed probe.
@task_fact n=3 slug=data-protection
fact: The live rig's kept record is keyed by CLI version first: a set for a version already kept is refused as exists and nothing is written, while another version's set is written beside the standing ones.
@task_fact n=4 slug=input-validation
fact: The fake agent's --turn-fail and --settings-hooks options are argv-only test seams in fake-agent builds alone: no env read, an absolute exec, a hook command with no args split on spaces with no shell.
@task_item n=1 part=untouched
The MCP tools, the wrapper channel methods and their frames, refusals and error codes: no channel method, MCP tool or frame changed.
@task_item n=2 part=untouched
Authentication and authorization at the seams: the socket directory and peer euid checks, server verification, strict-modes refusals and the home's owner-only modes; no product seam changed.
@task_item n=3 part=untouched
Dependency security: no dependency added or bumped, no toolchain, workflow, action pin, advisory or licence change.
@task_item n=4 part=untouched
The revive, list, link and unlink contracts and CLI exit codes: no product CLI verb or output changed.
@task_item n=5 part=untouched
The product hook's parsing and its closed security fields, the product config.json keys and the product's env handling: no product crate, schema, config key or env var changed.
@task_item n=6 part=untouched
The absences (hosting, public listener, outbound network calls by viola, secrets, accounts, release stage): viola itself gains no network call, listener, secret or release artefact.
@task_item n=7 part=untouched
Verify's probes, stamps and fixture refusal codes and the pinned binary and plugin placement: verify and the pin path did not change.
