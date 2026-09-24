# design extract

## No domain coverage
This chunk builds only test tooling: the `viola-fake-agent` test binary, the harness receipt format, the rstest temp-home fixtures, the fixture scrub-and-schema walk, proptest seeds and the harness `mutants` verdict. None of it is covered by design-system.md. The plan's §Surface: cli covers only "stdout/stderr of the `viola` binary", and §Surface: web-spa covers only the `viola ui` page. No tokens, SGR styling, phraseology or component pattern applies. The design amendment history (`.andromeda/design-system-amendments.md`) does not exist yet, so it is empty.
