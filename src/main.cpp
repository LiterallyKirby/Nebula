
#include <jni.h>
#include <thread>
#include <cstdio>
#include <atomic>
#include <vector>
#include <memory>
#include <dlfcn.h>
#include <filesystem>

#include "Module.hpp"   // <--- THIS WAS MISSING

std::atomic<bool> g_running{true};
std::vector<std::unique_ptr<Module>> g_modules;

using CreateModuleFn = Module*();

void loadModules() {
    std::string modulesPath = "./modules"; // adjust path if needed

    for (const auto& entry : std::filesystem::directory_iterator(modulesPath)) {
        if (entry.path().extension() == ".so") {
            void* handle = dlopen(entry.path().c_str(), RTLD_LAZY);
            if (!handle) {
                printf("[-] Failed to load module: %s\n", dlerror());
                continue;
            }

            auto createFn = (CreateModuleFn*) dlsym(handle, "createModule");
            if (!createFn) {
                printf("[-] Module missing createModule: %s\n", dlerror());
                dlclose(handle);
                continue;
            }

            g_modules.emplace_back(createFn());
            printf("[+] Loaded module: %s\n", entry.path().filename().c_str());
        }
    }
}

void mainThread(JNIEnv* env) {
    loadModules();

    while (g_running.load()) {
        for (auto& mod : g_modules)
            mod->onTick(env);

        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }
}

__attribute__((constructor))
void onLoad() {
    printf("[+] Linux PVP client loaded\n");

    JavaVM* jvm = nullptr;
    JNIEnv* env = nullptr;
    if (JNI_GetCreatedJavaVMs(&jvm, 1, nullptr) != JNI_OK || !jvm) {
        printf("[-] Failed to get JVM\n");
        return;
    }

    jvm->AttachCurrentThread((void**)&env, nullptr);
    std::thread(mainThread, env).detach();
}

__attribute__((destructor))
void onUnload() {
    g_running.store(false);
    printf("[+] Linux PVP client unloaded\n");
}
