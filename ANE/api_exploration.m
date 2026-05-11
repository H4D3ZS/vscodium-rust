#import <Foundation/Foundation.h>
#import <CoreML/CoreML.h>
#import <objc/runtime.h>
#import <objc/message.h>
#import <dlfcn.h>
#import <mach/mach_time.h>
#import <IOSurface/IOSurface.h>

static mach_timebase_info_data_t g_tb;
static double ticksToMs(uint64_t t) { return (double)t * g_tb.numer / g_tb.denom / 1e6; }

int main() {
    @autoreleasepool {
        mach_timebase_info(&g_tb);
        dlopen("/System/Library/PrivateFrameworks/AppleNeuralEngine.framework/AppleNeuralEngine", RTLD_NOW);

        // === Approach 1: MLModelAsset from compiled .mlmodelc data ===
        // First compile a known-working model to .mlmodelc
        printf("=== Approach 1: MLModelAsset in-memory ===\n");
        NSError *e = nil;
        NSURL *src = [NSURL fileURLWithPath:@"/tmp/ane_sram_1024ch_64sp.mlpackage"];
        NSURL *compiled = [MLModel compileModelAtURL:src error:&e];
        if (e) { printf("Compile failed: %s\n", [[e description] UTF8String]); return 1; }
        printf("Compiled to: %s\n", [[compiled path] UTF8String]);

        // Read the model.mlmodel spec from the compiled bundle
        // The spec is typically in coremldata.bin or model.mlmodel
        NSFileManager *fm = [NSFileManager defaultManager];
        NSArray *files = [fm contentsOfDirectoryAtPath:[compiled path] error:nil];
        printf("Files in .mlmodelc:\n");
        for (NSString *f in files) printf("  %s\n", [f UTF8String]);

        // Try loading with MLModelAsset
        // MLModelAsset has modelAssetWithURL: on macOS 15
        if (@available(macOS 14.0, *)) {
            // Read the spec data
            NSString *specPath = [[compiled path] stringByAppendingPathComponent:@"coremldata.bin"];
            if (![fm fileExistsAtPath:specPath]) {
                specPath = [[compiled path] stringByAppendingPathComponent:@"model.mlmodel"];
            }
            NSData *specData = [NSData dataWithContentsOfFile:specPath];
            printf("Spec data: %lu bytes from %s\n", (unsigned long)[specData length],
                   [[specPath lastPathComponent] UTF8String]);

            // Try MLModelAsset
            Class assetClass = NSClassFromString(@"MLModelAsset");
            if (assetClass) {
                printf("MLModelAsset class found\n");
                // List methods
                unsigned int count;
                Method *methods = class_copyMethodList(object_getClass(assetClass), &count);
                for (unsigned int i = 0; i < count; i++)
                    printf("  + %s\n", sel_getName(method_getName(methods[i])));
                free(methods);
            }
        }

        // === Approach 2: Read a .mlmodelc, extract MIL, feed to _ANEInMemoryModelDescriptor ===
        printf("\n=== Approach 2: Inspect MIL in compiled model ===\n");
        // Look for model.mil or any MIL file
        NSDirectoryEnumerator *en = [fm enumeratorAtPath:[compiled path]];
        NSString *f;
        while ((f = [en nextObject])) {
            NSString *full = [[compiled path] stringByAppendingPathComponent:f];
            BOOL isDir;
            [fm fileExistsAtPath:full isDirectory:&isDir];
            if (!isDir) {
                NSDictionary *attrs = [fm attributesOfItemAtPath:full error:nil];
                printf("  %s (%llu bytes)\n", [f UTF8String],
                       [[attrs objectForKey:NSFileSize] unsignedLongLongValue]);
            }
        }

        // Try to find and read model.mil
        NSString *milPath = [[compiled path] stringByAppendingPathComponent:@"model.mil"];
        if ([fm fileExistsAtPath:milPath]) {
            NSString *milText = [NSString stringWithContentsOfFile:milPath encoding:NSUTF8StringEncoding error:nil];
            printf("\n=== model.mil contents (first 2000 chars) ===\n");
            printf("%s\n", [[milText substringToIndex:MIN(2000, [milText length])] UTF8String]);
        }

        // Also check for mlmodelc structure
        NSString *aneDir = nil;
        en = [fm enumeratorAtPath:[compiled path]];
        while ((f = [en nextObject])) {
            if ([f hasSuffix:@".espresso.net"] || [f hasSuffix:@".hwx"] || [f hasSuffix:@".mil"]) {
                printf("  FOUND: %s\n", [f UTF8String]);
            }
        }

        // === Approach 3: Try _ANEInMemoryModelDescriptor with actual MIL from compiled model ===
        printf("\n=== Approach 3: _ANEInMemoryModelDescriptor ===\n");
        Class ANEInMemDesc = NSClassFromString(@"_ANEInMemoryModelDescriptor");
        if (ANEInMemDesc) {
            printf("Class exists. Methods:\n");
            unsigned int count;
            Method *methods = class_copyMethodList(object_getClass(ANEInMemDesc), &count);
            for (unsigned int i = 0; i < count; i++) {
                SEL s = method_getName(methods[i]);
                printf("  + %s  (args: %d)\n", sel_getName(s), method_getNumberOfArguments(methods[i]));
            }
            free(methods);
            methods = class_copyMethodList(ANEInMemDesc, &count);
            printf("Instance methods:\n");
            for (unsigned int i = 0; i < count; i++) {
                SEL s = method_getName(methods[i]);
                const char *enc = method_getTypeEncoding(methods[i]);
                printf("  - %s  [%s]\n", sel_getName(s), enc ? enc : "?");
            }
            free(methods);

            // If model.mil exists, try feeding it
            if ([fm fileExistsAtPath:milPath]) {
                NSString *milText = [NSString stringWithContentsOfFile:milPath encoding:NSUTF8StringEncoding error:nil];
                printf("\nTrying modelWithMILText: with actual model.mil...\n");
                id desc = ((id(*)(Class,SEL,id,id,id))objc_msgSend)(
                    ANEInMemDesc, @selector(modelWithMILText:weights:optionsPlist:),
                    milText, nil, nil);
                printf("Result: %s\n", desc ? [[desc description] UTF8String] : "nil");

                // Try with NSData
                NSData *milData = [milText dataUsingEncoding:NSUTF8StringEncoding];
                desc = ((id(*)(Class,SEL,id,id,id))objc_msgSend)(
                    ANEInMemDesc, @selector(modelWithMILText:weights:optionsPlist:),
                    milData, nil, nil);
                printf("Result (NSData): %s\n", desc ? [[desc description] UTF8String] : "nil");
            }
        } else {
            printf("_ANEInMemoryModelDescriptor NOT FOUND\n");
        }

        // === Approach 4: Hook into what CoreML actually sends to ANE ===
        printf("\n=== Approach 4: Trace CoreML -> ANE path ===\n");
        // Load the model the normal working way and inspect the _ANEModel
        MLModelConfiguration *config = [[MLModelConfiguration alloc] init];
        config.computeUnits = MLComputeUnitsAll;
        MLModel *model = [MLModel modelWithContentsOfURL:compiled configuration:config error:&e];
        if (e) { printf("MLModel load failed: %s\n", [[e description] UTF8String]); return 1; }

        // Try to get internal model object
        printf("MLModel: %s\n", [[model description] UTF8String]);

        // Check if we can access the ANE model through the MLModel
        // Try KVC for internal properties
        @try {
            id engine = [model valueForKey:@"engine"];
            printf("engine: %s\n", engine ? [[engine description] UTF8String] : "nil");
        } @catch(NSException *ex) {
            printf("No 'engine' key\n");
        }
        @try {
            id proxy = [model valueForKey:@"proxy"];
            printf("proxy: %s\n", proxy ? [NSStringFromClass([proxy class]) UTF8String] : "nil");
        } @catch(NSException *ex) {
            printf("No 'proxy' key\n");
        }

        // Check MLNeuralNetworkEngine or MLANEEngine
        Class aneEngine = NSClassFromString(@"MLANEEngine");
        Class nnEngine = NSClassFromString(@"MLNeuralNetworkEngine");
        Class milEngine = NSClassFromString(@"MLMILComputeEngine");
        printf("MLANEEngine: %s\n", aneEngine ? "exists" : "not found");
        printf("MLNeuralNetworkEngine: %s\n", nnEngine ? "exists" : "not found");
        printf("MLMILComputeEngine: %s\n", milEngine ? "exists" : "not found");
    }
    return 0;
}
