use std::os::raw::{c_int, c_longlong};
use crate::error::Result;
use crate::ffi::{self, helpers};
use crate::types::{CompressionMethod, EncryptionMethod};

/// Represents an entry (file or directory) within a zip archive
#[derive(Debug)]
pub struct ZipEntry {
    handle: c_longlong,
}

impl ZipEntry {
    /// Create a new ZipEntry from a handle
    pub(crate) fn new(handle: c_longlong) -> Result<Self> {
        if handle == 0 {
            return Err(crate::error::ZipError::InvalidHandle);
        }
        Ok(Self { handle })
    }
    
    /// Get the internal handle
    pub(crate) fn handle(&self) -> c_longlong {
        self.handle
    }
    
    /// Get the name of this entry
    pub fn name(&self) -> Result<String> {
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = vec![0i8; BUFFER_SIZE];
        let mut name_length: c_int = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_get_name(
                ffi::get_thread(),
                self.handle,
                buffer.as_mut_ptr(),
                BUFFER_SIZE as c_int,
                &mut name_length
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        helpers::read_string_from_buffer(&buffer, name_length)
    }
    
    /// Get the uncompressed size of this entry in bytes
    pub fn size(&self) -> Result<u64> {
        let mut size: c_longlong = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_get_size(
                ffi::get_thread(),
                self.handle,
                &mut size
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(size as u64)
    }
    
    /// Get the compressed size of this entry in bytes
    pub fn compressed_size(&self) -> Result<u64> {
        let mut compressed_size: c_longlong = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_get_compressed_size(
                ffi::get_thread(),
                self.handle,
                &mut compressed_size
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(compressed_size as u64)
    }
    
    /// Check if this entry is a directory
    pub fn is_directory(&self) -> Result<bool> {
        let mut is_directory: c_int = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_is_directory(
                ffi::get_thread(),
                self.handle,
                &mut is_directory
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(is_directory != 0)
    }
    
    /// Check if this entry is encrypted
    pub fn is_encrypted(&self) -> Result<bool> {
        let mut is_encrypted: c_int = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_is_encrypted(
                ffi::get_thread(),
                self.handle,
                &mut is_encrypted
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(is_encrypted != 0)
    }
    
    /// Get the CRC32 checksum of this entry
    pub fn crc32(&self) -> Result<u32> {
        let mut crc: c_longlong = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_get_crc(
                ffi::get_thread(),
                self.handle,
                &mut crc
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(crc as u32)
    }
    
    /// Get the last modified time of this entry (DOS time format)
    pub fn last_modified_time(&self) -> Result<u32> {
        let mut time: c_longlong = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_get_last_modified_time(
                ffi::get_thread(),
                self.handle,
                &mut time
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(time as u32)
    }
    
    /// Get the compression method used for this entry
    pub fn compression_method(&self) -> Result<CompressionMethod> {
        let mut method: c_int = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_get_compression_method(
                ffi::get_thread(),
                self.handle,
                &mut method
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(CompressionMethod::from(method))
    }
    
    /// Get the encryption method used for this entry
    pub fn encryption_method(&self) -> Result<EncryptionMethod> {
        let mut method: c_int = 0;
        
        let result = unsafe {
            ffi::zip4j_entry_get_encryption_method(
                ffi::get_thread(),
                self.handle,
                &mut method
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(EncryptionMethod::from(method))
    }
    
    /// Get the compression ratio as a percentage (0-100)
    pub fn compression_ratio(&self) -> Result<f64> {
        let size = self.size()? as f64;
        let compressed_size = self.compressed_size()? as f64;
        
        if size == 0.0 {
            return Ok(0.0);
        }
        
        let ratio = ((size - compressed_size) / size) * 100.0;
        Ok(ratio.max(0.0).min(100.0))
    }
}

impl Drop for ZipEntry {
    fn drop(&mut self) {
        // Release the entry handle
        unsafe {
            ffi::zip4j_release_entry(ffi::get_thread(), self.handle);
        }
    }
}
