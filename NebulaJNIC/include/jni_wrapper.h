
// jni_wrapper.h
#ifndef JNI_WRAPPER_H
#define JNI_WRAPPER_H

#include <jni.h>

typedef struct {
    JavaVM* jvm;
    JNIEnv* env;
} JNIContext;

JNIContext* jni_init();
void jni_main_loop(JNIContext* ctx);
void jni_shutdown(JNIContext* ctx);

#endif
