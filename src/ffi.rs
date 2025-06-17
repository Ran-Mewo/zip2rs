use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_longlong};
use std::sync::Once;
use crate::error::{Result, ZipError};

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Import embedded module when bundled feature is enabled
#[cfg(feature = "bundled")]
use crate::embedded;

// Re-export commonly used types for convenience
pub use graal_isolatethread_t as GraalIsolateThread;
pub use graal_isolate_t as GraalIsolate;

/// Global isolate and thread for GraalVM operations
static mut GRAAL_ISOLATE: *mut GraalIsolate = std::ptr::null_mut();
static mut GRAAL_THREAD: *mut GraalIsolateThread = std::ptr::null_mut();
static INIT_ONCE: Once = Once::new();

/// Initialize the zip4j library with proper GraalVM isolate setup
pub fn init() -> Result<()> {
    let mut init_result = Ok(());

    INIT_ONCE.call_once(|| {
        // Initialize embedded library if bundled feature is enabled
        #[cfg(feature = "bundled")]
        {
            if let Err(e) = embedded::initialize() {
                init_result = Err(ZipError::Unknown(format!("Failed to initialize embedded library: {}", e)));
                return;
            }
        }
        unsafe {
            let mut isolate: *mut GraalIsolate = std::ptr::null_mut();
            let mut thread: *mut GraalIsolateThread = std::ptr::null_mut();

            // Create a new GraalVM isolate
            let create_result = graal_create_isolate(
                std::ptr::null_mut(), // No special parameters
                &mut isolate,
                &mut thread
            );

            if create_result != 0 {
                init_result = Err(ZipError::Unknown(format!("Failed to create GraalVM isolate: {}", create_result)));
                return;
            }

            // Store the isolate and thread globally
            GRAAL_ISOLATE = isolate;
            GRAAL_THREAD = thread;

            // Now initialize the zip4j library
            let zip_init_result = zip4j_init(thread);
            if zip_init_result != 0 {
                init_result = Err(ZipError::from_code(zip_init_result));
                return;
            }
        }
    });

    init_result
}

/// Cleanup the zip4j library and tear down the GraalVM isolate
pub fn cleanup() -> Result<()> {
    unsafe {
        if !GRAAL_THREAD.is_null() {
            // Cleanup zip4j first
            let cleanup_result = zip4j_cleanup(GRAAL_THREAD);
            if cleanup_result != 0 {
                // Continue with teardown even if cleanup failed
                eprintln!("Warning: zip4j cleanup failed with code: {}", cleanup_result);
            }

            // Tear down the GraalVM isolate
            let teardown_result = graal_tear_down_isolate(GRAAL_THREAD);
            if teardown_result != 0 {
                return Err(ZipError::Unknown(format!("Failed to tear down GraalVM isolate: {}", teardown_result)));
            }

            // Reset global pointers
            GRAAL_ISOLATE = std::ptr::null_mut();
            GRAAL_THREAD = std::ptr::null_mut();
        }
    }
    Ok(())
}

/// Get the current GraalVM isolate thread
///
/// # Safety
///
/// This function should only be called after successful initialization.
/// Returns null if not initialized.
pub(crate) fn get_thread() -> *mut GraalIsolateThread {
    unsafe { GRAAL_THREAD }
}

/// Check if the library is initialized
pub fn is_initialized() -> bool {
    unsafe { !GRAAL_THREAD.is_null() }
}

/// Ensure the library is initialized, returning an error if not
pub(crate) fn ensure_initialized() -> Result<()> {
    if !is_initialized() {
        return Err(ZipError::Unknown("Library not initialized. Call zip2rs::init() first.".to_string()));
    }
    Ok(())
}

/// Constants from the C ABI
pub mod constants {
    use std::os::raw::c_int;

    // Error codes
    pub const SUCCESS: c_int = 0;
    pub const ERROR_INVALID_HANDLE: c_int = -1;
    pub const ERROR_FILE_NOT_FOUND: c_int = -2;
    pub const ERROR_ZIP_EXCEPTION: c_int = -3;
    pub const ERROR_IO_EXCEPTION: c_int = -4;
    pub const ERROR_INVALID_PARAMETER: c_int = -5;
    pub const ERROR_OUT_OF_MEMORY: c_int = -6;
    pub const ERROR_ENTRY_NOT_FOUND: c_int = -7;
    pub const ERROR_BUFFER_TOO_SMALL: c_int = -8;
    pub const ERROR_OPERATION_CANCELLED: c_int = -9;
    pub const ERROR_UNSUPPORTED_OPERATION: c_int = -10;
    pub const ERROR_NULL_POINTER: c_int = -11;
    pub const ERROR_PERMISSION_DENIED: c_int = -12;
    pub const ERROR_DISK_FULL: c_int = -13;

    // Compression methods
    pub const COMPRESSION_STORE: c_int = 0;
    pub const COMPRESSION_DEFLATE: c_int = 8;

    // Compression levels
    pub const COMPRESSION_LEVEL_NONE: c_int = 0;
    pub const COMPRESSION_LEVEL_FASTEST: c_int = 1;
    pub const COMPRESSION_LEVEL_NORMAL: c_int = 6;
    pub const COMPRESSION_LEVEL_MAXIMUM: c_int = 9;

    // Encryption methods
    pub const ENCRYPTION_NONE: c_int = 0;
    pub const ENCRYPTION_STANDARD: c_int = 1;
    pub const ENCRYPTION_AES_128: c_int = 2;
    pub const ENCRYPTION_AES_256: c_int = 3;

    // AES key strengths
    pub const AES_KEY_STRENGTH_128: c_int = 1;
    pub const AES_KEY_STRENGTH_192: c_int = 2;
    pub const AES_KEY_STRENGTH_256: c_int = 3;
}

/// Helper functions for FFI operations
pub mod helpers {
    use super::*;

    /// Convert a Rust string to a C string
    pub fn to_c_string(s: &str) -> Result<CString> {
        CString::new(s).map_err(ZipError::from)
    }

    /// Check if a return code indicates success
    pub fn is_success(code: c_int) -> bool {
        code == constants::SUCCESS
    }

    /// Check if a return code indicates an error
    pub fn is_error(code: c_int) -> bool {
        code < 0
    }

    /// Read a string from a C buffer with length
    pub fn read_string_from_buffer(buffer: &[c_char], length: c_int) -> Result<String> {
        if length <= 0 {
            return Ok(String::new());
        }

        let slice = &buffer[..length as usize];
        let cstr = unsafe { CStr::from_ptr(slice.as_ptr()) };
        Ok(cstr.to_str()?.to_string())
    }

    /// Get the last error message for a handle
    pub fn get_last_error(handle: c_longlong) -> Result<String> {
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let mut error_length: c_int = 0;

        let result = unsafe {
            zip4j_get_last_error(
                get_thread(),
                handle,
                buffer.as_mut_ptr() as *mut c_char,
                BUFFER_SIZE as c_int,
                &mut error_length
            )
        };

        if is_error(result) {
            return Ok("Failed to get error message".to_string());
        }

        read_string_from_buffer_u8(&buffer, error_length)
    }

    /// Read a string from a u8 buffer with length (for cross-platform compatibility)
    pub fn read_string_from_buffer_u8(buffer: &[u8], length: c_int) -> Result<String> {
        if length <= 0 {
            return Ok(String::new());
        }

        let slice = &buffer[..length as usize];
        let cstr = unsafe { CStr::from_ptr(slice.as_ptr() as *const c_char) };
        Ok(cstr.to_str()?.to_string())
    }
}
