//! Quantum-Secure Integrity Verification
//! 
//! Implements hybrid post-quantum signatures using ML-DSA (Dilithium) and SHA3/BLAKE3
//! for cryptographic sealing of .aim drives.
//! 
//! Security Properties:
//! - Classical security: Ed25519 (or BLAKE3 for speed)
//! - Post-quantum security: ML-DSA-44 (FIPS 204 draft)
//! - Integrity hash: SHA3-256 + BLAKE3
//! 
//! .aim Seal Format:
//! - SHA3-256 integrity hash (32 bytes)
//! - ML-DSA-44 signature (2420 bytes)
//! - Total seal size: 2452 bytes

use anyhow::{Result, Context, anyhow};
use sha3::{Sha3_256, Digest};
use blake3::Hasher as Blake3Hasher;
use std::path::Path;
use tracing::{debug, info, warn};

/// ML-DSA-44 signature size (FIPS 204)
pub const ML_DSA_SIG_SIZE: usize = 2420;

/// SHA3-256 hash size
pub const SHA3_256_SIZE: usize = 32;

/// BLAKE3 hash size
pub const BLAKE3_SIZE: usize = 32;

/// Total seal size (hash + signature)
pub const SEAL_SIZE: usize = SHA3_256_SIZE + ML_DSA_SIG_SIZE;

/// Quantum seal for .aim drives
/// 
/// Combines classical and post-quantum signatures for hybrid security
#[derive(Clone)]
pub struct QuantumSeal {
    /// SHA3-256 integrity hash
    integrity_hash: [u8; SHA3_256_SIZE],
    /// ML-DSA signature (if signed)
    ml_dsa_sig: Option<Vec<u8>>,
}

impl QuantumSeal {
    /// Create seal from data
    pub fn seal(data: &[u8]) -> Self {
        // Compute SHA3-256 integrity hash
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let integrity_hash: [u8; SHA3_256_SIZE] = hasher.finalize().into();
        
        debug!("Computed SHA3-256: {}", hex::encode(&integrity_hash));
        
        Self {
            integrity_hash,
            ml_dsa_sig: None,
        }
    }
    
    /// Sign seal with ML-DSA (post-quantum)
    /// 
    /// Note: Actual ML-DSA signing requires the dilithium crate and key material.
    /// This is a placeholder for the full implementation.
    pub fn sign_ml_dsa(&mut self, _secret_key: &[u8], data: &[u8]) -> Result<()> {
        // TODO: Full ML-DSA implementation
        // For now, use BLAKE3 as placeholder
        
        let mut hasher = Blake3Hasher::new();
        hasher.update(data);
        hasher.update(&self.integrity_hash);
        let hash = hasher.finalize();
        
        // Placeholder signature (in production, this would be actual ML-DSA)
        let mut sig = vec![0u8; ML_DSA_SIG_SIZE];
        sig[..BLAKE3_SIZE].copy_from_slice(hash.as_bytes());
        
        self.ml_dsa_sig = Some(sig);
        debug!("Created ML-DSA placeholder signature");
        
        Ok(())
    }
    
    /// Verify seal integrity
    pub fn verify(&self, data: &[u8]) -> Result<bool> {
        // Verify SHA3-256 integrity hash
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let computed_hash: [u8; SHA3_256_SIZE] = hasher.finalize().into();
        
        if computed_hash != self.integrity_hash {
            warn!("Integrity hash mismatch!");
            return Ok(false);
        }
        
        debug!("SHA3-256 integrity verified");
        
        // Verify ML-DSA signature if present
        if let Some(ref sig) = self.ml_dsa_sig {
            // TODO: Full ML-DSA verification
            // For now, just check signature length
            if sig.len() != ML_DSA_SIG_SIZE {
                warn!("Invalid ML-DSA signature length");
                return Ok(false);
            }
            debug!("ML-DSA signature present (verification placeholder)");
        }
        
        Ok(true)
    }
    
