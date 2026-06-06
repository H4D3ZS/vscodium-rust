// sim-capture.swift
//
// Headless mirror of the booted iOS Simulator device.
// Uses CoreSimulator private API (SimServiceContext / SimDeviceIOClient /
// SimDisplayIOSurfaceRenderable) to read the device framebuffer's IOSurface
// directly, with no Simulator.app window required.
//
// Streams length-prefixed JPEGs to stdout:
//   [u32 big-endian length][JPEG bytes]
//
// Status JSON + diagnostics on stderr.
//
// Compile:
//   swiftc -O -F /Library/Developer/PrivateFrameworks \
//     -framework CoreImage -framework Foundation -framework IOSurface \
//     sim-capture.swift -o sim-capture
//
// The helper polls for a booted device, and for each new IOSurface posted
// by the simulator (typically @60Hz) re-encodes a JPEG. If no booted
// device exists, prints a "no-booted-device" status and waits. Re-discovers
// devices when one boots/quits.

import Foundation
import CoreImage
import CoreGraphics
import IOSurface
import ImageIO
import UniformTypeIdentifiers

func eprint(_ s: String) {
    FileHandle.standardError.write((s + "\n").data(using: .utf8) ?? Data())
}

// MARK: - dlopen private framework

let CS_PATH = "/Library/Developer/PrivateFrameworks/CoreSimulator.framework/CoreSimulator"
guard dlopen(CS_PATH, RTLD_NOW) != nil else {
    eprint("[sim-capture] FAIL dlopen CoreSimulator")
    exit(2)
}

guard let SimServiceContext = NSClassFromString("SimServiceContext") as? NSObject.Type,
      let renderableProto = NSProtocolFromString("SimDisplayIOSurfaceRenderable") else {
    eprint("[sim-capture] FAIL CoreSimulator runtime symbols missing")
    exit(2)
}

// Resolve developer dir
func developerDir() -> String {
    let p = Process(); p.launchPath = "/usr/bin/xcode-select"; p.arguments = ["-p"]
    let pipe = Pipe(); p.standardOutput = pipe
    do { try p.run() } catch { return "/Applications/Xcode.app/Contents/Developer" }
    p.waitUntilExit()
    let s = String(data: pipe.fileHandleForReading.readDataToEndOfFile(), encoding: .utf8) ?? ""
    let trimmed = s.trimmingCharacters(in: .whitespacesAndNewlines)
    return trimmed.isEmpty ? "/Applications/Xcode.app/Contents/Developer" : trimmed
}

// MARK: - Bootstrap SimServiceContext + DeviceSet

func bootstrap() -> AnyObject? {
    let sel = NSSelectorFromString("sharedServiceContextForDeveloperDir:error:")
    typealias Sig = @convention(c) (AnyObject, Selector, NSString,
        AutoreleasingUnsafeMutablePointer<NSError?>) -> AnyObject?
    guard let imp = SimServiceContext.method(for: sel) else { return nil }
    let fn = unsafeBitCast(imp, to: Sig.self)
    var err: NSError? = nil
    let ctx = withUnsafeMutablePointer(to: &err) { ep -> AnyObject? in
        fn(SimServiceContext, sel, developerDir() as NSString, AutoreleasingUnsafeMutablePointer(ep))
    }
    if ctx == nil { eprint("[sim-capture] sharedServiceContext err: \(String(describing: err))") }
    return ctx
}

func defaultDeviceSet(_ ctx: AnyObject) -> AnyObject? {
    let sel = NSSelectorFromString("defaultDeviceSetWithError:")
    typealias Sig = @convention(c) (AnyObject, Selector,
        AutoreleasingUnsafeMutablePointer<NSError?>) -> AnyObject?
    guard let imp = (ctx as! NSObject).method(for: sel) else { return nil }
    let fn = unsafeBitCast(imp, to: Sig.self)
    var err: NSError? = nil
    let ds = withUnsafeMutablePointer(to: &err) { ep -> AnyObject? in
        fn(ctx, sel, AutoreleasingUnsafeMutablePointer(ep))
    }
    if ds == nil { eprint("[sim-capture] defaultDeviceSet err: \(String(describing: err))") }
    return ds
}

func bootedDevice(_ deviceSet: AnyObject) -> NSObject? {
    guard let devices = (deviceSet as AnyObject).value(forKey: "devices") as? NSArray else { return nil }
    for d in devices {
        let dev = d as AnyObject
        let state = (dev.value(forKey: "state") as? NSNumber)?.intValue ?? -1
        if state == 3 /* booted */ { return dev as? NSObject }
    }
    return nil
}

// MARK: - Locate primary display descriptor

