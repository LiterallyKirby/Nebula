//
// Created by Kirby on 9/17/25.
//

#ifndef NEBULA_NEBULA_H
#define NEBULA_NEBULA_H

#include <jvmti.h>
#include <Modules/Module.h>
#include <vector>
#include <memory>

class Nebula {
public:
    Nebula();

    void runClient();
    void onKey(int key);

    JavaVM* getJvm();
    JNIEnv* getEnv();
    void setRunning(bool running);
    bool isRunning() const;

private:
    bool running;
std::vector<std::unique_ptr<Module>> modules;
    JavaVM* jvm;
    JNIEnv* env;
};

#endif // NEBULA_NEBULA_H

