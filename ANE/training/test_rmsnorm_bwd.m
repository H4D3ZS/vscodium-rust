// test_rmsnorm_bwd.m — Test RMSNorm backward ANE kernel vs CPU reference
// Build: xcrun clang -O2 -framework Foundation -framework IOSurface \
//        -framework CoreML -framework Accelerate -ldl -lobjc \
//        -o test_rmsnorm_bwd test_rmsnorm_bwd.m
#include "ane_rmsnorm_bwd.h"
#include "stories_cpu_ops.h"

int main(void) {
    @autoreleasepool {
        setbuf(stdout, NULL);
        ane_init();
        mach_timebase_info(&g_tb);
        
        printf("=== Test: RMSNorm Backward on ANE ===\n");
        printf("DIM=%d SEQ=%d\n\n", DIM, SEQ);
        
        // Allocate test data
        float *x = (float*)malloc(DIM * SEQ * 4);
        float *dy = (float*)malloc(DIM * SEQ * 4);
        float *w = (float*)malloc(DIM * 4);
        float *dx_cpu = (float*)calloc(DIM * SEQ, 4);
        float *dw_cpu = (float*)calloc(DIM, 4);
        float *dx_ane = (float*)malloc(DIM * SEQ * 4);
        
        // Random init (channel-first [DIM, SEQ])
        srand48(42);
        for (int i = 0; i < DIM * SEQ; i++) {
            x[i] = (float)(drand48() * 2 - 1) * 0.5f;
            dy[i] = (float)(drand48() * 2 - 1) * 0.1f;
        }
        for (int i = 0; i < DIM; i++) {
            w[i] = (float)(drand48() * 0.5 + 0.75);  // close to 1.0
        }
        
        // === CPU Reference ===
        uint64_t t0 = mach_absolute_time();
        rmsnorm_bwd(dx_cpu, dw_cpu, dy, x, w, DIM, SEQ);
        uint64_t t1 = mach_absolute_time();
        printf("CPU rmsnorm_bwd: %.2f ms\n", tb_ms(t1 - t0));
        
        // === ANE Kernel ===
        printf("Compiling ANE rmsnorm_bwd kernel...\n");
        NSString *mil = gen_rmsnorm_bwd();
        
        // Build weight blob for RMSNorm weights
        NSData *rms_blob = build_blob(w, 1, DIM);
        
        int in_bytes = 2 * DIM * SEQ * 2;  // concat(dy, x) in fp16
        int out_bytes = DIM * SEQ * 2;      // dx in fp16
        
        Kern *kern = compile_kern_mil_w(mil, (@{
            @"@model_path/weights/rms_w.bin": @{@"offset":@0, @"data":rms_blob},
        }), in_bytes, out_bytes);
        
        if (!kern) {
            printf("FAIL: ANE kernel compilation failed!\n");
            return 1;
        }
        printf("Compile OK (compiles=%d)\n", g_compile_count);
        
        // Write input: concat(dy, x) into ioIn
        // dy goes at channel offset 0, x goes at channel offset DIM
        io_write_fp16_at(kern->ioIn, 0, dy, DIM, SEQ);
        io_write_fp16_at(kern->ioIn, DIM, x, DIM, SEQ);
        
        // Evaluate
        t0 = mach_absolute_time();
        ane_eval(kern);
        t1 = mach_absolute_time();
        printf("ANE eval: %.3f ms\n", tb_ms(t1 - t0));
        
        // Read output
        io_read_fp16(kern->ioOut, dx_ane, 0, DIM, SEQ);
        
        // === Compare ===
        float max_err = 0, sum_err = 0;
        int max_i = 0, max_j = 0;
        for (int i = 0; i < DIM; i++) {
            for (int j = 0; j < SEQ; j++) {
                int idx = i * SEQ + j;
                float err = fabsf(dx_cpu[idx] - dx_ane[idx]);
                sum_err += err;
                if (err > max_err) {
                    max_err = err;
                    max_i = i; max_j = j;
                }
            }
        }
        float mean_err = sum_err / (DIM * SEQ);
        
        printf("\n=== Results ===\n");
        printf("Max absolute error: %.6f at [%d,%d] (CPU=%.6f ANE=%.6f)\n",
               max_err, max_i, max_j, dx_cpu[max_i*SEQ+max_j], dx_ane[max_i*SEQ+max_j]);
        printf("Mean absolute error: %.6f\n", mean_err);
        
        // Sample outputs
        printf("\nSample dx values (first 4 channels, first 4 positions):\n");
        printf("%-6s %-12s %-12s %-10s\n", "Idx", "CPU", "ANE", "Error");
        for (int i = 0; i < 4 && i < DIM; i++) {
            for (int j = 0; j < 4 && j < SEQ; j++) {
                int idx = i * SEQ + j;
                printf("[%d,%d] %-12.6f %-12.6f %-10.6f\n",
                       i, j, dx_cpu[idx], dx_ane[idx], fabsf(dx_cpu[idx] - dx_ane[idx]));
            }
        }
        
        // Benchmark: multiple evals
        int N = 100;
        t0 = mach_absolute_time();
        for (int i = 0; i < N; i++) ane_eval(kern);
        t1 = mach_absolute_time();
        printf("\nBenchmark: %d evals in %.2f ms (%.3f ms/eval)\n",
               N, tb_ms(t1-t0), tb_ms(t1-t0)/N);
        
        // Pass/fail
        bool pass = max_err < 0.05f && mean_err < 0.01f;
        printf("\n%s (threshold: max<0.05, mean<0.01)\n", pass ? "PASS ✅" : "FAIL ❌");
        
        free_kern(kern);
        free(x); free(dy); free(w); free(dx_cpu); free(dw_cpu); free(dx_ane);
        return pass ? 0 : 1;
    }
}