func findDisplayDescriptor(_ device: NSObject) -> NSObject? {
    guard let io = device.value(forKey: "io") as? NSObject,
          let ports = io.value(forKey: "ioPorts") as? NSArray else { return nil }
    let descSel = NSSelectorFromString("descriptor")
    let surfSel = NSSelectorFromString("framebufferSurface")
    typealias DescFn = @convention(c) (AnyObject, Selector) -> AnyObject?
    var best: NSObject? = nil
    var bestArea = 0
    for p in ports {
        let port = p as! NSObject
        guard let imp = port.method(for: descSel) else { continue }
        let fn = unsafeBitCast(imp, to: DescFn.self)
        guard let desc = fn(port, descSel) as? NSObject else { continue }
        if !desc.conforms(to: renderableProto) { continue }
        // Read framebuffer surface to determine size — pick largest display.
        guard let s = desc.perform(surfSel)?.takeUnretainedValue(),
              CFGetTypeID(s) == IOSurfaceGetTypeID() else { continue }
        let surf = unsafeBitCast(s, to: IOSurfaceRef.self) as IOSurface
        let area = IOSurfaceGetWidth(surf) * IOSurfaceGetHeight(surf)
        if area > bestArea {
            bestArea = area
            best = desc
        }
    }
    return best
}

// MARK: - Stream

final class Stream {
    let descriptor: NSObject
    let device: NSObject
    let ciContext = CIContext(options: [.useSoftwareRenderer: false])
    let stdout = FileHandle.standardOutput
    let writeLock = NSLock()
    let callbackUUID: NSUUID
    let damageCallbackUUID: NSUUID
    var registered = false
    var damageRegistered = false
    var frameCount = 0
    var lastReportTime = Date()
    var width: Int = 0
    var height: Int = 0
    var encoding = false  // drop-frame guard

    init(descriptor: NSObject, device: NSObject) {
        self.descriptor = descriptor
        self.device = device
        self.callbackUUID = NSUUID()
        self.damageCallbackUUID = NSUUID()
    }

    func start() {
        // Initial frame to prime + report dims
        let initial: AnyObject? = descriptor.perform(NSSelectorFromString("framebufferSurface"))?.takeUnretainedValue()
        if let s = initial, CFGetTypeID(s) == IOSurfaceGetTypeID() {
            let surf = unsafeBitCast(s, to: IOSurfaceRef.self) as IOSurface
            width = IOSurfaceGetWidth(surf)
            height = IOSurfaceGetHeight(surf)
            emitStreamStarted()
            handle(surface: surf)
        } else {
            eprint("[sim-capture] no initial framebufferSurface")
        }

        // Register IOSurface change callback (fires when the backing
        // IOSurface object swaps — typically rare).
        let regSel = NSSelectorFromString("registerCallbackWithUUID:ioSurfacesChangeCallback:")
        typealias RegSig = @convention(c) (AnyObject, Selector, NSUUID,
            @convention(block) (AnyObject?, AnyObject?) -> Void) -> Void
        if let imp = descriptor.method(for: regSel) {
            let fn = unsafeBitCast(imp, to: RegSig.self)
            let block: @convention(block) (AnyObject?, AnyObject?) -> Void = { [weak self] _, new in
                guard let self = self, let n = new,
                      CFGetTypeID(n) == IOSurfaceGetTypeID() else { return }
                let surf = unsafeBitCast(n, to: IOSurfaceRef.self) as IOSurface
                self.handle(surface: surf)
            }
            fn(descriptor, regSel, callbackUUID, block)
            registered = true
        } else {
            eprint("[sim-capture] FAIL no register selector")
        }

        // Register damage-rectangle callback (fires per visual update — the
        // IOSurface object often stays the same but pixels mutate, so this
        // is the correct trigger for ~60Hz mirror).
        let damageSel = NSSelectorFromString("registerCallbackWithUUID:damageRectanglesCallback:")
        typealias DamSig = @convention(c) (AnyObject, Selector, NSUUID,
            @convention(block) (AnyObject?) -> Void) -> Void
        if let imp = descriptor.method(for: damageSel) {
            let fn = unsafeBitCast(imp, to: DamSig.self)
            let block: @convention(block) (AnyObject?) -> Void = { [weak self] _ in
                guard let self = self else { return }
                guard let s = self.descriptor.perform(NSSelectorFromString("framebufferSurface"))?.takeUnretainedValue(),
                      CFGetTypeID(s) == IOSurfaceGetTypeID() else { return }
                let surf = unsafeBitCast(s, to: IOSurfaceRef.self) as IOSurface
                self.handle(surface: surf)
            }
            fn(descriptor, damageSel, damageCallbackUUID, block)
            damageRegistered = true
            eprint("[sim-capture] registered damage callback uuid=\(damageCallbackUUID.uuidString)")
        } else {
            eprint("[sim-capture] WARN no damage register selector")
        }
        if registered {
            eprint("[sim-capture] registered IOSurface change callback uuid=\(callbackUUID.uuidString)")
        }
    }

