# Classified — obs-plan · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · the Cookie-header secret-scan test is not in the reached files and CI's secret scan is unchanged
pii-scrubbing-compliance#1.2 · no · the R8 strip is not in the reached files, and the fake agent's added field is an integer, never an env name or value
pii-scrubbing-compliance#1.3 · no · no signing or CI step changed, and CI is green 15/15 with its gate set unchanged
pii-scrubbing-compliance#1.4 · no · the confirmed-send and wait/last paths are untouched and no line or field is added
pii-scrubbing-compliance#1.5 · no · the dialog-answer path is untouched, no tool-input code is reached, and no field crosses into a log
pii-scrubbing-compliance#1.6 · no · the serde_path_to_error drift seam is not in the reached files and no line is added
pii-scrubbing-compliance#1.7 · no · no statusline code is reached, and strict.rs changed only a cfg(windows) test oracle with its product verdicts unchanged
pii-scrubbing-compliance#1.8 · no · the report holds the §8 path-free home-line rule unchanged, since place_private_new adds no line or span
pii-scrubbing-compliance#1.9 · no · pin.rs changed only the pinned-copy write path, and the report holds the pinned_bin never-logged rule unchanged, since place_private_new adds no line or span
pii-scrubbing-compliance#1.10 · no · no typed field is added or moved, and spawn_os_error is an integer on a test-only receipt that crosses into no log
pii-scrubbing-compliance#1.11 · no · the claude agents --json read and its row logging are not in the reached files and no line is added
pii-scrubbing-compliance#1.12 · no · the link and unlink code and its no-names test are not in the reached files
pii-scrubbing-compliance#1.13 · no · no code or field is added, the lost placement reuses the existing pinned-hash-mismatch code, and the closed enums hold
pii-scrubbing-compliance#1.14 · no · the change adds no fixture (the receipt field and Path 7's boot are test code), so obs adds no fixture content
pii-scrubbing-compliance#2.1 · no · the CI secret scan and uploads are unchanged with no CI step changed, strict.rs changed only a cfg(windows) test oracle, and fs.rs adds only the pinned-copy placement
