use std::os::raw::c_int;
use crate::ffi::constants;

/// Compression levels for zip entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression
    None,
    /// Fastest compression (lowest compression ratio)
    Fastest,
    /// Normal compression (balanced speed and ratio)
    Normal,
    /// Maximum compression (highest compression ratio, slowest)
    Maximum,
}

impl From<CompressionLevel> for c_int {
    fn from(level: CompressionLevel) -> Self {
        match level {
            CompressionLevel::None => constants::COMPRESSION_LEVEL_NONE,
            CompressionLevel::Fastest => constants::COMPRESSION_LEVEL_FASTEST,
            CompressionLevel::Normal => constants::COMPRESSION_LEVEL_NORMAL,
            CompressionLevel::Maximum => constants::COMPRESSION_LEVEL_MAXIMUM,
        }
    }
}

/// Compression methods for zip entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    /// Store without compression
    Store,
    /// Deflate compression (standard)
    Deflate,
}

impl From<CompressionMethod> for c_int {
    fn from(method: CompressionMethod) -> Self {
        match method {
            CompressionMethod::Store => constants::COMPRESSION_STORE,
            CompressionMethod::Deflate => constants::COMPRESSION_DEFLATE,
        }
    }
}

impl From<c_int> for CompressionMethod {
    fn from(value: c_int) -> Self {
        match value {
            constants::COMPRESSION_STORE => CompressionMethod::Store,
            constants::COMPRESSION_DEFLATE => CompressionMethod::Deflate,
            _ => CompressionMethod::Deflate, // Default
        }
    }
}

/// Encryption methods for zip entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionMethod {
    /// No encryption
    None,
    /// Standard ZIP encryption
    Standard,
    /// AES 128-bit encryption
    Aes128,
    /// AES 256-bit encryption
    Aes256,
}

impl From<EncryptionMethod> for c_int {
    fn from(method: EncryptionMethod) -> Self {
        match method {
            EncryptionMethod::None => constants::ENCRYPTION_NONE,
            EncryptionMethod::Standard => constants::ENCRYPTION_STANDARD,
            EncryptionMethod::Aes128 => constants::ENCRYPTION_AES_128,
            EncryptionMethod::Aes256 => constants::ENCRYPTION_AES_256,
        }
    }
}

impl From<c_int> for EncryptionMethod {
    fn from(value: c_int) -> Self {
        match value {
            constants::ENCRYPTION_NONE => EncryptionMethod::None,
            constants::ENCRYPTION_STANDARD => EncryptionMethod::Standard,
            constants::ENCRYPTION_AES_128 => EncryptionMethod::Aes128,
            constants::ENCRYPTION_AES_256 => EncryptionMethod::Aes256,
            _ => EncryptionMethod::None, // Default
        }
    }
}

/// AES key strength for AES encryption
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AesKeyStrength {
    /// 128-bit key
    Aes128,
    /// 192-bit key
    Aes192,
    /// 256-bit key
    Aes256,
}

impl From<AesKeyStrength> for c_int {
    fn from(strength: AesKeyStrength) -> Self {
        match strength {
            AesKeyStrength::Aes128 => constants::AES_KEY_STRENGTH_128,
            AesKeyStrength::Aes192 => constants::AES_KEY_STRENGTH_192,
            AesKeyStrength::Aes256 => constants::AES_KEY_STRENGTH_256,
        }
    }
}

/// Parameters for adding files to a zip archive
#[derive(Debug, Clone)]
pub struct ZipParameters {
    /// Compression level
    pub compression_level: CompressionLevel,
    /// Compression method
    pub compression_method: CompressionMethod,
    /// Encryption method
    pub encryption_method: EncryptionMethod,
    /// AES key strength (only used with AES encryption)
    pub aes_key_strength: AesKeyStrength,
    /// Password for encryption (if any)
    pub password: Option<String>,
}

impl Default for ZipParameters {
    fn default() -> Self {
        Self {
            compression_level: CompressionLevel::Normal,
            compression_method: CompressionMethod::Deflate,
            encryption_method: EncryptionMethod::None,
            aes_key_strength: AesKeyStrength::Aes256,
            password: None,
        }
    }
}

impl ZipParameters {
    /// Create new default parameters
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set compression level
    pub fn with_compression_level(mut self, level: CompressionLevel) -> Self {
        self.compression_level = level;
        self
    }
    
    /// Set compression method
    pub fn with_compression_method(mut self, method: CompressionMethod) -> Self {
        self.compression_method = method;
        self
    }
    
    /// Set encryption method
    pub fn with_encryption_method(mut self, method: EncryptionMethod) -> Self {
        self.encryption_method = method;
        self
    }
    
    /// Set AES key strength
    pub fn with_aes_key_strength(mut self, strength: AesKeyStrength) -> Self {
        self.aes_key_strength = strength;
        self
    }
    
    /// Set password for encryption
    pub fn with_password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = Some(password.into());
        self
    }
    
    /// Enable AES 256-bit encryption with password
    pub fn with_aes256_encryption<S: Into<String>>(mut self, password: S) -> Self {
        self.encryption_method = EncryptionMethod::Aes256;
        self.aes_key_strength = AesKeyStrength::Aes256;
        self.password = Some(password.into());
        self
    }
    
    /// Enable AES 128-bit encryption with password
    pub fn with_aes128_encryption<S: Into<String>>(mut self, password: S) -> Self {
        self.encryption_method = EncryptionMethod::Aes128;
        self.aes_key_strength = AesKeyStrength::Aes128;
        self.password = Some(password.into());
        self
    }
    
    /// Enable standard ZIP encryption with password
    pub fn with_standard_encryption<S: Into<String>>(mut self, password: S) -> Self {
        self.encryption_method = EncryptionMethod::Standard;
        self.password = Some(password.into());
        self
    }
}
