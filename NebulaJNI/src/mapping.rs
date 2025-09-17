// src/mapping.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Mem {
    pub obfuscated_name: String,
    pub descriptor: String,
    pub is_static: bool,
}

#[derive(Debug)]
pub struct CM {
    /// user-facing simple id or JVM internal name (whatever you want)
    pub name: String,
    pub fields: HashMap<String, Mem>,
    pub methods: HashMap<String, Mem>,
}

impl CM {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Mapping {
    pub lookup: HashMap<String, CM>,
}

impl Mapping {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

 pub fn get_method(&self, class_key: &str, method_key: &str) -> Option<&Mem> {
        self.lookup.get(class_key).and_then(|cm| cm.methods.get(method_key))
    }

    /// Return obfuscated name, descriptor and is_static for a field
    pub fn get_field(&self, class_key: &str, field_key: &str) -> Option<&Mem> {
        self.lookup.get(class_key).and_then(|cm| cm.fields.get(field_key))
    }

    /// Get the stored class "internal name" (e.g. "java.awt.Robot")
    pub fn get_class_name(&self, class_key: &str) -> Option<&str> {
        self.lookup.get(class_key).map(|cm| cm.name.as_str())
    }

    /// Insert a class mapping (if missing). Does not return a borrow to avoid borrow conflicts.
    pub fn make(&mut self, key: &str, name: &str) {
        self.lookup.entry(key.to_string()).or_insert_with(|| CM::new(name));
    }

    /// Add or replace a field on the class identified by `class_key`.
    pub fn add_field(&mut self, class_key: &str, key_name: &str, ob_name: &str, desc: &str, is_static: bool) {
        if let Some(cm) = self.lookup.get_mut(class_key) {
            let mem = Mem {
                obfuscated_name: ob_name.to_string(),
                descriptor: desc.to_string(),
                is_static,
            };
            cm.fields.insert(key_name.to_string(), mem);
        } else {
            // If you want, create the class on demand:
            let mut cm = CM::new(class_key);
            let mem = Mem {
                obfuscated_name: ob_name.to_string(),
                descriptor: desc.to_string(),
                is_static,
            };
            cm.fields.insert(key_name.to_string(), mem);
            self.lookup.insert(class_key.to_string(), cm);
        }
    }

    /// Add or replace a method on the class identified by `class_key`.
    pub fn add_method(&mut self, class_key: &str, key_name: &str, ob_name: &str, desc: &str, is_static: bool) {
        if let Some(cm) = self.lookup.get_mut(class_key) {
            let mem = Mem {
                obfuscated_name: ob_name.to_string(),
                descriptor: desc.to_string(),
                is_static,
            };
            cm.methods.insert(key_name.to_string(), mem);
        } else {
            // create the class on demand
            let mut cm = CM::new(class_key);
            let mem = Mem {
                obfuscated_name: ob_name.to_string(),
                descriptor: desc.to_string(),
                is_static,
            };
            cm.methods.insert(key_name.to_string(), mem);
            self.lookup.insert(class_key.to_string(), cm);
        }
    }

    /// Read-only access
    pub fn get_class(&self, key: &str) -> Option<&CM> {
        self.lookup.get(key)
    }

/// Setup only from what's in src/robot.rs (java.awt.Robot and its methods).
pub fn setup_mappings() -> Mapping {
    let mut mapping = Mapping::new();

    // Create the Robot class mapping
    mapping.make("Robot", "java.awt.Robot");

    // Add methods seen in robot.rs
    mapping.add_method("Robot", "mousePress", "mousePress", "(I)V", false);
    mapping.add_method("Robot", "mouseRelease", "mouseRelease", "(I)V", false);
    mapping.add_method("Robot", "delay", "delay", "(I)V", false);

    mapping
}
}


