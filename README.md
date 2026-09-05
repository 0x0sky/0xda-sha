# 0xda-sha

> Every state leaves a shape.

Deterministic visual fingerprints for Git commit digests.

`0xda-sha` is being built as a small Rust library-first system: a deterministic core owns fingerprint semantics, while renderers and Git/CLI integration stay at explicit boundaries.

Current status: architecture bootstrap. Fingerprint algorithm `v1` is intentionally not defined in this pull request.

## Workspace

- `oxda-sha-core` — canonical domain types and deterministic fingerprint semantics.
- `oxda-sha-svg` — deterministic SVG projection of the canonical model.
- `oxda-sha-cli` — process/Git/output shell around the core and renderers.

See [`docs/architecture.md`](docs/architecture.md) for dependency and ownership rules.

## Toolchain

The repository pins Rust `1.98.1`, the current stable toolchain selected for the bootstrap. CI treats formatting, Clippy, tests, and rustdoc warnings as correctness gates.

## Copyright

© 2026 aiaiaiai · aiaiaiai.org

Copyright and licensing are separate decisions. The repository license is intentionally still open; SPDX identifiers will be added only after that decision is made.
