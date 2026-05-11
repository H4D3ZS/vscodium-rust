#import <Foundation/Foundation.h>
#import <CoreML/CoreML.h>
#import <objc/runtime.h>
#import <objc/message.h>
#import <dlfcn.h>
#import <mach/mach_time.h>
#import <IOSurface/IOSurface.h>

static mach_timebase_info_data_t g_tb;
static double ticksToMs(uint64_t t) { return (double)t * g_tb.numer / g_tb.denom / 1e6; }

NSData *buildWeightBlob(int ch, int depth) {
    NSUInteger wsize = ch * ch * 2;
    NSUInteger chunkSize = 64 + wsize;
    NSUInteger total = 64 + chunkSize * depth;
    uint8_t *buf = calloc(total, 1);
    buf[0] = 0x01; buf[4] = 0x02;
    for (int i = 0; i < depth; i++) {
        uint8_t *chunk = buf + 64 + i * chunkSize;
        chunk[0]=0xEF; chunk[1]=0xBE; chunk[2]=0xAD; chunk[3]=0xDE;
        chunk[4]=0x01; chunk[10]=0x08;
        uint16_t *fp16 = (uint16_t*)(chunk + 64);
        for (NSUInteger j = 0; j < wsize/2; j++) fp16[j] = (arc4random()&0x03FF)|0x2000;
    }
    return [NSData dataWithBytesNoCopy:buf length:total freeWhenDone:YES];
}

NSString *genMIL(int ch, int sp, int depth) {
    NSMutableString *m = [NSMutableString string];
    [m appendString:@"program(1.3)\n[buildInfo = dict<string, string>({{\"coremlc-component-MIL\", \"3510.2.1\"}, {\"coremlc-version\", \"3505.4.1\"}, {\"coremltools-component-milinternal\", \"\"}, {\"coremltools-version\", \"9.0\"}})]\n{\n"];
    [m appendFormat:@"    func main<ios18>(tensor<fp32, [1, %d, 1, %d]> x) {\n", ch, sp];
    [m appendString:@"            string c_pad_type_0 = const()[name = string(\"c_pad_type_0\"), val = string(\"valid\")];\n"
        @"            tensor<int32, [2]> c_strides_0 = const()[name = string(\"c_strides_0\"), val = tensor<int32, [2]>([1, 1])];\n"
        @"            tensor<int32, [4]> c_pad_0 = const()[name = string(\"c_pad_0\"), val = tensor<int32, [4]>([0, 0, 0, 0])];\n"
        @"            tensor<int32, [2]> c_dilations_0 = const()[name = string(\"c_dilations_0\"), val = tensor<int32, [2]>([1, 1])];\n"
        @"            int32 c_groups_0 = const()[name = string(\"c_groups_0\"), val = int32(1)];\n"
        @"            string x_to_fp16_dtype_0 = const()[name = string(\"x_to_fp16_dtype_0\"), val = string(\"fp16\")];\n"];
    [m appendFormat:@"            tensor<fp16, [1, %d, 1, %d]> x_to_fp16 = cast(dtype = x_to_fp16_dtype_0, x = x)[name = string(\"cast_in\")];\n", ch, sp];
    NSUInteger cs = 64 + ch*ch*2;
    NSString *prev = @"x_to_fp16";
    for (int i = 0; i < depth; i++) {
        [m appendFormat:@"            tensor<fp16, [%d, %d, 1, 1]> W%d = const()[name = string(\"W%d\"), val = tensor<fp16, [%d, %d, 1, 1]>(BLOBFILE(path = string(\"@model_path/weights/weight.bin\"), offset = uint64(%lu)))];\n",
            ch, ch, i, i, ch, ch, (unsigned long)(64 + i*cs)];
        NSString *out = [NSString stringWithFormat:@"c%d", i];
        [m appendFormat:@"            tensor<fp16, [1, %d, 1, %d]> %@ = conv(dilations = c_dilations_0, groups = c_groups_0, pad = c_pad_0, pad_type = c_pad_type_0, strides = c_strides_0, weight = W%d, x = %@)[name = string(\"%@\")];\n",
            ch, sp, out, i, prev, out];
        prev = out;
    }
    [m appendString:@"            string to_fp32 = const()[name = string(\"to_fp32\"), val = string(\"fp32\")];\n"];
    [m appendFormat:@"            tensor<fp32, [1, %d, 1, %d]> c = cast(dtype = to_fp32, x = %@)[name = string(\"cast_out\")];\n", ch, sp, prev];
    [m appendString:@"        } -> (c);\n}\n"];
    return m;
}

