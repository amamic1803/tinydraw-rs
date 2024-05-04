//! A module containing the error enum for errors that can occur while using this library

/// An enum that represents errors that can occur while using this library
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    /// The file already exists
    FileExists,
    /// The invalid opacity value
    InvalidOpacity,
    /// The invalid size of the image
    InvalidSize,
    /// The invalid type
    InvalidType,
    /// The index is out of bounds
    IndexOutOfBounds,
    /// The given color is wrong
    WrongColor,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileExists => write!(f, "Error: File already exists!"),
            Error::InvalidOpacity => write!(f, "Error: Invalid opacity value!"),
            Error::InvalidSize => write!(f, "Error: The size of the image is invalid!"),
            Error::InvalidType => write!(f, "Error: The unsupported type!"),
            Error::IndexOutOfBounds => write!(f, "Error: Index out of bounds!"),
            Error::WrongColor => write!(f, "Error: Wrong color!"),
        }
    }
}
