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

## Determinism contract

For a released fingerprint algorithm version, the same canonical full digest must produce the same canonical fingerprint independently of machine, time, locale, repository, renderer, or invocation path.

Renderer byte stability is a separate versioned concern. A visual renderer may evolve without silently changing the canonical fingerprint algorithm.

## Bootstrap boundary

This repository foundation deliberately contains no fingerprint algorithm. The first semantic implementation must arrive in a separate PR with an explicit `v1` specification and golden vectors. That separation prevents repository/tooling choices from accidentally becoming protocol semantics.
