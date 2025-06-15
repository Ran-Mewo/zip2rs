use zip2rs::{init, Result};

fn main() -> Result<()> {
    println!("Minimal test - attempting to initialize library...");
    
    match init() {
        Ok(()) => {
            println!("✓ Library initialized successfully!");
            
            match zip2rs::cleanup() {
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
