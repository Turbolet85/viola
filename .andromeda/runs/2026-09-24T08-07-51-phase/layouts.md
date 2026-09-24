# layouts extract

## No domain coverage
This chunk only builds test-harness pieces: the fake-agent bin and its scripted modes and receipts, the rstest temp-home fixture chain, the fixture scrub-and-schema walk, the proptest seeds and the mutation-gate verdict. None of it creates or changes a web-spa or cli surface in layout-templates.md. The receipt and verdict formats are harness-owned test formats, not product output (per scope item 2). The amendment sidecar layout-templates-amendments.md does not exist, so the amendment history is empty.
