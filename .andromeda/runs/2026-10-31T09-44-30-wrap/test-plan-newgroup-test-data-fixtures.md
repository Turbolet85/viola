# New groups — test-plan · test-data-fixtures

## Verdicts
- cascade-compare-test-plan.md 1 · contradiction · lines 14–14 · «`Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on that one child» · the proposal says `Wrapper` takes no environment, while this line has a `Wrapper` boot setting `HOME` on its one child; the row's other facts (the `env_set` planting, the `planted` class of `assert_error_body_clean`) stand nowhere in the file
