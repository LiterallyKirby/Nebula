// src/jni_wrapper.rs
use crate::mapping::Mapping;
use crate::module_ctx::*;
use crate::modules::mouse_logger::*;
use crate::robot::CRobot;
use jni::{JNIEnv, JavaVM, errors::Result};
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::Write;
pub trait Module {
    fn name(&self) -> &'static str;
    fn is_enabled(&self) -> bool;
    fn tick(&mut self, env: &mut JNIEnv);
}

pub struct JNI {
    pub vm: Arc<JavaVM>,
    pub modules: Vec<Box<dyn Module + Send>>,
    pub mapping: Arc<Mutex<Mapping>>, // <<-- added field
}

impl JNI {
    pub fn main_loop(&mut self) {
  let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/NebulaJNI.log")
        .expect("Failed to open log file");

    writeln!(log_file, "[HOOK] Main loop startedwfweweg").unwrap();
        // Make env mutable so we can pass &mut env into Module::tick
        let mut env = self.vm.attach_current_thread().unwrap();
        for module in &mut self.modules {
            if module.is_enabled() {
                module.tick(&mut env);
            }
        }
    }

    /// Initialize JNI struct, modules and mapping.
    /// We attach a temporary env only inside this block so the AttachGuard
    /// doesn't live across the `Ok(Self { ... })` return (avoids borrow issues).

   

pub fn init(vm: Arc<JavaVM>) -> Result<Self> {
    crate::logger::log("[HOOK] Attaching to current thread");

    // Nested scope so AttachGuard drops before we move vm
    let (modules, mapping_arc) = {
        let mut env = vm.attach_current_thread()?;

        crate::logger::log("[HOOK] Setting up mapping");
        let mapping = Mapping::setup_mappings();
        let mapping_arc = Arc::new(Mutex::new(mapping));
        let ctx = ModuleCtx::new(mapping_arc.clone());

        crate::logger::log("[HOOK] Creating CRobot");
        let robot = CRobot::new(&mut env)?;

        crate::logger::log("[HOOK] Creating MouseLogger");
        let mouse_logger = MouseLogger::new(&mut env, ctx.clone())?;

        let modules: Vec<Box<dyn Module + Send>> = vec![Box::new(robot), Box::new(mouse_logger)];

        (modules, mapping_arc)
    }; // env dropped here

    crate::logger::log("[HOOK] JNI init complete");
    Ok(Self {
        vm,             // safe to move now
        modules,
        mapping: mapping_arc,
    })
}


}
