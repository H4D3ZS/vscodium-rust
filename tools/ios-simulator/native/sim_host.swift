// sim_host.swift — Headless CoreSimulator IOSurface → native NSView overlay above WKWebView.
// No Simulator.app window. Same private API as sim-capture; displays via CALayer IOSurface.

import AppKit
import Foundation
import IOSurface
import ObjectiveC
import QuartzCore

private let CS_PATH = "/Library/Developer/PrivateFrameworks/CoreSimulator.framework/CoreSimulator"

private func devDir() -> String {
    let p = Process()
    p.executableURL = URL(fileURLWithPath: "/usr/bin/xcode-select")
    p.arguments = ["-p"]
    let pipe = Pipe()
    p.standardOutput = pipe
    try? p.run()
    p.waitUntilExit()
    let s = String(data: pipe.fileHandleForReading.readDataToEndOfFile(), encoding: .utf8) ?? ""
    let t = s.trimmingCharacters(in: .whitespacesAndNewlines)
    return t.isEmpty ? "/Applications/Xcode.app/Contents/Developer" : t
}

@_silgen_name("sim_overlay_mount")
func sim_overlay_mount(_ webview: UnsafeMutableRawPointer?, _ child: UnsafeMutableRawPointer?)

@_silgen_name("sim_overlay_set_frame")
func sim_overlay_set_frame(_ webview: UnsafeMutableRawPointer?, _ child: UnsafeMutableRawPointer?,
                           _ x: Double, _ y: Double, _ w: Double, _ h: Double)

// MARK: - Display view (IOSurface → layer, GPU-backed)

private var gTouchCb: sim_host_touch_fn?
private var gSizeCb: sim_host_size_fn?
private var gLastReportedSize: (UInt32, UInt32) = (0, 0)

@_cdecl("sim_host_set_touch_callback")
public func sim_host_set_touch_callback(_ cb: sim_host_touch_fn?) {
    gTouchCb = cb
}

@_cdecl("sim_host_set_size_callback")
public func sim_host_set_size_callback(_ cb: sim_host_size_fn?) {
    gSizeCb = cb
}

private func forwardTouch(x: Double, y: Double, phase: String) {
    guard let cb = gTouchCb else { return }
    phase.withCString { cb(x, y, $0) }
}

private func findWKWebView(in view: NSView) -> NSView {
    let name = String(describing: type(of: view))
    if name.contains("WKWebView") || name.contains("WKFullScreen") { return view }
    for sub in view.subviews {
        let hit = findWKWebView(in: sub)
        if hit !== view { return hit }
    }
    return view
}

final class SimDisplayView: NSView {
    override var isFlipped: Bool { true }

    override func acceptsFirstMouse(for event: NSEvent?) -> Bool { true }

    override func viewDidMoveToWindow() {
        super.viewDidMoveToWindow()
        wantsLayer = true
        layer?.backgroundColor = NSColor.black.cgColor
        layer?.contentsGravity = .resizeAspect
        layer?.contentsScale = window?.backingScaleFactor ?? NSScreen.main?.backingScaleFactor ?? 2.0
    }

    func present(_ surface: IOSurface) {
        let w = UInt32(IOSurfaceGetWidth(surface))
        let h = UInt32(IOSurfaceGetHeight(surface))
        if w > 0, h > 0, (w, h) != gLastReportedSize, let cb = gSizeCb {
            gLastReportedSize = (w, h)
            cb(w, h)
        }
        if Thread.isMainThread {
            layer?.contents = surface
        } else {
            DispatchQueue.main.async { [weak self] in
                self?.layer?.contents = surface
            }
        }
    }

    private func normPoint(_ event: NSEvent) -> (Double, Double)? {
        guard bounds.width > 1, bounds.height > 1 else { return nil }
        let loc = convert(event.locationInWindow, from: nil)
        let x = min(1, max(0, Double(loc.x / bounds.width)))
        let y = min(1, max(0, Double(loc.y / bounds.height)))
        return (x, y)
    }

