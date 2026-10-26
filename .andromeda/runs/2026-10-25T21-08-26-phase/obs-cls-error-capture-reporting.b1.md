# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the raw backtrace is panic hook design (src/panic_frames.rs, unreached) that the chunk relies on and does not change
error-capture-reporting#2.1 · no · the catch-site exits and worker containment are per-role panic exits the chunk relies on and does not change
error-capture-reporting#3.1 · no · the pre-init window (before init step 4) is panic hook scope the chunk relies on and does not change; its run start, squatted refusal and exit-1 line all run after init step 4
error-capture-reporting#4.1 · yes · the G2 clause binds: the single seam exemption stays unchanged and a TestHome::outside_scan() home escapes G2, G4 and the secret scan, so new test homes must stay under target/e2e-home
