@fit n=1 row=decision/screen-fixture-signature-rows-only sha=e765d4d95470 v=fits type=2
@fit n=2 row=decision/verify-signature-only sha=a0609ad34c3d v=fits type=2
@fit n=3 row=decision/verify-screen-read-against-literals sha=e3bf1e9b8f48 v=fits type=2
@fit n=4 row=decision/verify-stamp-first-screen-closed-words sha=07a298be60f4 v=fits type=1
@fit n=5 row=decision/live-rig-seed-from-bundle-logic sha=20b6a5f1ac4f v=fits type=1
@fit n=6 row=decision/live-rig-git-root-as-files sha=69ee023eb7ba v=not_master type=4
why: How the rig builds its child's git root as plain files is one mechanism whose basis states no reason or named threat for it, so the rig's code holds it.