    override func mouseDown(with event: NSEvent) {
        if let (x, y) = normPoint(event) { forwardTouch(x: x, y: y, phase: "down") }
    }

    override func mouseDragged(with event: NSEvent) {
        if let (x, y) = normPoint(event) { forwardTouch(x: x, y: y, phase: "move") }
    }

    override func mouseUp(with event: NSEvent) {
        if let (x, y) = normPoint(event) { forwardTouch(x: x, y: y, phase: "up") }
    }
}

// MARK: - Headless capture (adapted from sim-capture.swift)

final class HeadlessCapture {
    weak var view: SimDisplayView?
    var stream: CaptureStream?

    init(view: SimDisplayView) {
        self.view = view
    }

    func start(udid: String) -> Bool {
        guard dlopen(CS_PATH, RTLD_NOW) != nil else { return false }
        guard let SimServiceContext = NSClassFromString("SimServiceContext") as? NSObject.Type,
              NSProtocolFromString("SimDisplayIOSurfaceRenderable") != nil else { return false }

        let sel = NSSelectorFromString("sharedServiceContextForDeveloperDir:error:")
        typealias Sig = @convention(c) (AnyObject, Selector, NSString,
            AutoreleasingUnsafeMutablePointer<NSError?>) -> AnyObject?
        guard let imp = SimServiceContext.method(for: sel) else { return false }
        let fn = unsafeBitCast(imp, to: Sig.self)
        var err: NSError?
        let ctx = withUnsafeMutablePointer(to: &err) { ep -> AnyObject? in
            fn(SimServiceContext, sel, devDir() as NSString, AutoreleasingUnsafeMutablePointer(ep))
        }
        guard let ctx else { return false }

        let dsSel = NSSelectorFromString("defaultDeviceSetWithError:")
        typealias DsSig = @convention(c) (AnyObject, Selector,
            AutoreleasingUnsafeMutablePointer<NSError?>) -> AnyObject?
        guard let dsImp = (ctx as! NSObject).method(for: dsSel) else { return false }
        let dsFn = unsafeBitCast(dsImp, to: DsSig.self)
        var dsErr: NSError?
        let ds = withUnsafeMutablePointer(to: &dsErr) { ep -> AnyObject? in
            dsFn(ctx, dsSel, AutoreleasingUnsafeMutablePointer(ep))
        }
        guard let ds else { return false }

        var device: NSObject?
        for _ in 0..<300 {
            device = bootedDevice(ds, udid: udid.lowercased())
            if device != nil { break }
            Thread.sleep(forTimeInterval: 0.2)
        }
        guard let dev = device, let desc = findDisplayDescriptor(dev), let view else { return false }

        let s = CaptureStream(descriptor: desc, view: view)
        s.start()
        stream = s
        return true
    }

    func stop() {
        stream?.stop()
        stream = nil
    }

    private func bootedDevice(_ deviceSet: AnyObject, udid: String) -> NSObject? {
        guard let devices = (deviceSet as AnyObject).value(forKey: "devices") as? NSArray else { return nil }
        for d in devices {
            let dev = d as AnyObject
            let state = (dev.value(forKey: "state") as? NSNumber)?.intValue ?? -1
            if state != 3 { continue }
            guard let booted = dev as? NSObject else { continue }
            let id = ((dev.value(forKey: "UDID") as? NSUUID)?.uuidString ?? "").lowercased()
            if id == udid { return booted }
        }
        return nil
    }

