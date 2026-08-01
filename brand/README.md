# Brand assets

The mark is a pair of braces with a single finding between them — the shape of every check in this toolkit, which reads structure and reports what sits inside it.

| File | Use |
| --- | --- |
| `logo.svg` | Organization and repository avatar. Square, safe down to 32px. |
| `banner.svg` | Repository social preview, 1280×640. |

## Colours

| Role | Hex |
| --- | --- |
| Background | `#0F172A` |
| Mark | `#F8FAFC` |
| Finding | `#F59E0B` |
| Muted text | `#94A3B8` |

## Regenerating the PNGs

The vector sources are authoritative. Export with any SVG renderer, for example:

```sh
npx sharp-cli --input brand/logo.svg --output logo.png resize 512 512
```

Render at 512×512 for avatars and 1280×640 for the social preview.
