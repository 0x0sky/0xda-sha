// © 2026 aiaiaiai · aiaiaiai.org

//! Deterministic SVG projection boundary for `0xda-sha`.

#![forbid(unsafe_code)]

use core::fmt::Write as _;
use oxda_sha_core::Fingerprint;

const GRID_SIDE: usize = 8;
const CELL_SIZE: usize = 16;
const CANVAS_SIZE: usize = GRID_SIDE * CELL_SIZE;
const PALETTE: [&str; 4] = ["#0b0f14", "#334155", "#94a3b8", "#f8fafc"];

/// Released SVG renderer versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SvgRendererVersion {
    /// First byte-stable SVG projection contract.
    V1,
}

/// Renders a canonical fingerprint as deterministic UTF-8 SVG.
#[must_use]
pub fn render(fingerprint: &Fingerprint, version: SvgRendererVersion) -> String {
    match version {
        SvgRendererVersion::V1 => render_v1(fingerprint),
    }
}

fn render_v1(fingerprint: &Fingerprint) -> String {
    let mut svg = String::with_capacity(4_096);
    write!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {CANVAS_SIZE} {CANVAS_SIZE}" width="{CANVAS_SIZE}" height="{CANVAS_SIZE}" shape-rendering="crispEdges">"#
    )
    .expect("writing to String is infallible");

    for (index, palette_index) in fingerprint.cells().iter().copied().enumerate() {
        let x = (index % GRID_SIDE) * CELL_SIZE;
        let y = (index / GRID_SIDE) * CELL_SIZE;
        let fill = PALETTE[usize::from(palette_index)];
        write!(
            svg,
            r#"<rect x="{x}" y="{y}" width="{CELL_SIZE}" height="{CELL_SIZE}" fill="{fill}"/>"#
        )
        .expect("writing to String is infallible");
    }

    svg.push_str("</svg>\n");
    svg
}

#[cfg(test)]
mod tests {
    use oxda_sha_core::{Digest, FingerprintVersion};

    use super::*;

    const GOLDEN_SVG: &str = include_str!("../../../fixtures/svg-v1/sha1-0123456789abcdef.svg");

    fn fixture() -> Fingerprint {
        let digest: Digest = "0123456789abcdef0123456789abcdef01234567"
            .parse()
            .expect("fixture digest must be valid");
        Fingerprint::derive(&digest, FingerprintVersion::V1)
    }

    #[test]
    fn v1_matches_golden_bytes() {
        assert_eq!(render(&fixture(), SvgRendererVersion::V1), GOLDEN_SVG);
    }

    #[test]
    fn render_is_byte_stable() {
        let fingerprint = fixture();
        assert_eq!(
            render(&fingerprint, SvgRendererVersion::V1),
            render(&fingerprint, SvgRendererVersion::V1)
        );
    }

    #[test]
    fn v1_has_canonical_structure() {
        let svg = render(&fixture(), SvgRendererVersion::V1);
        assert!(svg.starts_with(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" width="128" height="128" shape-rendering="crispEdges">"#
        ));
        assert!(svg.ends_with("</svg>\n"));
        assert_eq!(svg.matches("<rect ").count(), 64);
        assert!(!svg.contains("<script"));
        assert!(!svg.contains("href="));
        assert!(!svg.contains("id="));
    }

    #[test]
    fn v1_uses_only_declared_palette() {
        let svg = render(&fixture(), SvgRendererVersion::V1);
        for fill in svg
            .split("fill=\"")
            .skip(1)
            .filter_map(|value| value.split('"').next())
        {
            assert!(PALETTE.contains(&fill));
        }
    }
}
