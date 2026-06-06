// sim-input.m
//
// Headless HID input forwarder for the booted iOS Simulator.
//
// Reads NDJSON events on stdin and synthesises Indigo HID messages, sending
// them via SimulatorKit's SimDeviceLegacyHIDClient. No Simulator.app needed.
//
// Event schema (one JSON object per line):
//   {"type":"touch","phase":"down|move|up","x":0..1,"y":0..1}
//   {"type":"button","name":"home|lock|side|siri","phase":"down|up"}
//   {"type":"key","keyCode":4,"phase":"down|up"}       // USB HID keycode
//   {"type":"keyboard","usage":40,"phase":"down|up"}   // USB HID usage code
//   {"type":"key-tap","usage":40,"modifiers":[227]}    // optional modifier usages
//   {"type":"tap","x":0..1,"y":0..1,"hold":150}      // convenience
//   {"type":"button-tap","name":"home"}              // convenience
//
// x/y are normalised display ratios (0,0 = top-left, 1,1 = bottom-right).
//
// Compile:
//   clang -fobjc-arc -O2 -framework Foundation -framework CoreGraphics \
//     sim-input.m -o sim-input
//
// The helper dlopens CoreSimulator and SimulatorKit at runtime, so the build
// has no private-framework dependencies.
//
// Wire format (Indigo) ported from facebook/idb's FBSimulatorIndigoHID.

#import <Foundation/Foundation.h>
#import <CoreGraphics/CoreGraphics.h>
#import <objc/runtime.h>
#import <objc/message.h>
#import <dlfcn.h>
#import <mach/mach.h>
#import <mach/mach_time.h>
#import <malloc/malloc.h>

#pragma pack(push, 4)

// Mach message header used by Indigo (matches mach_msg_header_t prefix).
typedef struct {
    unsigned int  msgh_bits;
    unsigned int  msgh_size;
    unsigned int  msgh_remote_port;
    unsigned int  msgh_local_port;
    unsigned int  msgh_voucher_port;
    unsigned int  msgh_id;
} IndigoMachHeader;

typedef struct {
    unsigned int field1;
    unsigned int field2;
    unsigned int field3;
    double xRatio;
    double yRatio;
    double field6;
    double field7;
    double field8;
    unsigned int field9;
    unsigned int field10;
    unsigned int field11;
    unsigned int field12;
    unsigned int field13;
    double field14;
    double field15;
    double field16;
    double field17;
    double field18;
} IndigoTouch;

typedef struct {
    unsigned int eventSource;
    unsigned int eventType;
    unsigned int eventTarget;
    unsigned int keyCode;
    unsigned int field5;
} IndigoButton;

typedef union {
    IndigoTouch touch;
    IndigoButton button;
    unsigned char raw[144];
} IndigoEvent;

typedef struct {
    unsigned int field1;            // 0x20 (eventKind for guest dispatch)
    unsigned long long timestamp;   // 0x24
    unsigned int field3;            // 0x2c
    IndigoEvent event;              // 0x30
} IndigoPayload;

typedef struct {
    IndigoMachHeader header;        // 0x00
    unsigned int innerSize;         // 0x18 — always 0xa0 (160)
    unsigned char eventType;        // 0x1c — 1 button/keyboard, 2 touch
    IndigoPayload payload;          // 0x20
} IndigoMessage;

#pragma pack(pop)

#define IndigoEventTypeButton 1
#define IndigoEventTypeTouch  2

#define ButtonEventSourceHomeButton 0x0
#define ButtonEventSourceLock       0x1
#define ButtonEventSourceSideButton 0xbb8
#define ButtonEventSourceSiri       0x400002
#define ButtonEventSourceApplePay   0x1f4

#define ButtonEventTargetHardware   0x33
#define ButtonEventTypeDown         0x1
#define ButtonEventTypeUp           0x2

// Indigo C-function pointer types
typedef IndigoMessage *(*IndigoButtonFn)(int keyCode, int op, int target);
typedef IndigoMessage *(*IndigoKeyboardFn)(uint32_t usageCode, int op);
typedef IndigoMessage *(*IndigoMouseFn)(CGPoint *point0, CGPoint *point1, int target, int eventType, BOOL extra);

// ───────────────────────────────────────────────────────────────────────────
// Logging
// ───────────────────────────────────────────────────────────────────────────

static void elog(NSString *fmt, ...) {
    va_list a; va_start(a, fmt);
    NSString *s = [[NSString alloc] initWithFormat:fmt arguments:a];
    va_end(a);
    NSData *d = [[s stringByAppendingString:@"\n"] dataUsingEncoding:NSUTF8StringEncoding];
    [[NSFileHandle fileHandleWithStandardError] writeData:d];
}

