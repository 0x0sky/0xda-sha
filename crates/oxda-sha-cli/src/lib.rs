// © 2026 aiaiaiai · aiaiaiai.org

//! Application boundary for the `0xda-sha` command-line interface.

#![forbid(unsafe_code)]

use std::fmt;
use std::process::Command as ProcessCommand;

use oxda_sha_core::{Digest, DigestParseError, Fingerprint, FingerprintVersion};
use oxda_sha_svg::{SvgRendererVersion, render};

const MAX_REVISION_BYTES: usize = 256;

/// Exit codes owned by the CLI contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ExitCode {
    /// Command completed successfully.
    Success = 0,
    /// Arguments do not match the CLI grammar.
    Usage = 2,
    /// A full digest was provided but is invalid.
    InvalidDigest = 3,
    /// Git could not resolve the requested revision.
    GitResolution = 4,
}

/// Parsed CLI command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliCommand {
    /// Resolve an object expression to its canonical full digest.
    Resolve {
        /// Full digest or repository-relative Git expression.
        input: String,
    },
    /// Render an object expression as deterministic SVG v1.
    Svg {
        /// Full digest or repository-relative Git expression.
        input: String,
    },
}

/// Git revision-resolution boundary.
pub trait GitResolver {
    /// Resolves a repository-relative expression to a full validated digest.
    ///
    /// # Errors
    ///
    /// Returns an error when the revision is invalid, Git cannot be started,
    /// resolution fails, or Git returns a non-canonical digest.
    fn resolve(&self, revision: &str) -> Result<Digest, GitResolveError>;
}

/// Concrete Git adapter backed by the `git` executable.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemGitResolver;

impl GitResolver for SystemGitResolver {
    fn resolve(&self, revision: &str) -> Result<Digest, GitResolveError> {
        validate_revision(revision)?;

        let revision_spec = format!("{revision}^{{object}}");
        let output = ProcessCommand::new("git")
            .arg("rev-parse")
            .arg("--verify")
            .arg("--end-of-options")
            .arg(revision_spec)
            .output()
            .map_err(|error| GitResolveError::Spawn {
                message: error.to_string(),
            })?;

        if !output.status.success() {
            return Err(GitResolveError::Rejected {
                status: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            });
        }

        let resolved = String::from_utf8_lossy(&output.stdout);
        resolved
            .trim()
            .parse()
            .map_err(GitResolveError::InvalidResolvedDigest)
    }
}

/// Git adapter failure.
#[derive(Debug, Eq, PartialEq)]
pub enum GitResolveError {
    /// Revision is empty, oversized, or contains a control character.
    InvalidRevision,
    /// The Git process could not be started.
    Spawn {
        /// Platform error message.
        message: String,
    },
    /// Git rejected or could not resolve the revision.
    Rejected {
        /// Process exit code when one is available.
        status: Option<i32>,
        /// Trimmed Git diagnostic text.
        stderr: String,
    },
    /// Git returned output that is not a canonical full digest.
    InvalidResolvedDigest(DigestParseError),
}

impl fmt::Display for GitResolveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRevision => formatter.write_str("invalid Git revision"),
            Self::Spawn { message } => write!(formatter, "failed to start Git: {message}"),
            Self::Rejected { status, stderr } => match (status, stderr.is_empty()) {
                (Some(code), false) => {
                    write!(formatter, "Git resolution failed ({code}): {stderr}")
                }
                (Some(code), true) => write!(formatter, "Git resolution failed ({code})"),
                (None, false) => write!(formatter, "Git resolution failed: {stderr}"),
                (None, true) => formatter.write_str("Git resolution failed"),
            },
            Self::InvalidResolvedDigest(error) => {
                write!(formatter, "Git returned an invalid full digest: {error}")
            }
        }
    }
}

impl std::error::Error for GitResolveError {}

/// CLI-level failure.
#[derive(Debug, Eq, PartialEq)]
pub enum CliError {
    /// Arguments do not match the command grammar.
    Usage,
    /// A full digest-shaped input is malformed.
    InvalidDigest(DigestParseError),
    /// A repository-relative input could not be resolved.
    Git(GitResolveError),
}

