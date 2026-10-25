# design-system · Spacing
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Spacing detail.

## Spacing

<!-- detail: values · spacing -->

| Token | Value | Usage |
|-------|-------|-------|
| space-micro | 4px | Gap between the readback box and its word cell, and between `→` and a name. Never cell padding: field-cell inline padding is `space-xs` (8px), which `--strip-cols` is sized for |
| space-xs | 8px | Field-cell inline padding in strips, the rack row gap (between strips, around transfer markers, and between the caption row and the first strip), the transfer-marker indent step |
| space-sm | 12px | Gap between ATIS header cells (the cells have no inline padding, so neighbours sit exactly 12px apart). Equals `--cock-offset`: the rack reserves this much end padding so a cocked strip never causes horizontal scroll. |
| space-md | 16px | Page inline margin, gap between rack separator label and the caption row |
| space-lg | 24px | Separation between racks, and between the bay and the tape |
| space-xl | 32px | Top page margin under the sticky ATIS header, and the strip height (`--strip-h`) |

<!-- /detail -->

<!-- detail: values · spacing -->

- `--strip-h: 32px`
- `--line-h: 20px` (tape line and transfer marker)
- `--band-w: 4px`
- `--rb-size: 16px` (the square sits centred in a 20px line)

<!-- /detail -->
