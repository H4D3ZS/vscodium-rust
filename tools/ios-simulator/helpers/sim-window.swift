// sim-window.swift
//
// Programmatic Simulator.app control for IDE embedding.
// Uses Accessibility APIs to position the native Simulator window over an IDE panel hole.
//
// Commands:
//   open <udid>              Boot (if needed), launch Simulator, wait for device window → JSON stdout
//   set-frame <x> <y> <w> <h> Move/resize the device window (screen coords, top-left origin)
//   hide                     Move window off-screen (pause — do not orderOut)
//   raise                    Bring device window forward
//
// Compile:
//   swiftc -O sim-window.swift -o sim-window

import Foundation
import AppKit
import ApplicationServices

let SIM_BUNDLE = "com.apple.iphonesimulator"

func eprint(_ s: String) {
    FileHandle.standardError.write((s + "\n").data(using: .utf8) ?? Data())
}

func axWindows(_ element: AXUIElement) -> [AXUIElement] {
    var ref: CFTypeRef?
    guard AXUIElementCopyAttributeValue(element, kAXWindowsAttribute as CFString, &ref) == .success,
          let arr = ref as? [AXUIElement] else { return [] }
    return arr
}

func axPoint(_ element: AXUIElement) -> CGPoint? {
    var ref: CFTypeRef?
    guard AXUIElementCopyAttributeValue(element, kAXPositionAttribute as CFString, &ref) == .success,
          let ref else { return nil }
    var p = CGPoint.zero
    guard AXValueGetValue(ref as! AXValue, .cgPoint, &p) else { return nil }
    return p
}

func axSize(_ element: AXUIElement) -> CGSize? {
    var ref: CFTypeRef?
    guard AXUIElementCopyAttributeValue(element, kAXSizeAttribute as CFString, &ref) == .success,
          let ref else { return nil }
    var s = CGSize.zero
    guard AXValueGetValue(ref as! AXValue, .cgSize, &s) else { return nil }
    return s
}

func setFrame(_ win: AXUIElement, x: Double, y: Double, w: Double, h: Double) -> Bool {
    var p = CGPoint(x: x, y: y)
    var s = CGSize(width: w, height: h)
    guard let pVal = AXValueCreate(.cgPoint, &p),
          let sVal = AXValueCreate(.cgSize, &s) else { return false }
    let e1 = AXUIElementSetAttributeValue(win, kAXPositionAttribute as CFString, pVal)
    let e2 = AXUIElementSetAttributeValue(win, kAXSizeAttribute as CFString, sVal)
    return e1 == .success && e2 == .success
}

func simulatorPID() -> pid_t? {
    NSWorkspace.shared.runningApplications
        .first { $0.bundleIdentifier == SIM_BUNDLE }?
        .processIdentifier
}

func deviceWindow() -> AXUIElement? {
    guard let pid = simulatorPID() else { return nil }
    let axApp = AXUIElementCreateApplication(pid)
    guard let wins = Optional(axWindows(axApp)), !wins.isEmpty else { return nil }
    var best: AXUIElement?
    var bestArea: Double = 0
    for w in wins {
        guard let sz = axSize(w), sz.width > 200, sz.height > 300 else { continue }
        let area = sz.width * sz.height
        if area > bestArea {
            bestArea = area
            best = w
        }
    }
    return best
}

func simDeviceBooted(udid: String) -> Bool {
    let list = Process()
    list.executableURL = URL(fileURLWithPath: "/usr/bin/xcrun")
    list.arguments = ["simctl", "list", "devices", udid, "--json"]
    let pipe = Pipe()
    list.standardOutput = pipe
    list.standardError = FileHandle.nullDevice
    try? list.run()
    list.waitUntilExit()
    guard list.terminationStatus == 0,
          let data = try? pipe.fileHandleForReading.readToEnd(),
          let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
          let devices = json["devices"] as? [String: Any] else { return false }
    for (_, arr) in devices {
        guard let list = arr as? [[String: Any]] else { continue }
        for d in list {
            guard let id = d["udid"] as? String, id == udid else { continue }
            let state = (d["state"] as? String) ?? ""
            return state == "Booted" || state == "Booting"
        }
    }
    return false
}

func cmdOpen(udid: String) {
    if !AXIsProcessTrusted() {
        eprint("[sim-window] WARN: Accessibility not granted — enable for this IDE in System Settings → Privacy → Accessibility")
        let opts = [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true] as CFDictionary
        _ = AXIsProcessTrustedWithOptions(opts)
    }

    if !simDeviceBooted(udid: udid) {
        let boot = Process()
        boot.executableURL = URL(fileURLWithPath: "/usr/bin/xcrun")
        boot.arguments = ["simctl", "boot", udid]
        boot.standardOutput = FileHandle.nullDevice
        boot.standardError = FileHandle.nullDevice
        try? boot.run()
        boot.waitUntilExit()
    }

    let open = Process()
    open.executableURL = URL(fileURLWithPath: "/usr/bin/open")
    open.arguments = ["-a", "Simulator", "--args", "-CurrentDeviceUDID", udid]
    try? open.run()
    open.waitUntilExit()

    var win: AXUIElement?
    let deadline = Date().addingTimeInterval(90)
    while Date() < deadline {
        if let w = deviceWindow() {
            win = w
            break
        }
        Thread.sleep(forTimeInterval: 0.15)
    }
    guard let w = win else {
        print("{\"ok\":false,\"error\":\"simulator window not found\"}")
        exit(1)
    }

    let pos = axPoint(w) ?? .zero
    let sz = axSize(w) ?? CGSize(width: 393, height: 852)
    _ = AXUIElementPerformAction(w, kAXRaiseAction as CFString)

    let out: [String: Any] = [
        "ok": true,
        "x": pos.x,
        "y": pos.y,
        "width": sz.width,
        "height": sz.height,
    ]
    if let data = try? JSONSerialization.data(withJSONObject: out),
       let s = String(data: data, encoding: .utf8) {
        print(s)
    }
}

func cmdSetFrame(_ args: [String]) {
    guard args.count >= 4,
          let x = Double(args[0]), let y = Double(args[1]),
          let w = Double(args[2]), let h = Double(args[3]),
          let win = deviceWindow() else {
        exit(1)
    }
    guard setFrame(win, x: x, y: y, w: w, h: h) else { exit(1) }
    _ = AXUIElementPerformAction(win, kAXRaiseAction as CFString)
}

func cmdHide() {
    guard let win = deviceWindow() else { exit(0) }
    _ = setFrame(win, x: -20000, y: -20000, w: 320, h: 568)
}

func cmdRaise() {
    guard let win = deviceWindow() else { exit(1) }
    _ = AXUIElementPerformAction(win, kAXRaiseAction as CFString)
}

let args = CommandLine.arguments
guard args.count >= 2 else {
    eprint("usage: sim-window <open|set-frame|hide|raise> ...")
    exit(2)
}

switch args[1] {
case "open":
    guard args.count >= 3 else { exit(2) }
    cmdOpen(udid: args[2])
case "set-frame":
    cmdSetFrame(Array(args.dropFirst(2)))
case "hide":
    cmdHide()
case "raise":
    cmdRaise()
default:
    eprint("unknown command: \(args[1])")
    exit(2)
}
