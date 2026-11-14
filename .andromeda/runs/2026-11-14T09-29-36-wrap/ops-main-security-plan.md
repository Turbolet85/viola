@row_add n=1 row=decision/hyprland-recorder-string-allowlist area=data-protection detector=none
name: Hyprland recorder keeps strings by a closed allowlist
rule: viola-hypr-tap keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag); every other string becomes the placeholder, a non-JSON reply stays only as ok, then the shared scrub runs and an unclean set is refused whole.
why: Hyprland 0.56.2 carries titles and user text under keys no deny-list foresees (lastwindowtitle, nested workspace.name, tags).
guard: record_replaces_every_string_off_the_allowlist; planted_hyprland_lastwindowtitle_is_rejected
basis: fct-w4fzcahg
because: The recorder's rule is the closed keep-list the P5 review folded (inputs#I3 item 1) inside the founder's own answer to the P4 recorder fork (inputs#I2 Q2), and the plan's expected amendment names this handle; no standing row states it.
