
#include "../src/Module.hpp"
#include <jni.h>
#include <cstdio>

struct ToggleSprint : Module {
    bool sprinting = false;

    void onTick(JNIEnv* env) override {
        // Replace this with actual Minecraft JNI calls
        // Example: toggle player sprinting
        // EntityPlayerSP* player = ...;
        // player->setSprinting(!sprinting);

        sprinting = !sprinting; // just toggling for demo
        printf("[ToggleSprint] Sprinting: %s\n", sprinting ? "ON" : "OFF");
    }
};

// Factory function so main.cpp can create it
extern "C" Module* createModule() {
    return new ToggleSprint();
}
