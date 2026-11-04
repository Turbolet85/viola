@answer n=1 entity=motion#1.1 v=no
why: the hover, focus and page-transition timings for expression 0.0-0.2 are web motion, and the task's not_needed item on Motion says the chunk adds no animation and the cli has no motion at any level.
@answer n=2 entity=motion#1.2 v=no
why: the 0.3-0.4 timings (150ms ease-out, 200ms fade) are web motion that the chunk's pty backend and cross-OS tests never act on, per the task's Motion not_needed item.
@answer n=3 entity=motion#1.3 v=no
why: the 0.5-0.6 timings (200ms ease-out, 300ms slide, stagger 50ms) are web motion; the chunk adds no animation and the cli has no motion, per the task's Motion not_needed item.
@answer n=4 entity=motion#1.4 v=no
why: the 0.7-0.8 timings (250ms spring, 400ms spring, parallax) are web motion; the chunk adds no animation and the cli has no motion, per the task's Motion not_needed item.
@answer n=5 entity=motion#1.5 v=no
why: the 0.9-1.0 timings (300ms spring, 500ms+ custom, full scroll-driven) are web motion; the chunk adds no animation and the cli has no motion, per the task's Motion not_needed item.