    private func findDisplayDescriptor(_ device: NSObject) -> NSObject? {
        guard let proto = NSProtocolFromString("SimDisplayIOSurfaceRenderable"),
              let io = device.value(forKey: "io") as? NSObject,
              let ports = io.value(forKey: "ioPorts") as? NSArray else { return nil }
        let descSel = NSSelectorFromString("descriptor")
        let surfSel = NSSelectorFromString("framebufferSurface")
        typealias DescFn = @convention(c) (AnyObject, Selector) -> AnyObject?
        var best: NSObject?
        var bestArea = 0
        for p in ports {
            let port = p as! NSObject
            guard let imp = port.method(for: descSel) else { continue }
            let fn = unsafeBitCast(imp, to: DescFn.self)
            guard let desc = fn(port, descSel) as? NSObject, desc.conforms(to: proto) else { continue }
            guard let s = desc.perform(surfSel)?.takeUnretainedValue(),
                  CFGetTypeID(s) == IOSurfaceGetTypeID() else { continue }
            let surf = unsafeBitCast(s, to: IOSurfaceRef.self) as IOSurface
            let area = IOSurfaceGetWidth(surf) * IOSurfaceGetHeight(surf)
            if area > bestArea { bestArea = area; best = desc }
        }
        return best
    }
}

final class CaptureStream {
    let descriptor: NSObject
    weak var view: SimDisplayView?
    let callbackUUID = NSUUID()
    let damageUUID = NSUUID()
    var lastPresent = Date.distantPast
    let minInterval: TimeInterval = 1.0 / 60.0

    init(descriptor: NSObject, view: SimDisplayView) {
        self.descriptor = descriptor
        self.view = view
    }

    func start() {
        if let s = framebufferSurface() {
            view?.present(s)
        }
        registerCallbacks()
    }

    func stop() {
        unregister(sel: "unregisterIOSurfacesChangeCallbackWithUUID:", uuid: callbackUUID)
        unregister(sel: "unregisterDamageRectanglesCallbackWithUUID:", uuid: damageUUID)
    }

    private func framebufferSurface() -> IOSurface? {
        guard let s = descriptor.perform(NSSelectorFromString("framebufferSurface"))?.takeUnretainedValue(),
              CFGetTypeID(s) == IOSurfaceGetTypeID() else { return nil }
        return unsafeBitCast(s, to: IOSurfaceRef.self) as IOSurface
    }

    private func registerCallbacks() {
        let regSel = NSSelectorFromString("registerCallbackWithUUID:ioSurfacesChangeCallback:")
        typealias RegSig = @convention(c) (AnyObject, Selector, NSUUID,
            @convention(block) (AnyObject?, AnyObject?) -> Void) -> Void
        if let imp = descriptor.method(for: regSel) {
            let fn = unsafeBitCast(imp, to: RegSig.self)
            let block: @convention(block) (AnyObject?, AnyObject?) -> Void = { [weak self] _, new in
                guard let self, let n = new, CFGetTypeID(n) == IOSurfaceGetTypeID() else { return }
                self.present(unsafeBitCast(n, to: IOSurfaceRef.self) as IOSurface)
            }
            fn(descriptor, regSel, callbackUUID, block)
        }
        let damSel = NSSelectorFromString("registerCallbackWithUUID:damageRectanglesCallback:")
        typealias DamSig = @convention(c) (AnyObject, Selector, NSUUID,
            @convention(block) (AnyObject?) -> Void) -> Void
        if let imp = descriptor.method(for: damSel) {
            let fn = unsafeBitCast(imp, to: DamSig.self)
            let block: @convention(block) (AnyObject?) -> Void = { [weak self] _ in
                guard let self, let s = self.framebufferSurface() else { return }
                self.present(s)
            }
            fn(descriptor, damSel, damageUUID, block)
        }
    }

    private func unregister(sel name: String, uuid: NSUUID) {
        let sel = NSSelectorFromString(name)
        typealias S = @convention(c) (AnyObject, Selector, NSUUID) -> Void
        if let imp = descriptor.method(for: sel) {
            unsafeBitCast(imp, to: S.self)(descriptor, sel, uuid)
        }
    }

    private func present(_ surface: IOSurface) {
        let now = Date()
        if now.timeIntervalSince(lastPresent) < minInterval { return }
        lastPresent = now
        view?.present(surface)
    }
}

// MARK: - Host + C ABI

final class SimHost {
    let webview: NSView
    let view = SimDisplayView(frame: .zero)
    var capture: HeadlessCapture

