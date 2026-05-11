// test_classifier.m — Test classifier matmul (32000 channels) and softmax on ANE
// This tests the riskiest operations: VOCAB-sized conv and softmax
// Build: xcrun clang -O2 -framework Foundation -framework IOSurface \
//        -framework CoreML -framework Accelerate -ldl -lobjc \
//        -o test_classifier test_classifier.m
#include "ane_classifier.h"
#include "stories_cpu_ops.h"

int main(void) {
    @autoreleasepool {
        setbuf(stdout, NULL);
        ane_init();
        mach_timebase_info(&g_tb);
        
        printf("=== Test: Classifier + Softmax on ANE ===\n");
        printf("DIM=%d SEQ=%d VOCAB=%d\n\n", DIM, SEQ, VOCAB);
        
        // ======== Test 1: Final RMSNorm ========
        printf("--- Test 1: Final RMSNorm on ANE ---\n");
        {
            float *x = (float*)malloc(DIM * SEQ * 4);
            float *w = (float*)malloc(DIM * 4);
            float *out_cpu = (float*)malloc(DIM * SEQ * 4);
            float *out_ane = (float*)malloc(DIM * SEQ * 4);
            srand48(42);
            for (int i = 0; i < DIM * SEQ; i++) x[i] = (float)(drand48() * 2 - 1);
            for (int i = 0; i < DIM; i++) w[i] = (float)(drand48() * 0.5 + 0.75);
            
            rmsnorm(out_cpu, x, w, DIM, SEQ);
            
            Kern *kern = compile_kern_mil_w(gen_final_rmsnorm(), (@{
                @"@model_path/weights/rms_w.bin": @{@"offset":@0, @"data":build_blob(w, 1, DIM)},
            }), DIM*SEQ*2, DIM*SEQ*2);
            
            if (!kern) { printf("FAIL: Final RMSNorm compile failed\n"); return 1; }
            printf("Compile OK\n");
            
            io_write_fp16(kern->ioIn, x, DIM, SEQ);
            ane_eval(kern);
            io_read_fp16(kern->ioOut, out_ane, 0, DIM, SEQ);
            
            float max_err = 0;
            for (int i = 0; i < DIM*SEQ; i++) {
                float e = fabsf(out_cpu[i] - out_ane[i]);
                if (e > max_err) max_err = e;
            }
            printf("Max error: %.6f %s\n\n", max_err, max_err < 0.05 ? "PASS ✅" : "FAIL ❌");
            free_kern(kern);
            free(x); free(w); free(out_cpu); free(out_ane);
        }
        
        // ======== Test 2: Classifier forward (32000-channel conv) ========
        printf("--- Test 2: Classifier Forward (VOCAB=%d channel conv) ---\n", VOCAB);
        {
            float *x_final = (float*)malloc(DIM * SEQ * 4);
            float *embed = (float*)malloc((size_t)VOCAB * DIM * 4);
            float *logits_cpu = (float*)malloc((size_t)VOCAB * SEQ * 4);
            float *logits_ane = (float*)malloc((size_t)VOCAB * SEQ * 4);
            
            srand48(123);
            for (int i = 0; i < DIM * SEQ; i++) x_final[i] = (float)(drand48() * 2 - 1) * 0.1f;
            for (size_t i = 0; i < (size_t)VOCAB * DIM; i++) embed[i] = (float)(drand48() * 2 - 1) * 0.02f;
            
            // CPU reference: logits = embed @ x_final
            // logits[v, t] = sum_d embed[v,d] * x_final[d,t]
            // embed is [VOCAB, DIM] row-major, x_final is [DIM, SEQ] channel-first
            uint64_t t0 = mach_absolute_time();
            cblas_sgemm(CblasRowMajor, CblasNoTrans, CblasNoTrans,
                        VOCAB, SEQ, DIM, 1.0f,
                        embed, DIM, x_final, SEQ, 0.0f, logits_cpu, SEQ);
            uint64_t t1 = mach_absolute_time();
            printf("CPU cblas_sgemm: %.2f ms\n", tb_ms(t1-t0));
            
            // ANE: build weight blob for embed [VOCAB, DIM]
            printf("Building embed blob (%.1f MB fp16)...\n", (float)VOCAB*DIM*2/1e6);
            NSData *embed_blob = build_blob(embed, VOCAB, DIM);
            
            printf("Compiling classifier kernel...\n");
            t0 = mach_absolute_time();
            Kern *cls = compile_kern_mil_w(gen_classifier_fwd(), (@{
                @"@model_path/weights/embed.bin": @{@"offset":@0, @"data":embed_blob},
            }), DIM*SEQ*2, VOCAB*SEQ*2);
            t1 = mach_absolute_time();
            
            if (!cls) {
                printf("FAIL: Classifier compile failed (32000 channels too large for ANE)\n");
                printf("This confirms tiling is needed.\n\n");
            } else {
                printf("Compile OK in %.0f ms (compiles=%d)\n", tb_ms(t1-t0), g_compile_count);
                
                io_write_fp16(cls->ioIn, x_final, DIM, SEQ);
                t0 = mach_absolute_time();
                ane_eval(cls);
                t1 = mach_absolute_time();
                printf("ANE eval: %.2f ms\n", tb_ms(t1-t0));
                
                // Read back and compare (sample — full read would be 32000*256*4 = 32MB)
                io_read_fp16(cls->ioOut, logits_ane, 0, VOCAB, SEQ);
                
                float max_err = 0, sum_err = 0;
                int cnt = 0;
                for (int v = 0; v < VOCAB; v++) {
                    for (int t = 0; t < SEQ; t++) {
                        int idx = v*SEQ + t;
                        float e = fabsf(logits_cpu[idx] - logits_ane[idx]);
                        sum_err += e;
                        cnt++;
                        if (e > max_err) max_err = e;
                    }
                }
                printf("Max error: %.6f  Mean error: %.6f  %s\n",
                       max_err, sum_err/cnt, max_err < 1.0 ? "PASS ✅" : "FAIL ❌");
                
                // Benchmark
                int N = 10;
                t0 = mach_absolute_time();
                for (int i = 0; i < N; i++) ane_eval(cls);
                t1 = mach_absolute_time();
                printf("Benchmark: %d evals in %.2f ms (%.2f ms/eval)\n\n", N, tb_ms(t1-t0), tb_ms(t1-t0)/N);
                free_kern(cls);
            }
            free(x_final); free(embed); free(logits_cpu); free(logits_ane);
        }
        
        // ======== Test 3: Softmax over VOCAB dimension ========
        printf("--- Test 3: Softmax over VOCAB=%d ---\n", VOCAB);
        {
            float *logits = (float*)malloc((size_t)VOCAB * SEQ * 4);
            float *probs_cpu = (float*)malloc((size_t)VOCAB * SEQ * 4);
            float *probs_ane = (float*)malloc((size_t)VOCAB * SEQ * 4);
            
            srand48(999);
            for (size_t i = 0; i < (size_t)VOCAB * SEQ; i++) 
                logits[i] = (float)(drand48() * 10 - 5);
            
            // CPU reference softmax (per position, over vocab)
            // logits is [VOCAB, SEQ] channel-first
            uint64_t t0 = mach_absolute_time();
            for (int t = 0; t < SEQ; t++) {
                float maxv = -1e30f;
                for (int v = 0; v < VOCAB; v++) {
                    float val = logits[v*SEQ+t];
                    if (val > maxv) maxv = val;
                }
                float sum = 0;
                for (int v = 0; v < VOCAB; v++) {
                    probs_cpu[v*SEQ+t] = expf(logits[v*SEQ+t] - maxv);
                    sum += probs_cpu[v*SEQ+t];
                }
                for (int v = 0; v < VOCAB; v++) probs_cpu[v*SEQ+t] /= sum;
            }
            uint64_t t1 = mach_absolute_time();
            printf("CPU softmax: %.2f ms\n", tb_ms(t1-t0));
            
            printf("Compiling softmax kernel...\n");
            int sm_bytes = VOCAB * SEQ * 2;
            Kern *sm = compile_kern_mil_w(gen_softmax_vocab(), @{}, sm_bytes, sm_bytes);
            
            if (!sm) {
                printf("FAIL: Softmax compile failed\n\n");
            } else {
                printf("Compile OK\n");
                
                io_write_fp16(sm->ioIn, logits, VOCAB, SEQ);
                t0 = mach_absolute_time();
                ane_eval(sm);
                t1 = mach_absolute_time();
                printf("ANE eval: %.2f ms\n", tb_ms(t1-t0));
                
                io_read_fp16(sm->ioOut, probs_ane, 0, VOCAB, SEQ);
                
                // Check: probs should sum to ~1.0 per position
                float max_err = 0;
                for (int t = 0; t < 4; t++) {
                    float sum_cpu = 0, sum_ane = 0;
                    for (int v = 0; v < VOCAB; v++) {
                        sum_cpu += probs_cpu[v*SEQ+t];
                        sum_ane += probs_ane[v*SEQ+t];
                        float e = fabsf(probs_cpu[v*SEQ+t] - probs_ane[v*SEQ+t]);
                        if (e > max_err) max_err = e;
                    }
                    printf("  pos %d: CPU sum=%.4f  ANE sum=%.4f\n", t, sum_cpu, sum_ane);
                }
                printf("Max error (first 4 positions): %.6f  %s\n",
                       max_err, max_err < 0.01 ? "PASS ✅" : "FAIL ❌");
                
                int N = 10;
                t0 = mach_absolute_time();
                for (int i = 0; i < N; i++) ane_eval(sm);
                t1 = mach_absolute_time();
                printf("Benchmark: %d evals in %.2f ms (%.2f ms/eval)\n\n", N, tb_ms(t1-t0), tb_ms(t1-t0)/N);
                free_kern(sm);
            }
            free(logits); free(probs_cpu); free(probs_ane);
        }
        
        // ======== Test 4: Classifier backward ========
        printf("--- Test 4: Classifier Backward (DIM=%d from VOCAB=%d) ---\n", DIM, VOCAB);
        {
            float *dlogits = (float*)malloc((size_t)VOCAB * SEQ * 4);
            float *embed = (float*)malloc((size_t)VOCAB * DIM * 4);
            float *dx_cpu = (float*)malloc(DIM * SEQ * 4);
            float *dx_ane = (float*)malloc(DIM * SEQ * 4);
            
            srand48(456);
            for (size_t i = 0; i < (size_t)VOCAB * SEQ; i++) dlogits[i] = (float)(drand48() * 2 - 1) * 0.01f;
            for (size_t i = 0; i < (size_t)VOCAB * DIM; i++) embed[i] = (float)(drand48() * 2 - 1) * 0.02f;
            
            // CPU: dx = embed^T @ dlogits
            uint64_t t0 = mach_absolute_time();
            cblas_sgemm(CblasRowMajor, CblasTrans, CblasNoTrans,
                        DIM, SEQ, VOCAB, 1.0f,
                        embed, DIM, dlogits, SEQ, 0.0f, dx_cpu, SEQ);
            uint64_t t1 = mach_absolute_time();
            printf("CPU cblas_sgemm: %.2f ms\n", tb_ms(t1-t0));
            
            // Build transposed embed blob
            NSData *embed_t_blob = build_blob_t(embed, VOCAB, DIM);
            
            printf("Compiling classifier backward...\n");
            Kern *clsb = compile_kern_mil_w(gen_classifier_bwd(), (@{
                @"@model_path/weights/embed_t.bin": @{@"offset":@0, @"data":embed_t_blob},
            }), VOCAB*SEQ*2, DIM*SEQ*2);
            
            if (!clsb) {
                printf("FAIL: Classifier backward compile failed\n\n");
            } else {
                printf("Compile OK\n");
                
                io_write_fp16(clsb->ioIn, dlogits, VOCAB, SEQ);
                t0 = mach_absolute_time();
                ane_eval(clsb);
                t1 = mach_absolute_time();
                printf("ANE eval: %.2f ms\n", tb_ms(t1-t0));
                
                io_read_fp16(clsb->ioOut, dx_ane, 0, DIM, SEQ);
                
                float max_err = 0, sum_err = 0;
                for (int i = 0; i < DIM*SEQ; i++) {
                    float e = fabsf(dx_cpu[i] - dx_ane[i]);
                    sum_err += e;
                    if (e > max_err) max_err = e;
                }
                printf("Max error: %.6f  Mean error: %.6f  %s\n\n",
                       max_err, sum_err/(DIM*SEQ), max_err < 1.0 ? "PASS ✅" : "FAIL ❌");
                free_kern(clsb);
            }
            free(dlogits); free(embed); free(dx_cpu); free(dx_ane);
        }
        
        printf("=== All tests complete ===\n");
        printf("Total ANE compiles used: %d\n", g_compile_count);
        return 0;
    }
}
