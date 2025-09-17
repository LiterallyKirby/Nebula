// src/robot.rs
use jni::objects::{GlobalRef, JValue};
use jni::JNIEnv;
use jni::errors::Result;
use crate::jni_wrapper::Module;

pub struct CRobot {
    class: GlobalRef,
    instance: GlobalRef,
}

impl Module for CRobot {
    fn name(&self) -> &'static str {
        "CRobot"
    }

    fn is_enabled(&self) -> bool {
        true
    }

    // <-- signature changed to match trait: &mut JNIEnv
    fn tick(&mut self, env: &mut JNIEnv) {
        // Example: call a method on the instance
        let instance_obj = self.instance.as_obj();
        let _ = env.call_method(instance_obj, "doSomething", "()V", &[]);
        // ignore errors for demo; consider handling them/logging
    }
}

impl CRobot {
    pub fn new(env: &mut JNIEnv) -> Result<Self> {
        // Find java.awt.Robot
        let class_local = env.find_class("java/awt/Robot")?;
        let class = env.new_global_ref(class_local)?; // promote class to global ref

        // Construct Robot object
        let instance_local = env.new_object("java/awt/Robot", "()V", &[])?;
        let instance = env.new_global_ref(instance_local)?;

        Ok(Self { class, instance })
    }

    pub fn mouse_press(&self, env: &mut JNIEnv, buttons: i32) -> Result<()> {
        env.call_method(self.instance.as_obj(), "mousePress", "(I)V", &[JValue::Int(buttons)])?;
        Ok(())
    }

    pub fn mouse_release(&self, env: &mut JNIEnv, buttons: i32) -> Result<()> {
        env.call_method(self.instance.as_obj(), "mouseRelease", "(I)V", &[JValue::Int(buttons)])?;
        Ok(())
    }

    pub fn delay(&self, env: &mut JNIEnv, ms: i32) -> Result<()> {
        env.call_method(self.instance.as_obj(), "delay", "(I)V", &[JValue::Int(ms)])?;
        Ok(())
    }
}
