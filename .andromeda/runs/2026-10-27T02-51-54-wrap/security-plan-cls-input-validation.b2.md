# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.20 · no · the change moved only pin_bytes in pin.rs, whose callers are pin_exe and pin_and_plugin and never pin_companions, check_present's product code in strict.rs is unchanged, and the companions still write through replace_private_shared, so the claim holds on the changed files
input-validation#1.21 · no · the session adds no upload, webhook or database surface, its changes being a no-clobber placement helper, a test-only Windows module and a test-only receipt field
input-validation#2.0 · no · the report's Untouched section names the MAX_FRAME constant and its readers as none read by this change, so the readers list still holds on the changed files
