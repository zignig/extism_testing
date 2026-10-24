// Extism is not async, need to wrap in a task

use extism::{Manifest, Plugin, Wasm};

pub struct PluginRunner {
    manifest: Manifest,
}

impl PluginRunner {
    pub fn new(path: &str) -> Self {
        // Convert to bytes
        let wasm = Wasm::file(path);
        let manifest = Manifest::new([wasm]);
        Self { manifest }
    }

    pub fn run_plugin(&self, input: &str) -> Result<String, extism::Error> {
        let mut plugin = Plugin::new(&self.manifest, [], true)?;
        let res = plugin.call::<&str,&str>("count_vowels",input)?;
        Ok(res.to_string())
    }
}



