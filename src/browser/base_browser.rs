use reqwest::Client;
use std::collections::HashMap;

pub struct BaseBrowser {
    pub localstorage: HashMap<String, String>,
    pub session: Client,
}

pub trait Request {
    fn request();
}

pub trait Init {
    fn new();
}

pub trait LocalStorageOperation {
    fn get_local_item(&mut self, k: String);
    fn set_local_item(&mut self, k: String, v: String);
}

pub trait CookieStorageOpration {
    fn set_cookie(&mut self, name: String, value: String, domain: String, path: String);
}

// impl BaseBrowser{
//     fn
// }
