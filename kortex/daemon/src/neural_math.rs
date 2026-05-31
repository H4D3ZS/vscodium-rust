/// Neural Math Engine for Holographic Reduced Representations (HRR) and Surprise Analysis
/// Part of the .aim Sentient Singularity (Phase 6)
// Removed unused PI constant

pub const VECTOR_DIM: usize = 1536;

/// Performs Circular Convolution using the Fast Fourier Transform (FFT) equivalent.
/// This "smears" features of vector B into vector A to create a holographic bind.
pub fn circular_convolution(a: &[f32; VECTOR_DIM], b: &[f32; VECTOR_DIM]) -> [f32; VECTOR_DIM] {
    let mut result = [0.0; VECTOR_DIM];
    
    // Naive Circular Convolution (O(N^2)) - To be optimized with FFT in production
    for i in 0..VECTOR_DIM {
        let mut sum = 0.0;
        for j in 0..VECTOR_DIM {
            let k = (i + VECTOR_DIM - j) % VECTOR_DIM;
            sum += a[j] * b[k];
        }
        result[i] = sum;
    }
    
    result
}

/// Calculate the "Surprise" (Loss) of new information compared to the existing memory.
/// MIRAS Framework: Loss = ||V_new - V_old||^2
pub fn calculate_surprise(current: &[f32], incoming: &[f32; VECTOR_DIM]) -> f32 {
    let mut sum_sq_diff = 0.0;
    for i in 0..VECTOR_DIM {
        let diff = incoming[i] - current[i];
        sum_sq_diff += diff * diff;
    }
    sum_sq_diff.sqrt()
}

/// Circular Correlation (Inverse of Convolution) used for Unbinding / Retrieval
pub fn circular_correlation(a: &[f32; VECTOR_DIM], b: &[f32; VECTOR_DIM]) -> [f32; VECTOR_DIM] {
    let mut result = [0.0; VECTOR_DIM];
    for i in 0..VECTOR_DIM {
        let mut sum = 0.0;
        for j in 0..VECTOR_DIM {
            let k = (i + j) % VECTOR_DIM;
            sum += a[j] * b[k];
        }
        result[i] = sum;
    }
    result
}
