# Zip4j-Rust

A complete slop comprehensive Rust API for zip file operations that leverages the advanced capabilities of the [Zip4j](https://github.com/srikanth-lingala/zip4j) Java library through a GraalVM-generated native interface

## Features

- **ZipEntry-focused API**: Work with individual entries in zip archives with full metadata access
- **Complete zip operations**: Create, read, modify, and extract zip files
- **Advanced compression**: Multiple compression levels from none to maximum
- **Strong encryption**: Support for standard ZIP encryption and AES-128/256
- **Password protection**: Full support for password-protected archives
- **Memory safety**: All operations are memory-safe with proper resource management
- **Cross-platform**: Works on Windows, macOS, and Linux
- **High performance**: Leverages the mature and optimized Zip4j library

## Architecture

This project consists of two main components:

1. **zip4j-abi**: A Java project that creates GraalVM Native Image bindings to Zip4j
2. **zip4j-rust**: The Rust crate that provides safe, idiomatic bindings to the native library

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Rust Code     │───▶│  Native Library  │───▶│   Zip4j Java    │
│  (zip4j-rust)   │    │   (zip4j-abi)    │    │    Library      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.70+ 
- GraalVM with Native Image support
- Java 24+

### Building

1. First, build the native library:
```bash
cd zip4j-abi
./gradlew nativeCompile
```

2. Then build the Rust crate:
```bash
cargo build
```

### Basic Usage

```rust
use zip4j_rust::{ZipFile, ZipParameters, CompressionLevel, EncryptionMethod};

// Create a new zip file
let mut zip = ZipFile::new("archive.zip")?;

// Add files with default settings
zip.add_file("document.txt")?;
zip.add_directory("my_folder")?;

// Add files with custom compression and encryption
let params = ZipParameters::new()
    .compression_level(CompressionLevel::Maximum)
    .encryption_method(EncryptionMethod::Aes256)
    .password("secret123");

zip.add_file_with_params("sensitive.txt", &params)?;

// List all entries
for entry in zip.entries()? {
    println!("Entry: {} ({} bytes)", entry.name()?, entry.size()?);
    println!("  Compressed: {} bytes", entry.compressed_size()?);
    println!("  Directory: {}", entry.is_directory()?);
    println!("  Encrypted: {}", entry.is_encrypted()?);
}

// Extract files
zip.extract_all("output_directory")?;
zip.extract_file("specific_file.txt", "output_directory")?;

// Remove files
zip.remove_file("obsolete.txt")?;
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
let zip = ZipFile::new("archive.zip")?;
let zip = ZipFile::with_password("encrypted.zip", "password")?;

// Adding content
zip.add_file("file.txt")?;
zip.add_directory("folder")?;
zip.add_file_with_params("file.txt", &params)?;

// Removing content  
zip.remove_file("unwanted.txt")?;
zip.remove_entry(&entry)?;

// Extracting content
zip.extract_all("output")?;
zip.extract_file("specific.txt", "output")?;
zip.extract_entry(&entry, "output")?;

// Querying
let count = zip.entry_count()?;
let entry = zip.get_entry_by_name("file.txt")?;
let entry = zip.get_entry_by_index(0)?;
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
let compression_ratio = entry.compression_ratio()?;
```

## Examples

See the `examples/` directory for comprehensive usage examples:

- `basic_usage.rs`: Complete example showing all major features

Run examples with:
```bash
cargo run --example basic_usage
```

## Error Handling

The crate uses a comprehensive error type that covers all possible failure modes:

```rust
use zip4j_rust::ZipError;

match zip.add_file("nonexistent.txt") {
    Ok(()) => println!("File added successfully"),
    Err(ZipError::FileNotFound { path }) => println!("File not found: {}", path),
    Err(ZipError::ZipException { message }) => println!("Zip error: {}", message),
    Err(ZipError::IoError { message }) => println!("I/O error: {}", message),
    Err(e) => println!("Other error: {}", e),
}
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
