# 0xda-sha

> Every state leaves a shape.

Deterministic visual fingerprints for Git object digests.

`0xda-sha` is a small Rust library-first system: a deterministic core owns fingerprint semantics, renderers own versioned projections, and repository/process integration stays behind explicit application boundaries.

Current status: fingerprint algorithm `v1`, SVG renderer `v1`, and the first Git-aware CLI surface are implemented.

## Workspace

- `oxda-sha-core` — canonical digest types, fingerprint algorithm versioning, and deterministic fingerprint semantics.
- `oxda-sha-svg` — deterministic SVG projection of the canonical model.
- `oxda-sha-cli` — application/process boundary for Git revision resolution, command parsing, output, diagnostics, and exit codes.

See [`docs/architecture.md`](docs/architecture.md) for dependency and ownership rules and [`docs/cli-v1.md`](docs/cli-v1.md) for the CLI contract.

## CLI

```text
0xda-sha resolve <full-digest|git-revision>
0xda-sha svg <full-digest|git-revision>
```

A complete 40- or 64-character digest is validated directly by the deterministic core and does not invoke Git. Other inputs are resolved through the explicit Git adapter before entering the core.

From a source checkout:

```sh
cargo run -p oxda-sha-cli -- resolve HEAD
cargo run -p oxda-sha-cli -- svg HEAD
```

## Toolchain

The repository pins Rust `1.98.1`. CI treats formatting, Clippy, tests, and rustdoc warnings as correctness gates and runs tests on Linux, macOS, and Windows.

## Copyright

© 2026 aiaiaiai · aiaiaiai.org

Copyright and licensing are separate decisions. The repository license is intentionally still open; SPDX identifiers will be added only after that decision is made.
