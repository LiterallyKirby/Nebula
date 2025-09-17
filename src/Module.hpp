
#pragma once
#include <jni.h>

struct Module {
    virtual void onTick(JNIEnv* env) = 0;
    virtual ~Module() = default;
};
