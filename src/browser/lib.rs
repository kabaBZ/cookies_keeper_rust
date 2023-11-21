use crate::browser::base_browser::{BaseBrowser, LocalStorageOperation};

impl LocalStorageOperation for BaseBrowser {
    fn get_local_item(&mut self, k: String) {
        self.localstorage.get(&k);
    }
    fn set_local_item(&mut self, k: String, v: String) {
        println!("{:#?}{:#?}", k, v)
    }
}
