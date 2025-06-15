use zip4j_rust::{ZipFile, ZipParameters, CompressionLevel, EncryptionMethod, ZipError};
use std::fs;
use std::io::Write;

fn main() -> Result<(), ZipError> {
    // Create some test files
    setup_test_files()?;
    
    println!("=== Zip4j-Rust Basic Usage Example ===\n");
    
    // Example 1: Create a simple zip file
    println!("1. Creating a simple zip file...");
    create_simple_zip()?;
    
    // Example 2: Create a zip with compression and encryption
    println!("2. Creating an encrypted zip file...");
    create_encrypted_zip()?;
    
    // Example 3: Read and list zip contents
    println!("3. Reading zip file contents...");
    read_zip_contents()?;
    
    // Example 4: Extract files
    println!("4. Extracting files...");
    extract_files()?;
    
    // Example 5: Modify existing zip
    println!("5. Modifying existing zip...");
    modify_zip()?;
    
    // Cleanup
    cleanup_test_files();
    
    println!("\n=== All examples completed successfully! ===");
    Ok(())
}

fn setup_test_files() -> Result<(), ZipError> {
    // Create test directory
    fs::create_dir_all("test_files").map_err(|e| ZipError::io_error(e.to_string()))?;
    
    // Create some test files
    let mut file1 = fs::File::create("test_files/document.txt")
        .map_err(|e| ZipError::io_error(e.to_string()))?;
    file1.write_all(b"This is a test document.\nIt contains some sample text.")
        .map_err(|e| ZipError::io_error(e.to_string()))?;
    
    let mut file2 = fs::File::create("test_files/data.csv")
        .map_err(|e| ZipError::io_error(e.to_string()))?;
    file2.write_all(b"Name,Age,City\nJohn,30,New York\nJane,25,Los Angeles")
        .map_err(|e| ZipError::io_error(e.to_string()))?;
    
    // Create a subdirectory
    fs::create_dir_all("test_files/subdir").map_err(|e| ZipError::io_error(e.to_string()))?;
    let mut file3 = fs::File::create("test_files/subdir/readme.md")
        .map_err(|e| ZipError::io_error(e.to_string()))?;
    file3.write_all(b"# Test Project\n\nThis is a test project for zip4j-rust.")
        .map_err(|e| ZipError::io_error(e.to_string()))?;
    
    Ok(())
}

fn create_simple_zip() -> Result<(), ZipError> {
    let mut zip = ZipFile::new("simple_archive.zip")?;
    
    // Add individual files
    zip.add_file("test_files/document.txt")?;
    zip.add_file("test_files/data.csv")?;
    
    // Add entire directory
    zip.add_directory("test_files/subdir")?;
    
    println!("   ✓ Created simple_archive.zip with {} entries", zip.entry_count()?);
    Ok(())
}

fn create_encrypted_zip() -> Result<(), ZipError> {
    let mut zip = ZipFile::with_password("encrypted_archive.zip", "secret123")?;
    
    // Create parameters for maximum compression and AES-256 encryption
    let params = ZipParameters::new()
        .compression_level(CompressionLevel::Maximum)
        .encryption_method(EncryptionMethod::Aes256)
        .password("secret123");
    
    // Add files with encryption
    zip.add_file_with_params("test_files/document.txt", &params)?;
    zip.add_file_with_params("test_files/data.csv", &params)?;
    
    println!("   ✓ Created encrypted_archive.zip with AES-256 encryption");
    Ok(())
}

fn read_zip_contents() -> Result<(), ZipError> {
    let zip = ZipFile::new("simple_archive.zip")?;
    
    println!("   Archive: {}", zip.path());
    println!("   Valid: {}", zip.is_valid()?);
    println!("   Encrypted: {}", zip.is_encrypted()?);
    println!("   Entries: {}", zip.entry_count()?);
    println!();
    
    // List all entries
    for (i, entry) in zip.entries()?.enumerate() {
        let name = entry.name()?;
        let size = entry.size()?;
        let compressed_size = entry.compressed_size()?;
        let is_dir = entry.is_directory()?;
        let is_encrypted = entry.is_encrypted()?;
        let compression_ratio = entry.compression_ratio()?;
        
        println!("   Entry {}: {}", i + 1, name);
        println!("     Type: {}", if is_dir { "Directory" } else { "File" });
        println!("     Size: {} bytes", size);
        println!("     Compressed: {} bytes", compressed_size);
        println!("     Compression: {:.1}%", compression_ratio);
        println!("     Encrypted: {}", is_encrypted);
        println!("     CRC32: 0x{:08X}", entry.crc32()?);
        println!();
    }
    
    Ok(())
}

fn extract_files() -> Result<(), ZipError> {
    let zip = ZipFile::new("simple_archive.zip")?;
    
    // Create extraction directory
    fs::create_dir_all("extracted").map_err(|e| ZipError::io_error(e.to_string()))?;
    
    // Extract all files
    zip.extract_all("extracted")?;
    println!("   ✓ Extracted all files to 'extracted' directory");
    
    // Extract specific file
    zip.extract_file("document.txt", "extracted_specific")?;
    println!("   ✓ Extracted specific file to 'extracted_specific' directory");
    
    Ok(())
}

fn modify_zip() -> Result<(), ZipError> {
    let mut zip = ZipFile::new("simple_archive.zip")?;
    
    println!("   Original entry count: {}", zip.entry_count()?);
    
    // Remove a file
    zip.remove_file("data.csv")?;
    println!("   ✓ Removed data.csv");
    
    // Add a new file
    let mut new_file = fs::File::create("test_files/new_file.txt")
        .map_err(|e| ZipError::io_error(e.to_string()))?;
    new_file.write_all(b"This is a newly added file.")
        .map_err(|e| ZipError::io_error(e.to_string()))?;
    drop(new_file);
    
    zip.add_file("test_files/new_file.txt")?;
    println!("   ✓ Added new_file.txt");
    
    println!("   Final entry count: {}", zip.entry_count()?);
    
    Ok(())
}

fn cleanup_test_files() {
    let _ = fs::remove_dir_all("test_files");
    let _ = fs::remove_dir_all("extracted");
    let _ = fs::remove_dir_all("extracted_specific");
    let _ = fs::remove_file("simple_archive.zip");
    let _ = fs::remove_file("encrypted_archive.zip");
}
