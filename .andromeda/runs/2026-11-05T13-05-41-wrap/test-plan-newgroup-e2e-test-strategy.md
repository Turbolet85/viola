@verdict n=1 ref_file=fanout-test-plan--cascade-g5.md ref_n=26 v=contradiction fact=fct-vgds3skp
quote: its rule taking the `cfg_attr` case rows (`tests/hook_fail_open.rs`'s two functions)
why: the standing count holds hook_fail_open.rs's two cfg_attr gates (which name target_os = "macos", per the impostor rows) as named GATES rows, while the proposal makes any gate naming target_os = "macos" a fault even when a GATES row names it; the anchor fct-liulfbun's "faults reads both ways" names only the unnamed-gate and stale-row classes, which the proposal extends
