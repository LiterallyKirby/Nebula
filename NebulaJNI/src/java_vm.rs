use jni::{JavaVM, JNIEnv, objects::JValue};
use jni::errors::{Result, Error};
use jni::AttachGuard;
use jni::sys::{JNI_OK, jint};
use std::ptr;

pub fn get_created_java_vm() -> Option<JavaVM> {
    unsafe {
        let mut vm_buf: [*mut jni::sys::JavaVM; 1] = [ptr::null_mut()];
        let mut num_vms: jint = 0;

        let result = jni::sys::JNI_GetCreatedJavaVMs(
            vm_buf.as_mut_ptr(),
            1,
            &mut num_vms
        );

        if result == JNI_OK && num_vms > 0 && !vm_buf[0].is_null() {
            JavaVM::from_raw(vm_buf[0]).ok()
        } else {
            None
        }
    }
}


pub fn get_java_system_property(env: &mut JNIEnv, property: &str) -> Result<Option<String>> {
    let system_class = env.find_class("java/lang/System")?;
    let j_property = env.new_string(property)?;
    let result = env.call_static_method(
        system_class,
        "getProperty",
        "(Ljava/lang/String;)Ljava/lang/String;",
        &[JValue::Object(&j_property)]
    )?;
    let obj = result.l()?;
    if obj.is_null() {
        return Ok(None);
    }
    let rust_string = env.get_string((&obj).into())?.into();
    Ok(Some(rust_string))
}


pub fn attach_thread(vm: &JavaVM) -> Result<AttachGuard<'_>> {
    vm.attach_current_thread()
}
