//! Pure-Rust port of `gac/theory.py` from the geometry-of-consolidation paper.
//!
//! Three quantities, plus the bound that ties them together:
//!
//!   d_bar(C)        = mean within-cluster cosine distance
//!   d_eff(C)        = (sum lambda_i)^2 / sum lambda_i^2  on the cluster covariance
//!   rho(C)          = lambda_max / sum lambda_i           on the cluster covariance
//!   eps_id(C, r)   >= 1 - c1 * (theta'/d_bar)^d_eff       (Theorem §2.1)
//!
//! All inputs are rows of a sampled weight matrix that have been L2-normalised.
//! Sample sizes are kept small (<= 512) so we can use a tiny cyclic Jacobi
//! eigensolver instead of pulling in LAPACK or nalgebra.

const JACOBI_MAX_SWEEPS: usize = 64;
const JACOBI_TOL: f32 = 1e-7;

/// L2-normalise a row in place. Used aggressively because all GAC math assumes
/// unit-norm rows (cosine geometry).
#[inline]
pub fn l2_normalize(row: &mut [f32]) {
    let mut s = 0.0f32;
    for &v in row.iter() {
        s += v * v;
    }
    let inv = 1.0f32 / (s.sqrt().max(1e-12));
    for v in row.iter_mut() {
        *v *= inv;
    }
}

pub fn l2_normalize_rows(rows: &mut [Vec<f32>]) {
    for r in rows.iter_mut() {
        l2_normalize(r);
    }
}

/// Mean within-cluster cosine distance (paper notation: d̄).
///
/// Assumes `rows` are L2-normalised. Returns 0.0 for clusters with < 2 rows.
pub fn cluster_spread(rows: &[Vec<f32>]) -> f32 {
    let n = rows.len();
    if n < 2 {
        return 0.0;
    }
    let dim = rows[0].len();
    let mut sim_sum = 0.0f64;
    let mut count = 0u64;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut s = 0.0f32;
            for k in 0..dim {
                s += rows[i][k] * rows[j][k];
            }
            sim_sum += s as f64;
            count += 1;
        }
    }
    let mean_sim = (sim_sum / count as f64) as f32;
    1.0 - mean_sim
}

/// Build the n×n Gram matrix of mean-centered rows: G_ij = <r_i - mu, r_j - mu>.
///
/// Eigenvalues of (G / (n-1)) match the non-zero eigenvalues of the d×d sample
/// covariance, so we can do all spectral work in n-space when n < d.
fn centered_gram(rows: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let n = rows.len();
    if n == 0 {
        return Vec::new();
    }
    let dim = rows[0].len();
    let mut mu = vec![0.0f32; dim];
    for r in rows.iter() {
        for k in 0..dim {
            mu[k] += r[k];
        }
    }
    let inv_n = 1.0 / n as f32;
    for v in mu.iter_mut() {
        *v *= inv_n;
    }

    let mut centered: Vec<Vec<f32>> = rows
        .iter()
        .map(|r| {
            let mut c = r.clone();
            for k in 0..dim {
                c[k] -= mu[k];
            }
            c
        })
        .collect();

    // Build symmetric Gram. We don't divide by (n-1) here; that's a uniform
    // rescaling that doesn't affect d_eff or rho.
    let mut g = vec![vec![0.0f32; n]; n];
    for i in 0..n {
        for j in i..n {
            let mut s = 0.0f32;
            for k in 0..dim {
                s += centered[i][k] * centered[j][k];
            }
            g[i][j] = s;
            g[j][i] = s;
        }
    }
    // Free the temporary centered rows.
    centered.clear();
    g
}

