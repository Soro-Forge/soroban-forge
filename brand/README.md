# Brand assets

Vector sources for the Soro-Forge mark. The published PNGs are exported from
these files, so these are the originals to edit.

| File | Size | Where it is used |
| --- | --- | --- |
| `logo.svg` | 512x512 | Organization avatar, exported as PNG |
| `banner.svg` | 1280x640 | Repository social preview, exported as PNG |

## The mark

An anvil in steel, carrying three bars of glowing metal that read as lines of
code. The middle line is indented, so the workpiece reads as nested code rather
than three flat bars. The two four-pointed sparks are the Stellar visual
language.

The mark carries the three things this organization is: the forge, the code,
and Stellar.

## Palette

| Role | Hex |
| --- | --- |
| Background | `#0F172A` |
| Steel | `#E2E8F0` |
| Muted text | `#94A3B8` |
| Hot metal | `#F59E0B` |
| White hot | `#FDE68A` |

## Regenerating the PNGs

Any SVG rasterizer will do. With ImageMagick:

```sh
magick -background none -density 384 brand/logo.svg -resize 512x512 logo.png
magick -background none -density 192 brand/banner.svg -resize 1280x640 banner.png
```

The banner uses a system sans-serif stack rather than a bundled font, so the
wordmark shifts slightly between machines. Check the export before publishing
it.