impl CliError {
    /// Returns the stable process exit code for this error.
    #[must_use]
    pub const fn exit_code(&self) -> ExitCode {
        match self {
            Self::Usage => ExitCode::Usage,
            Self::InvalidDigest(_) => ExitCode::InvalidDigest,
            Self::Git(_) => ExitCode::GitResolution,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => formatter.write_str(usage()),
            Self::InvalidDigest(error) => write!(formatter, "invalid digest: {error}"),
            Self::Git(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for CliError {}

/// Parses CLI arguments excluding the executable name.
///
/// # Errors
///
/// Returns [`CliError::Usage`] for unsupported or incomplete commands.
pub fn parse(args: &[String]) -> Result<CliCommand, CliError> {
    match args {
        [command, input] if command == "resolve" => Ok(CliCommand::Resolve {
            input: input.clone(),
        }),
        [command, input] if command == "svg" => Ok(CliCommand::Svg {
            input: input.clone(),
        }),
        _ => Err(CliError::Usage),
    }
}

/// Executes a parsed command through an injected Git resolver.
///
/// # Errors
///
/// Returns a typed CLI error when digest validation or Git resolution fails.
pub fn execute(command: &CliCommand, git: &impl GitResolver) -> Result<String, CliError> {
    match command {
        CliCommand::Resolve { input } => {
            let digest = resolve_input(input, git)?;
            Ok(format!("{digest}\n"))
        }
        CliCommand::Svg { input } => {
            let digest = resolve_input(input, git)?;
            let fingerprint = Fingerprint::derive(&digest, FingerprintVersion::V1);
            Ok(render(&fingerprint, SvgRendererVersion::V1))
        }
    }
}

/// Stable human-readable command grammar.
#[must_use]
pub const fn usage() -> &'static str {
    "usage: 0xda-sha <resolve|svg> <full-digest|git-revision>"
}

fn resolve_input(input: &str, git: &impl GitResolver) -> Result<Digest, CliError> {
    if matches!(input.len(), 40 | 64) {
        return input.parse().map_err(CliError::InvalidDigest);
    }

    git.resolve(input).map_err(CliError::Git)
}

fn validate_revision(revision: &str) -> Result<(), GitResolveError> {
    if revision.is_empty()
        || revision.len() > MAX_REVISION_BYTES
        || revision.chars().any(char::is_control)
    {
        return Err(GitResolveError::InvalidRevision);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    const FULL_SHA1: &str = "0123456789abcdef0123456789abcdef01234567";

    struct FakeGit {
        calls: Cell<usize>,
    }

    impl FakeGit {
        fn new() -> Self {
            Self {
                calls: Cell::new(0),
            }
        }
    }

    impl GitResolver for FakeGit {
        fn resolve(&self, _revision: &str) -> Result<Digest, GitResolveError> {
            self.calls.set(self.calls.get() + 1);
            Ok(FULL_SHA1.parse().expect("fixture digest must be valid"))
        }
    }

    #[test]
    fn parses_declared_commands_only() {
        assert_eq!(
            parse(&["resolve".into(), "HEAD".into()]),
            Ok(CliCommand::Resolve {
                input: "HEAD".into()
            })
        );
        assert_eq!(parse(&["inspect".into()]), Err(CliError::Usage));
    }

    #[test]
    fn full_digest_bypasses_git() {
        let git = FakeGit::new();
        let output = execute(
            &CliCommand::Resolve {
                input: FULL_SHA1.into(),
            },
            &git,
        )
        .expect("full digest must resolve");

        assert_eq!(output, format!("{FULL_SHA1}\n"));
        assert_eq!(git.calls.get(), 0);
    }

    #[test]
    fn repository_relative_input_uses_git_port() {
        let git = FakeGit::new();
        let output = execute(
            &CliCommand::Resolve {
                input: "HEAD".into(),
            },
            &git,
        )
        .expect("fake Git resolution must succeed");

        assert_eq!(output, format!("{FULL_SHA1}\n"));
        assert_eq!(git.calls.get(), 1);
    }

    #[test]
    fn svg_command_composes_core_and_renderer() {
        let git = FakeGit::new();
        let output = execute(
            &CliCommand::Svg {
                input: "HEAD".into(),
            },
            &git,
        )
        .expect("fake Git resolution must succeed");

        assert!(output.starts_with("<svg "));
        assert!(output.ends_with("</svg>\n"));
        assert_eq!(git.calls.get(), 1);
    }

    #[test]
    fn rejects_control_characters_before_process_boundary() {
        assert_eq!(
            validate_revision("HEAD\n--help"),
            Err(GitResolveError::InvalidRevision)
        );
    }
}
