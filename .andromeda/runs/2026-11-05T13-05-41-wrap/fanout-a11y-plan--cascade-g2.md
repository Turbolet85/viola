@proposal n=1 detector=D-cascade severity=warning file=cognitive-accessibility anchor=fct-6edxk4iz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: ## 8. Cognitive Accessibility › ### Timeout extensions
change: the closing clause «(green on the three CI runners, run 38098238731)» reads as history, not current: «(green on run 38098238731, when CI had three OS legs; CI is now ubuntu-latest alone)»; every time bound and the rest of the entity unchanged
sidecar: cognitive-timeouts: the budget-gate case's three-runner reading re-worded as history (Linux-only, CI on ubuntu-latest; chunk 2026-11-05-linux-only-code-and-ci)
rationale: the report's counts bullet moves the CI matrix to [ubuntu-latest] and rules a dated measurement on windows-2025 is re-worded by its owner, never left as current (fct-tjt6dmfu); the timeout bounds themselves did not move
@proposal n=2 detector=D-retired severity=warning file=cognitive-accessibility anchor=fct-ms6n7636 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 8. Cognitive Accessibility › ### Error recovery
change: «**CLI, an unstattable home:** a home path under a regular file is a strict-modes refusal with its hint (`verify`: the home-state pair, exit 1; the six frame verbs: the instance-state pair, exit 21).» — the «on Unix» qualifier and the whole Windows clause (NotFound skip, the catch-site fault line with no hint, the frame verb's `internal-error` document there) dropped
sidecar: cognitive-errors: the unstattable-home entity loses its Windows arm (OS target Linux only; the Windows strict-modes arm retired)
rationale: the report's Retired bullet retires the OS target «Windows first, macOS and Linux» and the Windows home owner/DACL strict-modes arm (fct-tulramlm), and Symbols removes strict.rs's Windows arm with check_socket_dir's Unix body unconditional (fct-xxutheix)
@proposal n=3 detector=D-retired severity=warning file=cognitive-accessibility anchor=fct-g3h7n6e2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## 8. Cognitive Accessibility › ### Error recovery
change: the cli bullet's tail reads «These run on ubuntu-latest, `tests/security_negatives_server_identity.rs` among them (ungated).» in place of the three-OS-legs and not(target_os = "macos") sentence; the `cli, macOS` bullet is dropped whole; the client euid bullet's «its real read on Linux and macOS» reads «its real read on Linux»
sidecar: cognitive-errors: verification inventory to one leg (ubuntu-latest); the server-identity file ungated; the macOS no-peer-pid carve-out retired
rationale: deviation 11 made security_negatives_server_identity.rs unconditional (fct-iol4iyl6); the Retired bullet retires the macOS no-peer-pid carve-out and the windows-2025 and macos-latest legs (fct-tulramlm); SERVER_PID_REPORTED and PEER_EUID_REPORTED folded (fct-xxutheix)
@proposal n=4 detector=D-retired severity=warning file=cognitive-accessibility anchor=fct-cetk7x7u basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 8. Cognitive Accessibility › ### Error recovery
change: the closing parenthesis «`cfg(unix)`, so on ubuntu-latest and macos-latest only» reads «`cfg(unix)`, run on ubuntu-latest»; the rest unchanged
sidecar: cognitive-errors: link/unlink strict-modes case placed on ubuntu-latest alone (macos-latest leg retired)
rationale: the Retired bullet retires the windows-2025 and macos-latest CI legs (fct-tulramlm); ci.yml keeps strategy.matrix.os [ubuntu-latest] (fct-sy4jxsfp)
@proposal n=5 detector=D-retired severity=warning file=cognitive-accessibility anchor=fct-52gvy64u basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 8. Cognitive Accessibility › ### Error recovery
change: «(`tests/cli_verify.rs`, the latter's oracle per OS: the home-state pair on Unix, the catch-site line on Windows)» reads «(`tests/cli_verify.rs`, the latter's oracle the home-state pair)»; the last sentence «Unlike the server-verify cases, these run on all three OS legs.» reads «These run on ubuntu-latest, as the server-verify cases now do.»
sidecar: cognitive-errors: strict-modes verification loses its Windows oracle and three-leg contrast (Linux only, CI on ubuntu-latest)
rationale: the Retired bullet retires the OS target and the Windows home strict-modes arm (fct-tulramlm); deviation 11 ungates security_negatives_server_identity.rs, removing the contrast (fct-iol4iyl6)
@proposal n=6 detector=D-retired severity=warning file=cognitive-accessibility anchor=fct-trymhbcp basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## 8. Cognitive Accessibility › ### Error recovery
change: «**Verification, the socket-directory refusal:** `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`) is its witness, on ubuntu-latest; `check_socket_dir`'s Unix body is unconditional.» — the Windows-witness gap, macos-latest and the off-Unix reads-nothing clause dropped
sidecar: cognitive-errors: socket-directory verification to its Linux witness; the off-Unix check_socket_dir arm removed
rationale: Symbols removes the off-Unix Ok(()) arm of check_socket_dir, its Unix body unconditional (fct-xxutheix); the Retired bullet retires the macOS socket dir and the macos-latest leg (fct-tulramlm)
@proposal n=7 detector=D-retired severity=warning file=body anchor=fct-2jdilj7v basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## 8. Cognitive Accessibility › ### Error recovery
change: «**Verification:** trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on ubuntu-latest (the frame verbs' server-verify witness and the socket-directory refusal among them).»
sidecar: §8 Error recovery broad line: three OS legs and the macOS / windows-2025 carve-outs to ubuntu-latest alone
rationale: the Retired bullet retires the three-OS target, the macOS no-peer-pid carve-out and the windows-2025 and macos-latest legs (fct-tulramlm); security_negatives_server_identity.rs now ungated (fct-iol4iyl6)
@remark
F1 (slug=a11y-scope-summary) is not this group's; the same macOS carve-out and three-leg wording stand in security-plan and test-plan, owned by their passes.
