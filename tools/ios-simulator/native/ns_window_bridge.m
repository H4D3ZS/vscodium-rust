// ns_window_bridge.m — ObjC helpers for Tauri WKWebView overlay mounting.
#import <AppKit/AppKit.h>

void *ns_window_content_view(void *window) {
    if (window == NULL) return NULL;
    NSWindow *win = (__bridge NSWindow *)window;
    NSView *cv = win.contentView;
    return (__bridge void *)cv;
}

static NSRect clamp_rect_to_bounds(NSRect rect, NSRect bounds) {
    if (rect.size.width <= 0 || rect.size.height <= 0) {
        return NSZeroRect;
    }
    if (rect.origin.x < NSMinX(bounds)) {
        rect.size.width -= (NSMinX(bounds) - rect.origin.x);
        rect.origin.x = NSMinX(bounds);
    }
    if (rect.origin.y < NSMinY(bounds)) {
        rect.size.height -= (NSMinY(bounds) - rect.origin.y);
        rect.origin.y = NSMinY(bounds);
    }
    if (NSMaxX(rect) > NSMaxX(bounds)) {
        rect.size.width -= (NSMaxX(rect) - NSMaxX(bounds));
    }
    if (NSMaxY(rect) > NSMaxY(bounds)) {
        rect.size.height -= (NSMaxY(rect) - NSMaxY(bounds));
    }
    if (rect.size.width < 1 || rect.size.height < 1) {
        return NSZeroRect;
    }
    return rect;
}

void sim_overlay_mount(void *webview_ptr, void *child_ptr) {
    if (webview_ptr == NULL || child_ptr == NULL) return;
    NSView *webview = (__bridge NSView *)webview_ptr;
    NSView *child = (__bridge NSView *)child_ptr;
    NSView *superview = webview.superview;
    child.wantsLayer = YES;
    child.layer.masksToBounds = YES;
    if (superview != nil) {
        // Above WKWebView so IOSurface paints over the HTML hole in the panel.
        [superview addSubview:child positioned:NSWindowAbove relativeTo:webview];
    } else {
        [webview addSubview:child];
    }
}

void sim_overlay_set_frame(void *webview_ptr, void *child_ptr, double x, double y, double w,
                           double h) {
    if (webview_ptr == NULL || child_ptr == NULL) return;
    NSView *webview = (__bridge NSView *)webview_ptr;
    NSView *child = (__bridge NSView *)child_ptr;
    NSView *host = webview.superview != nil ? webview.superview : webview;
    NSRect hostBounds = host.bounds;
    NSRect wvFrame = webview.frame;
    // Web/CSS coords are top-left origin; AppKit host coords are bottom-left.
    double nx = wvFrame.origin.x + x;
    double ny = wvFrame.origin.y + (wvFrame.size.height - y - h);
    NSRect frame = clamp_rect_to_bounds(NSMakeRect(nx, ny, w, h), hostBounds);
    child.frame = frame;
    child.hidden = (frame.size.width < 1 || frame.size.height < 1);
}
