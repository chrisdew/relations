#![crate_id = "relations"]
#![crate_type = "lib"]

extern crate collections;

use std::collections::hashmap::HashMap;
use collections::hash::Hash;

pub fn foo() {
    println!("foo");
}

pub struct Foo;

pub struct OneToMany<O,M> {
    manyByOne: HashMap<O,Vec<M>>,
    oneByMany: HashMap<M,O>
}

impl<O,M> OneToMany<O,M> where O: Eq + Hash, M: Eq + Hash {
    pub fn new(one: O, many: M) -> OneToMany<O,M> {
        OneToMany {
            manyByOne: HashMap::new(),
            oneByMany: HashMap::new(),
        }
    }
}
