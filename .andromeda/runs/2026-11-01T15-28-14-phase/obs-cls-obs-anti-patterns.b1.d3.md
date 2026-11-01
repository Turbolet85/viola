# Classified — obs · obs-anti-patterns.b1

## Answers
obs-anti-patterns#1.1 · yes · the recorded `human::refuse(` call-site counts per verb (run 7, verify 6, list 2, revive 1, plugin install 1, client 1) must record the refusal sites this chunk adds or moves: the wrapper start refusal in `run`, the client-side euid mismatch and connect-time socket-directory refusal through `client::refuse_untrusted`, and list's `ScanRefused::Instance` pair.
