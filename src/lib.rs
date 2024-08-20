#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::prelude::{entrypoint, external, sol_storage};

sol_storage! {
    #[entrypoint]
    pub struct StringPoC {
        string _data;
    }
}

#[external]
impl StringPoC {
    pub fn hardcoded(&self) -> String {
        String::from("stylus-strings-poc")
    }

    pub fn empty(&self) -> String {
        String::new()
    }

    pub fn pass(&mut self, data: String) -> String {
        self._data.set_str(data);
        self._data.get_string()
    }
}
