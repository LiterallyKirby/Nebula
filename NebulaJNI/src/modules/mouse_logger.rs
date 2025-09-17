
// src/mouse_logger.rs
use crate::jni_wrapper::Module;
use crate::module_ctx::ModuleCtx;
use jni::{JNIEnv, objects::JValue};
use jni::errors::Result as JniResult;
use std::fs::OpenOptions;
use std::io::Write;

pub struct MouseLogger {
    ctx: ModuleCtx,
    prev_pressed: bool,
    log_file: Option<std::fs::File>,
}

impl MouseLogger {
    pub fn new(env: &mut JNIEnv, ctx: ModuleCtx) -> JniResult<Self> {
        // Ensure class is present / loadable (optional)
        if let Some(class_internal) = ctx.get_class_internal("Mouse") {
            env.find_class(class_internal.as_str())?;
        } // else we just try later and error gracefully

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/mouse_logger.log")
            .ok();

        Ok(Self {
            ctx,
            prev_pressed: false,
            log_file,
        })
    }

    fn write_log(&mut self, s: &str) {
        if let Some(f) = self.log_file.as_mut() {
            let _ = writeln!(f, "{}", s);
            let _ = f.flush();
        } else {
            eprintln!("{}", s);
        }
    }
}

impl Module for MouseLogger {
    fn name(&self) -> &'static str { "MouseLogger" }
    fn is_enabled(&self) -> bool { true }

    fn tick(&mut self, env: &mut JNIEnv) {
        // a single-line call using ModuleCtx helper
        match self.ctx.call_static_bool(env, "Mouse", "isButtonDown", &[JValue::Int(0)]) {
            Ok(is_down) => {
                if is_down && !self.prev_pressed {
                    self.write_log("[MouseLogger] button 0 pressed");
                }
                self.prev_pressed = is_down;
            }
            Err(e) => {
                self.write_log(&format!("[MouseLogger] JNI error: {:?}", e));
            }
        }
    }
}
