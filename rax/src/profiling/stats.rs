//! Core statistics data structures for instruction profiling.

use serde::Serialize;

/// Unique identifier for instruction variants, used for indexing statistics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OpcodeKey {
    /// Single-byte legacy opcode (0x00-0xFF, excluding escape bytes)
    Legacy(u8),
    /// Two-byte opcode (0x0F XX)
    TwoByte(u8),
    /// Three-byte opcode (0x0F 0x38 XX)
    ThreeByte38(u8),
    /// Three-byte opcode (0x0F 0x3A XX)
    ThreeByte3A(u8),
    /// VEX-encoded instruction (map, opcode)
    Vex { map: u8, opcode: u8 },
    /// EVEX-encoded instruction (map, opcode)
    Evex { map: u8, opcode: u8 },
    /// x87 FPU instruction (escape byte D8-DF, modrm high 3 bits for register ops)
    Fpu { escape: u8, modrm_reg: u8 },
}

impl OpcodeKey {
    /// Maximum index value (determines array size for stats storage)
    pub const MAX_INDEX: usize = 2048;

    /// Convert opcode key to a unique array index.
    ///
    /// Index layout:
    /// - 0-255: Legacy single-byte opcodes
    /// - 256-511: Two-byte opcodes (0x0F XX)
    /// - 512-767: Three-byte 0x0F 0x38 XX
    /// - 768-1023: Three-byte 0x0F 0x3A XX
    /// - 1024-1279: VEX map 1 (0F)
    /// - 1280-1535: VEX map 2 (0F38)
    /// - 1536-1791: VEX map 3 (0F3A)
    /// - 1792-1855: FPU D8-DF with 8 modrm_reg values each (64 total)
    /// - 1856-2047: EVEX (reserved space)
    #[inline]
    pub fn to_index(&self) -> usize {
        match self {
            OpcodeKey::Legacy(op) => *op as usize,
            OpcodeKey::TwoByte(op) => 256 + *op as usize,
            OpcodeKey::ThreeByte38(op) => 512 + *op as usize,
            OpcodeKey::ThreeByte3A(op) => 768 + *op as usize,
            OpcodeKey::Vex { map, opcode } => {
                let base = match map {
                    1 => 1024,
                    2 => 1280,
                    3 => 1536,
                    _ => 1024, // Default to map 1
                };
                base + *opcode as usize
            }
            OpcodeKey::Fpu { escape, modrm_reg } => {
                // D8=0, D9=1, ..., DF=7
                let esc_idx = (*escape - 0xD8) as usize;
                1792 + esc_idx * 8 + (*modrm_reg & 0x7) as usize
            }
            OpcodeKey::Evex { map, opcode } => {
                // Use remaining space for EVEX
                let base = 1856;
                let map_offset = (*map as usize).min(3) * 64;
                base + map_offset + (*opcode as usize & 0x3F)
            }
        }
    }

    /// Reconstruct an OpcodeKey from an array index (for reporting).
    pub fn from_index(idx: usize) -> Option<Self> {
        match idx {
            0..=255 => Some(OpcodeKey::Legacy(idx as u8)),
            256..=511 => Some(OpcodeKey::TwoByte((idx - 256) as u8)),
            512..=767 => Some(OpcodeKey::ThreeByte38((idx - 512) as u8)),
            768..=1023 => Some(OpcodeKey::ThreeByte3A((idx - 768) as u8)),
            1024..=1279 => Some(OpcodeKey::Vex {
                map: 1,
                opcode: (idx - 1024) as u8,
            }),
            1280..=1535 => Some(OpcodeKey::Vex {
                map: 2,
                opcode: (idx - 1280) as u8,
            }),
            1536..=1791 => Some(OpcodeKey::Vex {
                map: 3,
                opcode: (idx - 1536) as u8,
            }),
            1792..=1855 => {
                let rel = idx - 1792;
                let esc_idx = rel / 8;
                let modrm_reg = (rel % 8) as u8;
                Some(OpcodeKey::Fpu {
                    escape: 0xD8 + esc_idx as u8,
                    modrm_reg,
                })
            }
            1856..=2047 => {
                let rel = idx - 1856;
                let map = (rel / 64) as u8;
                let opcode = (rel % 64) as u8;
                Some(OpcodeKey::Evex { map, opcode })
            }
            _ => None,
        }
    }

    /// Format the opcode key as a human-readable string.
    pub fn format(&self) -> String {
        match self {
            OpcodeKey::Legacy(op) => format!("{:02X}", op),
            OpcodeKey::TwoByte(op) => format!("0F {:02X}", op),
            OpcodeKey::ThreeByte38(op) => format!("0F 38 {:02X}", op),
            OpcodeKey::ThreeByte3A(op) => format!("0F 3A {:02X}", op),
            OpcodeKey::Vex { map, opcode } => format!("VEX.{} {:02X}", map, opcode),
            OpcodeKey::Evex { map, opcode } => format!("EVEX.{} {:02X}", map, opcode),
            OpcodeKey::Fpu { escape, modrm_reg } => {
                format!("{:02X} /{}", escape, modrm_reg)
            }
        }
    }
}