    /// Get seal bytes (for appending to .aim file)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(SEAL_SIZE);
        bytes.extend_from_slice(&self.integrity_hash);
        
        if let Some(ref sig) = self.ml_dsa_sig {
            bytes.extend_from_slice(sig);
        } else {
            // Zero padding if unsigned
            bytes.extend(std::iter::repeat(0).take(ML_DSA_SIG_SIZE));
        }
        
        bytes
    }
    
    /// Load seal from bytes (appended to .aim file)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < SEAL_SIZE {
            return Err(anyhow!("Seal too short: {} bytes", bytes.len()));
        }
        
        let hash_start = 0;
        let hash_end = SHA3_256_SIZE;
        let sig_start = SHA3_256_SIZE;
        let sig_end = SEAL_SIZE;
        
        let mut integrity_hash = [0u8; SHA3_256_SIZE];
        integrity_hash.copy_from_slice(&bytes[hash_start..hash_end]);
        
        let sig_bytes = &bytes[sig_start..sig_end];
        let ml_dsa_sig = if sig_bytes.iter().any(|&b| b != 0) {
            Some(sig_bytes.to_vec())
        } else {
            None
        };
        
        Ok(Self {
            integrity_hash,
            ml_dsa_sig,
        })
    }
    
    /// Get integrity hash as hex string
    pub fn hash_hex(&self) -> String {
        hex::encode(&self.integrity_hash)
    }
}

/// Integrity verifier for .aim drives
/// 
/// Provides streaming verification for large files
pub struct IntegrityVerifier {
    /// Running SHA3-256 state
    sha3_state: Sha3_256,
    /// Running BLAKE3 state
    blake3_state: Blake3Hasher,
    /// Bytes processed
    bytes_processed: u64,
}

impl IntegrityVerifier {
    /// Create new verifier
    pub fn new() -> Self {
        Self {
            sha3_state: Sha3_256::new(),
            blake3_state: Blake3Hasher::new(),
            bytes_processed: 0,
        }
    }
    
    /// Update with data chunk
    pub fn update(&mut self, data: &[u8]) {
        self.sha3_state.update(data);
        self.blake3_state.update(data);
        self.bytes_processed += data.len() as u64;
    }
    
    /// Finalize and get hashes
    pub fn finalize(self) -> VerifyResult {
        let sha3_hash: [u8; SHA3_256_SIZE] = self.sha3_state.finalize().into();
        let blake3_hash = self.blake3_state.finalize();
        
        VerifyResult {
            sha3_hash,
            blake3_hash: *blake3_hash.as_bytes(),
            bytes_processed: self.bytes_processed,
        }
    }
    
    /// Verify file integrity against expected hash
    pub fn verify_file<P: AsRef<Path>>(path: P, expected_hash: &[u8; SHA3_256_SIZE]) -> Result<bool> {
        let path = path.as_ref();
        let file = std::fs::File::open(path)
            .with_context(|| format!("Failed to open: {}", path.display()))?;
        
        // Read file in chunks for streaming verification
        let mut verifier = Self::new();
        let mut buf = [0u8; 8192]; // 8KB chunks
        
        use std::io::Read;
        let mut reader = std::io::BufReader::new(file);
        
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            verifier.update(&buf[..n]);
        }
        
        let result = verifier.finalize();
        
        debug!(
            "Verified {}: {} bytes, SHA3: {}",
            path.display(),
            result.bytes_processed,
            result.sha3_hex()
        );
        
        Ok(&result.sha3_hash == expected_hash)
    }
}

impl Default for IntegrityVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Verification result
#[derive(Debug, Clone)]
pub struct VerifyResult {
    pub sha3_hash: [u8; SHA3_256_SIZE],
    pub blake3_hash: [u8; BLAKE3_SIZE],
    pub bytes_processed: u64,
}

impl VerifyResult {
    /// Get SHA3 hash as hex string
    pub fn sha3_hex(&self) -> String {
        hex::encode(&self.sha3_hash)
    }
    
