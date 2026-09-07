//! Ancient arithmetic, measured. The Rhind papyrus (multiply = doubling+adding),
//! Napier's logs (multiply -> add), and BitNet-1.58 are the same move: when
//! values are {-1,0,+1}, a dot product has NO multiplies — it collapses to
//! popcounts on packed sign-bits. This benchmarks that on the libaim retrieval
//! shape (dim-768 unit vectors) against the AVX2+FMA f32 kernel we ship.
//!
//! The finding (build: `rustc -O bench.rs -o bench && ./bench`):
//!   * The ARITHMETIC win is real and data-independent: ternary popcount is
//!     ~2.7x faster and 16x smaller than f32 (the 3 MB ternary corpus lives in
//!     cache; the 58 MB f32 corpus does not — the win is mostly BANDWIDTH).
//!   * Whether it holds RECALL depends entirely on the data distribution:
//!       - isotropic random (the known worst case for sign methods): fails.
//!       - anisotropic / power-law spectrum (how real embeddings look): the
//!         ternary top-50 contains ~99.8% of the exact top-10.
//!   * So the working architecture is COARSE-TO-FINE: ternary popcount scan of
//!     everything (cache-resident) -> exact f32 AVX2 rerank of the top ~50.
//!     Same shape as turbovec's quantized-scan-then-rerank; ternary is a
//!     viable, extremely cheap coarse tier.
//!
//! Caveat: synthetic vectors. Final validation needs real .aim embeddings.
extern crate core;
use std::time::Instant;
use std::arch::x86_64::*;
use std::collections::HashSet;

#[target_feature(enable = "avx2", enable = "fma")]
unsafe fn dot_f32(a: &[f32], b: &[f32]) -> f32 {
    let n=a.len(); let(pa,pb)=(a.as_ptr(),b.as_ptr());
    let(mut s0,mut s1,mut s2,mut s3)=(_mm256_setzero_ps(),_mm256_setzero_ps(),_mm256_setzero_ps(),_mm256_setzero_ps());
    let mut i=0;
    while i+32<=n{ s0=_mm256_fmadd_ps(_mm256_loadu_ps(pa.add(i)),_mm256_loadu_ps(pb.add(i)),s0);
        s1=_mm256_fmadd_ps(_mm256_loadu_ps(pa.add(i+8)),_mm256_loadu_ps(pb.add(i+8)),s1);
        s2=_mm256_fmadd_ps(_mm256_loadu_ps(pa.add(i+16)),_mm256_loadu_ps(pb.add(i+16)),s2);
        s3=_mm256_fmadd_ps(_mm256_loadu_ps(pa.add(i+24)),_mm256_loadu_ps(pb.add(i+24)),s3); i+=32; }
    while i+8<=n{ s0=_mm256_fmadd_ps(_mm256_loadu_ps(pa.add(i)),_mm256_loadu_ps(pb.add(i)),s0); i+=8; }
    let v=_mm256_add_ps(_mm256_add_ps(s0,s1),_mm256_add_ps(s2,s3));
    let hi=_mm256_extractf128_ps(v,1); let lo=_mm256_castps256_ps128(v);
    let h=_mm_hadd_ps(_mm_add_ps(hi,lo),_mm_add_ps(hi,lo)); let h=_mm_hadd_ps(h,h);
    let mut s=_mm_cvtss_f32(h); while i<n{ s+=*a.get_unchecked(i)* *b.get_unchecked(i); i+=1;} s
}

struct Tern{pos:Vec<u64>,neg:Vec<u64>}
fn ternarize(v:&[f32],tau:f32)->Tern{
    let w=(v.len()+63)/64; let(mut p,mut n)=(vec![0u64;w],vec![0u64;w]);
    for(i,&x)in v.iter().enumerate(){ if x>tau{p[i>>6]|=1u64<<(i&63);} else if x< -tau{n[i>>6]|=1u64<<(i&63);} }
    Tern{pos:p,neg:n}
}
fn tdot(a:&Tern,b:&Tern)->i32{
    let(mut ag,mut di)=(0u32,0u32);
    for w in 0..a.pos.len(){ ag+=(a.pos[w]&b.pos[w]).count_ones()+(a.neg[w]&b.neg[w]).count_ones();
        di+=(a.pos[w]&b.neg[w]).count_ones()+(a.neg[w]&b.pos[w]).count_ones(); }
    ag as i32 - di as i32
}
fn rnd(s:&mut u64)->f32{*s=s.wrapping_mul(6364136223846793005).wrapping_add(1);((*s>>33)as f32/u32::MAX as f32)*2.0-1.0}
fn normalize(v:&mut [f32]){let n=v.iter().map(|x|x*x).sum::<f32>().sqrt();if n>0.0{for x in v.iter_mut(){*x/=n;}}}
fn topk<F:Fn(usize)->f32>(n:usize,k:usize,f:F)->Vec<usize>{let mut v:Vec<(f32,usize)>=(0..n).map(|i|(f(i),i)).collect();v.sort_by(|a,b|b.0.partial_cmp(&a.0).unwrap());v.into_iter().take(k).map(|x|x.1).collect()}