/// Memory access context for a single instruction execution.
#[derive(Default, Clone, Copy)]
pub struct MemoryAccessStats {
    pub reads: u32,
    pub writes: u32,
    pub bytes_read: u32,
    pub bytes_written: u32,
}

impl MemoryAccessStats {
    #[inline(always)]
    pub fn record_read(&mut self, size: usize) {
        self.reads += 1;
        self.bytes_read += size as u32;
    }

    #[inline(always)]
    pub fn record_write(&mut self, size: usize) {
        self.writes += 1;
        self.bytes_written += size as u32;
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Statistics for a single opcode, using Welford's online algorithm
/// for numerically stable mean and variance computation.
#[derive(Clone)]
pub struct OpcodeStats {
    /// Total execution count
    pub count: u64,
    /// Total elapsed time in nanoseconds
    pub total_nanos: u64,
    /// Minimum latency observed (nanoseconds)
    pub min_nanos: u64,
    /// Maximum latency observed (nanoseconds)
    pub max_nanos: u64,
    /// Running mean for Welford's algorithm
    mean: f64,
    /// Running M2 for Welford's algorithm (sum of squared differences)
    m2: f64,
    /// Total memory read operations
    pub mem_reads: u64,
    /// Total memory write operations
    pub mem_writes: u64,
    /// Total bytes read
    pub bytes_read: u64,
    /// Total bytes written
    pub bytes_written: u64,
    /// Latency histogram buckets for percentile estimation.
    /// Buckets: [0-100ns, 100-500ns, 500-1000ns, 1-5us, 5-10us, 10-50us, 50-100us, 100us+]
    pub latency_buckets: [u64; 8],
}

impl Default for OpcodeStats {
    fn default() -> Self {
        Self {
            count: 0,
            total_nanos: 0,
            min_nanos: u64::MAX,
            max_nanos: 0,
            mean: 0.0,
            m2: 0.0,
            mem_reads: 0,
            mem_writes: 0,
            bytes_read: 0,
            bytes_written: 0,
            latency_buckets: [0; 8],
        }
    }
}

impl OpcodeStats {
    /// Record a new sample using Welford's online algorithm.
    #[inline]
    pub fn record(&mut self, latency_nanos: u64, mem: &MemoryAccessStats) {
        self.count += 1;
        self.total_nanos += latency_nanos;

        // Min/Max
        if latency_nanos < self.min_nanos {
            self.min_nanos = latency_nanos;
        }
        if latency_nanos > self.max_nanos {
            self.max_nanos = latency_nanos;
        }

        // Welford's online algorithm for mean and variance
        let x = latency_nanos as f64;
        let delta = x - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;

        // Histogram bucket
        let bucket = Self::latency_to_bucket(latency_nanos);
        self.latency_buckets[bucket] += 1;

        // Memory access stats
        self.mem_reads += mem.reads as u64;
        self.mem_writes += mem.writes as u64;
        self.bytes_read += mem.bytes_read as u64;
        self.bytes_written += mem.bytes_written as u64;
    }

    /// Get the computed mean latency in nanoseconds.
    #[inline]
    pub fn mean_nanos(&self) -> f64 {
        self.mean
    }

    /// Compute the sample variance.
    #[inline]
    pub fn variance(&self) -> f64 {
        if self.count < 2 {
            0.0
        } else {
            self.m2 / (self.count - 1) as f64
        }
    }

    /// Compute the sample standard deviation.
    #[inline]
    pub fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }

    /// Compute the coefficient of variation (stddev / mean).
    #[inline]
    pub fn cv(&self) -> f64 {
        if self.mean > 0.0 {
            self.stddev() / self.mean
        } else {
            0.0
        }
    }

    /// Count samples that are outliers (beyond 3 standard deviations).
    /// This is an estimate based on the histogram.
    pub fn outlier_count(&self) -> u64 {
        let threshold = self.mean + 3.0 * self.stddev();
        let threshold_bucket = Self::latency_to_bucket(threshold as u64);
        // Sum all buckets above the threshold bucket
        self.latency_buckets[threshold_bucket..].iter().sum::<u64>()
    }

    /// Estimate a percentile from the histogram.
    /// Returns the bucket midpoint for the bucket containing the percentile.
    pub fn percentile(&self, p: f64) -> u64 {
        if self.count == 0 {
            return 0;
        }

        let target = ((self.count as f64) * p).ceil() as u64;
        let mut cumulative = 0u64;

        // Bucket midpoints in nanoseconds
        const BUCKET_MIDPOINTS: [u64; 8] = [50, 300, 750, 3000, 7500, 30000, 75000, 150000];

        for (i, &count) in self.latency_buckets.iter().enumerate() {
            cumulative += count;
            if cumulative >= target {
                return BUCKET_MIDPOINTS[i];
            }
        }

        BUCKET_MIDPOINTS[7]
    }

    /// Get the median (p50).
    #[inline]
    pub fn median(&self) -> u64 {
        self.percentile(0.50)
    }

    /// Get the 90th percentile.
    #[inline]
    pub fn p90(&self) -> u64 {
        self.percentile(0.90)
    }

    /// Get the 99th percentile.
    #[inline]
    pub fn p99(&self) -> u64 {
        self.percentile(0.99)
    }

    /// Map a latency value to its histogram bucket index.
    #[inline]
    fn latency_to_bucket(nanos: u64) -> usize {
        match nanos {
            0..=99 => 0,
            100..=499 => 1,
            500..=999 => 2,
            1000..=4999 => 3,
            5000..=9999 => 4,
            10000..=49999 => 5,
            50000..=99999 => 6,
            _ => 7,
        }
    }
}

/// Serializable instruction statistics for JSON export.
#[derive(Serialize)]
pub struct InstructionReport {
    pub opcode: String,
    pub mnemonic: String,
    pub count: u64,
    pub total_time_ns: u64,
    pub min_latency_ns: u64,
    pub max_latency_ns: u64,
    pub mean_latency_ns: f64,
    pub median_latency_ns: u64,
    pub stddev_ns: f64,
    pub cv: f64,
    pub p50_ns: u64,
    pub p90_ns: u64,
    pub p99_ns: u64,
    pub outlier_count: u64,
    pub mem_reads: u64,
    pub mem_writes: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub latency_histogram: [u64; 8],
}

impl InstructionReport {
    pub fn from_stats(key: &OpcodeKey, stats: &OpcodeStats, mnemonic: &str) -> Self {
        Self {
            opcode: key.format(),
            mnemonic: mnemonic.to_string(),
            count: stats.count,
            total_time_ns: stats.total_nanos,
            min_latency_ns: if stats.count > 0 { stats.min_nanos } else { 0 },
            max_latency_ns: stats.max_nanos,
            mean_latency_ns: stats.mean_nanos(),
            median_latency_ns: stats.median(),
            stddev_ns: stats.stddev(),
            cv: stats.cv(),
            p50_ns: stats.percentile(0.50),
            p90_ns: stats.percentile(0.90),
            p99_ns: stats.percentile(0.99),
            outlier_count: stats.outlier_count(),
            mem_reads: stats.mem_reads,
            mem_writes: stats.mem_writes,
            bytes_read: stats.bytes_read,
            bytes_written: stats.bytes_written,
            latency_histogram: stats.latency_buckets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_key_index_roundtrip() {
        // Test legacy opcodes
        for op in 0..=255u8 {
            let key = OpcodeKey::Legacy(op);
            let idx = key.to_index();
            assert!(idx < OpcodeKey::MAX_INDEX);
            assert_eq!(OpcodeKey::from_index(idx), Some(key));
        }

        // Test two-byte opcodes
        for op in 0..=255u8 {
            let key = OpcodeKey::TwoByte(op);
            let idx = key.to_index();
            assert!(idx < OpcodeKey::MAX_INDEX);
            assert_eq!(OpcodeKey::from_index(idx), Some(key));
        }

        // Test FPU opcodes
        for esc in 0xD8..=0xDFu8 {
            for modrm_reg in 0..8u8 {
                let key = OpcodeKey::Fpu {
                    escape: esc,
                    modrm_reg,
                };
                let idx = key.to_index();
                assert!(idx < OpcodeKey::MAX_INDEX);
                assert_eq!(OpcodeKey::from_index(idx), Some(key));
            }
        }
    }

    #[test]
    fn test_welford_algorithm() {
        let mut stats = OpcodeStats::default();
        let mem = MemoryAccessStats::default();

        // Record some samples
        let samples = [100, 120, 110, 130, 90];
        for &s in &samples {
            stats.record(s, &mem);
        }

        assert_eq!(stats.count, 5);

        // Mean should be 110
        let expected_mean = 110.0;
        assert!((stats.mean_nanos() - expected_mean).abs() < 0.001);

        // Variance = sum((x - mean)^2) / (n-1)
        // = (100 + 100 + 0 + 400 + 400) / 4 = 250
        let expected_var = 250.0;
        assert!((stats.variance() - expected_var).abs() < 0.001);
    }

    #[test]
    fn test_min_max() {
        let mut stats = OpcodeStats::default();
        let mem = MemoryAccessStats::default();

        stats.record(100, &mem);
        stats.record(50, &mem);
        stats.record(200, &mem);

        assert_eq!(stats.min_nanos, 50);
        assert_eq!(stats.max_nanos, 200);
    }
}
