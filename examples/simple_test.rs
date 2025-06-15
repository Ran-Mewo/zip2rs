use zip2rs::{init, cleanup, ZipFile, ZipParameters, CompressionLevel, Result};

fn main() -> Result<()> {
    println!("Testing zip2rs implementation...");
    
    // Initialize the library
    init()?;
    println!("✓ Library initialized");
    
    // Test basic functionality
    test_basic_operations()?;
    
    // Test in-memory operations
    test_memory_operations()?;
    
    // Cleanup
    cleanup()?;
    println!("✓ Library cleaned up");
    
    println!("All tests passed!");
    Ok(())
}

fn test_basic_operations() -> Result<()> {
    println!("\nTesting basic operations...");

    // Create a new ZIP file
    let mut zip = ZipFile::new("test_basic.zip")?;
    println!("✓ Created ZIP file");

    // Check initial properties (empty ZIP)
    let count = zip.entry_count()?;
    println!("✓ Initial entry count: {} (empty ZIP)", count);

    let is_valid = zip.is_valid()?;
    println!("✓ Initial validity: {} (empty ZIP is invalid)", is_valid);

    let is_encrypted = zip.is_encrypted()?;
    println!("✓ Is encrypted: {}", is_encrypted);

    // Try to get comment from empty ZIP (should return empty string)
    let initial_comment = zip.comment()?;
    println!("✓ Initial comment: '{}' (empty for invalid ZIP)", initial_comment);

    // Try to set comment on empty ZIP (should fail gracefully)
    match zip.set_comment("This should fail") {
        Ok(()) => println!("! Unexpected: Comment set on empty ZIP"),
        Err(e) => println!("✓ Expected: Cannot set comment on empty ZIP: {}", e),
    }

    // Add some data to make the ZIP valid
    let test_data = b"Hello from basic test!";
    let params = ZipParameters::new();
    zip.add_data("test.txt", test_data, &params)?;
    println!("✓ Added test data");

    // Check properties after adding data
    let count_after = zip.entry_count()?;
    println!("✓ Entry count after adding data: {}", count_after);

    let is_valid_after = zip.is_valid()?;
    println!("✓ Is valid after adding data: {}", is_valid_after);

    // Now set a comment (should work on a valid ZIP)
    zip.set_comment("Test archive created by zip2rs")?;
    let comment = zip.comment()?;
    println!("✓ Comment set successfully: '{}'", comment);

    Ok(())
}

fn test_memory_operations() -> Result<()> {
    println!("\nTesting memory operations...");
    
    let mut zip = ZipFile::new("test_memory.zip")?;
    
    // Add data from memory
    let test_data = b"Hello, World! This is test data.";
    let params = ZipParameters::new()
        .with_compression_level(CompressionLevel::Maximum);
    
    zip.add_data("test_file.txt", test_data, &params)?;
    println!("✓ Added data from memory");
    
    // Verify entry was added
    let count = zip.entry_count()?;
    println!("✓ Entry count after adding: {}", count);
    
    // Get the entry
    let entry = zip.get_entry_by_name("test_file.txt")?;
    println!("✓ Retrieved entry: {}", entry.name()?);
    println!("  Size: {} bytes", entry.size()?);
    println!("  Compressed: {} bytes", entry.compressed_size()?);
    println!("  Is directory: {}", entry.is_directory()?);
    println!("  Is encrypted: {}", entry.is_encrypted()?);
    
    // Extract data to memory
    let extracted_data = zip.extract_data(&entry)?;
    println!("✓ Extracted data to memory: {} bytes", extracted_data.len());
    
    // Verify data integrity
    if extracted_data == test_data {
        println!("✓ Data integrity verified");
    } else {
        println!("✗ Data integrity check failed!");
        return Err(zip2rs::ZipError::Unknown("Data mismatch".to_string()));
    }
    
    Ok(())
}
