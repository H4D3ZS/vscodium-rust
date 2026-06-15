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

/// phase: "down" | "move" | "up" — normalized 0..1 coords, top-left origin.
typedef void (*sim_host_touch_fn)(double x, double y, const char *phase);
void sim_host_set_touch_callback(sim_host_touch_fn cb);

typedef void (*sim_host_size_fn)(uint32_t width, uint32_t height);
void sim_host_set_size_callback(sim_host_size_fn cb);

#ifdef __cplusplus
}
#endif

#endif
