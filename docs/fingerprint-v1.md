<!-- © 2026 aiaiaiai · aiaiaiai.org -->

# Fingerprint algorithm v1

`v1` defines the canonical renderer-neutral fingerprint. It is intentionally simple enough to reimplement without this Rust codebase.

## Input

The input is a complete Git object digest, not an abbreviated repository-relative object name.

Supported canonical inputs:

- SHA-1: exactly 40 hexadecimal characters / 20 bytes.
- SHA-256: exactly 64 hexadecimal characters / 32 bytes.

Hexadecimal input is case-insensitive. Canonical textual output is lowercase. The digest is not hashed again.

## Canonical model

The model is an 8×8 row-major grid of 64 cells. Every cell is a palette index in `0..=3`. Palette indices are semantic values; colors belong to renderers and are not part of v1.

There is no symmetry transform, randomness, floating-point arithmetic, machine-dependent state, repository state, locale, clock, or renderer input.

## Mapping

Let `D` be the canonical digest bytes.

For each row-major cell index `i` from `0` through `63`:

1. `bit_offset = i × 2`.
2. `byte = D[floor(bit_offset / 8)]`.
3. `shift = 6 - (bit_offset mod 8)`.
4. `cell[i] = (byte >> shift) & 0b11`.

This consumes two digest bits per cell, most-significant pair first within each byte. Both supported digest forms contain at least the 128 bits required by the v1 model; v1 therefore consumes the first 16 digest bytes and ignores the remaining bytes.

## Compatibility

For a given full digest and `FingerprintVersion::V1`, these 64 palette indices are frozen protocol semantics. Renderers must not reinterpret them.

Any future change to grid dimensions, bit consumption order, palette-index derivation, or canonical model semantics requires a new fingerprint algorithm version.
