mod java_vm;
mod jni_wrapper;
mod logger;
mod main_thread;
mod mapping;
mod module_ctx;
mod modules;
mod robot;
use java_vm::get_created_java_vm;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
lazy_static! {
    static ref RUNNING: AtomicBool = AtomicBool::new(true);
   static ref JAVA_VM: Mutex<Option<Arc<jni::JavaVM>>> = Mutex::new(None);
}

#[unsafe(no_mangle)]



pub extern "C" fn init() {
    logger::log("[HOOK] init() called");

    if let Some(vm) = get_created_java_vm() {
        let arc_vm = std::sync::Arc::new(vm);

        if let Ok(mut vm_guard) = JAVA_VM.lock() {
            *vm_guard = Some(arc_vm.clone()); // now types match
            logger::log("[HOOK] Successfully attached to Java VM!");
        }

        main_thread::start_hook(arc_vm); // pass Arc<JavaVM>
    } else {
        logger::log("[HOOK] No Java VM found!");
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn shutdown() {
    logger::log("[HOOK] shutdown() called");
    RUNNING.store(false, Ordering::Relaxed);

    if let Ok(mut vm_guard) = JAVA_VM.lock() {
        *vm_guard = None;
    }
}
