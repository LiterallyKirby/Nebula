
//
// Created by Kirby on 9/17/25.
//

#include "Nebula.h"

#include <thread>
#include "mapping/impl/net/minecraft/client/Minecraft.h"
#include <net/minecraft/client/multiplayer/WorldClient.h>
#include <net/minecraft/entity/EntityPlayerSP.h>

Nebula::Nebula() {
    running = false;
    jvm = nullptr;
    env = nullptr;

    jsize count;
    if (JNI_GetCreatedJavaVMs(&jvm, 1, &count) != JNI_OK || count == 0)
        return;

    jint res = jvm->GetEnv((void**)&env, JNI_VERSION_1_6);
    if (res == JNI_EDETACHED)
        res = jvm->AttachCurrentThread((void**)&env, nullptr);
    if (res != JNI_OK)
        return;

    // Mapping setup
    Mapping::setup();
}

void Nebula::runClient() {
    running = true;

    auto* mc = new Minecraft(this);

    while (running) {
        EntityPlayerSP player = mc->getPlayerContainer();
        WorldClient world = mc->getWorldContainer();

        if (player.getEntityPlayerSP() == nullptr || world.getWorld() == nullptr)
            continue;

        // You can add game logic here
        // Currently just a loop to keep Nebula running
    }

    jvm->DetachCurrentThread();
    delete mc;
}

void Nebula::onKey(int key) {
    // No cheats, so nothing here for now
}

JavaVM* Nebula::getJvm() {
    return jvm;
}

JNIEnv* Nebula::getEnv() {
    return env;
}

void Nebula::setRunning(bool p_running) {
    this->running = p_running;
}

bool Nebula::isRunning() const {
    return running;
}
