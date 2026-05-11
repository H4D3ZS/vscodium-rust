// ane_rmsnorm_bwd.h — MIL generator for RMSNorm backward on ANE
// Replaces CPU rmsnorm_bwd() from stories_cpu_ops.h
//
// RMSNorm forward:  xn = x * rrms * w,  where rrms = 1/sqrt(mean(x²) + eps)
// RMSNorm backward: dx = w * rrms * (dy - x * sum(dy*w*x) * invd * rrms²)
//
// Input:  concat(dy, x) as [1, 2*DIM, 1, SEQ]
// Baked:  RMSNorm weights w [1, DIM, 1, 1] as BLOBFILE
// Output: dx [1, DIM, 1, SEQ]
//
// Note: dw (weight gradient) stays on CPU — it requires reduce_sum over SEQ
// and accumulation across steps, which is cheap and better done on CPU.
#pragma once
#include "stories_mil.h"

// Generate MIL for RMSNorm backward
// Input: concat(dy, x) [1, 2*DIM, 1, SEQ]
// Baked weights: rms_w [DIM] — the RMSNorm scale weights
// Output: dx [1, DIM, 1, SEQ]
static NSString *gen_rmsnorm_bwd(void) {
    float invd = 1.0f / (float)DIM;
    NSMutableString *m = [NSMutableString string];
    [m appendString:MIL_HDR];
    
    // Input: concat of dy and x along channel dimension
    [m appendFormat:@"    func main<ios18>(tensor<fp16, [1, %d, 1, %d]> inp) {\n", 2*DIM, SEQ];
    
    // Slice out dy [1, DIM, 1, SEQ] and x [1, DIM, 1, SEQ]
    [m appendFormat:@"        tensor<int32, [4]> sz = const()[name=string(\"sz\"), val=tensor<int32, [4]>([1,%d,1,%d])];\n", DIM, SEQ];
    [m appendString:@"        tensor<int32, [4]> b0 = const()[name=string(\"b0\"), val=tensor<int32, [4]>([0,0,0,0])];\n"];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> dy = slice_by_size(x=inp,begin=b0,size=sz)[name=string(\"sdy\")];\n", DIM, SEQ];
    [m appendFormat:@"        tensor<int32, [4]> b1 = const()[name=string(\"b1\"), val=tensor<int32, [4]>([0,%d,0,0])];\n", DIM];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> x = slice_by_size(x=inp,begin=b1,size=sz)[name=string(\"sx\")];\n", DIM, SEQ];
    
    // Step 1: Compute rrms = 1/sqrt(mean(x²) + eps)
    // sq = x * x
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> sq = mul(x=x,y=x)[name=string(\"sq\")];\n", DIM, SEQ];
    // ss = sum(sq, axis=1, keepdims=true)  → [1,1,1,SEQ]
    [m appendFormat:@"        tensor<int32, [1]> rax = const()[name=string(\"rax\"), val=tensor<int32, [1]>([1])];\n"];
    [m appendFormat:@"        bool kd = const()[name=string(\"kd\"), val=bool(true)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> ss = reduce_sum(x=sq,axes=rax,keep_dims=kd)[name=string(\"ss\")];\n", SEQ];
    // ss2 = ss * invd + eps
    [m appendFormat:@"        fp16 invd = const()[name=string(\"invd\"), val=fp16(%f)];\n", invd];
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> ss2 = mul(x=ss,y=invd)[name=string(\"ss2\")];\n", SEQ];
    [m appendFormat:@"        fp16 eps = const()[name=string(\"eps\"), val=fp16(0.00001)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> ss3 = add(x=ss2,y=eps)[name=string(\"ss3\")];\n", SEQ];
    // rrms = pow(ss3, -0.5) → [1,1,1,SEQ]
    [m appendFormat:@"        fp16 nhalf = const()[name=string(\"nhalf\"), val=fp16(-0.5)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> rrms = pow(x=ss3,y=nhalf)[name=string(\"rrms\")];\n", SEQ];
    
    // Step 2: Load RMSNorm weights w [1, DIM, 1, 1]
    [m appendFormat:@"        tensor<fp16, [1,%d,1,1]> w = const()[name=string(\"w\"), val=tensor<fp16, [1,%d,1,1]>(BLOBFILE(path=string(\"@model_path/weights/rms_w.bin\"), offset=uint64(64)))];\n", DIM, DIM];
    
    // Step 3: Compute dot = sum(dy * w * x, axis=1) * invd * rrms²
    // dyw = dy * w  → [1, DIM, 1, SEQ]
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> dyw = mul(x=dy,y=w)[name=string(\"dyw\")];\n", DIM, SEQ];
    // dywx = dyw * x  → [1, DIM, 1, SEQ]
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> dywx = mul(x=dyw,y=x)[name=string(\"dywx\")];\n", DIM, SEQ];
    // dot_sum = sum(dywx, axis=1, keepdims=true)  → [1,1,1,SEQ]
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> dot_sum = reduce_sum(x=dywx,axes=rax,keep_dims=kd)[name=string(\"ds\")];\n", SEQ];
    // dot_scaled = dot_sum * invd  → [1,1,1,SEQ]
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> dot_sc = mul(x=dot_sum,y=invd)[name=string(\"dsc\")];\n", SEQ];
    // rrms_sq = rrms * rrms  → [1,1,1,SEQ]
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> rrms2 = mul(x=rrms,y=rrms)[name=string(\"rr2\")];\n", SEQ];
    // coeff = dot_scaled * rrms_sq  → [1,1,1,SEQ]
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> coeff = mul(x=dot_sc,y=rrms2)[name=string(\"cof\")];\n", SEQ];
    
    // Step 4: dx = (dy * w - x * coeff) * rrms
    // x_coeff = x * coeff  → [1, DIM, 1, SEQ]
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> xc = mul(x=x,y=coeff)[name=string(\"xc\")];\n", DIM, SEQ];
    // diff = dyw - xc  → [1, DIM, 1, SEQ]
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> diff = sub(x=dyw,y=xc)[name=string(\"dif\")];\n", DIM, SEQ];
    // dx = diff * rrms  → [1, DIM, 1, SEQ]
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> out = mul(x=diff,y=rrms)[name=string(\"out\")];\n", DIM, SEQ];
    
    [m appendString:@"    } -> (out);\n}\n"];
    return m;
}
