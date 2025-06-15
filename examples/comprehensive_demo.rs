use std::fs;
use zip4j_rust::{
    init, cleanup, ZipFile, ZipParameters, CompressionLevel, EncryptionMethod,
    Result, ZipError,
};

fn main() -> Result<()> {
    // Initialize the library
    init()?;
    
    println!("=== Zip4j Rust Comprehensive Demo ===\n");
    
    // Demo 1: Basic file operations
    demo_basic_operations()?;
    
    // Demo 2: In-memory operations
    demo_memory_operations()?;
    
    // Demo 3: Encryption and compression
    demo_encryption_compression()?;
    
    // Demo 4: Archive inspection
    demo_archive_inspection()?;
    
    // Demo 5: Advanced operations
    demo_advanced_operations()?;
    
    // Cleanup
    cleanup()?;
    println!("Demo completed successfully!");
    
    Ok(())
}

fn demo_basic_operations() -> Result<()> {
    println!("1. Basic File Operations");
    println!("------------------------");
    
    // Create some test files
    fs::write("test1.txt", "Hello from test1!")?;
    fs::write("test2.txt", "Hello from test2!")?;
    fs::create_dir_all("test_dir")?;
    fs::write("test_dir/nested.txt", "Nested file content")?;
    
    // Create a new ZIP file
    let mut zip = ZipFile::new("basic_demo.zip")?;
    
    // Add individual files
    zip.add_file("test1.txt")?;
    zip.add_file("test2.txt")?;
    
    // Add a directory
    zip.add_directory("test_dir")?;
    
    println!("✓ Created ZIP with {} entries", zip.entry_count()?);
    
    // Extract all files
    fs::create_dir_all("extracted_basic")?;
    zip.extract_all("extracted_basic")?;
    println!("✓ Extracted all files to 'extracted_basic'");
    
    // Extract specific file
    zip.extract_file("test1.txt", "extracted_basic")?;
    println!("✓ Extracted specific file");
    
    println!();
    Ok(())
}

fn demo_memory_operations() -> Result<()> {
    println!("2. In-Memory Operations");
    println!("-----------------------");
    
    let mut zip = ZipFile::new("memory_demo.zip")?;
    
    // Add data from memory
    let text_data = b"This is text data stored in memory";
    let binary_data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x21]; // "Hello!"
    
    let params = ZipParameters::new()
        .with_compression_level(CompressionLevel::Maximum);
    
    zip.add_data("memory_text.txt", text_data, &params)?;
    zip.add_data("memory_binary.bin", &binary_data, &params)?;
    
    println!("✓ Added data from memory to ZIP");
    
    // Extract data to memory
    let entry = zip.get_entry_by_name("memory_text.txt")?;
    let extracted_data = zip.extract_data(&entry)?;
    
    println!("✓ Extracted data to memory: {} bytes", extracted_data.len());
    println!("  Content: {}", String::from_utf8_lossy(&extracted_data));
    
    // Verify data integrity
    assert_eq!(extracted_data, text_data);
    println!("✓ Data integrity verified");
    
    println!();
    Ok(())
}

fn demo_encryption_compression() -> Result<()> {
    println!("3. Encryption and Compression");
    println!("-----------------------------");
    
    let mut zip = ZipFile::with_password("encrypted_demo.zip", "archive_password")?;
    
    // Add file with AES 256 encryption
    let aes256_params = ZipParameters::new()
        .with_compression_level(CompressionLevel::Maximum)
        .with_encryption_method(EncryptionMethod::Aes256)
        .with_password("super_secret");

    let secret_data = b"This is highly confidential information!";
    zip.add_data("secret.txt", secret_data, &aes256_params)?;

    // Add file with standard encryption
    let standard_params = ZipParameters::new()
        .with_encryption_method(EncryptionMethod::Standard)
        .with_password("basic_password");

    zip.add_data("basic_secret.txt", b"Basic encryption", &standard_params)?;
    
    println!("✓ Created encrypted ZIP with different encryption methods");
    println!("  Archive is encrypted: {}", zip.is_encrypted()?);
    
    println!();
    Ok(())
}

fn demo_archive_inspection() -> Result<()> {
    println!("4. Archive Inspection");
    println!("---------------------");
    
    let zip = ZipFile::new("memory_demo.zip")?;
    
    println!("Archive properties:");
    println!("  File path: {}", zip.file_path()?);
    println!("  Is valid: {}", zip.is_valid()?);
    println!("  Is encrypted: {}", zip.is_encrypted()?);
    println!("  Is split archive: {}", zip.is_split_archive()?);
    println!("  Entry count: {}", zip.entry_count()?);
    
    // Set and get comment
    let mut zip_mut = ZipFile::new("commented_demo.zip")?;
    zip_mut.set_comment("This is a demo archive created by zip4j-rust")?;
    println!("  Comment: {}", zip_mut.comment()?);
    
    println!("\nEntry details:");
    for (i, entry_result) in zip.entries()?.enumerate() {
        let entry = entry_result?;
        println!("  Entry {}: {}", i + 1, entry.name()?);
        println!("    Size: {} bytes", entry.size()?);
        println!("    Compressed: {} bytes", entry.compressed_size()?);
        println!("    Compression ratio: {:.1}%", entry.compression_ratio()?);
        println!("    Is directory: {}", entry.is_directory()?);
        println!("    Is encrypted: {}", entry.is_encrypted()?);
        println!("    Compression method: {:?}", entry.compression_method()?);
        println!("    Encryption method: {:?}", entry.encryption_method()?);
        println!("    CRC32: 0x{:08X}", entry.crc32()?);
    }
    
    println!();
    Ok(())
}

fn demo_advanced_operations() -> Result<()> {
    println!("5. Advanced Operations");
    println!("----------------------");
    
    let mut zip = ZipFile::new("advanced_demo.zip")?;
    
    // Add some files
    zip.add_data("file1.txt", b"Content 1", &ZipParameters::new())?;
    zip.add_data("file2.txt", b"Content 2", &ZipParameters::new())?;
    zip.add_data("file3.txt", b"Content 3", &ZipParameters::new())?;
    
    println!("✓ Created ZIP with {} entries", zip.entry_count()?);
    
    // Remove a file
    zip.remove_file("file2.txt")?;
    println!("✓ Removed file2.txt, now {} entries", zip.entry_count()?);
    
    // Remove by entry
    let entry = zip.get_entry_by_name("file3.txt")?;
    zip.remove_entry(&entry)?;
    println!("✓ Removed file3.txt by entry, now {} entries", zip.entry_count()?);
    
    // Demonstrate error handling
    match zip.get_entry_by_name("nonexistent.txt") {
        Ok(_) => println!("This shouldn't happen!"),
        Err(ZipError::EntryNotFound) => println!("✓ Correctly handled missing entry"),
        Err(e) => println!("Unexpected error: {}", e),
    }
    
    println!();
    Ok(())
}


