# design · Self-Validation Protocol — category read

## Carried
- (.andromeda/design-system.md:391) The chunk's per-cause `hint:` lines and refusal forms are human CLI output, so they must pass the protocol's checks before they are presented.
  > Before presenting ANY UI output, downstream implementation phases (per project's specialist plans) must run these checks:
- (.andromeda/design-system.md:404) Any change the catalogue makes to `viola send`'s refusal output must keep the readback column in its drawn form, and the table's `send` rows must be read against this test.
  > Point to the readback box in its one 0.1.0 place: the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`. If that column lacks the box or draws it differently, the signature does not exist. Inject it.

## Bindings
- **Signature Test ↔ body, Surface: cli pattern 2**: a `viola send` refusal must keep the mirror and one hint per cause, while the exit-21 strict-modes refusal's fixed `unable:` / `hint:` pair stands outside the mirror by rule, so that pair is no failed signature, (.andromeda/design-system.md:404) and (.andromeda/design-system.md:303).
- **Signature Test ↔ body, Surface: cli colour decision order**: the test reads human output only, since only `--json` replaces the ASCII mirror with its JSON document, so a distinct `--json` code must not bring the mirror into the document, (.andromeda/design-system.md:404) and (.andromeda/design-system.md:244).
