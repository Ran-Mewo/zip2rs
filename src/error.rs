use std::ffi::NulError;
use std::fmt;
use std::str::Utf8Error;

/// Result type for zip4j operations
pub type Result<T> = std::result::Result<T, ZipError>;

/// Errors that can occur during zip operations
#[derive(Debug)]
pub enum ZipError {
    /// Invalid handle provided
    InvalidHandle,
    /// File not found
    FileNotFound,
    /// Zip-specific error
    ZipException(String),
    /// I/O error
    IoError(String),
    /// Invalid parameter
    InvalidParameter(String),
    /// Out of memory
    OutOfMemory,
    /// Entry not found in archive
    EntryNotFound,
    /// Buffer too small for operation
    BufferTooSmall,
    /// Operation was cancelled
    OperationCancelled,
    /// Unsupported operation
    UnsupportedOperation,
    /// Null pointer error
    NullPointer,
    /// Permission denied
    PermissionDenied,
    /// Disk full
    DiskFull,
    /// Unknown error
    Unknown(String),
    /// String conversion error
    StringConversion(String),
}

impl fmt::Display for ZipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZipError::InvalidHandle => write!(f, "Invalid handle"),
            ZipError::FileNotFound => write!(f, "File not found"),
            ZipError::ZipException(msg) => write!(f, "Zip error: {}", msg),
            ZipError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ZipError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            ZipError::OutOfMemory => write!(f, "Out of memory"),
            ZipError::EntryNotFound => write!(f, "Entry not found in archive"),
            ZipError::BufferTooSmall => write!(f, "Buffer too small"),
            ZipError::OperationCancelled => write!(f, "Operation was cancelled"),
            ZipError::UnsupportedOperation => write!(f, "Unsupported operation"),
            ZipError::NullPointer => write!(f, "Null pointer error"),
            ZipError::PermissionDenied => write!(f, "Permission denied"),
            ZipError::DiskFull => write!(f, "Disk full"),
            ZipError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
            ZipError::StringConversion(msg) => write!(f, "String conversion error: {}", msg),
        }
    }
}

impl std::error::Error for ZipError {}

impl From<NulError> for ZipError {
    fn from(err: NulError) -> Self {
        ZipError::StringConversion(err.to_string())
    }
}

impl From<Utf8Error> for ZipError {
    fn from(err: Utf8Error) -> Self {
        ZipError::StringConversion(err.to_string())
    }
}

impl From<std::io::Error> for ZipError {
    fn from(err: std::io::Error) -> Self {
        ZipError::IoError(err.to_string())
    }
}

impl ZipError {
    /// Convert an error code from the native library to a ZipError
    pub fn from_code(code: i32) -> Self {
        match code {
            -1 => ZipError::InvalidHandle,
            -2 => ZipError::FileNotFound,
            -3 => ZipError::ZipException("Zip operation failed".to_string()),
            -4 => ZipError::IoError("I/O operation failed".to_string()),
            -5 => ZipError::InvalidParameter("Invalid parameter provided".to_string()),
            -6 => ZipError::OutOfMemory,
            -7 => ZipError::EntryNotFound,
            -8 => ZipError::BufferTooSmall,
            -9 => ZipError::OperationCancelled,
            -10 => ZipError::UnsupportedOperation,
            -11 => ZipError::NullPointer,
            -12 => ZipError::PermissionDenied,
            -13 => ZipError::DiskFull,
            _ => ZipError::Unknown(format!("Error code: {}", code)),
        }
    }
}
