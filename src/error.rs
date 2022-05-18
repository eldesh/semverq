use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid format as semver: {input}")]
    InvalidSemver {
        input: String,
        source: semver::Error,
    },
    #[error("invalid format as version requirement: {input}")]
    InvalidVersionReq {
        input: String,
        source: semver::Error,
    },
    #[error("{version} does not match that requirement ({req}).")]
    NotMatchReq {
        version: semver::Version,
        req: semver::VersionReq,
    },
}
