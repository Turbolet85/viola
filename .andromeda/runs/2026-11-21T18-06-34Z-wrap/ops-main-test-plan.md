@row_set n=1 row=decision/fixture-drift-byte-compare cell=except seen=1 detector=none
value: A hook whose payload names the send tool is left out of the dialog tier, so a side sender's hooks shift no position; every other dialog-tier hook is still compared by position, event and bytes.
basis: fct-tkwnjker, fct-lwwhvdyk
because: The drift check's dialog-tier reader now leaves out send-tool hooks; the exception says what the check still covers.
@row_set n=2 row=decision/fake-agent-matches-recordings cell=except seen=1 detector=none
value: A hook whose payload names the send tool is left out of the dialog tier, so a side sender's hooks shift no position; every other dialog-tier hook is still compared by position, event and bytes.
basis: fct-tkwnjker, fct-lwwhvdyk
because: The row states the same comparison whole; without the exception it is false, and the exception says what is still covered.
@row_set n=3 row=decision/fixture-hygiene-walk cell=rule seen=3 detector=none
value: A run-time directory walk checks every fixture for home, drive-letter, user-folder paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file by its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list.
basis: fct-wn36g3ew
because: The walk flags only home, drive-letter and user-folder paths, so «absolute paths» is false: the committed cross-session arrival's runtime-dir path passes it.
@row_add n=1 row=decision/fake-agent-cross-session-by-uds-address area=test-data-fixtures fact=F2 detector=none
name: Fake agent cross-session by uds address
rule: The fake agent's cross-session sender resolves its target by the uds socket address alone, delivers only to its sibling and refuses an address not of the uds shape; its receiver reads its inbox only under the accept settings.
why: The real CLI's send tool resolves no local session by a bare id, as its 2.1.328 bundle reads; confirmed only by the mended live start.
guard: sender_refuses_an_address_not_of_the_uds_shape
basis: fct-zjeenbpz, fct-vgvx3eex
because: The fake agent gained a sender and receiver held to the real CLI's address form; the cause is said as measured, and the guard is one the report names.
