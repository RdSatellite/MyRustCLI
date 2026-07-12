#[derive(Debug)]
pub enum CharacterError {
    Http(String),
    Io(std::io::Error),
}

impl std::fmt::Display for CharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterError::Http(msg) => write!(f, "Character HTTP error: {msg}"),
            CharacterError::Io(e) => write!(f, "Character I/O error: {e}"),
        }
    }
}

impl std::error::Error for CharacterError {}