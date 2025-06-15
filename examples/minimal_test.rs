use zip4j_rust::{init, Result};

fn main() -> Result<()> {
    println!("Minimal test - attempting to initialize library...");
    
    match init() {
        Ok(()) => {
            println!("✓ Library initialized successfully!");
            
            match zip4j_rust::cleanup() {
                Ok(()) => println!("✓ Library cleaned up successfully!"),
                Err(e) => println!("✗ Cleanup failed: {}", e),
            }
        }
        Err(e) => {
            println!("✗ Initialization failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}
