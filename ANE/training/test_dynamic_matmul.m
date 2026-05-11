// test_dynamic_matmul.m — Benchmark dynamic matmul on ANE (no recompile)
// Layout: input [1, D, 1, S+D] — activations in sp[0:S], weight rows in sp[S:S+D]
// MIL: slice → reshape → matmul → reshape → output
#import <Foundation/Foundation.h>
#import <objc/runtime.h>
#import <objc/message.h>
#import <dlfcn.h>
#import <IOSurface/IOSurface.h>
#import <mach/mach_time.h>
#include <arm_neon.h>
#include <Accelerate/Accelerate.h>

#include "stories_io.h"

// Generate MIL for y = x @ W where both come from input IOSurface
// Input: [1, IC, 1, SEQ+OC] fp32
//   sp[0:SEQ]    = activations x[IC, SEQ]
//   sp[SEQ:SEQ+OC] = weight W[IC, OC] (each channel d holds W[d, :])
// Output: [1, OC, 1, SEQ] fp32
static NSString *gen_dynamic_matmul_mil(int ic, int oc, int seq) {
    NSMutableString *m = [NSMutableString string];
    [m appendString:@"program(1.3)\n"
        "[buildInfo = dict<string, string>({{\"coremlc-component-MIL\", \"3510.2.1\"}, "
        "{\"coremlc-version\", \"3505.4.1\"}, {\"coremltools-component-milinternal\", \"\"}, "
        "{\"coremltools-version\", \"9.0\"}})]\n{\n"];
    int sp_total = seq + oc;
    [m appendFormat:@"    func main<ios18>(tensor<fp32, [1, %d, 1, %d]> x) {\n", ic, sp_total];
    // Cast to fp16
    [m appendString:@"        string to16 = const()[name = string(\"to16\"), val = string(\"fp16\")];\n"];
    [m appendFormat:@"        tensor<fp16, [1, %d, 1, %d]> xh = cast(dtype = to16, x = x)[name = string(\"cin\")];\n", ic, sp_total];
    // Slice activations [1, IC, 1, SEQ]
    [m appendString:@"        tensor<int32, [4]> ba = const()[name = string(\"ba\"), val = tensor<int32, [4]>([0,0,0,0])];\n"];
    [m appendFormat:@"        tensor<int32, [4]> sa = const()[name = string(\"sa\"), val = tensor<int32, [4]>([1,%d,1,%d])];\n", ic, seq];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> act = slice_by_size(x=xh,begin=ba,size=sa)[name=string(\"act\")];\n", ic, seq];
    // Slice weight [1, IC, 1, OC]
    [m appendFormat:@"        tensor<int32, [4]> bw = const()[name = string(\"bw\"), val = tensor<int32, [4]>([0,0,0,%d])];\n", seq];
    [m appendFormat:@"        tensor<int32, [4]> sw = const()[name = string(\"sw\"), val = tensor<int32, [4]>([1,%d,1,%d])];\n", ic, oc];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> wt = slice_by_size(x=xh,begin=bw,size=sw)[name=string(\"wt\")];\n", ic, oc];
    // Reshape act: [1,IC,1,SEQ] → [1,1,IC,SEQ] → transpose → [1,1,SEQ,IC]
    [m appendFormat:@"        tensor<int32, [4]> ra = const()[name = string(\"ra\"), val = tensor<int32, [4]>([1,1,%d,%d])];\n", ic, seq];
    [m appendFormat:@"        tensor<fp16, [1,1,%d,%d]> a2 = reshape(shape=ra,x=act)[name=string(\"a2\")];\n", ic, seq];
    [m appendString:@"        tensor<int32, [4]> pm = const()[name = string(\"pm\"), val = tensor<int32, [4]>([0,1,3,2])];\n"];
    [m appendFormat:@"        tensor<fp16, [1,1,%d,%d]> a3 = transpose(perm=pm,x=a2)[name=string(\"a3\")];\n", seq, ic];
    // Reshape weight: [1,IC,1,OC] → [1,1,IC,OC]
    [m appendFormat:@"        tensor<int32, [4]> rw = const()[name = string(\"rw\"), val = tensor<int32, [4]>([1,1,%d,%d])];\n", ic, oc];
    [m appendFormat:@"        tensor<fp16, [1,1,%d,%d]> W = reshape(shape=rw,x=wt)[name=string(\"W\")];\n", ic, oc];
    // matmul: [1,1,SEQ,IC] @ [1,1,IC,OC] → [1,1,SEQ,OC]
    [m appendString:@"        bool bF = const()[name = string(\"bF\"), val = bool(false)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,1,%d,%d]> yh = matmul(transpose_x=bF,transpose_y=bF,x=a3,y=W)[name=string(\"mm\")];\n", seq, oc];
    // Reshape+transpose back: [1,1,SEQ,OC] → transpose → [1,1,OC,SEQ] → reshape → [1,OC,1,SEQ]
    [m appendFormat:@"        tensor<fp16, [1,1,%d,%d]> yt = transpose(perm=pm,x=yh)[name=string(\"yt\")];\n", oc, seq];
    [m appendFormat:@"        tensor<int32, [4]> ro = const()[name = string(\"ro\"), val = tensor<int32, [4]>([1,%d,1,%d])];\n", oc, seq];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> yr = reshape(shape=ro,x=yt)[name=string(\"yr\")];\n", oc, seq];
    // Cast back to fp32
    [m appendString:@"        string to32 = const()[name = string(\"to32\"), val = string(\"fp32\")];\n"];
    [m appendFormat:@"        tensor<fp32, [1,%d,1,%d]> y = cast(dtype = to32, x = yr)[name = string(\"cout\")];\n", oc, seq];
    [m appendString:@"    } -> (y);\n}\n"];
    return m;
}

// Tiled version: splits OC into tiles, each tile is a separate kernel
// For W[IC, OC], tile along OC: each tile handles W[:, t*T:(t+1)*T]
// Input per tile: [1, IC, 1, SEQ+T]
// Output per tile: [1, T, 1, SEQ]
typedef struct {
    Kern **tiles;
    int n_tiles, tile_oc, ic, oc, seq;
} TiledMatmul;

static TiledMatmul *compile_tiled_matmul(int ic, int oc, int tile_oc, int seq) {
    TiledMatmul *tm = (TiledMatmul*)calloc(1, sizeof(TiledMatmul));
    tm->ic = ic; tm->oc = oc; tm->seq = seq; tm->tile_oc = tile_oc;
    tm->n_tiles = (oc + tile_oc - 1) / tile_oc;
    tm->tiles = (Kern**)calloc(tm->n_tiles, sizeof(Kern*));
    for (int t = 0; t < tm->n_tiles; t++) {
        int this_oc = (t == tm->n_tiles-1 && oc % tile_oc) ? (oc % tile_oc) : tile_oc;
        NSString *mil = gen_dynamic_matmul_mil(ic, this_oc, seq);
        int in_bytes = ic * (seq + this_oc) * 4;
        int out_bytes = this_oc * seq * 4;
        tm->tiles[t] = compile_kern_mil_w(mil, @{}, in_bytes, out_bytes);
        if (!tm->tiles[t]) { printf("Tile %d compile FAIL\n", t); return NULL; }
    }
    return tm;
}

// Write activations + weight tile into IOSurface
// act: [IC, SEQ] column-major (channel-first)
// W: [IC, OC] — full weight matrix, we extract the tile
static void write_tile_input(TiledMatmul *tm, int tile_idx, const float *act, const float *W) {
    Kern *k = tm->tiles[tile_idx];
    int ic = tm->ic, seq = tm->seq, toc = tm->tile_oc;
    int oc_off = tile_idx * toc;
    int this_oc = (tile_idx == tm->n_tiles-1 && tm->oc % toc) ? (tm->oc % toc) : toc;

    IOSurfaceLock(k->ioIn, 0, NULL);
    float *buf = (float*)IOSurfaceGetBaseAddress(k->ioIn);
    // Activations: buf[d * (seq+this_oc) + t] = act[d * seq + t]
    for (int d = 0; d < ic; d++) {
        memcpy(buf + d*(seq+this_oc), act + d*seq, seq*sizeof(float));
        // Weight: buf[d * (seq+this_oc) + seq + c] = W[d * oc + oc_off + c]
        for (int c = 0; c < this_oc; c++)
            buf[d*(seq+this_oc) + seq + c] = W[d*tm->oc + oc_off + c];
    }
    IOSurfaceUnlock(k->ioIn, 0, NULL);
}

// Read tile output into full output buffer
static void read_tile_output(TiledMatmul *tm, int tile_idx, float *out) {
    Kern *k = tm->tiles[tile_idx];
    int seq = tm->seq, toc = tm->tile_oc;
    int oc_off = tile_idx * toc;
    int this_oc = (tile_idx == tm->n_tiles-1 && tm->oc % toc) ? (tm->oc % toc) : toc;

    IOSurfaceLock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
    float *obuf = (float*)IOSurfaceGetBaseAddress(k->ioOut);
    for (int c = 0; c < this_oc; c++)
        memcpy(out + (oc_off+c)*seq, obuf + c*seq, seq*sizeof(float));
    IOSurfaceUnlock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
}

int main(int argc, char **argv) {
    @autoreleasepool {
        mach_timebase_info(&g_tb);
        ane_init();

        // === Test 1: Single 64×64 dynamic matmul (correctness) ===
        printf("=== Test 1: 64×64 dynamic matmul correctness ===\n");
        {
        int D = 64, S = 64;
        NSString *mil = gen_dynamic_matmul_mil(D, D, S);
        int in_b = D * (S+D) * 4, out_b = D * S * 4;
        Kern *k = compile_kern_mil_w(mil, @{}, in_b, out_b);
        if (!k) { printf("FAIL\n"); return 1; }

        // Identity test
        IOSurfaceLock(k->ioIn, 0, NULL);
        float *inp = (float*)IOSurfaceGetBaseAddress(k->ioIn);
        memset(inp, 0, in_b);
        for (int d = 0; d < D; d++)
            for (int s = 0; s < S; s++)
                inp[d*(S+D) + s] = (float)(d*S + s) * 0.001f;
        for (int d = 0; d < D; d++)
            for (int c = 0; c < D; c++)
                inp[d*(S+D) + S + c] = (d == c) ? 1.0f : 0.0f;
        IOSurfaceUnlock(k->ioIn, 0, NULL);

        ane_eval(k);
        IOSurfaceLock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
        float *out = (float*)IOSurfaceGetBaseAddress(k->ioOut);
        float me = 0;
        for (int d = 0; d < D; d++)
            for (int s = 0; s < S; s++) {
                float e = fabsf(out[d*S+s] - inp[d*(S+D)+s]);
                if (e > me) me = e;
            }
        IOSurfaceUnlock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
        printf("Identity: max_err=%.6f %s\n", me, me < 0.01 ? "PASS" : "FAIL");

        // 2× test
        IOSurfaceLock(k->ioIn, 0, NULL);
        for (int d = 0; d < D; d++)
            for (int c = 0; c < D; c++)
                inp[d*(S+D) + S + c] = (d == c) ? 2.0f : 0.0f;
        IOSurfaceUnlock(k->ioIn, 0, NULL);
        ane_eval(k);
        IOSurfaceLock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
        float sr = 0; int cnt = 0;
        for (int i = 0; i < D*S; i++)
            if (fabsf(inp[i/(S)*((S)+D) + i%S]) > 0.001f) { sr += out[i]/inp[i/S*(S+D)+i%S]; cnt++; }
        IOSurfaceUnlock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
        printf("2× W: ratio=%.3f %s\n\n", cnt?sr/cnt:0, fabsf(sr/cnt-2.0f)<0.1?"PASS":"FAIL");
        free_kern(k);
        }

        // === Test 2: 768×768 single kernel (if it compiles) ===
        printf("=== Test 2: 768×768 single dynamic matmul ===\n");
        {
        int D = 768, S = 256;
        int sp_total = S + D;  // 256 + 768 = 1024
        int in_b = D * sp_total * 4;  // 768 * 1024 * 4 = 3.1MB
        int out_b = D * S * 4;        // 768 * 256 * 4 = 786KB
        printf("IOSurface: in=%.1fMB out=%.1fKB\n", in_b/1e6, out_b/1e3);

        NSString *mil = gen_dynamic_matmul_mil(D, D, S);
        uint64_t t0 = mach_absolute_time();
        Kern *k = compile_kern_mil_w(mil, @{}, in_b, out_b);
        double compile_ms = tb_ms(mach_absolute_time() - t0);
        if (!k) { printf("768×768 compile FAIL\n"); }
        else {
            printf("Compile: %.1fms\n", compile_ms);
            // Random weights
            float *act = (float*)calloc(D*S, sizeof(float));
            float *W = (float*)calloc(D*D, sizeof(float));
            for (int i = 0; i < D*S; i++) act[i] = ((float)arc4random() / UINT32_MAX - 0.5f) * 0.1f;
            for (int i = 0; i < D*D; i++) W[i] = ((float)arc4random() / UINT32_MAX - 0.5f) * 0.01f;

            // Write to IOSurface
            IOSurfaceLock(k->ioIn, 0, NULL);
            float *inp = (float*)IOSurfaceGetBaseAddress(k->ioIn);
            for (int d = 0; d < D; d++) {
                memcpy(inp + d*(S+D), act + d*S, S*4);
                memcpy(inp + d*(S+D) + S, W + d*D, D*4);
            }
            IOSurfaceUnlock(k->ioIn, 0, NULL);

            // Warmup
            for (int i = 0; i < 3; i++) ane_eval(k);

            // Benchmark
            int iters = 50;
            t0 = mach_absolute_time();
            for (int i = 0; i < iters; i++) ane_eval(k);
            double total_ms = tb_ms(mach_absolute_time() - t0);
            double per_eval = total_ms / iters;
            double flops = 2.0 * D * D * S;  // matmul FLOPs
            double gflops = flops / (per_eval * 1e6);
            printf("768×768×256 matmul: %.3fms/eval  %.1f GFLOP/s\n", per_eval, gflops);

            // Benchmark with IO write (simulating weight update)
            t0 = mach_absolute_time();
            for (int i = 0; i < iters; i++) {
                IOSurfaceLock(k->ioIn, 0, NULL);
                float *p = (float*)IOSurfaceGetBaseAddress(k->ioIn);
                for (int d = 0; d < D; d++)
                    memcpy(p + d*(S+D) + S, W + d*D, D*4);
                IOSurfaceUnlock(k->ioIn, 0, NULL);
                ane_eval(k);
            }
            total_ms = tb_ms(mach_absolute_time() - t0);
            per_eval = total_ms / iters;
            gflops = flops / (per_eval * 1e6);
            printf("With weight IO: %.3fms/eval  %.1f GFLOP/s\n", per_eval, gflops);

            free(act); free(W); free_kern(k);
        }
        }

        // === Test 3: Tiled matmul benchmark ===
        int tile_sizes[] = {64, 128, 256, 384, 768};
        int n_tiles_test = sizeof(tile_sizes)/sizeof(tile_sizes[0]);
        printf("\n=== Test 3: Tiled 768×768 matmul (varying tile_oc) ===\n");
        printf("%-10s %-8s %-10s %-12s %-10s\n", "tile_oc", "tiles", "compile", "eval/ms", "GFLOP/s");
        {
        int D = 768, S = 256;
        float *act = (float*)calloc(D*S, sizeof(float));
        float *W = (float*)calloc(D*D, sizeof(float));
        float *out_full = (float*)calloc(D*S, sizeof(float));
        for (int i = 0; i < D*S; i++) act[i] = ((float)arc4random() / UINT32_MAX - 0.5f) * 0.1f;
        for (int i = 0; i < D*D; i++) W[i] = ((float)arc4random() / UINT32_MAX - 0.5f) * 0.01f;

        for (int ti = 0; ti < n_tiles_test; ti++) {
            int T = tile_sizes[ti];
            if (T > D) continue;
            uint64_t t0 = mach_absolute_time();
            TiledMatmul *tm = compile_tiled_matmul(D, D, T, S);
            double compile_ms = tb_ms(mach_absolute_time() - t0);
            if (!tm) { printf("%-10d FAIL\n", T); continue; }

            // Warmup
            for (int w = 0; w < 2; w++) {
                for (int t = 0; t < tm->n_tiles; t++) {
                    write_tile_input(tm, t, act, W);
                    ane_eval(tm->tiles[t]);
                }
            }

            // Benchmark (with IO)
            int iters = 20;
            t0 = mach_absolute_time();
            for (int i = 0; i < iters; i++) {
                for (int t = 0; t < tm->n_tiles; t++) {
                    write_tile_input(tm, t, act, W);
                    ane_eval(tm->tiles[t]);
                    read_tile_output(tm, t, out_full);
                }
            }
            double total_ms = tb_ms(mach_absolute_time() - t0);
            double per_matmul = total_ms / iters;
            double flops = 2.0 * D * D * S;
            double gflops = flops / (per_matmul * 1e6);
            printf("%-10d %-8d %-10.0fms %-12.3fms %-10.1f\n",
                T, tm->n_tiles, compile_ms, per_matmul, gflops);

            for (int t = 0; t < tm->n_tiles; t++) free_kern(tm->tiles[t]);
            free(tm->tiles); free(tm);
        }

        // === Correctness check: compare with cblas ===
        printf("\n=== Correctness: dynamic matmul vs cblas_sgemm ===\n");
        {
        int T = 768;  // full, no tiling
        TiledMatmul *tm = compile_tiled_matmul(D, D, T, S);
        if (tm) {
            write_tile_input(tm, 0, act, W);
            ane_eval(tm->tiles[0]);
            read_tile_output(tm, 0, out_full);

            // Reference: cblas  y = act^T @ W → y[s,oc] = sum_d act[d,s]*W[d,oc]
            // act is [D,S] col-major, W is [D,D] row-major
            // We want out[oc,s] = sum_d act[d,s] * W[d,oc]
            // = W^T @ act where W^T is [D,D] and act is [D,S] → out is [D,S]
            float *ref = (float*)calloc(D*S, sizeof(float));
            // out[oc*S+s] = sum_d W[d*D+oc] * act[d*S+s]
            // This is: (W^T) @ act in column-major: M=D,N=S,K=D
            // cblas: C = alpha*A*B + beta*C
            // A=W^T [D×D], B=act [D×S], C=ref [D×S]
            cblas_sgemm(CblasColMajor, CblasTrans, CblasNoTrans,
                D, S, D, 1.0f, W, D, act, D, 0.0f, ref, D);
            float me = 0;
            for (int i = 0; i < D*S; i++) {
                float e = fabsf(out_full[i] - ref[i]);
                if (e > me) me = e;
            }
            printf("vs cblas: max_err=%.6f %s\n", me, me < 1.0 ? "PASS" : "FAIL");
            free(ref);
            for (int t = 0; t < tm->n_tiles; t++) free_kern(tm->tiles[t]);
            free(tm->tiles); free(tm);
        }
        }

        free(act); free(W); free(out_full);
        }

        // === Summary for training ===
        printf("\n=== Summary ===\n");
        printf("Stories110M: 12 layers × 10 matmuls/layer = 120 matmuls/step\n");
        printf("Sizes: Wq/Wk/Wv/Wo [768,768], W1/W3 [2048,768], W2 [768,2048]\n");
        printf("With dynamic weights: compile once, update IOSurface every step\n");

        printf("\nDone.\n");
    }
    return 0;
}
