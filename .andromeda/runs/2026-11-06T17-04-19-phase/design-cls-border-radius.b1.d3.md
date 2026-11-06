@answer n=1 entity=border-radius#1.1 v=no
why: radius-sm is a web-spa token (readback box, field cells, focus ring) for the 0.2.0 frontend; the task's not_needed item 1 puts Border Radius out of this chunk, and no CLI human string or test pin reads a radius value.

@answer n=2 entity=border-radius#1.2 v=no
why: radius-md is a web-spa token (session strips, transfer markers, access and error strips) for the 0.2.0 frontend; the task's not_needed item 1 puts Border Radius out of this chunk, and no CLI human string or test pin reads a radius value.

@answer n=3 entity=border-radius#1.3 v=no
why: radius-lg is a web-spa token (racks, ATIS header, tape panel) for the 0.2.0 frontend; the task's not_needed item 1 puts Border Radius out of this chunk, and no CLI human string or test pin reads a radius value.

@answer n=4 entity=border-radius#1.4 v=no
why: radius-full is a web-spa token whose banned 9999px value is a frontend design rule; the task's not_needed item 1 puts Border Radius out of this chunk, and the chunk's Rust code and CLI tests read no radius value.