    init(webview: NSView) {
        self.webview = findWKWebView(in: webview)
        capture = HeadlessCapture(view: view)
        let wvPtr = Unmanaged.passUnretained(self.webview).toOpaque()
        let viewPtr = Unmanaged.passUnretained(view).toOpaque()
        sim_overlay_mount(wvPtr, viewPtr)
        view.autoresizingMask = []
    }

    func setFrame(x: Double, y: Double, w: Double, h: Double) {
        let wvPtr = Unmanaged.passUnretained(webview).toOpaque()
        let viewPtr = Unmanaged.passUnretained(view).toOpaque()
        sim_overlay_set_frame(wvPtr, viewPtr, x, y, w, h)
    }

    func setVisible(_ visible: Bool) {
        view.isHidden = !visible
    }

    func detach() {
        capture.stop()
        view.removeFromSuperview()
    }
}

private var hosts: [UnsafeMutableRawPointer: SimHost] = [:]
private let hostsLock = NSLock()

@_cdecl("sim_host_attach")
public func sim_host_attach(_ webview: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let webview else { return nil }
    var result: UnsafeMutableRawPointer?
    let work = {
        let wv = Unmanaged<NSView>.fromOpaque(webview).takeUnretainedValue()
        let host = SimHost(webview: wv)
        let key = UnsafeMutableRawPointer(Unmanaged.passRetained(host).toOpaque())
        hostsLock.lock()
        hosts[key] = host
        hostsLock.unlock()
        result = key
    }
    if Thread.isMainThread {
        work()
    } else {
        DispatchQueue.main.sync(execute: work)
    }
    return result
}

@_cdecl("sim_host_start")
public func sim_host_start(_ handle: UnsafeMutableRawPointer?, _ udid: UnsafePointer<CChar>?) -> Int32 {
    guard let handle, let udid else { return -1 }
    let host = Unmanaged<SimHost>.fromOpaque(handle).takeUnretainedValue()
    let id = String(cString: udid)
    var rc: Int32 = -1
    let work = { rc = host.capture.start(udid: id) ? 0 : -1 }
    if Thread.isMainThread {
        work()
    } else {
        DispatchQueue.main.sync(execute: work)
    }
    return rc
}

@_cdecl("sim_host_set_frame")
public func sim_host_set_frame(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ w: Double, _ h: Double) {
    guard let handle else { return }
    let host = Unmanaged<SimHost>.fromOpaque(handle).takeUnretainedValue()
    let work = { host.setFrame(x: x, y: y, w: w, h: h) }
    if Thread.isMainThread {
        work()
    } else {
        DispatchQueue.main.async(execute: work)
    }
}

@_cdecl("sim_host_set_visible")
public func sim_host_set_visible(_ handle: UnsafeMutableRawPointer?, _ visible: Int32) {
    guard let handle else { return }
    let host = Unmanaged<SimHost>.fromOpaque(handle).takeUnretainedValue()
    let work = { host.setVisible(visible != 0) }
    if Thread.isMainThread {
        work()
    } else {
        DispatchQueue.main.async(execute: work)
    }
}

@_cdecl("sim_host_stop")
public func sim_host_stop(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    let host = Unmanaged<SimHost>.fromOpaque(handle).takeUnretainedValue()
    let work = { host.capture.stop() }
    if Thread.isMainThread {
        work()
    } else {
        DispatchQueue.main.sync(execute: work)
    }
}

@_cdecl("sim_host_detach")
public func sim_host_detach(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    let work = {
        let host = Unmanaged<SimHost>.fromOpaque(handle).takeUnretainedValue()
        host.detach()
        hostsLock.lock()
        hosts.removeValue(forKey: handle)
        hostsLock.unlock()
        Unmanaged<SimHost>.fromOpaque(handle).release()
    }
    if Thread.isMainThread {
        work()
    } else {
        DispatchQueue.main.sync(execute: work)
    }
}
