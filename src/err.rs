pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    Reqwest(reqwest::Error),
    Cloudflare,
    API,
    Invalid,
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
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
            Self::API => "All public ip API failed".to_string(),
            Self::Invalid => "Invalid parsing for DNS record name".to_string(),
        })
    }
}
