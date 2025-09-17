// src/module_ctx.rs
use crate::mapping::{Mapping, Mem};
use jni::errors::{Error as JniError, Result as JniResult};
use jni::{JNIEnv, objects::JValue};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ModuleCtx {
    pub mapping: Arc<Mutex<Mapping>>,
}

impl ModuleCtx {
    pub fn new(mapping: Arc<Mutex<Mapping>>) -> Self {
        Self { mapping }
    }

    /// Return an owned Mem clone for (class_key, method_key) if present.
    /// This clones the Mem while holding the lock briefly and then drops the lock.
    pub fn get_method(&self, class_key: &str, method_key: &str) -> Option<Mem> {
        let map = self.mapping.lock().unwrap();
        map.get_method(class_key, method_key).cloned()
    }

    /// Return the class internal name (slashes), e.g. "java/awt/Robot"
    pub fn get_class_internal(&self, class_key: &str) -> Option<String> {
        let map = self.mapping.lock().unwrap();
        map.get_class_name(class_key).map(|s| s.replace('.', "/"))
    }

    /// Call a static boolean method identified by class_key and method_key.
    /// Returns a jni::errors::Result<bool>.
    pub fn call_static_bool(
        &self,
        env: &mut JNIEnv,
        class_key: &str,
        method_key: &str,
        args: &[JValue],
    ) -> JniResult<bool> {
        // Get method metadata (clone Mem while holding lock briefly)
        let mem = self.get_method(class_key, method_key).ok_or_else(|| {
            // Construct the structured enum variant expected by newer jni versions
            JniError::MethodNotFound {
                name: class_key.to_string(),
                sig: method_key.to_string(),
            }
        })?;

        let class_internal = self
            .get_class_internal(class_key)
            .unwrap_or_else(|| class_key.replace('.', "/"));

        let class = env.find_class(class_internal.as_str())?;

        let jv = env.call_static_method(
            class,
            mem.obfuscated_name.as_str(),
            mem.descriptor.as_str(),
            args,
        )?;

        match mem.descriptor.chars().next() {
            Some('Z') => Ok(jv.z()?),
            Some('B') => Ok(jv.b()? != 0),
            Some('S') => Ok(jv.s()? != 0),
            Some('I') => Ok(jv.i()? != 0),
            _ => Err(JniError::MethodNotFound {
                name: class_key.to_string(),
                sig: method_key.to_string(),
            }),
        }
    }
}
