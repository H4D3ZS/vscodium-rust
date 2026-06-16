// ane_classifier.h — MIL generators for classifier matmul and softmax on ANE
// Replaces classifier cblas_sgemm and cross-entropy softmax from CPU
#pragma once
#include "stories_mil.h"

// ============================================================
// Classifier forward: logits = embed @ x_final
// embed: [VOCAB, DIM] baked as conv weight [VOCAB, DIM, 1, 1]
// x:     [1, DIM, 1, SEQ] input
// out:   [1, VOCAB, 1, SEQ] logits
//
// VOCAB=32000 output channels — this is the largest conv we've attempted.
// If it fails, we'll need to tile into smaller chunks.
// ============================================================
static NSString *gen_classifier_fwd(void) {
    NSMutableString *m = [NSMutableString string];
    [m appendString:MIL_HDR];
    [m appendFormat:@"    func main<ios18>(tensor<fp16, [1, %d, 1, %d]> x) {\n", DIM, SEQ];
    [m appendString:@CONV_CONST];
    [m appendFormat:@"        tensor<fp16, [%d,%d,1,1]> We = const()[name=string(\"We\"), "
        "val=tensor<fp16, [%d,%d,1,1]>(BLOBFILE(path=string(\"@model_path/weights/embed.bin\"), offset=uint64(64)))];\n",
        VOCAB, DIM, VOCAB, DIM];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> out = conv(dilations=dl,groups=gr,pad=pd,pad_type=pt,strides=st,weight=We,x=x)[name=string(\"cls\")];\n", VOCAB, SEQ];
    [m appendString:@"    } -> (out);\n}\n"];
    return m;
}

// ============================================================
// Classifier backward: dx = embed^T @ dlogits
// ANE rejects conv with 32000 input channels.
// Use matmul instead: reshape dlogits to [1, VOCAB, SEQ],
// bake embed^T as [1, DIM, VOCAB], matmul → [1, DIM, SEQ],
// reshape back to [1, DIM, 1, SEQ].
// ============================================================
static NSString *gen_classifier_bwd(void) {
    NSMutableString *m = [NSMutableString string];
    [m appendString:MIL_HDR];
    [m appendFormat:@"    func main<ios18>(tensor<fp16, [1, %d, 1, %d]> dl) {\n", VOCAB, SEQ];
    // Reshape dlogits from [1, VOCAB, 1, SEQ] to [1, VOCAB, SEQ]
    [m appendFormat:@"        tensor<int32, [3]> sh3 = const()[name=string(\"sh3\"), val=tensor<int32, [3]>([1,%d,%d])];\n", VOCAB, SEQ];
    [m appendFormat:@"        tensor<fp16, [1,%d,%d]> dl3 = reshape(shape=sh3,x=dl)[name=string(\"rdl\")];\n", VOCAB, SEQ];
    // embed_t as baked constant [1, DIM, VOCAB]
    [m appendFormat:@"        tensor<fp16, [1,%d,%d]> Wet = const()[name=string(\"Wet\"), "
        "val=tensor<fp16, [1,%d,%d]>(BLOBFILE(path=string(\"@model_path/weights/embed_t.bin\"), offset=uint64(64)))];\n",
        DIM, VOCAB, DIM, VOCAB];
    // matmul: [1, DIM, VOCAB] @ [1, VOCAB, SEQ] -> [1, DIM, SEQ]
    [m appendString:@"        bool bF = const()[name=string(\"bF\"), val=bool(false)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,%d,%d]> dx3 = matmul(transpose_x=bF,transpose_y=bF,x=Wet,y=dl3)[name=string(\"mm\")];\n", DIM, SEQ];
    // Reshape back to [1, DIM, 1, SEQ]
    [m appendFormat:@"        tensor<int32, [4]> sh4 = const()[name=string(\"sh4\"), val=tensor<int32, [4]>([1,%d,1,%d])];\n", DIM, SEQ];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> out = reshape(shape=sh4,x=dx3)[name=string(\"out\")];\n", DIM, SEQ];
    [m appendString:@"    } -> (out);\n}\n"];
    return m;
}

// ============================================================
// Softmax over VOCAB dimension (channel axis) for cross-entropy
// Input:  logits [1, VOCAB, 1, SEQ]
// Output: probs  [1, VOCAB, 1, SEQ]
//
// softmax(x, axis=1) = exp(x - max(x)) / sum(exp(x - max(x)))
//
// Note: After getting probs from ANE, the NLL loss + gradient
// (prob[target] -= 1.0) are done on CPU since they need target indexing.
// ============================================================
static NSString *gen_softmax_vocab(void) {
    NSMutableString *m = [NSMutableString string];
    [m appendString:MIL_HDR];
    [m appendFormat:@"    func main<ios18>(tensor<fp16, [1, %d, 1, %d]> x) {\n", VOCAB, SEQ];
    [m appendString:@"        int32 ax = const()[name=string(\"ax\"), val=int32(1)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> out = softmax(axis=ax,x=x)[name=string(\"sm\")];\n", VOCAB, SEQ];
    [m appendString:@"    } -> (out);\n}\n"];
    return m;
}

// ============================================================
// Final RMSNorm on ANE (replaces CPU rmsnorm for final layer)
// Input:  x [1, DIM, 1, SEQ]
// Baked:  rms_final weights [DIM]
// Output: xn [1, DIM, 1, SEQ]
// ============================================================
static NSString *gen_final_rmsnorm(void) {
    float invd = 1.0f/(float)DIM;
    NSMutableString *m = [NSMutableString string];
    [m appendString:MIL_HDR];
    [m appendFormat:@"    func main<ios18>(tensor<fp16, [1, %d, 1, %d]> x) {\n", DIM, SEQ];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> sq = mul(x=x,y=x)[name=string(\"sq\")];\n", DIM, SEQ];
    [m appendFormat:@"        tensor<int32, [1]> rax = const()[name=string(\"rax\"), val=tensor<int32, [1]>([1])];\n"];
    [m appendFormat:@"        bool kd = const()[name=string(\"kd\"), val=bool(true)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> ss = reduce_sum(x=sq,axes=rax,keep_dims=kd)[name=string(\"ss\")];\n", SEQ];
    [m appendFormat:@"        fp16 invd = const()[name=string(\"invd\"), val=fp16(%f)];\n", invd];
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> ss2 = mul(x=ss,y=invd)[name=string(\"ss2\")];\n", SEQ];
    [m appendFormat:@"        fp16 eps = const()[name=string(\"eps\"), val=fp16(0.00001)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> ss3 = add(x=ss2,y=eps)[name=string(\"ss3\")];\n", SEQ];
    [m appendFormat:@"        fp16 nhalf = const()[name=string(\"nhalf\"), val=fp16(-0.5)];\n"];
    [m appendFormat:@"        tensor<fp16, [1,1,1,%d]> rrms = pow(x=ss3,y=nhalf)[name=string(\"rrms\")];\n", SEQ];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> xr = mul(x=x,y=rrms)[name=string(\"xr\")];\n", DIM, SEQ];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,1]> rw = const()[name=string(\"rw\"), val=tensor<fp16, [1,%d,1,1]>(BLOBFILE(path=string(\"@model_path/weights/rms_w.bin\"), offset=uint64(64)))];\n", DIM, DIM];
    [m appendFormat:@"        tensor<fp16, [1,%d,1,%d]> out = mul(x=xr,y=rw)[name=string(\"out\")];\n", DIM, SEQ];
    [m appendString:@"    } -> (out);\n}\n"];
    return m;
}
