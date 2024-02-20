pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    Reqwest(reqwest::Error),
    Cloudflare,
    Api,
    Invalid,
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Reqwest(e) => e.to_string(),
            Self::Cloudflare => "Failed to properly retrieve dns record".to_string(),
            Self::Api => "All public ip API failed".to_string(),
            Self::Invalid => "Invalid parsing for DNS record name".to_string(),
            Self::IO(e) => e.to_string(),
            Self::ParseInt(e) => e.to_string(),
        })
    }
}

impl std::error::Error for Error {}
