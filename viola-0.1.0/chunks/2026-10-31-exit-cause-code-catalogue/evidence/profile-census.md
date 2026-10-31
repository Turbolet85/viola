# Profile census of the new kill-shaped row

Read 2026-10-31T17:34:56Z, after entry 20 (the pre-push whose instrumented build it reads), under `timeout -k 10 300`.

`bash scripts/profraw-census.sh --case cross_exit_causes a_wrapper_killed_under_a_parked_wait_is_endpoint_lost::case_1_json 40 4`, exit 0:

```
profraw-census: case cross_exit_causes a_wrapper_killed_under_a_parked_wait_is_endpoint_lost::case_1_json · runs 40 · workers 4
profraw-census: runs 40 · passed 40 · profiles 160 · refused 0
```

Host record for the block just before it: `QUIET for viola - 10 s stalled on IO, but 10 s of them with only viola's own build alive`.