double bench(int ch, int sp, int depth) {
    @autoreleasepool {
        NSError *e = nil;
        NSData *milData = [[genMIL(ch,sp,depth) dataUsingEncoding:NSUTF8StringEncoding] copy];
        NSData *wb = buildWeightBlob(ch, depth);
        Class D=NSClassFromString(@"_ANEInMemoryModelDescriptor"), I=NSClassFromString(@"_ANEInMemoryModel");
        Class AR=NSClassFromString(@"_ANERequest"), AIO=NSClassFromString(@"_ANEIOSurfaceObject");
        id desc=((id(*)(Class,SEL,id,id,id))objc_msgSend)(D,@selector(modelWithMILText:weights:optionsPlist:),milData,@{@"@model_path/weights/weight.bin":@{@"offset":@0,@"data":wb}},nil);
        if(!desc)return -1;
        id mdl=((id(*)(Class,SEL,id))objc_msgSend)(I,@selector(inMemoryModelWithDescriptor:),desc);
        id hx=((id(*)(id,SEL))objc_msgSend)(mdl,@selector(hexStringIdentifier));
        NSString *td=[NSTemporaryDirectory() stringByAppendingPathComponent:hx];
        NSFileManager *fm=[NSFileManager defaultManager];
        [fm createDirectoryAtPath:[td stringByAppendingPathComponent:@"weights"] withIntermediateDirectories:YES attributes:nil error:nil];
        [milData writeToFile:[td stringByAppendingPathComponent:@"model.mil"] atomically:YES];
        [wb writeToFile:[td stringByAppendingPathComponent:@"weights/weight.bin"] atomically:YES];
        if(!((BOOL(*)(id,SEL,unsigned int,id,NSError**))objc_msgSend)(mdl,@selector(compileWithQoS:options:error:),21,@{},&e)){[fm removeItemAtPath:td error:nil];return -3;}
        if(!((BOOL(*)(id,SEL,unsigned int,id,NSError**))objc_msgSend)(mdl,@selector(loadWithQoS:options:error:),21,@{},&e)){[fm removeItemAtPath:td error:nil];return -4;}
        NSUInteger bytes=ch*sp*4;
        IOSurfaceRef ioI=IOSurfaceCreate((__bridge CFDictionaryRef)@{(id)kIOSurfaceWidth:@(bytes),(id)kIOSurfaceHeight:@1,(id)kIOSurfaceBytesPerElement:@1,(id)kIOSurfaceBytesPerRow:@(bytes),(id)kIOSurfaceAllocSize:@(bytes),(id)kIOSurfacePixelFormat:@0});
        IOSurfaceRef ioO=IOSurfaceCreate((__bridge CFDictionaryRef)@{(id)kIOSurfaceWidth:@(bytes),(id)kIOSurfaceHeight:@1,(id)kIOSurfaceBytesPerElement:@1,(id)kIOSurfaceBytesPerRow:@(bytes),(id)kIOSurfaceAllocSize:@(bytes),(id)kIOSurfacePixelFormat:@0});
        id wI=((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(AIO,@selector(objectWithIOSurface:),ioI);
        id wO=((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(AIO,@selector(objectWithIOSurface:),ioO);
        id req=((id(*)(Class,SEL,id,id,id,id,id,id,id))objc_msgSend)(AR,@selector(requestWithInputs:inputIndices:outputs:outputIndices:weightsBuffer:perfStats:procedureIndex:),@[wI],@[@0],@[wO],@[@0],nil,nil,@0);
        for(int i=0;i<10;i++)((BOOL(*)(id,SEL,unsigned int,id,id,NSError**))objc_msgSend)(mdl,@selector(evaluateWithQoS:options:request:error:),21,@{},req,&e);
        int it=50; uint64_t t0=mach_absolute_time();
        for(int i=0;i<it;i++)((BOOL(*)(id,SEL,unsigned int,id,id,NSError**))objc_msgSend)(mdl,@selector(evaluateWithQoS:options:request:error:),21,@{},req,&e);
        double ms=ticksToMs(mach_absolute_time()-t0)/it;
        ((BOOL(*)(id,SEL,unsigned int,NSError**))objc_msgSend)(mdl,@selector(unloadWithQoS:error:),21,&e);
        CFRelease(ioI);CFRelease(ioO);[fm removeItemAtPath:td error:nil];
        return ms;
    }
}

int main() {
    mach_timebase_info(&g_tb);
    dlopen("/System/Library/PrivateFrameworks/AppleNeuralEngine.framework/AppleNeuralEngine",RTLD_NOW);
    printf("=== Programmatic MIL → In-Memory ANE Peak ===\n\n");
    printf("%-28s %7s %7s %9s %7s %6s\n","Config","W(MB)","GFLOP","ms/eval","TFLOPS","%%peak");
    printf("----------------------------------------------------------------------\n");
    typedef struct{int c,s,d;}C;
    C cf[]={
        {512,64,32},{512,64,48},{512,64,64},{512,64,96},{512,64,128},
        {256,64,64},{256,64,128},{256,64,256},
        {384,64,64},{384,64,128},
    };
    for(int i=0;i<10;i++){
        int c=cf[i].c,s=cf[i].s,d=cf[i].d;
        double w=(double)c*c*2*d/1024/1024, gf=2.0*c*c*s*d/1e9;
        char l[64]; snprintf(l,64,"%dx conv %dch sp%d",d,c,s);
        double ms=bench(c,s,d);
        double tf=ms>0?gf/ms:0;
        if(ms>0)printf("%-28s %6.1f  %6.2f  %7.3f ms %6.2f  %5.1f%%\n",l,w,gf,ms,tf,tf/0.019*100);
        else printf("%-28s %6.1f  %6.2f  FAIL(%.0f)\n",l,w,gf,ms);
    }
    return 0;
}
