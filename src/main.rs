use std::u8;

use prost::Message;

pub mod data {
    include!(concat!(env!("OUT_DIR"), "/study.rs"));
}

fn main() {
    let p = new_person("Alice", 1, "alice@example.com");
    let d = marshal(p);
    let p2 = unmarshal(&d);

    println!("{:?}", p2);
}

fn new_person(name: &str, id: i32, email: &str) -> data::Person {
    let mut p = data::Person::default();
    p.name = name.to_string();
    p.id = id;
    p.email = email.to_string();

    p
}

fn marshal(p: data::Person) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(p.encoded_len());
    p.encode(&mut buf).unwrap();

    buf
}

fn unmarshal(buf: &[u8]) -> data::Person {
    data::Person::decode(buf).unwrap()
}