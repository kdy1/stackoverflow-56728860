extern crate chashmap;
extern crate lazy_static;
use chashmap::{CHashMap, ReadGuard};
use lazy_static::lazy_static;
use std::collections::HashMap;

struct Data {}

#[derive(Default)]
struct Merged {
    types: HashMap<String, Data>,
}

fn merge(ls: &[String]) -> ReadGuard<'static, Vec<String>, Merged> {
    lazy_static! {
        static ref CACHE: CHashMap<Vec<String>, Merged> = Default::default();
    }

    let libs = ls.to_vec();

    let merged = Merged::default();

    CACHE.insert(libs, merged);

    return CACHE.get(ls).unwrap();
}

fn get<'a>(ls: &[String], name: &str) -> Option<&'a Data> {
    let lib = merge(ls);

    if let Some(ty) = lib.types.get(name) {
        return Some(&*ty);
    }

    None
}

fn main() {}
