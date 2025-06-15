//! # zip2rs
//!
//! A comprehensive Rust wrapper for the zip4j Java library using GraalVM Native Image.
//!
//! This crate provides a safe, high-level Rust API for working with ZIP archives,
//! leveraging the powerful zip4j library through a native interface.
//!
//! ## Features
//!
//! - **Complete ZIP Operations**: Create, read, modify, and extract ZIP archives
//! - **In-Memory Operations**: Extract files to memory and add data from byte arrays
//! - **Advanced Encryption**: Support for Standard ZIP and AES encryption (128/256-bit)
//! - **Flexible Compression**: Multiple compression levels and methods
//! - **Streaming Support**: Handle large files efficiently
//! - **Progress Monitoring**: Track long-running operations
//! - **Split Archives**: Create and merge split ZIP files
//! - **Comprehensive Metadata**: Access detailed entry information
//! - **Iterator Support**: Iterate over entries with Rust iterators
//! - **Safe API**: Memory-safe operations with comprehensive error handling
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use zip2rs::{ZipFile, ZipParameters, CompressionLevel, EncryptionMethod};
//!
//! // Initialize the library (call once at startup)
//! zip2rs::init()?;
//!
//! // Create a new ZIP file
//! let mut zip = ZipFile::new("example.zip")?;
//!
//! // Add a file with default settings
//! zip.add_file("document.txt")?;
//!
//! // Add data from memory with custom parameters
//! let data = b"Hello, World!";
//! let params = ZipParameters::new()
//!     .with_compression_level(CompressionLevel::Maximum)
//!     .with_aes256_encryption("secret123");
//!
//! zip.add_data("greeting.txt", data, &params)?;
//!
//! // Extract a file to memory (no temporary files needed)
//! let entry = zip.get_entry_by_name("greeting.txt")?;
//! let extracted_data = zip.extract_data(&entry)?;
//! assert_eq!(extracted_data, data);
//!
//! // Extract all files to disk
//! zip.extract_all("output_directory")?;
//!
//! // Iterate over all entries
//! for entry_result in zip.entries()? {
//!     let entry = entry_result?;
//!     println!("Entry: {} ({} bytes)", entry.name()?, entry.size()?);
//! }
//!
//! # Ok::<(), zip2rs::ZipError>(())
//! ```
//!
//! ## Advanced Usage
//!
//! ```rust,no_run
//! use zip2rs::{ZipFile, ZipParameters, CompressionLevel, EncryptionMethod, AesKeyStrength};
//!
//! // Create an encrypted archive with custom settings
//! let mut zip = ZipFile::with_password("secure.zip", "archive_password")?;
//!
//! // Add files with different encryption methods
//! let aes_params = ZipParameters::new()
//!     .with_compression_level(CompressionLevel::Maximum)
//!     .with_encryption_method(EncryptionMethod::Aes256)
//!     .with_aes_key_strength(AesKeyStrength::Aes256)
//!     .with_password("file_password");
//!
//! zip.add_file_with_params("sensitive.doc", &aes_params)?;
//!
//! // Check archive properties
//! println!("Archive is encrypted: {}", zip.is_encrypted()?);
//! println!("Archive is valid: {}", zip.is_valid()?);
//! println!("Entry count: {}", zip.entry_count()?);
//!
//! # Ok::<(), zip2rs::ZipError>(())
//! ```

pub mod error;
pub mod ffi;
pub mod types;
pub mod zip_entry;
pub mod zip_file;

// Re-export main types for convenience
pub use error::{Result, ZipError};
pub use types::{
    AesKeyStrength, CompressionLevel, CompressionMethod, EncryptionMethod, ZipParameters,
};
pub use zip_entry::ZipEntry;
pub use zip_file::{ZipFile, ZipEntryIterator};

/// Initialize the zip4j library
/// 
/// This must be called once before using any other functions.
/// It's safe to call this multiple times.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// // Initialize at the start of your application
/// zip2rs::init()?;
/// 
/// // Now you can use the library
/// let zip = zip2rs::ZipFile::new("archive.zip")?;
/// # Ok::<(), zip2rs::ZipError>(())
/// ```
pub fn init() -> Result<()> {
    ffi::init()
}

/// Cleanup the zip4j library
/// 
/// This should be called when you're done using the library,
/// typically at application shutdown. It's safe to call this
/// multiple times.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// // At application shutdown
/// zip2rs::cleanup()?;
/// # Ok::<(), zip2rs::ZipError>(())
/// ```
pub fn cleanup() -> Result<()> {
    ffi::cleanup()
}
