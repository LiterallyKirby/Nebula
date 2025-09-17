#ifndef JNI_H_
#define JNI_H_

#include <memory>
#include <cstdio>
#include <jni.h>

#include "JNIMapper/JNIMapper.h"
#include "rsc.h"
#include "Utils.h"

class JNI final
{
public:
    JNI()
    {
        /* Init main pointers */
        {
            jint result = JNI_GetCreatedJavaVMs(&p_jvm, 1, nullptr);

            if (result != 0)
            {
                printf("[-] JNI() failed to initialize p_jvm\n");
                // On Linux we just print; no MessageBox
            }

            if (p_jvm)
                p_jvm->AttachCurrentThread((void**)&p_env, nullptr);
            else
                p_env = nullptr;

            if (p_env)
                p_mapper = std::make_unique<JNIMapper>(map, p_env);
        }

        /* Init game classes */
        if (p_mapper)
        {
            p_mapper->classes["Minecraft"]->SetInstance(
                p_mapper->classes["Minecraft"]->fields["theMinecraft"]->GetValueObject());
            p_mapper->classes["WorldClient"]->SetInstance(
                p_mapper->classes["Minecraft"]->fields["theWorld"]->GetValueObject());
            p_mapper->classes["PlayerControllerMP"]->SetInstance(
                p_mapper->classes["Minecraft"]->fields["playerController"]->GetValueObject());
        }

        is_init = true;
    }

    ~JNI()
    {
        if (p_jvm)
            p_jvm->DetachCurrentThread();

        is_init = false;
    }

    bool GetInit() const
    {
        return is_init;
    }

    JNIEnv* GetEnv() const
    {
        return p_env;
    }

public:
    std::unique_ptr<JNIMapper> p_mapper;

private:
    JavaVM* p_jvm{ nullptr };
    JNIEnv* p_env{ nullptr };
    bool is_init{ false };
};

// inline singleton pointer
inline std::unique_ptr<JNI> p_jni;

#endif
