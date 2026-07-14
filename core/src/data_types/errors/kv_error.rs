use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum KvError {
    /// Thrown when the system cannot locate or read the file from disk
    #[error("I/O error: Failed to access the dictionary source file: {0}")]
    Io(#[from] std::io::Error),

    /// Thrown when a file row contains missing data or text formatting issues
    #[error(
        "File format error on line {line}: Each row must contain exactly one key and one value (Found {found_tokens} parts)"
    )]
    InvalidRow { line: usize, found_tokens: usize },

    /// Thrown when a key string fails to deserialize into the target type K
    #[error(
        "Data parse error on line {line}: Failed to convert key text '{text}' into the requested type: {details}"
    )]
    InvalidKey {
        line: usize,
        text: String,
        details: String,
    },

    /// Thrown when a value string fails to deserialize into the target type V
    #[error(
        "Data parse error on line {line}: Failed to convert value text '{text}' into the requested type: {details}"
    )]
    InvalidValue {
        line: usize,
        text: String,
        details: String,
    },

    /// Thrown when the file stream finishes prematurely or contains no valid rows
    #[error("Data seeding failure: The provided data stream contains no valid records")]
    NoData,
}
