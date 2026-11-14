# security · standing conventions

- S1 · every test runs against its own home, never the live home, and removes only the directories it created · secret-management
  decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S2 · every recorded fixture added or re-recorded is scrubbed of paths, usernames and emails and reviewed before commit · data-protection
- S3 · every test seam stays under the fake-agent feature and is compiled out of the release build · input-validation
  contract/release-check-output/test-only-feature · release-check output · release-check: FAILED — test-only feature {feature} in {target} — Printed when a release artifact's features include a test-only feature.
