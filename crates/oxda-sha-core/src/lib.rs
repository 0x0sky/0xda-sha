// © 2026 aiaiaiai · aiaiaiai.org

//! Deterministic domain core for `0xda-sha`.

#![forbid(unsafe_code)]

use core::fmt;
use core::str::FromStr;

/// Supported canonical Git digest algorithms.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DigestAlgorithm {
    /// 160-bit SHA-1 object identifier.
    Sha1,
    /// 256-bit SHA-256 object identifier.
    Sha256,
}

impl DigestAlgorithm {
    /// Returns the canonical digest length in bytes.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        match self {
            Self::Sha1 => 20,
            Self::Sha256 => 32,
        }
    }
}

/// A validated full Git object digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Digest {
    algorithm: DigestAlgorithm,
    bytes: [u8; 32],
}

impl Digest {
    /// Returns the digest algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> DigestAlgorithm { self.algorithm }

    /// Returns the canonical digest bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] { &self.bytes[..self.algorithm.byte_len()] }
}

impl FromStr for Digest {
    type Err = DigestParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let algorithm = match value.len() {
            40 => DigestAlgorithm::Sha1,
            64 => DigestAlgorithm::Sha256,
            actual => return Err(DigestParseError::InvalidLength { actual }),
        };
        let mut bytes = [0_u8; 32];
        for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
            let high = decode_hex(pair[0]).ok_or(DigestParseError::InvalidHex { index: index * 2 })?;
            let low = decode_hex(pair[1]).ok_or(DigestParseError::InvalidHex { index: index * 2 + 1 })?;
            bytes[index] = (high << 4) | low;
        }
        Ok(Self { algorithm, bytes })
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.as_bytes() { write!(formatter, "{byte:02x}")?; }
        Ok(())
    }
}

/// Failure to parse a canonical full digest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DigestParseError {
    /// Input length is neither 40 nor 64 hexadecimal characters.
    InvalidLength { /// Actual input length in bytes.
        actual: usize },
    /// Input contains a non-hexadecimal byte.
    InvalidHex { /// Zero-based byte position of the invalid character.
        index: usize },
}

impl fmt::Display for DigestParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { actual } => write!(formatter, "invalid digest length: {actual}"),
            Self::InvalidHex { index } => write!(formatter, "invalid hexadecimal character at {index}"),
        }
    }
}
impl std::error::Error for DigestParseError {}

/// Released fingerprint algorithm versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FingerprintVersion { /// First canonical fingerprint mapping.
    V1 }

/// Renderer-neutral 8×8 fingerprint model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fingerprint { cells: [u8; 64] }

impl Fingerprint {
    /// Derives a canonical fingerprint.
    #[must_use]
    pub fn derive(digest: &Digest, version: FingerprintVersion) -> Self {
        match version { FingerprintVersion::V1 => derive_v1(digest) }
    }
    /// Returns row-major palette indices for the canonical grid.
    #[must_use]
    pub const fn cells(&self) -> &[u8; 64] { &self.cells }
}

fn derive_v1(digest: &Digest) -> Fingerprint {
    let source = digest.as_bytes();
    let mut cells = [0_u8; 64];
    for (cell_index, cell) in cells.iter_mut().enumerate() {
        let bit_offset = cell_index * 2;
        let byte = source[bit_offset / 8];
        let shift = 6 - (bit_offset % 8);
        *cell = (byte >> shift) & 0b11;
    }
    Fingerprint { cells }
}

const fn decode_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_normalizes_sha1() {
        let digest: Digest = "0123456789ABCDEF0123456789ABCDEF01234567".parse().unwrap();
        assert_eq!(digest.algorithm(), DigestAlgorithm::Sha1);
        assert_eq!(digest.to_string(), "0123456789abcdef0123456789abcdef01234567");
    }

    #[test]
    fn parses_sha256() {
        let digest: Digest = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f".parse().unwrap();
        assert_eq!(digest.algorithm(), DigestAlgorithm::Sha256);
    }

    #[test]
    fn rejects_short_git_identifiers() {
        assert_eq!("31ca016".parse::<Digest>().unwrap_err(), DigestParseError::InvalidLength { actual: 7 });
    }

    #[test]
    fn v1_matches_golden_vector() {
        let digest: Digest = "0123456789abcdef0123456789abcdef01234567".parse().unwrap();
        let fingerprint = Fingerprint::derive(&digest, FingerprintVersion::V1);
        assert_eq!(fingerprint.cells(), &[0,0,0,1,0,2,0,3,1,0,1,1,1,2,1,3,2,0,2,1,2,2,2,3,3,0,3,1,3,2,3,3,0,0,0,1,0,2,0,3,1,0,1,1,1,2,1,3,2,0,2,1,2,2,2,3,3,0,3,1,3,2,3,3]);
    }
}
