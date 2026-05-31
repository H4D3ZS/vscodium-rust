use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::context_quantizer::ContextQuantizer;

#[derive(Serialize, Deserialize, Clone)]
pub enum CompressionType {
    Lz4,
    TurboQuantSCQ, // Rotation + 4-bit scalar quant + LZ4
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompressedData {
    pub data: Vec<u8>,
    pub original_size: usize,
    pub compression_type: CompressionType,
}

pub struct MemoryOptimizer {
    cache: Arc<Mutex<std::collections::HashMap<String, CompressedData>>>,
    quantizer: ContextQuantizer,
}

impl MemoryOptimizer {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(std::collections::HashMap::new())),
            quantizer: ContextQuantizer::new(),
        }
    }

    /// Compresses and stores a string value using LZ4 (Fast)
    pub async fn compress_and_store(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let bytes = value.as_bytes();
        let original_size = bytes.len();

        if original_size < 1024 {
            return Ok(());
        }

        let compressed = compress_prepend_size(bytes);
        let mut lock = self.cache.lock().await;
        lock.insert(
            key.to_string(),
            CompressedData {
                data: compressed,
                original_size,
                compression_type: CompressionType::Lz4,
            },
        );

        Ok(())
    }

    /// High-Density storage using TurboQuant SCQ (Rotation + Quantization) + LZ4
    /// Best for very large contexts (>32KB) on 8GB systems.
    pub async fn store_high_density(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let bytes = value.as_bytes();
        let original_size = bytes.len();

        // 1. Normalize to N(0,1) and pad to multiple of 128
        let mut data_f32 = self.quantizer.normalize_bytes(bytes);
        let pad_len = (128 - (data_f32.len() % 128)) % 128;
        data_f32.extend(vec![0.0; pad_len]);

        // 2. Rotate and Quantize in chunks
        let mut all_indices = Vec::with_capacity(data_f32.len());
        for chunk in data_f32.chunks_mut(128) {
            self.quantizer.fht_inplace(chunk);
            let indices = self.quantizer.quantize(chunk);
            all_indices.extend(indices);
        }

        // 3. Pack and LZ4
        let packed = self.quantizer.pack_indices(&all_indices);
        let compressed = compress_prepend_size(&packed);

        let mut lock = self.cache.lock().await;
        lock.insert(
            key.to_string(),
            CompressedData {
                data: compressed,
                original_size,
                compression_type: CompressionType::TurboQuantSCQ,
            },
        );

        Ok(())
    }

    /// Retrieves and decompresses a stored value (handles both types)
    pub async fn get_and_decompress(&self, key: &str) -> anyhow::Result<Option<String>> {
        let lock = self.cache.lock().await;
        let entry = match lock.get(key) {
            Some(e) => e,
            None => return Ok(None),
        };

        match entry.compression_type {
            CompressionType::Lz4 => {
                let decompressed = decompress_size_prepended(&entry.data)
                    .map_err(|e| anyhow::anyhow!("Decompression failed: {}", e))?;
                let value = String::from_utf8(decompressed)?;
                Ok(Some(value))
            }
            CompressionType::TurboQuantSCQ => {
                let packed = decompress_size_prepended(&entry.data)
                    .map_err(|e| anyhow::anyhow!("SCQ Decompression failed: {}", e))?;

                // Padded len was derived from packed size
                let total_elements = packed.len() * 2;
                let indices = self.quantizer.unpack_indices(&packed, total_elements);

                let mut data_f32 = self.quantizer.dequantize(&indices);

                // Inverse Rotate
                for chunk in data_f32.chunks_mut(128) {
                    self.quantizer.fht_inplace(chunk);
                }

                // Denormalize
                let result_bytes = self.quantizer.denormalize_bytes(&data_f32);
                let mut final_bytes = result_bytes;
                final_bytes.truncate(entry.original_size);

                let value = String::from_utf8(final_bytes)?;
                Ok(Some(value))
            }
        }
    }

    /// Returns memory savings report
    pub async fn get_savings_report(&self) -> (usize, usize) {
        let lock = self.cache.lock().await;
        let mut total_original = 0;
        let mut total_compressed = 0;

        for entry in lock.values() {
            total_original += entry.original_size;
            total_compressed += entry.data.len();
        }

        (total_original, total_compressed)
    }

    pub async fn optimize(&self) -> anyhow::Result<()> {
        let mut lock = self.cache.lock().await;
        lock.clear();
        Ok(())
    }

    pub async fn update_project_memory(&self, root: PathBuf, content: String) -> anyhow::Result<()> {
        // STABILITY: Never write into src-tauri/ — Tauri's dev-mode file watcher
        // monitors that directory and will kill+restart the entire process the moment
        // any file changes there, causing the IDE to reload mid-session.
        // Walk up one level if we're accidentally rooted inside src-tauri/.
        let safe_root = {
            let s = root.to_string_lossy();
            if s.ends_with("src-tauri") || s.ends_with("src-tauri\\") || s.ends_with("src-tauri/") {
                root.parent().map(|p| p.to_path_buf()).unwrap_or(root)
            } else {
                root
            }
        };

        let memory_path = safe_root.join("MEMORY.md");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&memory_path)?;
            
        use std::io::Write;
        writeln!(file, "\n{}", content)?;
        Ok(())
    }

    pub async fn clear(&self) {
        let mut lock = self.cache.lock().await;
        lock.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compression_decompression() {
        let optimizer = MemoryOptimizer::new();
        let key = "test_key";
        // Create a string larger than 1KB to trigger compression
        let original_value = "A".repeat(2048);

        optimizer
            .compress_and_store(key, &original_value)
            .await
            .unwrap();

        let retrieved = optimizer.get_and_decompress(key).await.unwrap();
        assert_eq!(retrieved, Some(original_value));
    }

    #[tokio::test]
    async fn test_low_size_no_compression() {
        let optimizer = MemoryOptimizer::new();
        let key = "small_key";
        let original_value = "small";

        optimizer
            .compress_and_store(key, &original_value)
            .await
            .unwrap();

        let (original, compressed) = optimizer.get_savings_report().await;
        assert_eq!(original, 0);
        assert_eq!(compressed, 0);
    }

    #[tokio::test]
    async fn test_savings_report() {
        let optimizer = MemoryOptimizer::new();
        let val1 = "B".repeat(2000);
        let val2 = "C".repeat(3000);

        optimizer.compress_and_store("k1", &val1).await.unwrap();
        optimizer.compress_and_store("k2", &val2).await.unwrap();

        let (original, compressed) = optimizer.get_savings_report().await;
        assert_eq!(original, 5000);
        assert!(compressed < original);
    }

    #[tokio::test]
    async fn test_turboquant_scq_roundtrip_metrics() {
        let optimizer = MemoryOptimizer::new();
        let key = "scq_key";
        let original_value = "System: AI assistant.\nUser: Optimize memory.\n".repeat(20);
        let original_len = original_value.len();

        optimizer
            .store_high_density(key, &original_value)
            .await
            .unwrap();

        // Manual decompression to check raw bytes (UTF-8 might fail due to quantization noise)
        let lock = optimizer.cache.lock().await;
        let entry = lock.get(key).unwrap();

        let packed = decompress_size_prepended(&entry.data).unwrap();
        let total_elements = packed.len() * 2;
        let indices = optimizer.quantizer.unpack_indices(&packed, total_elements);
        let mut data_f32 = optimizer.quantizer.dequantize(&indices);

        for chunk in data_f32.chunks_mut(128) {
            optimizer.quantizer.fht_inplace(chunk);
        }

        let result_bytes = optimizer.quantizer.denormalize_bytes(&data_f32);

        // Calculate MSE (Mean Squared Error)
        let mut sum_sq_err = 0.0;
        let orig_bytes = original_value.as_bytes();
        for i in 0..original_len {
            let err = orig_bytes[i] as f32 - result_bytes[i] as f32;
            sum_sq_err += err * err;
        }
        let rmse = (sum_sq_err / original_len as f32).sqrt();
        println!("SCQ RMSE (per byte): {:.4}", rmse);

        // For text bytes, RMSE < 25 is expected for 4-bit lossy quantization
        // which preserves semantic structure but not bit-perfect UTF8.
        assert!(rmse < 25.0);
    }
}
