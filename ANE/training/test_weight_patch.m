// test_weight_patch.m — Test whether ANE weights can be patched after compile
#import <Foundation/Foundation.h>
#import <objc/runtime.h>
#import <objc/message.h>
#import <dlfcn.h>
#import <IOSurface/IOSurface.h>
#import <mach/mach.h>
#import <mach/mach_time.h>
#import <mach/vm_map.h>
#include <arm_neon.h>
#include <Accelerate/Accelerate.h>

#include "stories_io.h"

// MIL: fp32 in → cast fp16 → conv → cast fp32 out (matches inmem_peak.m pattern)
static NSString *gen_conv_mil(int ic, int oc, int sp) {
    NSMutableString *m = [NSMutableString string];
    [m appendString:@"program(1.3)\n"
        "[buildInfo = dict<string, string>({{\"coremlc-component-MIL\", \"3510.2.1\"}, "
        "{\"coremlc-version\", \"3505.4.1\"}, {\"coremltools-component-milinternal\", \"\"}, "
        "{\"coremltools-version\", \"9.0\"}})]\n{\n"];
    [m appendFormat:@"    func main<ios18>(tensor<fp32, [1, %d, 1, %d]> x) {\n", ic, sp];
    [m appendString:
        @"        string pt = const()[name = string(\"pt\"), val = string(\"valid\")];\n"
        "        tensor<int32, [2]> st = const()[name = string(\"st\"), val = tensor<int32, [2]>([1, 1])];\n"
        "        tensor<int32, [4]> pd = const()[name = string(\"pd\"), val = tensor<int32, [4]>([0, 0, 0, 0])];\n"
        "        tensor<int32, [2]> dl = const()[name = string(\"dl\"), val = tensor<int32, [2]>([1, 1])];\n"
        "        int32 gr = const()[name = string(\"gr\"), val = int32(1)];\n"
        "        string to16 = const()[name = string(\"to16\"), val = string(\"fp16\")];\n"];
    [m appendFormat:@"        tensor<fp16, [1, %d, 1, %d]> xh = cast(dtype = to16, x = x)[name = string(\"cast_in\")];\n", ic, sp];
    [m appendFormat:@"        tensor<fp16, [%d, %d, 1, 1]> W = const()[name = string(\"W\"), "
        "val = tensor<fp16, [%d, %d, 1, 1]>(BLOBFILE(path = string(\"@model_path/weights/w.bin\"), offset = uint64(64)))];\n",
        oc, ic, oc, ic];
    [m appendFormat:@"        tensor<fp16, [1, %d, 1, %d]> yh = conv(dilations = dl, groups = gr, pad = pd, pad_type = pt, strides = st, weight = W, x = xh)"
        "[name = string(\"conv\")];\n", oc, sp];
    [m appendString:@"        string to32 = const()[name = string(\"to32\"), val = string(\"fp32\")];\n"];
    [m appendFormat:@"        tensor<fp32, [1, %d, 1, %d]> y = cast(dtype = to32, x = yh)[name = string(\"cast_out\")];\n", oc, sp];
    [m appendString:@"    } -> (y);\n}\n"];
    return m;
}

