<!-- © 2026 aiaiaiai · aiaiaiai.org -->

# Architecture

## Decision

`0xda-sha` uses a **functional core / imperative shell** architecture with library-first ownership.

The deterministic fingerprint contract belongs to `oxda-sha-core`. Git/process/filesystem/terminal behavior belongs to adapters outside the core. Renderers consume the canonical model; they do not invent fingerprint semantics.

## Dependency direction

```text
oxda-sha-cli ───────▶ oxda-sha-svg ───────▶ oxda-sha-core
      │                                        ▲
      └────────────────────────────────────────┘
```

Dependencies point inward. The core never imports a renderer or CLI concern.

## Crate boundaries

### `oxda-sha-core`

Owns digest parsing/normalization, algorithm versioning, canonical fingerprint model, deterministic derivation, and model invariants.

Forbidden: filesystem, environment access, process execution, terminal detection, Git invocation, networking, renderer-specific serialization, and WASM-specific types.

### `oxda-sha-svg`

Owns deterministic projection from the canonical model into SVG bytes. It may depend on the core, but must not influence the model.

### `oxda-sha-cli`

Owns argument parsing, Git revision resolution through an explicit process adapter, renderer selection, output, diagnostics, and exit-code mapping.

Repository-relative state crosses into the application only through a `GitResolver` port. Complete canonical digests bypass Git entirely; repository-relative expressions are resolved to a validated full digest before crossing into `oxda-sha-core`.

## Determinism contract

For a released fingerprint algorithm version, the same canonical full digest must produce the same canonical fingerprint independently of machine, time, locale, repository, renderer, or invocation path.

Renderer byte stability is a separate versioned concern. A visual renderer may evolve without silently changing the canonical fingerprint algorithm.

## Versioned boundaries

Fingerprint algorithm `v1` and SVG renderer `v1` are released as independent deterministic contracts. The CLI composes those contracts but does not own or reinterpret them.

Adding a new renderer, application adapter, or repository integration must therefore preserve the existing core fingerprint contract unless a new fingerprint algorithm version is introduced explicitly. Likewise, changing fingerprint semantics must not silently reuse an existing released version.
