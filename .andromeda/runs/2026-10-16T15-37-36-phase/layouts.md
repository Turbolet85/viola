# layouts extract

## No domain coverage
The chunk removes only test and harness infrastructure (the browser leg, Node/npm, GUI harness leftovers, the reserved obs web half), and its scope rules out any change to a CLI verb's output. So no output structure or component under layout-templates §Surface: cli is created or modified, and layout-templates §Surface: web-spa is only the 0.2.0 deferral stub, with no layout content to extract.
