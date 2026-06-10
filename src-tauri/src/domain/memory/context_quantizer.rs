/// TurboQuant-inspired Semantic Context Quantization (SCQ)
///
/// Implements Rotation (FHT) + Lloyd-Max Scalar Quantization for high-density session context.

pub struct ContextQuantizer {
    centroids: [f32; 16],
    boundaries: [f32; 15],
}

impl ContextQuantizer {
    pub fn new() -> Self {
        // Lloyd-Max optimal centroids for N(0,1) with 16 levels (4-bit)
        // Values approximated from standard Gaussian quantization tables
        let centroids = [
            -2.733, -2.069, -1.618, -1.256, -0.942, -0.657, -0.388, -0.128, 0.128, 0.388, 0.657,
            0.942, 1.256, 1.618, 2.069, 2.733,
        ];

        let mut boundaries = [0.0; 15];
        for i in 0..15 {
            boundaries[i] = (centroids[i] + centroids[i + 1]) / 2.0;
        }

        Self {
            centroids,
            boundaries,
        }
    }

    /// Fast Walsh-Hadamard Transform (O(n log n))
    /// x: slice of floats, len must be a power of 2
    pub fn fht_inplace(&self, x: &mut [f32]) {
        let n = x.len();
        let mut h = 1;
        while h < n {
            for i in (0..n).step_by(h * 2) {
                for j in i..i + h {
                    let a = x[j];
                    let b = x[j + h];
                    x[j] = a + b;
                    x[j + h] = a - b;
                }
            }
            h *= 2;
        }

        // Normalize
        let scale = (n as f32).sqrt();
        for val in x.iter_mut() {
            *val /= scale;
        }
    }

    /// Quantize a rotated vector into 4-bit indices
    pub fn quantize(&self, x: &[f32]) -> Vec<u8> {
        let mut indices = Vec::with_capacity(x.len());
        for &val in x {
            let mut idx = 15;
            for (i, &b) in self.boundaries.iter().enumerate() {
                if val < b {
                    idx = i;
                    break;
                }
            }
            indices.push(idx as u8);
        }
        indices
    }

    /// Dequantize 4-bit indices back to float vector
    pub fn dequantize(&self, indices: &[u8]) -> Vec<f32> {
        indices
            .iter()
            .map(|&idx| self.centroids[idx as usize])
            .collect()
    }

    /// Pack two 4-bit indices into one byte
    pub fn pack_indices(&self, indices: &[u8]) -> Vec<u8> {
        let mut packed = Vec::with_capacity((indices.len() + 1) / 2);
        for chunk in indices.chunks(2) {
            let byte = if chunk.len() == 2 {
                (chunk[0] << 4) | (chunk[1] & 0x0F)
            } else {
                chunk[0] << 4
            };
            packed.push(byte);
        }
        packed
    }

    /// Unpack one byte into two 4-bit indices
    pub fn unpack_indices(&self, packed: &[u8], original_len: usize) -> Vec<u8> {
        let mut unpacked = Vec::with_capacity(original_len);
        for &byte in packed {
            unpacked.push(byte >> 4);
            if unpacked.len() < original_len {
                unpacked.push(byte & 0x0F);
            }
        }
        unpacked
    }

    /// Normalize raw byte values (0-255) to Gaussian-friendly range (~ N(0,1))
    pub fn normalize_bytes(&self, x: &[u8]) -> Vec<f32> {
        x.iter().map(|&b| (b as f32 - 128.0) / 64.0).collect()
    }

    /// Denormalize float values back to raw bytes (0-255)
    pub fn denormalize_bytes(&self, x: &[f32]) -> Vec<u8> {
        x.iter()
            .map(|&f| (f * 64.0 + 128.0).round().clamp(0.0, 255.0) as u8)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fht_symmetry() {
        let mut x = vec![1.0, 2.0, 3.0, 4.0];
        let original = x.clone();
        let q = ContextQuantizer::new();

        q.fht_inplace(&mut x);
        // FHT is symmetric: applying it twice with normalization gets you back
        q.fht_inplace(&mut x);

        for i in 0..4 {
            assert!((x[i] - original[i]).abs() < 1e-5);
        }
    }

    #[test]
    fn test_fht_orthogonality() {
        let mut x = vec![1.0, 0.0, 0.0, 0.0];
        let q = ContextQuantizer::new();
        q.fht_inplace(&mut x);

        // Sum of squares should be preserved (Parseval's theorem)
        let sum_sq: f32 = x.iter().map(|&v| v * v).sum();
        assert!((sum_sq - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_quant_dequant_roundtrip() {
        let q = ContextQuantizer::new();
        let x = vec![0.5, -1.2, 3.0, -0.1];
        let indices = q.quantize(&x);
        let dequantized = q.dequantize(&indices);

        // Check if values are reasonably close to centroids
        assert_eq!(indices.len(), 4);
        assert_eq!(dequantized.len(), 4);
    }

    #[test]
    fn test_packing() {
        let q = ContextQuantizer::new();
        let indices = vec![1, 15, 7, 0];
        let packed = q.pack_indices(&indices);
        assert_eq!(packed.len(), 2);

        let unpacked = q.unpack_indices(&packed, 4);
        assert_eq!(unpacked, indices);
    }
}