    func stop() {
        if registered {
            let sel = NSSelectorFromString("unregisterIOSurfacesChangeCallbackWithUUID:")
            typealias S = @convention(c) (AnyObject, Selector, NSUUID) -> Void
            if let imp = descriptor.method(for: sel) {
                let fn = unsafeBitCast(imp, to: S.self)
                fn(descriptor, sel, callbackUUID)
            }
            registered = false
        }
        if damageRegistered {
            let sel = NSSelectorFromString("unregisterDamageRectanglesCallbackWithUUID:")
            typealias S = @convention(c) (AnyObject, Selector, NSUUID) -> Void
            if let imp = descriptor.method(for: sel) {
                let fn = unsafeBitCast(imp, to: S.self)
                fn(descriptor, sel, damageCallbackUUID)
            }
            damageRegistered = false
        }
    }

    func emitStreamStarted() {
        let info: [String: Any] = [
            "type": "stream-started",
            "pixelWidth": width,
            "pixelHeight": height,
            "pointWidth": width / 3,    // most modern devices @ 3x; ok approx for coord mapping
            "pointHeight": height / 3,
            "scale": 3,
            "deviceUDID": (device.value(forKey: "UDID") as? NSUUID)?.uuidString ?? "?",
            "deviceName": (device.value(forKey: "name") as? String) ?? "?",
        ]
        if let data = try? JSONSerialization.data(withJSONObject: info),
           let s = String(data: data, encoding: .utf8) {
            eprint("[sim-capture] " + s)
        }
    }

    func handle(surface: IOSurface) {
        // Drop frames if encoder is busy (stay near real-time).
        objc_sync_enter(self)
        if encoding { objc_sync_exit(self); return }
        encoding = true
        objc_sync_exit(self)
        defer {
            objc_sync_enter(self); encoding = false; objc_sync_exit(self)
        }

        let ci = CIImage(ioSurface: surface)
        let opts: [CIImageRepresentationOption: Any] = [
            CIImageRepresentationOption(rawValue: kCGImageDestinationLossyCompressionQuality as String): 0.55
        ]
        guard let jpeg = ciContext.jpegRepresentation(
            of: ci,
            colorSpace: CGColorSpaceCreateDeviceRGB(),
            options: opts) else { return }

        var lenBE = UInt32(jpeg.count).bigEndian
        let header = withUnsafeBytes(of: &lenBE) { Data($0) }
        writeLock.lock(); defer { writeLock.unlock() }
        do {
            try stdout.write(contentsOf: header)
            try stdout.write(contentsOf: jpeg)
        } catch {
            // pipe closed
            exit(0)
        }
        frameCount += 1
        let now = Date()
        let dt = now.timeIntervalSince(lastReportTime)
        if dt >= 5 {
            eprint("[sim-capture] fps≈\(Int(Double(frameCount) / dt))")
            frameCount = 0
            lastReportTime = now
        }
    }
}

// MARK: - Main loop

guard let ctx = bootstrap() else { exit(2) }
guard let ds = defaultDeviceSet(ctx) else { exit(2) }

var currentStream: Stream? = nil
var currentDeviceUDID: String = ""

let sigSrc = DispatchSource.makeSignalSource(signal: SIGTERM, queue: .main)
sigSrc.setEventHandler { currentStream?.stop(); exit(0) }
sigSrc.resume()
signal(SIGTERM, SIG_IGN)
let sigInt = DispatchSource.makeSignalSource(signal: SIGINT, queue: .main)
sigInt.setEventHandler { currentStream?.stop(); exit(0) }
sigInt.resume()
signal(SIGINT, SIG_IGN)

DispatchQueue.global(qos: .userInitiated).async {
    var notifiedNoBoot = false
    while true {
        if let dev = bootedDevice(ds) {
            let udid = (dev.value(forKey: "UDID") as? NSUUID)?.uuidString ?? "?"
            if udid != currentDeviceUDID {
                currentStream?.stop()
                currentStream = nil
                if let disp = findDisplayDescriptor(dev) {
                    let s = Stream(descriptor: disp, device: dev)
                    s.start()
                    currentStream = s
                    currentDeviceUDID = udid
                    notifiedNoBoot = false
                } else {
                    // Descriptor isn't ready yet (early boot). Don't latch the
                    // udid; retry on next poll.
                    eprint("[sim-capture] no display descriptor on booted device (will retry)")
                }
            }
        } else {
            if currentStream != nil {
                eprint("[sim-capture] booted device gone")
                currentStream?.stop()
                currentStream = nil
                currentDeviceUDID = ""
            }
            if !notifiedNoBoot {
                let info = ["type":"no-booted-device"]
                if let data = try? JSONSerialization.data(withJSONObject: info),
                   let s = String(data: data, encoding: .utf8) { eprint("[sim-capture] " + s) }
                notifiedNoBoot = true
            }
        }
        Thread.sleep(forTimeInterval: 1.5)
    }
}

RunLoop.main.run()