// ───────────────────────────────────────────────────────────────────────────
// Bootstrap CoreSimulator → booted SimDevice (mirrors sim-capture.swift)
// ───────────────────────────────────────────────────────────────────────────

static NSString *developerDir(void) {
    NSTask *t = [NSTask new];
    t.launchPath = @"/usr/bin/xcode-select";
    t.arguments = @[@"-p"];
    NSPipe *p = [NSPipe pipe];
    t.standardOutput = p;
    @try { [t launch]; [t waitUntilExit]; } @catch (id e) {}
    NSString *s = [[NSString alloc] initWithData:p.fileHandleForReading.readDataToEndOfFile encoding:NSUTF8StringEncoding];
    s = [s stringByTrimmingCharactersInSet:NSCharacterSet.whitespaceAndNewlineCharacterSet];
    return s.length ? s : @"/Applications/Xcode.app/Contents/Developer";
}

static id sharedServiceContext(void) {
    Class C = NSClassFromString(@"SimServiceContext");
    if (!C) { elog(@"[sim-input] SimServiceContext missing"); return nil; }
    SEL sel = @selector(sharedServiceContextForDeveloperDir:error:);
    NSError *err = nil;
    id (*fn)(Class, SEL, NSString *, NSError **) = (id (*)(Class, SEL, NSString *, NSError **))objc_msgSend;
    id ctx = fn(C, sel, developerDir(), &err);
    if (!ctx) elog(@"[sim-input] sharedServiceContext err: %@", err);
    return ctx;
}

static id defaultDeviceSet(id ctx) {
    SEL sel = @selector(defaultDeviceSetWithError:);
    NSError *err = nil;
    id (*fn)(id, SEL, NSError **) = (id (*)(id, SEL, NSError **))objc_msgSend;
    id ds = fn(ctx, sel, &err);
    if (!ds) elog(@"[sim-input] defaultDeviceSet err: %@", err);
    return ds;
}

static NSString *gTargetUDID = nil;

static id bootedDevice(id deviceSet) {
    NSArray *devices = [deviceSet valueForKey:@"devices"];
    for (id d in devices) {
        NSNumber *st = [d valueForKey:@"state"];
        if (st.intValue != 3) continue; // Booted
        if (gTargetUDID.length) {
            NSString *udid = [[d valueForKey:@"UDID"] description];
            if (![udid isEqualToString:gTargetUDID]) continue;
        }
        return d;
    }
    return nil;
}

// ───────────────────────────────────────────────────────────────────────────
// HID client + Indigo function table
// ───────────────────────────────────────────────────────────────────────────

static id gHidClient = nil;
static IndigoButtonFn gButtonFn = NULL;
static IndigoKeyboardFn gKeyboardFn = NULL;
static IndigoMouseFn  gMouseFn  = NULL;
static dispatch_queue_t gSendQueue;

static BOOL ensureHID(void) {
    if (gHidClient) return YES;

    if (!dlopen("/Library/Developer/PrivateFrameworks/CoreSimulator.framework/CoreSimulator", RTLD_NOW)) {
        elog(@"[sim-input] FAIL dlopen CoreSimulator: %s", dlerror());
        return NO;
    }
    NSString *kitPath = [developerDir() stringByAppendingPathComponent:@"Library/PrivateFrameworks/SimulatorKit.framework/SimulatorKit"];
    void *kit = dlopen(kitPath.fileSystemRepresentation, RTLD_NOW);
    if (!kit) {
        elog(@"[sim-input] FAIL dlopen SimulatorKit (%@): %s", kitPath, dlerror());
        return NO;
    }
    gButtonFn = (IndigoButtonFn) dlsym(kit, "IndigoHIDMessageForButton");
    gKeyboardFn = (IndigoKeyboardFn) dlsym(kit, "IndigoHIDMessageForKeyboardArbitrary");
    gMouseFn  = (IndigoMouseFn)  dlsym(kit, "IndigoHIDMessageForMouseNSEvent");
    if (!gButtonFn || !gKeyboardFn || !gMouseFn) {
        elog(@"[sim-input] FAIL Indigo dlsym button=%p keyboard=%p mouse=%p", gButtonFn, gKeyboardFn, gMouseFn);
        return NO;
    }

    id ctx = sharedServiceContext(); if (!ctx) return NO;
    id ds  = defaultDeviceSet(ctx);  if (!ds)  return NO;
    id dev = bootedDevice(ds);
    if (!dev) {
        if (gTargetUDID.length) elog(@"[sim-input] no booted device matching %@", gTargetUDID);
        else elog(@"[sim-input] no booted device");
        return NO;
    }

    Class clientCls = objc_lookUpClass("_TtC12SimulatorKit24SimDeviceLegacyHIDClient");
    if (!clientCls) clientCls = NSClassFromString(@"SimulatorKit.SimDeviceLegacyHIDClient");
    if (!clientCls) { elog(@"[sim-input] FAIL no SimDeviceLegacyHIDClient class"); return NO; }

    NSError *err = nil;
    id alloc = [clientCls alloc];
    SEL sel = @selector(initWithDevice:error:);
    id (*initFn)(id, SEL, id, NSError **) = (id (*)(id, SEL, id, NSError **))objc_msgSend;
    id client = initFn(alloc, sel, dev, &err);
    if (!client) { elog(@"[sim-input] FAIL init HID client: %@", err); return NO; }
    gHidClient = client;
    gSendQueue = dispatch_queue_create("co.bennett.ios-sim.input", DISPATCH_QUEUE_SERIAL);
    elog(@"[sim-input] HID client ready dev=%@ udid=%@", [dev valueForKey:@"name"], [dev valueForKey:@"UDID"]);
    return YES;
}