fn funnel(name:&str, aniso:bool, dim:usize, n:usize, queries:usize, k:usize){
    let mut s=7u64;
    let scale:Vec<f32> = if aniso {(0..dim).map(|d|1.0/((d as f32+1.0).powf(0.7))).collect()} else {vec![1.0;dim]};
    let(ncl,jit)=(400usize,0.15f32);
    let mut ctr=vec![0f32;ncl*dim];
    for c in 0..ncl{ for d in 0..dim{ ctr[c*dim+d]=rnd(&mut s)*scale[d]; } normalize(&mut ctr[c*dim..(c+1)*dim]); }
    let mut corp=vec![0f32;n*dim];
    for i in 0..n{ let c=i%ncl; for d in 0..dim{ corp[i*dim+d]=ctr[c*dim+d]+jit*rnd(&mut s)*scale[d]; } normalize(&mut corp[i*dim..(i+1)*dim]); }
    let tau=0.3/(dim as f32).sqrt();
    let ct:Vec<Tern>=(0..n).map(|i|ternarize(&corp[i*dim..(i+1)*dim],tau)).collect();
    let cands=[10usize,20,50,100,200]; let mut rec=[0f64;5];
    for j in 0..queries{
        let c=(j*13)%ncl; let mut q=vec![0f32;dim]; for d in 0..dim{ q[d]=ctr[c*dim+d]+jit*rnd(&mut s)*scale[d]; } normalize(&mut q);
        let truth:HashSet<usize>=topk(n,k,|i|unsafe{dot_f32(&q,&corp[i*dim..(i+1)*dim])}).into_iter().collect();
        let qt=ternarize(&q,tau); let mut sc:Vec<(i32,usize)>=(0..n).map(|i|(tdot(&qt,&ct[i]),i)).collect(); sc.sort_by(|a,b|b.0.cmp(&a.0));
        for(ci,&cc)in cands.iter().enumerate(){ rec[ci]+=sc.iter().take(cc).filter(|(_,i)|truth.contains(i)).count() as f64/k as f64; }
    }
    print!("  {name:<28}");
    for(ci,&cc)in cands.iter().enumerate(){ print!("  top{cc}:{:.3}",rec[ci]/queries as f64); }
    println!();
}

fn main(){
    let(dim,n,queries,k)=(768usize,20000usize,200usize,10usize);
    println!("libaim retrieval shape: dim={dim} corpus={n} queries={queries}, exact top-{k} as ground truth\n");
    println!("FUNNEL RECALL (true top-{k} captured within ternary top-N):");
    funnel("isotropic (worst case)", false, dim,n,queries,k);
    funnel("anisotropic (realistic)", true, dim,n,queries,k);

    // speed + memory (anisotropic corpus)
    let mut s=99u64; let scale:Vec<f32>=(0..dim).map(|d|1.0/((d as f32+1.0).powf(0.7))).collect();
    let mut corp=vec![0f32;n*dim]; for x in corp.iter_mut(){*x=rnd(&mut s);} for i in 0..n{for d in 0..dim{corp[i*dim+d]*=scale[d];}normalize(&mut corp[i*dim..(i+1)*dim]);}
    let q:Vec<f32>=corp[..dim].to_vec();
    let tau=0.3/(dim as f32).sqrt(); let qt=ternarize(&q,tau);
    let ct:Vec<Tern>=(0..n).map(|i|ternarize(&corp[i*dim..(i+1)*dim],tau)).collect();
    let bench=|name:&str,f:&dyn Fn()->f64|{ let mut a=f(); let t=Instant::now(); for _ in 0..10{a+=f();} let el=t.elapsed().as_secs_f64(); println!("  {name:<12} {:6.1} ns/dot  (sink {:.0})",el/((10*n)as f64)*1e9,a); };
    println!("\nSCAN SPEED (dim {dim}, {n} rows):");
    bench("f32 AVX2", &||{ let mut a=0f64; for i in 0..n{a+=unsafe{dot_f32(&q,&corp[i*dim..(i+1)*dim])}as f64;} a });
    bench("ternary", &||{ let mut a=0f64; for i in 0..n{a+=tdot(&qt,&ct[i])as f64;} a });
    let fb=n*dim*4; let tb=n*((dim+63)/64)*8*2;
    println!("\nMEMORY: f32 {} MB -> ternary {} MB  ({:.0}x less; 3 MB fits L3, 58 MB does not)", fb/1048576, tb/1048576, fb as f64/tb as f64);
}
