#ifndef SIM_HOST_H
#define SIM_HOST_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Attach a native sim display host to an AppKit parent view (NSView*).
void *sim_host_attach(void *parent_nsview);

/// Boot device (if needed) and start headless IOSurface capture → in-view display.
/// Returns 0 on success.
int32_t sim_host_start(void *host, const char *udid);

/// Position the native view inside the parent (content-view coords, top-left origin).
void sim_host_set_frame(void *host, double x, double y, double width, double height);

void sim_host_set_visible(void *host, int32_t visible);

void sim_host_stop(void *host);

void sim_host_detach(void *host);

#ifdef __cplusplus
}
#endif

#endif
