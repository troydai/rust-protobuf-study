extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/data.proto"], &["src/"])
        .expect("Failed to compile protobuf files");
}