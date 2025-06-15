use std::path::Path;
use std::os::raw::{c_char, c_int, c_longlong};
use crate::error::Result;
use crate::ffi::{self, helpers};
use crate::zip_entry::ZipEntry;
use crate::types::ZipParameters;

/// Represents a zip file that can be read from or written to
#[derive(Debug)]
pub struct ZipFile {
    handle: c_longlong,
    path: String,
}

impl ZipFile {
    /// Create a new zip file or open an existing one
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the zip file
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use zip2rs::ZipFile;
    /// 
    /// let zip = ZipFile::new("archive.zip")?;
    /// # Ok::<(), zip2rs::ZipError>(())
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        ffi::ensure_initialized()?;

        let path_str = path.as_ref().to_string_lossy().to_string();
        let c_path = helpers::to_c_string(&path_str)?;
        let mut handle: c_longlong = 0;

        let result = unsafe {
            ffi::zip4j_create(
                ffi::get_thread(),
                c_path.as_ptr() as *mut c_char,
                &mut handle
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(Self { handle, path: path_str })
    }
    
    /// Create a new zip file or open an existing one with a password
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the zip file
    /// * `password` - Password for the zip file
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use zip2rs::ZipFile;
    /// 
    /// let zip = ZipFile::with_password("archive.zip", "secret")?;
    /// # Ok::<(), zip2rs::ZipError>(())
    /// ```
    pub fn with_password<P: AsRef<Path>, S: AsRef<str>>(path: P, password: S) -> Result<Self> {
        ffi::ensure_initialized()?;

        let path_str = path.as_ref().to_string_lossy().to_string();
        let c_path = helpers::to_c_string(&path_str)?;
        let c_password = helpers::to_c_string(password.as_ref())?;
        let mut handle: c_longlong = 0;

        let result = unsafe {
            ffi::zip4j_create_with_password(
                ffi::get_thread(),
                c_path.as_ptr() as *mut c_char,
                c_password.as_ptr() as *mut c_char,
                &mut handle
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(Self { handle, path: path_str })
    }
    
    /// Set or change the password for the zip file
    /// 
    /// # Arguments
    /// 
    /// * `password` - New password for the zip file
    pub fn set_password<S: AsRef<str>>(&mut self, password: S) -> Result<()> {
        let c_password = helpers::to_c_string(password.as_ref())?;
        
        let result = unsafe {
            ffi::zip4j_set_password(
                ffi::get_thread(),
                self.handle,
                c_password.as_ptr() as *mut c_char
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(())
    }
    
    /// Check if the zip file is valid
    ///
    /// Note: A newly created ZIP file may not be valid until entries are added
    pub fn is_valid(&self) -> Result<bool> {
        let mut is_valid: c_int = 0;

        let result = unsafe {
            ffi::zip4j_is_valid(
                ffi::get_thread(),
                self.handle,
                &mut is_valid
            )
        };

        // Don't treat "invalid" as an error - it's just a state
        if helpers::is_error(result) && result != ffi::constants::ERROR_INVALID_PARAMETER {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(is_valid != 0)
    }
    
    /// Check if the zip file is encrypted
    pub fn is_encrypted(&self) -> Result<bool> {
        let mut is_encrypted: c_int = 0;
        
        let result = unsafe {
            ffi::zip4j_is_encrypted(
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
    
    /// Check if the zip file is a split archive
    pub fn is_split_archive(&self) -> Result<bool> {
        let mut is_split: c_int = 0;
        
        let result = unsafe {
            ffi::zip4j_is_split_archive(
                ffi::get_thread(),
                self.handle,
                &mut is_split
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        Ok(is_split != 0)
    }
    
    /// Get the file path of the zip file
    pub fn file_path(&self) -> Result<String> {
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let mut path_length: c_int = 0;

        let result = unsafe {
            ffi::zip4j_get_file_path(
                ffi::get_thread(),
                self.handle,
                buffer.as_mut_ptr() as *mut c_char,
                BUFFER_SIZE as c_int,
                &mut path_length
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        helpers::read_string_from_buffer_u8(&buffer, path_length)
    }
    
    /// Get the comment of the zip file
    ///
    /// Returns an empty string if the ZIP file has no comment or is invalid
    pub fn comment(&self) -> Result<String> {
        // Check if the ZIP is valid first
        if !self.is_valid()? {
            return Ok(String::new());
        }

        const BUFFER_SIZE: usize = 1024;
        let mut buffer = vec![0u8; BUFFER_SIZE];
        let mut comment_length: c_int = 0;

        let result = unsafe {
            ffi::zip4j_get_comment(
                ffi::get_thread(),
                self.handle,
                buffer.as_mut_ptr() as *mut c_char,
                BUFFER_SIZE as c_int,
                &mut comment_length
            )
        };

        if helpers::is_error(result) {
            return Ok(String::new()); // Return empty string instead of error
        }

        helpers::read_string_from_buffer_u8(&buffer, comment_length)
    }

    /// Set the comment of the zip file
    ///
    /// # Arguments
    ///
    /// * `comment` - New comment for the zip file
    ///
    /// # Note
    ///
    /// This operation may fail if the ZIP file is empty or invalid.
    /// Add entries to the ZIP file first to ensure it's valid.
    pub fn set_comment<S: AsRef<str>>(&mut self, comment: S) -> Result<()> {
        // Check if the ZIP is valid first
        if !self.is_valid()? {
            return Err(crate::error::ZipError::InvalidParameter(
                "Cannot set comment on an empty or invalid ZIP file. Add entries first.".to_string()
            ));
        }

        let c_comment = helpers::to_c_string(comment.as_ref())?;

        let result = unsafe {
            ffi::zip4j_set_comment(
                ffi::get_thread(),
                self.handle,
                c_comment.as_ptr() as *mut c_char
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }
    
    /// Get the number of entries in the zip file
    ///
    /// Returns 0 for empty or invalid ZIP files
    pub fn entry_count(&self) -> Result<usize> {
        let mut count: c_longlong = 0;

        let result = unsafe {
            ffi::zip4j_get_entry_count(
                ffi::get_thread(),
                self.handle,
                &mut count
            )
        };

        if helpers::is_error(result) {
            // Return 0 for invalid ZIP files instead of error
            return Ok(0);
        }

        Ok(count as usize)
    }
    
    /// Get an entry by its index
    /// 
    /// # Arguments
    /// 
    /// * `index` - Zero-based index of the entry
    pub fn get_entry_by_index(&self, index: usize) -> Result<ZipEntry> {
        let mut entry_handle: c_longlong = 0;
        
        let result = unsafe {
            ffi::zip4j_get_entry_by_index(
                ffi::get_thread(),
                self.handle,
                index as c_longlong,
                &mut entry_handle
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        ZipEntry::new(entry_handle)
    }
    
    /// Get an entry by its name
    /// 
    /// # Arguments
    /// 
    /// * `name` - Name of the entry (full path within the zip)
    pub fn get_entry_by_name<S: AsRef<str>>(&self, name: S) -> Result<ZipEntry> {
        let c_name = helpers::to_c_string(name.as_ref())?;
        let mut entry_handle: c_longlong = 0;
        
        let result = unsafe {
            ffi::zip4j_get_entry_by_name(
                ffi::get_thread(),
                self.handle,
                c_name.as_ptr() as *mut c_char,
                &mut entry_handle
            )
        };
        
        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }
        
        ZipEntry::new(entry_handle)
    }

    /// Add a file to the zip archive with default parameters
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file to add
    pub fn add_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<()> {
        let path_str = file_path.as_ref().to_string_lossy();
        let c_path = helpers::to_c_string(&path_str)?;

        let result = unsafe {
            ffi::zip4j_add_file(
                ffi::get_thread(),
                self.handle,
                c_path.as_ptr() as *mut c_char
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Add a file to the zip archive with custom parameters
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file to add
    /// * `params` - Compression and encryption parameters
    pub fn add_file_with_params<P: AsRef<Path>>(&mut self, file_path: P, params: &ZipParameters) -> Result<()> {
        let path_str = file_path.as_ref().to_string_lossy();
        let c_path = helpers::to_c_string(&path_str)?;

        let c_password = match &params.password {
            Some(pwd) => {
                let c_pwd = helpers::to_c_string(pwd)?;
                c_pwd.as_ptr() as *mut c_char
            }
            None => std::ptr::null_mut(),
        };

        let result = unsafe {
            ffi::zip4j_add_file_with_params(
                ffi::get_thread(),
                self.handle,
                c_path.as_ptr() as *mut c_char,
                params.compression_level.into(),
                params.compression_method.into(),
                params.encryption_method.into(),
                params.aes_key_strength.into(),
                c_password,
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Add a directory to the zip archive
    ///
    /// # Arguments
    ///
    /// * `dir_path` - Path to the directory to add
    pub fn add_directory<P: AsRef<Path>>(&mut self, dir_path: P) -> Result<()> {
        let path_str = dir_path.as_ref().to_string_lossy();
        let c_path = helpers::to_c_string(&path_str)?;

        let result = unsafe {
            ffi::zip4j_add_directory(
                ffi::get_thread(),
                self.handle,
                c_path.as_ptr() as *mut c_char
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Add a directory to the zip archive with custom parameters
    ///
    /// # Arguments
    ///
    /// * `dir_path` - Path to the directory to add
    /// * `params` - Compression and encryption parameters
    pub fn add_directory_with_params<P: AsRef<Path>>(&mut self, dir_path: P, params: &ZipParameters) -> Result<()> {
        let path_str = dir_path.as_ref().to_string_lossy();
        let c_path = helpers::to_c_string(&path_str)?;

        let c_password = match &params.password {
            Some(pwd) => {
                let c_pwd = helpers::to_c_string(pwd)?;
                c_pwd.as_ptr() as *mut c_char
            }
            None => std::ptr::null_mut(),
        };

        let result = unsafe {
            ffi::zip4j_add_directory_with_params(
                ffi::get_thread(),
                self.handle,
                c_path.as_ptr() as *mut c_char,
                params.compression_level.into(),
                params.compression_method.into(),
                params.encryption_method.into(),
                params.aes_key_strength.into(),
                c_password,
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Add data from a byte slice to the zip archive
    ///
    /// # Arguments
    ///
    /// * `entry_name` - Name for the entry in the zip file
    /// * `data` - Byte data to add
    /// * `params` - Compression and encryption parameters
    pub fn add_data<S: AsRef<str>>(&mut self, entry_name: S, data: &[u8], params: &ZipParameters) -> Result<()> {
        let c_name = helpers::to_c_string(entry_name.as_ref())?;

        let c_password = match &params.password {
            Some(pwd) => {
                let c_pwd = helpers::to_c_string(pwd)?;
                c_pwd.as_ptr() as *mut c_char
            }
            None => std::ptr::null_mut(),
        };

        let result = unsafe {
            ffi::zip4j_add_data(
                ffi::get_thread(),
                self.handle,
                c_name.as_ptr() as *mut c_char,
                data.as_ptr() as *mut c_char,
                data.len() as c_int,
                params.compression_level.into(),
                params.compression_method.into(),
                params.encryption_method.into(),
                params.aes_key_strength.into(),
                c_password,
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Extract all files from the zip archive to a destination directory
    ///
    /// # Arguments
    ///
    /// * `dest_path` - Directory where files should be extracted
    pub fn extract_all<P: AsRef<Path>>(&self, dest_path: P) -> Result<()> {
        let path_str = dest_path.as_ref().to_string_lossy();
        let c_path = helpers::to_c_string(&path_str)?;

        let result = unsafe {
            ffi::zip4j_extract_all(
                ffi::get_thread(),
                self.handle,
                c_path.as_ptr() as *mut c_char
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Extract a specific file by name from the zip archive
    ///
    /// # Arguments
    ///
    /// * `file_name` - Name of the file to extract
    /// * `dest_path` - Directory where the file should be extracted
    pub fn extract_file<S: AsRef<str>, P: AsRef<Path>>(&self, file_name: S, dest_path: P) -> Result<()> {
        let c_name = helpers::to_c_string(file_name.as_ref())?;
        let path_str = dest_path.as_ref().to_string_lossy();
        let c_path = helpers::to_c_string(&path_str)?;

        let result = unsafe {
            ffi::zip4j_extract_file(
                ffi::get_thread(),
                self.handle,
                c_name.as_ptr() as *mut c_char,
                c_path.as_ptr() as *mut c_char
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Extract a specific entry to a destination directory
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to extract
    /// * `dest_path` - Directory where the entry should be extracted
    pub fn extract_entry<P: AsRef<Path>>(&self, entry: &ZipEntry, dest_path: P) -> Result<()> {
        let path_str = dest_path.as_ref().to_string_lossy();
        let c_path = helpers::to_c_string(&path_str)?;

        let result = unsafe {
            ffi::zip4j_extract_entry(
                ffi::get_thread(),
                self.handle,
                entry.handle(),
                c_path.as_ptr() as *mut c_char
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Extract data from an entry to a byte vector (in-memory extraction)
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to extract
    ///
    /// # Returns
    ///
    /// A vector containing the extracted data
    pub fn extract_data(&self, entry: &ZipEntry) -> Result<Vec<u8>> {
        // Start with a reasonable buffer size
        let mut buffer_size = entry.size()? as usize;
        if buffer_size == 0 {
            buffer_size = 1024; // Default size for unknown sizes
        }

        let mut buffer = vec![0u8; buffer_size];
        let mut data_length: c_int = 0;

        let result = unsafe {
            ffi::zip4j_extract_data(
                ffi::get_thread(),
                self.handle,
                entry.handle(),
                buffer.as_mut_ptr() as *mut c_char,
                buffer_size as c_int,
                &mut data_length
            )
        };

        if result == crate::ffi::constants::ERROR_BUFFER_TOO_SMALL {
            // Resize buffer and try again
            buffer.resize(data_length as usize, 0);
            let result = unsafe {
                ffi::zip4j_extract_data(
                    ffi::get_thread(),
                    self.handle,
                    entry.handle(),
                    buffer.as_mut_ptr() as *mut c_char,
                    buffer.len() as c_int,
                    &mut data_length
                )
            };

            if helpers::is_error(result) {
                return Err(crate::error::ZipError::from_code(result));
            }
        } else if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        // Truncate buffer to actual data length
        buffer.truncate(data_length as usize);
        Ok(buffer)
    }

    /// Remove a file from the zip archive by name
    ///
    /// # Arguments
    ///
    /// * `file_name` - Name of the file to remove
    pub fn remove_file<S: AsRef<str>>(&mut self, file_name: S) -> Result<()> {
        let c_name = helpers::to_c_string(file_name.as_ref())?;

        let result = unsafe {
            ffi::zip4j_remove_file(
                ffi::get_thread(),
                self.handle,
                c_name.as_ptr() as *mut c_char
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Remove an entry from the zip archive
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to remove
    pub fn remove_entry(&mut self, entry: &ZipEntry) -> Result<()> {
        let result = unsafe {
            ffi::zip4j_remove_entry(
                ffi::get_thread(),
                self.handle,
                entry.handle()
            )
        };

        if helpers::is_error(result) {
            return Err(crate::error::ZipError::from_code(result));
        }

        Ok(())
    }

    /// Get an iterator over all entries in the zip file
    pub fn entries(&self) -> Result<ZipEntryIterator> {
        let count = self.entry_count()?;
        Ok(ZipEntryIterator {
            zip_file: self,
            current_index: 0,
            total_count: count,
        })
    }

    /// Get the internal handle (for advanced use cases)
    pub(crate) fn handle(&self) -> c_longlong {
        self.handle
    }
}

impl Drop for ZipFile {
    fn drop(&mut self) {
        // Close the zip file handle
        unsafe {
            ffi::zip4j_close(ffi::get_thread(), self.handle);
        }
    }
}

/// Iterator over entries in a zip file
pub struct ZipEntryIterator<'a> {
    zip_file: &'a ZipFile,
    current_index: usize,
    total_count: usize,
}

impl<'a> Iterator for ZipEntryIterator<'a> {
    type Item = Result<ZipEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.total_count {
            return None;
        }

        let result = self.zip_file.get_entry_by_index(self.current_index);
        self.current_index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_count - self.current_index;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for ZipEntryIterator<'a> {
    fn len(&self) -> usize {
        self.total_count - self.current_index
    }
}
