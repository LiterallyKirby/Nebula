// src/main_thread.rs
use crate::jni_wrapper::JNI;
use jni::JavaVM;
use std::fs::OpenOptions;
use std::sync::{Mutex, Once};
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::io::Write;

static mut JNI_INSTANCE: Option<Mutex<JNI>> = None;
static INIT: Once = Once::new();



pub fn start_hook(vm: Arc<JavaVM>) {
    INIT.call_once(|| {
        unsafe {
            match JNI::init(vm.clone()) {
                Ok(jni) => {
                    JNI_INSTANCE = Some(Mutex::new(jni));
                    crate::logger::log("[HOOK] JNI initialized successfully");
                }
                Err(e) => {
                    crate::logger::log(&format!("[HOOK] Failed to init JNI: {:?}", e));
                }
            }
        }
    });
}



pub fn main_loop() {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/NebulaJNI.log")
        .expect("Failed to open log file");

    writeln!(log_file, "[HOOK] Main loop starwrgrted").unwrap();

    let mut _tick = 0; // renamed to avoid unused warning
   
loop {
    _tick += 1;
    unsafe {
        if let Some(ref instance) = JNI_INSTANCE {
            let mut jni = instance.lock().unwrap();
            crate::logger::log(&format!("[HOOK] Running tick {}", _tick));
            jni.main_loop();
        } else {
            crate::logger::log("[HOOK] JNI_INSTANCE not initialized yet");
        }
    }
    thread::sleep(Duration::from_secs(2));
}

}

pub fn stop_hook() {
    unsafe {
        JNI_INSTANCE = None;
    }
}