int main(int argc, char **argv) {
    @autoreleasepool {
        mach_timebase_info(&g_tb);
        ane_init();

        int IC = 256, OC = 256, SP = 64;
        int io_bytes = IC * SP * 4;  // fp32

        // Identity weight
        float *W_id = (float*)calloc(OC*IC, sizeof(float));
        for (int i = 0; i < IC; i++) W_id[i*IC+i] = 1.0f;

        NSString *mil = gen_conv_mil(IC, OC, SP);
        NSDictionary *wd = @{@"@model_path/weights/w.bin": @{@"offset":@0, @"data":build_blob(W_id, OC, IC)}};

        printf("=== Compiling conv %dx%d sp=%d ===\n", OC, IC, SP);
        Kern *k = compile_kern_mil_w(mil, wd, io_bytes, io_bytes);
        if (!k) { printf("COMPILE FAILED\n"); free(W_id); return 1; }
        printf("Compile OK!\n");

        // Write fp32 input
        IOSurfaceLock(k->ioIn, 0, NULL);
        float *inp = (float*)IOSurfaceGetBaseAddress(k->ioIn);
        for (int i = 0; i < IC*SP; i++) inp[i] = (i % 100) * 0.01f;
        IOSurfaceUnlock(k->ioIn, 0, NULL);

        // Eval with identity
        ane_eval(k);
        IOSurfaceLock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
        float *out = (float*)IOSurfaceGetBaseAddress(k->ioOut);
        printf("In:  [%.3f, %.3f, %.3f, %.3f]\n", inp[0], inp[1], inp[2], inp[3]);
        printf("Out: [%.3f, %.3f, %.3f, %.3f]\n", out[0], out[1], out[2], out[3]);
        float max_err = 0;
        for (int i = 0; i < OC*SP; i++) {
            float err = fabsf(out[i] - inp[i]);
            if (err > max_err) max_err = err;
        }
        IOSurfaceUnlock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
        printf("Identity max_err=%.6f %s\n\n", max_err, max_err < 0.1 ? "PASS" : "FAIL");

        // === Approach 1: Patch weight on disk, unload+reload ===
        printf("=== Approach 1: Disk patch + unload/reload ===\n");
        float *W_2x = (float*)calloc(OC*IC, sizeof(float));
        for (int i = 0; i < IC; i++) W_2x[i*IC+i] = 2.0f;
        [build_blob(W_2x, OC, IC) writeToFile:
            [(__bridge NSString*)k->tmpDir stringByAppendingPathComponent:@"weights/w.bin"] atomically:YES];

        id mdl = (__bridge id)k->model;
        NSError *e = nil;
        ((BOOL(*)(id,SEL,unsigned int,NSError**))objc_msgSend)(mdl, @selector(unloadWithQoS:error:), 21, &e);
        e = nil;
        BOOL ok = ((BOOL(*)(id,SEL,unsigned int,id,NSError**))objc_msgSend)(mdl, @selector(loadWithQoS:options:error:), 21, @{}, &e);
        printf("Reload: %s\n", ok?"OK":"FAIL");
        if (ok) {
            // Re-create request after reload
            id wI = ((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(g_AIO, @selector(objectWithIOSurface:), k->ioIn);
            id wO = ((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(g_AIO, @selector(objectWithIOSurface:), k->ioOut);
            CFRelease(k->request);
            k->request = (void*)CFBridgingRetain(((id(*)(Class,SEL,id,id,id,id,id,id,id))objc_msgSend)(g_AR,
                @selector(requestWithInputs:inputIndices:outputs:outputIndices:weightsBuffer:perfStats:procedureIndex:),
                @[wI], @[@0], @[wO], @[@0], nil, nil, @0));
            ane_eval(k);
            IOSurfaceLock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
            printf("Out: [%.3f, %.3f, %.3f, %.3f]\n", out[0], out[1], out[2], out[3]);
            float sr = 0; int cnt = 0;
            for (int i = 0; i < OC*SP; i++)
                if (fabsf(inp[i]) > 0.01f) { sr += out[i]/inp[i]; cnt++; }
            IOSurfaceUnlock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
            printf("Ratio: %.3f (2.0=patched, 1.0=cached)\n\n", cnt>0?sr/cnt:0);
        }

        // === Approach 2: Memory scan ===
        printf("=== Approach 2: Memory scan ===\n");
        uint16_t pat1[8] = {0x3C00, 0, 0, 0, 0, 0, 0, 0};
        uint16_t pat2[8] = {0x4000, 0, 0, 0, 0, 0, 0, 0};
        mach_port_t task = mach_task_self();
        vm_address_t addr = 0; vm_size_t sz; natural_t depth = 1;
        int f1 = 0, f2 = 0;
        while (1) {
            struct vm_region_submap_info_64 info;
            mach_msg_type_number_t count = VM_REGION_SUBMAP_INFO_COUNT_64;
            if (vm_region_recurse_64(task, &addr, &sz, &depth, (vm_region_recurse_info_t)&info, &count) != KERN_SUCCESS) break;
            if (info.is_submap) { depth++; continue; }
            if (!(info.protection & VM_PROT_READ) || sz < (size_t)(OC*IC*2)) { addr += sz; continue; }
            uint8_t *base = (uint8_t*)addr;
            for (size_t off = 0; off + OC*IC*2 <= sz; off += 2) {
                int w = 0;
                if (memcmp(base+off, pat1, 16) == 0) w = 1;
                else if (memcmp(base+off, pat2, 16) == 0) w = 2;
                if (!w) continue;
                uint16_t *p = (uint16_t*)(base+off), diag = (w==1)?0x3C00:0x4000;
                int ok2 = 1;
                for (int r = 0; r < OC && ok2; r++)
                    for (int c = 0; c < IC && ok2; c++)
                        if (p[r*IC+c] != ((r==c)?diag:0)) ok2 = 0;
                if (!ok2) continue;
                if (w==1) f1++; else f2++;
                printf("  FOUND %dx @%p prot=%d/%d %s\n", w, (void*)(addr+off),
                    info.protection, info.max_protection, (info.protection&VM_PROT_WRITE)?"WR":"RO");
            }
            addr += sz;
        }
        printf("Found: 1x=%d 2x=%d\n", f1, f2);

        // Now patch ALL found weight patterns to 3× and re-eval
        if (f1 > 0 || f2 > 0) {
            printf("Patching all found patterns to 3x identity...\n");
            addr = 0; depth = 1;
            while (1) {
                struct vm_region_submap_info_64 info2;
                mach_msg_type_number_t count2 = VM_REGION_SUBMAP_INFO_COUNT_64;
                if (vm_region_recurse_64(task, &addr, &sz, &depth, (vm_region_recurse_info_t)&info2, &count2) != KERN_SUCCESS) break;
                if (info2.is_submap) { depth++; continue; }
                if (!(info2.protection & VM_PROT_READ) || sz < (size_t)(OC*IC*2)) { addr += sz; continue; }
                uint8_t *base2 = (uint8_t*)addr;
                for (size_t off = 0; off + OC*IC*2 <= sz; off += 2) {
                    int w2 = 0;
                    if (memcmp(base2+off, pat1, 16) == 0) w2 = 1;
                    else if (memcmp(base2+off, pat2, 16) == 0) w2 = 2;
                    if (!w2) continue;
                    uint16_t *p2 = (uint16_t*)(base2+off), diag2 = (w2==1)?0x3C00:0x4000;
                    int ok3 = 1;
                    for (int r = 0; r < OC && ok3; r++)
                        for (int c = 0; c < IC && ok3; c++)
                            if (p2[r*IC+c] != ((r==c)?diag2:0)) ok3 = 0;
                    if (!ok3) continue;
                    if (info2.protection & VM_PROT_WRITE) {
                        printf("  Patching %dx @%p to 3x\n", w2, (void*)(addr+off));
                        for (int r = 0; r < OC; r++)
                            for (int c = 0; c < IC; c++)
                                p2[r*IC+c] = (r==c) ? 0x4200 : 0; // fp16(3.0)
                    }
                }
                addr += sz;
            }

            printf("\n=== Eval after memory patch (expect 3x) ===\n");
            ane_eval(k);
            IOSurfaceLock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
            printf("Out: [%.3f, %.3f, %.3f, %.3f]\n", out[0], out[1], out[2], out[3]);
            float sr2 = 0; int cnt2 = 0;
            for (int i = 0; i < OC*SP; i++)
                if (fabsf(inp[i]) > 0.01f) { sr2 += out[i]/inp[i]; cnt2++; }
            IOSurfaceUnlock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
            printf("Ratio: %.3f (3.0=mem patch works!, 1.0=ANE uses SRAM copy)\n", cnt2>0?sr2/cnt2:0);
        }
        printf("\n");

        // === Approach 3: Explore classes ===
        printf("=== ANE classes ===\n");
        const char *cn[] = {"_ANEWeight", "_ANEProgramForEvaluation", "_ANEChainingRequest", NULL};
        for (int i = 0; cn[i]; i++) {
            Class cls = NSClassFromString([NSString stringWithUTF8String:cn[i]]);
            if (!cls) { printf("%s: NOT FOUND\n", cn[i]); continue; }
            printf("%s:\n", cn[i]);
            unsigned int mc = 0; Method *ms = class_copyMethodList(cls, &mc);
            for (unsigned j = 0; j < mc; j++) printf("  - %s\n", sel_getName(method_getName(ms[j])));
            free(ms);
            mc = 0; ms = class_copyMethodList(object_getClass(cls), &mc);
            for (unsigned j = 0; j < mc; j++) printf("  + %s\n", sel_getName(method_getName(ms[j])));
            free(ms); printf("\n");
        }
        @try { printf("programHandle: %s\n", [[[mdl valueForKey:@"programHandle"] description] UTF8String]); } @catch(id x) {}
        @try { printf("intermediateBufferHandle: %s\n", [[[mdl valueForKey:@"intermediateBufferHandle"] description] UTF8String]); } @catch(id x) {}

        // === Approach 4: _ANEWeight + updateWeightURL ===
        printf("\n=== Approach 4: _ANEWeight API ===\n");
        Class AW = NSClassFromString(@"_ANEWeight");
        if (AW) {
            // Write 5× identity weights to a new file
            float *W_5x = (float*)calloc(OC*IC, sizeof(float));
            for (int i = 0; i < IC; i++) W_5x[i*IC+i] = 5.0f;
            NSString *wpath = [NSTemporaryDirectory() stringByAppendingPathComponent:@"patched_w.bin"];
            [build_blob(W_5x, OC, IC) writeToFile:wpath atomically:YES];
            free(W_5x);

            NSURL *wurl = [NSURL fileURLWithPath:wpath];
            id wobj = ((id(*)(Class,SEL,id,id))objc_msgSend)(AW,
                @selector(weightWithSymbolAndURL:weightURL:), @"W", wurl);
            printf("  _ANEWeight: %s\n", wobj ? [[wobj description] UTF8String] : "nil");
            if (wobj) {
                printf("  weightSymbol: %s\n", [((id(*)(id,SEL))objc_msgSend)(wobj, @selector(weightSymbol)) UTF8String]);
                printf("  weightURL: %s\n", [[((id(*)(id,SEL))objc_msgSend)(wobj, @selector(weightURL)) description] UTF8String]);
            }

            // Try to pass as weightsBuffer in request
            printf("\n  Trying weightsBuffer in request...\n");
            id wI2 = ((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(g_AIO, @selector(objectWithIOSurface:), k->ioIn);
            id wO2 = ((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(g_AIO, @selector(objectWithIOSurface:), k->ioOut);

            // Try passing weight array as weightsBuffer
            if (wobj) {
                CFRelease(k->request);
                k->request = (void*)CFBridgingRetain(((id(*)(Class,SEL,id,id,id,id,id,id,id))objc_msgSend)(g_AR,
                    @selector(requestWithInputs:inputIndices:outputs:outputIndices:weightsBuffer:perfStats:procedureIndex:),
                    @[wI2], @[@0], @[wO2], @[@0], @[wobj], nil, @0));
                printf("  Request with weightsBuffer created\n");
                @try {
                    ane_eval(k);
                    IOSurfaceLock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
                    printf("  Out: [%.3f, %.3f, %.3f, %.3f]\n", out[0], out[1], out[2], out[3]);
                    float sr3 = 0; int cnt3 = 0;
                    for (int i2 = 0; i2 < OC*SP; i2++)
                        if (fabsf(inp[i2]) > 0.01f) { sr3 += out[i2]/inp[i2]; cnt3++; }
                    IOSurfaceUnlock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
                    printf("  Ratio: %.3f (5.0=weightsBuffer works!)\n", cnt3>0?sr3/cnt3:0);
                } @catch(NSException *ex) {
                    printf("  Eval exception: %s\n", [[ex description] UTF8String]);
                }
            }

            // Also try IOSurface as weightsBuffer
            printf("\n  Trying IOSurface as weightsBuffer...\n");
            IOSurfaceRef wSurf = make_surface(OC*IC*2);  // fp16 weights
            IOSurfaceLock(wSurf, 0, NULL);
            _Float16 *wfp16 = (_Float16*)IOSurfaceGetBaseAddress(wSurf);
            for (int r = 0; r < OC; r++)
                for (int c2 = 0; c2 < IC; c2++)
                    wfp16[r*IC+c2] = (r==c2) ? (_Float16)7.0f : (_Float16)0.0f;  // 7× identity
            IOSurfaceUnlock(wSurf, 0, NULL);
            id wSurfObj = ((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(g_AIO, @selector(objectWithIOSurface:), wSurf);
            CFRelease(k->request);
            k->request = (void*)CFBridgingRetain(((id(*)(Class,SEL,id,id,id,id,id,id,id))objc_msgSend)(g_AR,
                @selector(requestWithInputs:inputIndices:outputs:outputIndices:weightsBuffer:perfStats:procedureIndex:),
                @[wI2], @[@0], @[wO2], @[@0], wSurfObj, nil, @0));
            @try {
                ane_eval(k);
                IOSurfaceLock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
                printf("  Out: [%.3f, %.3f, %.3f, %.3f]\n", out[0], out[1], out[2], out[3]);
                float sr4 = 0; int cnt4 = 0;
                for (int i3 = 0; i3 < OC*SP; i3++)
                    if (fabsf(inp[i3]) > 0.01f) { sr4 += out[i3]/inp[i3]; cnt4++; }
                IOSurfaceUnlock(k->ioOut, kIOSurfaceLockReadOnly, NULL);
                printf("  Ratio: %.3f (7.0=IOSurface weights work!)\n", cnt4>0?sr4/cnt4:0);
            } @catch(NSException *ex) {
                printf("  Eval exception: %s\n", [[ex description] UTF8String]);
            }
            CFRelease(wSurf);
        }

        // === Approach 5: Weights packed into input IOSurface (fp16 with cast) ===
        printf("\n=== Approach 5: Dynamic weights via input IOSurface ===\n");
        // Element-wise mul: x * w where both come from input
        // Input [1, IC*2, 1, SP] fp32 → cast fp16 → slice → mul → cast fp32
        {
        int C5 = IC;
        NSMutableString *m5 = [NSMutableString string];
        [m5 appendString:@"program(1.3)\n"
            "[buildInfo = dict<string, string>({{\"coremlc-component-MIL\", \"3510.2.1\"}, "
            "{\"coremlc-version\", \"3505.4.1\"}, {\"coremltools-component-milinternal\", \"\"}, "
            "{\"coremltools-version\", \"9.0\"}})]\n{\n"];
        [m5 appendFormat:@"    func main<ios18>(tensor<fp32, [1, %d, 1, %d]> x) {\n", C5*2, SP];
        [m5 appendString:@"        string to16 = const()[name = string(\"to16\"), val = string(\"fp16\")];\n"];
        [m5 appendFormat:@"        tensor<fp16, [1, %d, 1, %d]> xh = cast(dtype = to16, x = x)[name = string(\"cin\")];\n", C5*2, SP];
        [m5 appendFormat:@"        tensor<int32, [4]> b0 = const()[name = string(\"b0\"), val = tensor<int32, [4]>([0,0,0,0])];\n"];
        [m5 appendFormat:@"        tensor<int32, [4]> s0 = const()[name = string(\"s0\"), val = tensor<int32, [4]>([1,%d,1,%d])];\n", C5, SP];
        [m5 appendFormat:@"        tensor<fp16, [1,%d,1,%d]> data = slice_by_size(x=xh,begin=b0,size=s0)[name=string(\"data\")];\n", C5, SP];
        [m5 appendFormat:@"        tensor<int32, [4]> b1 = const()[name = string(\"b1\"), val = tensor<int32, [4]>([0,%d,0,0])];\n", C5];
        [m5 appendFormat:@"        tensor<fp16, [1,%d,1,%d]> wt = slice_by_size(x=xh,begin=b1,size=s0)[name=string(\"wt\")];\n", C5, SP];
        [m5 appendFormat:@"        tensor<fp16, [1,%d,1,%d]> yh = mul(x=data,y=wt)[name=string(\"mul\")];\n", C5, SP];
        [m5 appendString:@"        string to32 = const()[name = string(\"to32\"), val = string(\"fp32\")];\n"];
        [m5 appendFormat:@"        tensor<fp32, [1,%d,1,%d]> y = cast(dtype = to32, x = yh)[name = string(\"cout\")];\n", C5, SP];
        [m5 appendString:@"    } -> (y);\n}\n"];

        int io5_in = C5*2*SP*4;
        int io5_out = C5*SP*4;
        Kern *k5 = compile_kern_mil_w(m5, @{}, io5_in, io5_out);
        if (k5) {
            printf("Compile OK!\n");
            IOSurfaceLock(k5->ioIn, 0, NULL);
            float *in5 = (float*)IOSurfaceGetBaseAddress(k5->ioIn);
            for (int i = 0; i < C5*SP; i++) in5[i] = (i%100)*0.01f;
            for (int i = 0; i < C5*SP; i++) in5[C5*SP+i] = 2.0f;
            IOSurfaceUnlock(k5->ioIn, 0, NULL);
            ane_eval(k5);
            IOSurfaceLock(k5->ioOut, kIOSurfaceLockReadOnly, NULL);
            float *out5 = (float*)IOSurfaceGetBaseAddress(k5->ioOut);
            printf("data=[%.3f,%.3f,%.3f], w=2.0 → out=[%.3f,%.3f,%.3f]\n",
                in5[0],in5[1],in5[2], out5[0],out5[1],out5[2]);
            IOSurfaceUnlock(k5->ioOut, kIOSurfaceLockReadOnly, NULL);

            // Change weight dynamically — NO recompile!
            IOSurfaceLock(k5->ioIn, 0, NULL);
            for (int i = 0; i < C5*SP; i++) in5[C5*SP+i] = 5.0f;
            IOSurfaceUnlock(k5->ioIn, 0, NULL);
            ane_eval(k5);
            IOSurfaceLock(k5->ioOut, kIOSurfaceLockReadOnly, NULL);
            printf("w=5.0 → out=[%.3f,%.3f,%.3f] (expect 5×)\n", out5[0],out5[1],out5[2]);
            IOSurfaceUnlock(k5->ioOut, kIOSurfaceLockReadOnly, NULL);
            free_kern(k5);
        } else printf("Compile FAILED\n");
        }

        // === Approach 6: matmul with dynamic weights from input ===
        printf("\n=== Approach 6: matmul with dynamic W from input ===\n");
        // Pack x[1,D,S,1] and W[1,D,1,D] into input, then reshape+matmul
        // Input shape: [1, D+D*D, 1, S] — first D channels=activations, rest=weight matrix flattened
        // Actually, matmul needs [1,H,S,D] shapes. Let's try:
        // Input: [1, D*(S+D), 1, 1] reshaped as needed
        // Simpler: just test matmul with two sliced inputs
        {
        int D6 = 64, S6 = 64;  // small for test
        // Input: [1, D6+D6, S6, D6] — but that's 4D...
        // Actually ANE matmul works on [1,H,M,K] @ [1,H,K,N] → [1,H,M,N]
        // Let's pack x[1,1,S6,D6] and W[1,1,D6,D6] into [1,2,S6,D6]
        // Then slice → matmul
        NSMutableString *m6 = [NSMutableString string];
        [m6 appendString:@"program(1.3)\n"
            "[buildInfo = dict<string, string>({{\"coremlc-component-MIL\", \"3510.2.1\"}, "
            "{\"coremlc-version\", \"3505.4.1\"}, {\"coremltools-component-milinternal\", \"\"}, "
            "{\"coremltools-version\", \"9.0\"}})]\n{\n"];
        // Input: [1, D6+D6, 1, S6*D6] — flatten everything, then reshape
        // Actually simplest: two separate regions in channel dim
        // x_data: [1, D6, 1, S6] and W: [1, D6*D6, 1, 1]
        // Total input channels: D6 + D6*D6
        int total_ch = D6 + D6*D6;
        [m6 appendFormat:@"    func main<ios18>(tensor<fp32, [1, %d, 1, %d]> x) {\n", total_ch, S6];
        [m6 appendString:@"        string to16 = const()[name = string(\"to16\"), val = string(\"fp16\")];\n"];
        [m6 appendFormat:@"        tensor<fp16, [1, %d, 1, %d]> xh = cast(dtype = to16, x = x)[name = string(\"cin\")];\n", total_ch, S6];
        // Slice activations: [1, D6, 1, S6]
        [m6 appendFormat:@"        tensor<int32, [4]> b0 = const()[name = string(\"b0\"), val = tensor<int32, [4]>([0,0,0,0])];\n"];
        [m6 appendFormat:@"        tensor<int32, [4]> sa = const()[name = string(\"sa\"), val = tensor<int32, [4]>([1,%d,1,%d])];\n", D6, S6];
        [m6 appendFormat:@"        tensor<fp16, [1,%d,1,%d]> act = slice_by_size(x=xh,begin=b0,size=sa)[name=string(\"act\")];\n", D6, S6];
        // Slice weight: [1, D6*D6, 1, S6] but we only need [D6, D6] → reshape
        [m6 appendFormat:@"        tensor<int32, [4]> bw = const()[name = string(\"bw\"), val = tensor<int32, [4]>([0,%d,0,0])];\n", D6];
        [m6 appendFormat:@"        tensor<int32, [4]> sw = const()[name = string(\"sw\"), val = tensor<int32, [4]>([1,%d,1,%d])];\n", D6*D6, S6];
        [m6 appendFormat:@"        tensor<fp16, [1,%d,1,%d]> wf = slice_by_size(x=xh,begin=bw,size=sw)[name=string(\"wf\")];\n", D6*D6, S6];
        // Reshape weight to [1, D6, D6, S6] for matmul-like operation
        // Actually for conv: weight needs to be [OC, IC, 1, 1] const. Can't use dynamic weight with conv.
        // For matmul: need [1, 1, D6, D6] or similar
        // Let's try: reshape wf to [1, D6, D6, S6], take first slice [:,:,:,0] → no, that's hard
        // Simpler: reshape to [D6, D6] and use matmul
        // But matmul expects specific ranks... let me try:
        [m6 appendFormat:@"        tensor<int32, [4]> ws = const()[name = string(\"ws\"), val = tensor<int32, [4]>([1, 1, %d, %d])];\n", D6, D6];
        // Only take first column of wf to get [1, D6*D6, 1, 1]
        [m6 appendFormat:@"        tensor<int32, [4]> sw1 = const()[name = string(\"sw1\"), val = tensor<int32, [4]>([1,%d,1,1])];\n", D6*D6];
        [m6 appendFormat:@"        tensor<fp16, [1,%d,1,1]> wf1 = slice_by_size(x=wf,begin=b0,size=sw1)[name=string(\"wf1\")];\n", D6*D6];
        [m6 appendFormat:@"        tensor<fp16, [1,1,%d,%d]> W = reshape(shape=ws,x=wf1)[name=string(\"W\")];\n", D6, D6];
        // Reshape act to [1, 1, S6, D6] for matmul
        [m6 appendFormat:@"        tensor<int32, [4]> as2 = const()[name = string(\"as2\"), val = tensor<int32, [4]>([1, 1, %d, %d])];\n", D6, S6];
        [m6 appendFormat:@"        tensor<int32, [4]> pm = const()[name = string(\"pm\"), val = tensor<int32, [4]>([0, 1, 3, 2])];\n"];
        [m6 appendFormat:@"        tensor<fp16, [1,1,%d,%d]> a2 = reshape(shape=as2,x=act)[name=string(\"a2\")];\n", D6, S6];
        [m6 appendFormat:@"        tensor<fp16, [1,1,%d,%d]> a3 = transpose(perm=pm,x=a2)[name=string(\"a3\")];\n", S6, D6];
        // matmul: [1,1,S6,D6] @ [1,1,D6,D6] → [1,1,S6,D6]
        [m6 appendString:@"        bool bF = const()[name = string(\"bF\"), val = bool(false)];\n"];
        [m6 appendFormat:@"        tensor<fp16, [1, 1, %d, %d]> yh = matmul(transpose_x = bF, transpose_y = bF, x = a3, y = W)[name = string(\"mm\")];\n", S6, D6];
        // Reshape back to [1, D6, 1, S6]
        [m6 appendFormat:@"        tensor<fp16, [1,1,%d,%d]> yt = transpose(perm=pm,x=yh)[name=string(\"yt\")];\n", D6, S6];
        [m6 appendFormat:@"        tensor<int32, [4]> os = const()[name = string(\"os\"), val = tensor<int32, [4]>([1,%d,1,%d])];\n", D6, S6];
        [m6 appendFormat:@"        tensor<fp16, [1,%d,1,%d]> yr = reshape(shape=os,x=yt)[name=string(\"yr\")];\n", D6, S6];
        [m6 appendString:@"        string to32 = const()[name = string(\"to32\"), val = string(\"fp32\")];\n"];
        [m6 appendFormat:@"        tensor<fp32, [1,%d,1,%d]> y = cast(dtype = to32, x = yr)[name = string(\"cout\")];\n", D6, S6];
        [m6 appendString:@"    } -> (y);\n}\n"];

        int io6_in = total_ch * S6 * 4;
        int io6_out = D6 * S6 * 4;
        Kern *k6 = compile_kern_mil_w(m6, @{}, io6_in, io6_out);
        if (k6) {
            printf("Dynamic matmul compile OK!\n");
            // Set up: identity W, ramp input
            IOSurfaceLock(k6->ioIn, 0, NULL);
            float *in6 = (float*)IOSurfaceGetBaseAddress(k6->ioIn);
            memset(in6, 0, io6_in);
            // Activations: [D6, S6] in channel-first layout
            for (int d = 0; d < D6; d++)
                for (int s = 0; s < S6; s++)
                    in6[d*S6+s] = (d*S6+s) * 0.001f;
            // Weight: identity matrix [D6, D6] packed in channels D6..D6+D6*D6, only col 0
            float *wbase = in6 + D6*S6;
            for (int r = 0; r < D6; r++)
                for (int c = 0; c < D6; c++)
                    wbase[(r*D6+c)*S6] = (r==c) ? 1.0f : 0.0f;  // only sp=0 matters
            IOSurfaceUnlock(k6->ioIn, 0, NULL);

            ane_eval(k6);
            IOSurfaceLock(k6->ioOut, kIOSurfaceLockReadOnly, NULL);
            float *out6 = (float*)IOSurfaceGetBaseAddress(k6->ioOut);
            printf("Identity W: in=[%.4f,%.4f,%.4f] out=[%.4f,%.4f,%.4f]\n",
                in6[0],in6[1],in6[2], out6[0],out6[1],out6[2]);

            // Check
            float me6 = 0;
            for (int i = 0; i < D6*S6; i++) {
                float e6 = fabsf(out6[i] - in6[i]);
                if (e6 > me6) me6 = e6;
            }
            IOSurfaceUnlock(k6->ioOut, kIOSurfaceLockReadOnly, NULL);
            printf("max_err=%.6f %s\n", me6, me6 < 0.1 ? "PASS" : "FAIL");

            // Now: 2× identity — just change the IOSurface weight, no recompile!
            IOSurfaceLock(k6->ioIn, 0, NULL);
            for (int r = 0; r < D6; r++)
                for (int c = 0; c < D6; c++)
                    wbase[(r*D6+c)*S6] = (r==c) ? 2.0f : 0.0f;
            IOSurfaceUnlock(k6->ioIn, 0, NULL);
            ane_eval(k6);
            IOSurfaceLock(k6->ioOut, kIOSurfaceLockReadOnly, NULL);
            printf("2× W: in=[%.4f,%.4f] out=[%.4f,%.4f] (expect 2×)\n",
                in6[0],in6[1], out6[0],out6[1]);
            IOSurfaceUnlock(k6->ioOut, kIOSurfaceLockReadOnly, NULL);
            free_kern(k6);
        } else printf("Dynamic matmul compile FAILED\n");
        }

        free_kern(k); free(W_id); free(W_2x);
        printf("\nDone.\n");
    }
    return 0;
}
