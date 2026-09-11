<!-- © 2026 aiaiaiai · aiaiaiai.org -->

# CLI contract v1

The CLI is an imperative shell around deterministic `oxda-sha-core` and `oxda-sha-svg` libraries. Repository state is permitted only inside the Git adapter.

## Commands

```text
0xda-sha resolve <full-digest|git-revision>
0xda-sha svg <full-digest|git-revision>
```

`resolve` writes one lowercase canonical full digest followed by `LF`.

`svg` writes SVG renderer v1 output unchanged. That output already ends in `LF`.

## Input boundary

An input whose byte length is exactly 40 or 64 is treated as an explicit full digest and is validated by `oxda-sha-core`. It never invokes Git.

Every other input is repository-relative and must cross the `GitResolver` port. The system adapter executes Git directly, without a shell:

```text
git rev-parse --verify --end-of-options <revision>^{object}
```

The Git result is accepted only after `oxda-sha-core` validates it as a complete SHA-1 or SHA-256 digest. Short object names, `HEAD`, tags, branches, and other Git expressions therefore never enter the deterministic core.

## Revision safety

Repository-relative inputs must be non-empty, at most 256 bytes, and contain no control characters. `--end-of-options` prevents a user revision from becoming a Git command option. No shell interpolation is used.

## Exit codes

- `0`: success
- `2`: CLI usage error
- `3`: malformed explicit full digest
- `4`: Git revision validation, process, resolution, or resolved-digest error

Stdout is reserved for successful command output. Diagnostics are written to stderr.

## Ownership

`oxda-sha-core` owns digest validation and fingerprint semantics.

`oxda-sha-svg` owns SVG projection semantics.

`oxda-sha-cli` owns argument grammar, process exit codes, Git resolution, stdout, and stderr.
