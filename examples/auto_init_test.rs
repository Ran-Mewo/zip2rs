use zip2rs::{ZipFile, ZipParameters, CompressionLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Auto-initialization Test ===");
    println!("This test does NOT call zip2rs::init() explicitly.");
    println!("The library should initialize automatically when ZipFile::new() is called.");
    
    // Create a zip file WITHOUT calling zip2rs::init() first
    // This should trigger automatic initialization
    let mut zip = ZipFile::new("auto_init_test.zip")?;
    
    // Add some test data
    let data = b"This file was created without calling init() explicitly!";
    let params = ZipParameters::new()
        .with_compression_level(CompressionLevel::Maximum);
    
    zip.add_data("test.txt", data, &params)?;
    
    println!("✓ Successfully created zip file with automatic initialization!");
    println!("✓ Added test data to the zip file");
    
    // Verify the file was created
    let entry_count = zip.entry_count()?;
    println!("✓ Zip file contains {} entries", entry_count);
    
    if entry_count > 0 {
        let entry = zip.get_entry_by_index(0)?;
        println!("✓ First entry: {}", entry.name()?);
    }
    
    println!("\n=== Auto-initialization test completed successfully! ===");
    
    Ok(())
}
