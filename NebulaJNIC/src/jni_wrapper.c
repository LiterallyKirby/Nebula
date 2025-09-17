
// jni_wrapper.c

#include <stdlib.h>   // for malloc, free
#include <jni.h>
#include <unistd.h>
#include "jni_wrapper.h"


JNIContext* jni_init() {
    JNIContext* ctx = malloc(sizeof(JNIContext));
    ctx->jvm = NULL;
    ctx->env = NULL;
    printf("[HOOK] JNI initialized\n");
    return ctx;
}

void jni_main_loop(JNIContext* ctx) {
    printf("[HOOK] Main loop running\n");
    while (1) {
        // call module ticks here
        sleep(2);
    }
}

void jni_shutdown(JNIContext* ctx) {
    free(ctx);
    printf("[HOOK] JNI shutdown\n");
}
