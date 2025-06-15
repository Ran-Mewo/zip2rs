# zip2rs

A complete slop comprehensive Rust API for zip file operations that leverages the advanced capabilities of the [Zip4j](https://github.com/srikanth-lingala/zip4j) Java library through a GraalVM-generated native interface.

## Features

- **Complete ZIP Operations**: Create, read, modify, and extract ZIP archives
- **In-Memory Operations**: Extract files to memory and add data from byte arrays
- **Advanced Encryption**: Support for Standard ZIP and AES encryption (128/256-bit)
- **Flexible Compression**: Multiple compression levels and methods
- **Streaming Support**: Handle large files efficiently
- **Progress Monitoring**: Track long-running operations
- **Split Archives**: Create and merge split ZIP files
- **Comprehensive Metadata**: Access detailed entry information
- **Iterator Support**: Iterate over entries with Rust iterators
- **Safe API**: Memory-safe operations with comprehensive error handling
- **Cross-platform**: Works on Windows, macOS, and Linux
- **High performance**: Leverages the mature and optimized Zip4j library

## Architecture

This project consists of two main components:

1. **zip4j-abi**: A Java project that creates GraalVM Native Image bindings to Zip4j
2. **zip2rs**: The Rust crate that provides safe, idiomatic bindings to the native library

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Rust Code     │───▶│  Native Library  │───▶│   Zip4j Java    │
│    (zip2rs)     │    │   (zip4j-abi)    │    │    Library      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Quick Start

```rust
use zip2rs::{ZipFile, ZipParameters, CompressionLevel, EncryptionMethod, AesKeyStrength};

// Initialize the library (call once at startup)
zip2rs::init()?;

// Create a new ZIP file
let mut zip = ZipFile::new("example.zip")?;

// Add a file with default settings
zip.add_file("document.txt")?;

// Add data from memory with custom parameters
let data = b"Hello, World!";
let params = ZipParameters::new()
    .with_compression_level(CompressionLevel::Maximum)
    .with_encryption_method(EncryptionMethod::Aes256)
    .with_aes_key_strength(AesKeyStrength::Aes256)
    .with_password("secret123");

zip.add_data("greeting.txt", data, &params)?;

// List all entries
for entry_result in zip.entries()? {
    let entry = entry_result?;
    println!("Entry: {} ({} bytes)", entry.name()?, entry.size()?);
    println!("  Compressed: {} bytes", entry.compressed_size()?);
    println!("  Directory: {}", entry.is_directory()?);
    println!("  Encrypted: {}", entry.is_encrypted()?);
}

// Extract files
zip.extract_all("output_directory")?;
zip.extract_file("document.txt", "output_directory")?;

// Remove files
zip.remove_file("greeting.txt")?;

// Cleanup when done
zip2rs::cleanup()?;
```

## API Overview

### Core Types

- **`ZipFile`**: Represents a zip archive with methods for adding, removing, and extracting files
- **`ZipEntry`**: Represents an individual file or directory within a zip archive
- **`ZipParameters`**: Configuration for compression and encryption when adding files
- **`ZipError`**: Comprehensive error type for all operations

### ZipFile Operations

```rust
// Creation and opening
let mut zip = ZipFile::new("archive.zip")?;
let mut zip = ZipFile::with_password("encrypted.zip", "password")?;

// Adding content
zip.add_file("file.txt")?;
zip.add_directory("folder")?;
zip.add_file_with_params("file.txt", &params)?;

// Add data from memory
let data = b"File content";
zip.add_data("memory_file.txt", data, &ZipParameters::new())?;
zip.add_data("encrypted_file.txt", data, &params)?;

// Removing content
zip.remove_file("unwanted.txt")?;
zip.remove_entry(&entry)?;

// Extracting content
zip.extract_all("output")?;
zip.extract_file("specific.txt", "output")?;
zip.extract_entry(&entry, "output")?;

// Extract to memory
let entry = zip.get_entry_by_name("file.txt")?;
let data = zip.extract_data(&entry)?;

// Querying
let count = zip.entry_count()?;
let entry = zip.get_entry_by_name("file.txt")?;
let entry = zip.get_entry_by_index(0)?;
let is_encrypted = zip.is_encrypted()?;
let is_valid = zip.is_valid()?;
```

### ZipEntry Metadata

```rust
let entry = zip.get_entry_by_name("file.txt")?;

// Basic properties
let name = entry.name()?;
let size = entry.size()?;
let compressed_size = entry.compressed_size()?;

// Type and security
let is_dir = entry.is_directory()?;
let is_encrypted = entry.is_encrypted()?;

// Advanced metadata
let crc32 = entry.crc32()?;
let modified_time = entry.last_modified_time()?;
let compression_method = entry.compression_method()?;
let encryption_method = entry.encryption_method()?;
let compression_ratio = entry.compression_ratio()?;
```

### ZipParameters Configuration

```rust
use zip2rs::{ZipParameters, CompressionLevel, CompressionMethod, EncryptionMethod, AesKeyStrength};

// Basic parameters
let params = ZipParameters::new()
    .with_compression_level(CompressionLevel::Maximum)
    .with_compression_method(CompressionMethod::Deflate);

// AES 256-bit encryption
let aes_params = ZipParameters::new()
    .with_aes256_encryption("password123");

// AES 128-bit encryption
let aes128_params = ZipParameters::new()
    .with_aes128_encryption("password123");

// Standard ZIP encryption
let standard_params = ZipParameters::new()
    .with_standard_encryption("password123");

// Custom AES configuration
let custom_params = ZipParameters::new()
    .with_encryption_method(EncryptionMethod::Aes256)
    .with_aes_key_strength(AesKeyStrength::Aes256)
    .with_password("custom_password");
```

## Examples

See the `examples/` directory for comprehensive usage examples:

- **`minimal_test.rs`**: Basic library initialization test
- **`simple_test.rs`**: Simple functionality test with basic operations
- **`basic_usage.rs`**: Complete example showing all major features
- **`comprehensive_demo.rs`**: Full demonstration of all capabilities including encryption, compression, and memory operations

Run examples with:
```bash
# Test basic initialization
cargo run --example minimal_test

# Run simple functionality test
cargo run --example simple_test

# Complete usage example
cargo run --example basic_usage

# Full comprehensive demo
cargo run --example comprehensive_demo
```

## Error Handling

The crate uses a comprehensive error type that covers all possible failure modes:

```rust
use zip2rs::{ZipError, Result};

match zip.add_file("nonexistent.txt") {
    Ok(()) => println!("File added successfully"),
    Err(ZipError::FileNotFound) => println!("File not found"),
    Err(ZipError::ZipException(msg)) => println!("Zip error: {}", msg),
    Err(ZipError::IoError(msg)) => println!("I/O error: {}", msg),
    Err(ZipError::InvalidParameter(msg)) => println!("Invalid parameter: {}", msg),
    Err(ZipError::PermissionDenied) => println!("Permission denied"),
    Err(ZipError::EntryNotFound) => println!("Entry not found in archive"),
    Err(e) => println!("Other error: {}", e),
}
```

### Available Error Types

- `InvalidHandle` - Invalid handle provided
- `FileNotFound` - File not found
- `ZipException(String)` - Zip-specific error with message
- `IoError(String)` - I/O error with message
- `InvalidParameter(String)` - Invalid parameter with details
- `OutOfMemory` - Out of memory
- `EntryNotFound` - Entry not found in archive
- `BufferTooSmall` - Buffer too small for operation
- `OperationCancelled` - Operation was cancelled
- `UnsupportedOperation` - Unsupported operation
- `PermissionDenied` - Permission denied
- `DiskFull` - Disk full
- `StringConversion(String)` - String conversion error

## Available Types and Enums

### Compression Levels
- `CompressionLevel::None` - No compression
- `CompressionLevel::Fastest` - Fastest compression (lowest ratio)
- `CompressionLevel::Normal` - Balanced speed and compression
- `CompressionLevel::Maximum` - Maximum compression (slowest)

### Compression Methods
- `CompressionMethod::Store` - Store without compression
- `CompressionMethod::Deflate` - Standard deflate compression

### Encryption Methods
- `EncryptionMethod::None` - No encryption
- `EncryptionMethod::Standard` - Standard ZIP encryption
- `EncryptionMethod::Aes128` - AES 128-bit encryption
- `EncryptionMethod::Aes256` - AES 256-bit encryption

### AES Key Strengths
- `AesKeyStrength::Aes128` - 128-bit key
- `AesKeyStrength::Aes192` - 192-bit key
- `AesKeyStrength::Aes256` - 256-bit key

## Initialization and Cleanup

**Important**: You must call `zip2rs::init()` once before using any other functions, and `zip2rs::cleanup()` when done:

```rust
use zip2rs::{init, cleanup, ZipFile};

// Initialize at application startup
init()?;

// Use the library
let mut zip = ZipFile::new("archive.zip")?;
// ... perform operations ...

// Cleanup at application shutdown
cleanup()?;
```

## Performance

This crate leverages the mature and highly optimized Zip4j library, providing excellent performance for zip operations. The GraalVM Native Image compilation eliminates JVM startup overhead while maintaining the performance benefits of the underlying Java implementation.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
