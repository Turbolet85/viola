@row_add n=1 row=decision/live-rig-records-closed-words area=8-pii-scrubbing-compliance fact=F1 detector=D-obs-pii
name: Live rig records carry closed words only
rule: The live rig's records (its stdout summary, its start ledger lines and its frame index) carry closed words only, never a path, credential or config value, and the user's claude.json is compared by a hash prefix and mtime alone, never kept.
guard: rig_keeps_no_credential_or_host_path_in_any_record
basis: fct-nwy7zwl7
because: The chunk adds a standing test-only live rig that reads credentials and the user's claude.json, and every later live round reuses it, so its records' closed-word rule must stand beside the product's PII rules.
@row_add n=2 row=decision/live-rig-frames-kept-local area=8-pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Live rig screen frames kept local
rule: The live rig's kept screen frames stay local under the gitignored target tree, are never uploaded and are never a telemetry source.
except: A kept live frame may hold the scratch cwd path as the CLI draws it.
basis: fct-k4d3lfbo
because: The chunk keeps live screen frames that may hold a scratch path, so the plan must say where they stay and that no upload or telemetry reads them.
@row_add n=3 row=decision/no-real-cli-start-in-ci area=9-ci-integration fact=F3 detector=none
name: No real-CLI start in CI
rule: The live rig's rehearsals run in CI against the fake agent only, and its live arm against the real CLI refuses to start under CI, so no real-CLI start ever runs in CI.
guard: rig_live_arm_refuses_under_ci
basis: fct-sjq6de27
because: The chunk adds the rig's rehearsals to CI and a live arm that refuses under CI, a gate the CI section must carry.