/// Cyclic Jacobi eigendecomposition of a symmetric matrix. Returns eigenvalues
/// sorted descending.
///
/// O(n^3) per sweep, converges quadratically once below threshold. n ≤ 512 in our
/// use is comfortable: ~130 ms in release on a desktop CPU. We don't need
/// eigenvectors so we skip the rotation tracking.
fn jacobi_eigenvalues(mut a: Vec<Vec<f32>>) -> Vec<f32> {
    let n = a.len();
    if n == 0 {
        return Vec::new();
    }

    for _sweep in 0..JACOBI_MAX_SWEEPS {
        // Off-diagonal Frobenius norm; bail when small.
        let mut off = 0.0f32;
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    off += a[i][j] * a[i][j];
                }
            }
        }
        if off.sqrt() < JACOBI_TOL {
            break;
        }

        for p in 0..(n - 1) {
            for q in (p + 1)..n {
                let apq = a[p][q];
                if apq.abs() < 1e-12 {
                    continue;
                }
                let app = a[p][p];
                let aqq = a[q][q];
                let theta = (aqq - app) / (2.0 * apq);
                let t = if theta >= 0.0 {
                    1.0 / (theta + (1.0 + theta * theta).sqrt())
                } else {
                    1.0 / (theta - (1.0 + theta * theta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;

                a[p][p] = app - t * apq;
                a[q][q] = aqq + t * apq;
                a[p][q] = 0.0;
                a[q][p] = 0.0;

                for r in 0..n {
                    if r == p || r == q {
                        continue;
                    }
                    let arp = a[r][p];
                    let arq = a[r][q];
                    a[r][p] = c * arp - s * arq;
                    a[p][r] = a[r][p];
                    a[r][q] = s * arp + c * arq;
                    a[q][r] = a[r][q];
                }
            }
        }
    }

    let mut eigs: Vec<f32> = (0..n).map(|i| a[i][i]).collect();
    eigs.sort_by(|x, y| y.partial_cmp(x).unwrap_or(std::cmp::Ordering::Equal));
    eigs
}

/// Effective dimensionality via participation ratio: PR = (Σλ)² / Σλ².
///
/// When the cluster has fewer rows than feature dimensions (the common case for
/// our tiny samples) we operate on the n×n Gram matrix; eigenvalues match the
/// non-zero spectrum of the d×d covariance.
pub fn d_eff(rows: &[Vec<f32>]) -> f32 {
    if rows.len() < 2 {
        return 0.0;
    }
    let g = centered_gram(rows);
    let eigs = jacobi_eigenvalues(g);
    let mut sum = 0.0f64;
    let mut sum_sq = 0.0f64;
    for &e in eigs.iter() {
        let e = e.max(0.0) as f64;
        sum += e;
        sum_sq += e * e;
    }
    if sum_sq <= 0.0 {
        return 0.0;
    }
    ((sum * sum) / sum_sq) as f32
}

/// Spectral concentration ρ = λ_max / Σλ.
///
/// ρ → 1 means the cluster is near rank-1 — collapsing to the centroid is safe.
/// ρ → 1/d_eff means the cluster is isotropic — every direction matters.
pub fn rho_cluster(rows: &[Vec<f32>]) -> f32 {
    if rows.len() < 2 {
        return 1.0;
    }
    let g = centered_gram(rows);
    let eigs = jacobi_eigenvalues(g);
    let mut sum = 0.0f64;
    let mut max_eig = 0.0f64;
    for &e in eigs.iter() {
        let e = e.max(0.0) as f64;
        sum += e;
        if e > max_eig {
            max_eig = e;
        }
    }
    if sum <= 0.0 {
        return 1.0;
    }
    (max_eig / sum) as f32
}

/// The Consolidation-Interference identity-error lower bound (eq. 2.1 of the paper).
///
///   eps_id >= 1 - c1 * (theta' / d_bar)^d_eff
///
/// Returns 0 when the cluster fully fits inside the cap (theta' >= d_bar).
pub fn spectral_bound(d_bar: f32, theta: f32, d_eff_val: f32, c1: f32) -> f32 {
    if d_bar <= 0.0 {
        return 0.0;
    }
    let theta_prime = (1.0 - theta).max(0.0);
    if theta_prime >= d_bar {
        return 0.0;
    }
    let ratio = theta_prime / d_bar;
    let bound = 1.0 - c1 * ratio.powf(d_eff_val);
    bound.clamp(0.0, 1.0)
}

/// d_bar_critical: the threshold separating tight from spread regimes.
///
/// Derived in §7 of the paper:
///   d_bar_critical = (1 - theta) * 2^(1/d_eff)
///
/// Below this, identity is forced to be cheap; above this, every consolidator
/// trades errors along the same geometric axis.
pub fn d_bar_critical(theta: f32, d_eff_global: f32) -> f32 {
    let theta_prime = (1.0 - theta).max(1e-3);
    let d = d_eff_global.max(1.0);
    theta_prime * 2.0f32.powf(1.0 / d)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tight() -> Vec<Vec<f32>> {
        // 8 rows that are tiny perturbations of (1, 0, 0, ..) — high rho, low d_eff.
        let dim = 16;
        let mut rows = Vec::new();
        for i in 0..8 {
            let mut r = vec![0.0f32; dim];
            r[0] = 1.0;
            r[1] = 0.001 * i as f32;
            l2_normalize(&mut r);
            rows.push(r);
        }
        rows
    }

    fn make_spread() -> Vec<Vec<f32>> {
        // 8 nearly-orthogonal rows — low rho, high d_eff.
        let dim = 16;
        let mut rows = Vec::new();
        for i in 0..8 {
            let mut r = vec![0.0f32; dim];
            r[i] = 1.0;
            l2_normalize(&mut r);
            rows.push(r);
        }
        rows
    }

    #[test]
    fn tight_cluster_has_low_dbar() {
        let rows = make_tight();
        let d_bar = cluster_spread(&rows);
        assert!(d_bar < 0.05, "expected low d_bar, got {}", d_bar);
    }

    #[test]
    fn spread_cluster_has_high_dbar() {
        let rows = make_spread();
        let d_bar = cluster_spread(&rows);
        assert!(d_bar > 0.5, "expected high d_bar, got {}", d_bar);
    }

    #[test]
    fn tight_cluster_has_high_rho() {
        let rows = make_tight();
        let rho = rho_cluster(&rows);
        assert!(rho > 0.9, "expected rho near 1, got {}", rho);
    }

    #[test]
    fn spread_cluster_has_low_rho() {
        let rows = make_spread();
        let rho = rho_cluster(&rows);
        // 8 orthogonal rows -> rho should be near 1/7 (one eigenvalue per non-mean axis).
        assert!(rho < 0.30, "expected rho < 0.30, got {}", rho);
    }

    #[test]
    fn tight_cluster_has_low_deff() {
        let rows = make_tight();
        let de = d_eff(&rows);
        assert!(de < 2.0, "expected d_eff < 2, got {}", de);
    }

    #[test]
    fn spread_cluster_has_high_deff() {
        let rows = make_spread();
        let de = d_eff(&rows);
        assert!(de > 5.0, "expected d_eff > 5, got {}", de);
    }

    #[test]
    fn spectral_bound_saturates_inside_cap() {
        let b = spectral_bound(0.05, 0.85, 4.0, 1.0);
        assert_eq!(b, 0.0);
    }

    #[test]
    fn spectral_bound_grows_outside_cap() {
        let b = spectral_bound(0.5, 0.85, 4.0, 1.0);
        assert!(b > 0.0 && b <= 1.0);
    }

    #[test]
    fn d_bar_critical_matches_paper_formula() {
        // theta = 0.85, d_eff = 4 -> theta' = 0.15, 2^(1/4) ≈ 1.189
        let c = d_bar_critical(0.85, 4.0);
        assert!((c - 0.15 * 1.189207115).abs() < 1e-3, "got {}", c);
    }
}
