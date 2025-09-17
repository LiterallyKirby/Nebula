//
// Created by Kirby on 9/17/25.
//

#ifndef NEBULA_NEBULA_H
#define NEBULA_NEBULA_H

#include <jvmti.h>
#include <vector>

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
    JavaVM* jvm;
    JNIEnv* env;
};

#endif // NEBULA_NEBULA_H

