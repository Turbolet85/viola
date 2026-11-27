# security · standing conventions

- S1 · each new test runs under a home of its own, never the operator's viola home or Claude home, and nothing of a test home enters git · secret-management · bootstrap-phases
  decision/no-secrets-in-git · No secrets or local homes in git — .env files, local --home test directories and signing material are never committed to git.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S2 · each new test removes only the directories it created itself, following only a safe linked base for the e2e home · security-anti-patterns-never-do-these · security-anti-patterns
  decision/e2e-home-keepers-linked-base · e2e-home keepers follow only a safe linked base — The two target/e2e-home keepers never follow a linked base unless its target is absolute and an owner-only real directory, create the target 0700 and remove nothing.
  decision/e2e-home-link-guard · E2E home linked base guard — The e2e home keepers follow a linked base only when its target is absolute and reads as a real directory with no group or other bit, refuse a link to a link or a backing below the 9 GiB headroom floor with a fixed message naming no path, create the target 0700 and remove nothing. · why: a ratified boundary widening: on the dev host the path links outside the working directory
  decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
  decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.
- S3 · each test seam a new test uses sits only under the fake-agent feature and is compiled out of the release build · security-anti-patterns · input-validation
  decision/no-seam-for-foreign-peer · No seam to drive a foreign peer — A foreign euid and a foreign owner are held by injected values in tests, and no seam may be added to drive them. · why: one account can make no foreign peer
  decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
  decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
- S4 · no new test runs the real claude CLI, and CI keeps holding no secrets · threat-model-summary
  decision/ci-no-secrets-no-real-claude · CI holds no secrets — CI holds no secrets and never runs the real claude CLI, which runs only locally.
  decision/no-repo-secret-scanner · No repository secret scanner in v1 — v1 runs no repository secret scanner; revisit when the v1.x release workflow adds signing credentials. · why: there are no secrets in the repo or CI
  decision/real-cli-local-only · Real claude CLI only locally — The real claude CLI runs only locally; in CI viola verify runs only against the fake agent, and its real-CLI probe and record only locally.
