<!-- © 2026 aiaiaiai · aiaiaiai.org -->

# SVG renderer v1

SVG renderer `v1` is a deterministic projection of the canonical 8×8 `Fingerprint` model. It does not derive, transform, mirror, randomize, or otherwise reinterpret fingerprint semantics.

## Canvas

- logical and intrinsic size: 128×128
- cell size: 16×16
- cell order: canonical row-major order
- geometry: integer-only
- `shape-rendering="crispEdges"`

## Palette

Renderer v1 maps canonical palette indices to these fixed sRGB colors:

| index | color |
| --- | --- |
| 0 | `#0b0f14` |
| 1 | `#334155` |
| 2 | `#94a3b8` |
| 3 | `#f8fafc` |

The palette is renderer semantics, not fingerprint semantics. Changing it requires a new SVG renderer version, not a new fingerprint algorithm version.

## Byte stability

For the same canonical `Fingerprint`, renderer v1 must produce identical UTF-8 bytes across machines, operating systems, locales, and invocations.

The document contains exactly one root `<svg>` element followed by 64 `<rect>` elements in row-major order and a final newline. It contains no timestamps, generated identifiers, metadata, scripts, stylesheets, external resources, floating-point values, or environment-derived content.

`fixtures/svg-v1/sha1-0123456789abcdef.svg` is the golden byte fixture for the corresponding fingerprint-v1 digest fixture.