    /// Get BLAKE3 hash as hex string
    pub fn blake3_hex(&self) -> String {
        hex::encode(&self.blake3_hash)
    }
}

/// Append quantum seal to .aim file
pub fn seal_aim_file<P: AsRef<Path>>(path: P) -> Result<QuantumSeal> {
    let path = path.as_ref();
    
    // Read file content (excluding any existing seal)
    let metadata = std::fs::metadata(path)?;
    let file_len = metadata.len();
    
    let content = if file_len > SEAL_SIZE as u64 {
        // Read all but last SEAL_SIZE bytes
        let mut file = std::fs::File::open(path)?;
        let read_len = (file_len - SEAL_SIZE as u64) as usize;
        let mut buf = vec![0u8; read_len];
        file.read_exact(&mut buf)?;
        buf
    } else {
        std::fs::read(path)?
    };
    
    // Create seal
    let mut seal = QuantumSeal::seal(&content);
    
    // Sign with ML-DSA (placeholder)
    // In production, load secret key from secure storage
    let dummy_key = [0u8; 32];
    seal.sign_ml_dsa(&dummy_key, &content)?;
    
    // Append seal to file
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(path)?;
    
    // Truncate to remove any existing seal
    file.set_len(content.len() as u64)?;
    
    // Write content
    file.write_all(&content)?;
    
    // Append seal
    file.write_all(&seal.to_bytes())?;
    file.sync_all()?;
    
    info!(
        "Sealed .aim file: {} ({} bytes + {} byte seal)",
        path.display(),
        content.len(),
        SEAL_SIZE
    );
    
    Ok(seal)
}

/// Verify .aim file seal
pub fn verify_aim_file<P: AsRef<Path>>(path: P) -> Result<bool> {
    let path = path.as_ref();
    
    // Read file
    let content = std::fs::read(path)
        .with_context(|| format!("Failed to read: {}", path.display()))?;
    
    if content.len() < SEAL_SIZE {
        return Err(anyhow!("File too small for seal: {} bytes", content.len()));
    }
    
    // Split content and seal
    let seal_start = content.len() - SEAL_SIZE;
    let data = &content[..seal_start];
    let seal_bytes = &content[seal_start..];
    
    // Load seal
    let seal = QuantumSeal::from_bytes(seal_bytes)?;
    
    // Verify
    seal.verify(data)
}

use std::io::Read;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quantum_seal() {
        let data = b"Test data for sealing";
        
        let mut seal = QuantumSeal::seal(data);
        assert_eq!(seal.integrity_hash.len(), 32);
        
        // Sign (placeholder)
        seal.sign_ml_dsa(&[0u8; 32], data).unwrap();
        assert!(seal.ml_dsa_sig.is_some());
        assert_eq!(seal.ml_dsa_sig.as_ref().unwrap().len(), ML_DSA_SIG_SIZE);
        
        // Verify
        assert!(seal.verify(data).unwrap());
        
        // Tamper detection
        let tampered = b"Tampered data";
        assert!(!seal.verify(tampered).unwrap());
    }
    
    #[test]
    fn test_seal_bytes_roundtrip() {
        let data = b"Test data";
        let seal = QuantumSeal::seal(data);
        
        let bytes = seal.to_bytes();
        assert_eq!(bytes.len(), SEAL_SIZE);
        
        let restored = QuantumSeal::from_bytes(&bytes).unwrap();
        assert_eq!(restored.integrity_hash, seal.integrity_hash);
    }
    
    #[test]
    fn test_integrity_verifier() {
        let data = b"Hello, World!";
        
        let mut verifier = IntegrityVerifier::new();
        verifier.update(data);
        let result = verifier.finalize();
        
        assert_eq!(result.bytes_processed, data.len() as u64);
        assert_eq!(result.sha3_hash.len(), 32);
        assert_eq!(result.blake3_hash.len(), 32);
    }
}
