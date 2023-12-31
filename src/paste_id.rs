use rand::{self, Rng};
use rocket::request::FromParam;
use rocket::serde::{Deserialize, Serialize};

use std::borrow::Cow;

#[derive(UriDisplayPath, Serialize, Deserialize, Clone)]
pub struct PasteId<'a>(pub Cow<'a, str>);

impl<'a> ToString for PasteId<'a> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
impl PasteId<'_> {
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        PasteId(Cow::Owned(id))
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param
            .chars()
            .all(|c| c.is_ascii_alphabetic())
            .then(|| PasteId(param.into()))
            .ok_or(param)
    }
}