static void sendIndigo(IndigoMessage *msg) {
    if (!gHidClient || !msg) return;
    SEL sel = @selector(sendWithMessage:freeWhenDone:completionQueue:completion:);
    void (^cb)(NSError *) = ^(NSError *err) {
        if (err) elog(@"[sim-input] send err: %@", err);
    };
    void (*sendFn)(id, SEL, IndigoMessage *, BOOL, dispatch_queue_t, void(^)(NSError *)) =
        (void (*)(id, SEL, IndigoMessage *, BOOL, dispatch_queue_t, void(^)(NSError *)))objc_msgSend;
    sendFn(gHidClient, sel, msg, YES, gSendQueue, cb);
}

// ───────────────────────────────────────────────────────────────────────────
// Touch message construction (port of FBSimulatorIndigoHID.touchMessageWith…)
// ───────────────────────────────────────────────────────────────────────────

static void sendTouch(double xRatio, double yRatio, BOOL down) {
    if (!gMouseFn) return;
    CGPoint pt = CGPointMake(xRatio, yRatio);
    int evtType = down ? ButtonEventTypeDown : ButtonEventTypeUp;
    IndigoMessage *seed = gMouseFn(&pt, NULL, 0x32, evtType, NO);
    if (!seed) { elog(@"[sim-input] MouseFn returned NULL"); return; }

    // Allocate canonical 320-byte two-payload message
    size_t messageSize = sizeof(IndigoMessage) + sizeof(IndigoPayload);
    size_t stride = sizeof(IndigoPayload);
    IndigoMessage *msg = calloc(1, messageSize);
    msg->innerSize = (unsigned int) sizeof(IndigoPayload);
    msg->eventType = IndigoEventTypeTouch;
    msg->payload.field1 = 0x0000000b;
    msg->payload.timestamp = mach_absolute_time();

    // Copy the IndigoTouch produced by the seed message
    memcpy(&msg->payload.event.touch, &seed->payload.event.touch, sizeof(IndigoTouch));
    msg->payload.event.touch.xRatio = xRatio;
    msg->payload.event.touch.yRatio = yRatio;

    // Duplicate payload into second slot, tweak field1/field2
    void *first = &msg->payload;
    void *second = (void *)((uintptr_t)first + stride);
    memcpy(second, first, stride);
    IndigoPayload *secondP = (IndigoPayload *) second;
    secondP->event.touch.field1 = 0x00000001;
    secondP->event.touch.field2 = 0x00000002;

    free(seed);
    sendIndigo(msg);
}

static void sendButton(NSString *name, BOOL down) {
    if (!gButtonFn) return;
    int src = ButtonEventSourceHomeButton;
    if      ([name isEqualToString:@"home"]) src = ButtonEventSourceHomeButton;
    else if ([name isEqualToString:@"lock"]) src = ButtonEventSourceLock;
    else if ([name isEqualToString:@"side"]) src = ButtonEventSourceSideButton;
    else if ([name isEqualToString:@"siri"]) src = ButtonEventSourceSiri;
    else if ([name isEqualToString:@"applepay"]) src = ButtonEventSourceApplePay;
    else { elog(@"[sim-input] unknown button %@", name); return; }
    int op = down ? ButtonEventTypeDown : ButtonEventTypeUp;
    IndigoMessage *m = gButtonFn(src, op, ButtonEventTargetHardware);
    sendIndigo(m);
}

static void sendKey(uint32_t keyCode, BOOL down) {
    if (!gKeyboardFn) return;
    int op = down ? ButtonEventTypeDown : ButtonEventTypeUp;
    IndigoMessage *m = gKeyboardFn(keyCode, op);
    sendIndigo(m);
}

