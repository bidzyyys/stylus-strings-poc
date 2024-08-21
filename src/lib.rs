#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_sol_types::sol;
use stylus_sdk::evm;
use stylus_sdk::prelude::{entrypoint, external, sol_storage};

sol! {
    #[allow(missing_docs)]
    event DataSet(string data);
}

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

    pub fn set(&mut self, data: String) -> Result<(), Vec<u8>> {
        self._data.set_str(data.clone());
        evm::log(DataSet { data });
        Ok(())
    }
    pub fn pass(&self) -> String {
        self._data.get_string()
    }
}
