CC = xcrun clang
CFLAGS = -O2 -Wall -Wno-deprecated-declarations -fobjc-arc -fPIC
FRAMEWORKS = -framework Foundation -framework IOSurface -ldl
TARGET = libane_bridge.dylib

all: $(TARGET)

$(TARGET): ane_bridge.m ane_bridge.h
	$(CC) $(CFLAGS) -dynamiclib -o $@ ane_bridge.m $(FRAMEWORKS)

test: test_bridge.m ane_bridge.h $(TARGET)
	$(CC) $(CFLAGS) -o test_bridge test_bridge.m -L. -lane_bridge $(FRAMEWORKS)

clean:
	rm -f $(TARGET) test_bridge

.PHONY: all clean test