static void sendKeyTap(uint32_t usage, NSArray *modifiers) {
    NSMutableArray *validModifiers = [NSMutableArray new];
    for (id value in modifiers ?: @[]) {
        if (![value respondsToSelector:@selector(unsignedIntValue)]) continue;
        NSNumber *usageNumber = @([value unsignedIntValue]);
        [validModifiers addObject:usageNumber];
        sendKey(usageNumber.unsignedIntValue, YES);
    }

    sendKey(usage, YES);
    usleep(10000);
    sendKey(usage, NO);

    for (NSInteger i = (NSInteger)validModifiers.count - 1; i >= 0; i--) {
        NSNumber *usageNumber = validModifiers[(NSUInteger)i];
        sendKey(usageNumber.unsignedIntValue, NO);
    }
}

// ───────────────────────────────────────────────────────────────────────────
// stdin event loop
// ───────────────────────────────────────────────────────────────────────────

static void processEvent(NSDictionary *evt) {
    if (!ensureHID()) return;
    NSString *type = evt[@"type"];
    if ([type isEqualToString:@"touch"]) {
        NSString *phase = evt[@"phase"] ?: @"down";
        double x = [evt[@"x"] doubleValue];
        double y = [evt[@"y"] doubleValue];
        if ([phase isEqualToString:@"up"]) sendTouch(x, y, NO);
        else sendTouch(x, y, YES); // down + move both keep finger on screen
    } else if ([type isEqualToString:@"tap"]) {
        double x = [evt[@"x"] doubleValue];
        double y = [evt[@"y"] doubleValue];
        int hold = (int)([evt[@"hold"] doubleValue] ?: 80);
        sendTouch(x, y, YES);
        usleep((useconds_t)(hold * 1000));
        sendTouch(x, y, NO);
    } else if ([type isEqualToString:@"button"]) {
        NSString *phase = evt[@"phase"] ?: @"down";
        sendButton(evt[@"name"] ?: @"home", [phase isEqualToString:@"down"]);
    } else if ([type isEqualToString:@"button-tap"]) {
        NSString *name = evt[@"name"] ?: @"home";
        sendButton(name, YES);
        usleep(80000);
        sendButton(name, NO);
    } else if ([type isEqualToString:@"key"]) {
        NSString *phase = evt[@"phase"] ?: @"down";
        sendKey((uint32_t)[evt[@"keyCode"] unsignedIntValue], [phase isEqualToString:@"down"]);
    } else if ([type isEqualToString:@"keyboard"]) {
        NSNumber *usage = evt[@"usage"];
        if (!usage) { elog(@"[sim-input] keyboard missing usage"); return; }
        NSString *phase = evt[@"phase"] ?: @"down";
        sendKey(usage.unsignedIntValue, [phase isEqualToString:@"down"]);
    } else if ([type isEqualToString:@"key-tap"]) {
        NSNumber *usage = evt[@"usage"];
        if (!usage) { elog(@"[sim-input] key-tap missing usage"); return; }
        NSArray *modifiers = [evt[@"modifiers"] isKindOfClass:NSArray.class] ? evt[@"modifiers"] : @[];
        sendKeyTap(usage.unsignedIntValue, modifiers);
    } else {
        elog(@"[sim-input] unknown event type: %@", type);
    }
}

int main(int argc, const char **argv) {
    @autoreleasepool {
        if (argc > 1 && argv[1] && argv[1][0]) {
            gTargetUDID = [NSString stringWithUTF8String:argv[1]];
            elog(@"[sim-input] target udid=%@", gTargetUDID);
        }
        // Pre-warm: try to attach now so first event has no latency
        ensureHID();
        elog(@"[sim-input] ready");

        NSFileHandle *in = [NSFileHandle fileHandleWithStandardInput];
        NSMutableData *buf = [NSMutableData new];
        while (true) {
            NSData *chunk;
            @try { chunk = [in availableData]; } @catch (id e) { break; }
            if (chunk.length == 0) break;
            [buf appendData:chunk];
            while (true) {
                const char *bytes = (const char *)buf.bytes;
                NSUInteger len = buf.length;
                NSUInteger nl = NSNotFound;
                for (NSUInteger i = 0; i < len; i++) if (bytes[i] == '\n') { nl = i; break; }
                if (nl == NSNotFound) break;
                NSData *line = [buf subdataWithRange:NSMakeRange(0, nl)];
                [buf replaceBytesInRange:NSMakeRange(0, nl + 1) withBytes:NULL length:0];
                if (line.length == 0) continue;
                NSError *err = nil;
                id obj = [NSJSONSerialization JSONObjectWithData:line options:0 error:&err];
                if (![obj isKindOfClass:NSDictionary.class]) {
                    elog(@"[sim-input] bad JSON: %@", err ?: line);
                    continue;
                }
                processEvent((NSDictionary *)obj);
            }
        }
        elog(@"[sim-input] stdin closed, exiting");
    }
    return 0;
}
